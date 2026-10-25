# World holidays — group D: SG, PH, TH

Compiled 23 Oct 2026 for the weathrz holiday greetings and decorations (2026, 2027, 2028).
All three countries complete (SG, PH, TH); counts at the end.

**Schema conventions used below**

- Same as group C. `rule` is one string from the fixed grammar; `listed` covers lunar,
  lunisolar and announced dates. Weekday tokens are `mon tue wed thu fri sat sun`.
- `dates_*` hold the holiday day itself. Substitute / observed / bridge days are in Notes.
- `tentative_2027 = true` / `tentative_2028 = true` mean the **date** may still move (moon
  sighting, a lunar computation, or a list not yet published; each country's Notes say which).
- `observance = true` marks a day that is not a day off (Ramadan month, Easter Sunday, half-mast
  days, other widely marked days).
- `look = "half_mast"` is a new look for days the national flag is officially flown at half-mast.
- Easter Sunday is 2026-04-05, 2027-03-28 and 2028-04-16 (same as group C).
- Lunar New Year is 2026-02-17, 2027-02-06 and 2028-01-26 (generator `LUNAR_NEW_YEAR`).

**Primary sources checked**

| Country | Official source | Cross-check |
|---|---|---|
| SG | [MOM public holidays](https://www.mom.gov.sg/employment-practices/public-holidays) (2026 + 2027, fetched 23 Oct 2026); MOM-managed [data.gov.sg "Public Holidays for 2027"](https://data.gov.sg/datasets/d_0ba69fc6d56717ff0bf8083c6af3bb84/view) (read via API); [Holidays Act 1998](https://sso.agc.gov.sg/Act/HA1998) | officeholidays.com [2026](https://www.officeholidays.com/countries/singapore/2026) / [2027](https://www.officeholidays.com/countries/singapore/2027) (2027 stale, see Notes); timeanddate.com [2028](https://www.timeanddate.com/holidays/singapore/2028) via Wayback 11 Feb 2026 |
| PH | [Proclamation No. 1006 s.2025](https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html) (2026) and [No. 1427 s.2026](https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html) (2027, signed 8 Sep 2026); Eid proclamations [1189](https://lawphil.net/executive/proc/proc2026/proc_1189_2026.html) / [1264](https://lawphil.net/executive/proc/proc2026/proc_1264_2026.html); [RA 9492](https://lawphil.net/statutes/repacts/ra2007/ra_9492_2007.html), [RA 8491](https://lawphil.net/statutes/repacts/ra1998/ra_8491_1998.html) (flag code) | officeholidays.com [2026](https://www.officeholidays.com/countries/philippines/2026) / [2027](https://www.officeholidays.com/countries/philippines/2027); timeanddate.com [2027](https://www.timeanddate.com/holidays/philippines/2027) / [2028](https://www.timeanddate.com/holidays/philippines/2028) via Wayback (28 Sep / 17 Apr 2026); AlAdhan Umm al-Qura |
| TH | BOT [Notification 37/2569](https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2569/EngPDF/25690175.pdf) (2027 bank holidays, Royal Gazette 25 Aug 2026) and [26/2569](https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2569/EngPDF/25690116.pdf); [BOT holiday page](https://www.bot.or.th/en/financial-institutions-holiday.html) (2026, rendered by script, not readable here); 2026 government list via the [Singapore Embassy Bangkok](https://isomer-user-content.by.gov.sg/251/8e1ca282-3965-4e24-8b61-764cc74923bd/Thai%20Public%20Holidays%20Updated%20May%202026.pdf) and [MFA (Royal Thai Embassy Abu Dhabi)](https://image.mfa.go.th/mfa/0/2QL32ZatFo/A_King/Official_Holidays_2026_(B.E._2569)_rev_August_2026.pdf) lists | officeholidays.com [2026](https://www.officeholidays.com/countries/thailand/2026) / [2027](https://www.officeholidays.com/countries/thailand/2027); timeanddate.com [2027](https://www.timeanddate.com/holidays/thailand/2027) via Wayback 6 Mar 2026 (no Buddhist days yet; no 2028 snapshot); own Thai lunar-calendar and Meeus full-moon computation for 2028 |

Fetch problems (23 Oct 2026): sso.agc.gov.sg (Singapore statutes) returned 403, so the National
Symbols Regulations were not read directly. officialgazette.gov.ph is behind a Cloudflare challenge;
the same proclamation texts were read on lawphil.net. The BOT holiday page renders its tables by
script and the Thai Cabinet's 2026 resolution text was not found online, so 2026 Thai government
days rest on two embassy lists plus officeholidays.com and Thai news. myhora.com (Thai lunar
calendars) returned 403. officeholidays.com has no 2028 pages (404 for all three countries);
timeanddate.com was read through Wayback snapshots (SG 2028: 11 Feb 2026; PH 2027: 28 Sep 2026,
PH 2028: 17 Apr 2026; TH 2027: 6 Mar 2026; no TH 2028 snapshot exists).

---

## Singapore (SG)

```toml
[[holiday]]
country = "SG"
id = "new_year"
name = "New Year's Day"
rule = "fixed(1,1)"
dates_2026 = ["2026-01-01"]
dates_2027 = ["2027-01-01"]
dates_2028 = ["2028-01-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = true
greeting = "Happy New Year"
greeting_en = "Happy New Year"
look = "fireworks_night"
source = ["https://www.mom.gov.sg/employment-practices/public-holidays", "https://data.gov.sg/datasets/d_0ba69fc6d56717ff0bf8083c6af3bb84/view", "https://www.officeholidays.com/countries/singapore/2027"]

[[holiday]]
country = "SG"
id = "chinese_new_year"
name = "Chinese New Year (农历新年 / Hari Tahun Baru Cina)"
rule = "listed"
dates_2026 = ["2026-02-17", "2026-02-18"]
dates_2027 = ["2027-02-06", "2027-02-07"]
dates_2028 = ["2028-01-26", "2028-01-27"]
tentative_2027 = false
tentative_2028 = true
observance = false
eve = true
greeting = "Gong Xi Fa Cai"
greeting_en = "Happy Chinese New Year · Year of the {animal}"
look = "lunar_new_year"
source = ["https://www.mom.gov.sg/employment-practices/public-holidays", "https://data.gov.sg/datasets/d_0ba69fc6d56717ff0bf8083c6af3bb84/view", "https://www.officeholidays.com/countries/singapore/2027", "https://www.timeanddate.com/holidays/singapore/2028"]

[[holiday]]
country = "SG"
id = "eid_al_fitr"
name = "Hari Raya Puasa (Hari Raya Aidilfitri)"
rule = "listed"
dates_2026 = ["2026-03-21"]
dates_2027 = ["2027-03-10"]
dates_2028 = ["2028-02-27"]
tentative_2027 = false
tentative_2028 = true
observance = false
eve = true
greeting = "Selamat Hari Raya Aidilfitri"
greeting_en = "Happy Hari Raya Puasa {era} AH"
look = "ketupat"
source = ["https://www.mom.gov.sg/employment-practices/public-holidays", "https://data.gov.sg/datasets/d_0ba69fc6d56717ff0bf8083c6af3bb84/view", "https://www.muis.gov.sg/resources/media-releases/", "https://www.timeanddate.com/holidays/singapore/2028"]

[[holiday]]
country = "SG"
id = "good_friday"
name = "Good Friday"
rule = "easter(-2)"
dates_2026 = ["2026-04-03"]
dates_2027 = ["2027-03-26"]
dates_2028 = ["2028-04-14"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Good Friday"
greeting_en = "Good Friday"
look = "none"
source = ["https://www.mom.gov.sg/employment-practices/public-holidays", "https://data.gov.sg/datasets/d_0ba69fc6d56717ff0bf8083c6af3bb84/view", "https://www.timeanddate.com/holidays/singapore/2028"]

[[holiday]]
country = "SG"
id = "labour_day"
name = "Labour Day (Hari Buruh)"
rule = "fixed(5,1)"
dates_2026 = ["2026-05-01"]
dates_2027 = ["2027-05-01"]
dates_2028 = ["2028-05-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy Labour Day"
greeting_en = "Happy Labour Day"
look = "none"
source = ["https://www.mom.gov.sg/employment-practices/public-holidays", "https://data.gov.sg/datasets/d_0ba69fc6d56717ff0bf8083c6af3bb84/view", "https://www.officeholidays.com/countries/singapore/2027"]

[[holiday]]
country = "SG"
id = "eid_al_adha"
name = "Hari Raya Haji (Hari Raya Aidiladha)"
rule = "listed"
dates_2026 = ["2026-05-27"]
dates_2027 = ["2027-05-17"]
dates_2028 = ["2028-05-05"]
tentative_2027 = false
tentative_2028 = true
observance = false
eve = true
greeting = "Selamat Hari Raya Aidiladha"
greeting_en = "Happy Hari Raya Haji {era} AH"
look = "eid"
source = ["https://www.mom.gov.sg/employment-practices/public-holidays", "https://data.gov.sg/datasets/d_0ba69fc6d56717ff0bf8083c6af3bb84/view", "https://www.timeanddate.com/holidays/singapore/2028"]

[[holiday]]
country = "SG"
id = "vesak"
name = "Vesak Day (卫塞节 / Hari Wesak)"
rule = "listed"
dates_2026 = ["2026-05-31"]
dates_2027 = ["2027-05-20"]
dates_2028 = ["2028-05-09"]
tentative_2027 = false
tentative_2028 = true
observance = false
eve = false
greeting = "Happy Vesak Day"
greeting_en = "Happy Vesak Day"
look = "candles"
source = ["https://www.mom.gov.sg/employment-practices/public-holidays", "https://data.gov.sg/datasets/d_0ba69fc6d56717ff0bf8083c6af3bb84/view", "https://www.officeholidays.com/countries/singapore/2027"]

[[holiday]]
country = "SG"
id = "national_day"
name = "National Day (Hari Kebangsaan)"
rule = "fixed(8,9)"
dates_2026 = ["2026-08-09"]
dates_2027 = ["2027-08-09"]
dates_2028 = ["2028-08-09"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy National Day, Singapore"
greeting_en = "Happy {nth since 1965} National Day, Singapore"
look = "flag+fireworks_night"
source = ["https://www.mom.gov.sg/employment-practices/public-holidays", "https://data.gov.sg/datasets/d_0ba69fc6d56717ff0bf8083c6af3bb84/view", "https://www.timeanddate.com/holidays/singapore/2028"]

[[holiday]]
country = "SG"
id = "diwali"
name = "Deepavali (தீபாவளி)"
rule = "listed"
dates_2026 = ["2026-11-08"]
dates_2027 = ["2027-10-28"]
dates_2028 = ["2028-10-17"]
tentative_2027 = false
tentative_2028 = true
observance = false
eve = false
greeting = "Happy Deepavali"
greeting_en = "Happy Deepavali"
look = "diwali"
source = ["https://www.mom.gov.sg/employment-practices/public-holidays", "https://data.gov.sg/datasets/d_0ba69fc6d56717ff0bf8083c6af3bb84/view", "https://www.timeanddate.com/holidays/singapore/2028"]

[[holiday]]
country = "SG"
id = "christmas"
name = "Christmas Day"
rule = "fixed(12,25)"
dates_2026 = ["2026-12-25"]
dates_2027 = ["2027-12-25"]
dates_2028 = ["2028-12-25"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = true
greeting = "Merry Christmas"
greeting_en = "Merry Christmas"
look = "none"
source = ["https://www.mom.gov.sg/employment-practices/public-holidays", "https://data.gov.sg/datasets/d_0ba69fc6d56717ff0bf8083c6af3bb84/view", "https://www.officeholidays.com/countries/singapore/2027"]

[[holiday]]
country = "SG"
id = "ramadan"
name = "Ramadan / Bulan Puasa (fasting month; not a day off)"
rule = "listed"
dates_2026 = ["2026-02-19", "2026-02-20", "2026-02-21", "2026-02-22", "2026-02-23", "2026-02-24", "2026-02-25", "2026-02-26", "2026-02-27", "2026-02-28", "2026-03-01", "2026-03-02", "2026-03-03", "2026-03-04", "2026-03-05", "2026-03-06", "2026-03-07", "2026-03-08", "2026-03-09", "2026-03-10", "2026-03-11", "2026-03-12", "2026-03-13", "2026-03-14", "2026-03-15", "2026-03-16", "2026-03-17", "2026-03-18", "2026-03-19", "2026-03-20"]
dates_2027 = ["2027-02-08", "2027-02-09", "2027-02-10", "2027-02-11", "2027-02-12", "2027-02-13", "2027-02-14", "2027-02-15", "2027-02-16", "2027-02-17", "2027-02-18", "2027-02-19", "2027-02-20", "2027-02-21", "2027-02-22", "2027-02-23", "2027-02-24", "2027-02-25", "2027-02-26", "2027-02-27", "2027-02-28", "2027-03-01", "2027-03-02", "2027-03-03", "2027-03-04", "2027-03-05", "2027-03-06", "2027-03-07", "2027-03-08", "2027-03-09"]
dates_2028 = ["2028-01-28", "2028-01-29", "2028-01-30", "2028-01-31", "2028-02-01", "2028-02-02", "2028-02-03", "2028-02-04", "2028-02-05", "2028-02-06", "2028-02-07", "2028-02-08", "2028-02-09", "2028-02-10", "2028-02-11", "2028-02-12", "2028-02-13", "2028-02-14", "2028-02-15", "2028-02-16", "2028-02-17", "2028-02-18", "2028-02-19", "2028-02-20", "2028-02-21", "2028-02-22", "2028-02-23", "2028-02-24", "2028-02-25", "2028-02-26"]
tentative_2027 = true
tentative_2028 = true
observance = true
eve = true
greeting = "Selamat Berpuasa"
greeting_en = "Ramadan Mubarak"
look = "ramadan"
source = ["https://mothership.sg/2026/02/singapore-muslims-ramadan-feb-19/", "https://www.muis.gov.sg/resources/media-releases/", "https://www.timeanddate.com/holidays/singapore/2028"]

[[holiday]]
country = "SG"
id = "thaipusam"
name = "Thaipusam (தைப்பூசம்; not a day off)"
rule = "listed"
dates_2026 = ["2026-02-01"]
dates_2027 = ["2027-01-22"]
dates_2028 = ["2028-02-09"]
tentative_2027 = true
tentative_2028 = true
observance = true
eve = false
greeting = "Happy Thaipusam"
greeting_en = "Happy Thaipusam"
look = "none"
source = ["https://www.officeholidays.com/holidays/malaysia/thaipusam", "https://www.drikpanchang.com/festivals/thaipusam/tamil-thaipusam-date.html?year=2028"]

[[holiday]]
country = "SG"
id = "total_defence_day"
name = "Total Defence Day (not a day off)"
rule = "fixed(2,15)"
dates_2026 = ["2026-02-15"]
dates_2027 = ["2027-02-15"]
dates_2028 = ["2028-02-15"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Total Defence Day"
greeting_en = "Total Defence Day"
look = "none"
source = ["https://www.mindef.gov.sg/", "https://www.timeanddate.com/holidays/singapore/2028"]

[[holiday]]
country = "SG"
id = "dragon_boat"
name = "Dragon Boat Festival (端午节, Duanwu; not a day off)"
rule = "listed"
dates_2026 = ["2026-06-19"]
dates_2027 = ["2027-06-09"]
dates_2028 = ["2028-05-28"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Duanwu Jie Kuai Le"
greeting_en = "Happy Dragon Boat Festival"
look = "none"
source = ["docs/holidays/group-b.md", "https://www.hko.gov.hk/en/gts/time/conversion.htm"]

[[holiday]]
country = "SG"
id = "hungry_ghost_festival"
name = "Hungry Ghost Festival (中元节, Zhongyuan; not a day off)"
rule = "listed"
dates_2026 = ["2026-08-27"]
dates_2027 = ["2027-08-16"]
dates_2028 = ["2028-09-03"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Zhongyuan Jie"
greeting_en = "Hungry Ghost Festival"
look = "none"
source = ["https://www.hko.gov.hk/en/gts/time/conversion.htm", "https://www.roots.gov.sg/stories-landing/stories/hungry-ghost-festival"]

[[holiday]]
country = "SG"
id = "mid_autumn"
name = "Mid-Autumn Festival (中秋节; not a day off)"
rule = "listed"
dates_2026 = ["2026-09-25"]
dates_2027 = ["2027-09-15"]
dates_2028 = ["2028-10-03"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Zhongqiu Jie Kuai Le"
greeting_en = "Happy Mid-Autumn Festival"
look = "lanterns+harvest_moon"
source = ["docs/holidays/group-b.md", "https://www.hko.gov.hk/en/gts/time/conversion.htm"]
```

**Notes (SG)**

- **Legal basis:** Holidays Act 1998 (Schedule lists the 11 days; [SSO](https://sso.agc.gov.sg/Act/HA1998)).
  MOM publishes each year's dates. The **2027 list is official**: MOM page (fetched 23 Oct 2026,
  "Last Updated 19 June 2026") and the MOM-managed data.gov.sg dataset "Public Holidays for 2027"
  (created 3 Jul 2026, read through its API). **2028 is not published** (MOM usually releases it
  in June of the year before); the 2028 dates above are computed and match the timeanddate 2028
  snapshot of 11 Feb 2026 for every day it lists.
- **Sunday rule:** a holiday on a Sunday moves the day off to the next Monday (Holidays Act;
  MOM wording "Monday … is a public holiday"). A Saturday holiday gets no extra public holiday;
  employers give pay or a day off in lieu under the Employment Act.
  - 2026 in-lieu days: Mon 1 Jun (Vesak), Mon 10 Aug (National Day), Mon 9 Nov (Deepavali).
    Hari Raya Puasa (Sat 21 Mar) had none.
  - 2027: **Mon 8 Feb** (CNY day 2 on Sun 7 Feb) is listed by data.gov.sg as "Chinese New Year
    (Observed)". 1 May and 25 Dec are Saturdays (no in-lieu).
  - 2028 expected: Mon 28 Feb (Hari Raya Puasa on Sun 27 Feb). 1 Jan is a Saturday.
- **Islamic dates:** MUIS fixes them by calculation (imkanur rukyah, the MABIMS criterion), not by
  sighting; Ramadan 1447 began Thu 19 Feb 2026 after MUIS reported the moon set 4 min before the
  sun on 17 Feb. Hari Raya Puasa 2026 was Sat 21 Mar (30 fasts); Hari Raya Haji 2026 Wed 27 May.
  2027 dates are MOM-official and robust: on 8 Mar 2027 the moon is ~2 h old at Singapore sunset
  and on 6 May 2027 the conjunction is minutes before sunset, so neither month can start a day
  early. That is why `tentative_2027 = false` for both Hari Rayas; MUIS still confirms them a few
  days ahead. 2028 follows the MABIMS estimate used for Malaysia in group A (Puasa 27 Feb, Haji
  5 May; timeanddate agrees).
  - Ramadan block 2027 (8 Feb–9 Mar) assumes MUIS matches Malaysia's official Awal Ramadan
    (8 Feb); MUIS has not announced it, hence tentative. 2028: 28 Jan–26 Feb (timeanddate also
    has 28 Jan).
- **Disagreement (2027):** officeholidays.com still shows the old estimates **Hari Raya Puasa
  Tue 9 Mar**, **Hari Raya Haji Sun 16 May** and **Deepavali Fri 29 Oct**, lists a bogus "National
  Day (in lieu) Tue 10 Aug" (9 Aug 2027 is a Monday) and misses Mon 8 Feb. MOM and data.gov.sg
  give 10 Mar, 17 May, 28 Oct and Mon 8 Feb; those are used.
- **Vesak rule:** Singapore's Vesak is the 15th day of the 4th Chinese lunar month (this matches
  2020 7 May, 2021 26 May, 2022 15 May, 2023 2 Jun, 2024 22 May, 2025 12 May, 2026 31 May, 2027
  20 May; 2023 differs from Malaysia's 4 May because of that year's leap 2nd month). 2028's 4th
  month begins 25 Apr (leap month is the 5th), so Vesak is **Tue 9 May 2028**. timeanddate's 2028
  snapshot has no Vesak row at all.
- **Deepavali** is the Tamil Naraka Chaturdashi date (same as Malaysia): 2028 → Tue 17 Oct
  (timeanddate agrees). Hindu Advisory Board confirms it each year.
- **Thaipusam** stopped being a public holiday in 1968 but its kavadi procession (Little India to
  Tank Road) is one of the island's best-known events. Dates: Poosam nakshatra in the Tamil month
  Thai (2026 Sun 1 Feb, 2027 Fri 22 Jan per officeholidays/drikpanchang; 2028 Wed 9 Feb: drikpanchang
  has Poosam 06:50 IST 9 Feb – 04:02 IST 10 Feb, i.e. 09:20 SGT 9 Feb – 06:32 SGT 10 Feb, so 9 Feb
  in Singapore too). Not officially fixed; tentative.
- **Chinese-calendar observances** (Duanwu, Zhongyuan, Mid-Autumn) are fixed by the lunisolar
  calendar (HKO tables), so they are not tentative. Hungry Ghost is the 15th of the 7th month;
  the whole 7th month (getai shows, roadside offerings) is "Hungry Ghost month". 2028 has a leap
  5th month, which is why Zhongyuan lands on 3 Sep.
- **Total Defence Day** marks the 15 Feb 1942 surrender to Japan; the SCDF sounds the Public
  Warning System "Important Message" signal islandwide that day. SAF Day (1 Jul), Racial Harmony
  Day (21 Jul), Teachers' Day and Children's Day are school/organisation observances only — left out.
- **Looks:** National Day gets `flag+fireworks_night` for the NDP fireworks on the evening of
  9 Aug (no eve). New Year gets `fireworks_night` with `eve` for the Marina Bay countdown; the
  global New Year entry already does this, so SG's row is skipped by the generator (SKIP).
  Christmas likewise. `id = "diwali"`, the id MY and IN use for the same day.
- **Not public holidays:** Christmas Eve, New Year's Eve, Easter Sunday, Thaipusam, Mid-Autumn.
  Polling Day for a general/presidential election is a public holiday by law (elections acts)
  but is ad hoc.

### Country notes (SG)

- **Christmas / New Year:** both are gazetted public holidays. Christmas is widely marked (the
  Orchard Road light-up runs mid-Nov to early Jan) and the Marina Bay New Year countdown has
  fireworks — keep both defaults.
- **Half-mast:** **none fixed.** Under the National Symbols Act 2022 and National Symbols
  Regulations 2023 (in force 1 Aug 2023, replacing the SAFNA Rules 2003 r.10) the Government
  directs half-mast case by case (deaths of Presidents and leaders, national mourning, e.g. the
  2005 tsunami, Queen Elizabeth II's funeral 19 Sep 2022). No annual half-mast day.
- **Flag:** 2:3, horizontal halves — **red (Pantone 032 C, #EE2536) on top, white below**. In
  the red half's hoist quarter: a **white crescent** opening to the fly and, beside it on the fly
  side, **five small white five-point stars in a pentagon** (point up). For 9×3, rows 1–2 red
  with a white crescent glyph plus a star/dot cluster in cols 1–3 of row 1, row 3 all white
  (nothing is drawn in the white half). The true split is 1:1; 2 red rows + 1 white row is the
  closest fit in 3 rows.
- **Geocoder language:** `en` (the other official languages: `ms`, `zh`, `ta`).
- **Anchor cities** (Singapore is tiny; Johor Bahru, MY, is 1–2 km north across the Strait, and
  Batam, ID, 20 km south):
  - Woodlands 1.44, 103.79 (north, Causeway)
  - Tuas 1.32, 103.64 (west, Second Link)
  - Changi 1.36, 103.99 (east)
  - Pulau Ubin 1.41, 103.96 (north-east island)
  - Sentosa 1.25, 103.83 (south)
  - Jurong East 1.33, 103.74 (west-central)
  - Punggol 1.40, 103.91 (north-east)

---

## Philippines (PH)

```toml
[[holiday]]
country = "PH"
id = "new_year"
name = "Araw ng Bagong Taon (New Year's Day)"
rule = "fixed(1,1)"
dates_2026 = ["2026-01-01"]
dates_2027 = ["2027-01-01"]
dates_2028 = ["2028-01-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = true
greeting = "Manigong Bagong Taon"
greeting_en = "Happy New Year"
look = "fireworks_night"
source = ["https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html", "https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html", "https://lawphil.net/statutes/repacts/ra2007/ra_9492_2007.html"]

[[holiday]]
country = "PH"
id = "chinese_new_year"
name = "Chinese New Year (Bagong Taon ng mga Tsino)"
rule = "listed"
dates_2026 = ["2026-02-17"]
dates_2027 = ["2027-02-06"]
dates_2028 = ["2028-01-26"]
tentative_2027 = false
tentative_2028 = true
observance = false
eve = true
greeting = "Kung Hei Fat Choi"
greeting_en = "Happy Chinese New Year · Year of the {animal}"
look = "lunar_new_year"
source = ["https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html", "https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html", "https://www.timeanddate.com/holidays/philippines/2028"]

[[holiday]]
country = "PH"
id = "eid_al_fitr"
name = "Eid'l Fitr (Pista ng Ramadan / Feast of Ramadhan)"
rule = "listed"
dates_2026 = ["2026-03-21"]
dates_2027 = ["2027-03-10"]
dates_2028 = ["2028-02-27"]
tentative_2027 = true
tentative_2028 = true
observance = false
eve = true
greeting = "Eid Mubarak"
greeting_en = "Happy Eid'l Fitr {era} AH"
look = "eid"
source = ["https://lawphil.net/executive/proc/proc2026/proc_1189_2026.html", "https://lawphil.net/statutes/repacts/ra2002/ra_9177_2002.html", "https://www.philstar.com/nation/2026/03/20/2515674/moon-sighting-fails-eidl-fitr-moved-march-21-bangsamoro", "https://www.timeanddate.com/holidays/philippines/2027", "https://www.timeanddate.com/holidays/philippines/2028", "https://www.officeholidays.com/countries/philippines/2027"]

[[holiday]]
country = "PH"
id = "maundy_thursday"
name = "Huwebes Santo (Maundy Thursday)"
rule = "easter(-3)"
dates_2026 = ["2026-04-02"]
dates_2027 = ["2027-03-25"]
dates_2028 = ["2028-04-13"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Huwebes Santo"
greeting_en = "Maundy Thursday"
look = "none"
source = ["https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html", "https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html", "https://www.timeanddate.com/holidays/philippines/2028"]

[[holiday]]
country = "PH"
id = "good_friday"
name = "Biyernes Santo (Good Friday)"
rule = "easter(-2)"
dates_2026 = ["2026-04-03"]
dates_2027 = ["2027-03-26"]
dates_2028 = ["2028-04-14"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Biyernes Santo"
greeting_en = "Good Friday"
look = "none"
source = ["https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html", "https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html", "https://www.timeanddate.com/holidays/philippines/2028"]

[[holiday]]
country = "PH"
id = "black_saturday"
name = "Sabado de Gloria (Black Saturday)"
rule = "easter(-1)"
dates_2026 = ["2026-04-04"]
dates_2027 = ["2027-03-27"]
dates_2028 = ["2028-04-15"]
tentative_2027 = false
tentative_2028 = true
observance = false
eve = false
greeting = "Sabado de Gloria"
greeting_en = "Black Saturday"
look = "none"
source = ["https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html", "https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html", "https://www.timeanddate.com/holidays/philippines/2028"]

[[holiday]]
country = "PH"
id = "araw_ng_kagitingan"
name = "Araw ng Kagitingan (Day of Valor)"
rule = "fixed(4,9)"
dates_2026 = ["2026-04-09"]
dates_2027 = ["2027-04-09"]
dates_2028 = ["2028-04-09"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Araw ng Kagitingan"
greeting_en = "Day of Valor"
look = "flag"
source = ["https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html", "https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html", "https://lawphil.net/statutes/repacts/ra2007/ra_9492_2007.html"]

[[holiday]]
country = "PH"
id = "labour_day"
name = "Araw ng Paggawa (Labor Day)"
rule = "fixed(5,1)"
dates_2026 = ["2026-05-01"]
dates_2027 = ["2027-05-01"]
dates_2028 = ["2028-05-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Maligayang Araw ng Paggawa"
greeting_en = "Happy Labor Day"
look = "none"
source = ["https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html", "https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html", "https://lawphil.net/statutes/repacts/ra2007/ra_9492_2007.html"]

[[holiday]]
country = "PH"
id = "eid_al_adha"
name = "Eid'l Adha (Pista ng Sakripisyo / Feast of Sacrifice)"
rule = "listed"
dates_2026 = ["2026-05-27"]
dates_2027 = ["2027-05-17"]
dates_2028 = ["2028-05-05"]
tentative_2027 = true
tentative_2028 = true
observance = false
eve = true
greeting = "Eid Mubarak"
greeting_en = "Happy Eid'l Adha {era} AH"
look = "eid"
source = ["https://lawphil.net/executive/proc/proc2026/proc_1264_2026.html", "https://lawphil.net/statutes/repacts/ra2009/ra_9849_2009.html", "https://www.timeanddate.com/holidays/philippines/2027", "https://www.timeanddate.com/holidays/philippines/2028", "https://www.officeholidays.com/countries/philippines/2027"]

[[holiday]]
country = "PH"
id = "independence_day"
name = "Araw ng Kalayaan (Independence Day)"
rule = "fixed(6,12)"
dates_2026 = ["2026-06-12"]
dates_2027 = ["2027-06-12"]
dates_2028 = ["2028-06-12"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Maligayang Araw ng Kalayaan"
greeting_en = "Happy {nth since 1898} Independence Day, Philippines"
look = "flag"
source = ["https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html", "https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html", "https://lawphil.net/statutes/repacts/ra2007/ra_9492_2007.html"]

[[holiday]]
country = "PH"
id = "ninoy_aquino_day"
name = "Araw ni Ninoy Aquino (Ninoy Aquino Day)"
rule = "fixed(8,21)"
dates_2026 = ["2026-08-21"]
dates_2027 = ["2027-08-21"]
dates_2028 = ["2028-08-21"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Araw ni Ninoy Aquino"
greeting_en = "Ninoy Aquino Day"
look = "none"
source = ["https://lawphil.net/statutes/repacts/ra2004/ra_9256_2004.html", "https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html", "https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html"]

[[holiday]]
country = "PH"
id = "national_heroes_day"
name = "Araw ng mga Bayani (National Heroes Day)"
rule = "last_weekday(8, mon)"
dates_2026 = ["2026-08-31"]
dates_2027 = ["2027-08-30"]
dates_2028 = ["2028-08-28"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Araw ng mga Bayani"
greeting_en = "National Heroes Day"
look = "flag"
source = ["https://lawphil.net/statutes/repacts/ra2007/ra_9492_2007.html", "https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html", "https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html"]

[[holiday]]
country = "PH"
id = "all_saints_day"
name = "Undas / Araw ng mga Santo (All Saints' Day)"
rule = "fixed(11,1)"
dates_2026 = ["2026-11-01"]
dates_2027 = ["2027-11-01"]
dates_2028 = ["2028-11-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Araw ng mga Santo"
greeting_en = "All Saints' Day"
look = "candles"
source = ["https://lawphil.net/statutes/repacts/ra2007/ra_9492_2007.html", "https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html", "https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html"]

[[holiday]]
country = "PH"
id = "all_souls_day"
name = "Araw ng mga Kaluluwa (All Souls' Day)"
rule = "fixed(11,2)"
dates_2026 = ["2026-11-02"]
dates_2027 = ["2027-11-02"]
dates_2028 = ["2028-11-02"]
tentative_2027 = false
tentative_2028 = true
observance = false
eve = false
greeting = "Araw ng mga Kaluluwa"
greeting_en = "All Souls' Day"
look = "candles"
source = ["https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html", "https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html", "https://www.timeanddate.com/holidays/philippines/2028"]

[[holiday]]
country = "PH"
id = "bonifacio_day"
name = "Araw ni Bonifacio (Bonifacio Day)"
rule = "fixed(11,30)"
dates_2026 = ["2026-11-30"]
dates_2027 = ["2027-11-30"]
dates_2028 = ["2028-11-30"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Araw ni Bonifacio"
greeting_en = "Bonifacio Day"
look = "flag"
source = ["https://lawphil.net/statutes/repacts/ra2007/ra_9492_2007.html", "https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html", "https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html"]

[[holiday]]
country = "PH"
id = "immaculate_conception"
name = "Kapistahan ng Immaculada Concepcion (Feast of the Immaculate Conception of Mary)"
rule = "fixed(12,8)"
dates_2026 = ["2026-12-08"]
dates_2027 = ["2027-12-08"]
dates_2028 = ["2028-12-08"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Immaculada Concepcion"
greeting_en = "Immaculate Conception"
look = "none"
source = ["https://lawphil.net/statutes/repacts/ra2017/ra_10966_2017.html", "https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html", "https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html"]

[[holiday]]
country = "PH"
id = "christmas_eve"
name = "Bisperas ng Pasko / Noche Buena (Christmas Eve)"
rule = "fixed(12,24)"
dates_2026 = ["2026-12-24"]
dates_2027 = ["2027-12-24"]
dates_2028 = ["2028-12-24"]
tentative_2027 = false
tentative_2028 = true
observance = false
eve = false
greeting = "Maligayang Noche Buena"
greeting_en = "Christmas Eve"
look = "none"
source = ["https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html", "https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html", "https://www.timeanddate.com/holidays/philippines/2028"]

[[holiday]]
country = "PH"
id = "christmas"
name = "Araw ng Pasko (Christmas Day)"
rule = "fixed(12,25)"
dates_2026 = ["2026-12-25"]
dates_2027 = ["2027-12-25"]
dates_2028 = ["2028-12-25"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = true
greeting = "Maligayang Pasko"
greeting_en = "Merry Christmas"
look = "none"
source = ["https://lawphil.net/statutes/repacts/ra2007/ra_9492_2007.html", "https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html", "https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html"]

[[holiday]]
country = "PH"
id = "rizal_day"
name = "Araw ni Rizal (Rizal Day)"
rule = "fixed(12,30)"
dates_2026 = ["2026-12-30"]
dates_2027 = ["2027-12-30"]
dates_2028 = ["2028-12-30"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Araw ni Rizal"
greeting_en = "Rizal Day"
look = "flag"
source = ["https://lawphil.net/statutes/repacts/ra2007/ra_9492_2007.html", "https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html", "https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html"]

[[holiday]]
country = "PH"
id = "new_years_eve"
name = "Huling Araw ng Taon (Last Day of the Year)"
rule = "fixed(12,31)"
dates_2026 = ["2026-12-31"]
dates_2027 = ["2027-12-31"]
dates_2028 = ["2028-12-31"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Bisperas ng Bagong Taon"
greeting_en = "New Year's Eve"
look = "none"
source = ["https://lawphil.net/statutes/repacts/ra2007/ra_9492_2007.html", "https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html", "https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html"]

[[holiday]]
country = "PH"
id = "ramadan"
name = "Ramadan (fasting month; not a day off)"
rule = "listed"
dates_2026 = ["2026-02-19", "2026-02-20", "2026-02-21", "2026-02-22", "2026-02-23", "2026-02-24", "2026-02-25", "2026-02-26", "2026-02-27", "2026-02-28", "2026-03-01", "2026-03-02", "2026-03-03", "2026-03-04", "2026-03-05", "2026-03-06", "2026-03-07", "2026-03-08", "2026-03-09", "2026-03-10", "2026-03-11", "2026-03-12", "2026-03-13", "2026-03-14", "2026-03-15", "2026-03-16", "2026-03-17", "2026-03-18", "2026-03-19", "2026-03-20"]
dates_2027 = ["2027-02-08", "2027-02-09", "2027-02-10", "2027-02-11", "2027-02-12", "2027-02-13", "2027-02-14", "2027-02-15", "2027-02-16", "2027-02-17", "2027-02-18", "2027-02-19", "2027-02-20", "2027-02-21", "2027-02-22", "2027-02-23", "2027-02-24", "2027-02-25", "2027-02-26", "2027-02-27", "2027-02-28", "2027-03-01", "2027-03-02", "2027-03-03", "2027-03-04", "2027-03-05", "2027-03-06", "2027-03-07", "2027-03-08", "2027-03-09"]
dates_2028 = ["2028-01-28", "2028-01-29", "2028-01-30", "2028-01-31", "2028-02-01", "2028-02-02", "2028-02-03", "2028-02-04", "2028-02-05", "2028-02-06", "2028-02-07", "2028-02-08", "2028-02-09", "2028-02-10", "2028-02-11", "2028-02-12", "2028-02-13", "2028-02-14", "2028-02-15", "2028-02-16", "2028-02-17", "2028-02-18", "2028-02-19", "2028-02-20", "2028-02-21", "2028-02-22", "2028-02-23", "2028-02-24", "2028-02-25", "2028-02-26"]
tentative_2027 = true
tentative_2028 = true
observance = true
eve = true
greeting = "Ramadan Mubarak"
greeting_en = "Ramadan Mubarak"
look = "ramadan"
source = ["https://www.abs-cbn.com/news/nation/2026/2/17/moon-not-sighted-ramadan-2026-in-ph-to-begin-on-feb-19-2022", "https://www.philstar.com/nation/2026/03/20/2515674/moon-sighting-fails-eidl-fitr-moved-march-21-bangsamoro", "https://www.timeanddate.com/holidays/philippines/2027", "https://www.timeanddate.com/holidays/philippines/2028"]

[[holiday]]
country = "PH"
id = "black_nazarene"
name = "Kapistahan ng Itim na Nazareno / Traslación (Feast of the Black Nazarene; not a day off)"
rule = "fixed(1,9)"
dates_2026 = ["2026-01-09"]
dates_2027 = ["2027-01-09"]
dates_2028 = ["2028-01-09"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Viva Señor Nazareno"
greeting_en = "Feast of the Black Nazarene"
look = "none"
source = ["https://en.wikipedia.org/wiki/Feast_of_the_Black_Nazarene", "https://newsinfo.inquirer.net/2021251/jan-9-declared-a-holiday/amp"]

[[holiday]]
country = "PH"
id = "santo_nino"
name = "Kapistahan ng Santo Niño / Sinulog (Feast of the Holy Child, 3rd Sunday of January; not a day off)"
rule = "nth_weekday(1, sun, 3)"
dates_2026 = ["2026-01-18"]
dates_2027 = ["2027-01-17"]
dates_2028 = ["2028-01-16"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Pit Señor"
greeting_en = "Feast of the Santo Nino"
look = "none"
source = ["https://en.wikipedia.org/wiki/Sinulog", "https://cebudailynews.inquirer.net/tag/sinulog-2027"]

[[holiday]]
country = "PH"
id = "edsa_revolution_anniversary"
name = "Anibersaryo ng EDSA People Power (EDSA People Power Revolution Anniversary; special working day)"
rule = "fixed(2,25)"
dates_2026 = ["2026-02-25"]
dates_2027 = ["2027-02-25"]
dates_2028 = ["2028-02-25"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "EDSA People Power"
greeting_en = "EDSA People Power Day"
look = "none"
source = ["https://lawphil.net/executive/proc/proc2025/proc_1006_2025.html", "https://lawphil.net/executive/proc/proc2026/proc_1427_2026.html"]

[[holiday]]
country = "PH"
id = "easter_sunday"
name = "Pasko ng Pagkabuhay (Easter Sunday; not a day off)"
rule = "easter(0)"
dates_2026 = ["2026-04-05"]
dates_2027 = ["2027-03-28"]
dates_2028 = ["2028-04-16"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Maligayang Pasko ng Pagkabuhay"
greeting_en = "Happy Easter"
look = "none"
source = ["https://www.timeanddate.com/holidays/philippines/2028", "https://www.officeholidays.com/countries/philippines/2027"]

[[holiday]]
country = "PH"
id = "national_flag_day"
name = "Pambansang Araw ng Watawat (National Flag Day; not a day off)"
rule = "fixed(5,28)"
dates_2026 = ["2026-05-28"]
dates_2027 = ["2027-05-28"]
dates_2028 = ["2028-05-28"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Araw ng Watawat"
greeting_en = "National Flag Day"
look = "flag"
source = ["https://lawphil.net/statutes/repacts/ra1998/ra_8491_1998.html"]

[[holiday]]
country = "PH"
id = "simbang_gabi"
name = "Simbang Gabi / Misa de Gallo (nine dawn Masses before Christmas; not a day off)"
rule = "fixed(12,16)"
dates_2026 = ["2026-12-16", "2026-12-17", "2026-12-18", "2026-12-19", "2026-12-20", "2026-12-21", "2026-12-22", "2026-12-23", "2026-12-24"]
dates_2027 = ["2027-12-16", "2027-12-17", "2027-12-18", "2027-12-19", "2027-12-20", "2027-12-21", "2027-12-22", "2027-12-23", "2027-12-24"]
dates_2028 = ["2028-12-16", "2028-12-17", "2028-12-18", "2028-12-19", "2028-12-20", "2028-12-21", "2028-12-22", "2028-12-23", "2028-12-24"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Simbang Gabi"
greeting_en = "Simbang Gabi"
look = "none"
source = ["https://en.wikipedia.org/wiki/Simbang_Gabi"]
```

**Notes (PH)**

- **Legal basis:** Administrative Code 1987 s.26 as amended by RA 9492 (2007), plus RA 9256 (Ninoy
  Aquino Day), RA 10966 (Immaculate Conception), RA 9177 (Eid'l Fitr) and RA 9849 (Eid'l Adha).
  Each year a proclamation fixes the actual dates and adds the "additional special (non-working)
  days".
  - **2026:** Proclamation No. 1006, signed 3 Sep 2025 (read on lawphil.net).
  - **2027: published.** Proclamation No. 1427, signed 8 Sep 2026 (read on lawphil.net; the
    Official Gazette page is behind a Cloudflare challenge). It keeps the actual dates (no Monday
    moves) and the same four additional special days. lawphil's transcription drops the
    "D. Additional Special (Non-Working) Days" heading, so those four rows appear under
    "C. Special (Working) Day"; the WHEREAS clauses and the news reports (PhilSTAR Life, Inquirer,
    Manila Times) make clear they are additional special non-working days.
  - **2028: not published** (expected ~Aug–Sep 2027). Statutory days are `tentative_2028 = false`;
    the four "additional" days (CNY, Black Saturday, All Souls, Christmas Eve) are
    `tentative_2028 = true` because they exist only by yearly proclamation (granted in both 2026
    and 2027). timeanddate's 2028 snapshot (17 Apr 2026) lists all of them on these dates.
- **Two classes of day off:** *regular holidays* (New Year, Maundy Thursday, Good Friday, Araw ng
  Kagitingan, Labor Day, Independence Day, National Heroes Day, Bonifacio Day, Christmas, Rizal
  Day, Eid'l Fitr, Eid'l Adha — paid even if unworked) and *special non-working days* (Ninoy
  Aquino Day, All Saints, Immaculate Conception, Last Day of the Year + the four additional days —
  "no work, no pay" for the private sector, but offices and schools close). Both are listed as
  holidays.
- **No substitute days.** A holiday on a weekend is simply lost (e.g. 2027: Labor Day and
  Independence Day on Saturdays, Christmas on a Saturday; 2028: Araw ng Kagitingan on a Sunday,
  Christmas Eve and 31 Dec on Sundays). RA 9492's "Monday nearest" wording is overridden by the
  yearly proclamation ("unless otherwise modified by law and/or proclamation"); the 2026 and
  2027 proclamations keep the actual dates. A future proclamation could still move one to a
  Monday ("holiday economics"; the Marcos government did this for some 2023 holidays).
- **Eid dates.** The day off is fixed by a separate proclamation a week or two ahead, on the NCMF's
  recommendation; the religious date is set by the Bangsamoro Darul Ifta (Mufti, local sighting).
  They do not always agree:
  - 2026 Eid'l Fitr: **holiday Fri 20 Mar** (Proclamation 1189, 12 Mar; NCMF had offered 20 or
    21 Mar), but the crescent was not seen on 19 Mar and the Mufti set **Eid on Sat 21 Mar**.
    Following the NG convention (the Eid day itself in `dates_*`), 21 Mar is listed; 20 Mar was
    the proclaimed day off. Ramadan ran Thu 19 Feb (no sighting on 17 Feb) to Fri 20 Mar.
  - 2026 Eid'l Adha: Wed 27 May (Proclamation 1264, 21 May, "following the 1447 Hijrah
    calendar").
  - Earlier years: Eid'l Fitr 2025 proclaimed Tue 1 Apr (Proclamation 839; NCMF offered 31 Mar or
    1 Apr); Eid'l Adha 2024 proclaimed Mon 17 Jun (Proclamation 579).
  - **2027 (tentative):** Ramadan from Mon 8 Feb, **Eid'l Fitr Wed 10 Mar**, **Eid'l Adha Mon
    17 May** — timeanddate (snapshot 28 Sep 2026) and the crescent geometry agree (the moon is
    under an hour old at Philippine sunset on 8 Mar 2027, and the Dhu al-Hijjah conjunction on 6 May falls
    after sunset). **Disagreement:** officeholidays.com and the Umm al-Qura calendar (AlAdhan)
    give **Tue 9 Mar** and **Sun 16 May**; since the Palace proclaimed the earlier option in 2026,
    the 2027 day off could well be 9 Mar and/or 16 May.
  - **2028 (tentative):** Ramadan 28 Jan–26 Feb, **Eid'l Fitr Sun 27 Feb**, **Eid'l Adha Fri
    5 May** (timeanddate 2028 agrees; Umm al-Qura has Fitr **26 Feb**).
  - Other Islamic days (Islamic New Year, Mawlid, Isra Mi'raj, Eid'l Adha day 2) are BARMM regional
    holidays only, so they are left out.
- **Disagreements with officeholidays.com:** its 2026 page still shows Eid'l Adha on **28 May**
  (actual: 27 May); its 2027 page omits the additional special days **All Souls' Day (Tue 2 Nov)**
  and **Christmas Eve (Fri 24 Dec)** that Proclamation 1427 declares, and has the Eid estimates
  above.
- **Not days off:** EDSA anniversary (25 Feb) is a *special working day* in both 2026 and 2027.
  Other statutory special working days (First Philippine Republic Day 23 Jan, Iglesia ni Cristo
  Day 27 Jul, Yamashita Surrender Day 3 Sep, Nativity of Mary 8 Sep, Sheikh Karimul Makhdum Day
  7 Nov) are little marked and are left out. Local city/province days (Manila 24 Jun, Davao
  16 Mar, Cebu charter days, Quezon 19 Aug) are local only.
- **Observances:** the Traslación of the Black Nazarene (9 Jan, Quiapo; Manila usually declares
  a local non-working day) and the Feast of the Santo Niño (3rd Sunday of January; Sinulog in
  Cebu, Ati-Atihan in Kalibo) are the year's biggest religious crowds. Easter Sunday is marked with
  the dawn *Salubong*. National Flag Day (28 May) opens the Flag Days that run to 12 June, when RA
  8491 s.7 asks private buildings to fly the flag. Simbang Gabi is the nine pre-dawn (or
  anticipated evening) Masses, 16–24 Dec, ending with the Misa de Gallo.
- **Looks:** `flag` on the days RA 8491 s.7 names as flag-display days (Araw ng Kagitingan,
  Independence Day, National Heroes Day, Bonifacio Day, Rizal Day, Flag Day). `candles` on
  All Saints/All Souls (Undas: candles and flowers at the family graves). New Year keeps
  `fireworks_night` + `eve` (Philippine New Year's Eve is famously loud), but the generator skips
  PH's 1 Jan row in favour of the global New Year; 31 Dec is a day off and gets `none` so it does
  not double up with the global countdown. Eid uses `eid` rather than `ketupat`: ketupat
  (*ta'mu*/*katupat*) exists among Mindanao Muslims but is not the national Eid symbol it is in
  MY/ID/SG.

### Country notes (PH)

- **Christmas / New Year:** both are regular holidays, and Christmas Eve, 31 Dec, All Saints and
  All Souls are days off too. Christmas is the country's biggest season ("ber months" — carols and
  *parol* star lanterns from September; Simbang Gabi 16–24 Dec); New Year's Eve is loud with
  fireworks and *torotot* horns. **Keep both defaults.** A longer Christmas window (from 16 Dec, or
  even 1 Dec) would match local practice better than 15–26 Dec. **Built (25 Oct 2026):** PH uses
  `christmas_season`, lights from 1 Dec to the end of 6 Jan (Three Kings), instead of the default.
- **Half-mast:** **none fixed.** RA 8491 (Flag and Heraldic Code, 1998) s.23 sets half-mast only
  on the official announcement of a death: President or former President 10 days; Vice-President,
  Chief Justice, Senate President, Speaker 7 days; others as the National Historical Commission
  (formerly the Institute) decides, < 7 days. National days of mourning are declared ad hoc by
  proclamation. No annual half-mast day.
- **Flag:** 1:2. Two equal horizontal bands, **royal blue (#0038A8) on top, scarlet red (#CE1126)
  below**; a **white equilateral triangle** at the hoist whose base is the whole hoist edge; in
  its centre a **golden-yellow (#FCD116) sun with eight rays** (each ray a triple), and a
  **golden-yellow five-point star in each corner** of the triangle (three stars). (Hex values are
  the commonly used ones; RA 8491 s.4 names the colours only.) Red goes on top only in a state of
  war. For 9×3: col 1–2 white with a star at row 1 and row 3, col 1–3 of row 2 white with the sun
  `☼` and the triangle point; cols 3–9 row 1 blue, row 3 red, row 2 split blue/red (e.g. `▀`
  with blue fg on red bg).
- **Geocoder language:** `tl` (Tagalog; Filipino has no ISO 639-1 code, `fil` is 639-2), plus `en`
  (co-official; most place names are the same in both).
- **Anchor cities:**
  - Basco, Batanes 20.45, 121.97 (far north, toward Taiwan)
  - Laoag 18.20, 120.59 (north-west Luzon)
  - Puerto Princesa 9.74, 118.74 (Palawan, west)
  - Zamboanga City 6.91, 122.07 (south-west Mindanao)
  - Jolo 6.05, 121.00 (Sulu)
  - Bongao, Tawi-Tawi 5.03, 119.77 (south-west tip, toward Sabah, MY)
  - General Santos 6.11, 125.17 (south, toward Indonesia's Sangihe islands)
  - Davao City 7.07, 125.61 (south-east)

---

## Thailand (TH)

```toml
[[holiday]]
country = "TH"
id = "new_year"
name = "วันขึ้นปีใหม่ (New Year's Day)"
rule = "fixed(1,1)"
dates_2026 = ["2026-01-01"]
dates_2027 = ["2027-01-01"]
dates_2028 = ["2028-01-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = true
greeting = "Sawatdi Pi Mai"
greeting_en = "Happy New Year"
look = "fireworks_night"
source = ["https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2569/EngPDF/25690175.pdf", "https://isomer-user-content.by.gov.sg/251/8e1ca282-3965-4e24-8b61-764cc74923bd/Thai%20Public%20Holidays%20Updated%20May%202026.pdf", "https://www.officeholidays.com/countries/thailand/2027"]

[[holiday]]
country = "TH"
id = "makha_bucha"
name = "วันมาฆบูชา (Makha Bucha Day)"
rule = "listed"
dates_2026 = ["2026-03-03"]
dates_2027 = ["2027-02-21"]
dates_2028 = ["2028-02-10"]
tentative_2027 = false
tentative_2028 = true
observance = false
eve = false
greeting = "Wan Makha Bucha"
greeting_en = "Makha Bucha Day"
look = "candles"
source = ["https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2569/EngPDF/25690175.pdf", "https://isomer-user-content.by.gov.sg/251/8e1ca282-3965-4e24-8b61-764cc74923bd/Thai%20Public%20Holidays%20Updated%20May%202026.pdf", "https://www.officeholidays.com/countries/thailand/2027"]

[[holiday]]
country = "TH"
id = "chakri_day"
name = "วันจักรี (Chakri Memorial Day)"
rule = "fixed(4,6)"
dates_2026 = ["2026-04-06"]
dates_2027 = ["2027-04-06"]
dates_2028 = ["2028-04-06"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Wan Chakri"
greeting_en = "Chakri Memorial Day"
look = "none"
source = ["https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2569/EngPDF/25690175.pdf", "https://isomer-user-content.by.gov.sg/251/8e1ca282-3965-4e24-8b61-764cc74923bd/Thai%20Public%20Holidays%20Updated%20May%202026.pdf"]

[[holiday]]
country = "TH"
id = "songkran"
name = "วันสงกรานต์ (Songkran Festival, Thai New Year)"
rule = "fixed(4,13)"
dates_2026 = ["2026-04-13", "2026-04-14", "2026-04-15"]
dates_2027 = ["2027-04-13", "2027-04-14", "2027-04-15"]
dates_2028 = ["2028-04-13", "2028-04-14", "2028-04-15"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Suk San Wan Songkran"
greeting_en = "Happy Songkran"
look = "songkran"
source = ["https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2569/EngPDF/25690175.pdf", "https://isomer-user-content.by.gov.sg/251/8e1ca282-3965-4e24-8b61-764cc74923bd/Thai%20Public%20Holidays%20Updated%20May%202026.pdf"]

[[holiday]]
country = "TH"
id = "labour_day"
name = "วันแรงงานแห่งชาติ (National Labour Day; banks and private sector)"
rule = "fixed(5,1)"
dates_2026 = ["2026-05-01"]
dates_2027 = ["2027-05-01"]
dates_2028 = ["2028-05-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Wan Raeng-ngan Haeng Chat"
greeting_en = "Happy Labour Day"
look = "none"
source = ["https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2569/EngPDF/25690175.pdf", "https://www.thairath.co.th/news/society/2929548", "https://www.officeholidays.com/countries/thailand/2026"]

[[holiday]]
country = "TH"
id = "coronation_day"
name = "วันฉัตรมงคล (Coronation Day)"
rule = "fixed(5,4)"
dates_2026 = ["2026-05-04"]
dates_2027 = ["2027-05-04"]
dates_2028 = ["2028-05-04"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Wan Chat Mongkhon"
greeting_en = "Coronation Day"
look = "none"
source = ["https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2569/EngPDF/25690175.pdf", "https://isomer-user-content.by.gov.sg/251/8e1ca282-3965-4e24-8b61-764cc74923bd/Thai%20Public%20Holidays%20Updated%20May%202026.pdf"]

[[holiday]]
country = "TH"
id = "royal_ploughing_ceremony"
name = "วันพืชมงคล (Royal Ploughing Ceremony Day; government offices only)"
rule = "listed"
dates_2026 = ["2026-05-13"]
dates_2027 = []
dates_2028 = []
tentative_2027 = true
tentative_2028 = true
observance = false
eve = false
greeting = "Wan Phuetchamongkhon"
greeting_en = "Royal Ploughing Ceremony"
look = "none"
source = ["https://www.thansettakij.com/lifestyle/658280", "https://www.thairath.co.th/news/society/2929548"]

[[holiday]]
country = "TH"
id = "vesak"
name = "วันวิสาขบูชา (Visakha Bucha Day)"
rule = "listed"
dates_2026 = ["2026-05-31"]
dates_2027 = ["2027-05-20"]
dates_2028 = ["2028-05-08"]
tentative_2027 = false
tentative_2028 = true
observance = false
eve = false
greeting = "Wan Wisakha Bucha"
greeting_en = "Visakha Bucha Day"
look = "candles"
source = ["https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2569/EngPDF/25690175.pdf", "https://isomer-user-content.by.gov.sg/251/8e1ca282-3965-4e24-8b61-764cc74923bd/Thai%20Public%20Holidays%20Updated%20May%202026.pdf", "https://www.officeholidays.com/countries/thailand/2027"]

[[holiday]]
country = "TH"
id = "queen_suthida_birthday"
name = "วันเฉลิมพระชนมพรรษา สมเด็จพระราชินี (H.M. Queen Suthida's Birthday)"
rule = "fixed(6,3)"
dates_2026 = ["2026-06-03"]
dates_2027 = ["2027-06-03"]
dates_2028 = ["2028-06-03"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Song Phra Charoen"
greeting_en = "Long Live the Queen"
look = "none"
source = ["https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2569/EngPDF/25690175.pdf", "https://isomer-user-content.by.gov.sg/251/8e1ca282-3965-4e24-8b61-764cc74923bd/Thai%20Public%20Holidays%20Updated%20May%202026.pdf"]

[[holiday]]
country = "TH"
id = "asarnha_bucha"
name = "วันอาสาฬหบูชา (Asarnha Bucha Day)"
rule = "listed"
dates_2026 = ["2026-07-29"]
dates_2027 = ["2027-07-18"]
dates_2028 = ["2028-07-06"]
tentative_2027 = false
tentative_2028 = true
observance = false
eve = false
greeting = "Wan Asanha Bucha"
greeting_en = "Asarnha Bucha Day"
look = "candles"
source = ["https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2569/EngPDF/25690175.pdf", "https://isomer-user-content.by.gov.sg/251/8e1ca282-3965-4e24-8b61-764cc74923bd/Thai%20Public%20Holidays%20Updated%20May%202026.pdf", "https://www.officeholidays.com/countries/thailand/2027"]

[[holiday]]
country = "TH"
id = "khao_phansa"
name = "วันเข้าพรรษา (Khao Phansa, Buddhist Lent Day; government offices only)"
rule = "listed"
dates_2026 = ["2026-07-30"]
dates_2027 = ["2027-07-19"]
dates_2028 = ["2028-07-07"]
tentative_2027 = false
tentative_2028 = true
observance = false
eve = false
greeting = "Wan Khao Phansa"
greeting_en = "Buddhist Lent Day"
look = "candles"
source = ["https://isomer-user-content.by.gov.sg/251/8e1ca282-3965-4e24-8b61-764cc74923bd/Thai%20Public%20Holidays%20Updated%20May%202026.pdf", "https://www.officeholidays.com/countries/thailand/2026"]

[[holiday]]
country = "TH"
id = "king_birthday"
name = "วันเฉลิมพระชนมพรรษา พระบาทสมเด็จพระเจ้าอยู่หัว (H.M. King Vajiralongkorn's Birthday)"
rule = "fixed(7,28)"
dates_2026 = ["2026-07-28"]
dates_2027 = ["2027-07-28"]
dates_2028 = ["2028-07-28"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Song Phra Charoen"
greeting_en = "Long Live the King"
look = "flag"
source = ["https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2569/EngPDF/25690175.pdf", "https://image.mfa.go.th/mfa/0/2QL32ZatFo/A_King/Official_Holidays_2026_(B.E._2569)_rev_August_2026.pdf", "https://www.officeholidays.com/countries/thailand/2026"]

[[holiday]]
country = "TH"
id = "mothers_day"
name = "วันแม่แห่งชาติ (Queen Sirikit the Queen Mother's Birthday Anniversary and National Mother's Day)"
rule = "fixed(8,12)"
dates_2026 = ["2026-08-12"]
dates_2027 = ["2027-08-12"]
dates_2028 = ["2028-08-12"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Suk San Wan Mae"
greeting_en = "Happy Mother's Day"
look = "candles"
source = ["https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2569/EngPDF/25690175.pdf", "https://isomer-user-content.by.gov.sg/251/8e1ca282-3965-4e24-8b61-764cc74923bd/Thai%20Public%20Holidays%20Updated%20May%202026.pdf"]

[[holiday]]
country = "TH"
id = "king_bhumibol_memorial_day"
name = "วันนวมินทรมหาราช (H.M. King Bhumibol Adulyadej the Great Memorial Day)"
rule = "fixed(10,13)"
dates_2026 = ["2026-10-13"]
dates_2027 = ["2027-10-13"]
dates_2028 = ["2028-10-13"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Wan Nawamintha Maharat"
greeting_en = "King Bhumibol Memorial Day"
look = "none"
source = ["https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2569/EngPDF/25690175.pdf", "https://isomer-user-content.by.gov.sg/251/8e1ca282-3965-4e24-8b61-764cc74923bd/Thai%20Public%20Holidays%20Updated%20May%202026.pdf"]

[[holiday]]
country = "TH"
id = "chulalongkorn_day"
name = "วันปิยมหาราช (H.M. King Chulalongkorn the Great Memorial Day)"
rule = "fixed(10,23)"
dates_2026 = ["2026-10-23"]
dates_2027 = ["2027-10-23"]
dates_2028 = ["2028-10-23"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Wan Piya Maharat"
greeting_en = "Chulalongkorn Day"
look = "none"
source = ["https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2569/EngPDF/25690175.pdf", "https://isomer-user-content.by.gov.sg/251/8e1ca282-3965-4e24-8b61-764cc74923bd/Thai%20Public%20Holidays%20Updated%20May%202026.pdf"]

[[holiday]]
country = "TH"
id = "national_day"
name = "วันชาติ และวันพ่อแห่งชาติ (King Bhumibol the Great's Birthday Anniversary, National Day and Father's Day)"
rule = "fixed(12,5)"
dates_2026 = ["2026-12-05"]
dates_2027 = ["2027-12-05"]
dates_2028 = ["2028-12-05"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Suk San Wan Pho"
greeting_en = "Happy Father's Day"
look = "flag"
source = ["https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2569/EngPDF/25690175.pdf", "https://isomer-user-content.by.gov.sg/251/8e1ca282-3965-4e24-8b61-764cc74923bd/Thai%20Public%20Holidays%20Updated%20May%202026.pdf"]

[[holiday]]
country = "TH"
id = "constitution_day"
name = "วันรัฐธรรมนูญ (Constitution Day)"
rule = "fixed(12,10)"
dates_2026 = ["2026-12-10"]
dates_2027 = ["2027-12-10"]
dates_2028 = ["2028-12-10"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Wan Ratthathammanun"
greeting_en = "Constitution Day"
look = "flag"
source = ["https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2569/EngPDF/25690175.pdf", "https://isomer-user-content.by.gov.sg/251/8e1ca282-3965-4e24-8b61-764cc74923bd/Thai%20Public%20Holidays%20Updated%20May%202026.pdf"]

[[holiday]]
country = "TH"
id = "new_years_eve"
name = "วันสิ้นปี (New Year's Eve)"
rule = "fixed(12,31)"
dates_2026 = ["2026-12-31"]
dates_2027 = ["2027-12-31"]
dates_2028 = ["2028-12-31"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Wan Sin Pi"
greeting_en = "New Year's Eve"
look = "none"
source = ["https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2569/EngPDF/25690175.pdf", "https://isomer-user-content.by.gov.sg/251/8e1ca282-3965-4e24-8b61-764cc74923bd/Thai%20Public%20Holidays%20Updated%20May%202026.pdf"]

[[holiday]]
country = "TH"
id = "childrens_day"
name = "วันเด็กแห่งชาติ (National Children's Day, 2nd Saturday of January; not a day off)"
rule = "nth_weekday(1, sat, 2)"
dates_2026 = ["2026-01-10"]
dates_2027 = ["2027-01-09"]
dates_2028 = ["2028-01-08"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Wan Dek Haeng Chat"
greeting_en = "Happy Children's Day"
look = "none"
source = ["https://www.timeanddate.com/holidays/thailand/2027"]

[[holiday]]
country = "TH"
id = "chinese_new_year"
name = "วันตรุษจีน (Chinese New Year; regional holiday in the far south only)"
rule = "listed"
dates_2026 = ["2026-02-17"]
dates_2027 = ["2027-02-06"]
dates_2028 = ["2028-01-26"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = true
greeting = "Sin Chia Yu Ee"
greeting_en = "Happy Chinese New Year · Year of the {animal}"
look = "lunar_new_year"
source = ["https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2569/EngPDF/25690175.pdf", "https://www.officeholidays.com/countries/thailand/2027"]

[[holiday]]
country = "TH"
id = "national_flag_day"
name = "วันพระราชทานธงชาติไทย (National Thai Flag Day; not a day off)"
rule = "fixed(9,28)"
dates_2026 = ["2026-09-28"]
dates_2027 = ["2027-09-28"]
dates_2028 = ["2028-09-28"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Wan Phra Ratchathan Thong Chat Thai"
greeting_en = "Thai National Flag Day"
look = "flag"
source = ["https://en.wikipedia.org/wiki/Flag_of_Thailand"]

[[holiday]]
country = "TH"
id = "ok_phansa"
name = "วันออกพรรษา (Ok Phansa, end of Buddhist Lent; not a day off)"
rule = "listed"
dates_2026 = ["2026-10-26"]
dates_2027 = ["2027-10-15"]
dates_2028 = ["2028-10-03"]
tentative_2027 = false
tentative_2028 = true
observance = true
eve = false
greeting = "Wan Ok Phansa"
greeting_en = "End of Buddhist Lent"
look = "none"
source = ["https://thaiholidayguide.com/events/festivals-in-thailand"]

[[holiday]]
country = "TH"
id = "loy_krathong"
name = "วันลอยกระทง (Loy Krathong; not a day off)"
rule = "listed"
dates_2026 = ["2026-11-24"]
dates_2027 = ["2027-11-13"]
dates_2028 = ["2028-11-01"]
tentative_2027 = false
tentative_2028 = true
observance = true
eve = false
greeting = "Suk San Wan Loi Krathong"
greeting_en = "Happy Loy Krathong"
look = "lanterns"
source = ["https://www.esimpass.com/en/events/loy-krathong", "https://thaiholidayguide.com/events/festivals-in-thailand"]
```

**Notes (TH)**

- **Whose holidays:** Thailand has three overlapping lists. *Government holidays* (วันหยุดราชการ) are
  set by Cabinet resolution each year; *bank holidays* by a Bank of Thailand notification under the
  Financial Institution Business Act s.37; *private-sector holidays* are at least 13 days chosen by
  each employer from the official list (Labour Protection Act s.29, and 1 May is mandatory). The
  entries above are the union of the government and bank lists, which is what people treat as
  "public holidays". Differences: **Labour Day** is a bank and private-sector holiday but
  government offices stay open (2026: Thai news reports, e.g. Thairath, Sanook); **Royal Ploughing
  Day** and **Khao Phansa** are government-only (banks and most firms open).
- **2026:** the government/bank list checked against the Singapore Embassy Bangkok's list
  (updated May 2026), the Royal Thai Embassy Abu Dhabi's list (MFA, rev. Aug 2026) and
  officeholidays.com. Extra days: **Fri 2 Jan** (special holiday, nationwide); **Mon 1 Jun**
  (substitute for Visakha Bucha, Sun 31 May); **Mon 7 Dec** (substitute for 5 Dec, a Saturday);
  **Fri 16 Oct** Bangkok only (Cabinet 19 May 2026; IMF/World Bank Annual Meetings; BOT
  notification 26/2569); news reports a flood-related special holiday on Mon 28–Tue 29 Sep in
  Bangkok, Nonthaburi, Pathum Thani and Samut Prakan only. Royal Ploughing Day was **Wed 13 May
  2026** (government only).
- **2027: bank list official**, BOT Notification No. 37/2569 (announced 17 Aug 2026, Royal Gazette
  vol. 143 special part 202 ง, 25 Aug 2026; read from the BOT's English PDF). It fixes the lunar
  days: **Makha Bucha Sun 21 Feb**, **Visakha Bucha Thu 20 May**, **Asarnha Bucha Sun 18 Jul**.
  Substitutes: Mon 22 Feb (Makha), Mon 3 May (Labour Day), Mon 19 Jul (Asarnha), Mon 25 Oct
  (Chulalongkorn, Sat 23 Oct), Mon 6 Dec (5 Dec, Sunday). The **2027 Cabinet (government) list is
  not out yet** (usually Nov–Dec). Khao Phansa 2027 is therefore **Mon 19 Jul** by the lunar
  calendar; for government offices the Asarnha substitute then moves to Tue 20 Jul (expected, not
  announced). Royal Ploughing 2027 is not announced (the Bureau of the Royal Household picks an
  auspicious day each year; it is not computable), so its 2027/2028 lists are empty.
  Per the same notice, CNY and the two Eids are extra bank holidays only in Narathiwat, Pattani,
  Yala, Satun and Songkhla.
- **2028: nothing published.** Civic dates are fixed; lunar dates are computed (tentative).
  Expected substitutes: Mon 3 Jan (1 Jan Sat), Mon 17 Apr (Songkran day 3 Sat), Mon 5 Jun (3 Jun
  Sat), Mon 14 Aug (12 Aug Sat), Mon 11 Dec (10 Dec Sun), and 31 Dec (Sun) → a weekday in early
  Jan 2029.
- **Thai lunar dates (how computed).** Thai lunar months alternate 29 days (odd months) and 30 days
  (even months); month 8 is always 30 days; an *adhikavara* year gives month 7 a 30th day; an
  *adhikamasa* year repeats month 8 (8/8, 30 days). Makha Bucha = 15th waxing day of month 3
  (month 4 in an adhikamasa year), Visakha Bucha = 15th of month 6 (7 in adhikamasa), Asarnha
  Bucha = 15th of month 8 (of 8/8 in adhikamasa), Khao Phansa = the next day (1st waning of 8),
  Ok Phansa = 15th of month 11, Loy Krathong = 15th of month 12. Stepping these month lengths
  forward from Makha Bucha 2024 (24 Feb) with 2024 normal, **2025 adhikavara**, **2026
  adhikamasa** and 2027 normal reproduces every official date for 2024–2027 (incl. Loy Krathong
  15 Nov 2024 and 5 Nov 2025, Ok Phansa 17 Oct 2024 / 7 Oct 2025) and the BOT 2027 dates.
  **2028 is taken as a normal year**: it is not adhikamasa (leap-month years follow the 19-year
  cycle 2004, 2007, 2010, 2012, 2015, 2018, 2021, 2023, 2026 → next 2029; 2028 corresponds to
  2009, a year without a leap month (Makha Bucha 9 Feb); a 2028 leap month would put Makha Bucha on
  10 Mar and Visakha Bucha on 7 Jun, and leave 2029's Makha Bucha on an impossible ~29 Jan), and
  not adhikavara (by my reading of past calendars adhikavara fell in 2016, 2020 and 2025, every
  4–5 years, as the 354-day year loses ~0.37 day a year; the drift since 2025 is only ~0.6 day by
  2028). That gives **Makha Bucha Thu 10 Feb**, **Visakha Bucha Mon 8 May**, **Asarnha Bucha
  Thu 6 Jul**, **Khao Phansa Fri 7 Jul**, Ok Phansa Tue 3 Oct, Loy Krathong Wed 1 Nov 2028. Each
  falls on, or the day before, the astronomical full moon in Thai time (my Meeus computation:
  10 Feb 22:04, 9 May 02:49, 7 Jul 01:11, 3 Oct 23:25, 2 Nov 16:17 ICT). If 2028 turned out to be
  adhikavara, Asarnha/Khao Phansa/Ok Phansa/Loy Krathong would each move one day later (7 Jul,
  8 Jul, 4 Oct, 2 Nov); Makha and Visakha would not change.
- **Disagreements:**
  - officeholidays.com 2027 puts Asarnha Bucha on **Mon 19 Jul** and Buddhist Lent on **Tue 20 Jul**
    (BOT: Asarnha **Sun 18 Jul**, so Lent is Mon 19 Jul); lists "New Year's Day Holiday" Sat 2 Jan
    2027 (no such day in the BOT notice) and Royal Ploughing 20 May 2027 "to be confirmed"; its 2026
    page has Royal Ploughing on 11 May (actual 13 May) and lists Prince Mahidol Day (24 Sep) as a
    national holiday — it is an observance only (timeanddate agrees).
  - Loy Krathong 2026: several travel sites give **25 Nov**; the lunar calendar (Ok Phansa 26 Oct,
    which every source agrees on, plus the 29-day 11th month) gives **Tue 24 Nov**, the night of the
    full moon (21:53 ICT).
  - The Singapore Embassy's 2026 list omits the King's Birthday (28 Jul), which every other source,
    the MFA list included, has.
- **Islamic and Chinese days** (Eid al-Fitr, Eid al-Adha, CNY) are regional only (the four/five
  southern border provinces), so Eid is left out and CNY is an observance (Yaowarat and Thai-
  Chinese families mark it nationwide). No Ramadan block for Thailand (Eid is not a national
  holiday).
- **Royal days:** Queen Sirikit the Queen Mother died on 24 Oct 2025; 12 Aug stays a holiday
  ("Birthday Anniversary and National Mother's Day" in the BOT 2027 notice). No new memorial
  holiday for her has been added as of the 2027 bank list.
- **Looks:** `candles` for Makha/Visakha/Asarnha Bucha (the evening *wian thian* candle
  procession round the temple) and Khao Phansa (the candle festival, e.g. Ubon Ratchathani);
  `candles` on Mother's Day for the candle-lighting tribute; `flag` on the King's Birthday, 5 Dec
  and Constitution Day; `lanterns` for Loy Krathong (Yi Peng sky lanterns in Chiang Mai).
  Songkran has no fitting look (see the final report). National Day is the 5 Dec "National Day /
  Father's Day"; Thais greet it as Father's Day, hence `greeting_en`.

### Country notes (TH)

- **Christmas / New Year:** 1 Jan and 31 Dec are public holidays and New Year's Eve countdowns with
  fireworks (Bangkok riverside, CentralWorld, ICONSIAM) are big — keep the New Year countdown.
  **Christmas is not a holiday** and Christians are ~1%, but malls and hotels decorate heavily
  from November, so Christmas lights are recognised; keeping the 15–26 Dec lights is fine, without
  a "Merry Christmas" greeting being expected.
- **Half-mast:** **none fixed.** Under the Flag Act B.E. 2522 (1979) and the Prime Minister's Office
  Regulation on the use, hoisting or display of the national flag B.E. 2529 (amended to No. 4,
  B.E. 2560), the flag is lowered only when the Government orders mourning, case by case: 30 days
  from 14 Oct 2016 (King Bhumibol); 30 days from 25 Oct 2025 for Queen Sirikit (Cabinet special
  meeting 25 Oct 2025, with 1 year of mourning for officials); 15 days from 12 Jun 2026 (Cabinet
  order reported that day). 13 Oct is a memorial holiday, not a half-mast day.
- **Flag (ธงไตรรงค์, Trairanga):** 2:3, five horizontal stripes **red, white, blue, white, red** in
  heights **1:1:2:1:1** (blue double). Colours per the 2017 PM's Office standard: red **#A51931**,
  white **#F4F5F8**, blue **#2D2A4A** (NSTDA-derived values of the CIELAB spec). For 9×3: row 1
  `▀` red fg on white bg (red over white), row 2 solid blue, row 3 `▄` red fg on white bg (white
  over red).
- **Geocoder language:** `th`.
- **Anchor cities:**
  - Mae Sai 20.43, 99.88 (northernmost, Myanmar border)
  - Chiang Rai 19.91, 99.83 (north, Golden Triangle)
  - Mae Sot 16.71, 98.57 (west, Myanmar border)
  - Nong Khai 17.88, 102.74 (Mekong, Laos border)
  - Ubon Ratchathani 15.24, 104.85 (east, Laos/Cambodia)
  - Aranyaprathet 13.69, 102.51 (Cambodia border)
  - Hat Yai 7.01, 100.47 (south)
  - Sungai Kolok 6.03, 101.97 (Malaysia border)

---

## Counts

| Country | Holidays (days off) | Observances | Half-mast | 2027 status | 2028 status |
|---|---|---|---|---|---|
| SG | 10 gazetted (11 days; CNY ×2) | 6 (Ramadan, Thaipusam, Total Defence Day, Duanwu, Hungry Ghost, Mid-Autumn) | none fixed | official (MOM, data.gov.sg) | not published; computed, lunar days tentative |
| PH | 20 (12 regular incl. 2 Eids, 8 special non-working) | 7 (Ramadan, Black Nazarene, Santo Niño, EDSA, Easter Sunday, Flag Day, Simbang Gabi) | none fixed | official (Proclamation 1427) except the two Eids | not published; statutory days firm, 4 additional days + Eids tentative |
| TH | 18 (incl. Labour Day bank/private only; Royal Ploughing and Khao Phansa government only) | 5 (Children's Day, CNY, Thai Flag Day, Ok Phansa, Loy Krathong) | none fixed | bank list official (BOT 37/2569); Cabinet list pending; Royal Ploughing unknown | not published; lunar days computed (tentative), Royal Ploughing unknown |
