use crate::error::GeolocationError;
use crate::geolocation::GeoLocation;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::time::{Duration, Instant};
use tokio::sync::oneshot;

pub const MISMATCH_THRESHOLD_KM: f64 = 100.0;
pub const PROMPT_DURATION: Duration = Duration::from_secs(30);

const EARTH_RADIUS_KM: f64 = 6371.0088;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    Config,
    Ip,
    Quit,
}

#[derive(Debug, Clone)]
pub struct LocationPrompt {
    pub ip: GeoLocation,
    pub km: f64,
    config_label: String,
    shown_at: Instant,
}

impl LocationPrompt {
    pub fn evaluate(
        config: (f64, f64),
        config_label: String,
        ip: GeoLocation,
        now: Instant,
    ) -> Option<Self> {
        let km = mismatch_km(config, (ip.latitude, ip.longitude))?;
        Some(Self {
            ip,
            km,
            config_label,
            shown_at: now,
        })
    }

    pub fn is_expired(&self, now: Instant) -> bool {
        now.saturating_duration_since(self.shown_at) >= PROMPT_DURATION
    }

    pub fn text(&self, now: Instant) -> String {
        let remaining =
            PROMPT_DURATION.saturating_sub(now.saturating_duration_since(self.shown_at));
        let seconds_left = remaining.as_millis().div_ceil(1000);
        let ip_label = place_label(self.ip.city.as_deref(), self.ip.latitude, self.ip.longitude);
        format!(
            "IP location: {ip_label}, ~{km:.0} km from {config} \u{b7} [i] use it \u{b7} [c] keep {config} \u{b7} {seconds_left}s",
            km = self.km,
            config = self.config_label,
        )
    }
}

pub type IpLookup = oneshot::Receiver<Result<GeoLocation, GeolocationError>>;

pub struct IpCheck {
    lookup: Option<IpLookup>,
    config: (f64, f64),
    config_label: String,
    prompt: Option<LocationPrompt>,
}

impl IpCheck {
    pub fn new(lookup: IpLookup, config: (f64, f64), config_label: String) -> Self {
        Self {
            lookup: Some(lookup),
            config,
            config_label,
            prompt: None,
        }
    }

    pub fn poll(&mut self, now: Instant) -> Option<&LocationPrompt> {
        if let Some(lookup) = self.lookup.as_mut() {
            match lookup.try_recv() {
                Ok(Ok(ip)) => {
                    self.prompt =
                        LocationPrompt::evaluate(self.config, self.config_label.clone(), ip, now);
                    self.lookup = None;
                }
                Ok(Err(_)) | Err(oneshot::error::TryRecvError::Closed) => self.lookup = None,
                Err(oneshot::error::TryRecvError::Empty) => {}
            }
        }
        if self
            .prompt
            .as_ref()
            .is_some_and(|prompt| prompt.is_expired(now))
        {
            self.prompt = None;
        }
        self.prompt.as_ref()
    }

    pub fn take_prompt(&mut self) -> Option<LocationPrompt> {
        self.prompt.take()
    }

    pub fn dismiss(&mut self) {
        self.prompt = None;
    }
}

pub fn distance_km(a: (f64, f64), b: (f64, f64)) -> f64 {
    let (lat1, lon1) = (a.0.to_radians(), a.1.to_radians());
    let (lat2, lon2) = (b.0.to_radians(), b.1.to_radians());
    let h = ((lat2 - lat1) / 2.0).sin().powi(2)
        + lat1.cos() * lat2.cos() * ((lon2 - lon1) / 2.0).sin().powi(2);
    2.0 * EARTH_RADIUS_KM * h.sqrt().asin()
}

pub fn mismatch_km(config: (f64, f64), ip: (f64, f64)) -> Option<f64> {
    let km = distance_km(config, ip);
    (km > MISMATCH_THRESHOLD_KM).then_some(km)
}

pub fn place_label(city: Option<&str>, latitude: f64, longitude: f64) -> String {
    match city {
        Some(city) => city.to_string(),
        None => format!("{latitude:.4}, {longitude:.4}"),
    }
}

pub fn choice_for_key(key: &KeyEvent) -> Option<Choice> {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        return matches!(key.code, KeyCode::Char('c') | KeyCode::Char('C')).then_some(Choice::Quit);
    }
    match key.code {
        KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Enter | KeyCode::Esc => {
            Some(Choice::Config)
        }
        KeyCode::Char('i') | KeyCode::Char('I') => Some(Choice::Ip),
        KeyCode::Char('q') | KeyCode::Char('Q') => Some(Choice::Quit),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const JAKARTA: (f64, f64) = (-6.2088, 106.8456);
    const SINGAPORE: (f64, f64) = (1.3521, 103.8198);
    const BEKASI: (f64, f64) = (-6.2383, 106.9756);
    const BANDUNG: (f64, f64) = (-6.9175, 107.6191);

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn distance_jakarta_to_singapore_is_about_905_km() {
        let km = distance_km(JAKARTA, SINGAPORE);
        assert!((km - 905.4).abs() < 1.0, "got {km}");
    }

    #[test]
    fn distance_to_same_point_is_zero() {
        assert!(distance_km(JAKARTA, JAKARTA) < 1e-9);
    }

    #[test]
    fn mismatch_reported_for_another_country() {
        let km = mismatch_km(JAKARTA, SINGAPORE).expect("Singapore should mismatch Jakarta");
        assert!((km - 905.4).abs() < 1.0, "got {km}");
    }

    #[test]
    fn mismatch_reported_for_another_city_past_threshold() {
        assert!(mismatch_km(JAKARTA, BANDUNG).is_some());
    }

    #[test]
    fn no_mismatch_within_the_same_metro_area() {
        assert_eq!(mismatch_km(JAKARTA, BEKASI), None);
    }

    #[test]
    fn place_label_prefers_city() {
        assert_eq!(place_label(Some("Jakarta"), -6.2088, 106.8456), "Jakarta");
    }

    #[test]
    fn place_label_without_city_shows_coordinates() {
        assert_eq!(place_label(None, 1.2897, 103.8501), "1.2897, 103.8501");
    }

    #[test]
    fn config_keys_keep_config() {
        for code in [
            KeyCode::Char('c'),
            KeyCode::Char('C'),
            KeyCode::Enter,
            KeyCode::Esc,
        ] {
            assert_eq!(choice_for_key(&key(code)), Some(Choice::Config), "{code:?}");
        }
    }

    #[test]
    fn ip_keys_pick_ip() {
        assert_eq!(choice_for_key(&key(KeyCode::Char('i'))), Some(Choice::Ip));
        assert_eq!(choice_for_key(&key(KeyCode::Char('I'))), Some(Choice::Ip));
    }

    #[test]
    fn quit_keys_quit() {
        assert_eq!(choice_for_key(&key(KeyCode::Char('q'))), Some(Choice::Quit));
        let ctrl_c = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert_eq!(choice_for_key(&ctrl_c), Some(Choice::Quit));
    }

    #[test]
    fn other_keys_are_ignored() {
        assert_eq!(choice_for_key(&key(KeyCode::Char('x'))), None);
        assert_eq!(choice_for_key(&key(KeyCode::Up)), None);
    }

    fn singapore_ip() -> GeoLocation {
        GeoLocation {
            latitude: SINGAPORE.0,
            longitude: SINGAPORE.1,
            city: Some("Singapore".to_string()),
        }
    }

    fn singapore_prompt(now: Instant) -> LocationPrompt {
        LocationPrompt::evaluate(JAKARTA, "Jakarta".to_string(), singapore_ip(), now)
            .expect("Singapore should prompt against Jakarta")
    }

    #[test]
    fn evaluate_prompts_when_ip_is_far_from_config() {
        let prompt = singapore_prompt(Instant::now());
        assert_eq!(prompt.ip.city.as_deref(), Some("Singapore"));
        assert!((prompt.km - 905.4).abs() < 1.0, "got {}", prompt.km);
    }

    #[test]
    fn evaluate_stays_quiet_when_ip_is_nearby() {
        let ip = GeoLocation {
            latitude: BEKASI.0,
            longitude: BEKASI.1,
            city: Some("Bekasi".to_string()),
        };
        assert!(
            LocationPrompt::evaluate(JAKARTA, "Jakarta".to_string(), ip, Instant::now()).is_none()
        );
    }

    #[test]
    fn prompt_expires_after_its_duration() {
        let start = Instant::now();
        let prompt = singapore_prompt(start);
        assert!(!prompt.is_expired(start + PROMPT_DURATION - Duration::from_secs(1)));
        assert!(prompt.is_expired(start + PROMPT_DURATION));
    }

    #[test]
    fn prompt_text_offers_both_choices_with_countdown() {
        let start = Instant::now();
        let prompt = singapore_prompt(start);
        assert_eq!(
            prompt.text(start),
            "IP location: Singapore, ~905 km from Jakarta \u{b7} [i] use it \u{b7} [c] keep Jakarta \u{b7} 30s"
        );
        assert!(
            prompt
                .text(start + Duration::from_secs(10))
                .ends_with("\u{b7} 20s")
        );
    }

    fn ip_check_with(result: Option<Result<GeoLocation, GeolocationError>>) -> IpCheck {
        let (tx, rx) = oneshot::channel();
        match result {
            Some(result) => tx.send(result).unwrap(),
            None => std::mem::forget(tx),
        }
        IpCheck::new(rx, JAKARTA, "Jakarta".to_string())
    }

    #[test]
    fn ip_check_waits_while_lookup_is_pending() {
        let mut check = ip_check_with(None);
        assert!(check.poll(Instant::now()).is_none());
    }

    #[test]
    fn ip_check_shows_prompt_when_lookup_mismatches() {
        let mut check = ip_check_with(Some(Ok(singapore_ip())));
        let prompt = check.poll(Instant::now()).expect("prompt expected");
        assert_eq!(prompt.ip.city.as_deref(), Some("Singapore"));
    }

    #[test]
    fn ip_check_prompt_disappears_after_it_expires() {
        let mut check = ip_check_with(Some(Ok(singapore_ip())));
        let start = Instant::now();
        assert!(check.poll(start).is_some());
        assert!(check.poll(start + Duration::from_secs(5)).is_some());
        assert!(check.poll(start + PROMPT_DURATION).is_none());
    }

    #[test]
    fn ip_check_stays_quiet_when_lookup_fails() {
        let mut check = ip_check_with(Some(Err(GeolocationError::ParseError("bad".to_string()))));
        assert!(check.poll(Instant::now()).is_none());
    }

    #[test]
    fn ip_check_dismiss_hides_prompt() {
        let mut check = ip_check_with(Some(Ok(singapore_ip())));
        let now = Instant::now();
        assert!(check.poll(now).is_some());
        check.dismiss();
        assert!(check.poll(now).is_none());
    }

    #[test]
    fn ip_check_take_prompt_hands_over_ip_once() {
        let mut check = ip_check_with(Some(Ok(singapore_ip())));
        let now = Instant::now();
        check.poll(now);
        let prompt = check.take_prompt().expect("prompt expected");
        assert_eq!(prompt.ip.city.as_deref(), Some("Singapore"));
        assert!(check.take_prompt().is_none());
        assert!(check.poll(now).is_none());
    }
}
