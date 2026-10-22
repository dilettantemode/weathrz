use super::{Dated, Stamp};

pub(super) const fn stamp(year: i32, month: u32, day: u32, hour: u32) -> Stamp {
    Stamp {
        year,
        month,
        day,
        hour,
    }
}

pub(super) const fn on(start: Stamp, end: Stamp, era: u32) -> Dated {
    Dated {
        start,
        end,
        era,
        tentative: false,
    }
}

pub(super) const fn tentative(start: Stamp, end: Stamp, era: u32) -> Dated {
    Dated {
        start,
        end,
        era,
        tentative: true,
    }
}

pub const SOURCES: &str = "SKB 3 Menteri 2026 (No. 1497/2/5 of 2025) and 2027 (No. 1205/3/2 of 2026), Kemenag isbat; 2028 estimated from the neo-MABIMS hisab, HKO and the Saka calendar until its SKB";

pub const LAST_YEAR: i32 = 2028;

pub const ISRA_MIKRAJ: &[Dated] = &[
    on(stamp(2026, 1, 15, 18), stamp(2026, 1, 17, 0), 1447),
    on(stamp(2027, 1, 4, 18), stamp(2027, 1, 6, 0), 1448),
    on(stamp(2027, 12, 25, 18), stamp(2027, 12, 27, 0), 1449),
    tentative(stamp(2028, 12, 13, 18), stamp(2028, 12, 15, 0), 1450),
];

pub const IMLEK: &[Dated] = &[
    on(stamp(2026, 2, 16, 18), stamp(2026, 3, 4, 0), 2577),
    on(stamp(2027, 2, 5, 18), stamp(2027, 2, 21, 0), 2578),
    tentative(stamp(2028, 1, 25, 18), stamp(2028, 2, 10, 0), 2579),
];

pub const RAMADAN: &[Dated] = &[
    on(stamp(2026, 2, 18, 18), stamp(2026, 3, 20, 18), 1447),
    tentative(stamp(2027, 2, 7, 18), stamp(2027, 3, 9, 18), 1448),
    tentative(stamp(2028, 1, 27, 18), stamp(2028, 2, 26, 18), 1449),
];

pub const NYEPI: &[Dated] = &[
    on(stamp(2026, 3, 19, 6), stamp(2026, 3, 20, 6), 1948),
    on(stamp(2027, 3, 8, 6), stamp(2027, 3, 9, 6), 1949),
    tentative(stamp(2028, 3, 26, 6), stamp(2028, 3, 27, 6), 1950),
];

pub const MUDIK: &[Dated] = &[
    on(stamp(2026, 3, 14, 0), stamp(2026, 3, 20, 18), 1447),
    tentative(stamp(2027, 3, 3, 0), stamp(2027, 3, 9, 18), 1448),
    tentative(stamp(2028, 2, 20, 0), stamp(2028, 2, 26, 18), 1449),
];

pub const IDULFITRI: &[Dated] = &[
    on(stamp(2026, 3, 20, 18), stamp(2026, 3, 25, 0), 1447),
    tentative(stamp(2027, 3, 9, 18), stamp(2027, 3, 16, 0), 1448),
    tentative(stamp(2028, 2, 26, 18), stamp(2028, 3, 2, 0), 1449),
];

pub const IDULADHA: &[Dated] = &[
    on(stamp(2026, 5, 26, 18), stamp(2026, 5, 29, 0), 1447),
    tentative(stamp(2027, 5, 16, 18), stamp(2027, 5, 19, 0), 1448),
    tentative(stamp(2028, 5, 4, 18), stamp(2028, 5, 7, 0), 1449),
];

pub const WAISAK: &[Dated] = &[
    on(stamp(2026, 5, 31, 0), stamp(2026, 6, 1, 6), 2570),
    on(stamp(2027, 5, 20, 0), stamp(2027, 5, 21, 6), 2571),
    tentative(stamp(2028, 5, 9, 0), stamp(2028, 5, 10, 6), 2572),
];

pub const MUHARAM: &[Dated] = &[
    on(stamp(2026, 6, 15, 18), stamp(2026, 6, 17, 0), 1448),
    on(stamp(2027, 6, 5, 18), stamp(2027, 6, 7, 0), 1449),
    tentative(stamp(2028, 5, 25, 18), stamp(2028, 5, 27, 0), 1450),
];

pub const MAULID: &[Dated] = &[
    on(stamp(2026, 8, 25, 0), stamp(2026, 8, 26, 0), 1448),
    on(stamp(2027, 8, 15, 0), stamp(2027, 8, 16, 0), 1449),
    tentative(stamp(2028, 8, 4, 0), stamp(2028, 8, 5, 0), 1450),
];
