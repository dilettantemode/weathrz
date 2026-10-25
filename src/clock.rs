use chrono::{
    DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, Offset, TimeDelta, TimeZone, Utc,
};
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub enum Clock {
    Real,
    Fake {
        start: NaiveDateTime,
        started_at: Instant,
        offset: FixedOffset,
    },
}

impl Clock {
    pub fn fake(start: NaiveDateTime) -> Self {
        Self::fake_at(start, offset_at(&Local, start))
    }

    pub fn fake_at(start: NaiveDateTime, offset: FixedOffset) -> Self {
        Self::Fake {
            start,
            started_at: Instant::now(),
            offset,
        }
    }

    pub fn now(&self) -> NaiveDateTime {
        match self {
            Self::Real => Local::now().naive_local(),
            Self::Fake {
                start, started_at, ..
            } => *start + TimeDelta::from_std(started_at.elapsed()).unwrap_or(TimeDelta::zero()),
        }
    }

    pub fn now_utc(&self) -> DateTime<Utc> {
        match self {
            Self::Real => Utc::now(),
            Self::Fake { offset, .. } => {
                (self.now() - TimeDelta::seconds(i64::from(offset.local_minus_utc()))).and_utc()
            }
        }
    }

    pub fn is_fake(&self) -> bool {
        matches!(self, Self::Fake { .. })
    }
}

fn offset_at<Tz: TimeZone>(zone: &Tz, local: NaiveDateTime) -> FixedOffset {
    zone.offset_from_local_datetime(&local)
        .earliest()
        .map(|offset| offset.fix())
        .unwrap_or_else(|| zone.offset_from_utc_datetime(&local).fix())
}

pub fn parse_date_arg(input: &str, now: NaiveDateTime) -> Result<NaiveDateTime, String> {
    let text = input.trim();
    if let Ok(at) = NaiveDateTime::parse_from_str(text, "%Y-%m-%dT%H:%M") {
        return Ok(at);
    }
    NaiveDate::parse_from_str(text, "%Y-%m-%d")
        .map(|date| date.and_time(now.time()))
        .map_err(|_| format!("invalid --date '{input}': use YYYY-MM-DD or YYYY-MM-DDTHH:MM"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn at(text: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(text, "%Y-%m-%dT%H:%M:%S").unwrap()
    }

    #[test]
    fn parses_date_and_time() {
        let now = at("2026-10-13T09:27:05");
        assert_eq!(
            parse_date_arg("2026-10-31T21:00", now),
            Ok(at("2026-10-31T21:00:00"))
        );
        assert_eq!(
            parse_date_arg(" 2026-10-31T17:55 ", now),
            Ok(at("2026-10-31T17:55:00"))
        );
    }

    #[test]
    fn date_only_keeps_the_time_of_day() {
        let now = at("2026-10-13T09:27:05");
        assert_eq!(
            parse_date_arg("2026-10-31", now),
            Ok(at("2026-10-31T09:27:05"))
        );
    }

    #[test]
    fn rejects_other_shapes_with_the_formats() {
        let now = at("2026-10-13T09:27:05");
        for bad in [
            "2026-13-01",
            "2026-02-30",
            "31-10-2026",
            "2026-10-31 21:00",
            "2026-10-31T21:00:30",
            "tomorrow",
            "",
        ] {
            let err = parse_date_arg(bad, now).unwrap_err();
            assert!(
                err.contains("use YYYY-MM-DD or YYYY-MM-DDTHH:MM"),
                "{bad}: {err}"
            );
            assert!(err.contains(&format!("'{bad}'")), "{err}");
        }
    }

    #[test]
    fn fake_clock_runs_forward_from_its_start() {
        let start = at("2026-10-31T17:59:58");
        let clock = Clock::fake(start);
        assert!(clock.is_fake());
        std::thread::sleep(Duration::from_millis(20));
        let elapsed = clock.now() - start;
        assert!(
            elapsed >= TimeDelta::milliseconds(20) && elapsed < TimeDelta::seconds(5),
            "{elapsed}"
        );
    }

    #[derive(Debug, Clone, Copy)]
    struct SpringForward;

    impl SpringForward {
        fn winter() -> FixedOffset {
            FixedOffset::east_opt(3600).unwrap()
        }
        fn summer() -> FixedOffset {
            FixedOffset::east_opt(7200).unwrap()
        }
    }

    impl TimeZone for SpringForward {
        type Offset = FixedOffset;

        fn from_offset(_offset: &FixedOffset) -> Self {
            SpringForward
        }

        fn offset_from_local_date(&self, _local: &NaiveDate) -> chrono::LocalResult<FixedOffset> {
            chrono::LocalResult::Single(Self::winter())
        }

        fn offset_from_local_datetime(
            &self,
            local: &NaiveDateTime,
        ) -> chrono::LocalResult<FixedOffset> {
            if *local < at("2026-03-29T02:00:00") {
                chrono::LocalResult::Single(Self::winter())
            } else if *local < at("2026-03-29T03:00:00") {
                chrono::LocalResult::None
            } else {
                chrono::LocalResult::Single(Self::summer())
            }
        }

        fn offset_from_utc_date(&self, _utc: &NaiveDate) -> FixedOffset {
            Self::winter()
        }

        fn offset_from_utc_datetime(&self, utc: &NaiveDateTime) -> FixedOffset {
            if *utc < at("2026-03-29T01:00:00") {
                Self::winter()
            } else {
                Self::summer()
            }
        }
    }

    #[test]
    fn a_preview_inside_a_daylight_saving_gap_keeps_the_zone_offset() {
        assert_eq!(
            offset_at(&SpringForward, at("2026-03-29T01:30:00")),
            SpringForward::winter()
        );
        assert_eq!(
            offset_at(&SpringForward, at("2026-03-29T02:30:00")),
            SpringForward::summer()
        );
        assert_eq!(
            offset_at(&SpringForward, at("2026-03-29T12:00:00")),
            SpringForward::summer()
        );
    }

    #[test]
    fn a_fake_clock_knows_its_utc_time() {
        let wib = chrono::FixedOffset::east_opt(7 * 3600).unwrap();
        let clock = Clock::fake_at(at("2026-10-19T17:00:00"), wib);
        assert_eq!(
            clock.now_utc().format("%Y-%m-%dT%H:%M").to_string(),
            "2026-10-19T10:00"
        );
    }

    #[test]
    fn the_real_clock_reads_utc_now() {
        let gap = Clock::Real.now_utc() - chrono::Utc::now();
        assert!(gap.num_seconds().abs() < 2, "{gap}");
    }

    #[test]
    fn real_clock_is_not_fake() {
        assert!(!Clock::Real.is_fake());
        let before = Local::now().naive_local();
        assert!(Clock::Real.now() >= before);
    }
}
