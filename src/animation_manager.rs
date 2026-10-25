use crate::animation::{
    AnimationSystem, ChimneyPosition, FrameCommands, FrameContext, RenderLayer, TerminalSize, Wind,
    birds::BirdSystem,
    chimney::ChimneySmoke,
    christmas::lights::ChristmasLights,
    clouds::CloudSystem,
    fireflies::FireflySystem,
    fireworks::FireworksSystem,
    flyers::settings::FlyerSettings,
    flyers::system::FlyerSystem,
    fog::FogSystem,
    halloween::bats::HalloweenBats,
    halloween::pumpkins::HalloweenPumpkins,
    holidays::bonfire::HolidayBonfire,
    holidays::bunting::HolidayBunting,
    holidays::confetti::CarnivalConfetti,
    holidays::flagpole::HolidayFlag,
    holidays::ground::HolidayGround,
    holidays::hangings::HolidayHangings,
    holidays::holi::HoliColours,
    holidays::lanterns::WaisakLanterns,
    holidays::mudik::MudikCars,
    holidays::songkran::SongkranWater,
    kite::KiteSystem,
    leaves::FallingLeaves,
    moon::MoonSystem,
    raindrops::{Depth, RaindropSystem},
    snow::SnowSystem,
    stars::StarSystem,
    sunny::SunSystem,
    thunderstorm::ThunderstormSystem,
};
use crate::app_state::AppState;
use crate::events::ActiveEvents;
use crate::hide::HideSet;
use crate::render::TerminalRenderer;
use crate::scene::SceneLayout;
use crate::theme::colors::ElementColors;
use crate::weather::{FogIntensity, RainIntensity, SnowIntensity, WeatherConditions};
use rand::Rng;
use std::io;

pub struct FrameInputs<'a> {
    pub conditions: &'a WeatherConditions,
    pub state: &'a AppState,
    pub layout: &'a SceneLayout,
    pub colors: &'a ElementColors,
    pub events: &'a ActiveEvents,
}

pub struct AnimationManager {
    systems: Vec<Box<dyn AnimationSystem>>,
    show_leaves: bool,
}

pub(crate) const CALM_QUIET: &[&str] = &[
    "airplanes",
    "birds",
    "chimney_smoke",
    crate::animation::flyers::catalogue::KITE,
    crate::animation::fireworks::FIREWORKS,
    crate::animation::holidays::hangings::HANGINGS,
    crate::animation::holidays::ground::GROUND,
    crate::animation::holidays::lanterns::LANTERNS,
    crate::animation::holidays::mudik::MUDIK_CARS,
    crate::animation::holidays::flagpole::FLAGPOLE,
    crate::animation::holidays::bunting::BUNTING,
    crate::animation::holidays::bonfire::BONFIRE,
    crate::animation::holidays::holi::HOLI,
    crate::animation::holidays::songkran::SONGKRAN,
    crate::animation::holidays::confetti::CONFETTI,
];

fn silenced(id: &str, events: &ActiveEvents) -> bool {
    events.calm() && CALM_QUIET.contains(&id)
}

fn draw_order(systems: &[Box<dyn AnimationSystem>], layer: RenderLayer) -> Vec<usize> {
    let mut order: Vec<usize> = (0..systems.len())
        .filter(|index| systems[*index].layer() == layer)
        .collect();
    for index in order.clone() {
        let Some(target) = systems[index].drawn_before() else {
            continue;
        };
        let from = order.iter().position(|at| *at == index);
        let to = order.iter().position(|at| systems[*at].id() == target);
        if let (Some(from), Some(to)) = (from, to)
            && from > to
        {
            order.remove(from);
            order.insert(to, index);
        }
    }
    order
}

fn flyer_settings(hidden: &HideSet, flyers: &FlyerSettings) -> FlyerSettings {
    FlyerSettings {
        moon_hidden: hidden.hides_system("moon"),
        ..flyers.clone()
    }
}

impl AnimationManager {
    pub fn new(
        term_width: u16,
        term_height: u16,
        show_leaves: bool,
        hidden: &HideSet,
        flyers: &FlyerSettings,
        kite_preview: bool,
    ) -> Self {
        let mut systems: Vec<Box<dyn AnimationSystem>> = vec![
            // Background (code-defined order)
            Box::new(StarSystem::new(term_width, term_height)),
            Box::new(MoonSystem::new(term_width, term_height, None)),
            Box::new(FireflySystem::new(term_width, term_height)),
            Box::new(BirdSystem::new(term_width, term_height)),
            Box::new(SunSystem::new()),
            Box::new(CloudSystem::new(term_width, term_height)),
            Box::new(RaindropSystem::new(
                Depth::Far,
                term_width,
                term_height,
                RainIntensity::Light,
            )),
            Box::new(ThunderstormSystem::new(term_width, term_height)),
            Box::new(FireworksSystem::new()),
            Box::new(WaisakLanterns::new()),
            Box::new(FlyerSystem::new(flyer_settings(hidden, flyers))),
            Box::new(HalloweenBats::new()),
            // Post-scene
            Box::new(ChimneySmoke::new()),
            Box::new(HalloweenPumpkins::new()),
            Box::new(ChristmasLights::new()),
            Box::new(HolidayHangings::new()),
            Box::new(HolidayBunting::new()),
            Box::new(HolidayFlag::new()),
            Box::new(HolidayBonfire::new()),
            Box::new(HoliColours::new()),
            Box::new(SongkranWater::new()),
            Box::new(HolidayGround::new()),
            Box::new(MudikCars::new()),
            Box::new(KiteSystem::new(kite_preview)),
            // Foreground
            Box::new(RaindropSystem::new(
                Depth::Near,
                term_width,
                term_height,
                RainIntensity::Light,
            )),
            Box::new(SnowSystem::new(
                term_width,
                term_height,
                SnowIntensity::Light,
            )),
            Box::new(FogSystem::new(term_width, term_height, FogIntensity::Light)),
            Box::new(FallingLeaves::new(term_width, term_height)),
            Box::new(CarnivalConfetti::new()),
        ];
        systems.retain(|system| !hidden.hides_system(system.id()));

        debug_assert!(
            {
                let mut seen = std::collections::HashSet::<&'static str>::new();
                systems.iter().all(|s| seen.insert(s.id()))
            },
            "duplicate animation system ids"
        );

        Self {
            systems,
            show_leaves,
        }
    }

    pub fn on_resize(&mut self, width: u16, height: u16) {
        let size = TerminalSize { width, height };
        for system in &mut self.systems {
            system.on_resize(size);
        }
    }

    pub fn update_moon_phase(&mut self, phase: f64) {
        for system in &mut self.systems {
            system.on_moon_phase(phase);
        }
    }

    pub fn update_rain_intensity(&mut self, intensity: RainIntensity) {
        for system in &mut self.systems {
            system.on_rain_intensity(intensity);
        }
    }

    pub fn update_snow_intensity(&mut self, intensity: SnowIntensity) {
        for system in &mut self.systems {
            system.on_snow_intensity(intensity);
        }
    }

    pub fn update_wind(&mut self, speed_kmh: f32, direction_deg: f32) {
        let wind = Wind {
            speed_kmh,
            direction_deg,
        };
        for system in &mut self.systems {
            system.on_wind(wind);
        }
    }

    pub fn update_fog_intensity(&mut self, intensity: FogIntensity) {
        for system in &mut self.systems {
            system.on_fog_intensity(intensity);
        }
    }

    pub fn update_only(&mut self, frame: &FrameInputs<'_>, rng: &mut impl Rng) {
        let ctx = self.make_context(frame);
        if ctx.size.width == 0 || ctx.size.height == 0 {
            return;
        }
        let mut commands = FrameCommands::default();
        for system in &mut self.systems {
            if !silenced(system.id(), ctx.events) && system.is_active(&ctx) {
                system.update(&ctx, rng, &mut commands);
            }
        }
    }

    fn make_context<'a>(&self, frame: &FrameInputs<'a>) -> FrameContext<'a> {
        let chimney = frame
            .layout
            .chimney_pos
            .map(|pos| ChimneyPosition { x: pos.x, y: pos.y });

        FrameContext {
            size: TerminalSize {
                width: frame.layout.width,
                height: frame.layout.height,
            },
            horizon_y: frame.layout.ground_y,
            skyline_y: frame.layout.skyline_y,
            conditions: frame.conditions,
            state: frame.state,
            show_leaves: self.show_leaves,
            chimney,
            door: frame.layout.door,
            colors: frame.colors,
            events: frame.events,
        }
    }

    fn render_layer(
        &mut self,
        renderer: &mut TerminalRenderer,
        layer: RenderLayer,
        ctx: &FrameContext<'_>,
        rng: &mut impl Rng,
    ) -> io::Result<()> {
        if ctx.size.width == 0 || ctx.size.height == 0 {
            return Ok(());
        }

        let mut commands = FrameCommands::default();

        for index in draw_order(&self.systems, layer) {
            let system = &mut self.systems[index];
            if silenced(system.id(), ctx.events) || !system.is_active(ctx) {
                continue;
            }
            system.update(ctx, rng, &mut commands);
            system.render(renderer, ctx)?;
        }

        if commands.flash_screen {
            renderer.flash_screen(ctx.colors.lightning[0])?;
        }

        Ok(())
    }

    pub fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        layer: RenderLayer,
        frame: &FrameInputs<'_>,
        rng: &mut impl Rng,
    ) -> io::Result<()> {
        let ctx = self.make_context(frame);
        self.render_layer(renderer, layer, &ctx, rng)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hidden_systems_are_never_built() {
        let (hidden, _) = HideSet::parse(&["birds".to_string(), "airplanes".to_string()]);
        let manager =
            AnimationManager::new(120, 40, false, &hidden, &FlyerSettings::default(), false);
        let ids: Vec<&str> = manager.systems.iter().map(|s| s.id()).collect();
        assert!(!ids.contains(&"birds"));
        assert!(!ids.contains(&"airplanes"));
        assert!(ids.contains(&"rain"));
        assert_eq!(ids.len(), 27);
    }

    #[test]
    fn a_system_can_ask_to_be_drawn_before_another() {
        struct Stub(&'static str, Option<&'static str>);
        impl AnimationSystem for Stub {
            fn id(&self) -> &'static str {
                self.0
            }
            fn layer(&self) -> RenderLayer {
                RenderLayer::Background
            }
            fn drawn_before(&self) -> Option<&'static str> {
                self.1
            }
            fn update(&mut self, _: &FrameContext<'_>, _: &mut dyn Rng, _: &mut FrameCommands) {}
            fn render(&mut self, _: &mut TerminalRenderer, _: &FrameContext<'_>) -> io::Result<()> {
                Ok(())
            }
        }
        let systems: Vec<Box<dyn AnimationSystem>> = vec![
            Box::new(Stub("sun", None)),
            Box::new(Stub("clouds", None)),
            Box::new(Stub("rain", None)),
            Box::new(Stub("airplanes", Some("clouds"))),
        ];
        assert_eq!(draw_order(&systems, RenderLayer::Background), [0, 3, 1, 2]);
        assert!(draw_order(&systems, RenderLayer::Foreground).is_empty());
    }

    #[test]
    fn nothing_hidden_builds_every_system() {
        let manager = AnimationManager::new(
            120,
            40,
            false,
            &HideSet::default(),
            &FlyerSettings::default(),
            false,
        );
        assert_eq!(manager.systems.len(), 29);
    }

    #[test]
    fn hiding_the_moon_keeps_the_moon_biker_home() {
        let (hidden, _) = HideSet::parse(&["moon".to_string()]);
        assert!(flyer_settings(&hidden, &FlyerSettings::default()).moon_hidden);
        assert!(!flyer_settings(&HideSet::default(), &FlyerSettings::default()).moon_hidden);
    }

    #[test]
    fn nyepi_silences_the_sky_traffic_smoke_and_decorations() {
        let manager = AnimationManager::new(
            120,
            40,
            false,
            &HideSet::default(),
            &FlyerSettings::default(),
            false,
        );
        let ids: Vec<&str> = manager.systems.iter().map(|s| s.id()).collect();
        for id in CALM_QUIET {
            assert!(ids.contains(id), "{id} is not a system");
        }
        let at = |text: &str| {
            crate::events::resolve(
                chrono::NaiveDateTime::parse_from_str(text, "%Y-%m-%dT%H:%M").unwrap(),
                &crate::events::EventSettings::in_country("ID"),
            )
        };
        let nyepi = at("2027-03-08T21:00");
        let after = at("2027-03-09T07:00");
        for id in ["airplanes", "birds", "chimney_smoke", "kite", "fireworks"] {
            assert!(silenced(id, &nyepi), "{id}");
            assert!(!silenced(id, &after), "{id}");
        }
        for id in ["rain", "stars", "moon", "clouds", "fireflies"] {
            assert!(!silenced(id, &nyepi), "{id}");
        }
    }

    #[test]
    fn the_kite_is_its_own_system() {
        let (hidden, _) = HideSet::parse(&["kite".to_string()]);
        let manager =
            AnimationManager::new(120, 40, false, &hidden, &FlyerSettings::default(), false);
        let ids: Vec<&str> = manager.systems.iter().map(|s| s.id()).collect();
        assert!(!ids.contains(&"kite"));
        assert!(ids.contains(&"airplanes"));
        assert_eq!(ids.len(), 28);
    }
}
