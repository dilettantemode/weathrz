use crate::error::{GeolocationError, NetworkError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

const SEARCH_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";
const CANDIDATES: usize = 10;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Place {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(default)]
    pub admin1: Option<String>,
    #[serde(default)]
    pub admin2: Option<String>,
    #[serde(default)]
    pub country_code: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub population: Option<u64>,
}

impl Place {
    pub fn describe(&self) -> String {
        let region: Vec<&str> = [&self.admin2, &self.admin1, &self.country]
            .into_iter()
            .filter_map(|part| part.as_deref())
            .filter(|part| !part.is_empty() && *part != self.name)
            .collect();
        let region = if region.is_empty() {
            String::new()
        } else {
            format!("  {}", region.join(", "))
        };
        format!(
            "{}{region}  ({:.4}, {:.4})",
            self.name, self.latitude, self.longitude
        )
    }
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    #[serde(default)]
    results: Vec<Place>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub name: String,
    pub country_code: Option<String>,
    pub region: Option<String>,
}

pub fn parse_query(text: &str) -> Query {
    let mut parts = text
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty());
    let name = parts.next().unwrap_or_default().to_string();
    let rest: Vec<&str> = parts.collect();
    let (country_code, region) = match rest.as_slice() {
        [] => (None, None),
        [only] if only.len() == 2 && only.chars().all(|ch| ch.is_ascii_alphabetic()) => {
            (Some(only.to_ascii_uppercase()), None)
        }
        [only] => (None, Some(only.to_string())),
        [region, code, ..] => (Some(code.to_ascii_uppercase()), Some(region.to_string())),
    };
    Query {
        name,
        country_code,
        region,
    }
}

pub fn parse_coordinates(text: &str) -> Option<(f64, f64)> {
    let (lat, lon) = text.split_once(',')?;
    let latitude: f64 = lat.trim().parse().ok()?;
    let longitude: f64 = lon.trim().parse().ok()?;
    ((-90.0..=90.0).contains(&latitude) && (-180.0..=180.0).contains(&longitude))
        .then_some((latitude, longitude))
}

fn mentions(place: &Place, region: &str) -> bool {
    let wanted = region.to_lowercase();
    [&place.admin1, &place.admin2, &place.country]
        .into_iter()
        .filter_map(|part| part.as_deref())
        .any(|part| part.to_lowercase().contains(&wanted))
}

pub fn narrow(query: &Query, places: Vec<Place>) -> Vec<Place> {
    places
        .into_iter()
        .filter(|place| {
            query.country_code.as_deref().is_none_or(|code| {
                place
                    .country_code
                    .as_deref()
                    .is_some_and(|own| own.eq_ignore_ascii_case(code))
            })
        })
        .filter(|place| {
            query
                .region
                .as_deref()
                .is_none_or(|region| mentions(place, region))
        })
        .collect()
}

const LANGUAGES: &[(&str, &str)] = &[
    ("ID", "id"),
    ("MY", "ms"),
    ("IN", "hi"),
    ("CN", "zh"),
    ("TW", "zh"),
    ("JP", "ja"),
    ("KR", "ko"),
    ("RU", "ru"),
    ("BR", "pt"),
    ("PT", "pt"),
    ("MX", "es"),
    ("AR", "es"),
    ("CO", "es"),
    ("ES", "es"),
    ("ET", "am"),
    ("BD", "bn"),
    ("PK", "ur"),
    ("DE", "de"),
    ("AT", "de"),
    ("FR", "fr"),
    ("IT", "it"),
    ("NL", "nl"),
    ("TR", "tr"),
    ("TH", "th"),
    ("VN", "vi"),
    ("SA", "ar"),
    ("EG", "ar"),
];

pub fn language_for(country_code: &str) -> String {
    LANGUAGES
        .iter()
        .find(|(code, _)| code.eq_ignore_ascii_case(country_code))
        .map_or_else(
            || country_code.to_ascii_lowercase(),
            |(_, language)| language.to_string(),
        )
}

fn search_url(query: &Query, language: &str) -> String {
    let mut url = reqwest::Url::parse(SEARCH_URL).expect("static url");
    url.query_pairs_mut()
        .append_pair("name", &query.name)
        .append_pair("count", &CANDIDATES.to_string())
        .append_pair("language", language)
        .append_pair("format", "json");
    if let Some(code) = &query.country_code {
        url.query_pairs_mut().append_pair("countryCode", code);
    }
    url.to_string()
}

pub async fn search(text: &str) -> Result<Vec<Place>, GeolocationError> {
    let query = parse_query(text);
    if query.name.is_empty() {
        return Ok(Vec::new());
    }
    let found = fetch(&query, "en").await?;
    let narrowed = narrow(&query, found.clone());
    if !narrowed.is_empty() || query.region.is_none() {
        return Ok(narrowed);
    }
    let mut languages: Vec<String> = Vec::new();
    for code in found
        .iter()
        .filter_map(|place| place.country_code.as_deref())
    {
        let language = language_for(code);
        if language != "en" && !languages.contains(&language) {
            languages.push(language);
        }
    }
    for language in languages.iter().take(3) {
        let local = narrow(&query, fetch(&query, language).await?);
        if !local.is_empty() {
            return Ok(in_english(local, &found));
        }
    }
    Ok(Vec::new())
}

fn in_english(local: Vec<Place>, english: &[Place]) -> Vec<Place> {
    local
        .into_iter()
        .map(|place| {
            english
                .iter()
                .find(|known| {
                    (known.latitude - place.latitude).abs() < 1e-4
                        && (known.longitude - place.longitude).abs() < 1e-4
                })
                .cloned()
                .unwrap_or(place)
        })
        .collect()
}

async fn fetch(query: &Query, language: &str) -> Result<Vec<Place>, GeolocationError> {
    let url = search_url(query, language);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(5))
        .build()
        .map_err(|e| GeolocationError::Unreachable(NetworkError::ClientCreation(e)))?;
    let response = client
        .get(&url)
        .send()
        .await
        .and_then(|resp| resp.error_for_status())
        .map_err(|e| GeolocationError::Unreachable(NetworkError::from_reqwest(e, &url, 10)))?;
    let data: SearchResponse = response
        .json()
        .await
        .map_err(|e| GeolocationError::ParseError(e.to_string()))?;
    Ok(data.results)
}

#[derive(Serialize, Deserialize)]
struct Remembered {
    query: String,
    place: Place,
}

fn cache_key(text: &str) -> String {
    let normal = text.trim().to_lowercase();
    let hash = normal
        .bytes()
        .fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
        });
    format!("{hash:016x}")
}

fn cache_path(text: &str) -> Option<PathBuf> {
    Some(
        dirs::cache_dir()?
            .join("weathrz")
            .join(format!("place_{}.json", cache_key(text))),
    )
}

fn cached(text: &str) -> Option<Place> {
    let contents = std::fs::read_to_string(cache_path(text)?).ok()?;
    let remembered: Remembered = serde_json::from_str(&contents).ok()?;
    (remembered.query == text.trim().to_lowercase()).then_some(remembered.place)
}

fn remember(text: &str, place: &Place) {
    let Some(path) = cache_path(text) else {
        return;
    };
    let remembered = Remembered {
        query: text.trim().to_lowercase(),
        place: place.clone(),
    };
    let Ok(json) = serde_json::to_string(&remembered) else {
        return;
    };
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let temp = path.with_extension(format!("json.{}.tmp", std::process::id()));
    if std::fs::write(&temp, json).is_ok() {
        let _ = std::fs::rename(temp, path);
    }
}

pub async fn resolve(text: &str) -> Result<Place, String> {
    if let Some((latitude, longitude)) = parse_coordinates(text) {
        return Ok(Place {
            name: format!("{latitude:.4}, {longitude:.4}"),
            latitude,
            longitude,
            admin1: None,
            admin2: None,
            country_code: None,
            country: None,
            population: None,
        });
    }
    if let Some(place) = cached(text) {
        return Ok(place);
    }
    let places = search(text)
        .await
        .map_err(|error| format!("could not look up '{text}': {error}"))?;
    let place = places.into_iter().next().ok_or_else(|| {
        format!("no place called '{text}' (weathrz --locate \"{text}\" lists matches)")
    })?;
    remember(text, &place);
    Ok(place)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn place(name: &str, admin1: &str, code: &str) -> Place {
        Place {
            name: name.to_string(),
            latitude: -6.6,
            longitude: 106.8,
            admin1: Some(admin1.to_string()),
            admin2: None,
            country_code: Some(code.to_string()),
            country: None,
            population: None,
        }
    }

    #[test]
    fn queries_split_a_region_or_a_country_code() {
        assert_eq!(
            parse_query("Bogor"),
            Query {
                name: "Bogor".into(),
                country_code: None,
                region: None
            }
        );
        assert_eq!(parse_query("Bogor, id").country_code.as_deref(), Some("ID"));
        assert_eq!(
            parse_query("Bogor, Jawa Barat").region.as_deref(),
            Some("Jawa Barat")
        );
        let both = parse_query("Bogor, Jawa Barat, ID");
        assert_eq!(
            (both.region.as_deref(), both.country_code.as_deref()),
            (Some("Jawa Barat"), Some("ID"))
        );
    }

    #[test]
    fn coordinates_pass_straight_through() {
        assert_eq!(parse_coordinates("-6.59, 106.79"), Some((-6.59, 106.79)));
        assert_eq!(parse_coordinates("Bogor"), None);
        assert_eq!(parse_coordinates("95, 10"), None);
    }

    #[test]
    fn narrowing_keeps_the_matching_region_and_country() {
        let places = vec![
            place("Bogor", "Banten", "ID"),
            place("Bogor", "West Java", "ID"),
            place("Bogor", "Somewhere", "PH"),
        ];
        let java = narrow(&parse_query("Bogor, west java"), places.clone());
        assert_eq!(java.len(), 1);
        assert_eq!(java[0].admin1.as_deref(), Some("West Java"));
        assert_eq!(narrow(&parse_query("Bogor, PH"), places.clone()).len(), 1);
        assert_eq!(narrow(&parse_query("Bogor"), places).len(), 3);
    }

    #[test]
    fn the_search_url_escapes_the_name() {
        let url = search_url(&parse_query("São Paulo, BR"), "pt");
        assert!(url.contains("name=S%C3%A3o+Paulo"), "{url}");
        assert!(url.contains("countryCode=BR"), "{url}");
        assert!(url.contains("language=pt"), "{url}");
    }

    #[test]
    fn a_local_language_match_shows_its_english_name() {
        let local = vec![Place {
            name: "Kota Bogor".to_string(),
            admin1: Some("Jawa Barat".to_string()),
            ..place("Kota Bogor", "Jawa Barat", "ID")
        }];
        let english = vec![place("Bogor", "West Java", "ID")];
        let shown = in_english(local, &english);
        assert_eq!(shown[0].name, "Bogor");
        assert_eq!(shown[0].admin1.as_deref(), Some("West Java"));
    }

    #[test]
    fn local_languages_for_a_second_try() {
        assert_eq!(language_for("ID"), "id");
        assert_eq!(language_for("jp"), "ja");
        assert_eq!(language_for("NO"), "no");
    }

    #[test]
    fn cache_keys_do_not_collide_on_accents_or_scripts() {
        assert_ne!(cache_key("Bühl"), cache_key("Böhl"));
        assert_ne!(cache_key("東京"), cache_key("大阪"));
        assert_eq!(cache_key(" Bogor "), cache_key("bogor"));
    }

    #[test]
    fn results_parse_with_missing_fields() {
        let data: SearchResponse = serde_json::from_str(
            r#"{"results":[{"name":"Bogor","latitude":-6.59,"longitude":106.79,"admin1":"West Java","country_code":"ID","country":"Indonesia","population":1030720}]}"#,
        )
        .unwrap();
        assert_eq!(
            data.results[0].describe(),
            "Bogor  West Java, Indonesia  (-6.5900, 106.7900)"
        );
        let empty: SearchResponse = serde_json::from_str(r#"{"generationtime_ms":0.5}"#).unwrap();
        assert!(empty.results.is_empty());
    }
}
