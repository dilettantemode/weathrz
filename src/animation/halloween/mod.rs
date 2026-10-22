pub mod bats;
pub mod pumpkins;

#[cfg(test)]
pub(crate) mod test_support {
    use crate::animation::{FrameContext, TerminalSize};
    use crate::app_state::AppState;
    use crate::config::LocationDisplay;
    use crate::events::{self, ActiveEvents, EventSettings};
    use crate::scene::DoorAnchor;
    use crate::theme::colors::ElementColors;
    use crate::weather::types::CelestialEvents;
    use crate::weather::{WeatherCondition, WeatherData, WeatherLocation, WeatherUnits};
    use chrono::NaiveDateTime;

    pub(crate) struct Fixture {
        pub state: AppState,
        pub colors: ElementColors,
        pub events: ActiveEvents,
        pub size: TerminalSize,
        pub horizon_y: u16,
        pub skyline_y: u16,
        pub door: Option<DoorAnchor>,
    }

    impl Fixture {
        pub(crate) fn new(condition: WeatherCondition, is_day: bool) -> Self {
            let location = WeatherLocation {
                latitude: -6.2,
                longitude: 106.8,
                elevation: None,
            };
            let mut state = AppState::new(
                location,
                None,
                LocationDisplay::Coordinates,
                false,
                WeatherUnits::metric(),
            );
            state.update_weather(WeatherData {
                condition,
                temperature: 27.0,
                precipitation: 0.0,
                wind_speed: 2.0,
                wind_direction: 0.0,
                sun: CelestialEvents::from_bool(is_day),
                moon_phase: None,
                humidity: None,
                timestamp: String::new(),
                attribution: String::new(),
                feels_like: None,
                uv_index: None,
                outlook: Vec::new(),
            });
            let halloween_night =
                NaiveDateTime::parse_from_str("2026-10-31T21:00", "%Y-%m-%dT%H:%M").unwrap();
            Self {
                state,
                colors: crate::theme::catalogue::default_colors().night,
                events: events::resolve(halloween_night, &EventSettings::in_country("ID")),
                size: TerminalSize {
                    width: 110,
                    height: 32,
                },
                horizon_y: 25,
                skyline_y: 15,
                door: Some(DoorAnchor {
                    x: 39,
                    width: 4,
                    ground_y: 25,
                }),
            }
        }

        pub(crate) fn ctx(&self) -> FrameContext<'_> {
            FrameContext {
                size: self.size,
                horizon_y: self.horizon_y,
                skyline_y: self.skyline_y,
                conditions: &self.state.weather_conditions,
                state: &self.state,
                show_leaves: false,
                chimney: None,
                door: self.door,
                colors: &self.colors,
                events: &self.events,
            }
        }
    }
}
