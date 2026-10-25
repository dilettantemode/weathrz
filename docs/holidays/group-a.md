# World holidays, group A: IN, PK, BD, MY (2026 + 2027 + 2028)

Compiled 2026-10-20. 2027 re-checked and 2028 added on 2026-10-21. Dates that have already passed in 2026 are the dates actually observed (moon-sighting outcomes checked in news reports). Every lunar date was checked against at least two sources. The disagreements are listed in each country's Notes. All 2028 dates are estimates.

Status of the official lists:

| Country | 2027 list | Where | 2028 list |
|---|---|---|---|
| IN | **Published.** DoP&T O.M. F.No.12/2/2023-JCA dated 16.07.2026 (17 gazetted + 32 restricted holidays). Checked 21 Oct 2026: no amendment, and no Ambedkar Jayanti 2027 O.M. yet (it usually comes in late Mar/early Apr). | https://www.staffnews.in/2026/07/list-of-holidays-for-the-year-2027-gazetted.html , PDF mirror https://www.gconnect.in/gc22/wp-content/uploads/2026/07/Holiday-list-2027.pdf | Not published (expected ~Jul 2027). |
| PK | **Not published.** Checked 21 Oct 2026: still not published. The Cabinet Division usually issues it in Dec/Jan. The 2026 circular was dated 19.01.2026. | (estimates only) | Not published (expected Dec 2027/Jan 2028). |
| BD | **Not published.** Checked 21 Oct 2026: still not published. The cabinet usually approves it in Oct/Nov. The 2026 list was approved 06.11.2025. | (estimates only) | Not published (expected Oct/Nov 2027). |
| MY | **Published.** Federal schedule from BKPP, Prime Minister's Dept., reported 21–25 Aug 2026. Hari Raya and Deepavali carry "*subject to change". Checked 21 Oct 2026: no amendment found. | https://www.humanresourcesonline.net/malaysia-s-2027-public-holidays-full-list-of-federal-and-state-holidays , https://ecentral.my/cuti-2027/ (official page https://www.kabinet.gov.my/hari-kelepasan-am/ refused connection from here) | Not published (expected ~Aug 2027). |

Primary sources:
- IN: DoP&T O.M. dated 03.07.2025 (2026 list, Annexure-I gazetted / Annexure-II restricted), the O.M. of 22.05.2026 that moved Bakrid, and the O.M. of 09.04.2026 that added Ambedkar Jayanti. All read through staffnews.in reproductions; the DoP&T PDF itself is a scanned image.
- PK: Cabinet Division circular No. 10-1/2025-Min-II dated 19.01.2026 (via Daily Pakistan), and the later Cabinet Division notifications for the Eids, Ashura and Milad (via APP, Zameen, The Standard, Daily Pakistan, Shafaqna).
- BD: Ministry of Public Administration 2026 list (Daily Star), officeholidays.com 2026, bdnews24 for the Eid blocks, and Prothom Alo for the 7 Nov reinstatement and the extra 22 Oct day. 2027 estimates come from bangladatetoday.com, checked against officeholidays.com and astronomy.
- MY: JPM/BKPP 2026 schedule (ecentral.my mirror of the official PDF), officeholidays.com 2026/2027, Bernama (Aidiladha), Malay Mail and Berita Harian (Aidilfitri extra day).
- 2028 (no official list anywhere): Hindu and Sikh dates from drikpanchang.com (New Delhi) checked against calendarlabs.com. Bangladesh from bangladatetoday.com 2028 and the hijri.habibur.com Bangladesh tables. Malaysia from kalendarcuti.my, time.now and holidaydb.com, the Hong Kong Observatory 2028 lunar table (CNY, Wesak), and hijri.habibur.com. officeholidays.com has no 2028 pages and timeanddate.com refused the fetch.
- 2028 Islamic dates: I computed the new moons (Meeus) and the crescent at local sunset (Yallop q for Karachi, Dhaka and Delhi; the MABIMS rule of altitude ≥ 3° and elongation ≥ 6.4° for Malaysia). Calibrated on past outcomes: Pakistan has sighted at q ≈ +0.07 (2016, 2023, 2024) but usually not at +0.01 or below (2020, 2022). Bangladesh and India have missed crescents up to q = +0.10 (2016, 2024, 2026). MABIMS reproduces Malaysia's 2027 official dates. The global calendar sites (drikpanchang, calendarlabs, time.now, kalendarcuti.my) follow the Saudi Umm al-Qura month starts, which run a day ahead of South Asia most years. drikpanchang's 2026 forecasts for India (Eid 20 Mar, Bakrid 27 May) were each a day early.

Conventions (same as group B):
- `rule` is the anchor rule of the holiday. `listed` covers all lunar or lunisolar dates and announced dates.
- `dates_*` are ISO strings of the days actually celebrated. For multi-day Eids I list Eid days 1–3 where the country celebrates three days (PK, BD). Bridge days, executive-order padding and replacement days appear in Notes only.
- `eve = true` means the observance starts the evening before the first listed date. Islamic days start at sunset. The Bangladesh Shab-e-Barat and Shab-e-Qadr holiday falls on the day after the holy night.
- `tentative_2027 = true` when the 2027 date depends on moon sighting or the country's 2027 list is not out. PK and BD are therefore `true` throughout; their `fixed(M,D)` civic dates are statutory and not really in doubt.
- `tentative_2028 = true` for every `listed` date (no country has a 2028 list), for PK and BD throughout, and for IN Ambedkar Jayanti (it needs its own O.M. each year). The other IN and MY rule-based dates are `false`.
- Greetings are romanised and ASCII. Solemn days use the plain name.

## India (IN)

```toml
[[holiday]]
country = "IN"
id = "republic_day"
name = "Republic Day (Gantantra Diwas)"
rule = "fixed(1,26)"
dates_2026 = ["2026-01-26"]
dates_2027 = ["2027-01-26"]
dates_2028 = ["2028-01-26"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Happy Republic Day"
greeting_en = "Happy {nth since 1949} Republic Day, India"
look = "flag"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-closed-holidays.html", "https://www.staffnews.in/2026/07/list-of-holidays-for-the-year-2027-gazetted.html"]

[[holiday]]
country = "IN"
id = "makar_sankranti"
name = "Makar Sankranti (restricted holiday; Pongal / Magha Bihu / Lohri region-wise)"
rule = "listed"
dates_2026 = ["2026-01-14"]
dates_2027 = ["2027-01-14"]
dates_2028 = ["2028-01-15"]
tentative_2027 = false
tentative_2028 = true
eve = false
greeting = "Happy Makar Sankranti"
greeting_en = "Happy Makar Sankranti"
look = "kites"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-restricted-holidays.html", "https://www.staffnews.in/2026/07/list-of-restricted-holidays-for-the-year-2027.html", "https://www.drikpanchang.com/calendars/indian/indiancalendar.html?year=2028", "https://www.calendarlabs.com/holidays/india/2028"]

[[holiday]]
country = "IN"
id = "maha_shivratri"
name = "Maha Shivratri (restricted holiday)"
rule = "listed"
dates_2026 = ["2026-02-15"]
dates_2027 = ["2027-03-06"]
dates_2028 = ["2028-02-23"]
tentative_2027 = false
tentative_2028 = true
eve = false
greeting = "Happy Maha Shivratri"
greeting_en = "Happy Maha Shivratri"
look = "none"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-restricted-holidays.html", "https://www.staffnews.in/2026/07/list-of-restricted-holidays-for-the-year-2027.html", "https://www.drikpanchang.com/calendars/indian/indiancalendar.html?year=2028", "https://www.calendarlabs.com/holidays/india/2028"]

[[holiday]]
country = "IN"
id = "holi"
name = "Holi"
rule = "listed"
dates_2026 = ["2026-03-04"]
dates_2027 = ["2027-03-23"]
dates_2028 = ["2028-03-11"]
tentative_2027 = false
tentative_2028 = true
eve = true
greeting = "Happy Holi"
greeting_en = "Happy Holi"
look = "holi"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-closed-holidays.html", "https://www.staffnews.in/2026/07/list-of-holidays-for-the-year-2027-gazetted.html", "https://www.drikpanchang.com/calendars/indian/indiancalendar.html?year=2028", "https://www.calendarlabs.com/holidays/india/2028"]

[[holiday]]
country = "IN"
id = "eid_al_fitr"
name = "Id-ul-Fitr (Eid al-Fitr)"
rule = "listed"
dates_2026 = ["2026-03-21"]
dates_2027 = ["2027-03-10"]
dates_2028 = ["2028-02-27"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Eid Mubarak"
greeting_en = "Happy Eid al-Fitr {era} AH"
look = "eid"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-closed-holidays.html", "https://www.etvbharat.com/en/state/eid-to-be-celebrated-on-saturday-after-no-moon-sighting-says-jama-masjid-naib-imam-enn26032001535", "https://www.staffnews.in/2026/07/list-of-holidays-for-the-year-2027-gazetted.html", "https://www.drikpanchang.com/calendars/indian/indiancalendar.html?year=2028", "https://www.calendarlabs.com/holidays/india/2028"]

[[holiday]]
country = "IN"
id = "ram_navami"
name = "Ram Navami"
rule = "listed"
dates_2026 = ["2026-03-26"]
dates_2027 = ["2027-04-15"]
dates_2028 = ["2028-04-03"]
tentative_2027 = false
tentative_2028 = true
eve = false
greeting = "Happy Ram Navami"
greeting_en = "Happy Ram Navami"
look = "none"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-closed-holidays.html", "https://www.staffnews.in/2026/07/list-of-holidays-for-the-year-2027-gazetted.html", "https://www.drikpanchang.com/calendars/indian/indiancalendar.html?year=2028"]

[[holiday]]
country = "IN"
id = "mahavir_jayanti"
name = "Mahavir Jayanti"
rule = "listed"
dates_2026 = ["2026-03-31"]
dates_2027 = ["2027-04-19"]
dates_2028 = ["2028-04-07"]
tentative_2027 = false
tentative_2028 = true
eve = false
greeting = "Happy Mahavir Jayanti"
greeting_en = "Happy Mahavir Jayanti"
look = "none"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-closed-holidays.html", "https://www.staffnews.in/2026/07/list-of-holidays-for-the-year-2027-gazetted.html", "https://www.drikpanchang.com/calendars/indian/indiancalendar.html?year=2028", "https://www.calendarlabs.com/holidays/india/2028"]

[[holiday]]
country = "IN"
id = "good_friday"
name = "Good Friday"
rule = "easter(-2)"
dates_2026 = ["2026-04-03"]
dates_2027 = ["2027-03-26"]
dates_2028 = ["2028-04-14"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Good Friday"
greeting_en = "Good Friday"
look = "none"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-closed-holidays.html", "https://www.staffnews.in/2026/07/list-of-holidays-for-the-year-2027-gazetted.html"]

[[holiday]]
country = "IN"
id = "ambedkar_jayanti"
name = "Dr. B.R. Ambedkar's Birthday (Ambedkar Jayanti)"
rule = "fixed(4,14)"
dates_2026 = ["2026-04-14"]
dates_2027 = ["2027-04-14"]
dates_2028 = ["2028-04-14"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Happy Ambedkar Jayanti"
greeting_en = "Happy Ambedkar Jayanti"
look = "none"
source = ["https://www.staffnews.in/2026/04/declaration-of-holiday-on-14th-april-2026.html"]

[[holiday]]
country = "IN"
id = "vesak"
name = "Buddha Purnima"
rule = "listed"
dates_2026 = ["2026-05-01"]
dates_2027 = ["2027-05-20"]
dates_2028 = ["2028-05-08"]
tentative_2027 = false
tentative_2028 = true
eve = false
greeting = "Happy Buddha Purnima"
greeting_en = "Happy Vesak"
look = "candles"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-closed-holidays.html", "https://www.staffnews.in/2026/07/list-of-holidays-for-the-year-2027-gazetted.html", "https://www.drikpanchang.com/calendars/indian/indiancalendar.html?year=2028", "https://www.calendarlabs.com/holidays/india/2028"]

[[holiday]]
country = "IN"
id = "eid_al_adha"
name = "Id-ul-Zuha (Bakrid / Eid al-Adha)"
rule = "listed"
dates_2026 = ["2026-05-28"]
dates_2027 = ["2027-05-17"]
dates_2028 = ["2028-05-06"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Bakrid Mubarak"
greeting_en = "Happy Eid al-Adha {era} AH"
look = "eid"
source = ["https://www.peoplematters.in/news/business/bakrid-2026-holiday-moved-from-may-27-to-may-28-says-centre-49897", "https://www.staffnews.in/2026/07/list-of-holidays-for-the-year-2027-gazetted.html", "https://hijri.habibur.com/bd/1449/"]

[[holiday]]
country = "IN"
id = "ashura"
name = "Muharram (Ashura, 10 Muharram)"
rule = "listed"
dates_2026 = ["2026-06-26"]
dates_2027 = ["2027-06-16"]
dates_2028 = ["2028-06-04"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Muharram"
greeting_en = "Ashura"
look = "none"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-closed-holidays.html", "https://blog.wego.com/muharram-holiday-in-india/", "https://www.staffnews.in/2026/07/list-of-holidays-for-the-year-2027-gazetted.html", "https://hijri.habibur.com/1450/", "https://hijri.habibur.com/bd/1450/"]

[[holiday]]
country = "IN"
id = "independence_day"
name = "Independence Day (Swatantrata Diwas)"
rule = "fixed(8,15)"
dates_2026 = ["2026-08-15"]
dates_2027 = ["2027-08-15"]
dates_2028 = ["2028-08-15"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Happy Independence Day"
greeting_en = "Happy {nth since 1946} Independence Day, India"
look = "flag"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-closed-holidays.html", "https://www.staffnews.in/2026/07/list-of-holidays-for-the-year-2027-gazetted.html"]

[[holiday]]
country = "IN"
id = "mawlid"
name = "Milad-un-Nabi / Id-e-Milad (Birthday of Prophet Mohammad)"
rule = "listed"
dates_2026 = ["2026-08-26"]
dates_2027 = ["2027-08-15"]
dates_2028 = ["2028-08-04"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Eid Milad-un-Nabi Mubarak"
greeting_en = "Mawlid of the Prophet"
look = "none"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-closed-holidays.html", "https://www.deccanchronicle.com/southern-states/andhra-pradesh/ap-govt-reschedules-milad-un-nabi-holiday-to-aug-1980919", "https://www.staffnews.in/2026/07/list-of-holidays-for-the-year-2027-gazetted.html", "https://hijri.habibur.com/bd/1450/"]

[[holiday]]
country = "IN"
id = "raksha_bandhan"
name = "Raksha Bandhan (restricted holiday)"
rule = "listed"
dates_2026 = ["2026-08-28"]
dates_2027 = ["2027-08-17"]
dates_2028 = ["2028-08-05"]
tentative_2027 = false
tentative_2028 = true
eve = false
greeting = "Happy Raksha Bandhan"
greeting_en = "Happy Raksha Bandhan"
look = "none"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-restricted-holidays.html", "https://www.staffnews.in/2026/07/list-of-restricted-holidays-for-the-year-2027.html", "https://www.drikpanchang.com/calendars/indian/indiancalendar.html?year=2028", "https://www.calendarlabs.com/holidays/india/2028"]

[[holiday]]
country = "IN"
id = "janmashtami"
name = "Janmashtami (Vaishnava)"
rule = "listed"
dates_2026 = ["2026-09-04"]
dates_2027 = ["2027-08-25"]
dates_2028 = ["2028-08-13"]
tentative_2027 = false
tentative_2028 = true
eve = false
greeting = "Happy Janmashtami"
greeting_en = "Happy Janmashtami"
look = "none"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-closed-holidays.html", "https://www.staffnews.in/2026/07/list-of-holidays-for-the-year-2027-gazetted.html", "https://www.drikpanchang.com/calendars/indian/indiancalendar.html?year=2028", "https://www.calendarlabs.com/holidays/india/2028"]

[[holiday]]
country = "IN"
id = "ganesh_chaturthi"
name = "Ganesh Chaturthi / Vinayaka Chaturthi (restricted holiday)"
rule = "listed"
dates_2026 = ["2026-09-14"]
dates_2027 = ["2027-09-04"]
dates_2028 = ["2028-08-23"]
tentative_2027 = false
tentative_2028 = true
eve = false
greeting = "Happy Ganesh Chaturthi"
greeting_en = "Happy Ganesh Chaturthi"
look = "none"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-restricted-holidays.html", "https://www.staffnews.in/2026/07/list-of-restricted-holidays-for-the-year-2027.html", "https://www.drikpanchang.com/calendars/indian/indiancalendar.html?year=2028", "https://www.calendarlabs.com/holidays/india/2028"]

[[holiday]]
country = "IN"
id = "gandhi_jayanti"
name = "Mahatma Gandhi's Birthday (Gandhi Jayanti)"
rule = "fixed(10,2)"
dates_2026 = ["2026-10-02"]
dates_2027 = ["2027-10-02"]
dates_2028 = ["2028-10-02"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Happy Gandhi Jayanti"
greeting_en = "Happy Gandhi Jayanti"
look = "none"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-closed-holidays.html", "https://www.staffnews.in/2026/07/list-of-holidays-for-the-year-2027-gazetted.html"]

[[holiday]]
country = "IN"
id = "dussehra"
name = "Dussehra (Vijaya Dashami)"
rule = "listed"
dates_2026 = ["2026-10-20"]
dates_2027 = ["2027-10-09"]
dates_2028 = ["2028-09-27"]
tentative_2027 = false
tentative_2028 = true
eve = false
greeting = "Happy Dussehra"
greeting_en = "Happy Dussehra"
look = "fireworks_night"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-closed-holidays.html", "https://www.staffnews.in/2026/07/list-of-holidays-for-the-year-2027-gazetted.html", "https://www.drikpanchang.com/calendars/indian/indiancalendar.html?year=2028", "https://www.calendarlabs.com/holidays/india/2028"]

[[holiday]]
country = "IN"
id = "diwali"
name = "Diwali (Deepavali)"
rule = "listed"
dates_2026 = ["2026-11-08"]
dates_2027 = ["2027-10-29"]
dates_2028 = ["2028-10-17"]
tentative_2027 = false
tentative_2028 = true
eve = true
greeting = "Happy Diwali"
greeting_en = "Happy Diwali"
look = "diwali"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-closed-holidays.html", "https://www.staffnews.in/2026/07/list-of-holidays-for-the-year-2027-gazetted.html", "https://www.drikpanchang.com/calendars/indian/indiancalendar.html?year=2028", "https://www.calendarlabs.com/holidays/india/2028"]

[[holiday]]
country = "IN"
id = "guru_nanak_jayanti"
name = "Guru Nanak's Birthday (Gurpurab)"
rule = "listed"
dates_2026 = ["2026-11-24"]
dates_2027 = ["2027-11-14"]
dates_2028 = ["2028-11-02"]
tentative_2027 = false
tentative_2028 = true
eve = false
greeting = "Happy Gurpurab"
greeting_en = "Happy Guru Nanak Jayanti"
look = "candles"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-closed-holidays.html", "https://www.staffnews.in/2026/07/list-of-holidays-for-the-year-2027-gazetted.html", "https://www.drikpanchang.com/calendars/indian/indiancalendar.html?year=2028", "https://www.calendarlabs.com/holidays/india/2028"]

[[holiday]]
country = "IN"
id = "christmas"
name = "Christmas Day"
rule = "fixed(12,25)"
dates_2026 = ["2026-12-25"]
dates_2027 = ["2027-12-25"]
dates_2028 = ["2028-12-25"]
tentative_2027 = false
tentative_2028 = false
eve = true
greeting = "Merry Christmas"
greeting_en = "Merry Christmas"
look = "candles"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-closed-holidays.html", "https://www.staffnews.in/2026/07/list-of-holidays-for-the-year-2027-gazetted.html"]
```

Ramadan / Eid as India observes them:

| | 2026 (1447, actual) | 2027 (1448, estimate) | 2028 (1449, estimate) |
|---|---|---|---|
| Ramadan starts (first fast) | 2026-02-19 (crescent seen in Delhi on 18 Feb) | ~2027-02-09 (02-08 if seen on the 7th) | ~2028-01-29 (01-28 if seen on the 27th; Delhi q −0.17) |
| `ramadan` look window | 2026-02-19 .. 2026-03-20 | ~2027-02-09 .. 2027-03-09 | ~2028-01-29 .. 2028-02-26 |
| Eid al-Fitr | 2026-03-21 (Kerala: 03-20) | 2027-03-10 (DoP&T list) | ~2028-02-27 (02-28 if not seen on 26 Feb; Delhi q +0.17) |
| Eid al-Adha | 2026-05-28 (J&K: 05-27) | 2027-05-17 (DoP&T list) | ~2028-05-06 (Delhi q −0.18 on 25 Apr) |

Notes (IN):
- Scope: the 17 Delhi gazetted holidays (14 compulsory nationwide plus Delhi's 3 picks: Holi, Ram Navami, Janmashtami), plus Ambedkar Jayanti, plus 4 national-scale restricted holidays marked "(restricted holiday)". An employee may take any 2 restricted holidays; they are not general days off.
- Ambedkar Jayanti: added by a separate DoP&T O.M. (F.No.12/4/2020-JCA, 09.04.2026) for all central offices nationwide. The same was done in 2025. It is not in the 2027 list, hence tentative.
- 2026 changes: Bakrid moved 27 → 28 May by O.M. of 22.05.2026 (crescent not seen). I found no DoP&T change for Muharram (26 Jun) or Milad (26 Aug). Andhra Pradesh and Goa moved their Milad from 25 to 26 Aug, which matches the Delhi list. Eid al-Fitr: Jama Masjid and the Lucknow Markazi Chand Committee announced Sat 21 Mar. Kerala celebrated Fri 20 Mar.
- No substitute days: O.M. para 3.2 forbids them when a holiday falls on a weekend. 2027 has 5 weekend falls: 15 Aug (Independence Day + Milad, both Sunday), 2 Oct (Sat), 9 Oct (Sat), 14 Nov (Sun), 25 Dec (Sat). The app must handle two holidays on 2027-08-15.
- Eve: Holi's eve is Holika Dahan (2026-03-03, 2027-03-22). Diwali's eve is Choti Diwali / Naraka Chaturdashi (2027-10-28; same day as Diwali in 2026). The main Lakshmi Puja night is the night of the listed date. O.M. para 6 lets states keep Naraka Chaturdashi as their Diwali holiday.
- Islamic dates in the 2027 list are "subject to change depending on the sighting of the moon" (para 5.1). The Hindu, Sikh and Jain dates are fixed by the O.M. The Department of Posts clarified on 08.09.2026 that they cannot be changed.
- Regional only (excluded): Pongal, Onam, Bihu, Vishu/Vaisakhi, Ugadi/Gudi Padwa, Chhath, Durga Puja days (WB), Rath Yatra, Karwa Chauth, Lohri, Basant Panchami, Shivaji Jayanti, state formation days. Most of these are Delhi restricted holidays or state-gazetted days.
- Disagreement: in 2026 Delhi's Dussehra (20 Oct) fell on the same day as Maha Navami; Bengal's Bijoya Dashami was 21 Oct (see BD).
- 2028: Hindu, Sikh and Jain dates are drikpanchang (New Delhi), all matched by calendarlabs except Ram Navami. Good Friday and Ambedkar Jayanti both fall on Fri 2028-04-14, so the app must handle two holidays on one date again. Weekend falls: Makar Sankranti (Sat), Eid al-Fitr (Sun), Holi (Sat), Bakrid (Sat), Muharram (Sun), Raksha Bandhan (Sat), Janmashtami (Sun).
- 2028 disagreements:
  - Ram Navami: drikpanchang Smarta 3 Apr vs ISKCON 4 Apr; calendarlabs 4 Apr. I chose 3 Apr because the DoP&T took the Smarta date in 2026 (26 Mar).
  - Bakrid: drikpanchang and calendarlabs 5 May vs 6 May here. India has observed Bakrid a day after Saudi Arabia every year from 2022 to 2026, and the 25 Apr crescent is below India's past sighting level.
  - Muharram: drikpanchang and calendarlabs 3 Jun vs 4 Jun here. 1 Muharram is 26 May in South Asia whether Zil Hajj starts on 26 or 27 Apr; 3 Jun follows the Saudi 25 May.
  - Milad: drikpanchang and calendarlabs 3 Aug vs 4 Aug here. The 22 Jul crescent is not visible in South Asia (q ≈ −0.8).
  - Eid al-Fitr: drikpanchang and calendarlabs 27 Feb; the habibur.com Bangladesh table has 28 Feb. Delhi's 26 Feb crescent (q +0.17) is borderline.

## Pakistan (PK)

```toml
[[holiday]]
country = "PK"
id = "kashmir_day"
name = "Kashmir Solidarity Day (Youm-e-Yakjehti-e-Kashmir)"
rule = "fixed(2,5)"
dates_2026 = ["2026-02-05"]
dates_2027 = ["2027-02-05"]
dates_2028 = ["2028-02-05"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Youm-e-Yakjehti-e-Kashmir"
greeting_en = "Kashmir Solidarity Day"
look = "none"
source = ["https://en.dailypakistan.com.pk/19-Jan-2026/public-holidays-in-pakistan-2026-revealed-check-notification-here", "https://www.officeholidays.com/countries/pakistan/2027"]

[[holiday]]
country = "PK"
id = "eid_al_fitr"
name = "Eid-ul-Fitr (1-3 Shawwal)"
rule = "listed"
dates_2026 = ["2026-03-21", "2026-03-22", "2026-03-23"]
dates_2027 = ["2027-03-10", "2027-03-11", "2027-03-12"]
dates_2028 = ["2028-02-27", "2028-02-28", "2028-02-29"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Eid Mubarak"
greeting_en = "Happy Eid al-Fitr {era} AH"
look = "eid"
source = ["https://www.app.com.pk/?p=1180682", "https://www.zameen.com/news/eid-ul-fitr-2026-holidays-confirmed.html", "https://www.officeholidays.com/countries/pakistan/2027", "https://hijri.habibur.com/1449/", "https://www.drikpanchang.com/calendars/indian/indiancalendar.html?year=2028"]

[[holiday]]
country = "PK"
id = "pakistan_day"
name = "Pakistan Day (Youm-e-Pakistan)"
rule = "fixed(3,23)"
dates_2026 = ["2026-03-23"]
dates_2027 = ["2027-03-23"]
dates_2028 = ["2028-03-23"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Youm-e-Pakistan Mubarak"
greeting_en = "Happy Pakistan Day"
look = "flag"
source = ["https://en.dailypakistan.com.pk/19-Jan-2026/public-holidays-in-pakistan-2026-revealed-check-notification-here", "https://www.officeholidays.com/countries/pakistan/2027"]

[[holiday]]
country = "PK"
id = "labour_day"
name = "Labour Day (Youm-e-Mazdoor)"
rule = "fixed(5,1)"
dates_2026 = ["2026-05-01"]
dates_2027 = ["2027-05-01"]
dates_2028 = ["2028-05-01"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Youm-e-Mazdoor"
greeting_en = "Happy Labour Day"
look = "none"
source = ["https://en.dailypakistan.com.pk/19-Jan-2026/public-holidays-in-pakistan-2026-revealed-check-notification-here", "https://www.officeholidays.com/countries/pakistan/2027"]

[[holiday]]
country = "PK"
id = "eid_al_adha"
name = "Eid-ul-Azha (10-12 Zil Hajj)"
rule = "listed"
dates_2026 = ["2026-05-27", "2026-05-28", "2026-05-29"]
dates_2027 = ["2027-05-17", "2027-05-18", "2027-05-19"]
dates_2028 = ["2028-05-06", "2028-05-07", "2028-05-08"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Eid ul Adha Mubarak"
greeting_en = "Happy Eid al-Adha {era} AH"
look = "eid"
source = ["https://thestandard.com.pk/no-holiday-on-third-day-of-eidul-azha-2026/", "https://en.dailypakistan.com.pk/19-Jan-2026/public-holidays-in-pakistan-2026-revealed-check-notification-here", "https://www.officeholidays.com/countries/pakistan/2027", "https://hijri.habibur.com/bd/1449/"]

[[holiday]]
country = "PK"
id = "youm_e_takbeer"
name = "Youm-e-Takbeer"
rule = "fixed(5,28)"
dates_2026 = ["2026-05-28"]
dates_2027 = ["2027-05-28"]
dates_2028 = ["2028-05-28"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Youm-e-Takbeer Mubarak"
greeting_en = "Youm-e-Takbeer"
look = "none"
source = ["https://en.dailypakistan.com.pk/19-Jan-2026/public-holidays-in-pakistan-2026-revealed-check-notification-here", "https://www.officeholidays.com/countries/pakistan/2027"]

[[holiday]]
country = "PK"
id = "ashura"
name = "Ashura (9th and 10th Muharram)"
rule = "listed"
dates_2026 = ["2026-06-25", "2026-06-26"]
dates_2027 = ["2027-06-15", "2027-06-16"]
dates_2028 = ["2028-06-03", "2028-06-04"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Youm-e-Ashur"
greeting_en = "Ashura"
look = "none"
source = ["https://en.dailypakistan.com.pk/22-Jun-2026/ashura-holidays-notification-june-2026", "https://www.officeholidays.com/countries/pakistan/2027", "https://hijri.habibur.com/1450/", "https://hijri.habibur.com/bd/1450/"]

[[holiday]]
country = "PK"
id = "independence_day"
name = "Independence Day (Youm-e-Azadi)"
rule = "fixed(8,14)"
dates_2026 = ["2026-08-14"]
dates_2027 = ["2027-08-14"]
dates_2028 = ["2028-08-14"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Jashn-e-Azadi Mubarak"
greeting_en = "Happy {nth since 1947} Independence Day, Pakistan"
look = "flag"
source = ["https://en.dailypakistan.com.pk/19-Jan-2026/public-holidays-in-pakistan-2026-revealed-check-notification-here", "https://www.officeholidays.com/countries/pakistan/2027"]

[[holiday]]
country = "PK"
id = "mawlid"
name = "Eid Milad-un-Nabi (12 Rabi-ul-Awwal)"
rule = "listed"
dates_2026 = ["2026-08-26"]
dates_2027 = ["2027-08-15"]
dates_2028 = ["2028-08-04"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Eid Milad-un-Nabi Mubarak"
greeting_en = "Mawlid of the Prophet"
look = "none"
source = ["https://pakistan.shafaqna.com/EN/334051", "https://www.officeholidays.com/countries/pakistan/2027", "https://hijri.habibur.com/bd/1450/"]

[[holiday]]
country = "PK"
id = "iqbal_day"
name = "Allama Iqbal Day (Youm-e-Iqbal)"
rule = "fixed(11,9)"
dates_2026 = ["2026-11-09"]
dates_2027 = ["2027-11-09"]
dates_2028 = ["2028-11-09"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Youm-e-Iqbal"
greeting_en = "Iqbal Day"
look = "none"
source = ["https://en.dailypakistan.com.pk/19-Jan-2026/public-holidays-in-pakistan-2026-revealed-check-notification-here", "https://www.officeholidays.com/countries/pakistan/2027"]

[[holiday]]
country = "PK"
id = "quaid_e_azam_day"
name = "Quaid-e-Azam Day / Christmas"
rule = "fixed(12,25)"
dates_2026 = ["2026-12-25"]
dates_2027 = ["2027-12-25"]
dates_2028 = ["2028-12-25"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Happy Quaid-e-Azam Day"
greeting_en = "Happy Quaid-e-Azam Day"
look = "none"
source = ["https://en.dailypakistan.com.pk/19-Jan-2026/public-holidays-in-pakistan-2026-revealed-check-notification-here", "https://www.officeholidays.com/countries/pakistan/2027"]
```

Ramadan / Eid as Pakistan observes them:

| | 2026 (1447, actual) | 2027 (1448, estimate) | 2028 (1449, estimate) |
|---|---|---|---|
| Ramadan starts (first fast) | 2026-02-19 (crescent seen on 18 Feb) | ~2027-02-09 (02-08 possible) | ~2028-01-29 (01-28 possible; Karachi q −0.12 on 27 Jan) |
| `ramadan` look window | 2026-02-19 .. 2026-03-20 (30 fasts) | ~2027-02-09 .. 2027-03-09 | ~2028-01-29 .. 2028-02-26 (29 fasts) |
| Eid al-Fitr | 2026-03-21 (Shawwal moon not seen on 19 Mar) | ~2027-03-10 | ~2028-02-27 (Karachi q +0.20 on 26 Feb; 27 Feb also if Ramadan starts on the 28th) |
| Eid al-Adha | 2026-05-27 (Zil Hajj moon seen on 17 May) | ~2027-05-17 | ~2028-05-06 (Karachi q −0.17 on 25 Apr) |

Notes (PK):
- The 2026 circular (19.01.2026) was later amended by notifications. Eid-ul-Fitr: days off were notified for Fri 20 + Sat 21 Mar; Sun 22 is the weekend and Mon 23 is Pakistan Day, so Eid day 3 and Pakistan Day coincided. Eid-ul-Azha: days off 26–28 May (the circular had 27–29). 29 May, Eid day 3, was not a day off. Ashura: 25–26 Jun (circular 24–25). Milad: 26 Aug (circular 25).
- No substitute-day system. In 2026 Youm-e-Takbeer (28 May) fell inside the Eid days.
- 2027 is estimated from conjunctions (8 Mar 09:29, 6 May 10:58, 4 Jun 19:40, 2 Aug 10:05 UTC) and agrees with India's and Malaysia's official 2027 lists. officeholidays.com instead has Eid 9 Mar, Eid-ul-Azha 16 May and Ashura 14–15 Jun, which look like Saudi-style dates. Pakistan matched Saudi Arabia for Eid-ul-Azha in 2026, so a one-day-earlier outcome is possible. In 2027, Independence Day (14 Aug) and Milad (15 Aug) fall on consecutive days.
- Excluded: 26 Dec (Christians only); minority optional holidays (Holi, Diwali, Easter, Guru Nanak's birthday, etc.); bank-only holidays (1 Jan, 1st Ramadan, 1 Jul). 10 May "Youm-e-Marka-e-Haq" is an annual observance, not a holiday.
- Regional only (excluded): Sindh (Urs of Shah Abdul Latif Bhitai, Z.A. Bhutto 4 Apr, Benazir Bhutto 27 Dec, Shab-e-Meraj, Holi/Diwali), AJK Founding Day 24 Oct, Gilgit-Baltistan Independence Day 1 Nov.
- Look: Milad is celebrated with lighting and green flags. None of the menu looks fits exactly, so it is set to `none`. Independence night (13–14 Aug) has midnight fireworks, hence `eve = true`.
- 2028: Eid-ul-Fitr days are 27–29 Feb (29 Feb is the leap day), clear of Pakistan Day. 1 Muharram is 26 May whichever day Zil Hajj starts, so Ashura is 3–4 Jun. Milad is 4 Aug, ten days before Independence Day.
- 2028 disagreements:
  - Ramadan start: the global calendars (habibur.com, time.now) have 28 Jan vs 29 Jan here. Karachi's 27 Jan crescent (q −0.12) is below the level Pakistan has sighted at. Pakistan started Ramadan a day after Saudi Arabia in 2024 and 2025.
  - Eid-ul-Azha: drikpanchang, calendarlabs, habibur.com (global) and time.now have 5 May (Saudi-style) vs 6 May here. The 25 Apr crescent is q −0.17 at Karachi and −0.13 at Quetta. In 2026 Pakistan matched Saudi because its 17 May crescent was visible (q +0.13).
  - Ashura and Milad: the Saudi-style sites have 10 Muharram on 3 Jun and Milad on 3 Aug, one day earlier.

## Bangladesh (BD)

```toml
[[holiday]]
country = "BD"
id = "shab_e_barat"
name = "Shab-e-Barat"
rule = "listed"
dates_2026 = ["2026-02-04"]
dates_2027 = ["2027-01-24"]
dates_2028 = ["2028-01-13"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Pobitro Shab-e-Barat"
greeting_en = "Shab-e-Barat"
look = "candles"
source = ["https://www.thedailystar.net/news/bangladesh/news/govt-announces-official-list-public-holidays-2026-4031596", "https://www.officeholidays.com/countries/bangladesh/2026", "https://www.bangladatetoday.com/blog/bangladesh-government-holidays-2027", "https://www.bangladatetoday.com/holidays-bangladesh/2028", "https://hijri.habibur.com/bd/1449/"]

[[holiday]]
country = "BD"
id = "language_martyrs_day"
name = "Shaheed Dibosh and International Mother Language Day (Ekushey February)"
rule = "fixed(2,21)"
dates_2026 = ["2026-02-21"]
dates_2027 = ["2027-02-21"]
dates_2028 = ["2028-02-21"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Amar Ekushey"
greeting_en = "Language Martyrs' Day"
look = "none"
source = ["https://www.thedailystar.net/news/bangladesh/news/govt-announces-official-list-public-holidays-2026-4031596", "https://www.officeholidays.com/countries/bangladesh/2027"]

[[holiday]]
country = "BD"
id = "shab_e_qadr"
name = "Shab-e-Qadr"
rule = "listed"
dates_2026 = ["2026-03-17"]
dates_2027 = ["2027-03-07"]
dates_2028 = ["2028-02-24"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Pobitro Shab-e-Qadr"
greeting_en = "Shab-e-Qadr"
look = "none"
source = ["https://www.thedailystar.net/news/bangladesh/news/govt-announces-official-list-public-holidays-2026-4031596", "https://www.officeholidays.com/countries/bangladesh/2026", "https://www.bangladatetoday.com/blog/bangladesh-government-holidays-2027", "https://www.bangladatetoday.com/holidays-bangladesh/2028", "https://hijri.habibur.com/bd/1449/"]

[[holiday]]
country = "BD"
id = "jumatul_bida"
name = "Jumatul Bida (last Friday of Ramadan)"
rule = "listed"
dates_2026 = ["2026-03-20"]
dates_2027 = ["2027-03-05"]
dates_2028 = ["2028-02-25"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Jumatul Bida"
greeting_en = "Jumatul Bida"
look = "none"
source = ["https://www.thedailystar.net/news/bangladesh/news/govt-announces-official-list-public-holidays-2026-4031596", "https://www.bangladatetoday.com/blog/bangladesh-government-holidays-2027", "https://www.bangladatetoday.com/holidays-bangladesh/2028", "https://www.drikpanchang.com/calendars/indian/indiancalendar.html?year=2028"]

[[holiday]]
country = "BD"
id = "eid_al_fitr"
name = "Eid-ul-Fitr"
rule = "listed"
dates_2026 = ["2026-03-21", "2026-03-22", "2026-03-23"]
dates_2027 = ["2027-03-10", "2027-03-11", "2027-03-12"]
dates_2028 = ["2028-02-27", "2028-02-28", "2028-02-29"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Eid Mubarak"
greeting_en = "Happy Eid al-Fitr {era} AH"
look = "eid"
source = ["https://bdnews24.com/bangladesh/8618af4cfd38", "https://www.officeholidays.com/countries/bangladesh/2026", "https://www.bangladatetoday.com/blog/bangladesh-government-holidays-2027", "https://www.bangladatetoday.com/holidays-bangladesh/2028", "https://hijri.habibur.com/1449/"]

[[holiday]]
country = "BD"
id = "independence_day"
name = "Independence and National Day (Shadhinota Dibosh)"
rule = "fixed(3,26)"
dates_2026 = ["2026-03-26"]
dates_2027 = ["2027-03-26"]
dates_2028 = ["2028-03-26"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Shubho Shadhinota Dibosh"
greeting_en = "Happy Independence Day"
look = "flag"
source = ["https://www.thedailystar.net/news/bangladesh/news/govt-announces-official-list-public-holidays-2026-4031596", "https://www.officeholidays.com/countries/bangladesh/2027"]

[[holiday]]
country = "BD"
id = "pohela_boishakh"
name = "Bangla New Year (Pohela Boishakh)"
rule = "fixed(4,14)"
dates_2026 = ["2026-04-14"]
dates_2027 = ["2027-04-14"]
dates_2028 = ["2028-04-14"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Shubho Noboborsho"
greeting_en = "Happy Bengali New Year"
look = "none"
source = ["https://www.thedailystar.net/news/bangladesh/news/govt-announces-official-list-public-holidays-2026-4031596", "https://www.officeholidays.com/countries/bangladesh/2027"]

[[holiday]]
country = "BD"
id = "labour_day"
name = "May Day (Mohan May Dibosh)"
rule = "fixed(5,1)"
dates_2026 = ["2026-05-01"]
dates_2027 = ["2027-05-01"]
dates_2028 = ["2028-05-01"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "May Dibosh"
greeting_en = "Happy May Day"
look = "none"
source = ["https://www.thedailystar.net/news/bangladesh/news/govt-announces-official-list-public-holidays-2026-4031596", "https://www.officeholidays.com/countries/bangladesh/2027"]

[[holiday]]
country = "BD"
id = "vesak"
name = "Buddha Purnima (Boishakhi Purnima)"
rule = "listed"
dates_2026 = ["2026-05-01"]
dates_2027 = ["2027-05-20"]
dates_2028 = ["2028-05-08"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Shubho Buddha Purnima"
greeting_en = "Happy Vesak"
look = "candles"
source = ["https://www.thedailystar.net/news/bangladesh/news/govt-announces-official-list-public-holidays-2026-4031596", "https://www.officeholidays.com/countries/bangladesh/2027", "https://www.bangladatetoday.com/blog/bangladesh-government-holidays-2027", "https://www.bangladatetoday.com/holidays-bangladesh/2028", "https://www.drikpanchang.com/calendars/indian/indiancalendar.html?year=2028"]

[[holiday]]
country = "BD"
id = "eid_al_adha"
name = "Eid-ul-Azha"
rule = "listed"
dates_2026 = ["2026-05-28", "2026-05-29", "2026-05-30"]
dates_2027 = ["2027-05-17", "2027-05-18", "2027-05-19"]
dates_2028 = ["2028-05-06", "2028-05-07", "2028-05-08"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Eid Mubarak"
greeting_en = "Happy Eid al-Adha {era} AH"
look = "eid"
source = ["https://bdnews24.com/bangladesh/a11a17f999fc", "https://www.officeholidays.com/countries/bangladesh/2026", "https://www.bangladatetoday.com/blog/bangladesh-government-holidays-2027", "https://www.bangladatetoday.com/holidays-bangladesh/2028", "https://hijri.habibur.com/bd/1449/"]

[[holiday]]
country = "BD"
id = "ashura"
name = "Ashura (10 Muharram)"
rule = "listed"
dates_2026 = ["2026-06-26"]
dates_2027 = ["2027-06-16"]
dates_2028 = ["2028-06-04"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Pobitro Ashura"
greeting_en = "Ashura"
look = "none"
source = ["https://www.thedailystar.net/news/bangladesh/news/govt-announces-official-list-public-holidays-2026-4031596", "https://www.officeholidays.com/countries/bangladesh/2026", "https://www.bangladatetoday.com/blog/bangladesh-government-holidays-2027", "https://www.bangladatetoday.com/holidays-bangladesh/2028", "https://hijri.habibur.com/bd/1450/"]

[[holiday]]
country = "BD"
id = "july_uprising_day"
name = "July Mass Uprising Day (July Gono-obhyutthan Dibosh)"
rule = "fixed(8,5)"
dates_2026 = ["2026-08-05"]
dates_2027 = ["2027-08-05"]
dates_2028 = ["2028-08-05"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "July Gono-obhyutthan Dibosh"
greeting_en = "July Mass Uprising Day"
look = "none"
source = ["https://www.thedailystar.net/news/bangladesh/news/govt-announces-official-list-public-holidays-2026-4031596", "https://www.officeholidays.com/countries/bangladesh/2027"]

[[holiday]]
country = "BD"
id = "mawlid"
name = "Eid-e-Miladunnabi"
rule = "listed"
dates_2026 = ["2026-08-26"]
dates_2027 = ["2027-08-15"]
dates_2028 = ["2028-08-04"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Pobitro Eid-e-Miladunnabi"
greeting_en = "Mawlid of the Prophet"
look = "none"
source = ["https://www.dhakatribune.com/bangladesh/417407/eid-e-miladunnabi-on-aug-26", "https://www.officeholidays.com/countries/bangladesh/2027", "https://www.bangladatetoday.com/holidays-bangladesh/2028", "https://hijri.habibur.com/bd/1450/"]

[[holiday]]
country = "BD"
id = "janmashtami"
name = "Janmashtami"
rule = "listed"
dates_2026 = ["2026-09-04"]
dates_2027 = ["2027-08-25"]
dates_2028 = ["2028-08-13"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Shubho Janmashtami"
greeting_en = "Happy Janmashtami"
look = "none"
source = ["https://www.thedailystar.net/news/bangladesh/news/govt-announces-official-list-public-holidays-2026-4031596", "https://www.officeholidays.com/countries/bangladesh/2027", "https://www.bangladatetoday.com/holidays-bangladesh/2028", "https://www.drikpanchang.com/calendars/indian/indiancalendar.html?year=2028"]

[[holiday]]
country = "BD"
id = "durga_puja"
name = "Durga Puja (Mahanabami + Bijoya Dashami)"
rule = "listed"
dates_2026 = ["2026-10-20", "2026-10-21"]
dates_2027 = ["2027-10-08", "2027-10-09"]
dates_2028 = ["2028-09-27", "2028-09-28"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Shubho Sharodiya"
greeting_en = "Happy Durga Puja"
look = "none"
source = ["https://www.thedailystar.net/news/bangladesh/news/govt-announces-official-list-public-holidays-2026-4031596", "https://en.prothomalo.com/amp/story/bangladesh/government/mzmtj28ubh", "https://www.bangladatetoday.com/blog/bangladesh-government-holidays-2027", "https://www.bangladatetoday.com/holidays-bangladesh/2028", "https://vedpanchang.com/festivals/maha-navami-2028"]

[[holiday]]
country = "BD"
id = "solidarity_day"
name = "National Revolution and Solidarity Day (Jatiyo Biplob o Songhoti Dibosh)"
rule = "fixed(11,7)"
dates_2026 = ["2026-11-07"]
dates_2027 = ["2027-11-07"]
dates_2028 = ["2028-11-07"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Jatiyo Biplob o Songhoti Dibosh"
greeting_en = "National Revolution and Solidarity Day"
look = "none"
source = ["https://en.prothomalo.com/amp/story/bangladesh/government/mzmtj28ubh", "https://www.officeholidays.com/countries/bangladesh/2026", "https://www.officeholidays.com/countries/bangladesh/2027"]

[[holiday]]
country = "BD"
id = "victory_day"
name = "Victory Day (Bijoy Dibosh)"
rule = "fixed(12,16)"
dates_2026 = ["2026-12-16"]
dates_2027 = ["2027-12-16"]
dates_2028 = ["2028-12-16"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Shubho Bijoy Dibosh"
greeting_en = "Happy Victory Day"
look = "flag"
source = ["https://www.thedailystar.net/news/bangladesh/news/govt-announces-official-list-public-holidays-2026-4031596", "https://www.officeholidays.com/countries/bangladesh/2027"]

[[holiday]]
country = "BD"
id = "christmas"
name = "Christmas Day (Borodin)"
rule = "fixed(12,25)"
dates_2026 = ["2026-12-25"]
dates_2027 = ["2027-12-25"]
dates_2028 = ["2028-12-25"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Shubho Borodin"
greeting_en = "Merry Christmas"
look = "candles"
source = ["https://www.thedailystar.net/news/bangladesh/news/govt-announces-official-list-public-holidays-2026-4031596", "https://www.officeholidays.com/countries/bangladesh/2027"]
```

Ramadan / Eid as Bangladesh observes them:

| | 2026 (1447, actual) | 2027 (1448, estimate) | 2028 (1449, estimate) |
|---|---|---|---|
| Ramadan starts (first fast) | 2026-02-19 (crescent seen on 18 Feb) | ~2027-02-09 | ~2028-01-29 (Dhaka q −0.21 on 27 Jan) |
| `ramadan` look window | 2026-02-19 .. 2026-03-20 (30 fasts) | ~2027-02-09 .. 2027-03-09 | ~2028-01-29 .. 2028-02-26 (29 fasts) |
| Eid al-Fitr | 2026-03-21 (moon not seen on 19 Mar) | ~2027-03-10 | ~2028-02-27 (02-28 if not seen on 26 Feb; Dhaka q +0.10) |
| Eid al-Adha | 2026-05-28 (moon seen on 18 May; a day after Saudi) | ~2027-05-17 | ~2028-05-06 (Dhaka q −0.28 on 25 Apr) |

Notes (BD):
- The 2026 list was approved 06.11.2025: 28 days, 14 general + 14 by executive order. All of them are government days off, so both kinds are included. The weekend is Fri+Sat and there are no substitute days. In 2026, 9+ holidays fell on a Fri/Sat.
- Changes during 2026 under the new government:
  - Eid-ul-Fitr block extended to 7 days (17–23 Mar; 18 Mar added).
  - Eid-ul-Azha block 25–31 May (25 May added, offset by working Sat 23 May).
  - 22 Oct added by executive order after Bijoya Dashami (offset by working Sat 17 Oct).
  - 7 Nov reinstated as an annual general holiday (decided 16 Apr; confirmed by cabinet 05.10.2026).
  - One-off election holidays 10–12 Feb.
- Eid padding: the Eid day itself is the general holiday. The days before and after are executive-order "Eid holidays". `dates_*` list Eid days 1–3 only. Planned 2026 blocks: 19–20 + 22–23 Mar and 26–27 + 29–31 May, later extended as above.
- Shab-e-Barat and Shab-e-Qadr: the holy night is the evening before the holiday (Shab-e-Barat 2026 night = 3 Feb; Shab-e-Qadr 2026 night = 16 Mar). Shab-e-Barat is customarily marked with candles and lights.
- 2027 has no official list (expected Oct/Nov 2026). Islamic 2027 dates follow bangladatetoday.com plus astronomy; Bangladesh usually falls one day after Saudi Arabia. officeholidays.com disagrees: Eid 9 Mar, Eid-ul-Azha 16 May, Ashura 15 Jun, Shab-e-Qadr 6 Mar (Saudi-style). Durga Puja 2027 follows the Bengali panjika, which can differ by a day from Delhi: in 2026 Delhi's Dussehra was 20 Oct and BD's Bijoya Dashami was 21 Oct. officeholidays has Dashami 9 Oct, consistent with my estimate.
- Classification doubt: the Daily Star and officeholidays put Pohela Boishakh 2026 under executive order (it was historically a general holiday). The Daily Star text lists only 12 of its stated 14 general holidays.
- Excluded optional or community-only holidays: Shab-e-Meraj, Akheri Chahar Shamba, Saraswati Puja, Shivratri, Dol Purnima, Mahalaya, Lakshmi/Shyama Puja, Easter, Good Friday, Maghi/Ashari/Prabarana Purnima, Boisabi (Biju/Sangrai/Boisuk, Chittagong Hill Tracts, mid-Apr).
- Regional only (excluded): Boisabi in the three Hill Tracts districts. Every other holiday is nationwide.
- 2028: Islamic dates follow bangladatetoday.com 2028 and the hijri.habibur.com Bangladesh tables (Sha'ban 30 Dec 2027, Ramadan 29 Jan, Zil Hajj 27 Apr, Muharram 26 May, Rabi-ul-Awwal 24 Jul), which agree with astronomy. Eid-ul-Azha day 3 and Buddha Purnima are both 2028-05-08, so two holidays share a date. Shab-e-Barat 1450 falls just after the year: holy night 31 Dec 2028, holiday 1 Jan 2029. Durga Puja: Navami 27 Sep and Bijoya Dashami 28 Sep by the tithi at sunrise in Dhaka, a day after Delhi (Navami 26 Sep, Dussehra 27 Sep).
- 2028 disagreements:
  - Eid-ul-Fitr: bangladatetoday has 27 Feb; the habibur.com Bangladesh table has 28 Feb. Dhaka's 26 Feb crescent is borderline (q +0.10). Bangladesh missed q +0.002 in May 2026 but saw q +0.19 in Feb 2026. I kept 27 Feb; 28 Feb would move the block to 28 Feb–1 Mar and add a 30th fast on 27 Feb.
  - Durga Puja: publicholidays.in has Navami 26 Sep and Vijaya Dashami 28 Sep, which skips a day. vedpanchang.com has Navami 27 Sep for Bengal.
  - The Saudi-style sites (drikpanchang, calendarlabs) put Eid-ul-Azha, Ashura and Milad a day earlier (5 May, 3 Jun, 3 Aug).

## Malaysia (MY)

```toml
[[holiday]]
country = "MY"
id = "chinese_new_year"
name = "Chinese New Year (Tahun Baharu Cina)"
rule = "listed"
dates_2026 = ["2026-02-17", "2026-02-18"]
dates_2027 = ["2027-02-06", "2027-02-07"]
dates_2028 = ["2028-01-26", "2028-01-27"]
tentative_2027 = false
tentative_2028 = true
eve = true
greeting = "Gong Xi Fa Cai"
greeting_en = "Happy Chinese New Year · Year of the {animal}"
look = "lunar_new_year"
source = ["https://ecentral.my/cuti-umum-2026/", "https://www.officeholidays.com/countries/malaysia/2026", "https://www.humanresourcesonline.net/malaysia-s-2027-public-holidays-full-list-of-federal-and-state-holidays", "https://www.officeholidays.com/countries/malaysia/2027", "https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2028e.txt", "https://kalendarcuti.my/kalendar-cuti-umum-malaysia-2028/"]

[[holiday]]
country = "MY"
id = "eid_al_fitr"
name = "Hari Raya Aidilfitri (Hari Raya Puasa)"
rule = "listed"
dates_2026 = ["2026-03-21", "2026-03-22"]
dates_2027 = ["2027-03-10", "2027-03-11"]
dates_2028 = ["2028-02-27", "2028-02-28"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Selamat Hari Raya Aidilfitri"
greeting_en = "Happy Eid al-Fitr {era} AH"
look = "ketupat"
source = ["https://www.malaymail.com/news/malaysia/2026/03/15/anwar-announces-extra-public-holiday-for-hari-raya-either-march-20-or-23-depending-on-moon-sighting/212709", "https://www.officeholidays.com/countries/malaysia/2026", "https://www.humanresourcesonline.net/malaysia-s-2027-public-holidays-full-list-of-federal-and-state-holidays", "https://ecentral.my/cuti-2027/", "https://kalendarcuti.my/kalendar-cuti-umum-malaysia-2028/", "https://holidaydb.com/guides/public-holidays/malaysia/2028"]

[[holiday]]
country = "MY"
id = "labour_day"
name = "Labour Day (Hari Pekerja)"
rule = "fixed(5,1)"
dates_2026 = ["2026-05-01"]
dates_2027 = ["2027-05-01"]
dates_2028 = ["2028-05-01"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Selamat Hari Pekerja"
greeting_en = "Happy Labour Day"
look = "none"
source = ["https://ecentral.my/cuti-umum-2026/", "https://www.humanresourcesonline.net/malaysia-s-2027-public-holidays-full-list-of-federal-and-state-holidays"]

[[holiday]]
country = "MY"
id = "eid_al_adha"
name = "Hari Raya Haji / Aidiladha (Hari Raya Qurban)"
rule = "listed"
dates_2026 = ["2026-05-27"]
dates_2027 = ["2027-05-17"]
dates_2028 = ["2028-05-05"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Selamat Hari Raya Aidiladha"
greeting_en = "Happy Eid al-Adha {era} AH"
look = "eid"
source = ["https://www.bernama.com/en/news.php?id=2558314", "https://ecentral.my/cuti-umum-2026/", "https://www.humanresourcesonline.net/malaysia-s-2027-public-holidays-full-list-of-federal-and-state-holidays", "https://www.officeholidays.com/countries/malaysia/2027", "https://kalendarcuti.my/kalendar-cuti-umum-malaysia-2028/", "https://time.now/holidays/malaysia/?year=2028", "https://holidaydb.com/guides/public-holidays/malaysia/2028"]

[[holiday]]
country = "MY"
id = "vesak"
name = "Wesak Day (Hari Wesak)"
rule = "listed"
dates_2026 = ["2026-05-31"]
dates_2027 = ["2027-05-20"]
dates_2028 = ["2028-05-09"]
tentative_2027 = false
tentative_2028 = true
eve = false
greeting = "Selamat Hari Wesak"
greeting_en = "Happy Vesak"
look = "candles"
source = ["https://ecentral.my/cuti-umum-2026/", "https://www.officeholidays.com/countries/malaysia/2026", "https://www.humanresourcesonline.net/malaysia-s-2027-public-holidays-full-list-of-federal-and-state-holidays", "https://www.officeholidays.com/countries/malaysia/2027", "https://kalendarcuti.my/kalendar-cuti-umum-malaysia-2028/", "https://time.now/holidays/malaysia/?year=2028", "https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2028e.txt"]

[[holiday]]
country = "MY"
id = "agong_birthday"
name = "Official Birthday of the Yang di-Pertuan Agong (Hari Keputeraan Agong)"
rule = "nth_weekday(6,Mon,1)"
dates_2026 = ["2026-06-01"]
dates_2027 = ["2027-06-07"]
dates_2028 = ["2028-06-05"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Daulat Tuanku"
greeting_en = "The King's Birthday"
look = "none"
source = ["https://ecentral.my/cuti-umum-2026/", "https://www.humanresourcesonline.net/malaysia-s-2027-public-holidays-full-list-of-federal-and-state-holidays", "https://www.officeholidays.com/countries/malaysia/2027"]

[[holiday]]
country = "MY"
id = "islamic_new_year"
name = "Awal Muharram (Maal Hijrah)"
rule = "listed"
dates_2026 = ["2026-06-17"]
dates_2027 = ["2027-06-06"]
dates_2028 = ["2028-05-26"]
tentative_2027 = false
tentative_2028 = true
eve = true
greeting = "Salam Maal Hijrah"
greeting_en = "Happy Islamic New Year {era} AH"
look = "none"
source = ["https://ecentral.my/cuti-umum-2026/", "https://www.officeholidays.com/countries/malaysia/2026", "https://www.humanresourcesonline.net/malaysia-s-2027-public-holidays-full-list-of-federal-and-state-holidays", "https://www.officeholidays.com/countries/malaysia/2027", "https://hijri.habibur.com/1450/"]

[[holiday]]
country = "MY"
id = "mawlid"
name = "Maulidur Rasul (Prophet Muhammad's Birthday)"
rule = "listed"
dates_2026 = ["2026-08-25"]
dates_2027 = ["2027-08-15"]
dates_2028 = ["2028-08-04"]
tentative_2027 = false
tentative_2028 = true
eve = true
greeting = "Salam Maulidur Rasul"
greeting_en = "Mawlid of the Prophet"
look = "none"
source = ["https://www.malaymail.com/amp/news/malaysia/2026/08/25/thousands-join-maulidur-rasul-nationwide-anwar-leads-upm-procession-under-madani-dihayati-theme/232584", "https://ecentral.my/cuti-umum-2026/", "https://www.humanresourcesonline.net/malaysia-s-2027-public-holidays-full-list-of-federal-and-state-holidays", "https://www.officeholidays.com/countries/malaysia/2027", "https://hijri.habibur.com/bd/1450/"]

[[holiday]]
country = "MY"
id = "national_day"
name = "National Day (Hari Kebangsaan / Hari Merdeka)"
rule = "fixed(8,31)"
dates_2026 = ["2026-08-31"]
dates_2027 = ["2027-08-31"]
dates_2028 = ["2028-08-31"]
tentative_2027 = false
tentative_2028 = false
eve = true
greeting = "Selamat Hari Kebangsaan"
greeting_en = "Happy {nth since 1957} National Day, Malaysia"
look = "flag"
source = ["https://ecentral.my/cuti-umum-2026/", "https://www.humanresourcesonline.net/malaysia-s-2027-public-holidays-full-list-of-federal-and-state-holidays"]

[[holiday]]
country = "MY"
id = "malaysia_day"
name = "Malaysia Day (Hari Malaysia)"
rule = "fixed(9,16)"
dates_2026 = ["2026-09-16"]
dates_2027 = ["2027-09-16"]
dates_2028 = ["2028-09-16"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Selamat Hari Malaysia"
greeting_en = "Happy {nth since 1963} Malaysia Day"
look = "flag"
source = ["https://ecentral.my/cuti-umum-2026/", "https://www.humanresourcesonline.net/malaysia-s-2027-public-holidays-full-list-of-federal-and-state-holidays"]

[[holiday]]
country = "MY"
id = "diwali"
name = "Deepavali (Hari Deepavali)"
rule = "listed"
dates_2026 = ["2026-11-08"]
dates_2027 = ["2027-10-28"]
dates_2028 = ["2028-10-17"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Selamat Hari Deepavali"
greeting_en = "Happy Diwali"
look = "diwali"
source = ["https://ecentral.my/cuti-umum-2026/", "https://www.officeholidays.com/countries/malaysia/2026", "https://www.humanresourcesonline.net/malaysia-s-2027-public-holidays-full-list-of-federal-and-state-holidays", "https://ecentral.my/cuti-2027/", "https://kalendarcuti.my/kalendar-cuti-umum-malaysia-2028/", "https://www.drikpanchang.com/calendars/indian/indiancalendar.html?year=2028"]

[[holiday]]
country = "MY"
id = "christmas"
name = "Christmas Day (Hari Krismas)"
rule = "fixed(12,25)"
dates_2026 = ["2026-12-25"]
dates_2027 = ["2027-12-25"]
dates_2028 = ["2028-12-25"]
tentative_2027 = false
tentative_2028 = false
eve = true
greeting = "Selamat Hari Krismas"
greeting_en = "Merry Christmas"
look = "candles"
source = ["https://ecentral.my/cuti-umum-2026/", "https://www.humanresourcesonline.net/malaysia-s-2027-public-holidays-full-list-of-federal-and-state-holidays"]
```

Ramadan / Eid as Malaysia observes them:

| | 2026 (1447, actual) | 2027 (1448, official list, subject to change) | 2028 (1449, estimate) |
|---|---|---|---|
| Ramadan starts (first fast) | 2026-02-19 | 2027-02-08 (Awal Ramadan, Johor/Kedah state holiday) | 2028-01-28 (MABIMS met on 27 Jan: altitude 6.7°, elongation 9.3°) |
| `ramadan` look window | 2026-02-19 .. 2026-03-20 (30 fasts) | 2027-02-08 .. 2027-03-09 | 2028-01-28 .. 2028-02-26 (30 fasts) |
| Aidilfitri | 2026-03-21 (declared by the Keeper of the Rulers' Seal on 19 Mar) | 2027-03-10 | 2028-02-27 (moon below the horizon at sunset on 25 Feb) |
| Aidiladha | 2026-05-27 (1 Zulhijjah = 18 May) | 2027-05-17 | 2028-05-05 (MABIMS met on 25 Apr: 4.1°, 9.0°) |

Notes (MY):
- Scope: the BKPP federal schedule, 15 federal days in 2027: CNY ×2, Raya Puasa ×2, Labour, Raya Qurban, Wesak, Awal Muharram, Agong, Maulidur Rasul, National Day, Malaysia Day, Deepavali, Christmas, plus Raya Qurban day 2 listed for 4 states. Deepavali is federal but not a holiday in Sarawak; one ecentral 2026 row excludes more states, which looks like an error. Malaysia fixes Awal Muharram and Maulidur Rasul by its taqwim, so they are not marked tentative. Only Raya Puasa, Raya Qurban and Deepavali carry the "*subject to change" mark.
- 2026 extra and replacement days:
  - Raya Puasa: PM Anwar added an extra holiday (announced 15 Mar). Raya fell on Saturday, so the extra day was Fri 20 Mar, and Mon 23 Mar was the automatic replacement for Raya day 2 (Sunday), per Holidays Act 1951 s.3.
  - Wesak (Sun 31 May): replaced on Tue 2 Jun, because Mon 1 Jun was the Agong's birthday.
  - Deepavali (Sun 8 Nov): replaced on Mon 9 Nov.
  - States with a Fri/Sat weekend (Johor, Kedah, Kelantan, Terengganu) shift these differently.
- 2027 expected replacements (by the same rule, not checked against the official PDF): CNY day 2 (Sun 7 Feb) → Mon 8 Feb. Awal Muharram (Sun 6 Jun) → Tue 8 Jun, since Mon 7 Jun is the Agong's birthday. Maulidur Rasul (Sun 15 Aug) → Mon 16 Aug.
- Disagreements:
  - Deepavali 2027: the official list and ecentral say 28 Oct (Naraka Chaturdashi, the Malaysian Tamil custom); officeholidays says 29 Oct, which is North India's Diwali.
  - Wesak 2026: 31 May in Malaysia vs 1 May in India and Bangladesh. The Hindu calendar's adhik month shifted South Asia's date.
  - Maulidur Rasul 2026: 25 Aug in Malaysia vs 26 Aug in South Asia.
- National Day: the Ambang Merdeka countdown with fireworks on the night of 30 Aug, hence `eve = true`. Jalur Gemilang flags stay up through Malaysia Day.
- Regional only (excluded):
  - New Year's Day (all except Johor, Kedah, Kelantan, Perlis, Terengganu)
  - Thaipusam; Federal Territory Day; Israk & Mikraj; Awal Ramadan (Johor, Kedah); Nuzul Al-Quran (most states)
  - Good Friday (Sabah, Sarawak); Kaamatan (Sabah, Labuan); Gawai (Sarawak); Sarawak Day
  - Arafah day (Kelantan, Terengganu); Raya Haji day 2 (Kedah, Kelantan, Perlis, Terengganu); Raya Puasa day 3 (Melaka)
  - Rulers' and governors' birthdays; Penang/Melaka heritage days; Christmas Eve (Sabah/Sarawak)
- 2028: Islamic dates are from the MABIMS rule (altitude ≥ 3°, elongation ≥ 6.4° at sunset on the 29th), which reproduces every 2027 official date. Wesak is the 15th day of the 4th Chinese lunar month; that rule gives the official 2026 (31 May) and 2027 (20 May) dates. 2028's 4th month starts 25 Apr (HKO), so Wesak is 9 May, a day after South Asia's Buddha Purnima (8 May). Deepavali: Naraka Chaturdashi is the tithi at sunrise in Malaysia on 17 Oct, the same day as India's Diwali. Expected replacement day: Raya Puasa day 1 (Sun 27 Feb) → Tue 29 Feb, since Mon 28 Feb is Raya day 2.
- 2028 disagreements:
  - Awal Muharram: kalendarcuti.my, time.now, holidaydb.com and drikpanchang have 25 May vs 26 May here. On 24 May the moon sets before the sun in Malaysia (altitude −0.9°, elongation 4.1°), so 25 May would need the Saudi Umm al-Qura calendar, which Malaysia does not follow. hijri.habibur.com also has 26 May.
  - Maulidur Rasul: the same sites have 3 Aug vs 4 Aug here. The 22 Jul crescent fails MABIMS (altitude 2.5°, elongation 5.0°), so 1 Rabiulawal is 24 Jul.
  - Raya Puasa: time.now has 26–27 Feb (Saudi-style) vs 27–28 Feb in kalendarcuti.my and holidaydb.com.
  - time.now lists Deepavali on 14 Nov 2028 (for states only), which looks like a data error; kalendarcuti.my, drikpanchang and calendarlabs have 17 Oct.
