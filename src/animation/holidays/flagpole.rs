use super::{Placed, place};
use crate::animation::flyers::{mask, mirror};
use crate::animation::{AnimationSystem, FrameCommands, FrameContext, RenderLayer, Wind};
use crate::events::{ActiveEvents, Look};
use crate::render::TerminalRenderer;
use crate::scene::DoorAnchor;
use crate::scene::world::house::House;
use rand::Rng;
use std::io;

pub const FLAGPOLE: &str = "holiday_flag";
pub const CREDIT: &str = "flags and koinobori drawn for this fork";

const FLAG_POLE_ROWS: i32 = 7;
const KOINOBORI_POLE_ROWS: i32 = 10;
const RIPPLE_PERIOD: i32 = 5;
const STILL_KMH: f32 = 3.0;
const BREEZE_KMH: f32 = 15.0;
const GALE_KMH: f32 = 30.0;
const PINWHEEL: [char; 4] = ['*', '+', 'x', '+'];
const PINWHEEL_FRAMES: u64 = 4;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Cloth {
    pub art: &'static [&'static str],
    pub mask: &'static [&'static str],
}

const fn cloth(art: &'static [&'static str], mask: &'static [&'static str]) -> Cloth {
    Cloth { art, mask }
}

const FLAGS: &[(&str, Cloth)] = &[
    (
        "ID",
        cloth(&["========", "========"], &["rrrrrrrr", "ffffffff"]),
    ),
    (
        "IN",
        cloth(
            &["=========", "====*====", "========="],
            &["ooooooooo", "ffffbffff", "ggggggggg"],
        ),
    ),
    (
        "PK",
        cloth(
            &["=========", "====(*===", "========="],
            &["ffggggggg", "ffggffggg", "ffggggggg"],
        ),
    ),
    (
        "BD",
        cloth(
            &["=========", "==@@@====", "========="],
            &["ggggggggg", "ggrrrgggg", "ggggggggg"],
        ),
    ),
    (
        "MY",
        cloth(
            &["=(*======", "=========", "========="],
            &["byybrrrrr", "bbbbfffff", "rrrrrrrrr"],
        ),
    ),
    (
        "CN",
        cloth(
            &["=*=.=====", "===.=====", "========="],
            &["ryryrrrrr", "rrryrrrrr", "rrrrrrrrr"],
        ),
    ),
    (
        "JP",
        cloth(
            &["=========", "===@@@===", "========="],
            &["fffffffff", "fffrrrfff", "fffffffff"],
        ),
    ),
    (
        "RU",
        cloth(
            &["=========", "=========", "========="],
            &["fffffffff", "bbbbbbbbb", "rrrrrrrrr"],
        ),
    ),
    (
        "ET",
        cloth(
            &["=========", "====@====", "========="],
            &["ggggggggg", "yyyybyyyy", "rrrrrrrrr"],
        ),
    ),
    (
        "US",
        cloth(
            &["*=*======", "=*=*=====", "========="],
            &["fbfbrrrrr", "bfbffffff", "rrrrrrrrr"],
        ),
    ),
    (
        "MX",
        cloth(
            &["=========", "====@====", "========="],
            &["gggfffrrr", "gggfnfrrr", "gggfffrrr"],
        ),
    ),
    (
        "NG",
        cloth(
            &["=========", "=========", "========="],
            &["gggfffggg", "gggfffggg", "gggfffggg"],
        ),
    ),
    (
        "BR",
        cloth(
            &["===/=\\===", "==<=@=>==", "===\\=/==="],
            &["gggyyyggg", "ggyybyygg", "gggyyyggg"],
        ),
    ),
    (
        "GB",
        cloth(
            &["\\==|=|==/", "=========", "/==|=|==\\"],
            &["fbbfrfbbf", "rrrrrrrrr", "fbbfrfbbf"],
        ),
    ),
];

const KOINOBORI: Cloth = cloth(
    &["=~=~=~=", "<'))))))><", "<')))))><", "<'))))><"],
    &["bryfgbr", "BfBBBBBBBB", "rfrrrrrrr", "cfcccccc"],
);

pub(crate) fn flag_of(country: &str) -> Option<&'static Cloth> {
    FLAGS
        .iter()
        .find(|(code, _)| code.eq_ignore_ascii_case(country))
        .map(|(_, cloth)| cloth)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Banner {
    Flag(&'static Cloth),
    Koinobori,
}

pub(crate) fn banner_for(events: &ActiveEvents) -> Option<Banner> {
    if events.has(Look::Koinobori) {
        return Some(Banner::Koinobori);
    }
    events
        .country_of(Look::Flag)
        .and_then(flag_of)
        .map(Banner::Flag)
}

pub(crate) fn flies_left(wind: Wind) -> bool {
    let toward_east = -wind.direction_deg.to_radians().sin();
    wind.speed_kmh >= STILL_KMH && toward_east < -0.2
}

fn ripple_step_frames(wind_kmh: f32) -> Option<u64> {
    if wind_kmh < STILL_KMH {
        None
    } else if wind_kmh < BREEZE_KMH {
        Some(9)
    } else if wind_kmh < GALE_KMH {
        Some(6)
    } else {
        Some(3)
    }
}

fn rippled(ch: char, fish: bool) -> char {
    match ch {
        '=' => '~',
        ')' if fish => '}',
        '(' if fish => '{',
        other => other,
    }
}

pub(crate) fn pole_x(door: DoorAnchor) -> i32 {
    i32::from(door.x) - i32::from(House::DOOR_X_OFFSET) + i32::from(House::WIDTH) + 2
}

pub(crate) fn placed(
    banner: Banner,
    door: DoorAnchor,
    width: u16,
    height: u16,
    wind: Wind,
    frame: u64,
) -> Vec<Placed> {
    let (cloth, pole_rows, fish) = match banner {
        Banner::Flag(cloth) => (cloth, FLAG_POLE_ROWS, false),
        Banner::Koinobori => (&KOINOBORI, KOINOBORI_POLE_ROWS, true),
    };
    let ground = i32::from(door.ground_y);
    let x = pole_x(door);
    let top = ground - pole_rows;
    let mut out = Vec::new();
    let finial = match banner {
        Banner::Flag(_) => ('o', 'y'),
        Banner::Koinobori => {
            let turn = ripple_step_frames(wind.speed_kmh)
                .map_or(0, |_| (frame / PINWHEEL_FRAMES) as usize % PINWHEEL.len());
            (PINWHEEL[turn], 'y')
        }
    };
    out.extend(place(x, top - 1, finial.0, finial.1, width, height));
    for y in top..ground {
        out.extend(place(x, y, '|', 'u', width, height));
    }
    let left = flies_left(wind);
    let phase = ripple_step_frames(wind.speed_kmh).map(|step| (frame / step) as i32);
    for (dy, (row, letters)) in cloth.art.iter().zip(cloth.mask).enumerate() {
        for (from_pole, (ch, letter)) in row.chars().zip(letters.chars()).enumerate() {
            let from_pole = from_pole as i32;
            let crest = phase.is_some_and(|phase| {
                (from_pole + dy as i32 - phase).rem_euclid(RIPPLE_PERIOD) == 0
            });
            let ch = if crest { rippled(ch, fish) } else { ch };
            let (cx, ch) = if left {
                (x - 1 - from_pole, mirror::mirror_char(ch).unwrap_or(ch))
            } else {
                (x + 1 + from_pole, ch)
            };
            out.extend(place(cx, top + dy as i32, ch, letter, width, height));
        }
    }
    out
}

pub struct HolidayFlag {
    frame: u64,
    wind: Wind,
}

impl HolidayFlag {
    pub fn new() -> Self {
        Self {
            frame: 0,
            wind: Wind {
                speed_kmh: 0.0,
                direction_deg: 270.0,
            },
        }
    }
}

impl Default for HolidayFlag {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationSystem for HolidayFlag {
    fn id(&self) -> &'static str {
        FLAGPOLE
    }

    fn layer(&self) -> RenderLayer {
        RenderLayer::PostScene
    }

    fn is_active(&self, ctx: &FrameContext<'_>) -> bool {
        ctx.door.is_some() && banner_for(ctx.events).is_some()
    }

    fn on_wind(&mut self, wind: Wind) {
        self.wind = wind;
    }

    fn update(
        &mut self,
        _ctx: &FrameContext<'_>,
        _rng: &mut dyn Rng,
        _commands: &mut FrameCommands,
    ) {
        self.frame = self.frame.wrapping_add(1);
    }

    fn render(
        &mut self,
        renderer: &mut TerminalRenderer,
        ctx: &FrameContext<'_>,
    ) -> io::Result<()> {
        let (Some(door), Some(banner)) = (ctx.door, banner_for(ctx.events)) else {
            return Ok(());
        };
        let wind = if ctx.state.is_offline {
            Wind {
                speed_kmh: 0.0,
                ..self.wind
            }
        } else {
            self.wind
        };
        for cell in placed(
            banner,
            door,
            ctx.size.width,
            ctx.size.height,
            wind,
            self.frame,
        ) {
            let color =
                mask::colour(cell.letter, &ctx.colors.palette).unwrap_or(ctx.colors.palette[0]);
            let ch = crate::glyphs::swap(ctx.state.glyphs, crate::glyphs::Element::Cloth, cell.ch);
            renderer.render_char(cell.x, cell.y, ch, color)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::holidays::assert_mask_fits;
    use crate::events::{self, EventSettings};
    use chrono::NaiveDateTime;

    fn at(text: &str, country: &'static str) -> ActiveEvents {
        events::resolve(
            NaiveDateTime::parse_from_str(text, "%Y-%m-%dT%H:%M").unwrap(),
            &EventSettings::in_country(country),
        )
    }

    fn door() -> DoorAnchor {
        DoorAnchor {
            x: 39,
            width: 4,
            ground_y: 25,
        }
    }

    fn wind(speed_kmh: f32, direction_deg: f32) -> Wind {
        Wind {
            speed_kmh,
            direction_deg,
        }
    }

    #[test]
    fn every_cloth_has_a_mask_that_copies_it() {
        for (code, cloth) in FLAGS
            .iter()
            .map(|(code, cloth)| (*code, cloth))
            .chain([("koinobori", &KOINOBORI)])
        {
            assert_mask_fits(code, &cloth.art.join("\n"), &cloth.mask.join("\n"));
            let widths: Vec<usize> = cloth.art.iter().map(|row| row.len()).collect();
            if code != "koinobori" {
                assert!(widths.windows(2).all(|pair| pair[0] == pair[1]), "{code}");
            }
        }
    }

    #[test]
    fn every_flag_day_has_its_country_flag() {
        for def in events::all().filter(|def| def.looks.contains(&Look::Flag)) {
            let country = def.country().expect("flag days belong to a country");
            assert!(flag_of(country).is_some(), "{} has no flag", def.id);
        }
    }

    #[test]
    fn indonesia_flies_merah_putih_all_august_and_kites_on_the_seventeenth() {
        let early = at("2026-08-03T10:00", "ID");
        let day = at("2026-08-17T10:00", "ID");
        let september = at("2026-09-01T10:00", "ID");
        assert_eq!(banner_for(&early), flag_of("ID").map(Banner::Flag));
        assert!(early.has(Look::Bunting) && !early.has(Look::Kites));
        assert!(day.has(Look::Kites));
        assert_eq!(
            day.greeting().map(|(_, text)| text).as_deref(),
            Some("Happy 81st Independence Day, Indonesia")
        );
        assert_eq!(
            at("2027-08-17T10:00", "ID")
                .greeting()
                .map(|(_, text)| text)
                .as_deref(),
            Some("Happy 82nd Independence Day, Indonesia")
        );
        assert_eq!(banner_for(&september), None);
    }

    #[test]
    fn other_countries_fly_their_own_flag() {
        assert_eq!(
            banner_for(&at("2026-08-15T10:00", "IN")),
            flag_of("IN").map(Banner::Flag)
        );
        assert_eq!(
            banner_for(&at("2026-06-12T10:00", "RU")),
            flag_of("RU").map(Banner::Flag)
        );
        assert_eq!(
            banner_for(&at("2026-08-15T10:00", "ID")),
            flag_of("ID").map(Banner::Flag)
        );
    }

    #[test]
    fn childrens_day_flies_koinobori() {
        assert_eq!(
            banner_for(&at("2027-05-05T10:00", "JP")),
            Some(Banner::Koinobori)
        );
        assert_eq!(banner_for(&at("2027-05-07T10:00", "JP")), None);
    }

    #[test]
    fn the_cloth_flies_downwind_from_the_pole() {
        let flag = Banner::Flag(flag_of("ID").unwrap());
        let x = pole_x(door());
        let from_west = placed(flag, door(), 200, 32, wind(12.0, 270.0), 0);
        let from_east = placed(flag, door(), 200, 32, wind(12.0, 90.0), 0);
        let cloth_xs = |cells: &[Placed]| -> Vec<i32> {
            cells
                .iter()
                .filter(|cell| cell.letter == 'r')
                .map(|cell| i32::from(cell.x))
                .collect()
        };
        assert!(cloth_xs(&from_west).iter().all(|&cx| cx > x));
        assert!(cloth_xs(&from_east).iter().all(|&cx| cx < x));
        let calm = placed(flag, door(), 200, 32, wind(0.0, 90.0), 0);
        assert!(cloth_xs(&calm).iter().all(|&cx| cx > x));
    }

    #[test]
    fn the_cloth_ripples_only_in_wind() {
        let flag = Banner::Flag(flag_of("ID").unwrap());
        let tildes = |speed: f32, frame: u64| {
            placed(flag, door(), 200, 32, wind(speed, 270.0), frame)
                .iter()
                .filter(|cell| cell.ch == '~')
                .count()
        };
        assert_eq!(tildes(0.0, 0), 0);
        assert!(tildes(12.0, 0) > 0);
        let a = placed(flag, door(), 200, 32, wind(12.0, 270.0), 0);
        let b = placed(flag, door(), 200, 32, wind(12.0, 270.0), 9);
        assert_ne!(a, b, "the ripple travels");
    }

    #[test]
    fn the_pole_stands_on_the_ground_by_the_fence() {
        let cells = placed(
            Banner::Flag(flag_of("ID").unwrap()),
            door(),
            200,
            32,
            wind(0.0, 0.0),
            0,
        );
        let x = pole_x(door()) as u16;
        let pole: Vec<u16> = cells
            .iter()
            .filter(|cell| cell.x == x && cell.ch == '|')
            .map(|cell| cell.y)
            .collect();
        assert_eq!(pole.iter().max(), Some(&24));
        assert_eq!(pole.len(), FLAG_POLE_ROWS as usize);
        assert!(cells.iter().any(|cell| cell.x == x && cell.ch == 'o'));
    }

    #[test]
    fn a_narrow_pane_clips_instead_of_wrapping() {
        let cells = placed(Banner::Koinobori, door(), 90, 32, wind(20.0, 270.0), 0);
        assert!(cells.iter().all(|cell| cell.x < 90));
    }
}
