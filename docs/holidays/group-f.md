# World holidays — group F: EG, TR, DE, FR

Compiled 23 Oct 2026 for the weathrz holiday greetings and decorations (2026, 2027, 2028).
All four countries complete (EG, TR, DE, FR); counts at the end.

**Schema conventions used below**

- Same as group C / D / E. `rule` is one string from the fixed grammar; `listed` covers lunar
  (Hijri), Coptic and announced dates. Weekday tokens are `mon tue wed thu fri sat sun`.
- `dates_*` hold the holiday day itself; multi-day blocks (Eid, bayram) list every day.
  Moved, bridge and substitute days off are in Notes.
- `tentative_2027 = true` / `tentative_2028 = true` mean the **date** may still move (moon
  sighting, Egypt's cabinet moving a day to a Thursday, or a list not yet published); each
  country's Notes say which.
- `observance = true` marks a day that is not a day off (Ramadan month, half-mast days, other
  widely marked days).
- `eve = true` when the evening before belongs to the day (Islamic days begin at sunset; Turkey's
  arife half-day; New Year's Eve).
- `look = "half_mast"` is the new look for days the national flag is officially flown at
  half-mast. When a half-mast day is also a public holiday listed here, it is a separate entry with
  id `<holiday_id>_half_mast`, so the holiday keeps its own greeting and look. A half-mast day
  that is not a holiday is a single entry with `look = "half_mast"` and `observance = true`.
  None of the four countries has a half-mast day that falls on one of its public holidays, so
  there are no `_half_mast` twins in this file.
- Easter Sunday is 2026-04-05, 2027-03-28 and 2028-04-16 (same as group C). Orthodox/Coptic
  Easter is 2026-04-12, 2027-05-02 and 2028-04-16 (generator `orthodox()`).

**Primary sources checked**

| Country | Official source | Cross-check |
|---|---|---|
| EG | PM Decree [1193/2018](https://manshurat.org/node/44922) (Official Gazette 26 bis, 2 Jul 2018; PDF read) and its amendment [1249/2020](https://manshurat.org/node/66981) (day-swap power); the 2026 PM decrees as published by the [Cairo Governorate portal](https://www.cairo.gov.eg/en/) (linked per entry); Dar al-Ifta statements on [Ramadan](https://www.dar-alifta.org/ar/Articles/Details/11039/) and [Shawwal 1447](https://dar-alifta.org/ar/articles/details/11079); NRIAG 2027 predictions via [banker.news](https://banker.news/155540) | officeholidays.com [2026](https://www.officeholidays.com/countries/egypt/2026) / [2027](https://www.officeholidays.com/countries/egypt/2027); timeanddate.com [2027](https://www.timeanddate.com/holidays/egypt/2027) / [2028](https://www.timeanddate.com/holidays/egypt/2028) via Wayback (8 Feb / 5 Mar 2026); AlAdhan Umm al-Qura; Ahram Online, Egypt Today, Arab Finance, Egyptian Streets; own Meeus new-moon and Cairo moonset-lag calculation |
| TR | [Law 2429](https://www.mevzuat.gov.tr/mevzuat?MevzuatNo=2429&MevzuatTur=1&MevzuatTertip=5) on National and General Holidays (read); Diyanet official-holiday tables [2026](https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=158) / [2027](https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=159) / [2028](https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=193) and religious-day lists [2026](https://vakithesaplama.diyanet.gov.tr/dinigunler.php?yil=2026) / [2027](https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=154) / [2028](https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=185); [Turkish Flag Regulation 85/9034](https://www.mevzuat.gov.tr/mevzuat?MevzuatNo=859034&MevzuatTur=2&MevzuatTertip=5) art. 24 (read) | officeholidays.com [2026](https://www.officeholidays.com/countries/turkey/2026) / [2027](https://www.officeholidays.com/countries/turkey/2027); timeanddate.com [2028](https://www.timeanddate.com/holidays/turkey/2028) via Wayback (16 Apr 2026); Milliyet, NTV, CNN Türk |
| DE | [BMI "Nationale Feiertage"](https://www.bmi.bund.de/DE/themen/verfassung/staatliche-symbole/nationale-feiertage/nationale-feiertage-node.html) (the nine holidays protected in every state; read); federal flag decree of 2005 via [Baden-Württemberg municipal copy](https://www.zaberfeld.de/resources/ecics_3320.pdf) and [Brandenburg VV Beflaggung 2018](https://bravors.brandenburg.de/verwaltungsvorschriften/beflaggung_2018) (read); [Bundesregierung on 11 March](https://www.bundesregierung.de/breg-de/aktuelles/11-maerz-nationaler-gedenktag-2004930) | officeholidays.com [2026](https://www.officeholidays.com/countries/germany/2026) / [2027](https://www.officeholidays.com/countries/germany/2027); timeanddate.com [2027](https://www.timeanddate.com/holidays/germany/2027) via Wayback (10 Mar 2026; no 2028 snapshot); 2028 computed from Easter |
| FR | [service-public.fr F2405](https://www.service-public.fr/particuliers/vosdroits/F2405) (fêtes légales 2026 and 2027 for the general case, Alsace-Moselle and overseas; verified 1 Jan 2026, read); Code du travail art. L3133-1 | officeholidays.com [2026](https://www.officeholidays.com/countries/france/2026) / [2027](https://www.officeholidays.com/countries/france/2027); timeanddate.com [2027](https://www.timeanddate.com/holidays/france/2027) via Wayback (7 Mar 2026; no 2028 snapshot); 2028 computed from Easter; Assemblée nationale written answer and Public Sénat (flag practice) |

Fetch problems (23 Oct 2026):

- EG: the gazette text of Labour Minister Decree 294/2025 was not found online, so it is cited
  from Egyptian press. manshurat.org timed out twice on the PDF of Law 41/2014 (flag law).
  akhbarelyom.com returned 403.
- DE: the federal flag decree (Beflaggungserlass 2005) is not on gesetze-im-internet.de (404) and
  BMI has no page for it, so its list was read from a Baden-Württemberg municipal copy and
  Brandenburg's own VV. bmi.bund.de only answers after a cookie round-trip.
- FR: the Interior Ministry's "Protocole à l'usage des maires" PDF (doubs.gouv.fr) returned an
  HTML page instead of the file.
- All: officeholidays.com has no 2028 pages (404). timeanddate.com blocks scripts, so its tables
  were read from Wayback snapshots (EG 2027/2028, TR 2028, DE 2027, FR 2027; no DE or FR 2028
  snapshot exists). DE and FR 2028 dates follow from fixed dates and Easter.

---

## Egypt (EG)

```toml
[[holiday]]
country = "EG"
id = "coptic_christmas"
name = "عيد الميلاد المجيد (Coptic Christmas)"
rule = "fixed(1,7)"
dates_2026 = ["2026-01-07"]
dates_2027 = ["2027-01-07"]
dates_2028 = ["2028-01-07"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = true
greeting = "عيد ميلاد مجيد"
greeting_en = "Merry Christmas"
look = "candles"
source = ["https://manshurat.org/node/44922", "https://www.cairo.gov.eg/en/news/news-eng/2026/d09dd057a7e5495487cc136f7e95434d", "https://www.officeholidays.com/countries/egypt/2027", "https://www.timeanddate.com/holidays/egypt/2028"]

[[holiday]]
country = "EG"
id = "revolution_police_day"
name = "عيد ثورة 25 يناير وعيد الشرطة (25 January Revolution and Police Day)"
rule = "fixed(1,25)"
dates_2026 = ["2026-01-25"]
dates_2027 = ["2027-01-25"]
dates_2028 = ["2028-01-25"]
tentative_2027 = true
tentative_2028 = true
observance = false
eve = false
greeting = "عيد ثورة 25 يناير وعيد الشرطة"
greeting_en = "Revolution & Police Day"
look = "flag"
source = ["https://manshurat.org/node/44922", "https://www.cairo.gov.eg/en/news/news-eng/2026/bd6a361fe04f4d268157c39e3103c96d", "https://www.officeholidays.com/countries/egypt/2027", "https://www.timeanddate.com/holidays/egypt/2028"]

[[holiday]]
country = "EG"
id = "eid_al_fitr"
name = "عيد الفطر المبارك (Eid al-Fitr)"
rule = "listed"
dates_2026 = ["2026-03-20", "2026-03-21", "2026-03-22"]
dates_2027 = ["2027-03-09", "2027-03-10", "2027-03-11"]
dates_2028 = ["2028-02-27", "2028-02-28", "2028-02-29"]
tentative_2027 = true
tentative_2028 = true
observance = false
eve = true
greeting = "عيد فطر مبارك"
greeting_en = "Happy Eid al-Fitr {era} AH"
look = "eid"
source = ["https://dar-alifta.org/ar/articles/details/11079", "https://manshurat.org/node/44922", "https://cairo.gov.eg/en/news/news-eng/2026/0a1f1510fc5940479a6f3bbab133859f", "https://banker.news/155540", "https://www.officeholidays.com/countries/egypt/2027", "https://www.timeanddate.com/holidays/egypt/2028", "https://api.aladhan.com/v1/gToH/26-02-2028?calendarMethod=UAQ", "https://egyptianstreets.com/2025/03/29/egypt-announces-monday-as-the-first-day-of-eid-al-fitr/"]

[[holiday]]
country = "EG"
id = "sham_el_nessim"
name = "شم النسيم (Sham el-Nessim, spring festival)"
rule = "orthodox_easter(1)"
dates_2026 = ["2026-04-13"]
dates_2027 = ["2027-05-03"]
dates_2028 = ["2028-04-17"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "شم نسيم سعيد"
greeting_en = "Happy Sham el-Nessim"
look = "easter_eggs"
source = ["https://manshurat.org/node/44922", "https://cairo.gov.eg/en/news/news-eng/2026/3b7ca4dd41e243c888fa75480453fedb", "https://arabfinance.com/en/news/newdetails/egypt-declares-april-13th-public-holiday-for-sham-el-nessim", "https://www.officeholidays.com/countries/egypt/2027", "https://www.timeanddate.com/holidays/egypt/2028"]

[[holiday]]
country = "EG"
id = "sinai_liberation_day"
name = "عيد تحرير سيناء (Sinai Liberation Day)"
rule = "fixed(4,25)"
dates_2026 = ["2026-04-25"]
dates_2027 = ["2027-04-25"]
dates_2028 = ["2028-04-25"]
tentative_2027 = true
tentative_2028 = true
observance = false
eve = false
greeting = "عيد تحرير سيناء"
greeting_en = "Sinai Liberation Day"
look = "flag"
source = ["https://manshurat.org/node/44922", "https://egyptianstreets.com/2026/04/19/egypt-declares-25-april-public-holiday-for-sinai-liberation-day/", "https://www.officeholidays.com/countries/egypt/2027", "https://www.timeanddate.com/holidays/egypt/2028"]

[[holiday]]
country = "EG"
id = "labour_day"
name = "عيد العمال (Labour Day)"
rule = "fixed(5,1)"
dates_2026 = ["2026-05-01"]
dates_2027 = ["2027-05-01"]
dates_2028 = ["2028-05-01"]
tentative_2027 = true
tentative_2028 = true
observance = false
eve = false
greeting = "عيد عمال سعيد"
greeting_en = "Happy Labour Day"
look = "none"
source = ["https://manshurat.org/node/44922", "https://www.cairo.gov.eg/en/news/news-eng/2026/fda45a85d7f446c1b4cd55e0f8259f4e", "https://www.officeholidays.com/countries/egypt/2027", "https://www.timeanddate.com/holidays/egypt/2028"]

[[holiday]]
country = "EG"
id = "arafat_day"
name = "وقفة عيد الأضحى / يوم عرفة (Day of Arafah)"
rule = "listed"
dates_2026 = ["2026-05-26"]
dates_2027 = ["2027-05-15"]
dates_2028 = ["2028-05-04"]
tentative_2027 = true
tentative_2028 = true
observance = false
eve = true
greeting = "يوم عرفة"
greeting_en = "Day of Arafah"
look = "none"
source = ["https://manshurat.org/node/44922", "https://www.cairo.gov.eg/en/news/2026/may/19-2026/d1c50de4c7d743e9b9bcd7f5971a4e6b", "https://egyptianstreets.com/2026/05/18/egypt-confirms-eid-al-adha-dates-after-moon-sighting/", "https://banker.news/155540", "https://www.officeholidays.com/countries/egypt/2027", "https://www.timeanddate.com/holidays/egypt/2028"]

[[holiday]]
country = "EG"
id = "eid_al_adha"
name = "عيد الأضحى المبارك (Eid al-Adha)"
rule = "listed"
dates_2026 = ["2026-05-27", "2026-05-28", "2026-05-29", "2026-05-30"]
dates_2027 = ["2027-05-16", "2027-05-17", "2027-05-18", "2027-05-19"]
dates_2028 = ["2028-05-05", "2028-05-06", "2028-05-07", "2028-05-08"]
tentative_2027 = true
tentative_2028 = true
observance = false
eve = true
greeting = "عيد أضحى مبارك"
greeting_en = "Happy Eid al-Adha {era} AH"
look = "eid"
source = ["https://manshurat.org/node/44922", "https://www.cairo.gov.eg/en/news/2026/may/19-2026/d1c50de4c7d743e9b9bcd7f5971a4e6b", "https://egyptianstreets.com/2026/05/18/egypt-confirms-eid-al-adha-dates-after-moon-sighting/", "https://banker.news/155540", "https://www.officeholidays.com/countries/egypt/2027", "https://www.timeanddate.com/holidays/egypt/2028", "https://api.aladhan.com/v1/gToH/05-05-2028?calendarMethod=UAQ"]

[[holiday]]
country = "EG"
id = "islamic_new_year"
name = "رأس السنة الهجرية (Islamic New Year)"
rule = "listed"
dates_2026 = ["2026-06-16"]
dates_2027 = ["2027-06-06"]
dates_2028 = ["2028-05-25"]
tentative_2027 = true
tentative_2028 = true
observance = false
eve = true
greeting = "سنة هجرية سعيدة"
greeting_en = "Happy Hijri New Year {era}"
look = "none"
source = ["https://manshurat.org/node/44922", "https://www.cairo.gov.eg/en/news/2026/june/9_june/de4fb420b9864472ae90cbef4d019f05", "https://www.officeholidays.com/countries/egypt/2027", "https://www.timeanddate.com/holidays/egypt/2028", "https://api.aladhan.com/v1/gToH/26-05-2028?calendarMethod=UAQ"]

[[holiday]]
country = "EG"
id = "june30_revolution"
name = "عيد ثورة 30 يونيو (30 June Revolution Day)"
rule = "fixed(6,30)"
dates_2026 = ["2026-06-30"]
dates_2027 = ["2027-06-30"]
dates_2028 = ["2028-06-30"]
tentative_2027 = true
tentative_2028 = true
observance = false
eve = false
greeting = "ذكرى ثورة 30 يونيو"
greeting_en = "June 30 Revolution Day"
look = "flag"
source = ["https://manshurat.org/node/44922", "https://www.cairo.gov.eg/en/news/2026/june/24/30_june/", "https://www.officeholidays.com/countries/egypt/2027", "https://www.timeanddate.com/holidays/egypt/2028"]

[[holiday]]
country = "EG"
id = "national_day"
name = "عيد ثورة 23 يوليو (23 July Revolution Day, the National Day)"
rule = "fixed(7,23)"
dates_2026 = ["2026-07-23"]
dates_2027 = ["2027-07-23"]
dates_2028 = ["2028-07-23"]
tentative_2027 = true
tentative_2028 = true
observance = false
eve = false
greeting = "عيد ثورة 23 يوليو"
greeting_en = "Revolution Day · {nth since 1952} anniversary"
look = "flag"
source = ["https://manshurat.org/node/44922", "https://english.ahram.org.eg/News/572817.aspx", "https://www.officeholidays.com/countries/egypt/2027", "https://www.timeanddate.com/holidays/egypt/2028"]

[[holiday]]
country = "EG"
id = "mawlid"
name = "المولد النبوي الشريف (Mawlid an-Nabi, the Prophet's Birthday)"
rule = "listed"
dates_2026 = ["2026-08-25"]
dates_2027 = ["2027-08-15"]
dates_2028 = ["2028-08-03"]
tentative_2027 = true
tentative_2028 = true
observance = false
eve = true
greeting = "مولد نبوي شريف"
greeting_en = "Mawlid an-Nabi"
look = "none"
source = ["https://manshurat.org/node/44922", "https://cairo.gov.eg/en/news/2026/august/18/de610148a17d4867b142201cdca78d7e", "https://www.officeholidays.com/countries/egypt/2027", "https://www.timeanddate.com/holidays/egypt/2028", "https://api.aladhan.com/v1/gToH/03-08-2028?calendarMethod=UAQ"]

[[holiday]]
country = "EG"
id = "armed_forces_day"
name = "عيد القوات المسلحة (Armed Forces Day, 6 October Victory)"
rule = "fixed(10,6)"
dates_2026 = ["2026-10-06"]
dates_2027 = ["2027-10-06"]
dates_2028 = ["2028-10-06"]
tentative_2027 = true
tentative_2028 = true
observance = false
eve = false
greeting = "عيد القوات المسلحة"
greeting_en = "Armed Forces Day"
look = "flag"
source = ["https://manshurat.org/node/44922", "https://arabfinance.com/en/news/newdetails/madbouly-declares-october-8-public-holiday-for-armed-forces-day", "https://www.officeholidays.com/countries/egypt/2027", "https://www.timeanddate.com/holidays/egypt/2028"]

[[holiday]]
country = "EG"
id = "ramadan"
name = "رمضان (Ramadan, fasting month; not a day off)"
rule = "listed"
dates_2026 = ["2026-02-19", "2026-02-20", "2026-02-21", "2026-02-22", "2026-02-23", "2026-02-24", "2026-02-25", "2026-02-26", "2026-02-27", "2026-02-28", "2026-03-01", "2026-03-02", "2026-03-03", "2026-03-04", "2026-03-05", "2026-03-06", "2026-03-07", "2026-03-08", "2026-03-09", "2026-03-10", "2026-03-11", "2026-03-12", "2026-03-13", "2026-03-14", "2026-03-15", "2026-03-16", "2026-03-17", "2026-03-18", "2026-03-19"]
dates_2027 = ["2027-02-08", "2027-02-09", "2027-02-10", "2027-02-11", "2027-02-12", "2027-02-13", "2027-02-14", "2027-02-15", "2027-02-16", "2027-02-17", "2027-02-18", "2027-02-19", "2027-02-20", "2027-02-21", "2027-02-22", "2027-02-23", "2027-02-24", "2027-02-25", "2027-02-26", "2027-02-27", "2027-02-28", "2027-03-01", "2027-03-02", "2027-03-03", "2027-03-04", "2027-03-05", "2027-03-06", "2027-03-07", "2027-03-08"]
dates_2028 = ["2028-01-28", "2028-01-29", "2028-01-30", "2028-01-31", "2028-02-01", "2028-02-02", "2028-02-03", "2028-02-04", "2028-02-05", "2028-02-06", "2028-02-07", "2028-02-08", "2028-02-09", "2028-02-10", "2028-02-11", "2028-02-12", "2028-02-13", "2028-02-14", "2028-02-15", "2028-02-16", "2028-02-17", "2028-02-18", "2028-02-19", "2028-02-20", "2028-02-21", "2028-02-22", "2028-02-23", "2028-02-24", "2028-02-25", "2028-02-26"]
tentative_2027 = true
tentative_2028 = true
observance = true
eve = true
greeting = "رمضان كريم"
greeting_en = "Ramadan Kareem"
look = "ramadan"
source = ["https://www.dar-alifta.org/ar/Articles/Details/11039/", "https://dar-alifta.org/ar/articles/details/11079", "https://banker.news/155540", "https://www.timeanddate.com/holidays/egypt/2028"]

[[holiday]]
country = "EG"
id = "mothers_day"
name = "عيد الأم (Mother's Day; not a day off)"
rule = "fixed(3,21)"
dates_2026 = ["2026-03-21"]
dates_2027 = ["2027-03-21"]
dates_2028 = ["2028-03-21"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "كل سنة وانتي طيبة يا أمي"
greeting_en = "Happy Mother's Day"
look = "none"
source = ["https://aucegypt.edu/news/stories/history-mothers-day-ancient-egypt-modern-times", "https://egyptianstreets.com/2025/03/21/from-newsprint-to-national-celebration-how-egyptian-media-pioneered-mothers-day/", "https://www.officeholidays.com/holidays/egypt/mothers-day-middle-east"]

[[holiday]]
country = "EG"
id = "coptic_easter"
name = "عيد القيامة المجيد (Coptic Easter; day off for Christians only)"
rule = "orthodox_easter(0)"
dates_2026 = ["2026-04-12"]
dates_2027 = ["2027-05-02"]
dates_2028 = ["2028-04-16"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = true
greeting = "عيد قيامة مجيد"
greeting_en = "Happy Easter"
look = "none"
source = ["https://manshurat.org/node/44922", "https://www.officeholidays.com/countries/egypt/2027", "https://www.timeanddate.com/holidays/egypt/2028"]
```

**Notes (EG)**

- **Legal basis:** Prime Minister's Decree **1193/2018** (Official Gazette no. 26 bis, 2 Jul 2018;
  [manshurat.org](https://manshurat.org/node/44922), PDF read) lists the 12 occasions on which
  ministries, government bodies and local units close: Hijri New Year; Mawlid; **the eve (waqfa)
  of Eid al-Fitr and the three Eid days**; **the waqfa (Arafat) and the four days of Eid
  al-Adha**; Christmas 7 Jan; 25 Jan Revolution and Police Day; Sham el-Nessim; Sinai Liberation
  25 Apr; Labour Day 1 May; 30 June Revolution; 23 July Revolution; Armed Forces Day 6 Oct.
  Decree **1249/2020** ([manshurat.org](https://manshurat.org/node/66981), PDF read) adds: "the
  Prime Minister may, when he sees fit, substitute another day at the start or end of the working
  week for any of these holidays". That is the legal hook for the Thursday moves.
  - Private sector: Labour Minister's Decree **294/2025** under the new Labour Law 14/2025
    (art. 129) has the same 12 occasions but shorter Eids: **Eid al-Fitr = 1–2 Shawwal**, **Eid
    al-Adha = Arafat + Eid days 1–2** (as reported by
    [hapijournal](https://hapijournal.com/2025/12/26/وزير-العمل-يصدر-قرارا-بتحديد-الإجازا/),
    26 Dec 2025; the gazette text was not found online). Each year's PM decree is usually
    extended to the private sector by a Labour Ministry statement.
  - `dates_*` hold the religious/commemorative day itself: Eid al-Fitr = 1–3 Shawwal, Eid al-Adha
    = 10–13 Dhu al-Hijjah (the government's four days), Arafat as its own entry. The waqfa of Eid
    al-Fitr (last day of Ramadan) is also a government day off; it is not listed separately
    because the Ramadan block covers it and `eve = true` on Eid shows the evening.
- **2026 decrees (all verified, Cairo Governorate portal and Egyptian press):**
  - Coptic Christmas: Wed **7 Jan** (PM Decree 33/2026), not moved.
  - 25 Jan (Sun) → day off **Thu 29 Jan**.
  - Eid al-Fitr: Dar al-Ifta fixed 1 Shawwal 1447 = **Fri 20 Mar** (statement of 19 Mar). Day-off
    block **Thu 19 – Mon 23 Mar**; banks 19–24 Mar.
  - Sham el-Nessim: Mon **13 Apr** (PM Decree 986/2026), public and private sector.
  - Sinai Liberation: **Sat 25 Apr**, kept on the day.
  - Labour Day (Fri 1 May) → extra day off **Thu 7 May**.
  - Eid al-Adha: Dar al-Ifta: 1 Dhu al-Hijjah = 18 May, Arafat **Tue 26 May**, Eid **Wed 27 May**.
    Day-off block **Tue 26 – Sun 31 May**.
  - Hijri New Year 1448: 1 Muharram = Tue 16 Jun (29 Dhu al-Hijjah was 15 Jun; conjunction 15 Jun
    02:54 UTC, crescent sets 47 min after the sun in Cairo, my calc). Day off moved to **Thu 18 Jun**
    (PM Decree 1865/2026).
  - 30 June (Tue) → **Thu 2 Jul**.
  - 23 July: **Thu 23 Jul**, kept.
  - Mawlid: 12 Rabi' al-Awwal 1448 = Tue 25 Aug; day off moved to **Thu 27 Aug**.
  - Armed Forces Day (Tue 6 Oct) → **Thu 8 Oct**.
  - Pattern: a fixed national day on Sun–Wed is moved to the following Thursday; on a Friday it
    may get an extra Thursday (1 May 2026); Saturday and Thursday days stay. Christmas, Sham
    el-Nessim and the two Eids are never moved. Each move is a PM decree a few days to a week ahead.
- **Why the fixed national days are `tentative_2027/2028 = true`:** the commemoration date never
  changes (the greeting is right on the day itself), but the **day off** is decided by decree each
  year and is usually moved to a Thursday. Likely 2027 moves: 25 Jan (Mon) → Thu 28 Jan; 30 Jun
  (Wed) → Thu 1 Jul; 6 Oct (Wed) → Thu 7 Oct (timeanddate already shows all three); Sinai (Sun
  25 Apr) and 1 May (Sat) are open questions. Likely 2028 moves: 25 Jan (Tue) → Thu 27 Jan; Sinai
  (Tue 25 Apr) → Thu 27 Apr; 23 Jul (Sun) → Thu 27 Jul (timeanddate shows 27 Jan and 27 Jul).
- **Islamic dates:** Egypt starts each month from Dar al-Ifta's sighting, run by Sharia and
  scientific committees and checked against the National Research Institute of Astronomy and
  Geophysics (NRIAG). Dar al-Ifta issues a sighting statement for every month, and the eye
  sighting decides, not the calculation. Egypt's record, with lags from my own Meeus calculation
  for Cairo: it **accepted** a ~14 min lag (10 Mar 2024, Ramadan 1445 began Mon 11 Mar 2024) and
  rejected an **11 min** lag even though NRIAG had predicted it visible (29 Mar 2025; Eid al-Fitr
  was **Mon 31 Mar 2025**, a day after Saudi Arabia, per
  [Egyptian Streets](https://egyptianstreets.com/2025/03/29/egypt-announces-monday-as-the-first-day-of-eid-al-fitr/)).
  It also rejected a ~3 min lag (17 Feb 2026, so Ramadan 1447 began **19 Feb**, again a day
  after Saudi Arabia). So the working threshold is a lag somewhere between 11 and 14 minutes.
  - **2027 (NRIAG, via [banker.news](https://banker.news/155540), 8 Oct 2026):** Ramadan 29 days;
    1 Shawwal = **Tue 9 Mar** (conjunction 8 Mar 11:31 Cairo); 1 Dhu al-Hijjah = Fri 7 May
    (conjunction 6 May 14:00 Cairo), Arafat **Sat 15 May**, Eid **Sun 16 May**. Kept as NRIAG
    gives them. My calc: 17 min lag on 6 May (safe), but only **12 min on 8 Mar**, right at the
    2025 rejection. **Eid al-Fitr on Wed 10 Mar 2027 is a real possibility** (then Ramadan runs
    to 9 Mar). Ramadan 1448 begins **Mon 8 Feb** whatever happens (conjunction 6 Feb 15:56 UTC,
    after Cairo sunset). Hijri New Year **Sun 6 Jun** either way (conjunction 4 Jun 19:40 UTC,
    after sunset, so Dhu al-Hijjah has 30 days). Mawlid **Sun 15 Aug**: the 2 Aug crescent has
    only an 8 min lag, below what Egypt accepts, so Rabi' al-Awwal should start 4 Aug.
    Umm al-Qura (AlAdhan) and officeholidays.com have 14 Aug; timeanddate has 15 Aug.
  - **2028 (computed; no NRIAG figure yet):** Ramadan **Fri 28 Jan** (the 26 Jan conjunction is
    11 min before sunset and the moon sets first). Eid al-Fitr **Sun 27 Feb**: the 25 Feb crescent
    has a 9 min lag, so Ramadan should run 30 days (block 28 Jan – 26 Feb). Umm al-Qura has
    26 Feb. Arafat **Thu 4 May**, Eid al-Adha **Fri 5 May** (25 Apr crescent: 52 min lag);
    Hijri New Year **Thu 25 May** (24 May: 24 min lag); Mawlid **Thu 3 Aug** (22 Jul: 18 min
    lag, Rabi' al-Awwal from 23 Jul). These three match Umm al-Qura.
  - **Disagreement:** timeanddate (Wayback snapshots of 8 Feb and 5 Mar 2026) is a day later than
    the dates kept here for 2027 Eid al-Fitr (10 Mar), Arafat/Eid al-Adha 2027 (16/17 May) and
    2028 (5/6 May), Muharram 2027 (7 Jun) and 2028 (26 May) and Mawlid 2028 (4 Aug). It agrees
    on 2027 Mawlid (15 Aug) and 2028 Eid al-Fitr (27 Feb). For the big crescents (lags of 17–52
    min) its rule is stricter than anything Egypt applies, so those rows were not followed.
  - officeholidays.com 2026 has Arafat 27 May and Eid al-Adha 28 May; that is wrong. Dar al-Ifta
    and the PM's decree give Arafat 26 May and Eid 27 May. Its 2026 Armed Forces Day row (Fri
    9 Oct) is also wrong; the day off was Thu 8 Oct.
- **Coptic Easter** is not on the national list. Christians get their religious feasts off under a
  separate Cabinet decision (Decree 294/2025 art. 3; Labour Law 14/2025 art. 129). officeholidays.com
  lists it as a national holiday; timeanddate lists it as an observance, which matches the decrees.
  Coptic Good Friday and Palm Sunday are left out.
- **Sham el-Nessim** is the Monday after Coptic (Orthodox) Easter, hence `orthodox_easter(1)`.
  Families picnic and eat feseekh (salted fish), green onions and coloured eggs, so the look is
  `easter_eggs`.
- **Mother's Day (21 Mar)** has been marked in Egypt since 1956, after Mustafa and Ali Amin's
  newspaper campaign. It is a school and family occasion, not a day off. In 2026 it fell on Eid
  al-Fitr day 2.
- **Ramadan block:** Egypt's fanous lanterns and street decorations run all month. "رمضان كريم"
  (Ramadan Kareem) is the usual Egyptian greeting; the reply is "الله أكرم". The 2027 block is
  29 days (8 Feb – 8 Mar) and the 2028 block 30 days (28 Jan – 26 Feb), each ending the day
  before the Eid kept above.
- **Other days, not added:** Taba Liberation (19 Mar) and Evacuation Day (18 Jun) close schools
  and universities only (Decree 1193/2018 art. 2); Suez National Day (24 Oct), Port Said Victory
  Day (23 Dec) and Wafaa el-Nil (15 Aug) are local celebrations. Coptic New Year (Nayrouz, 11/12
  Sep) is a church feast. Laylat al-Qadr and Isra' wal-Mi'raj are marked religiously but are not
  days off.
- **Looks:** Coptic Christmas uses `candles` like Ethiopia's Genna. The national days use `flag`.
  Mawlid is `none`: Egypt's sugar dolls and horses (aroset el-moulid) have no look yet. Arafat
  and Hijri New Year are `none` on purpose.

### Country notes (EG)

- **Christmas / New Year:** **25 Dec is not a holiday**; Copts (about 10% of Egyptians) celebrate
  on 7 Jan, which is a national day off. Malls and hotels put up Christmas trees from mid-December
  for the combined Christmas/New Year season, so the default lights are not out of place, but
  ending them at 7 Jan would fit Egypt better. **1 Jan is not a public holiday** (banks close; it
  is a "bank holiday" on officeholidays.com), but New Year's Eve parties and countdowns are
  common in Cairo and Alexandria: keep the countdown.
  **Built (25 Oct 2026):** EG (and RU, for Orthodox Christmas and the New Year holidays) use
  `lights_to_january_7`, lights from 15 Dec to the end of 7 Jan; TR uses `new_year_lights`, 15 Dec
  to the morning of 1 Jan.
- **Half-mast:** **none fixed.** Law 41/2014 on the flag, anthem and national salute has no annual
  half-mast day; flags are lowered only when the President declares national mourning (for
  example three days for Hosni Mubarak in Feb 2020). The text of Law 41/2014 could not be
  downloaded here (manshurat.org timed out twice), so the absence of a fixed day rests on the
  decrees above and press practice.
- **Flag:** 2:3, three equal horizontal bands — **red (#CE1126) on top, white in the middle, black
  at the bottom** — with the **golden Eagle of Saladin (#C09300)** centred in the white band
  (Law 41/2014 art. 2). For 9×3: row 1 red, row 2 white with a gold glyph in the middle cell
  (col 5), row 3 black.
- **Geocoder language:** `ar`.
- **Anchor cities** (Cairo is 30.04, 31.24):
  - Alexandria 31.20, 29.92 (north coast)
  - El Salloum 31.55, 25.16 (Libyan border)
  - Siwa 29.20, 25.52 (Western Desert, near Libya)
  - Abu Simbel 22.34, 31.63 (just north of the Sudan border)
  - Aswan 24.09, 32.90 (south)
  - El Arish 31.13, 33.80 (north Sinai, near Gaza)
  - Taba 29.49, 34.89 (Israel/Jordan corner)
  - Hurghada 27.26, 33.81 (Red Sea coast)

---

## Turkey (TR)

```toml
[[holiday]]
country = "TR"
id = "new_year"
name = "Yılbaşı (New Year's Day)"
rule = "fixed(1,1)"
dates_2026 = ["2026-01-01"]
dates_2027 = ["2027-01-01"]
dates_2028 = ["2028-01-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = true
greeting = "Mutlu Yıllar"
greeting_en = "Happy New Year"
look = "fireworks_night"
source = ["https://www.mevzuat.gov.tr/mevzuat?MevzuatNo=2429&MevzuatTur=1&MevzuatTertip=5", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=158", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=159", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=193"]

[[holiday]]
country = "TR"
id = "eid_al_fitr"
name = "Ramazan Bayramı (Eid al-Fitr)"
rule = "listed"
dates_2026 = ["2026-03-20", "2026-03-21", "2026-03-22"]
dates_2027 = ["2027-03-09", "2027-03-10", "2027-03-11"]
dates_2028 = ["2028-02-26", "2028-02-27", "2028-02-28"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = true
greeting = "İyi Bayramlar"
greeting_en = "Happy Eid al-Fitr {era} AH"
look = "eid"
source = ["https://www.mevzuat.gov.tr/mevzuat?MevzuatNo=2429&MevzuatTur=1&MevzuatTertip=5", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=158", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=159", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=193", "https://vakithesaplama.diyanet.gov.tr/dinigunler.php?yil=2026", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=154", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=185", "https://www.officeholidays.com/countries/turkey/2027", "https://www.timeanddate.com/holidays/turkey/2028"]

[[holiday]]
country = "TR"
id = "national_sovereignty_childrens_day"
name = "Ulusal Egemenlik ve Çocuk Bayramı (National Sovereignty and Children's Day)"
rule = "fixed(4,23)"
dates_2026 = ["2026-04-23"]
dates_2027 = ["2027-04-23"]
dates_2028 = ["2028-04-23"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "23 Nisan Kutlu Olsun"
greeting_en = "Sovereignty & Children's Day"
look = "flag"
source = ["https://www.mevzuat.gov.tr/mevzuat?MevzuatNo=2429&MevzuatTur=1&MevzuatTertip=5", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=158", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=159", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=193"]

[[holiday]]
country = "TR"
id = "labour_day"
name = "Emek ve Dayanışma Günü (Labour and Solidarity Day)"
rule = "fixed(5,1)"
dates_2026 = ["2026-05-01"]
dates_2027 = ["2027-05-01"]
dates_2028 = ["2028-05-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "1 Mayıs Kutlu Olsun"
greeting_en = "Labour and Solidarity Day"
look = "none"
source = ["https://www.mevzuat.gov.tr/mevzuat?MevzuatNo=2429&MevzuatTur=1&MevzuatTertip=5", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=158", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=159", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=193"]

[[holiday]]
country = "TR"
id = "youth_and_sports_day"
name = "Atatürk'ü Anma, Gençlik ve Spor Bayramı (Commemoration of Atatürk, Youth and Sports Day)"
rule = "fixed(5,19)"
dates_2026 = ["2026-05-19"]
dates_2027 = ["2027-05-19"]
dates_2028 = ["2028-05-19"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "19 Mayıs Kutlu Olsun"
greeting_en = "Youth and Sports Day"
look = "flag"
source = ["https://www.mevzuat.gov.tr/mevzuat?MevzuatNo=2429&MevzuatTur=1&MevzuatTertip=5", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=158", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=159", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=193"]

[[holiday]]
country = "TR"
id = "eid_al_adha"
name = "Kurban Bayramı (Eid al-Adha)"
rule = "listed"
dates_2026 = ["2026-05-27", "2026-05-28", "2026-05-29", "2026-05-30"]
dates_2027 = ["2027-05-16", "2027-05-17", "2027-05-18", "2027-05-19"]
dates_2028 = ["2028-05-05", "2028-05-06", "2028-05-07", "2028-05-08"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = true
greeting = "İyi Bayramlar"
greeting_en = "Happy Eid al-Adha {era} AH"
look = "eid"
source = ["https://www.mevzuat.gov.tr/mevzuat?MevzuatNo=2429&MevzuatTur=1&MevzuatTertip=5", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=158", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=159", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=193", "https://vakithesaplama.diyanet.gov.tr/dinigunler.php?yil=2026", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=154", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=185", "https://www.officeholidays.com/countries/turkey/2027", "https://www.timeanddate.com/holidays/turkey/2028"]

[[holiday]]
country = "TR"
id = "democracy_and_national_unity_day"
name = "Demokrasi ve Millî Birlik Günü (Democracy and National Unity Day)"
rule = "fixed(7,15)"
dates_2026 = ["2026-07-15"]
dates_2027 = ["2027-07-15"]
dates_2028 = ["2028-07-15"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "15 Temmuz Demokrasi ve Millî Birlik Günü"
greeting_en = "Democracy & Unity Day"
look = "flag"
source = ["https://www.mevzuat.gov.tr/mevzuat?MevzuatNo=2429&MevzuatTur=1&MevzuatTertip=5", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=158", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=159", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=193"]

[[holiday]]
country = "TR"
id = "victory_day"
name = "Zafer Bayramı (Victory Day)"
rule = "fixed(8,30)"
dates_2026 = ["2026-08-30"]
dates_2027 = ["2027-08-30"]
dates_2028 = ["2028-08-30"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Zafer Bayramımız Kutlu Olsun"
greeting_en = "Happy {nth since 1922} Victory Day"
look = "flag"
source = ["https://www.mevzuat.gov.tr/mevzuat?MevzuatNo=2429&MevzuatTur=1&MevzuatTertip=5", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=158", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=159", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=193"]

[[holiday]]
country = "TR"
id = "republic_day"
name = "Cumhuriyet Bayramı (Republic Day)"
rule = "fixed(10,29)"
dates_2026 = ["2026-10-29"]
dates_2027 = ["2027-10-29"]
dates_2028 = ["2028-10-29"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = true
greeting = "Cumhuriyet Bayramımız Kutlu Olsun"
greeting_en = "Happy {nth since 1923} Republic Day, Türkiye"
look = "flag+fireworks_night"
source = ["https://www.mevzuat.gov.tr/mevzuat?MevzuatNo=2429&MevzuatTur=1&MevzuatTertip=5", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=158", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=159", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=193"]

[[holiday]]
country = "TR"
id = "ramadan"
name = "Ramazan (Ramadan, fasting month; not a day off)"
rule = "listed"
dates_2026 = ["2026-02-19", "2026-02-20", "2026-02-21", "2026-02-22", "2026-02-23", "2026-02-24", "2026-02-25", "2026-02-26", "2026-02-27", "2026-02-28", "2026-03-01", "2026-03-02", "2026-03-03", "2026-03-04", "2026-03-05", "2026-03-06", "2026-03-07", "2026-03-08", "2026-03-09", "2026-03-10", "2026-03-11", "2026-03-12", "2026-03-13", "2026-03-14", "2026-03-15", "2026-03-16", "2026-03-17", "2026-03-18", "2026-03-19"]
dates_2027 = ["2027-02-08", "2027-02-09", "2027-02-10", "2027-02-11", "2027-02-12", "2027-02-13", "2027-02-14", "2027-02-15", "2027-02-16", "2027-02-17", "2027-02-18", "2027-02-19", "2027-02-20", "2027-02-21", "2027-02-22", "2027-02-23", "2027-02-24", "2027-02-25", "2027-02-26", "2027-02-27", "2027-02-28", "2027-03-01", "2027-03-02", "2027-03-03", "2027-03-04", "2027-03-05", "2027-03-06", "2027-03-07", "2027-03-08"]
dates_2028 = ["2028-01-28", "2028-01-29", "2028-01-30", "2028-01-31", "2028-02-01", "2028-02-02", "2028-02-03", "2028-02-04", "2028-02-05", "2028-02-06", "2028-02-07", "2028-02-08", "2028-02-09", "2028-02-10", "2028-02-11", "2028-02-12", "2028-02-13", "2028-02-14", "2028-02-15", "2028-02-16", "2028-02-17", "2028-02-18", "2028-02-19", "2028-02-20", "2028-02-21", "2028-02-22", "2028-02-23", "2028-02-24", "2028-02-25"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = true
greeting = "Hayırlı Ramazanlar"
greeting_en = "Ramadan Mubarak"
look = "ramadan"
source = ["https://vakithesaplama.diyanet.gov.tr/dinigunler.php?yil=2026", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=154", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=185", "https://www.timeanddate.com/holidays/turkey/2028"]

[[holiday]]
country = "TR"
id = "canakkale_martyrs_day"
name = "18 Mart Şehitleri Anma Günü ve Çanakkale Zaferi (Martyrs' Day and Çanakkale Victory; not a day off)"
rule = "fixed(3,18)"
dates_2026 = ["2026-03-18"]
dates_2027 = ["2027-03-18"]
dates_2028 = ["2028-03-18"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Şehitlerimizi Rahmetle Anıyoruz"
greeting_en = "Çanakkale Martyrs' Day"
look = "flag"
source = ["https://aile.gov.tr/denizli/haberler/18-mart-sehitleri-anma-gunu-ve-canakkale-zaferi-nin-111-yil-donumu-torenlerle-anildi/", "https://www.timeanddate.com/holidays/turkey/2027"]

[[holiday]]
country = "TR"
id = "ashura"
name = "Aşure Günü (Ashura, 10 Muharram; not a day off)"
rule = "listed"
dates_2026 = ["2026-06-25"]
dates_2027 = ["2027-06-15"]
dates_2028 = ["2028-06-03"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = true
greeting = "Aşure Gününüz Mübarek Olsun"
greeting_en = "Ashura (Aşure Day)"
look = "none"
source = ["https://vakithesaplama.diyanet.gov.tr/dinigunler.php?yil=2026", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=154", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=185"]

[[holiday]]
country = "TR"
id = "mawlid"
name = "Mevlid Kandili (Mawlid an-Nabi night; not a day off)"
rule = "listed"
dates_2026 = ["2026-08-25"]
dates_2027 = ["2027-08-14"]
dates_2028 = ["2028-08-03"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = true
greeting = "Kandiliniz Mübarek Olsun"
greeting_en = "Blessed Mawlid night"
look = "none"
source = ["https://vakithesaplama.diyanet.gov.tr/dinigunler.php?yil=2026", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=154", "https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=185"]

[[holiday]]
country = "TR"
id = "ataturk_memorial_day"
name = "10 Kasım Atatürk'ü Anma Günü (Atatürk Memorial Day; flag at half-mast, not a day off)"
rule = "fixed(11,10)"
dates_2026 = ["2026-11-10"]
dates_2027 = ["2027-11-10"]
dates_2028 = ["2028-11-10"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Saygıyla Anıyoruz"
greeting_en = "Atatürk Memorial · half-mast"
look = "half_mast"
source = ["https://www.mevzuat.gov.tr/mevzuat?MevzuatNo=859034&MevzuatTur=2&MevzuatTertip=5", "https://www.timeanddate.com/holidays/turkey/2028"]

[[holiday]]
country = "TR"
id = "teachers_day"
name = "Öğretmenler Günü (Teachers' Day; not a day off)"
rule = "fixed(11,24)"
dates_2026 = ["2026-11-24"]
dates_2027 = ["2027-11-24"]
dates_2028 = ["2028-11-24"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Öğretmenler Günü Kutlu Olsun"
greeting_en = "Happy Teachers' Day"
look = "none"
source = ["https://www.cnnturk.com/egitim/2025-ogretmenler-gunu-ne-zaman-bu-yil-ogretmenler-gunu-hangi-gune-denk-geliyor-iste-tarihcesi-2355464", "https://www.timeanddate.com/holidays/turkey/2028"]
```

**Notes (TR)**

- **Legal basis:** Law **2429** on National Holidays and General Holidays (1981, amended by Law
  6752 of 2016; [mevzuat.gov.tr](https://www.mevzuat.gov.tr/mevzuat?MevzuatNo=2429&MevzuatTur=1&MevzuatTertip=5),
  read). Art. 1: 29 October is the National Day; it **starts at 13:00 on 28 October**. Art. 2:
  23 April, 19 May, 30 August; **Ramazan Bayramı = 3.5 days from 13:00 on arife**; **Kurban
  Bayramı = 4.5 days from 13:00 on arife**; 1 January, 1 May and 15 July. There are no
  substitute days: a holiday on a weekend is simply lost, and two holidays on the same day
  (Kurban day 4 and 19 May 2027) do not add a day.
- **All dates are official for 2026, 2027 and 2028:** Diyanet's "Resmi Tatil Günleri" tables for
  [2026](https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=158),
  [2027](https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=159) and
  [2028](https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=193), and its "Dini Günler"
  lists for [2026](https://vakithesaplama.diyanet.gov.tr/dinigunler.php?yil=2026),
  [2027](https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=154) and
  [2028](https://vakithesaplama.diyanet.gov.tr/icerik.php?icerik=185) (all read 23 Oct 2026; the
  site publishes tables through 2035).
- **Islamic days are not tentative.** Turkey does not sight the moon: Diyanet uses the calculated
  calendar adopted at the 2016 Istanbul International Hijri Calendar Unity Congress and publishes
  the dates years ahead. Hence `tentative_2027 = tentative_2028 = false` for both bayrams, the
  Ramadan block, Aşure and Mevlid Kandili. Diyanet's dates:
  - Ramadan: 1 Ramazan = 19 Feb 2026, 8 Feb 2027, 28 Jan 2028. Arife (29 Ramazan) = 19 Mar 2026,
    8 Mar 2027, 25 Feb 2028, so each block is 29 days and ends on arife. Kadir Gecesi (Night of
    Power): 16 Mar 2026, 5 Mar 2027, 22/23 Feb 2028.
  - Ramazan Bayramı: 20–22 Mar 2026, 9–11 Mar 2027, 26–28 Feb 2028.
  - Kurban Bayramı: arife 26 May 2026 / 15 May 2027 / 4 May 2028; bayram 27–30 May 2026,
    16–19 May 2027, 5–8 May 2028.
  - Aşure (10 Muharrem): 25 Jun 2026, 15 Jun 2027, 3 Jun 2028.
  - Mevlid Kandili is the **night before 12 Rebiülevvel**: Diyanet lists it on 11 Rebiülevvel,
    Mon 24 Aug 2026, Fri 13 Aug 2027, and "2/3 Aug" 2028. The entry holds the 12 Rebiülevvel
    date (25 Aug 2026, 14 Aug 2027, 3 Aug 2028) with `eve = true`, so the kandil evening shows
    and the `mawlid` id keeps the same Hijri day as other countries.
- **Arife (eve) half-days:** `eve = true` on Republic Day and both bayrams matches the law's
  13:00 start. The half-day arife is a public holiday afternoon, not a separate entry.
- **Administrative leave (idari izin):** the Presidency often bridges bayrams for civil servants
  by circular. For Kurban Bayramı 2026, 25 May (full day) and the morning of 26 May were given,
  making a 9-day break from 23 to 31 May (as reported by
  [Milliyet](https://www.milliyet.com.tr/haberleri/kurban-bayrami-ne-zaman-basliyor) and others;
  the circular itself was not read). officeholidays.com shows Mon 23 Mar 2026 as a
  "Government Holiday" after Ramazan Bayramı. These are not public holidays, so they stay in
  Notes. No 2027 or 2028 circulars exist yet.
- **Cross-checks and disagreements:**
  - officeholidays.com 2026 and 2027 agree on every holiday date. Its 2027 page also lists
    **"Nowruz, 22 Mar" as a national holiday; that is wrong** (Nowruz is not in Law 2429).
  - timeanddate.com 2028 (Wayback, 16 Apr 2026) gives **Ramazan Bayramı 27–29 Feb** with arife
    26 Feb. Diyanet, which sets the date, gives **26–28 Feb** (arife 25 Feb). Diyanet is used.
    timeanddate agrees with Diyanet on Kurban Bayramı 2028 (5–8 May) and on Ramadan's start (28 Jan).
- **Half-mast:** the Turkish Flag Regulation (Türk Bayrağı Tüzüğü, 85/9034) art. 24: "Bayrak,
  10 Kasımda Türkiye'de ve Türkiye Cumhuriyetinin dış temsilciliklerinde, resmi ve millî
  binalarında yarıya çekilir" ([mevzuat.gov.tr](https://www.mevzuat.gov.tr/mevzuat?MevzuatNo=859034&MevzuatTur=2&MevzuatTertip=5),
  read). Other half-mast days are announced case by case (national mourning). The regulation
  sets the day, not an hour. In practice the country stops at **09:05**, the minute Atatürk died
  in 1938, with sirens and a two-minute silence, and many reports describe the flag as lowered
  from 09:05 until sunset (for example [NTV](https://www.ntv.com.tr/turkiye/galeri-10-kasimda-bayraklar-ne-zaman-yariya-indirilir-1695098)).
  Showing it all day is safe. 10 November is a commemoration, not a day off. No other day is
  half-mast every year; 18 March and 15 July keep the flag at full mast. 15 July is a public
  holiday, so it has no `_half_mast` twin.
- **Observances chosen:** 18 March (Çanakkale victory and Martyrs' Day, ceremonies in every
  province), Aşure Günü (households cook and share aşure pudding), Mevlid Kandili (mosques lit,
  kandil simidi pastries), 24 November Teachers' Day (every school marks it; the date of
  Atatürk's 1928 appointment as Head Teacher) and the Ramadan month. Left out: Kandil nights other
  than Mevlid (Regaip, Miraç, Berat), Kadir Gecesi (inside the Ramadan block), Mother's Day and
  Valentine's Day (commercial), Nowruz (marked mainly in the south-east, not national).
- **Greetings:** "İyi bayramlar" is the everyday wish for both bayrams; "Bayramınız mübarek
  olsun" is the more religious one. "Hayırlı Ramazanlar" for the fasting month, "Kandiliniz
  mübarek olsun" for the kandil night. The national days use the standard "... Kutlu Olsun".
- **Looks:** Republic Day gets `flag+fireworks_night` (city fireworks and the Bosphorus show on
  29 Oct evening) with `eve` for the 28 Oct 13:00 start. New Year gets `fireworks_night` with `eve`;
  the generator skips it in favour of the global New Year. The other national days use `flag`.

### Country notes (TR)

- **Christmas / New Year:** **25 Dec is not a holiday** and Christmas itself is little marked
  outside the small Christian communities and tourist areas. **1 Jan is a public holiday**, and
  New Year (Yılbaşı) is celebrated with a decorated tree, Noel Baba (Santa Claus), lights and
  midnight fireworks. In Turkey these trappings belong to New Year, not Christmas. Keep the New
  Year countdown. The 15–26 Dec lights fit Turkish streets, but the framing is New Year: show them
  as New Year lights running to 31 Dec rather than as Christmas.
- **Flag:** 2:3, **red field (#E30A17) with a white crescent and a white five-point star**. The
  crescent sits in the hoist half, opening towards the fly, with the star just outside its mouth
  and one point towards the hoist (Law 2893 / Flag Regulation). For 9×3: all cells red, with a
  white crescent glyph in col 3 and a white star glyph in col 4 on row 2.
- **Geocoder language:** `tr`.
- **Anchor cities** (Ankara is 39.93, 32.86):
  - Edirne 41.68, 26.56 (Greek and Bulgarian borders)
  - Istanbul 41.01, 28.98 (largest city, straddles Europe and Asia)
  - İzmir 38.42, 27.14 (Aegean coast)
  - Kaş 36.20, 29.64 (south coast, opposite Kastellorizo, GR)
  - Antakya 36.20, 36.16 (Hatay, Syrian border)
  - Hakkari 37.58, 43.74 (Iraqi and Iranian borders)
  - Doğubayazıt 39.55, 44.08 (Iranian border, Mount Ararat)
  - Hopa 41.39, 41.43 (Black Sea, Georgian border)

---

## Germany (DE)

```toml
[[holiday]]
country = "DE"
id = "new_year"
name = "Neujahr (New Year's Day)"
rule = "fixed(1,1)"
dates_2026 = ["2026-01-01"]
dates_2027 = ["2027-01-01"]
dates_2028 = ["2028-01-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = true
greeting = "Frohes neues Jahr"
greeting_en = "Happy New Year"
look = "fireworks_night"
source = ["https://www.bmi.bund.de/DE/themen/verfassung/staatliche-symbole/nationale-feiertage/nationale-feiertage-node.html", "https://www.officeholidays.com/countries/germany/2026", "https://www.officeholidays.com/countries/germany/2027"]

[[holiday]]
country = "DE"
id = "good_friday"
name = "Karfreitag (Good Friday)"
rule = "easter(-2)"
dates_2026 = ["2026-04-03"]
dates_2027 = ["2027-03-26"]
dates_2028 = ["2028-04-14"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Karfreitag"
greeting_en = "Good Friday"
look = "none"
source = ["https://www.bmi.bund.de/DE/themen/verfassung/staatliche-symbole/nationale-feiertage/nationale-feiertage-node.html", "https://www.officeholidays.com/countries/germany/2026", "https://www.officeholidays.com/countries/germany/2027"]

[[holiday]]
country = "DE"
id = "easter_monday"
name = "Ostermontag (Easter Monday)"
rule = "easter(1)"
dates_2026 = ["2026-04-06"]
dates_2027 = ["2027-03-29"]
dates_2028 = ["2028-04-17"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Frohe Ostern"
greeting_en = "Happy Easter"
look = "easter_eggs"
source = ["https://www.bmi.bund.de/DE/themen/verfassung/staatliche-symbole/nationale-feiertage/nationale-feiertage-node.html", "https://www.officeholidays.com/countries/germany/2026", "https://www.officeholidays.com/countries/germany/2027"]

[[holiday]]
country = "DE"
id = "labour_day"
name = "Tag der Arbeit (Labour Day)"
rule = "fixed(5,1)"
dates_2026 = ["2026-05-01"]
dates_2027 = ["2027-05-01"]
dates_2028 = ["2028-05-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Tag der Arbeit"
greeting_en = "Labour Day"
look = "none"
source = ["https://www.bmi.bund.de/DE/themen/verfassung/staatliche-symbole/nationale-feiertage/nationale-feiertage-node.html", "https://www.officeholidays.com/countries/germany/2026", "https://www.officeholidays.com/countries/germany/2027"]

[[holiday]]
country = "DE"
id = "ascension"
name = "Christi Himmelfahrt (Ascension Day)"
rule = "easter(39)"
dates_2026 = ["2026-05-14"]
dates_2027 = ["2027-05-06"]
dates_2028 = ["2028-05-25"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Christi Himmelfahrt"
greeting_en = "Ascension Day"
look = "none"
source = ["https://www.bmi.bund.de/DE/themen/verfassung/staatliche-symbole/nationale-feiertage/nationale-feiertage-node.html", "https://www.officeholidays.com/countries/germany/2026", "https://www.officeholidays.com/countries/germany/2027"]

[[holiday]]
country = "DE"
id = "whit_monday"
name = "Pfingstmontag (Whit Monday)"
rule = "easter(50)"
dates_2026 = ["2026-05-25"]
dates_2027 = ["2027-05-17"]
dates_2028 = ["2028-06-05"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Frohe Pfingsten"
greeting_en = "Happy Pentecost"
look = "none"
source = ["https://www.bmi.bund.de/DE/themen/verfassung/staatliche-symbole/nationale-feiertage/nationale-feiertage-node.html", "https://www.officeholidays.com/countries/germany/2026", "https://www.officeholidays.com/countries/germany/2027"]

[[holiday]]
country = "DE"
id = "national_day"
name = "Tag der Deutschen Einheit (Day of German Unity)"
rule = "fixed(10,3)"
dates_2026 = ["2026-10-03"]
dates_2027 = ["2027-10-03"]
dates_2028 = ["2028-10-03"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Tag der Deutschen Einheit"
greeting_en = "German Unity Day · {since 1990} years"
look = "flag"
source = ["https://www.bmi.bund.de/DE/themen/verfassung/staatliche-symbole/nationale-feiertage/nationale-feiertage-node.html", "https://www.officeholidays.com/countries/germany/2026", "https://www.officeholidays.com/countries/germany/2027"]

[[holiday]]
country = "DE"
id = "christmas"
name = "Erster Weihnachtstag (Christmas Day)"
rule = "fixed(12,25)"
dates_2026 = ["2026-12-25"]
dates_2027 = ["2027-12-25"]
dates_2028 = ["2028-12-25"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = true
greeting = "Frohe Weihnachten"
greeting_en = "Merry Christmas"
look = "none"
source = ["https://www.bmi.bund.de/DE/themen/verfassung/staatliche-symbole/nationale-feiertage/nationale-feiertage-node.html", "https://www.officeholidays.com/countries/germany/2026", "https://www.officeholidays.com/countries/germany/2027"]

[[holiday]]
country = "DE"
id = "christmas_second_day"
name = "Zweiter Weihnachtstag (Second Day of Christmas, St Stephen's Day)"
rule = "fixed(12,26)"
dates_2026 = ["2026-12-26"]
dates_2027 = ["2027-12-26"]
dates_2028 = ["2028-12-26"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Frohe Weihnachten"
greeting_en = "Merry Christmas"
look = "none"
source = ["https://www.bmi.bund.de/DE/themen/verfassung/staatliche-symbole/nationale-feiertage/nationale-feiertage-node.html", "https://www.officeholidays.com/countries/germany/2026", "https://www.officeholidays.com/countries/germany/2027"]

[[holiday]]
country = "DE"
id = "rosenmontag_carnival"
name = "Rosenmontag (Carnival Monday; not a day off)"
rule = "easter(-48)"
dates_2026 = ["2026-02-16"]
dates_2027 = ["2027-02-08"]
dates_2028 = ["2028-02-28"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Helau und Alaaf!"
greeting_en = "Happy Carnival"
look = "carnival"
source = ["https://www.officeholidays.com/countries/germany/2026", "https://www.officeholidays.com/countries/germany/2027", "https://www.timeanddate.com/holidays/germany/2027"]

[[holiday]]
country = "DE"
id = "easter_sunday"
name = "Ostersonntag (Easter Sunday; a legal holiday only in Brandenburg)"
rule = "easter(0)"
dates_2026 = ["2026-04-05"]
dates_2027 = ["2027-03-28"]
dates_2028 = ["2028-04-16"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Frohe Ostern"
greeting_en = "Happy Easter"
look = "easter_eggs"
source = ["https://www.officeholidays.com/countries/germany/2026", "https://www.officeholidays.com/countries/germany/2027", "https://www.timeanddate.com/holidays/germany/2027"]

[[holiday]]
country = "DE"
id = "st_martin"
name = "Martinstag (St Martin's Day, lantern processions; not a day off)"
rule = "fixed(11,11)"
dates_2026 = ["2026-11-11"]
dates_2027 = ["2027-11-11"]
dates_2028 = ["2028-11-11"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Sankt Martin"
greeting_en = "St Martin's Day"
look = "lanterns"
source = ["https://www.timeanddate.com/holidays/germany/2027", "https://www.officeholidays.com/countries/germany/2027"]

[[holiday]]
country = "DE"
id = "advent"
name = "Advent (the four Sundays before Christmas; not days off)"
rule = "listed"
dates_2026 = ["2026-11-29", "2026-12-06", "2026-12-13", "2026-12-20"]
dates_2027 = ["2027-11-28", "2027-12-05", "2027-12-12", "2027-12-19"]
dates_2028 = ["2028-12-03", "2028-12-10", "2028-12-17", "2028-12-24"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Schönen Advent"
greeting_en = "Happy Advent"
look = "candles+wreath"
source = ["https://www.timeanddate.com/holidays/germany/2027"]

[[holiday]]
country = "DE"
id = "nikolaus"
name = "Nikolaustag (St Nicholas' Day; not a day off)"
rule = "fixed(12,6)"
dates_2026 = ["2026-12-06"]
dates_2027 = ["2027-12-06"]
dates_2028 = ["2028-12-06"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = true
greeting = "Frohen Nikolaus"
greeting_en = "St Nicholas' Day"
look = "none"
source = ["https://www.timeanddate.com/holidays/germany/2027"]

[[holiday]]
country = "DE"
id = "christmas_eve"
name = "Heiligabend (Christmas Eve; not a legal holiday, shops close at 14:00)"
rule = "fixed(12,24)"
dates_2026 = ["2026-12-24"]
dates_2027 = ["2027-12-24"]
dates_2028 = ["2028-12-24"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Frohe Weihnachten"
greeting_en = "Merry Christmas"
look = "candles"
source = ["https://www.timeanddate.com/holidays/germany/2027"]

[[holiday]]
country = "DE"
id = "holocaust_remembrance_day"
name = "Tag des Gedenkens an die Opfer des Nationalsozialismus (Holocaust Remembrance Day; flags at half-mast)"
rule = "fixed(1,27)"
dates_2026 = ["2026-01-27"]
dates_2027 = ["2027-01-27"]
dates_2028 = ["2028-01-27"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Gedenktag für die Opfer des Nationalsozialismus"
greeting_en = "Holocaust Remembrance Day"
look = "half_mast"
source = ["https://www.zaberfeld.de/resources/ecics_3320.pdf", "https://bravors.brandenburg.de/verwaltungsvorschriften/beflaggung_2018"]

[[holiday]]
country = "DE"
id = "terror_victims_remembrance_day"
name = "Gedenktag für die Opfer terroristischer Gewalt (Remembrance Day for Victims of Terrorism; flags at half-mast)"
rule = "fixed(3,11)"
dates_2026 = ["2026-03-11"]
dates_2027 = ["2027-03-11"]
dates_2028 = ["2028-03-11"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Gedenktag für die Opfer terroristischer Gewalt"
greeting_en = "Terror Victims Remembrance"
look = "half_mast"
source = ["https://www.bundesregierung.de/breg-de/aktuelles/11-maerz-nationaler-gedenktag-2004930", "https://www.zaberfeld.de/resources/ecics_3320.pdf"]

[[holiday]]
country = "DE"
id = "volkstrauertag"
name = "Volkstrauertag (National Day of Mourning; flags at half-mast)"
rule = "listed"
dates_2026 = ["2026-11-15"]
dates_2027 = ["2027-11-14"]
dates_2028 = ["2028-11-19"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Volkstrauertag"
greeting_en = "Day of National Mourning"
look = "half_mast"
source = ["https://www.zaberfeld.de/resources/ecics_3320.pdf", "https://bravors.brandenburg.de/verwaltungsvorschriften/beflaggung_2018", "https://www.timeanddate.com/holidays/germany/2027"]
```

**Notes (DE)**

- **Legal basis:** holidays are state law (each Land's Sonn- und Feiertagsgesetz). The only
  federal holiday is 3 October (Unification Treaty). The Federal Interior Ministry
  ([BMI, "Nationale Feiertage"](https://www.bmi.bund.de/DE/themen/verfassung/staatliche-symbole/nationale-feiertage/nationale-feiertage-node.html),
  read 23 Oct 2026) lists **nine holidays protected in every state**: Neujahr, Karfreitag,
  Ostermontag, Christi Himmelfahrt, Pfingstmontag, 1. Mai, Tag der Deutschen Einheit, erster and
  zweiter Weihnachtstag. Those nine are the entries above.
- **No substitute days:** a holiday on a weekend is lost (3 Oct 2026 is a Saturday, 1 May and
  25 Dec 2027 are Saturdays, 26 Dec 2027 and 3 Oct 2027 are Sundays).
- **Dates:** fixed or Easter-based, so 2027 and 2028 are certain (`tentative = false`). Easter
  2028 is 16 Apr, so Ascension is 25 May and Whit Monday 5 Jun 2028. officeholidays.com 2026 and
  2027 agree on every date. timeanddate (Wayback, 10 Mar 2026) agrees on 2027.
- **Big state holidays (not listed as entries):**
  - Heilige Drei Könige (Epiphany) 6 Jan: BW, BY, ST.
  - Internationaler Frauentag 8 Mar: Berlin, Mecklenburg-Vorpommern.
  - Fronleichnam (Corpus Christi, `easter(60)`: 4 Jun 2026, 27 May 2027, 15 Jun 2028): BW, BY,
    HE, NW, RP, SL, plus Catholic parts of SN and TH. This covers most of the south and west.
  - Mariä Himmelfahrt 15 Aug: Saarland and Catholic municipalities in Bavaria.
  - Weltkindertag 20 Sep: Thuringia.
  - Reformationstag 31 Oct: BB, HB, HH, MV, NI, SN, ST, SH, TH (all the north and east).
  - Allerheiligen 1 Nov: BW, BY, NW, RP, SL.
  - Buß- und Bettag (Wednesday before 23 Nov: 18 Nov 2026, 17 Nov 2027, 22 Nov 2028): Saxony.
  - Augsburger Hohes Friedensfest 8 Aug: city of Augsburg only.
  - Ostersonntag and Pfingstsonntag are legal holidays only in Brandenburg; they are Sundays
    everywhere anyway. Easter Sunday is kept as an observance for the eggs.
  - A coordinate-based app could add Fronleichnam, Reformationstag and Allerheiligen per state
    later; that needs state boundaries, so they are left to Notes.
- **Observances chosen:** Rosenmontag (the Rhineland carnival peak; Weiberfastnacht is the
  Thursday before, `easter(-52)`, and Mainz and Düsseldorf say "Helau", Cologne "Alaaf"), Easter
  Sunday, St Martin's Day (children's lantern processions on or near 11 Nov, hence `lanterns`),
  the four Advent Sundays (the Advent wreath gains a candle each Sunday, hence `candles`),
  Nikolaustag (children put out boots on the evening of 5 Dec, hence `eve = true`) and
  Heiligabend. Heiligabend is when Germans exchange presents (Bescherung). It is not a legal
  holiday, but shops close around 14:00 and several states treat it as a quiet day from the
  afternoon. Left out: Muttertag and Vatertag (Vatertag = Ascension Day), Oktoberfest (Bavarian),
  Halloween (global), 9 November and Totensonntag (solemn, little visible).
- **Half-mast (Trauerbeflaggung):** the federal flag decree (Erlass der Bundesregierung über die
  Beflaggung der Dienstgebäude des Bundes, in force since 2 Apr 2005, section II) sets the
  regular flag days. **Half-mast on 27 January** (Remembrance of the Victims of National
  Socialism; Auschwitz was liberated on 27 Jan 1945) and on **Volkstrauertag** (second Sunday
  before the first Advent: 15 Nov 2026, 14 Nov 2027, 19 Nov 2028). Since 2022 the federal
  government has also marked **11 March** (Remembrance Day for Victims of Terrorism, the date of
  the 2004 Madrid bombings) with half-mast flags on federal buildings
  ([Bundesregierung](https://www.bundesregierung.de/breg-de/aktuelles/11-maerz-nationaler-gedenktag-2004930)).
  Baden-Württemberg's published copy of the federal list includes it among the half-mast days
  ([Zaberfeld municipal page](https://www.zaberfeld.de/resources/ecics_3320.pdf)). Brandenburg's
  own rules ([VV Beflaggung 2018](https://bravors.brandenburg.de/verwaltungsvorschriften/beflaggung_2018),
  read) confirm 27 Jan and Volkstrauertag as half-mast days.
  - The federal decree text itself was not reachable (gesetze-im-internet 404, BMI has no page),
    so the list comes from those two state copies.
  - **Time window:** flags fly from daybreak, but not before 07:00, until sunset (Brandenburg
    VV 1.8), so the whole day. They are hoisted fully first, then lowered to half-mast.
  - None of the three falls on a public holiday, so there are no `_half_mast` twins.
  - The other flag days (1 May, 9 May, 23 May, 17 Jun, 20 Jul, 3 Oct, election days) fly the
    flag at full mast. 3 Oct already has `look = "flag"`.
- **Looks:** New Year has `fireworks_night` with `eve` because Silvester fireworks are huge. The
  generator skips DE `new_year` and `christmas` in favour of the global events. Easter Monday and
  Easter Sunday get `easter_eggs`; the Osterbaum and egg hunts are everywhere. There is no
  carnival look yet, so Rosenmontag is `none`.
- **Greetings:** "Frohe Ostern", "Frohe Pfingsten" and "Frohe Weihnachten" are the standard
  wishes. Good Friday is a quiet day (a dance ban applies in most states), so the greeting is just
  the name. "Guten Rutsch" is the New Year's Eve wish, and "Frohes neues Jahr" applies from
  1 Jan.

### Country notes (DE)

- **Christmas / New Year:** 25 and 26 Dec and 1 Jan are all public holidays. Christmas markets run
  from late November, and Silvester (31 Dec) has the biggest private fireworks of the year. Keep
  both defaults. Germany's main Christmas moment is the evening of 24 Dec, which the
  `christmas_eve` observance covers.
- **Flag:** 5:3 (3 high, 5 wide), three equal horizontal bands — **black (#000000) on top, red
  (#DD0000) in the middle, gold (#FFCE00) at the bottom** (Basic Law art. 22(2): "schwarz-rot-
  gold"). No emblem on the civil flag. For 9×3: row 1 black, row 2 red, row 3 gold.
- **Geocoder language:** `de`.
- **Anchor cities** (Berlin is 52.52, 13.40):
  - Flensburg 54.79, 9.44 (Danish border)
  - Aachen 50.78, 6.08 (Belgian and Dutch borders)
  - Saarbrücken 49.24, 6.99 (French border)
  - Freiburg im Breisgau 47.99, 7.85 (French and Swiss borders)
  - Konstanz 47.66, 9.18 (Swiss border, Lake Constance)
  - Passau 48.57, 13.43 (Austrian border)
  - Görlitz 51.15, 14.99 (Polish border, also near Czechia)
  - Frankfurt (Oder) 52.35, 14.55 (Polish border)

---

## France (FR)

```toml
[[holiday]]
country = "FR"
id = "new_year"
name = "Jour de l'An (New Year's Day)"
rule = "fixed(1,1)"
dates_2026 = ["2026-01-01"]
dates_2027 = ["2027-01-01"]
dates_2028 = ["2028-01-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = true
greeting = "Bonne année"
greeting_en = "Happy New Year"
look = "fireworks_night"
source = ["https://www.service-public.fr/particuliers/vosdroits/F2405", "https://www.officeholidays.com/countries/france/2026", "https://www.officeholidays.com/countries/france/2027", "https://www.timeanddate.com/holidays/france/2027"]

[[holiday]]
country = "FR"
id = "easter_monday"
name = "Lundi de Pâques (Easter Monday)"
rule = "easter(1)"
dates_2026 = ["2026-04-06"]
dates_2027 = ["2027-03-29"]
dates_2028 = ["2028-04-17"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Joyeuses Pâques"
greeting_en = "Happy Easter"
look = "easter_eggs"
source = ["https://www.service-public.fr/particuliers/vosdroits/F2405", "https://www.officeholidays.com/countries/france/2026", "https://www.officeholidays.com/countries/france/2027", "https://www.timeanddate.com/holidays/france/2027"]

[[holiday]]
country = "FR"
id = "labour_day"
name = "Fête du Travail (Labour Day)"
rule = "fixed(5,1)"
dates_2026 = ["2026-05-01"]
dates_2027 = ["2027-05-01"]
dates_2028 = ["2028-05-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Bonne fête du Travail"
greeting_en = "Happy Labour Day"
look = "none"
source = ["https://www.service-public.fr/particuliers/vosdroits/F2405", "https://www.officeholidays.com/countries/france/2026", "https://www.officeholidays.com/countries/france/2027", "https://www.timeanddate.com/holidays/france/2027"]

[[holiday]]
country = "FR"
id = "victory_day"
name = "Victoire 1945 (Victory in Europe Day)"
rule = "fixed(5,8)"
dates_2026 = ["2026-05-08"]
dates_2027 = ["2027-05-08"]
dates_2028 = ["2028-05-08"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "8 mai 1945"
greeting_en = "Victory in Europe Day"
look = "flag"
source = ["https://www.service-public.fr/particuliers/vosdroits/F2405", "https://www.officeholidays.com/countries/france/2026", "https://www.officeholidays.com/countries/france/2027", "https://www.timeanddate.com/holidays/france/2027"]

[[holiday]]
country = "FR"
id = "ascension"
name = "Ascension (Ascension Day)"
rule = "easter(39)"
dates_2026 = ["2026-05-14"]
dates_2027 = ["2027-05-06"]
dates_2028 = ["2028-05-25"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Ascension"
greeting_en = "Ascension Day"
look = "none"
source = ["https://www.service-public.fr/particuliers/vosdroits/F2405", "https://www.officeholidays.com/countries/france/2026", "https://www.officeholidays.com/countries/france/2027", "https://www.timeanddate.com/holidays/france/2027"]

[[holiday]]
country = "FR"
id = "whit_monday"
name = "Lundi de Pentecôte (Whit Monday)"
rule = "easter(50)"
dates_2026 = ["2026-05-25"]
dates_2027 = ["2027-05-17"]
dates_2028 = ["2028-06-05"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Lundi de Pentecôte"
greeting_en = "Whit Monday"
look = "none"
source = ["https://www.service-public.fr/particuliers/vosdroits/F2405", "https://www.officeholidays.com/countries/france/2026", "https://www.officeholidays.com/countries/france/2027", "https://www.timeanddate.com/holidays/france/2027"]

[[holiday]]
country = "FR"
id = "national_day"
name = "Fête nationale (Bastille Day, 14 July)"
rule = "fixed(7,14)"
dates_2026 = ["2026-07-14"]
dates_2027 = ["2027-07-14"]
dates_2028 = ["2028-07-14"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Bonne fête nationale"
greeting_en = "Happy Bastille Day"
look = "flag+fireworks_night"
source = ["https://www.service-public.fr/particuliers/vosdroits/F2405", "https://www.officeholidays.com/countries/france/2026", "https://www.officeholidays.com/countries/france/2027", "https://www.timeanddate.com/holidays/france/2027"]

[[holiday]]
country = "FR"
id = "assumption"
name = "Assomption (Assumption Day)"
rule = "fixed(8,15)"
dates_2026 = ["2026-08-15"]
dates_2027 = ["2027-08-15"]
dates_2028 = ["2028-08-15"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Assomption"
greeting_en = "Assumption Day"
look = "none"
source = ["https://www.service-public.fr/particuliers/vosdroits/F2405", "https://www.officeholidays.com/countries/france/2026", "https://www.officeholidays.com/countries/france/2027", "https://www.timeanddate.com/holidays/france/2027"]

[[holiday]]
country = "FR"
id = "all_saints"
name = "Toussaint (All Saints' Day)"
rule = "fixed(11,1)"
dates_2026 = ["2026-11-01"]
dates_2027 = ["2027-11-01"]
dates_2028 = ["2028-11-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Toussaint"
greeting_en = "All Saints' Day"
look = "none"
source = ["https://www.service-public.fr/particuliers/vosdroits/F2405", "https://www.officeholidays.com/countries/france/2026", "https://www.officeholidays.com/countries/france/2027", "https://www.timeanddate.com/holidays/france/2027"]

[[holiday]]
country = "FR"
id = "armistice_day"
name = "Armistice 1918 (Armistice Day)"
rule = "fixed(11,11)"
dates_2026 = ["2026-11-11"]
dates_2027 = ["2027-11-11"]
dates_2028 = ["2028-11-11"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "11 novembre 1918"
greeting_en = "Armistice Day"
look = "flag"
source = ["https://www.service-public.fr/particuliers/vosdroits/F2405", "https://www.officeholidays.com/countries/france/2026", "https://www.officeholidays.com/countries/france/2027", "https://www.timeanddate.com/holidays/france/2027"]

[[holiday]]
country = "FR"
id = "christmas"
name = "Noël (Christmas Day)"
rule = "fixed(12,25)"
dates_2026 = ["2026-12-25"]
dates_2027 = ["2027-12-25"]
dates_2028 = ["2028-12-25"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = true
greeting = "Joyeux Noël"
greeting_en = "Merry Christmas"
look = "none"
source = ["https://www.service-public.fr/particuliers/vosdroits/F2405", "https://www.officeholidays.com/countries/france/2026", "https://www.officeholidays.com/countries/france/2027", "https://www.timeanddate.com/holidays/france/2027"]

[[holiday]]
country = "FR"
id = "epiphany"
name = "Épiphanie / galette des rois (Epiphany; not a day off)"
rule = "fixed(1,6)"
dates_2026 = ["2026-01-06"]
dates_2027 = ["2027-01-06"]
dates_2028 = ["2028-01-06"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Bonne fête des rois"
greeting_en = "Happy Epiphany"
look = "none"
source = ["https://fr.wikipedia.org/wiki/Galette_des_rois"]

[[holiday]]
country = "FR"
id = "chandeleur"
name = "Chandeleur (Candlemas, crêpe day; not a day off)"
rule = "fixed(2,2)"
dates_2026 = ["2026-02-02"]
dates_2027 = ["2027-02-02"]
dates_2028 = ["2028-02-02"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Bonne Chandeleur"
greeting_en = "Happy Candlemas"
look = "none"
source = ["https://fr.wikipedia.org/wiki/Chandeleur"]

[[holiday]]
country = "FR"
id = "mardi_gras_carnival"
name = "Mardi gras (Shrove Tuesday, carnival; not a day off)"
rule = "easter(-47)"
dates_2026 = ["2026-02-17"]
dates_2027 = ["2027-02-09"]
dates_2028 = ["2028-02-29"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Bon Mardi gras"
greeting_en = "Happy Mardi Gras"
look = "carnival"
source = ["https://fr.wikipedia.org/wiki/Mardi_gras", "https://www.nicecarnaval.com/"]

[[holiday]]
country = "FR"
id = "easter_sunday"
name = "Pâques (Easter Sunday)"
rule = "easter(0)"
dates_2026 = ["2026-04-05"]
dates_2027 = ["2027-03-28"]
dates_2028 = ["2028-04-16"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Joyeuses Pâques"
greeting_en = "Happy Easter"
look = "easter_eggs"
source = ["https://www.timeanddate.com/holidays/france/2027", "https://www.officeholidays.com/countries/france/2026"]

[[holiday]]
country = "FR"
id = "fete_de_la_musique"
name = "Fête de la Musique (Music Day; not a day off)"
rule = "fixed(6,21)"
dates_2026 = ["2026-06-21"]
dates_2027 = ["2027-06-21"]
dates_2028 = ["2028-06-21"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Bonne Fête de la Musique"
greeting_en = "Happy Music Day"
look = "none"
source = ["https://fetedelamusique.culture.gouv.fr/", "https://www.culture.gouv.fr/"]
```

**Notes (FR)**

- **Legal basis:** Code du travail art. L3133-1 lists the 11 "fêtes légales". The official
  dates come from [service-public.fr F2405](https://www.service-public.fr/particuliers/vosdroits/F2405),
  "Vérifié le 01 janvier 2026" (read 23 Oct 2026), which gives the 2026 and 2027 tables for the
  general case, Alsace-Moselle and overseas. Every 2026 and 2027 date above matches it.
  2028 is computed from the same fixed dates and Easter (16 Apr 2028).
- **Not every jour férié is a day off.** Only 1 May is compulsorily non-working for everyone; for
  the other ten, being off depends on the collective agreement, although most employees are off
  in practice. Lundi de Pentecôte is often the **journée de solidarité**, an unpaid working day
  set by the company. It is still a legal holiday, so it is kept.
- **No substitute days:** a holiday on a weekend is lost (2026: 15 Aug Sat, 1 Nov Sun; 2027: 1 and
  8 May Sat, 15 Aug Sun, 25 Dec Sat; 2028: 1 Jan Sat). "Ponts" (bridge days) are at the
  employer's discretion, for example Fri 15 May 2026 after Ascension and Fri 12 Nov 2027 after
  Armistice.
- **The 2025 plan to scrap Easter Monday and 8 May** (Prime Minister Bayrou, July 2025, for the
  2026 budget) was not enacted: service-public.fr still lists both for 2026 and 2027. If a later
  budget revives it, these two entries would need a year cut-off.
- **Alsace-Moselle** (Bas-Rhin, Haut-Rhin, Moselle) has two more holidays under local law:
  **Vendredi saint** (Good Friday; 3 Apr 2026, 26 Mar 2027, 14 Apr 2028) and the **2e jour de
  Noël / Saint-Étienne** (26 Dec). Under the law, Good Friday applies in communes with a
  Protestant church or mixed church, which in practice means the whole region. Not added, since
  the brief is metropolitan national holidays; a Strasbourg/Metz bounding box could switch them
  on later.
- **Overseas:** slavery-abolition days are public holidays in each DROM (Mayotte 27 Apr,
  Martinique 22 May, Guadeloupe and Saint-Martin 27 May, Guyane 10 Jun, Saint-Barthélemy 9 Oct,
  La Réunion 20 Dec) per service-public.fr. Not added.
- **Cross-checks:** officeholidays.com 2026 and 2027 and timeanddate 2027 (Wayback, 7 Mar 2026)
  agree on every date. officeholidays tags Good Friday and 26 Dec as "region" (Alsace-Moselle).
- **Half-mast:** **none fixed.** No law or decree sets annual half-mast ("en berne") days.
  Apart from art. 2 of the Constitution (the tricolour), flag display on public buildings rests
  on custom and on instructions from the Prime Minister (Secrétariat général du Gouvernement),
  relayed by the Interior Ministry to prefects. They order **full-mast** pavoisement for the
  national commemoration days (8 May, 14 July, 11 Nov, Déportation day, etc.). Half-mast is
  ordered case by case for national mourning (for example Jacques Chirac, 30 Sep 2019; the
  Élysée's order for Pope Francis, April 2025), sources
  [Assemblée nationale written answer](https://questions.assemblee-nationale.fr/dyn/14/questions/QANR5L14QE59082.pdf)
  and [Public Sénat](https://www.publicsenat.fr/actualites/politique/mort-du-pape-la-mise-en-berne-des-drapeaux-fait-polemique-jusquau-senat).
- **Observances chosen:** Épiphanie (the galette des rois is shared through January; the Church
  in France celebrates Epiphany on the Sunday between 2 and 8 Jan, so 6 Jan is the customary
  date, not the liturgical one), Chandeleur (2 Feb, crêpes), Mardi gras (`easter(-47)`, carnival
  and beignets; Nice and Dunkirk carnivals run around it), Easter Sunday (chocolate eggs and
  bells, hence `easter_eggs`) and Fête de la Musique (21 Jun, free music in every town since
  1982, run by the Culture Ministry). Left out: Fête des mères and des pères (commercial),
  Poisson d'avril (a prank day), Beaujolais nouveau (commercial), Saint-Sylvestre (the eve of
  New Year).
- **Looks:** 14 July gets `flag+fireworks_night` (the Bastille Day parade in the morning, fireworks
  at night; the bals des pompiers are on the 13th and 14th, so no `eve`). New Year has
  `fireworks_night` with `eve`; the generator skips FR `new_year` and `christmas` in favour of the
  global events. 8 May and 11 Nov use `flag` (ceremonies at every war memorial).
- **Greetings:** "Bonne année", "Joyeuses Pâques", "Joyeux Noël" and "Bonne fête nationale" are
  what people say. 1 May is muguet (lily of the valley) day. 8 May and 11 Nov are
  commemorations, so the greeting is the date.

### Country notes (FR)

- **Christmas / New Year:** 25 Dec and 1 Jan are both jours fériés. Christmas lights and markets
  (Strasbourg's is the oldest) run from late November, and the Réveillon de la Saint-Sylvestre
  has city fireworks. Keep both defaults.
- **Flag:** 2:3, **three equal vertical bands — blue at the hoist, white, red at the fly**
  (Constitution art. 2: "bleu, blanc, rouge"). Blue #000091 (the navy shade the Élysée has used
  since 2020; the older standard is #0055A4), white #FFFFFF, red #E1000F. For 9×3: cols 1–3 blue,
  4–6 white, 7–9 red on all three rows.
- **Geocoder language:** `fr`.
- **Anchor cities** (Paris is 48.86, 2.35):
  - Lille 50.63, 3.06 (Belgian border)
  - Strasbourg 48.57, 7.75 (German border, Alsace-Moselle)
  - Chamonix 45.92, 6.87 (Italian and Swiss borders)
  - Nice 43.70, 7.27 (near Monaco and Italy)
  - Ajaccio 41.93, 8.74 (Corsica)
  - Perpignan 42.70, 2.90 (near Spain, Catalan side)
  - Hendaye 43.36, -1.77 (Spanish border, Basque coast)
  - Brest 48.39, -4.49 (far west, Brittany)
  - Overseas departments geocode as their own ISO codes in many geocoders (GP, MQ, GF, RE, YT);
    they would need an FR mapping if the overseas holidays are ever added.

---

## Counts

| Country | Holidays (days off) | Observances | Half-mast | 2027 status | 2028 status |
|---|---|---|---|---|---|
| EG | 13 (the 12 occasions of PM Decree 1193/2018, with Arafat split from Eid al-Adha; Eid al-Fitr ×3 days, Eid al-Adha ×4) | 3 (Ramadan, Mother's Day, Coptic Easter) | none fixed | fixed days certain, but each day off is set by PM decree and often moved to a Thursday (`tentative`); Islamic days per NRIAG (Eid al-Fitr 9 Mar may become 10 Mar) and own calc (Mawlid 15 Aug) | not published; Islamic days computed (Eid al-Fitr 27 Feb, Ramadan 30 days), fixed days may move to Thursdays |
| TR | 9 (Ramazan Bayramı ×3 days, Kurban Bayramı ×4, plus arife and 28 Oct half-days via `eve`) | 5 (Ramadan, 18 March Çanakkale, Aşure, Mevlid Kandili, Teachers' Day) | 1 (10 Nov, Flag Regulation art. 24) | official (Diyanet's tables, calculated calendar, not tentative) | official (Diyanet's tables, not tentative) |
| DE | 9 nationwide (state holidays in Notes) | 6 (Rosenmontag, Easter Sunday, St Martin, Advent ×4 Sundays, Nikolaus, Heiligabend) | 3 (27 Jan, 11 Mar, Volkstrauertag) | certain (fixed/Easter; officeholidays and timeanddate agree) | certain (fixed/Easter) |
| FR | 11 metropolitan (Alsace-Moselle +2 in Notes) | 5 (Épiphanie, Chandeleur, Mardi gras, Easter Sunday, Fête de la Musique) | none fixed | official (service-public.fr lists 2027) | certain (fixed/Easter), barring a law change |

**Looks wished for:** `carnival` (Rosenmontag, Mardi gras), `sugar_doll` or a generic `sweets`
look for Egypt's Mawlid (aroset el-moulid), `music` for Fête de la Musique, `crescent_star` for
Turkey's kandil nights (the `ramadan` crescent is close), `lily_of_the_valley` for France's
1 May (muguet), and `wreath` for remembrance days (Volkstrauertag, 11 Nov) if `half_mast` alone
feels too bare.
