import datetime as dt
import re
import subprocess
import sys
import tomllib
from pathlib import Path
from urllib.parse import urlparse

ROOT = Path(__file__).resolve().parent.parent
HERE = ROOT / "docs" / "holidays"
OUT = ROOT / "src" / "events" / "world.rs"
WEEKDAYS = {"mon": "Mon", "tue": "Tue", "wed": "Wed", "thu": "Thu", "fri": "Fri", "sat": "Sat", "sun": "Sun"}
NIGHT_LOOKS = {"fireworks_night", "diwali", "lanterns", "lunar_new_year", "bonfire"}
DAY_LOOKS = {"ketupat", "eid", "easter_eggs", "holi", "koinobori"}
LUNAR_NEW_YEAR = {2026: dt.date(2026, 2, 17), 2027: dt.date(2027, 2, 6), 2028: dt.date(2028, 1, 26)}
LOOKS = {
    "none": [], "flag": ["Flag"], "bonfire": ["Bonfire"], "holi": ["Bonfire", "Holi"],
    "koinobori": ["Koinobori"],
    "lunar_new_year": ["LunarNewYear"], "lanterns": ["Lanterns"], "diwali": ["Diwali"],
    "ramadan": ["Ramadan"], "eid": ["Eid"], "ketupat": ["Ketupat"],
    "fireworks_night": ["Fireworks"], "candles": ["Candles"], "easter_eggs": ["Easter"],
}
ICON_BY_LOOK = {
    "lunar_new_year": "eef8", "lanterns": "f0f62", "diwali": "f05e2", "ramadan": "f0f67",
    "eid": "f0979", "ketupat": "f0979", "fireworks_night": "f0e30", "bonfire": "f0edd",
    "candles": "f05e2", "easter_eggs": "f0ab0", "flag": "f023b", "holi": "f0e30",
    "koinobori": "f068f",
}
ICON_BY_WORD = [
    (("independence", "national_day", "republic", "constitution", "victory", "unity", "democracy",
      "liberation", "freedom", "martyr", "patriot", "pakistan_day", "kashmir", "adwa", "derg",
      "russia_day", "merdeka", "malaysia_day", "revolution", "proclamation", "tiradentes", "juneteenth",
      "veterans", "memorial", "remembrance", "flag", "language", "statehood", "armed_forces", "defender",
      "presidents", "king", "agong", "birthday", "benito", "juarez", "quaid", "iqbal", "mujib", "consciencia",
      "black_awareness", "gandhi", "columbus", "indigenous", "mlk", "martin_luther"), "f023b"),
    (("labour", "labor", "workers", "may_day", "spring_and_labour"), "f1323"),
    (("good_friday", "easter", "christmas", "ascension", "corpus", "epiphany", "assumption", "all_saints",
      "genna", "timkat", "fasika", "siklet", "meskel", "aparecida", "virgin", "guadalupe", "holy",
      "boxing", "lady"), "f0144"),
    (("vesak", "buddha", "waisak", "wesak"), "f094b"),
    (("holi", "dussehra", "janmashtami", "ram_navami", "mahavir", "guru", "durga", "deepavali", "thaipusam",
      "shivaratri", "rath", "onam", "pongal", "makar", "navratri"), "f0973"),
    (("eid", "ashura", "mawlid", "milad", "isra", "hijri", "islamic", "muharram", "shab", "nuzul", "awal",
      "laylat", "arafat", "prophet"), "f0979"),
    (("children", "kodomo", "youth", "coming_of_age", "seijin", "respect_for_the_aged", "keiro"), "f068f"),
    (("thanksgiving",), "f171c"),
    (("muertos", "dead"), "f0bc8"),
    (("carnival", "carnaval"), "f0d02"),
    (("new_year", "spring_festival", "enkutatash", "boishakh", "nowruz"), "f0e30"),
    (("mid_autumn", "moon"), "f0f62"),
    (("qingming", "tomb", "obon", "dragon_boat"), "f024a"),
    (("bank_holiday",), "f0599"),
    (("equinox", "showa", "greenery", "mountain", "marine", "sea_day", "culture", "sports", "health", "foundation", "emperor"), "f0531"),
]


def parse_date(text):
    return dt.date.fromisoformat(text)


def easter(year):
    a = year % 19
    b, c = divmod(year, 100)
    d, e = divmod(b, 4)
    f = (b + 8) // 25
    g = (b - f + 1) // 3
    h = (19 * a + b - d - g + 15) % 30
    i, k = divmod(c, 4)
    l = (32 + 2 * e + 2 * i - h - k) % 7
    m = (a + 11 * h + 22 * l) // 451
    n = h + l - 7 * m + 114
    return dt.date(year, n // 31, n % 31 + 1)


def orthodox(year):
    a, b, c = year % 4, year % 7, year % 19
    d = (19 * c + 15) % 30
    e = (2 * a + 4 * b - d + 34) % 7
    n = d + e + 114
    return dt.date(year, n // 31, n % 31 + 1) + dt.timedelta(days=13)


def equinox(year, season):
    since = year - 1980
    base = 20.8431 if season == "spring" else 23.2488
    day = int(base + 0.242194 * since - since // 4)
    return dt.date(year, 3 if season == "spring" else 9, day)


def nth_weekday(year, month, weekday, n):
    target = list(WEEKDAYS).index(weekday)
    day = dt.date(year, month, 1)
    while day.weekday() != target:
        day += dt.timedelta(days=1)
    day += dt.timedelta(weeks=n - 1)
    return day if day.month == month else None


def last_weekday(year, month, weekday):
    target = list(WEEKDAYS).index(weekday)
    day = dt.date(year + (month == 12), month % 12 + 1, 1) - dt.timedelta(days=1)
    while day.weekday() != target:
        day -= dt.timedelta(days=1)
    return day


def parse_rule(text):
    text = text.strip().replace(" ", "")
    if text == "listed":
        return None
    found = re.fullmatch(r"([a-z_]+)\((.*)\)", text)
    if not found:
        raise ValueError("expected name(args) or listed")
    name, args = found.groups()
    parts = [part.strip().lower() for part in args.split(",")] if args else []
    if name == "fixed":
        month, day = map(int, parts)
        return (f"DayRule::Fixed({month}, {day})", lambda y: dt.date(y, month, day))
    if name == "nth_weekday":
        month, weekday, n = int(parts[0]), parts[1][:3], int(parts[2])
        if not 1 <= n <= 5:
            raise ValueError(f"nth_weekday needs n from 1 to 5, not {n}")
        return (f"DayRule::Nth({month}, Weekday::{WEEKDAYS[weekday]}, {n})", lambda y: nth_weekday(y, month, weekday, n))
    if name == "last_weekday":
        month, weekday = int(parts[0]), parts[1][:3]
        return (f"DayRule::Last({month}, Weekday::{WEEKDAYS[weekday]})", lambda y: last_weekday(y, month, weekday))
    if name in ("easter", "orthodox_easter"):
        offset = int(parts[0]) if parts and parts[0] else 0
        rust = "Easter" if name == "easter" else "Orthodox"
        base = easter if name == "easter" else orthodox
        return (f"DayRule::{rust}({offset})", lambda y: base(y) + dt.timedelta(days=offset))
    if name == "equinox":
        season = parts[0]
        return (f"DayRule::Equinox(Season::{season.capitalize()})", lambda y: equinox(y, season))
    raise ValueError(text)


def runs(dates):
    dates = sorted(set(dates))
    out = []
    for day in dates:
        if out and day - out[-1][-1] == dt.timedelta(days=1):
            out[-1].append(day)
        else:
            out.append([day])
    return out


def look_parts(holiday):
    return [part.strip() for part in holiday.get("look", "none").split("+")]


def icon(holiday):
    for look in look_parts(holiday):
        if look in ICON_BY_LOOK:
            return ICON_BY_LOOK[look]
    key = holiday["id"]
    for words, code in ICON_BY_WORD:
        if any(word in key for word in words):
            return code
    return "f09d3"


def rust_string(text):
    return '"' + text.replace("\\", "\\\\").replace('"', '\\"') + '"'


def stamp(day, hour):
    return f"stamp({day.year}, {day.month}, {day.day}, {hour})"


def load():
    holidays = []
    for path in sorted(HERE.glob("group-*.md")) + [HERE / "extras.md"]:
        text = path.read_text()
        for block in re.findall(r"```toml\n(.*?)```", text, re.S):
            data = tomllib.loads(block)
            holidays.extend(data.get("holiday", []))
    return holidays


EXCLUDE = {
    "et:derg_downfall_day": "not a day off under Proclamation 1334/2024",
}

SKIP = {
    ("christmas", 12, 25), ("christmas_day", 12, 25), ("new_years_day", 1, 1), ("new_year", 1, 1),
    ("new_year_day", 1, 1), ("new_years", 1, 1),
}


def main():
    holidays = load()
    defs, checks, sources, notes = [], [], {}, []
    last_years = {}
    seen = set()
    for holiday in holidays:
        country = holiday["country"].upper().replace("UK", "GB")
        hid = holiday["id"]
        key = f"{country.lower()}:{hid}"
        if key in EXCLUDE:
            notes.append(f"{key}: excluded ({EXCLUDE[key]})")
            continue
        if key in seen:
            notes.append(f"duplicate {key} skipped")
            continue
        seen.add(key)
        years = sorted(int(name[6:]) for name in holiday if re.fullmatch(r"dates_\d{4}", name))
        by_year = {year: [parse_date(x) for x in holiday[f"dates_{year}"]] for year in years}
        by_year = {year: dates for year, dates in by_year.items() if dates}
        if not by_year:
            notes.append(f"{key}: no dates")
            continue
        last_years[country] = max([last_years.get(country, 0), *by_year])
        if all(
            len(dates) == 1 and (hid, dates[0].month, dates[0].day) in SKIP for dates in by_year.values()
        ):
            notes.append(f"{key}: handled by the global {hid}")
            continue
        parts = look_parts(holiday)
        look = parts[0]
        looks = []
        for part in parts:
            if part not in LOOKS:
                raise SystemExit(f"{key}: unknown look {part!r}")
            looks.extend(name for name in LOOKS[part] if name not in looks)
        eve_wanted = bool(holiday.get("eve", False)) and look != "diwali"
        eve = eve_wanted and look != "lunar_new_year"
        night = any(part in NIGHT_LOOKS for part in parts) and look not in DAY_LOOKS
        tentative_years = {year for year in by_year if holiday.get(f"tentative_{year}", False)}
        rule_text = holiday.get("rule", "listed")
        if isinstance(rule_text, list):
            rule_text = rule_text[0]
        try:
            rule = parse_rule(rule_text)
        except (ValueError, KeyError, IndexError) as error:
            raise SystemExit(f"{key}: bad rule {rule_text!r}: {error}")
        rust_dates = None
        if rule:
            rust_rule, compute = rule
            lengths = {len(r) for dates in by_year.values() for r in runs(dates)}
            single_run = all(len(runs(dates)) == 1 for dates in by_year.values())
            matches = all(compute(dates[0].year) == dates[0] for dates in by_year.values())
            if single_run and len(lengths) == 1 and matches:
                days = lengths.pop()
                showcase = 10 if look in DAY_LOOKS else (-3 if eve else (21 if night else 10))
                rust_dates = f"rule({rust_rule}, {days}, {str(eve).lower()}, {showcase})"
            else:
                notes.append(f"{key}: rule {rule_text} does not reproduce the listed dates; tabled")
        if rust_dates is None:
            rows = []
            first_row_eve = None
            for year, dates in by_year.items():
                for index, run in enumerate(runs(dates)):
                    if look == "lunar_new_year":
                        run_eve = eve_wanted and run[0] == LUNAR_NEW_YEAR.get(run[0].year)
                    else:
                        run_eve = eve and index == 0
                    if first_row_eve is None:
                        first_row_eve = run_eve
                    start = run[0] - dt.timedelta(days=1) if run_eve else run[0]
                    begin = stamp(start, 18 if run_eve else 0)
                    end = stamp(run[-1] + dt.timedelta(days=1), 0)
                    maker = "tentative" if year in tentative_years else "on"
                    rows.append(f"{maker}({begin}, {end}, 0)")
            if look in DAY_LOOKS:
                showcase = 16 if first_row_eve else 10
            else:
                showcase = 3 if first_row_eve else (21 if night else 10)
            rust_dates = "listed(&[" + ", ".join(rows) + f"], {showcase})"
        if "greeting_en" not in holiday:
            raise SystemExit(f"{key}: no greeting_en")
        greeting = holiday["greeting_en"].strip()
        defs.append(
            "    EventDef {\n"
            f"        id: {rust_string(key)},\n"
            f"        dates: {rust_dates},\n"
            f"        greeting: {rust_string(greeting)},\n"
            f"        nerd_icon: '\\u{{{icon(holiday)}}}',\n"
            f"        looks: &[{', '.join('Look::' + name for name in looks)}],\n"
            "    },"
        )
        every_date = [day for dates in by_year.values() for day in dates]
        checks.append(f"    ({rust_string(key)}, &[{', '.join(rust_string(d.isoformat()) for d in every_date)}]),")
        source = holiday.get("source", "")
        urls = source if isinstance(source, list) else str(source).replace(";", " ").replace(",", " ").split()
        for url in urls:
            host = urlparse(url).netloc.removeprefix("www.")
            if host:
                sources.setdefault(country, [])
                if host not in sources[country]:
                    sources[country].append(host)
    source_rows = "\n".join(
        f"    ({rust_string(code)}, {rust_string(', '.join(hosts[:6]))}),"
        for code, hosts in sorted(sources.items())
    )
    body = "\n".join(defs)
    dated = [name for name in ("on", "stamp", "tentative") if f"{name}(" in body]
    parent = [name for name in ("DayRule", "EventDef", "Look", "Season") if name == "EventDef" or f"{name}::" in body]
    parent += [name for name in ("listed", "rule") if f"{name}(" in body]
    header = ""
    if dated:
        header += "use super::dates::{" + ", ".join(dated) + "};\n"
    header += "use super::{" + ", ".join(parent) + "};\n"
    if "Weekday::" in body:
        header += "use chrono::Weekday;\n"
    OUT.write_text(
        header + "\n"
        "pub const WORLD: &[EventDef] = &[\n" + body + "\n];\n\n"
        "pub const LAST_YEARS: &[(&str, i32)] = &[\n"
        + "\n".join(f"    ({rust_string(code)}, {year})," for code, year in sorted(last_years.items()))
        + "\n];\n\n"
        "pub const SOURCES: &[(&str, &str)] = &[\n" + source_rows + "\n];\n\n"
        "#[cfg(test)]\n"
        "pub(super) const CHECKS: &[(&str, &[&str])] = &[\n" + "\n".join(checks) + "\n];\n"
    )
    subprocess.run(["rustfmt", "--edition", "2024", str(OUT)], check=True)
    print(f"{len(defs)} events, {len(checks)} checks, {len(sources)} countries", file=sys.stderr)
    for note in notes:
        print("note:", note, file=sys.stderr)


main()
