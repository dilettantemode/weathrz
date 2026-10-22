use chrono::{DateTime, Duration, Timelike, Utc};

const UNIX_EPOCH_JULIAN_DAY: f64 = 2_440_587.5;
const J2000_JULIAN_DAY: f64 = 2_451_545.0;
const SECONDS_PER_DAY: f64 = 86_400.0;
const DAYS_PER_CENTURY: f64 = 36_525.0;
const MINUTES_PER_DAY: f64 = 1_440.0;
const SUNRISE_ZENITH_DEG: f64 = 90.833;
const SCAN_STEP_MINUTES: i64 = 10;
const SCAN_HOURS: i64 = 26;

fn sin_deg(degrees: f64) -> f64 {
    degrees.to_radians().sin()
}

fn cos_deg(degrees: f64) -> f64 {
    degrees.to_radians().cos()
}

fn tan_deg(degrees: f64) -> f64 {
    degrees.to_radians().tan()
}

fn days_since_j2000(at: DateTime<Utc>) -> f64 {
    let seconds = at.timestamp() as f64 + f64::from(at.timestamp_subsec_nanos()) / 1e9;
    seconds / SECONDS_PER_DAY + UNIX_EPOCH_JULIAN_DAY - J2000_JULIAN_DAY
}

pub fn moon_elongation_deg(at: DateTime<Utc>) -> f64 {
    let d = days_since_j2000(at);
    let t = d / DAYS_PER_CENTURY;
    let moon_longitude = 218.32 + 481_267.881 * t + 6.29 * sin_deg(135.0 + 477_198.87 * t)
        - 1.27 * sin_deg(259.3 - 413_335.36 * t)
        + 0.66 * sin_deg(235.7 + 890_534.22 * t)
        + 0.21 * sin_deg(269.9 + 954_397.74 * t)
        - 0.19 * sin_deg(357.5 + 35_999.05 * t)
        - 0.11 * sin_deg(186.5 + 966_404.03 * t);
    let sun_anomaly = 357.528 + 0.985_600_3 * d;
    let sun_longitude = 280.460
        + 0.985_647_4 * d
        + 1.915 * sin_deg(sun_anomaly)
        + 0.020 * sin_deg(2.0 * sun_anomaly);
    let elongation = (moon_longitude - sun_longitude).rem_euclid(360.0);
    if elongation >= 360.0 { 0.0 } else { elongation }
}

pub fn moon_phase(at: DateTime<Utc>) -> f64 {
    moon_elongation_deg(at) / 360.0
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SunSky {
    pub elevation_deg: f64,
    pub azimuth_deg: f64,
    pub noon_elevation_deg: f64,
    pub day_progress: Option<f64>,
}

pub fn sun_is_up(latitude: f64, longitude: f64, at: DateTime<Utc>) -> bool {
    sun_sky(latitude, longitude, at).elevation_deg > 90.0 - SUNRISE_ZENITH_DEG
}

pub fn sun_sky(latitude: f64, longitude: f64, at: DateTime<Utc>) -> SunSky {
    let t = days_since_j2000(at) / DAYS_PER_CENTURY;
    let mean_longitude = (280.46646 + t * (36_000.769_83 + t * 0.000_303_2)).rem_euclid(360.0);
    let mean_anomaly = 357.52911 + t * (35_999.050_29 - 0.000_153_7 * t);
    let eccentricity = 0.016_708_634 - t * (0.000_042_037 + 0.000_000_126_7 * t);
    let centre = sin_deg(mean_anomaly) * (1.914_602 - t * (0.004_817 + 0.000_014 * t))
        + sin_deg(2.0 * mean_anomaly) * (0.019_993 - 0.000_101 * t)
        + sin_deg(3.0 * mean_anomaly) * 0.000_289;
    let omega = 125.04 - 1_934.136 * t;
    let apparent_longitude = mean_longitude + centre - 0.005_69 - 0.004_78 * sin_deg(omega);
    let mean_obliquity =
        23.0 + (26.0 + (21.448 - t * (46.815 + t * (0.000_59 - t * 0.001_813))) / 60.0) / 60.0;
    let obliquity = mean_obliquity + 0.002_56 * cos_deg(omega);
    let declination = (sin_deg(obliquity) * sin_deg(apparent_longitude))
        .asin()
        .to_degrees();
    let y = tan_deg(obliquity / 2.0).powi(2);
    let equation_of_time = 4.0
        * (y * sin_deg(2.0 * mean_longitude) - 2.0 * eccentricity * sin_deg(mean_anomaly)
            + 4.0 * eccentricity * y * sin_deg(mean_anomaly) * cos_deg(2.0 * mean_longitude)
            - 0.5 * y * y * sin_deg(4.0 * mean_longitude)
            - 1.25 * eccentricity * eccentricity * sin_deg(2.0 * mean_anomaly))
        .to_degrees();
    let utc_minutes = f64::from(at.num_seconds_from_midnight()) / 60.0;
    let solar_minutes =
        (utc_minutes + equation_of_time + 4.0 * longitude).rem_euclid(MINUTES_PER_DAY);
    let hour_angle = solar_minutes / 4.0 - 180.0;
    let cos_zenith = sin_deg(latitude) * sin_deg(declination)
        + cos_deg(latitude) * cos_deg(declination) * cos_deg(hour_angle);
    let elevation = 90.0 - cos_zenith.clamp(-1.0, 1.0).acos().to_degrees();
    let azimuth = (sin_deg(hour_angle)
        .atan2(cos_deg(hour_angle) * sin_deg(latitude) - tan_deg(declination) * cos_deg(latitude))
        .to_degrees()
        + 180.0)
        .rem_euclid(360.0);
    let cos_sunset = (cos_deg(SUNRISE_ZENITH_DEG) - sin_deg(latitude) * sin_deg(declination))
        / (cos_deg(latitude) * cos_deg(declination));
    let day_progress = (cos_sunset < 1.0).then(|| {
        let sunset_hour_angle = if cos_sunset <= -1.0 {
            180.0
        } else {
            cos_sunset.acos().to_degrees()
        };
        (hour_angle + sunset_hour_angle) / (2.0 * sunset_hour_angle)
    });
    SunSky {
        elevation_deg: elevation,
        azimuth_deg: azimuth,
        noon_elevation_deg: 90.0 - (latitude - declination).abs(),
        day_progress,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MoonSky {
    pub altitude_deg: f64,
    pub azimuth_deg: f64,
    pub limb_deg: f64,
    pub elongation_deg: f64,
    pub progress: Option<f64>,
    pub peak_deg: f64,
}

struct Equatorial {
    right_ascension: f64,
    declination: f64,
}

fn obliquity(t: f64) -> f64 {
    23.4393 - 0.013 * t
}

fn to_equatorial(longitude: f64, latitude: f64, t: f64) -> Equatorial {
    let tilt = obliquity(t);
    let declination = (sin_deg(latitude) * cos_deg(tilt)
        + cos_deg(latitude) * sin_deg(tilt) * sin_deg(longitude))
    .clamp(-1.0, 1.0)
    .asin()
    .to_degrees();
    let right_ascension = (sin_deg(longitude) * cos_deg(tilt) - tan_deg(latitude) * sin_deg(tilt))
        .atan2(cos_deg(longitude))
        .to_degrees()
        .rem_euclid(360.0);
    Equatorial {
        right_ascension,
        declination,
    }
}

fn moon_place(t: f64) -> (Equatorial, f64) {
    let longitude = 218.32 + 481_267.881 * t + 6.29 * sin_deg(135.0 + 477_198.87 * t)
        - 1.27 * sin_deg(259.3 - 413_335.36 * t)
        + 0.66 * sin_deg(235.7 + 890_534.22 * t)
        + 0.21 * sin_deg(269.9 + 954_397.74 * t)
        - 0.19 * sin_deg(357.5 + 35_999.05 * t)
        - 0.11 * sin_deg(186.5 + 966_404.03 * t);
    let latitude = 5.13 * sin_deg(93.3 + 483_202.03 * t) + 0.28 * sin_deg(228.2 + 960_400.87 * t)
        - 0.28 * sin_deg(318.3 + 6_003.18 * t)
        - 0.17 * sin_deg(217.6 - 407_332.20 * t);
    let parallax = 0.9508
        + 0.0518 * cos_deg(135.0 + 477_198.87 * t)
        + 0.0095 * cos_deg(259.3 - 413_335.36 * t)
        + 0.0078 * cos_deg(235.7 + 890_534.22 * t)
        + 0.0028 * cos_deg(269.9 + 954_397.74 * t);
    (to_equatorial(longitude, latitude, t), parallax)
}

fn sun_place(d: f64, t: f64) -> Equatorial {
    let anomaly = 357.528 + 0.985_600_3 * d;
    let longitude =
        280.460 + 0.985_647_4 * d + 1.915 * sin_deg(anomaly) + 0.020 * sin_deg(2.0 * anomaly);
    to_equatorial(longitude, 0.0, t)
}

fn hour_angle(d: f64, longitude: f64, right_ascension: f64) -> f64 {
    let sidereal = (280.460_618_37 + 360.985_647_366_29 * d + longitude).rem_euclid(360.0);
    (sidereal - right_ascension).rem_euclid(360.0)
}

pub fn moon_altitude(latitude: f64, longitude: f64, at: DateTime<Utc>) -> f64 {
    let d = days_since_j2000(at);
    let t = d / DAYS_PER_CENTURY;
    let (moon, parallax) = moon_place(t);
    let h = hour_angle(d, longitude, moon.right_ascension);
    let geocentric = (sin_deg(latitude) * sin_deg(moon.declination)
        + cos_deg(latitude) * cos_deg(moon.declination) * cos_deg(h))
    .clamp(-1.0, 1.0)
    .asin()
    .to_degrees();
    geocentric - parallax * cos_deg(geocentric)
}

fn crossing(
    altitude: &impl Fn(DateTime<Utc>) -> f64,
    mut below: DateTime<Utc>,
    mut above: DateTime<Utc>,
) -> DateTime<Utc> {
    for _ in 0..12 {
        let middle = below + (above - below) / 2;
        if altitude(middle) > 0.0 {
            above = middle;
        } else {
            below = middle;
        }
    }
    above
}

fn up_span(
    altitude: impl Fn(DateTime<Utc>) -> f64,
    at: DateTime<Utc>,
) -> Option<(DateTime<Utc>, DateTime<Utc>, f64)> {
    if altitude(at) <= 0.0 {
        return None;
    }
    let step = Duration::minutes(SCAN_STEP_MINUTES);
    let steps = SCAN_HOURS * 60 / SCAN_STEP_MINUTES;
    let mut peak = altitude(at);
    let mut rise = None;
    let mut probe = at;
    for _ in 0..steps {
        let earlier = probe - step;
        let height = altitude(earlier);
        if height <= 0.0 {
            rise = Some(crossing(&altitude, earlier, probe));
            break;
        }
        peak = peak.max(height);
        probe = earlier;
    }
    let mut set = None;
    let mut probe = at;
    for _ in 0..steps {
        let later = probe + step;
        let height = altitude(later);
        if height <= 0.0 {
            set = Some(crossing(&altitude, later, probe));
            break;
        }
        peak = peak.max(height);
        probe = later;
    }
    Some((rise?, set?, peak))
}

pub fn moon_sky(latitude: f64, longitude: f64, at: DateTime<Utc>) -> MoonSky {
    let d = days_since_j2000(at);
    let t = d / DAYS_PER_CENTURY;
    let (moon, _) = moon_place(t);
    let sun = sun_place(d, t);
    let h = hour_angle(d, longitude, moon.right_ascension);
    let altitude_deg = moon_altitude(latitude, longitude, at);
    let azimuth_deg = (sin_deg(h)
        .atan2(cos_deg(h) * sin_deg(latitude) - tan_deg(moon.declination) * cos_deg(latitude))
        .to_degrees()
        + 180.0)
        .rem_euclid(360.0);
    let delta = sun.right_ascension - moon.right_ascension;
    let bright = (cos_deg(sun.declination) * sin_deg(delta))
        .atan2(
            sin_deg(sun.declination) * cos_deg(moon.declination)
                - cos_deg(sun.declination) * sin_deg(moon.declination) * cos_deg(delta),
        )
        .to_degrees();
    let parallactic = sin_deg(h)
        .atan2(
            tan_deg(latitude) * cos_deg(moon.declination) - sin_deg(moon.declination) * cos_deg(h),
        )
        .to_degrees();
    let span = up_span(|when| moon_altitude(latitude, longitude, when), at);
    let (progress, peak_deg) = match span {
        Some((rise, set, peak)) => (
            Some(
                ((at - rise).num_seconds() as f64 / (set - rise).num_seconds().max(1) as f64)
                    .clamp(0.0, 1.0),
            ),
            peak.max(1.0),
        ),
        None if altitude_deg > 0.0 => {
            let signed = if h > 180.0 { h - 360.0 } else { h };
            let transit = 90.0 - (latitude - moon.declination).abs();
            (
                Some(((signed + 180.0) / 360.0).clamp(0.0, 1.0)),
                transit.max(altitude_deg).max(1.0),
            )
        }
        None => (None, altitude_deg.max(1.0)),
    };
    MoonSky {
        altitude_deg,
        azimuth_deg,
        limb_deg: (bright - parallactic).rem_euclid(360.0),
        elongation_deg: moon_elongation_deg(at),
        progress,
        peak_deg,
    }
}

pub fn sunset_after(latitude: f64, longitude: f64, from: DateTime<Utc>) -> Option<DateTime<Utc>> {
    let altitude = |when: DateTime<Utc>| {
        sun_sky(latitude, longitude, when).elevation_deg - (90.0 - SUNRISE_ZENITH_DEG)
    };
    let step = Duration::minutes(SCAN_STEP_MINUTES);
    let mut probe = from;
    for _ in 0..(24 * 60 / SCAN_STEP_MINUTES) {
        let later = probe + step;
        if altitude(probe) > 0.0 && altitude(later) <= 0.0 {
            return Some(crossing(&altitude, later, probe));
        }
        probe = later;
    }
    None
}

const DELTA_T_SECONDS: f64 = 69.0;
const SYNODIC_DAYS: f64 = 29.530_588_861;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    New,
    Full,
}

fn from_julian(jde: f64) -> DateTime<Utc> {
    let seconds = (jde - UNIX_EPOCH_JULIAN_DAY) * SECONDS_PER_DAY - DELTA_T_SECONDS;
    DateTime::from_timestamp(seconds.floor() as i64, 0).unwrap_or_default()
}

pub fn lunation(k: i64, phase: Phase) -> DateTime<Utc> {
    let k = k as f64
        + match phase {
            Phase::New => 0.0,
            Phase::Full => 0.5,
        };
    let t = k / 1236.85;
    let t2 = t * t;
    let jde = 2_451_550.097_66 + SYNODIC_DAYS * k + 0.000_154_37 * t2 - 0.000_000_150 * t2 * t
        + 0.000_000_000_73 * t2 * t2;
    let e = 1.0 - 0.002_516 * t - 0.000_007_4 * t2;
    let m = 2.5534 + 29.105_356_70 * k - 0.000_001_4 * t2 - 0.000_000_11 * t2 * t;
    let mp = 201.5643 + 385.816_935_28 * k + 0.010_758_2 * t2 + 0.000_012_38 * t2 * t
        - 0.000_000_058 * t2 * t2;
    let f = 160.7108 + 390.670_502_84 * k - 0.001_611_8 * t2 - 0.000_002_27 * t2 * t
        + 0.000_000_011 * t2 * t2;
    let omega = 124.7746 - 1.563_755_88 * k + 0.002_067_2 * t2 + 0.000_002_15 * t2 * t;
    let (first, second, third, fourth) = match phase {
        Phase::New => (-0.40720, 0.17241, 0.01608, 0.01039),
        Phase::Full => (-0.40614, 0.17302, 0.01614, 0.01043),
    };
    let (fifth, sixth, seventh) = match phase {
        Phase::New => (0.00739, -0.00514, 0.00208),
        Phase::Full => (0.00734, -0.00515, 0.00209),
    };
    let periodic = first * sin_deg(mp)
        + second * e * sin_deg(m)
        + third * sin_deg(2.0 * mp)
        + fourth * sin_deg(2.0 * f)
        + fifth * e * sin_deg(mp - m)
        + sixth * e * sin_deg(mp + m)
        + seventh * e * e * sin_deg(2.0 * m)
        - 0.00111 * sin_deg(mp - 2.0 * f)
        - 0.00057 * sin_deg(mp + 2.0 * f)
        + 0.00056 * e * sin_deg(2.0 * mp + m)
        - 0.00042 * sin_deg(3.0 * mp)
        + 0.00042 * e * sin_deg(m + 2.0 * f)
        + 0.00038 * e * sin_deg(m - 2.0 * f)
        - 0.00024 * e * sin_deg(2.0 * mp - m)
        - 0.00017 * sin_deg(omega)
        - 0.00007 * sin_deg(mp + 2.0 * m)
        + 0.00004 * sin_deg(2.0 * mp - 2.0 * f)
        + 0.00004 * sin_deg(3.0 * m)
        + 0.00003 * sin_deg(mp + m - 2.0 * f)
        + 0.00003 * sin_deg(2.0 * mp + 2.0 * f)
        - 0.00003 * sin_deg(mp + m + 2.0 * f)
        + 0.00003 * sin_deg(mp - m + 2.0 * f)
        - 0.00002 * sin_deg(mp - m - 2.0 * f)
        - 0.00002 * sin_deg(3.0 * mp + m)
        + 0.00002 * sin_deg(4.0 * mp);
    let planets = [
        (0.000_325, 299.77 + 0.107_408 * k - 0.009_173 * t2),
        (0.000_165, 251.88 + 0.016_321 * k),
        (0.000_164, 251.83 + 26.651_886 * k),
        (0.000_126, 349.42 + 36.412_478 * k),
        (0.000_110, 84.66 + 18.206_239 * k),
        (0.000_062, 141.74 + 53.303_771 * k),
        (0.000_060, 207.14 + 2.453_732 * k),
        (0.000_056, 154.84 + 7.306_860 * k),
        (0.000_047, 34.52 + 27.261_239 * k),
        (0.000_042, 207.19 + 0.121_824 * k),
        (0.000_040, 291.34 + 1.844_379 * k),
        (0.000_037, 161.72 + 24.198_154 * k),
        (0.000_035, 239.56 + 25.513_099 * k),
        (0.000_023, 331.55 + 3.592_518 * k),
    ]
    .iter()
    .map(|(size, angle)| size * sin_deg(*angle))
    .sum::<f64>();
    from_julian(jde + periodic + planets)
}

pub fn lunation_near(at: DateTime<Utc>) -> i64 {
    let years = days_since_j2000(at) / 365.25;
    (years * 12.3685).round() as i64
}

pub fn sun_longitude(at: DateTime<Utc>) -> f64 {
    let t = days_since_j2000(at) / DAYS_PER_CENTURY;
    let mean_longitude = (280.46646 + t * (36_000.769_83 + t * 0.000_303_2)).rem_euclid(360.0);
    let mean_anomaly = 357.52911 + t * (35_999.050_29 - 0.000_153_7 * t);
    let centre = sin_deg(mean_anomaly) * (1.914_602 - t * (0.004_817 + 0.000_014 * t))
        + sin_deg(2.0 * mean_anomaly) * (0.019_993 - 0.000_101 * t)
        + sin_deg(3.0 * mean_anomaly) * 0.000_289;
    let omega = 125.04 - 1_934.136 * t;
    (mean_longitude + centre - 0.005_69 - 0.004_78 * sin_deg(omega)).rem_euclid(360.0)
}

pub fn sun_reaches(longitude: f64, around: DateTime<Utc>) -> DateTime<Utc> {
    let ahead =
        |at: DateTime<Utc>| (sun_longitude(at) - longitude + 540.0).rem_euclid(360.0) - 180.0;
    let mut low = around - Duration::days(20);
    let mut high = around + Duration::days(20);
    for _ in 0..40 {
        let middle = low + (high - low) / 2;
        if ahead(middle) < 0.0 {
            low = middle;
        } else {
            high = middle;
        }
    }
    low
}

pub fn moon_sun_separation(at: DateTime<Utc>) -> f64 {
    let d = days_since_j2000(at);
    let t = d / DAYS_PER_CENTURY;
    let (moon, _) = moon_place(t);
    let sun = sun_place(d, t);
    (sin_deg(moon.declination) * sin_deg(sun.declination)
        + cos_deg(moon.declination)
            * cos_deg(sun.declination)
            * cos_deg(moon.right_ascension - sun.right_ascension))
    .clamp(-1.0, 1.0)
    .acos()
    .to_degrees()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    const TOLERANCE_DEG: f64 = 0.6;

    const CATALOG: [(i32, u32, u32, u32, u32, f64); 15] = [
        (2008, 12, 27, 12, 23, 0.0),
        (2010, 8, 10, 3, 8, 0.0),
        (2012, 12, 28, 10, 21, 180.0),
        (2016, 8, 2, 20, 45, 0.0),
        (2025, 12, 20, 1, 43, 0.0),
        (2026, 10, 3, 13, 25, 270.0),
        (2026, 10, 10, 15, 50, 0.0),
        (2026, 10, 18, 16, 13, 90.0),
        (2026, 10, 26, 4, 12, 180.0),
        (2026, 11, 1, 20, 28, 270.0),
        (2054, 8, 10, 10, 6, 90.0),
        (2061, 7, 17, 1, 11, 0.0),
        (2066, 5, 16, 10, 1, 270.0),
        (2089, 9, 19, 21, 56, 180.0),
        (2097, 7, 2, 17, 6, 270.0),
    ];

    fn utc(y: i32, mo: u32, d: u32, h: u32, mi: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(y, mo, d, h, mi, 0).unwrap()
    }

    fn angular_gap(a: f64, b: f64) -> f64 {
        ((a - b + 540.0) % 360.0 - 180.0).abs()
    }

    #[test]
    fn elongation_matches_espenak_catalog_within_about_an_hour() {
        for (y, mo, d, h, mi, target) in CATALOG {
            let got = moon_elongation_deg(utc(y, mo, d, h, mi));
            assert!(
                angular_gap(got, target) < TOLERANCE_DEG,
                "{y}-{mo:02}-{d:02} {h:02}:{mi:02} UT: expected {target}, got {got:.3}"
            );
        }
    }

    #[test]
    fn phase_is_a_fraction_of_the_cycle() {
        for (y, mo, d, h, mi, _) in CATALOG {
            let phase = moon_phase(utc(y, mo, d, h, mi));
            assert!((0.0..1.0).contains(&phase), "{phase}");
        }
    }

    #[test]
    fn full_moon_is_half_phase() {
        assert!((moon_phase(utc(2026, 10, 26, 4, 12)) - 0.5).abs() < 0.002);
    }

    #[test]
    fn jakarta_morning_of_2026_10_13_is_a_young_waxing_crescent() {
        let phase = moon_phase(utc(2026, 10, 13, 3, 0));
        assert!((0.07..0.09).contains(&phase), "{phase}");
        assert_eq!((phase * 8.0).round() as usize % 8, 1);
    }

    const JAKARTA: (f64, f64) = (-6.2088, 106.8456);

    fn progress_at(place: (f64, f64), at: DateTime<Utc>) -> f64 {
        sun_sky(place.0, place.1, at)
            .day_progress
            .expect("a sunrise")
    }

    #[test]
    fn jakarta_sun_at_five_in_the_afternoon_of_19_october_2026() {
        let sky = sun_sky(JAKARTA.0, JAKARTA.1, utc(2026, 10, 19, 10, 0));
        assert!((sky.elevation_deg - 10.2).abs() < 0.3, "{sky:?}");
        assert!((sky.azimuth_deg - 261.0).abs() < 1.0, "{sky:?}");
        assert!((sky.noon_elevation_deg - 86.1).abs() < 0.3, "{sky:?}");
        let progress = progress_at(JAKARTA, utc(2026, 10, 19, 10, 0));
        assert!((progress - 691.0 / 736.0).abs() < 0.01, "{progress}");
    }

    #[test]
    fn jakarta_sunrise_noon_and_sunset_on_19_october_2026() {
        for (at, expected) in [
            (utc(2026, 10, 18, 22, 29), 0.0),
            (utc(2026, 10, 19, 4, 37), 0.5),
            (utc(2026, 10, 19, 10, 45), 1.0),
        ] {
            let progress = progress_at(JAKARTA, at);
            assert!((progress - expected).abs() < 0.01, "{at} {progress}");
        }
        let noon = sun_sky(JAKARTA.0, JAKARTA.1, utc(2026, 10, 19, 4, 37));
        assert!(
            (noon.elevation_deg - noon.noon_elevation_deg).abs() < 0.2,
            "{noon:?}"
        );
    }

    #[test]
    fn the_sun_is_up_between_sunrise_and_sunset() {
        for (at, up) in [
            (utc(2026, 10, 30, 22, 10), false),
            (utc(2026, 10, 30, 22, 45), true),
            (utc(2026, 10, 31, 10, 35), true),
            (utc(2026, 10, 31, 11, 0), false),
        ] {
            assert_eq!(sun_is_up(JAKARTA.0, JAKARTA.1, at), up, "{at}");
        }
    }

    #[test]
    fn london_midsummer_sunrise_and_sunset() {
        let london = (51.5074, -0.1278);
        assert!(progress_at(london, utc(2026, 6, 21, 3, 43)).abs() < 0.01);
        assert!((progress_at(london, utc(2026, 6, 21, 20, 21)) - 1.0).abs() < 0.01);
        let noon = sun_sky(london.0, london.1, utc(2026, 6, 21, 12, 2));
        assert!((noon.noon_elevation_deg - 62.0).abs() < 0.3, "{noon:?}");
    }

    #[test]
    fn the_midnight_sun_is_always_up_and_the_polar_night_never_rises() {
        let tromso = (69.6492, 18.9553);
        let summer = sun_sky(tromso.0, tromso.1, utc(2026, 6, 21, 22, 0));
        assert!(summer.elevation_deg > 0.0, "{summer:?}");
        assert!(
            summer
                .day_progress
                .is_some_and(|p| (0.0..=1.0).contains(&p))
        );
        let winter = sun_sky(tromso.0, tromso.1, utc(2026, 12, 21, 11, 0));
        assert!(winter.elevation_deg < 0.0, "{winter:?}");
        assert_eq!(winter.day_progress, None);
    }

    fn local(y: i32, mo: u32, d: u32, h: u32, mi: u32, offset_hours: i64) -> DateTime<Utc> {
        utc(y, mo, d, h, mi) - Duration::hours(offset_hours)
    }

    #[test]
    fn the_evening_crescent_lies_on_its_back_in_jakarta_but_not_in_berlin() {
        let jakarta = moon_sky(JAKARTA.0, JAKARTA.1, local(2026, 10, 13, 18, 30, 7));
        assert!((180.0..=230.0).contains(&jakarta.limb_deg), "{jakarta:?}");
        assert!((12.0..=26.0).contains(&jakarta.altitude_deg), "{jakarta:?}");
        let berlin = moon_sky(52.52, 13.41, local(2026, 10, 13, 18, 30, 2));
        assert!((240.0..=285.0).contains(&berlin.limb_deg), "{berlin:?}");
        let sydney = moon_sky(-33.87, 151.21, local(2026, 10, 13, 18, 30, 11));
        assert!((150.0..=205.0).contains(&sydney.limb_deg), "{sydney:?}");
    }

    #[test]
    fn the_young_moon_sets_about_eight_in_the_evening_in_jakarta() {
        assert!(moon_sky(JAKARTA.0, JAKARTA.1, local(2026, 10, 13, 19, 40, 7)).altitude_deg > 0.0);
        assert!(moon_sky(JAKARTA.0, JAKARTA.1, local(2026, 10, 13, 20, 20, 7)).altitude_deg < 0.0);
        let dusk = moon_sky(JAKARTA.0, JAKARTA.1, local(2026, 10, 13, 18, 30, 7));
        assert!(dusk.progress.unwrap() > 0.7, "{dusk:?}");
    }

    #[test]
    fn the_full_moon_rises_at_dusk_and_crosses_the_sky_all_night() {
        let evening = moon_sky(JAKARTA.0, JAKARTA.1, local(2026, 10, 26, 18, 45, 7));
        let midnight = moon_sky(JAKARTA.0, JAKARTA.1, local(2026, 10, 27, 0, 0, 7));
        let dawn = moon_sky(JAKARTA.0, JAKARTA.1, local(2026, 10, 27, 5, 30, 7));
        assert!(
            evening.altitude_deg > 0.0 && evening.progress.unwrap() < 0.15,
            "{evening:?}"
        );
        assert!(midnight.altitude_deg > 60.0, "{midnight:?}");
        assert!(
            (0.35..=0.65).contains(&midnight.progress.unwrap()),
            "{midnight:?}"
        );
        assert!(dawn.progress.unwrap() > 0.85, "{dawn:?}");
        assert!(midnight.peak_deg >= midnight.altitude_deg);
        assert!(moon_sky(JAKARTA.0, JAKARTA.1, local(2026, 10, 26, 16, 0, 7)).altitude_deg < 0.0);
    }

    #[test]
    fn sunset_in_jakarta_is_a_quarter_to_six() {
        let set = sunset_after(JAKARTA.0, JAKARTA.1, local(2026, 10, 19, 12, 0, 7)).unwrap();
        let minutes = (set - local(2026, 10, 19, 17, 45, 7)).num_minutes();
        assert!(minutes.abs() <= 4, "{set}");
        let london = sunset_after(51.5, -0.13, utc(2026, 12, 21, 12, 0)).unwrap();
        assert_eq!((london.hour(), london.minute() / 10), (15, 5), "{london}");
    }

    #[test]
    fn a_far_northern_moon_keeps_to_its_side_of_the_sky() {
        for minutes in [0, 110] {
            let at = local(2026, 1, 1, 4, 0, 1) + Duration::minutes(minutes);
            let sky = moon_sky(59.91, 10.75, at);
            assert!(sky.altitude_deg > 0.0, "{sky:?}");
            assert!(sky.progress.unwrap() > 0.6, "{sky:?}");
            assert!(sky.peak_deg > sky.altitude_deg + 5.0, "{sky:?}");
        }
    }

    #[test]
    fn lunations_hit_the_catalogue_within_two_minutes() {
        for (y, mo, d, h, mi, elongation) in CATALOG {
            let at = utc(y, mo, d, h, mi);
            let phase = match elongation as i64 {
                0 => Phase::New,
                180 => Phase::Full,
                _ => continue,
            };
            let k = lunation_near(at);
            let found = (k - 2..=k + 2)
                .map(|k| lunation(k, phase))
                .min_by_key(|when| (*when - at).num_seconds().abs())
                .unwrap();
            assert!((found - at).num_minutes().abs() <= 2, "{at} vs {found}");
        }
    }

    #[test]
    fn the_sun_reaches_the_solstices_and_equinoxes_on_time() {
        let december = sun_reaches(270.0, utc(2026, 12, 21, 0, 0));
        assert!(
            (december - utc(2026, 12, 21, 20, 50)).num_minutes().abs() < 30,
            "{december}"
        );
        let september = sun_reaches(180.0, utc(2026, 9, 23, 0, 0));
        assert!(
            (september - utc(2026, 9, 23, 0, 5)).num_minutes().abs() < 30,
            "{september}"
        );
    }
}
