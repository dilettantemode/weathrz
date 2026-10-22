use std::sync::OnceLock;

const PLACES: &str = include_str!("assets/places.txt");
const NEAREST_KM: f64 = 1200.0;
const EARTH_KM: f64 = 6371.0;

pub const COUNTRIES: &[(&str, &str)] = &[
    ("BD", "Bangladesh"),
    ("BR", "Brazil"),
    ("CN", "China"),
    ("ET", "Ethiopia"),
    ("GB", "United Kingdom"),
    ("ID", "Indonesia"),
    ("IN", "India"),
    ("JP", "Japan"),
    ("MX", "Mexico"),
    ("MY", "Malaysia"),
    ("NG", "Nigeria"),
    ("PK", "Pakistan"),
    ("RU", "Russia"),
    ("US", "United States"),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountrySetting {
    Auto,
    Fixed(&'static str),
    Nowhere,
}

fn places() -> &'static [(String, f64, f64)] {
    static PARSED: OnceLock<Vec<(String, f64, f64)>> = OnceLock::new();
    PARSED.get_or_init(|| {
        PLACES
            .lines()
            .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
            .filter_map(|line| {
                let mut parts = line.split_whitespace();
                let code = parts.next()?.to_string();
                let latitude = parts.next()?.parse().ok()?;
                let longitude = parts.next()?.parse().ok()?;
                Some((code, latitude, longitude))
            })
            .collect()
    })
}

fn distance_km(from: (f64, f64), to: (f64, f64)) -> f64 {
    let (lat1, lat2) = (from.0.to_radians(), to.0.to_radians());
    let half_lat = (lat2 - lat1) / 2.0;
    let half_lon = (to.1 - from.1).to_radians() / 2.0;
    let h = half_lat.sin().powi(2) + lat1.cos() * lat2.cos() * half_lon.sin().powi(2);
    2.0 * EARTH_KM * h.sqrt().min(1.0).asin()
}

pub fn country_at(latitude: f64, longitude: f64) -> Option<&'static str> {
    let (code, km) = places()
        .iter()
        .map(|(code, lat, lon)| (code, distance_km((latitude, longitude), (*lat, *lon))))
        .min_by(|a, b| a.1.total_cmp(&b.1))?;
    if km > NEAREST_KM {
        return None;
    }
    COUNTRIES
        .iter()
        .map(|(known, _)| *known)
        .find(|known| known == code)
        .or(Some("other"))
}

pub fn known(code: &str) -> Option<&'static str> {
    let wanted = code.trim().to_ascii_uppercase();
    let wanted = if wanted == "UK" {
        "GB".to_string()
    } else {
        wanted
    };
    COUNTRIES
        .iter()
        .map(|(known, _)| *known)
        .find(|known| *known == wanted)
}

pub fn name_of(code: &str) -> Option<&'static str> {
    COUNTRIES
        .iter()
        .find(|(known, _)| *known == code)
        .map(|(_, name)| *name)
}

pub fn parse_setting(text: &str) -> Result<CountrySetting, String> {
    let wanted = text.trim().to_ascii_uppercase();
    match wanted.as_str() {
        "" | "AUTO" => return Ok(CountrySetting::Auto),
        "NONE" => return Ok(CountrySetting::Nowhere),
        "UK" => return Ok(CountrySetting::Fixed("GB")),
        _ => {}
    }
    COUNTRIES
        .iter()
        .find(|(code, _)| *code == wanted)
        .map(|(code, _)| CountrySetting::Fixed(code))
        .ok_or_else(|| {
            format!(
                "country = \"{}\" is not known (use \"auto\", \"none\" or one of {}); using \"auto\"",
                text.trim(),
                COUNTRIES
                    .iter()
                    .map(|(code, _)| *code)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
}

pub fn resolve(setting: CountrySetting, latitude: f64, longitude: f64) -> Option<&'static str> {
    match setting {
        CountrySetting::Auto => country_at(latitude, longitude).filter(|code| *code != "other"),
        CountrySetting::Fixed(code) => Some(code),
        CountrySetting::Nowhere => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_place_line_parses() {
        let lines = PLACES
            .lines()
            .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
            .count();
        assert_eq!(places().len(), lines);
        assert!(places().len() > 900);
        for (code, lat, lon) in places() {
            assert!(code.len() == 2 && code.chars().all(|ch| ch.is_ascii_uppercase()));
            assert!((-90.0..=90.0).contains(lat) && (-180.0..=180.0).contains(lon));
        }
        for (code, _) in COUNTRIES {
            assert!(
                places()
                    .iter()
                    .filter(|(place, _, _)| place == code)
                    .count()
                    >= 6,
                "{code}"
            );
        }
    }

    #[test]
    fn places_name_their_countries() {
        for (place, lat, lon, expected) in [
            ("Jakarta", -6.2088, 106.8456, Some("ID")),
            ("Denpasar", -8.4, 115.2, Some("ID")),
            ("Batam", 1.13, 104.05, Some("ID")),
            ("Johor Bahru", 1.46, 103.76, Some("MY")),
            ("Kota Kinabalu", 5.98, 116.07, Some("MY")),
            ("Singapore", 1.29, 103.85, Some("other")),
            ("Kolkata", 22.57, 88.36, Some("IN")),
            ("Amritsar", 31.63, 74.87, Some("IN")),
            ("Lahore", 31.55, 74.34, Some("PK")),
            ("Dhaka", 23.8, 90.4, Some("BD")),
            ("Shenzhen", 22.54, 114.06, Some("CN")),
            ("Hong Kong", 22.3, 114.17, Some("other")),
            ("Tibet", 33.0, 88.0, Some("CN")),
            ("Tokyo", 35.68, 139.69, Some("JP")),
            ("Okinawa", 26.3, 127.8, Some("JP")),
            ("Moscow", 55.75, 37.62, Some("RU")),
            ("Siberia", 60.0, 100.0, Some("RU")),
            ("Kaliningrad", 54.71, 20.51, Some("RU")),
            ("Addis Ababa", 9.03, 38.74, Some("ET")),
            ("Lagos", 6.5, 3.4, Some("NG")),
            ("Cotonou", 6.37, 2.39, Some("other")),
            ("London", 51.5072, -0.1276, Some("GB")),
            ("Belfast", 54.6, -5.93, Some("GB")),
            ("Dublin", 53.35, -6.26, Some("other")),
            ("Paris", 48.8566, 2.3522, Some("other")),
            ("New York", 40.71, -74.01, Some("US")),
            ("Honolulu", 21.3, -157.86, Some("US")),
            ("Vancouver", 49.28, -123.12, Some("other")),
            ("San Diego", 32.72, -117.16, Some("US")),
            ("Tijuana", 32.5, -117.0, Some("MX")),
            ("Mexico City", 19.43, -99.13, Some("MX")),
            ("Rio de Janeiro", -22.9, -43.2, Some("BR")),
            ("Amazon", -5.0, -65.0, Some("BR")),
            ("Zhuhai", 22.27, 113.58, Some("CN")),
            ("Macau", 22.19, 113.54, Some("other")),
            ("Blagoveshchensk", 50.27, 127.53, Some("RU")),
            ("Laredo", 27.51, -99.51, Some("US")),
            ("Nuevo Laredo", 27.48, -99.52, Some("MX")),
            ("Calexico", 32.68, -115.50, Some("US")),
            ("Yuma", 32.69, -114.62, Some("US")),
            ("Tapachula", 14.90, -92.26, Some("MX")),
            ("Wakkanai", 45.42, 141.67, Some("JP")),
            ("Derry", 55.00, -7.32, Some("GB")),
            ("Silchar", 24.83, 92.78, Some("IN")),
            ("Karimganj", 24.87, 92.36, Some("IN")),
            ("Malda", 25.01, 88.14, Some("IN")),
            ("Dumai", 1.67, 101.45, Some("ID")),
            ("Bengkalis", 1.47, 102.11, Some("ID")),
            ("Bagansiapiapi", 2.16, 100.81, Some("ID")),
            ("Natuna", 3.94, 108.38, Some("ID")),
            ("Sambas", 1.36, 109.30, Some("ID")),
            ("Putussibau", 0.84, 112.93, Some("ID")),
            ("Tanjung Balai Karimun", 1.00, 103.38, Some("ID")),
            ("Mid-Atlantic", 30.0, -40.0, None),
        ] {
            assert_eq!(country_at(lat, lon), expected, "{place}");
        }
    }

    #[test]
    fn the_setting_picks_auto_a_country_or_none() {
        assert_eq!(parse_setting("auto"), Ok(CountrySetting::Auto));
        assert_eq!(parse_setting(""), Ok(CountrySetting::Auto));
        assert_eq!(parse_setting(" my "), Ok(CountrySetting::Fixed("MY")));
        assert_eq!(parse_setting("UK"), Ok(CountrySetting::Fixed("GB")));
        assert_eq!(parse_setting("none"), Ok(CountrySetting::Nowhere));
        let unknown = parse_setting("Narnia").unwrap_err();
        assert!(unknown.contains("\"Narnia\" is not known"), "{unknown}");
        assert!(unknown.contains("ID, IN, JP"), "{unknown}");
        assert_eq!(resolve(CountrySetting::Auto, -6.2, 106.8), Some("ID"));
        assert_eq!(resolve(CountrySetting::Auto, 48.86, 2.35), None);
        assert_eq!(
            resolve(CountrySetting::Fixed("JP"), -6.2, 106.8),
            Some("JP")
        );
        assert_eq!(resolve(CountrySetting::Nowhere, -6.2, 106.8), None);
        assert_eq!(name_of("GB"), Some("United Kingdom"));
        assert_eq!(name_of("FR"), None);
    }
}
