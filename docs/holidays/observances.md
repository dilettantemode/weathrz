# World holidays — observances and half-mast days (14 countries)

Compiled 23 Oct 2026 for weathrz. Observances are widely marked days that are **not** days off;
half-mast days are days the national flag is officially flown at half-mast every year. The days
off are in group-a.md (IN, PK, BD, MY), group-b.md (CN, JP, RU, ET), group-c.md (US, UK, BR, MX,
NG) and, for Indonesia, in `src/events/dates.rs` / `src/events/mod.rs`. No id here repeats one
of those.

**Schema conventions used below** (same as group C)

- `rule` is one string from the fixed grammar (`listed`, `fixed(M,D)`, `nth_weekday(M, mon, N)`,
  `last_weekday(M, mon)`, `easter(OFFSET)`, `orthodox_easter(OFFSET)`, `equinox(spring|autumn)`).
  A multi-day observance has one rule per date in an array, or `listed`.
- `dates_*` hold the day itself. `tentative_2027` / `tentative_2028 = true` means the date itself
  may still move (lunar or lunisolar days, or a date set by a yearly notice not yet out).
- Every entry has `observance = true`.
- `look = "half_mast"` is the new look for half-mast days. When a half-mast day is also an
  existing holiday or observance, it is a separate entry with id `<holiday_id>_half_mast`, so the
  holiday keeps its own greeting and look.
- `greeting_en` is what the app shows. Half-mast greetings are never "Happy …".
- The UK uses country code `UK` like group-c.md (the generator maps it to `GB`).

**Primary sources checked**

| Country | Official source | Cross-check |
|---|---|---|
| ID | [UU 24/2009](https://id.wikisource.org/wiki/Undang-Undang_Republik_Indonesia_Nomor_24_Tahun_2009) Pasal 7, 12 + Penjelasan; [Keppres 316/1959](https://id.wikisource.org/wiki/Pembaharuan_Keputusan_Presiden_Republik_Indonesia_Nomor_316_Tahun_1959); Keppres 108/1964, 153/1967, 78/1994, 28/2006, 33/2009, 22/2015 (JDIH/BPK, linked per entry); Kemenbud SE B/668/MK/TU.02.03/2026 (30 Sep) | detik, Katadata, Kompas TV, CNN Indonesia, Kontan (linked per entry) |
| IN | DoP&T restricted-holiday lists [2026](https://www.staffnews.in/2025/07/list-of-holidays-2026-restricted-holidays.html) / [2027](https://www.staffnews.in/2026/07/list-of-restricted-holidays-for-the-year-2027.html) (staffnews.in reproductions); Flag Code of India 2002 (MHA; 403 here) via [Patna HC note](https://patnahighcourt.gov.in/PDF/FLAGCODE/2.Salient%20Features%20of%20Flag%20Code%20of%20India-2002.PDF); MHA Martyrs Day instructions (state circulars) | [drikpanchang 2028](https://www.drikpanchang.com/calendars/hindu/hinducalendar.html?year=2028); group-a.md Diwali dates; PIB archive, DAE education page |
| CN | 全国年节及纪念日放假办法 2024 revision ([Xinhua](https://www.news.cn/politics/20241112/53da215f46c546a480f0a01d339dafa2/c.html)); 国旗法 2020 half-mast article (via [ifeng](https://news.ifeng.com/c/86UA0JEzrkK)); HKO conversion tables [2026](https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2026e.txt) / [2027](https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2027e.txt) / [2028](https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2028e.txt) | China Daily 2024, chinanews 2022 (13 Dec half-mast at the main venue) |
| US | [4 U.S.C. §7(m)](https://www.law.cornell.edu/uscode/text/4/7) (Memorial Day, Peace Officers); 36 U.S.C. [§109](https://www.law.cornell.edu/uscode/text/36/109), [§110](https://www.law.cornell.edu/uscode/text/36/110), [§117](https://www.law.cornell.edu/uscode/text/36/117), [§129](https://www.law.cornell.edu/uscode/text/36/129), [§136](https://www.law.cornell.edu/uscode/text/36/136), [§144](https://www.law.cornell.edu/uscode/text/36/144); [Pub. L. 107-51](https://www.govinfo.gov/link/plaw/107/public/51) | Federal Register (Proclamation 10970), state flag orders (IA, HI, NC), FOP 2021 White House note |
| PK | [Cabinet Division flag instructions, 31 Dec 2025](https://www.cabinet.gov.pk/SiteImage/Policy/Notification%20Flag%20Rules%2031-12-2025-N.pdf); Ruet-e-Hilal via [Radio Pakistan](https://www.radio.gov.pk/03-02-2026/shab-e-baraat-being-observed-with-full-religious-fervor-tonight) | Daily Pakistan, Aaj, ARY, Dawn, Wikipedia |
| NG | No federal flag-day or half-mast list found; Federal Ministry of Education Children's Day 2026 (via Gazette NG); Public Holidays Act as in group-c.md | TheCable, The Will, Blueprint, Guardian NG, NOUN News, Wikipedia |
| BR | Lei 5.700/1971 arts. 15, 18, 32 (via [Migalhas](https://www.migalhas.com.br/quentes/733/bandeira-nacional); [Senado entry](https://legis.senado.leg.br/norma/547573)); [Portaria MGI 11.460/2025](https://www.in.gov.br/web/dou/-/portaria-mgi-n-11.460-de-29-de-dezembro-de-2025-678388627) | Exame, Infoescola, São Paulo câmara opinion, officeholidays.com, Wikipedia (pt) |
| BD | Flag Rules 1972 via [Banglapedia](https://en.banglapedia.org/index.php/National_Flag); Cultural Affairs Ministry 2026 Ekushey programme (via [Financial Express](https://thefinancialexpress.com.bd/national/cultural-ministry-takes-elaborate-programs-on-shahid-dibash)); 15 Aug cancellation (via [Prothom Alo](https://en.prothomalo.com/bangladesh/w7ccjdfinl)) | BSS, UNB, BUP, KUET, Dhaka Tribune, Ittefaq, AIR |
| RU | [Decree No. 857 of 8 Jun 1996](https://zakonbase.ru/content/base/56243) (22 Jun half-mast); [Federal Law 32-FZ art. 1.1](https://legalacts.ru/doc/federalnyi-zakon-ot-13031995-n-32-fz-o/); Decree No. 1714/1994 (flag day); [Labour Code art. 112](https://www.consultant.ru/document/cons_doc_LAW_34683/98ef2900507766e70ff29c0b9d8e2353ea80a1cf/) | MChS regional pages, Rossiyskaya Gazeta, Rosstat regional releases, officeholidays.com |
| ET | [NBE](https://nbe.gov.et/nbe_news/national-flag-day-observed-at-the-national-bank-of-ethiopia/) and [ECMA](https://ecma.gov.et/2025/10/13) Flag Day 2025 notices; Flag Proclamation 654/2009 (PDF not decodable here) | Fana, AllAfrica (ENA/EPA reports), Wikipedia, FOTW, officeholidays.com, time.et |
| MX | Ley sobre el Escudo, la Bandera y el Himno Nacionales arts. 15, 18 (official [diputados.gob.mx PDF](https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf) refused connections; read on [mley.mx](https://mley.mx/LEBHN/articulo/18/)); DOF reform 21 Jul 2025 | [sdv.com.mx](https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/) (matches mley.mx); Gaceta UNAM, UNAM Global, Excélsior, El Financiero, officeholidays.com |
| JP | Cabinet decision of 13 Apr 1982 via [MHLW 2025 release](https://www.mhlw.go.jp/stf/newpage_61038.html) (15 Aug half-mast); [NAOJ 暦要項 2027](https://eco.mtk.nao.ac.jp/koyomi/yoko/2027/rekiyou272.html) (節分) | Shiga prefecture notice; HKO solar terms; [nippon.com](https://www.nippon.com/en/japan-glances/jg00139/), Just One Cookbook |
| MY | JPM/BKPP federal and state schedules (2026 via [officeholidays.com](https://www.officeholidays.com/countries/malaysia/2026); 2027 via [HR Online](https://www.humanresourcesonline.net/malaysia-s-2027-public-holidays-full-list-of-federal-and-state-holidays)); RTM and Bernama (Hari Pahlawan, Hari Guru) | drikpanchang, QPP Studio, HolidayAPI, calendarlabs (Thaipusam 2028), ecentral.my |
| UK | [DCMS: Designated days for Union flag flying](https://www.gov.uk/guidance/designated-days-for-union-flag-flying) (updated 28 Jan 2026); [gov.uk bank holidays](https://www.gov.uk/bank-holidays) | Council flag policies (Thurrock, Princes Risborough), Royal British Legion, Wikipedia, officeholidays.com |

Fetch problems are noted per country.

---

## Indonesia (ID)

12 observances, 1 half-mast day. Not repeated here (curated in `src/events/`): Hari Lahir
Pancasila 1 Jun, Hari Buruh 1 May, Kemerdekaan 17 Aug and the Agustusan month, and the religious days.

```toml
[[holiday]]
country = "ID"
id = "hari_kartini"
name = "Hari Kartini (Kartini Day)"
rule = "fixed(4,21)"
dates_2026 = ["2026-04-21"]
dates_2027 = ["2027-04-21"]
dates_2028 = ["2028-04-21"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Selamat Hari Kartini"
greeting_en = "Happy Kartini Day"
look = "none"
source = ["https://jdih.kemenkoinfra.go.id/en/keppres-no-108-tahun-1964", "https://www.detik.com/jabar/jabar-gaskeun/d-7875628/hari-kartini-21-april-2025-apakah-libur-cek-kepastiannya-di-sini"]

[[holiday]]
country = "ID"
id = "hardiknas"
name = "Hari Pendidikan Nasional (National Education Day)"
rule = "fixed(5,2)"
dates_2026 = ["2026-05-02"]
dates_2027 = ["2027-05-02"]
dates_2028 = ["2028-05-02"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Selamat Hari Pendidikan Nasional"
greeting_en = "National Education Day"
look = "flag"
source = ["https://id.wikisource.org/wiki/Undang-Undang_Republik_Indonesia_Nomor_24_Tahun_2009", "https://id.wikisource.org/wiki/Pembaharuan_Keputusan_Presiden_Republik_Indonesia_Nomor_316_Tahun_1959"]

[[holiday]]
country = "ID"
id = "harkitnas"
name = "Hari Kebangkitan Nasional (National Awakening Day)"
rule = "fixed(5,20)"
dates_2026 = ["2026-05-20"]
dates_2027 = ["2027-05-20"]
dates_2028 = ["2028-05-20"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Selamat Hari Kebangkitan Nasional"
greeting_en = "National Awakening Day"
look = "flag"
source = ["https://id.wikisource.org/wiki/Undang-Undang_Republik_Indonesia_Nomor_24_Tahun_2009", "https://id.wikisource.org/wiki/Pembaharuan_Keputusan_Presiden_Republik_Indonesia_Nomor_316_Tahun_1959"]

[[holiday]]
country = "ID"
id = "g30s_remembrance"
name = "Peringatan 30 September, bendera setengah tiang (30 September remembrance, flags at half-mast)"
rule = "fixed(9,30)"
dates_2026 = ["2026-09-30"]
dates_2027 = ["2027-09-30"]
dates_2028 = ["2028-09-30"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Mengenang 30 September"
greeting_en = "Remembering 30 September"
look = "half_mast"
source = ["https://katadata.co.id/lifestyle/varia/6abb6bb643928/susunan-upacara-hari-kesaktian-pancasila-2026-resmi-pedoman-kemenbud", "https://news.detik.com/berita/d-8133398/imbauan-pengibaran-bendera-dalam-rangka-hari-kesaktian-pancasila-2025", "https://id.wikisource.org/wiki/Undang-Undang_Republik_Indonesia_Nomor_24_Tahun_2009"]

[[holiday]]
country = "ID"
id = "hari_kesaktian_pancasila"
name = "Hari Kesaktian Pancasila (Pancasila Sanctity Day)"
rule = "fixed(10,1)"
dates_2026 = ["2026-10-01"]
dates_2027 = ["2027-10-01"]
dates_2028 = ["2028-10-01"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Selamat Hari Kesaktian Pancasila"
greeting_en = "Pancasila Sanctity Day"
look = "flag"
source = ["https://id.wikisource.org/wiki/Undang-Undang_Republik_Indonesia_Nomor_24_Tahun_2009", "https://katadata.co.id/lifestyle/varia/6abcd191a050f/tema-hari-kesaktian-pancasila-2026-dan-pedoman-upacaranya", "https://www.metrotvnews.com/read/bzGCR7E7-pemerintah-imbau-pengibaran-bendera-setengah-tiang-pada-30-september-ini-aturan-dan-tata-caranya"]

[[holiday]]
country = "ID"
id = "hari_batik"
name = "Hari Batik Nasional (National Batik Day)"
rule = "fixed(10,2)"
dates_2026 = ["2026-10-02"]
dates_2027 = ["2027-10-02"]
dates_2028 = ["2028-10-02"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Selamat Hari Batik Nasional"
greeting_en = "Happy National Batik Day"
look = "none"
source = ["https://peraturan.bpk.go.id/Details/55430/keppres-no-33-tahun-2009", "https://id.wikisource.org/wiki/Keputusan_Presiden_Republik_Indonesia_Nomor_33_Tahun_2009"]

[[holiday]]
country = "ID"
id = "hut_tni"
name = "Hari Ulang Tahun TNI (Armed Forces Day)"
rule = "fixed(10,5)"
dates_2026 = ["2026-10-05"]
dates_2027 = ["2027-10-05"]
dates_2028 = ["2028-10-05"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Dirgahayu TNI"
greeting_en = "Happy Armed Forces Day"
look = "none"
source = ["https://id.wikisource.org/wiki/Pembaharuan_Keputusan_Presiden_Republik_Indonesia_Nomor_316_Tahun_1959"]

[[holiday]]
country = "ID"
id = "hari_santri"
name = "Hari Santri Nasional (National Santri Day)"
rule = "fixed(10,22)"
dates_2026 = ["2026-10-22"]
dates_2027 = ["2027-10-22"]
dates_2028 = ["2028-10-22"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Selamat Hari Santri"
greeting_en = "Happy Santri Day"
look = "none"
source = ["https://nu.or.id/opini/hari-santri-milik-seluruh-eleman-bangsa-Q89c3", "https://www.kompas.tv/nasional/454204/hari-santri-2023-mahfud-md-harap-santri-jaga-keutuhan-nkri"]

[[holiday]]
country = "ID"
id = "sumpah_pemuda"
name = "Hari Sumpah Pemuda (Youth Pledge Day)"
rule = "fixed(10,28)"
dates_2026 = ["2026-10-28"]
dates_2027 = ["2027-10-28"]
dates_2028 = ["2028-10-28"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Selamat Hari Sumpah Pemuda"
greeting_en = "Youth Pledge Day"
look = "flag"
source = ["https://id.wikisource.org/wiki/Undang-Undang_Republik_Indonesia_Nomor_24_Tahun_2009", "https://id.wikisource.org/wiki/Pembaharuan_Keputusan_Presiden_Republik_Indonesia_Nomor_316_Tahun_1959", "https://www.metrotvnews.com/read/ba4CzpP6-hari-sumpah-pemuda-libur-atau-tidak"]

[[holiday]]
country = "ID"
id = "hari_pahlawan"
name = "Hari Pahlawan (Heroes' Day)"
rule = "fixed(11,10)"
dates_2026 = ["2026-11-10"]
dates_2027 = ["2027-11-10"]
dates_2028 = ["2028-11-10"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Selamat Hari Pahlawan"
greeting_en = "Heroes' Day"
look = "flag"
source = ["https://id.wikisource.org/wiki/Undang-Undang_Republik_Indonesia_Nomor_24_Tahun_2009", "https://id.wikisource.org/wiki/Pembaharuan_Keputusan_Presiden_Republik_Indonesia_Nomor_316_Tahun_1959", "https://cnnindonesia.com/edukasi/20251028102755-561-1289174/apakah-hari-pahlawan-libur-cek-ketentuannya-di-sini"]

[[holiday]]
country = "ID"
id = "hari_guru"
name = "Hari Guru Nasional (National Teachers' Day)"
rule = "fixed(11,25)"
dates_2026 = ["2026-11-25"]
dates_2027 = ["2027-11-25"]
dates_2028 = ["2028-11-25"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Selamat Hari Guru Nasional"
greeting_en = "Happy Teachers' Day"
look = "none"
source = ["https://id.wikisource.org/wiki/Keputusan_Presiden_Republik_Indonesia_Nomor_78_Tahun_1994"]

[[holiday]]
country = "ID"
id = "hari_bela_negara"
name = "Hari Bela Negara (National Defence Day)"
rule = "fixed(12,19)"
dates_2026 = ["2026-12-19"]
dates_2027 = ["2027-12-19"]
dates_2028 = ["2028-12-19"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Selamat Hari Bela Negara"
greeting_en = "National Defence Day"
look = "none"
source = ["https://setneg.go.id/baca/index/siaran_pers_kemhan_peringatan_hari_bela_negara_nasional_2016", "https://nasional.kontan.co.id/news/apakah-anda-sudah-tahu-ada-hari-bela-negara"]

[[holiday]]
country = "ID"
id = "hari_ibu"
name = "Hari Ibu (Mother's Day)"
rule = "fixed(12,22)"
dates_2026 = ["2026-12-22"]
dates_2027 = ["2027-12-22"]
dates_2028 = ["2028-12-22"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Selamat Hari Ibu"
greeting_en = "Happy Mother's Day"
look = "none"
source = ["https://id.wikisource.org/wiki/Pembaharuan_Keputusan_Presiden_Republik_Indonesia_Nomor_316_Tahun_1959", "https://kumparan.com/kabar-harian/apakah-hari-ibu-22-desember-libur-ini-penjelasan-resminya-26QAde0z1tR"]
```

Notes:

- **Which days fly the flag.** UU 24/2009 Pasal 7(3) makes the flag compulsory only on 17 August.
  Pasal 7(5) adds "hari-hari besar nasional", and its Penjelasan names, "antara lain", 2 May
  (Hardiknas), 20 May (Harkitnas), 1 October (Kesaktian Pancasila), 28 October (Sumpah Pemuda)
  and 10 November (Hari Pahlawan). Those five have `look = "flag"`. The others are national days
  by Keppres with ceremonies but no flag instruction in the law, so `look = "none"`.
- **Legal basis per day.** Keppres 316/1959 (16 Dec 1959, "hari-hari nasional yang bukan hari
  libur"): Hardiknas, Harkitnas, Hari Angkatan Perang 5 Oct (today's HUT TNI), Sumpah Pemuda,
  Hari Pahlawan, Hari Ibu. Wikisource's transcription of Keppres 316/1959 reads "8 Mei" for
  Hardiknas; the later Keppres 67/1961 and the UU 24/2009 Penjelasan give 2 May, which is what
  is observed. Kartini: Keppres 108/1964 (2 May 1964), national hero + 21 April. Kesaktian
  Pancasila: Keppres 153/1967. Batik: Keppres 33/2009 (17 Nov 2009). Bela Negara: Keppres
  28/2006. Santri: Keppres 22/2015. Guru: Keppres 78/1994 (24 Nov 1994). None is in the SKB
  3 Menteri, so none is a day off. The Keppres 28/2006 and 22/2015 texts were not opened; their
  numbers come from Setneg/Kemhan and NU Online.
- **30 September half-mast.** UU 24/2009 Pasal 12 lists half-mast only for deaths of officials;
  30 September is not in the law. It comes from a circular reissued every year (Kemendikbud
  until 2024, Kemenbud since: SE No. 8417/MK.L/TU.02.03/2025 for 2025 and
  SE No. B/668/MK/TU.02.03/2026 for 2026, both read through press reports; the kebudayaan.go.id
  text was not found). Window: all day 30 September (flags fly sunrise to sunset under Pasal
  7(1)); flags go back to full mast at 06:00 local time on 1 October. Treat it as standing
  practice since 1966, not statute.
- `g30s_remembrance` uses "30 September" rather than "G30S/PKI" in the greeting on purpose.
- `hut_tni` was the 12th pick over Hari Anak Nasional (23 Jul, Keppres 44/1984); see Summary.

## United States (US)

5 observances, 5 half-mast days. Easter Sunday is already in group-c.md; Halloween, Christmas Eve
and New Year's Eve are global events.

```toml
[[holiday]]
country = "US"
id = "groundhog_day"
name = "Groundhog Day"
rule = "fixed(2,2)"
dates_2026 = ["2026-02-02"]
dates_2027 = ["2027-02-02"]
dates_2028 = ["2028-02-02"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Happy Groundhog Day"
greeting_en = "Happy Groundhog Day"
look = "none"
source = ["https://www.ncei.noaa.gov/news/groundhog-day-forecasts-and-climate-history", "https://www.groundhog.org/"]

[[holiday]]
country = "US"
id = "st_patricks_day"
name = "Saint Patrick's Day"
rule = "fixed(3,17)"
dates_2026 = ["2026-03-17"]
dates_2027 = ["2027-03-17"]
dates_2028 = ["2028-03-17"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Happy St. Patrick's Day"
greeting_en = "Happy St. Patrick's Day"
look = "none"
source = ["https://census.gov/newsroom/facts-for-features/2025/irish-american-heritage.html", "https://www.census.gov/newsroom/facts-for-features/2017/cb17-ff05.html"]

[[holiday]]
country = "US"
id = "fallen_firefighters_memorial"
name = "National Fallen Firefighters Memorial Service (flags at half-staff on federal buildings)"
rule = "nth_weekday(5, sun, 1)"
dates_2026 = ["2026-05-03"]
dates_2027 = ["2027-05-02"]
dates_2028 = ["2028-05-07"]
tentative_2027 = true
tentative_2028 = true
observance = true
eve = false
greeting = "Fallen Firefighters Memorial · flags at half-staff"
greeting_en = "Fallen Firefighters Memorial"
look = "half_mast"
source = ["https://www.govinfo.gov/link/plaw/107/public/51", "https://governor.iowa.gov/press-release/2026-05-01/gov-reynolds-orders-flags-half-staff-national-fallen-firefighters-memorial-service", "https://governor.hawaii.gov/?p=11819"]

[[holiday]]
country = "US"
id = "mothers_day"
name = "Mother's Day"
rule = "nth_weekday(5, sun, 2)"
dates_2026 = ["2026-05-10"]
dates_2027 = ["2027-05-09"]
dates_2028 = ["2028-05-14"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Happy Mother's Day"
greeting_en = "Happy Mother's Day"
look = "none"
source = ["https://www.law.cornell.edu/uscode/text/36/117", "https://www.census.gov/newsroom/stories/mothers-day.html"]

[[holiday]]
country = "US"
id = "peace_officers_memorial_day"
name = "Peace Officers Memorial Day (flags at half-staff)"
rule = "listed"
dates_2026 = ["2026-05-15"]
dates_2027 = []
dates_2028 = ["2028-05-15"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Peace Officers Memorial Day · flags at half-staff"
greeting_en = "Peace Officers Memorial Day"
look = "half_mast"
source = ["https://www.law.cornell.edu/uscode/text/4/7", "https://www.law.cornell.edu/uscode/text/36/136", "https://fop.net/2021/05/message-from-the-white-house-on-national-peace-officers-memorial-day-and-armed-forces-day", "https://doa.nc.gov/documents/files/flag-alert-national-armed-forces-day-and-national-peace/open"]

[[holiday]]
country = "US"
id = "memorial_day_half_mast"
name = "Memorial Day (flags at half-staff until noon)"
rule = "last_weekday(5, mon)"
dates_2026 = ["2026-05-25"]
dates_2027 = ["2027-05-31"]
dates_2028 = ["2028-05-29"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Memorial Day · flags at half-staff until noon"
greeting_en = "Memorial Day · half-staff"
look = "half_mast"
source = ["https://www.law.cornell.edu/uscode/text/4/7", "https://www.law.cornell.edu/uscode/text/36/116"]

[[holiday]]
country = "US"
id = "flag_day"
name = "Flag Day"
rule = "fixed(6,14)"
dates_2026 = ["2026-06-14"]
dates_2027 = ["2027-06-14"]
dates_2028 = ["2028-06-14"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Happy Flag Day"
greeting_en = "Happy Flag Day"
look = "flag"
source = ["https://www.law.cornell.edu/uscode/text/36/110", "https://www.law.cornell.edu/uscode/text/4/6"]

[[holiday]]
country = "US"
id = "fathers_day"
name = "Father's Day"
rule = "nth_weekday(6, sun, 3)"
dates_2026 = ["2026-06-21"]
dates_2027 = ["2027-06-20"]
dates_2028 = ["2028-06-18"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Happy Father's Day"
greeting_en = "Happy Father's Day"
look = "none"
source = ["https://www.law.cornell.edu/uscode/text/36/109", "https://www.census.gov/newsroom/stories/fathers-day.html"]

[[holiday]]
country = "US"
id = "patriot_day"
name = "Patriot Day (flags at half-staff)"
rule = "fixed(9,11)"
dates_2026 = ["2026-09-11"]
dates_2027 = ["2027-09-11"]
dates_2028 = ["2028-09-11"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Remembering September 11 · flags at half-staff"
greeting_en = "Remembering September 11"
look = "half_mast"
source = ["https://www.law.cornell.edu/uscode/text/36/144", "https://www.federalregister.gov/documents/full_text/text/2025/09/16/2025-17966.txt"]

[[holiday]]
country = "US"
id = "pearl_harbor_remembrance_day"
name = "National Pearl Harbor Remembrance Day (flags at half-staff)"
rule = "fixed(12,7)"
dates_2026 = ["2026-12-07"]
dates_2027 = ["2027-12-07"]
dates_2028 = ["2028-12-07"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Pearl Harbor Remembrance Day · flags at half-staff"
greeting_en = "Pearl Harbor Remembrance"
look = "half_mast"
source = ["https://www.law.cornell.edu/uscode/text/36/129", "https://www.doa.nc.gov/news/press-releases/2025/12/05/us-and-nc-flags-half-staff-honor-national-pearl-harbor-remembrance-day"]
```

Notes:

- **Half-staff windows.** Memorial Day: sunrise to 12:00 only, then full staff (4 U.S.C. §7(m):
  "half-staff until noon only, then raised to the top of the staff"). The other four: all day,
  sunrise to sunset. Every half-staff day starts with the flag hoisted briefly to the peak.
- **Peace Officers Memorial Day (15 May).** 4 U.S.C. §7(m): half-staff "unless that day is also
  Armed Forces Day" (third Saturday in May). 15 May 2027 is a Saturday and the third Saturday,
  so it is Armed Forces Day and the flag stays at full staff; hence `rule = "listed"` with an
  empty `dates_2027`. In 2021, the last clash, the White House and North Carolina kept flags at
  full staff while some governors (WV, AZ, NE) lowered state flags anyway. 2026 (Fri) and 2028
  (Mon) are normal half-staff days. 36 U.S.C. §136 asks for the yearly proclamation.
- **Fallen Firefighters.** Pub. L. 107-51 (2001) lowers flags on all federal office buildings
  each year on the day of the National Fallen Firefighters Memorial Service in Emmitsburg, MD.
  The law names no date. The service was on the first Sunday of October through at least 2022 and
  has been on the first Sunday of May since 2024 (5 May 2024, 4 May 2025, 3 May 2026, per state
  flag orders). 2027 (2 May) and 2028 (7 May) are projections from that pattern, so tentative.
- **Patriot Day** (36 U.S.C. §144) and **Pearl Harbor** (36 U.S.C. §129) ask for a yearly
  proclamation calling for half-staff; 2025 Proclamation 10970 (Patriot Day) did so, and the
  2025 Pearl Harbor state orders cite the presidential proclamation.
- Mother's Day (§117), Father's Day (§109) and Flag Day (§110) are statutory observances;
  4 U.S.C. §6(d) lists Mother's Day and Flag Day among the days the flag should be displayed,
  but only Flag Day is a flag custom people actually follow, so only it gets `look = "flag"`.

## United Kingdom (UK)

8 observances, 0 half-mast days. Easter Sunday, Bonfire Night and Remembrance Sunday are already
in group-c.md. Country code `UK` as in group-c.md (the generator maps it to `GB`).

```toml
[[holiday]]
country = "UK"
id = "shrove_tuesday"
name = "Shrove Tuesday (Pancake Day)"
rule = "easter(-47)"
dates_2026 = ["2026-02-17"]
dates_2027 = ["2027-02-09"]
dates_2028 = ["2028-02-29"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Happy Pancake Day"
greeting_en = "Happy Pancake Day"
look = "none"
source = ["https://en.wikipedia.org/wiki/Shrove_Tuesday", "https://christchurchsb.org.uk/services/lent"]

[[holiday]]
country = "UK"
id = "st_davids_day"
name = "St David's Day (Dydd Gŵyl Dewi)"
rule = "fixed(3,1)"
dates_2026 = ["2026-03-01"]
dates_2027 = ["2027-03-01"]
dates_2028 = ["2028-03-01"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Dydd Gŵyl Dewi Hapus"
greeting_en = "Happy St David's Day"
look = "flag"
source = ["https://www.gov.uk/guidance/designated-days-for-union-flag-flying", "https://www.college-of-arms.gov.uk/resources/flag-flying-days-2026"]

[[holiday]]
country = "UK"
id = "mothering_sunday"
name = "Mothering Sunday"
rule = "easter(-21)"
dates_2026 = ["2026-03-15"]
dates_2027 = ["2027-03-07"]
dates_2028 = ["2028-03-26"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Happy Mothering Sunday"
greeting_en = "Happy Mother's Day"
look = "none"
source = ["https://en.wikipedia.org/wiki/Mothering_Sunday", "https://www.portsmouthcathedral.org.uk/portsmouth-cathedral-news/the-origins-and-history-of-mothering-sunday"]

[[holiday]]
country = "UK"
id = "st_patricks_day"
name = "St Patrick's Day"
rule = "fixed(3,17)"
dates_2026 = ["2026-03-17"]
dates_2027 = ["2027-03-17"]
dates_2028 = ["2028-03-17"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Happy St Patrick's Day"
greeting_en = "Happy St Patrick's Day"
look = "flag"
source = ["https://www.gov.uk/guidance/designated-days-for-union-flag-flying", "https://www.gov.uk/bank-holidays"]

[[holiday]]
country = "UK"
id = "st_georges_day"
name = "St George's Day"
rule = "fixed(4,23)"
dates_2026 = ["2026-04-23"]
dates_2027 = ["2027-04-23"]
dates_2028 = ["2028-04-23"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Happy St George's Day"
greeting_en = "Happy St George's Day"
look = "flag"
source = ["https://www.gov.uk/guidance/designated-days-for-union-flag-flying", "https://www.college-of-arms.gov.uk/resources/flag-flying-days-2026"]

[[holiday]]
country = "UK"
id = "fathers_day"
name = "Father's Day"
rule = "nth_weekday(6, sun, 3)"
dates_2026 = ["2026-06-21"]
dates_2027 = ["2027-06-20"]
dates_2028 = ["2028-06-18"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Happy Father's Day"
greeting_en = "Happy Father's Day"
look = "none"
source = ["https://www.officeholidays.com/holidays/united-kingdom/fathers-day"]

[[holiday]]
country = "UK"
id = "armistice_day"
name = "Armistice Day"
rule = "fixed(11,11)"
dates_2026 = ["2026-11-11"]
dates_2027 = ["2027-11-11"]
dates_2028 = ["2028-11-11"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Armistice Day · two-minute silence at 11:00"
greeting_en = "Armistice Day"
look = "none"
source = ["https://www.britishlegion.org.uk/get-involved/remembrance/about-remembrance/armistice-day"]

[[holiday]]
country = "UK"
id = "st_andrews_day"
name = "St Andrew's Day (Là Naomh Anndrais)"
rule = "fixed(11,30)"
dates_2026 = ["2026-11-30"]
dates_2027 = ["2027-11-30"]
dates_2028 = ["2028-11-30"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Happy St Andrew's Day"
greeting_en = "Happy St Andrew's Day"
look = "flag"
source = ["https://www.gov.uk/guidance/designated-days-for-union-flag-flying", "https://www.gov.uk/bank-holidays"]
```

Notes:

- **No annual half-mast day.** The DCMS guidance "Designated days for Union flag flying"
  (gov.uk, updated 28 Jan 2026) lists 13 designated days for 2026 and no yearly half-mast day;
  half-masting is announced case by case on a royal or national death (it was added and removed
  for the Duchess of Kent in Sep 2025). Remembrance Day (second Sunday in November, 8 Nov 2026)
  is a designated day at **full** mast; council flag policies (Thurrock, Princes Risborough)
  say so explicitly. So `remembrance_sunday` gets no `_half_mast` twin.
- The four patron saints' days are designated Union flag days (each in its own nation), so
  `look = "flag"`. St Patrick's Day is a bank holiday in Northern Ireland and St Andrew's Day in
  Scotland; for England & Wales, which group-c.md covers, they are observances. If the app ever
  splits the UK by nation, those two move to the day-off list there.
- Armistice Day (11 Nov) is the two-minute silence at 11:00 on the day itself, separate from
  Remembrance Sunday; both are kept because the 11th falls on a weekday most years.
- Mothering Sunday is the fourth Sunday of Lent (Easter − 21): 15 Mar 2026, 7 Mar 2027,
  26 Mar 2028. Shrove Tuesday is Easter − 47.
- The College of Arms 2026 flag-days page timed out from here; the gov.uk page was read
  directly.

## India (IN)

8 observances, 0 half-mast days. Days already in group-a.md (Makar Sankranti, Holi, Raksha
Bandhan, Dussehra, Diwali, etc.) are not repeated.

```toml
[[holiday]]
country = "IN"
id = "martyrs_day"
name = "Martyrs' Day (Shaheed Diwas; two minutes' silence at 11:00)"
rule = "fixed(1,30)"
dates_2026 = ["2026-01-30"]
dates_2027 = ["2027-01-30"]
dates_2028 = ["2028-01-30"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Shaheed Diwas"
greeting_en = "Martyrs' Day"
look = "none"
source = ["https://wcd.delhi.gov.in/sites/default/files/WCD/circulars-orders/regd._2_minute_silent.pdf", "https://www.meghalaya.gov.in/press/content/49979", "https://dip.goa.gov.in/state-to-observe-2-minute-silence-on-30th-january/"]

[[holiday]]
country = "IN"
id = "basant_panchami"
name = "Basant Panchami / Sri Panchami (restricted holiday)"
rule = "listed"
dates_2026 = ["2026-01-23"]
dates_2027 = ["2027-02-11"]
dates_2028 = ["2028-01-31"]
tentative_2027 = false
tentative_2028 = true
observance = true
eve = false
greeting = "Basant Panchami ki shubhkamnayen"
greeting_en = "Happy Basant Panchami"
look = "none"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-restricted-holidays.html", "https://www.staffnews.in/2026/07/list-of-restricted-holidays-for-the-year-2027.html", "https://www.drikpanchang.com/calendars/hindu/hinducalendar.html?year=2028"]

[[holiday]]
country = "IN"
id = "vaisakhi"
name = "Vaisakhi / Vishu / Meshadi (restricted holiday)"
rule = "listed"
dates_2026 = ["2026-04-14"]
dates_2027 = ["2027-04-14"]
dates_2028 = ["2028-04-13"]
tentative_2027 = false
tentative_2028 = true
observance = true
eve = false
greeting = "Vaisakhi diyan lakh lakh vadhaiyan"
greeting_en = "Happy Vaisakhi"
look = "none"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-restricted-holidays.html", "https://www.staffnews.in/2026/07/list-of-restricted-holidays-for-the-year-2027.html", "https://www.drikpanchang.com/sankranti/mesha-sankranti-date-time.html?year=2028"]

[[holiday]]
country = "IN"
id = "onam"
name = "Onam / Thiru Onam (restricted holiday)"
rule = "listed"
dates_2026 = ["2026-08-26"]
dates_2027 = ["2027-09-12"]
dates_2028 = ["2028-09-01"]
tentative_2027 = false
tentative_2028 = true
observance = true
eve = false
greeting = "Onashamsakal"
greeting_en = "Happy Onam"
look = "none"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-restricted-holidays.html", "https://www.staffnews.in/2026/07/list-of-restricted-holidays-for-the-year-2027.html", "https://www.drikpanchang.com/calendars/hindu/hinducalendar.html?year=2028"]

[[holiday]]
country = "IN"
id = "teachers_day"
name = "Teachers' Day (Shikshak Diwas)"
rule = "fixed(9,5)"
dates_2026 = ["2026-09-05"]
dates_2027 = ["2027-09-05"]
dates_2028 = ["2028-09-05"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Shikshak Diwas ki shubhkamnayen"
greeting_en = "Happy Teachers' Day"
look = "none"
source = ["https://archive.pib.gov.in/release02/lyr2003/rsep2003/04092003/r040920033.html", "https://dae.education.gov.in/important-days.html"]

[[holiday]]
country = "IN"
id = "karva_chauth"
name = "Karaka Chaturthi / Karva Chauth (restricted holiday)"
rule = "listed"
dates_2026 = ["2026-10-29"]
dates_2027 = ["2027-10-18"]
dates_2028 = ["2028-10-07"]
tentative_2027 = false
tentative_2028 = true
observance = true
eve = false
greeting = "Karva Chauth ki shubhkamnayen"
greeting_en = "Happy Karva Chauth"
look = "none"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-restricted-holidays.html", "https://www.staffnews.in/2026/07/list-of-restricted-holidays-for-the-year-2027.html", "https://www.drikpanchang.com/calendars/hindu/hinducalendar.html?year=2028"]

[[holiday]]
country = "IN"
id = "bhai_dooj"
name = "Bhai Duj / Bhai Dooj (restricted holiday)"
rule = "listed"
dates_2026 = ["2026-11-11"]
dates_2027 = ["2027-10-31"]
dates_2028 = ["2028-10-19"]
tentative_2027 = false
tentative_2028 = true
observance = true
eve = false
greeting = "Bhai Dooj ki shubhkamnayen"
greeting_en = "Happy Bhai Dooj"
look = "none"
source = ["https://www.staffnews.in/2025/07/list-of-holidays-2026-restricted-holidays.html", "https://www.staffnews.in/2026/07/list-of-restricted-holidays-for-the-year-2027.html", "https://www.drikpanchang.com/calendars/hindu/hinducalendar.html?year=2028"]

[[holiday]]
country = "IN"
id = "childrens_day"
name = "Children's Day (Bal Diwas)"
rule = "fixed(11,14)"
dates_2026 = ["2026-11-14"]
dates_2027 = ["2027-11-14"]
dates_2028 = ["2028-11-14"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Bal Diwas ki shubhkamnayen"
greeting_en = "Happy Children's Day"
look = "none"
source = ["https://dae.education.gov.in/important-days.html"]
```

Notes:

- **No annual half-mast day.** The Flag Code of India 2002 says the flag is flown at half-mast
  only on occasions the Government of India instructs (deaths of listed dignitaries, state
  mourning). If a half-masting falls on Republic Day, Independence Day, Gandhi Jayanti, National
  Week (6–13 April) or another day of national rejoicing, the flag stays at full mast except
  over the building where the body lies. Martyrs' Day (30 Jan) is marked by the MHA's yearly
  instruction for two minutes' silence at 11:00 (sirens 10:59–11:00 and 11:02–11:03), not by
  half-masting. The MHA PDF (mha.gov.in/sites/default/files/flagcodeofindia_070214.pdf)
  returned 403 here; the half-mast wording was taken from the Patna High Court "Salient
  features of Flag Code of India 2002" note and two secondary summaries, which agree.
- **Five of the eight are Central Government restricted holidays** (each employee picks two a
  year), so they are observances, not days off. 2026 and 2027 dates are from the DoP&T
  Annexure-II lists (O.M.s of 03.07.2025 and 16.07.2026, via staffnews.in). 2028 dates are
  drikpanchang (New Delhi) and tentative. Checks: Bhai Dooj is Diwali + 2 and Karva Chauth
  Diwali − 10/11 against group-a's Diwali (8 Nov 2026, 29 Oct 2027, 17 Oct 2028). The 2026
  Bhai Duj (11 Nov) is three days after Diwali because the pratipada falls on 10 Nov; DoP&T and
  drikpanchang agree.
- Vaisakhi is solar (Mesha Sankranti): 14 Apr in 2026–2027, 13 Apr in 2028 (sankranti 21:47 IST
  on 13 Apr). The same day is Vishu (Kerala) and Puthandu (Tamil Nadu), which are state holidays
  there.
- Considered and dropped (cap of 8): Chhath Puja, Lohri, Navratri (first day), Gudi Padwa/Ugadi,
  National Youth Day (12 Jan); see Summary.

## China (CN)

8 observances, 1 half-mast day (venue-only, see Notes). Mid-Autumn, Qingming, Dragon Boat and the
Spring Festival block (with Chuxi) are in group-b.md.

```toml
[[holiday]]
country = "CN"
id = "lantern_festival"
name = "元宵节 Yuanxiao Jie (Lantern Festival)"
rule = "listed"
dates_2026 = ["2026-03-03"]
dates_2027 = ["2027-02-20"]
dates_2028 = ["2028-02-09"]
tentative_2027 = true
tentative_2028 = true
observance = true
eve = false
greeting = "Yuanxiao jie kuaile"
greeting_en = "Happy Lantern Festival"
look = "lanterns"
source = ["https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2026e.txt", "https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2027e.txt", "https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2028e.txt"]

[[holiday]]
country = "CN"
id = "womens_day"
name = "妇女节 Funü Jie (International Women's Day; half day off for women)"
rule = "fixed(3,8)"
dates_2026 = ["2026-03-08"]
dates_2027 = ["2027-03-08"]
dates_2028 = ["2028-03-08"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Funü jie kuaile"
greeting_en = "Happy Women's Day"
look = "none"
source = ["https://www.news.cn/politics/20241112/53da215f46c546a480f0a01d339dafa2/c.html", "https://www.gov.cn/zhengce/content/202511/content_7047090.htm"]

[[holiday]]
country = "CN"
id = "youth_day"
name = "五四青年节 Wusi Qingnian Jie (Youth Day; half day off for 14+)"
rule = "fixed(5,4)"
dates_2026 = ["2026-05-04"]
dates_2027 = ["2027-05-04"]
dates_2028 = ["2028-05-04"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Wusi qingnian jie kuaile"
greeting_en = "Happy Youth Day"
look = "none"
source = ["https://www.news.cn/politics/20241112/53da215f46c546a480f0a01d339dafa2/c.html"]

[[holiday]]
country = "CN"
id = "childrens_day"
name = "六一儿童节 Liuyi Ertong Jie (Children's Day; day off for under-14s)"
rule = "fixed(6,1)"
dates_2026 = ["2026-06-01"]
dates_2027 = ["2027-06-01"]
dates_2028 = ["2028-06-01"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Liuyi ertong jie kuaile"
greeting_en = "Happy Children's Day"
look = "none"
source = ["https://www.news.cn/politics/20241112/53da215f46c546a480f0a01d339dafa2/c.html"]

[[holiday]]
country = "CN"
id = "qixi"
name = "七夕 Qixi (Double Seventh Festival)"
rule = "listed"
dates_2026 = ["2026-08-19"]
dates_2027 = ["2027-08-08"]
dates_2028 = ["2028-08-26"]
tentative_2027 = true
tentative_2028 = true
observance = true
eve = false
greeting = "Qixi kuaile"
greeting_en = "Happy Qixi"
look = "none"
source = ["https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2026e.txt", "https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2027e.txt", "https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2028e.txt"]

[[holiday]]
country = "CN"
id = "teachers_day"
name = "教师节 Jiaoshi Jie (Teachers' Day)"
rule = "fixed(9,10)"
dates_2026 = ["2026-09-10"]
dates_2027 = ["2027-09-10"]
dates_2028 = ["2028-09-10"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Jiaoshi jie kuaile"
greeting_en = "Happy Teachers' Day"
look = "none"
source = ["https://www.news.cn/politics/20241112/53da215f46c546a480f0a01d339dafa2/c.html"]

[[holiday]]
country = "CN"
id = "double_ninth"
name = "重阳节 Chongyang Jie (Double Ninth Festival)"
rule = "listed"
dates_2026 = ["2026-10-18"]
dates_2027 = ["2027-10-08"]
dates_2028 = ["2028-10-26"]
tentative_2027 = true
tentative_2028 = true
observance = true
eve = false
greeting = "Chongyang jie kuaile"
greeting_en = "Happy Double Ninth"
look = "none"
source = ["https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2026e.txt", "https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2027e.txt", "https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2028e.txt"]

[[holiday]]
country = "CN"
id = "national_memorial_day"
name = "南京大屠杀死难者国家公祭日 (National Memorial Day for the Nanjing Massacre Victims; half-mast at the ceremony)"
rule = "fixed(12,13)"
dates_2026 = ["2026-12-13"]
dates_2027 = ["2027-12-13"]
dates_2028 = ["2028-12-13"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Guojia gongji ri"
greeting_en = "National Memorial Day"
look = "candles"  # half-mast only at the Nanjing ceremony venue, so no lowered flag
source = ["https://cn.chinadaily.com.cn/a/202412/13/WS675b87aca310b59111da88e4.html", "https://news.ifeng.com/c/86UA0JEzrkK", "https://www.chinanews.com.cn/sh/2022/12-13/9913800.shtml"]

[[holiday]]
country = "CN"
id = "winter_solstice"
name = "冬至 Dongzhi (Winter Solstice)"
rule = "listed"
dates_2026 = ["2026-12-22"]
dates_2027 = ["2027-12-22"]
dates_2028 = ["2028-12-21"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Dongzhi kuaile"
greeting_en = "Happy Winter Solstice"
look = "none"
source = ["https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2026e.txt", "https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2027e.txt", "https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2028e.txt"]
```

Notes:

- **Partial days off.** 全国年节及纪念日放假办法 (as revised Nov 2024) art. 3 gives a half day
  off to women on 8 March, to young people aged 14+ on 4 May and (not listed here) to serving
  military on 1 August, and a full day to children under 14 on 1 June. Most people work, so these
  are observances. Art. 4 lists 教师节 (Teachers' Day, 10 Sep) among days that are not days off.
  Read through the Xinhua copy of the revision (the gov.cn State Council page is the 2026
  holiday notice already cited in group-b.md).
- **Lunar dates** are from the Hong Kong Observatory conversion tables (text files, fetched
  23 Oct 2026). They are marked tentative only because they are lunar, per this file's rule;
  HKO and the Purple Mountain Observatory calendar normally agree. 2028 has a leap 5th month,
  so Qixi (26 Aug) and Double Ninth (26 Oct) fall late that year. Winter solstice is a solar
  term (HKO), not a lunar day, so it is not tentative; the rule grammar has no solstice rule,
  so it is `listed`.
- **13 December half-mast is venue-only.** The 2020 Flag Law (国旗法) allows half-mast "in the
  whole country, in some regions or at specific places" when a national memorial ceremony
  (国家公祭仪式) is held, with the State Council deciding the scope. Under the State Council's
  standing approval, the flag is lowered every year at the **main venue** of the Nanjing
  ceremony (the Memorial Hall, flag raised and lowered to half-mast at 08:00, ceremony at
  10:00), per China Daily 2024 (read) and a 2025 Yangtse Evening Post report (search snippet
  only). It is not a nationwide
  half-mast day. The app may prefer `look = "candles"` or `"none"` for CN users outside Nanjing;
  kept as `half_mast` here because the instruction recurs every year by State Council approval.
  The day itself was set by the NPC Standing Committee decision of 27 Feb 2014.
- The full 2020 Flag Law text on npc.gov.cn was not reached; the half-mast article is quoted
  from Phoenix (ifeng) and partial copies on 12371.cn. yzwb.net failed to fetch.
- Considered and dropped: Laba (腊八), Army Day (1 Aug, half day only for the military),
  Martyrs' Day (30 Sep, no half-mast).

## Japan (JP)

5 observances, 1 half-mast day. The 16 national holidays (祝日) are in group-b.md; Children's Day
already carries the koinobori look.

```toml
[[holiday]]
country = "JP"
id = "setsubun"
name = "節分 Setsubun (bean-throwing day before Risshun)"
rule = "listed"
dates_2026 = ["2026-02-03"]
dates_2027 = ["2027-02-03"]
dates_2028 = ["2028-02-03"]
tentative_2027 = false
tentative_2028 = true
observance = true
eve = false
greeting = "Oni wa soto, fuku wa uchi"
greeting_en = "Happy Setsubun"
look = "none"
source = ["https://eco.mtk.nao.ac.jp/koyomi/yoko/2027/rekiyou272.html", "https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2028e.txt", "https://www.nippon.com/en/japan-glances/jg00139/"]

[[holiday]]
country = "JP"
id = "hinamatsuri"
name = "ひな祭り Hinamatsuri (Doll Festival / Girls' Day)"
rule = "fixed(3,3)"
dates_2026 = ["2026-03-03"]
dates_2027 = ["2027-03-03"]
dates_2028 = ["2028-03-03"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Hinamatsuri omedetō"
greeting_en = "Happy Hinamatsuri"
look = "none"
source = ["https://www.nippon.com/en/japan-glances/jg00139/", "https://www.justonecookbook.com/japanese-public-holidays-and-annual-events/"]

[[holiday]]
country = "JP"
id = "tanabata"
name = "七夕 Tanabata (Star Festival)"
rule = "fixed(7,7)"
dates_2026 = ["2026-07-07"]
dates_2027 = ["2027-07-07"]
dates_2028 = ["2028-07-07"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Tanabata"
greeting_en = "Happy Tanabata"
look = "none"
source = ["https://www.nippon.com/en/japan-glances/jg00139/", "https://www.justonecookbook.com/japanese-public-holidays-and-annual-events/"]

[[holiday]]
country = "JP"
id = "obon"
name = "お盆 Obon (August Bon)"
rule = ["fixed(8,13)", "fixed(8,14)", "fixed(8,15)", "fixed(8,16)"]
dates_2026 = ["2026-08-13", "2026-08-14", "2026-08-15", "2026-08-16"]
dates_2027 = ["2027-08-13", "2027-08-14", "2027-08-15", "2027-08-16"]
dates_2028 = ["2028-08-13", "2028-08-14", "2028-08-15", "2028-08-16"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Obon"
greeting_en = "Obon"
look = "lanterns"
source = ["https://www.nippon.com/en/japan-glances/jg00139/", "https://www.justonecookbook.com/japanese-public-holidays-and-annual-events/"]

[[holiday]]
country = "JP"
id = "war_dead_memorial_day"
name = "戦没者を追悼し平和を祈念する日 / 終戦の日 (Day for Mourning the War Dead and Praying for Peace; flags at half-mast)"
rule = "fixed(8,15)"
dates_2026 = ["2026-08-15"]
dates_2027 = ["2027-08-15"]
dates_2028 = ["2028-08-15"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Shūsen no Hi"
greeting_en = "Remembering the war dead"
look = "half_mast"
source = ["https://www.mhlw.go.jp/stf/newpage_61038.html", "https://www.pref.shiga.lg.jp/kensei/koho/e-shinbun/oshirase/345537.html"]

[[holiday]]
country = "JP"
id = "shichi_go_san"
name = "七五三 Shichi-Go-San"
rule = "fixed(11,15)"
dates_2026 = ["2026-11-15"]
dates_2027 = ["2027-11-15"]
dates_2028 = ["2028-11-15"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Shichi-Go-San omedetō"
greeting_en = "Happy Shichi-Go-San"
look = "none"
source = ["https://www.nippon.com/en/japan-glances/jg00139/", "https://www.justonecookbook.com/japanese-public-holidays-and-annual-events/"]
```

Notes:

- **15 August half-mast.** Under the Cabinet decision of 13 Apr 1982 (「戦没者を追悼し平和を祈念す
  る日」について), 15 August is the day for mourning the war dead, and the government holds the
  National Memorial Service for the War Dead each year. MHLW's yearly release (2025: 8 Aug 2025)
  says ministries, their branch offices and national facilities fly flags at half-mast that day
  and observe one minute of silence from 12:00; prefectures, municipalities, the Diet, the
  courts and private bodies are asked to do the same (Shiga prefecture's notice asks for
  half-mast at the prefectural and municipal offices). Window: the whole day at government
  buildings; the silence is 12:00–12:01 JST. Not a day off.
- No other yearly half-mast day: the 11 March earthquake memorial ceremony has not been held by
  the government since 2021, and 6/9 August (Hiroshima/Nagasaki) are city ceremonies.
- **Setsubun** is the day before Risshun. Risshun is 4 Feb in 2026–2028 (NAOJ 暦要項 2027 lists
  節分 2月3日 and 立春 2月4日 10:46; HKO lists "Spring Commences" on 4 Feb 2026 and 2028). NAOJ
  publishes the 2028 暦要項 in Feb 2027, so 2028 is tentative. The NAOJ 2027 page rendered as
  mojibake here; the dates were matched by solar longitude.
- **Obon** is not a holiday, but many firms close 13–16 Aug and it is the big travel week. Parts
  of Tokyo and some other areas keep Bon in July (13–16 Jul); the August dates are the national
  norm.
- No official source exists for Hinamatsuri, Tanabata, Obon or Shichi-Go-San (customs, not
  laws); nippon.com (Nippon Communications Foundation) and Just One Cookbook agree on the dates.
- Considered and dropped: Mother's Day, White Day, Valentine's Day (commercial), Ōmisoka (the
  global New Year event already covers 31 Dec evening).

## Russia (RU)

8 observances, 1 half-mast day. The Labour Code art. 112 days off (incl. Victory Day, Russia Day,
Unity Day, 8 March, 23 February) are in group-b.md.

```toml
[[holiday]]
country = "RU"
id = "students_day"
name = "Татьянин день / День российского студенчества (Tatiana Day, Russian Students' Day)"
rule = "fixed(1,25)"
dates_2026 = ["2026-01-25"]
dates_2027 = ["2027-01-25"]
dates_2028 = ["2028-01-25"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "S Tatyaninym dnyom"
greeting_en = "Happy Students' Day"
look = "none"
source = ["https://legalacts.ru/doc/federalnyi-zakon-ot-13031995-n-32-fz-o/", "https://www.officeholidays.com/countries/russia/2026"]

[[holiday]]
country = "RU"
id = "maslenitsa"
name = "Масленица (Maslenitsa, Pancake Week)"
rule = ["orthodox_easter(-55)", "orthodox_easter(-54)", "orthodox_easter(-53)", "orthodox_easter(-52)", "orthodox_easter(-51)", "orthodox_easter(-50)", "orthodox_easter(-49)"]
dates_2026 = ["2026-02-16", "2026-02-17", "2026-02-18", "2026-02-19", "2026-02-20", "2026-02-21", "2026-02-22"]
dates_2027 = ["2027-03-08", "2027-03-09", "2027-03-10", "2027-03-11", "2027-03-12", "2027-03-13", "2027-03-14"]
dates_2028 = ["2028-02-21", "2028-02-22", "2028-02-23", "2028-02-24", "2028-02-25", "2028-02-26", "2028-02-27"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "S Maslenitsey"
greeting_en = "Happy Maslenitsa"
look = "none"
source = ["https://znanierussia.ru/articles/%D0%9C%D0%B0%D1%81%D0%BB%D0%B5%D0%BD%D0%B8%D1%86%D0%B0", "https://www.officeholidays.com/countries/russia/2026"]

[[holiday]]
country = "RU"
id = "cosmonautics_day"
name = "День космонавтики (Cosmonautics Day)"
rule = "fixed(4,12)"
dates_2026 = ["2026-04-12"]
dates_2027 = ["2027-04-12"]
dates_2028 = ["2028-04-12"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "S Dnyom kosmonavtiki"
greeting_en = "Happy Cosmonautics Day"
look = "none"
source = ["https://legalacts.ru/doc/federalnyi-zakon-ot-13031995-n-32-fz-o/", "https://www.officeholidays.com/countries/russia/2026"]

[[holiday]]
country = "RU"
id = "orthodox_easter"
name = "Пасха (Orthodox Easter; not a day off)"
rule = "orthodox_easter(0)"
dates_2026 = ["2026-04-12"]
dates_2027 = ["2027-05-02"]
dates_2028 = ["2028-04-16"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Khristos voskrese!"
greeting_en = "Happy Orthodox Easter"
look = "easter_eggs"
source = ["https://www.consultant.ru/document/cons_doc_LAW_34683/98ef2900507766e70ff29c0b9d8e2353ea80a1cf/", "https://www.officeholidays.com/countries/russia/2026"]

[[holiday]]
country = "RU"
id = "day_of_memory_and_sorrow"
name = "День памяти и скорби (Day of Memory and Sorrow; flags at half-mast)"
rule = "fixed(6,22)"
dates_2026 = ["2026-06-22"]
dates_2027 = ["2027-06-22"]
dates_2028 = ["2028-06-22"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Den pamyati i skorbi"
greeting_en = "Day of Memory and Sorrow"
look = "half_mast"
source = ["https://zakonbase.ru/content/base/56243", "https://legalacts.ru/doc/federalnyi-zakon-ot-13031995-n-32-fz-o/", "https://rg.ru/2026/06/22/kak-pomianut-pogibshih-v-den-pamiati-i-skorbi-22-iiunia.html"]

[[holiday]]
country = "RU"
id = "state_flag_day"
name = "День Государственного флага (Day of the State Flag)"
rule = "fixed(8,22)"
dates_2026 = ["2026-08-22"]
dates_2027 = ["2027-08-22"]
dates_2028 = ["2028-08-22"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "S Dnyom Gosudarstvennogo flaga"
greeting_en = "Happy Flag Day"
look = "flag"
source = ["https://zakonbase.ru/content/base/3889", "https://profile.ru/news/society/kakoj-segodnya-prazdnik-22-avgusta-den-gosudarstvennogo-flaga-rossijskoj-federacii-2-1574147/amp/"]

[[holiday]]
country = "RU"
id = "day_of_knowledge"
name = "День знаний (Day of Knowledge, first day of school)"
rule = "fixed(9,1)"
dates_2026 = ["2026-09-01"]
dates_2027 = ["2027-09-01"]
dates_2028 = ["2028-09-01"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "S Dnyom znaniy"
greeting_en = "Happy Day of Knowledge"
look = "none"
source = ["https://03.rosstat.gov.ru/storage/document/document_publication_plan/2019-11/26/press_20190829.pdf", "https://znanierussia.ru/articles/%D0%94%D0%B5%D0%BD%D1%8C_%D0%B7%D0%BD%D0%B0%D0%BD%D0%B8%D0%B9"]

[[holiday]]
country = "RU"
id = "teachers_day"
name = "День учителя (Teachers' Day)"
rule = "fixed(10,5)"
dates_2026 = ["2026-10-05"]
dates_2027 = ["2027-10-05"]
dates_2028 = ["2028-10-05"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "S Dnyom uchitelya"
greeting_en = "Happy Teachers' Day"
look = "none"
source = ["https://26.rosstat.gov.ru/storage/mediabank/8XFcyQ6g/%E2%84%9685%20%D0%94%D0%B5%D0%BD%D1%8C%20%D1%83%D1%87%D0%B8%D1%82%D0%B5%D0%BB%D1%8F.pdf", "https://aif.ru/dontknows/1236575"]

[[holiday]]
country = "RU"
id = "constitution_day"
name = "День Конституции (Constitution Day; not a day off since 2005)"
rule = "fixed(12,12)"
dates_2026 = ["2026-12-12"]
dates_2027 = ["2027-12-12"]
dates_2028 = ["2028-12-12"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "S Dnyom Konstitutsii"
greeting_en = "Constitution Day"
look = "none"
source = ["https://legalacts.ru/doc/federalnyi-zakon-ot-13031995-n-32-fz-o/", "https://www.consultant.ru/document/cons_doc_LAW_34683/98ef2900507766e70ff29c0b9d8e2353ea80a1cf/"]
```

Notes:

- **22 June half-mast.** Presidential Decree No. 857 of 8 Jun 1996 "О Дне памяти и скорби",
  point 1: on the Day of Memory and Sorrow "на территории всей страны приспускаются
  Государственные флаги Российской Федерации" and entertainment is cancelled on TV, radio and
  in cultural institutions "в течение всего дня". Window: all day. 22 June is also a памятная
  дата under Federal Law 32-FZ art. 1.1. The kremlin.ru copy failed to fetch; the text was read
  on zakonbase.ru, and MChS regional pages and Rossiyskaya Gazeta (22 Jun 2026) confirm the
  practice. No other yearly half-mast day was found (3 Sep, Day of Solidarity against
  Terrorism, has no flag rule).
- **Maslenitsa** is the week before Orthodox Lent: Monday to Forgiveness Sunday, Orthodox Easter
  − 55 to − 49 (16–22 Feb 2026, 8–14 Mar 2027, 21–27 Feb 2028). It has no legal status; Orthodox
  Easter itself is not in the Labour Code art. 112 list, so it is an observance too.
- Students' Day (25 Jan), Cosmonautics Day (12 Apr) and Constitution Day (12 Dec) are памятные
  даты in 32-FZ art. 1.1 (legalacts.ru copy, as amended 29 Dec 2025). Constitution Day stopped
  being a day off in 2005. No flag custom beyond the everyday flag on state buildings was
  found, so `look = "none"`.
- Day of the State Flag: Decree No. 1714 of 20 Aug 1994 (in force; not a day off).
  Day of Knowledge: Decree of the USSR Supreme Soviet Presidium No. 373-XI of 15 Jun 1984.
  Teachers' Day: Decree No. 1961 of 3 Oct 1994. These three were confirmed through Rosstat
  regional press releases and secondary sites (rosstat.gov.ru did not answer from here).
- Considered and dropped: Old New Year (14 Jan, informal), Radonitsa (regional day off in some
  regions), Day of Solidarity against Terrorism (3 Sep).

## Pakistan (PK)

4 observances, 0 half-mast days. Days off (Kashmir Day, Pakistan Day, Youm-e-Takbeer, Iqbal Day,
Quaid-e-Azam Day, the Islamic days) and Ramadan are in group-a.md and extras.md.

```toml
[[holiday]]
country = "PK"
id = "shab_e_barat"
name = "Shab-e-Barat (15 Sha'ban; night of 14/15 Sha'ban)"
rule = "listed"
dates_2026 = ["2026-02-04"]
dates_2027 = ["2027-01-24"]
dates_2028 = ["2028-01-13"]
tentative_2027 = true
tentative_2028 = true
observance = true
eve = true
greeting = "Shab-e-Barat Mubarak"
greeting_en = "Shab-e-Barat"
look = "candles"
source = ["https://www.radio.gov.pk/03-02-2026/shab-e-baraat-being-observed-with-full-religious-fervor-tonight", "https://en.dailypakistan.com.pk/30-Jan-2026/shab-e-barat-holiday-notification-2026", "https://dawateislami.net/events/en/shab-e-barat"]

[[holiday]]
country = "PK"
id = "youm_e_marka_e_haq"
name = "Youm-e-Marka-e-Haq (Marka-e-Haq Day)"
rule = "fixed(5,10)"
dates_2026 = ["2026-05-10"]
dates_2027 = ["2027-05-10"]
dates_2028 = ["2028-05-10"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Youm-e-Marka-e-Haq"
greeting_en = "Marka-e-Haq Day"
look = "flag"
source = ["https://en.dailypakistan.com.pk/12-May-2025/pakistan-announces-observing-youm-e-marka-e-haq-every-year-on-may-10", "https://english.aaj.tv/news/amp/330416054", "https://www.etvbharat.com/en/international/pak-pm-announces-to-celebrate-may-10-as-marka-e-haq-day-every-year-enn26051100218"]

[[holiday]]
country = "PK"
id = "defence_day"
name = "Youm-e-Difa (Defence Day)"
rule = "fixed(9,6)"
dates_2026 = ["2026-09-06"]
dates_2027 = ["2027-09-06"]
dates_2028 = ["2028-09-06"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Youm-e-Difa Mubarak"
greeting_en = "Happy Defence Day"
look = "flag"
source = ["https://en.wikipedia.org/wiki/Defence_Day", "https://pakobserver.net/celebrating-defence-day-alongside-marka-e-haq/"]

[[holiday]]
country = "PK"
id = "quaid_e_azam_death_anniversary"
name = "Death anniversary of Quaid-e-Azam Muhammad Ali Jinnah (Yaum-e-Wafat-e-Quaid)"
rule = "fixed(9,11)"
dates_2026 = ["2026-09-11"]
dates_2027 = ["2027-09-11"]
dates_2028 = ["2028-09-11"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Yaum-e-Wafat-e-Quaid"
greeting_en = "Remembering the Quaid"
look = "none"
source = ["https://arynews.tv/quaid-e-azam-death-anniversary-jinnah", "https://www.dawn.com/news/56716"]
```

Notes:

- **No standing half-mast day.** The Cabinet Division's updated "Instructions on the Design and
  Use of the Pakistan Flag" (No. 11-1/2021-Min-II, 31 Dec 2025, read from the cabinet.gov.pk PDF)
  list only the free-flying days (23 Mar, 14 Aug, 25 Dec, others as notified). Para 31: the flag
  "shall not be flown at half-mast without the orders of the Prime Minister" (or a Chief
  Minister with the PM's prior approval). ARY News claims the flag flies at half-mast every year
  on 21 Apr (Iqbal), 11 Sep (Quaid) and 16 Oct (Liaquat). Dawn reported half-mast on government
  buildings for the Quaid's anniversary in 2002, but no recent order was found, and the 2025
  instructions have no such list. **Sources disagree**; the official text wins, so 0 half-mast
  days. If the owner wants one, `quaid_e_azam_death_anniversary_half_mast` (11 Sep) is the
  candidate.
- **Shab-e-Barat** 2026: Sha'ban 1447 began 21 Jan (Ruet-e-Hilal), the holy night was Tue
  3 Feb and Punjab and Sindh closed schools on Wed 4 Feb (provincial, not federal). Dates follow
  group-a.md's Bangladesh convention (the day after the night, `eve = true`). 2027 and 2028
  copy group-a's South Asian estimates (24 Jan 2027, 13 Jan 2028), tentative.
- **Youm-e-Marka-e-Haq** (10 May) was announced by the PM in May 2025 as a yearly day for the
  May 2025 operation and was marked again in May 2026; group-a.md notes it is an observance,
  not a day off. No formal gazette notification was found.
- **Defence Day** (6 Sep) is a national day but not in the 2026 holiday circular (group-a.md).
- Considered and dropped: Chand Raat (covered by `eid_al_fitr` with `eve = true`), Shab-e-Miraj
  (Sindh holiday only), Youm-e-Istehsal (5 Aug) and Kashmir Black Day (27 Oct) as political
  protest days unsuited to a greeting, Iqbal's and Liaquat's death anniversaries (low
  recognition).

## Bangladesh (BD)

2 observances, 2 half-mast days (one of them, 14 Dec, is the observance itself). Days off,
Shab-e-Barat and Ramadan are in group-a.md and extras.md.

```toml
[[holiday]]
country = "BD"
id = "pohela_falgun"
name = "Pohela Falgun / Basanta Utsab (first day of spring)"
rule = "fixed(2,14)"
dates_2026 = ["2026-02-14"]
dates_2027 = ["2027-02-14"]
dates_2028 = ["2028-02-14"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Shubho Basanta"
greeting_en = "Happy Pohela Falgun"
look = "none"
source = ["https://en.ittefaq.com.bd/15328/pahela-falgun-brings-colour-music-as-spring", "https://www.dhakatribune.com/bangladesh/14350/pahela-falgun-celebrated-across-the-country"]

[[holiday]]
country = "BD"
id = "language_martyrs_day_half_mast"
name = "Shaheed Dibosh (Language Martyrs' Day; flags at half-mast)"
rule = "fixed(2,21)"
dates_2026 = ["2026-02-21"]
dates_2027 = ["2027-02-21"]
dates_2028 = ["2028-02-21"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Amar Ekushey"
greeting_en = "Ekushey · flags at half-mast"
look = "half_mast"
source = ["https://en.banglapedia.org/index.php/National_Flag", "https://thefinancialexpress.com.bd/national/cultural-ministry-takes-elaborate-programs-on-shahid-dibash", "https://bup.edu.bd/news/details/932"]

[[holiday]]
country = "BD"
id = "genocide_day"
name = "Gonohotya Dibosh (Genocide Day)"
rule = "fixed(3,25)"
dates_2026 = ["2026-03-25"]
dates_2027 = ["2027-03-25"]
dates_2028 = ["2028-03-25"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Gonohotya Dibosh"
greeting_en = "Genocide Day"
look = "none"
source = ["https://www.newsonair.gov.in/bangladesh-observes-genocide-day-to-remember-victims-of-1971-military-crackdown", "https://en.prothomalo.com/bangladesh/5vx1vft6nr"]

[[holiday]]
country = "BD"
id = "martyred_intellectuals_day"
name = "Shaheed Buddhijibi Dibosh (Martyred Intellectuals Day; flags at half-mast)"
rule = "fixed(12,14)"
dates_2026 = ["2026-12-14"]
dates_2027 = ["2027-12-14"]
dates_2028 = ["2028-12-14"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Shaheed Buddhijibi Dibosh"
greeting_en = "Martyred Intellectuals Day"
look = "half_mast"
source = ["https://en.prothomalo.com/bangladesh/kmknpq6avl", "https://www.bssnews.net/others/341900", "https://election.unb.com.bd/news/tag/7471"]
```

Notes:

- **Half-mast basis.** The People's Republic of Bangladesh Flag Rules, 1972 (as summarised by
  Banglapedia's "National Flag" entry): the flag is flown at half-mast on Shaheed Day
  (21 February) and on any other day the government notifies. The rules text itself was not
  found online.
  - **21 Feb** (`language_martyrs_day_half_mast`, twin of group-a's `language_martyrs_day`):
    the Cultural Affairs Ministry's 2026 programme put the flag at half-mast on government,
    semi-government, autonomous and private buildings and educational institutions; BUP
    confirms it on the day. Window: all day; observance begins at 00:01 with wreaths at the
    Central Shaheed Minar.
  - **14 Dec** (`martyred_intellectuals_day`): flown at half-mast nationwide every year by
    government programme (Prothom Alo, Dhaka Tribune; BSS district reports for 2025; KUET's
    2025 notice cites the Flag Rules 1972). Window: all day. Black flags are also raised.
- **15 August dropped.** National Mourning Day used to be a half-mast day. The interim
  government cancelled the holiday on 13 Aug 2024 (gazette, Ministry of Public Administration)
  and dropped it from the national days on 16 Oct 2024 (Dhaka Tribune, Prothom Alo). It was not
  restored in the 2026 list (group-a.md), so it is not included.
- **25 March** (Genocide Day; Jatiya Sangsad resolution of 11 Mar 2017) has a one-minute
  blackout 22:30–22:31 and no illumination, but no half-mast instruction was found, so it is a
  plain observance. It was observed again in 2026 (BSS, AIR).
- **Pohela Falgun** is 14 Feb since the Bangla Academy calendar revision (13 Feb before that).
- Considered and dropped: Rabindra Jayanti (25 Boishakh) and Nazrul Jayanti (11 Joishtho), not
  verified for the revised calendar; Shab-e-Meraj (optional holiday only).

## Malaysia (MY)

6 observances, 0 half-mast days. Federal holidays are in group-a.md, Ramadan in extras.md.

```toml
[[holiday]]
country = "MY"
id = "thaipusam"
name = "Thaipusam (state holiday in seven states/FTs)"
rule = "listed"
dates_2026 = ["2026-02-01"]
dates_2027 = ["2027-01-22"]
dates_2028 = ["2028-02-09"]
tentative_2027 = false
tentative_2028 = true
observance = true
eve = false
greeting = "Selamat Hari Thaipusam"
greeting_en = "Happy Thaipusam"
look = "none"
source = ["https://www.officeholidays.com/countries/malaysia/2026", "https://www.humanresourcesonline.net/malaysia-s-2027-public-holidays-full-list-of-federal-and-state-holidays", "https://www.drikpanchang.com/festivals/thaipusam/tamil-thaipusam-date.html?year=2028", "https://www.qppstudio.net/global-holidays-observances/thaipusam.htm"]

[[holiday]]
country = "MY"
id = "nuzul_al_quran"
name = "Nuzul Al-Quran (17 Ramadan; state holiday in most states)"
rule = "listed"
dates_2026 = ["2026-03-07"]
dates_2027 = ["2027-02-24"]
dates_2028 = ["2028-02-13"]
tentative_2027 = true
tentative_2028 = true
observance = true
eve = false
greeting = "Salam Nuzul Al-Quran"
greeting_en = "Nuzul al-Quran"
look = "none"
source = ["https://www.officeholidays.com/countries/malaysia/2026", "https://www.humanresourcesonline.net/malaysia-s-2027-public-holidays-full-list-of-federal-and-state-holidays"]

[[holiday]]
country = "MY"
id = "hari_guru"
name = "Hari Guru (Teachers' Day)"
rule = "fixed(5,16)"
dates_2026 = ["2026-05-16"]
dates_2027 = ["2027-05-16"]
dates_2028 = ["2028-05-16"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Selamat Hari Guru"
greeting_en = "Happy Teachers' Day"
look = "none"
source = ["https://ecentral.my/tema-hari-guru-2026", "https://www.bernama.com/radio/news.php?id=2557868"]

[[holiday]]
country = "MY"
id = "pesta_kaamatan"
name = "Pesta Kaamatan (Sabah harvest festival; Sabah and Labuan holiday)"
rule = ["fixed(5,30)", "fixed(5,31)"]
dates_2026 = ["2026-05-30", "2026-05-31"]
dates_2027 = ["2027-05-30", "2027-05-31"]
dates_2028 = ["2028-05-30", "2028-05-31"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Kotobian Tadau Tagazo do Kaamatan"
greeting_en = "Happy Kaamatan"
look = "none"
source = ["https://www.officeholidays.com/countries/malaysia/2026", "https://www.humanresourcesonline.net/malaysia-s-2027-public-holidays-full-list-of-federal-and-state-holidays"]

[[holiday]]
country = "MY"
id = "hari_gawai"
name = "Hari Gawai Dayak (Sarawak harvest festival; Sarawak holiday)"
rule = ["fixed(6,1)", "fixed(6,2)"]
dates_2026 = ["2026-06-01", "2026-06-02"]
dates_2027 = ["2027-06-01", "2027-06-02"]
dates_2028 = ["2028-06-01", "2028-06-02"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Selamat Hari Gawai, Gayu Guru Gerai Nyamai"
greeting_en = "Happy Gawai"
look = "none"
source = ["https://www.officeholidays.com/countries/malaysia/2026", "https://www.humanresourcesonline.net/malaysia-s-2027-public-holidays-full-list-of-federal-and-state-holidays"]

[[holiday]]
country = "MY"
id = "hari_pahlawan"
name = "Hari Pahlawan (Warriors' Day)"
rule = "fixed(7,31)"
dates_2026 = ["2026-07-31"]
dates_2027 = ["2027-07-31"]
dates_2028 = ["2028-07-31"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Hari Pahlawan"
greeting_en = "Warriors' Day"
look = "none"
source = ["https://berita.rtm.gov.my/nasional/senarai-berita-nasional/senarai-artikel/hari-pahlawan-2026-semarak-patriotisme-bersama/", "https://bernama.com/bm/am/news.php?id=2588201"]
```

Notes:

- **No annual half-mast day found.** Half-mast in Malaysia is ordered case by case (deaths of
  rulers and leaders, disasters). Hari Pahlawan (31 Jul, held every year since 1968 at Dataran
  Pahlawan Negara, Putrajaya) is a parade and wreath-laying, with no half-mast instruction in
  the Bernama/RTM 2026 reports. The Jalur Gemilang rules text was not found online.
- Thaipusam, Nuzul Al-Quran, Kaamatan and Gawai are **state** holidays (days off only in the
  states named), so at country level they are observances. 2026 dates: officeholidays.com
  (from the JPM/BKPP schedule); 2027: the BKPP 2027 schedule via HR Online. The official
  kabinet.gov.my page refused connections in group-a's research too.
- Thaipusam 2028 is tentative: drikpanchang, HolidayAPI and QPP Studio give Wed 9 Feb;
  calendarlabs gives Sun 13 Feb. 9 Feb is next to the Feb 2028 full moon, which Thaipusam
  follows, so 9 Feb is used.
- Nuzul Al-Quran 2027 (24 Feb) is official but lunar, so `tentative_2027 = true` like group-a's
  Islamic days; 2028 (13 Feb) is 17 Ramadan counted from extras.md's 28 Jan 2028 start.
- Kaamatan (30–31 May) and Gawai (1–2 Jun) are fixed by state law; in-lieu days are not listed.
- Considered and dropped: Israk Mikraj (state only; two dates in 2027), Awal Ramadan (state
  only; Ramadan is already in extras.md), Hari Wilayah (Federal Territories only), Bulan
  Kebangsaan / Kibar Jalur Gemilang (the flag month has no fixed start date).

## Brazil (BR)

6 observances, 0 half-mast days. Feriados, Carnaval and Easter Sunday are in group-c.md.

```toml
[[holiday]]
country = "BR"
id = "ash_wednesday"
name = "Quarta-feira de Cinzas (Ash Wednesday; federal ponto facultativo until 14:00)"
rule = "easter(-46)"
dates_2026 = ["2026-02-18"]
dates_2027 = ["2027-02-10"]
dates_2028 = ["2028-03-01"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Quarta-feira de Cinzas"
greeting_en = "Ash Wednesday"
look = "none"
source = ["https://www.in.gov.br/web/dou/-/portaria-mgi-n-11.460-de-29-de-dezembro-de-2025-678388627", "https://www.officeholidays.com/holidays/brazil/ash-wednesday"]

[[holiday]]
country = "BR"
id = "dia_das_maes"
name = "Dia das Mães (Mother's Day)"
rule = "nth_weekday(5, sun, 2)"
dates_2026 = ["2026-05-10"]
dates_2027 = ["2027-05-09"]
dates_2028 = ["2028-05-14"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Feliz Dia das Mães"
greeting_en = "Happy Mother's Day"
look = "none"
source = ["https://exame.com/brasil/dia-das-maes-como-surgiu-a-celebracao-no-brasil-e-no-resto-do-mundo/", "https://www.infoescola.com/datas-comemorativas/dia-das-maes/"]

[[holiday]]
country = "BR"
id = "sao_joao"
name = "Dia de São João (Festa Junina; state holiday in parts of the Northeast)"
rule = "fixed(6,24)"
dates_2026 = ["2026-06-24"]
dates_2027 = ["2027-06-24"]
dates_2028 = ["2028-06-24"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = true
greeting = "Viva São João!"
greeting_en = "Happy São João"
look = "bonfire"
source = ["https://pt.wikipedia.org/wiki/Festa_junina", "https://www.officeholidays.com/countries/brazil/2026"]

[[holiday]]
country = "BR"
id = "dia_dos_pais"
name = "Dia dos Pais (Father's Day)"
rule = "nth_weekday(8, sun, 2)"
dates_2026 = ["2026-08-09"]
dates_2027 = ["2027-08-08"]
dates_2028 = ["2028-08-13"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Feliz Dia dos Pais"
greeting_en = "Happy Father's Day"
look = "none"
source = ["https://www.officeholidays.com/holidays/brazil/fathers-day", "https://pt.wikipedia.org/wiki/Dia_dos_Pais"]

[[holiday]]
country = "BR"
id = "dia_do_professor"
name = "Dia do Professor (Teachers' Day; school holiday)"
rule = "fixed(10,15)"
dates_2026 = ["2026-10-15"]
dates_2027 = ["2027-10-15"]
dates_2028 = ["2028-10-15"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Feliz Dia do Professor"
greeting_en = "Happy Teachers' Day"
look = "none"
source = ["https://www.saopaulo.sp.leg.br/iah/fulltext/parecer/EDUC1445-2014.pdf", "https://www.migalhas.com.br/depeso/334992/profissao-dignidade-o-dia-do-professor"]

[[holiday]]
country = "BR"
id = "dia_da_bandeira"
name = "Dia da Bandeira (Flag Day)"
rule = "fixed(11,19)"
dates_2026 = ["2026-11-19"]
dates_2027 = ["2027-11-19"]
dates_2028 = ["2028-11-19"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Dia da Bandeira"
greeting_en = "Flag Day"
look = "flag"
source = ["https://legis.senado.leg.br/norma/547573", "https://www.migalhas.com.br/quentes/733/bandeira-nacional"]
```

Notes:

- **No annual half-mast day.** Lei 5.700/1971 art. 18 (as amended by Lei 5.812/1973):
  "Hasteia-se a Bandeira Nacional em funeral nas seguintes situações, desde que não coincidam
  com os dias de festa nacional": nationwide only when the President decrees official mourning;
  otherwise at a legislature, court, or state/municipal seat on the death of its member or head.
  No fixed date, so nothing for Finados (2 Nov) either. planalto.gov.br reset the connection
  twice; the text was read from Migalhas' reproduction and cross-checked with the Senate's
  legislation portal entry (legis.senado.leg.br/norma/547573).
- **Dia da Bandeira**: art. 15 §2: on 19 November the flag is hoisted at 12:00 "com
  solenidades especiais", and art. 32 has worn-out flags burned that day. Not a day off.
- **Ash Wednesday** is a federal ponto facultativo until 14:00 (Portaria MGI 11.460/2025 for
  2026; same every year), so most people work at least half the day. Carnaval Monday and Tuesday
  stay in group-c.md.
- **São João** (24 Jun) is the peak of the June festivals, with bonfires on the night of the
  23rd (hence `eve = true`, `look = "bonfire"`); it is a state holiday in several Northeastern
  states only.
- Dia das Mães: Decreto 21.366 of 5 May 1932 (second Sunday of May), per Exame and Infoescola;
  not a day off. Dia dos Pais (second Sunday of August) is custom only. Dia do Professor:
  Decreto 52.682 of 14 Oct 1963 made 15 Oct a school holiday (per a São Paulo câmara opinion and
  Migalhas).
- Considered and dropped: Dia dos Namorados (12 Jun, commercial), Dia das Crianças (12 Oct,
  same day as the Aparecida holiday), Dia dos Povos Indígenas (19 Apr, lower recognition).

## Mexico (MX)

8 observances, 32 half-mast days (every "a media asta" date in art. 18 of the flag law). Días de
descanso and Día de Muertos are in group-c.md. Entries are in calendar order.

```toml
[[holiday]]
country = "MX"
id = "media_asta_margarita_maza"
name = "Aniversario luctuoso de Margarita Maza (1871) — bandera a media asta"
rule = "fixed(1,2)"
dates_2026 = ["2026-01-02"]
dates_2027 = ["2027-01-02"]
dates_2028 = ["2028-01-02"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Margarita Maza (1871)"
greeting_en = "Remembering Margarita Maza"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "dia_de_reyes"
name = "Día de Reyes (Epiphany, Three Kings' Day)"
rule = "fixed(1,6)"
dates_2026 = ["2026-01-06"]
dates_2027 = ["2027-01-06"]
dates_2028 = ["2028-01-06"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "¡Feliz Día de Reyes!"
greeting_en = "Happy Three Kings' Day"
look = "none"
source = ["https://www.mexicodesconocido.com.mx/dia-de-reyes.html", "https://www.officeholidays.com/countries/mexico/2026"]

[[holiday]]
country = "MX"
id = "media_asta_ignacia_riesch"
name = "Aniversario luctuoso de Ignacia Riesch (1865) — bandera a media asta"
rule = "fixed(1,15)"
dates_2026 = ["2026-01-15"]
dates_2027 = ["2027-01-15"]
dates_2028 = ["2028-01-15"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Ignacia Riesch (1865)"
greeting_en = "Remembering Ignacia Riesch"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "dia_de_la_candelaria"
name = "Día de la Candelaria (Candlemas)"
rule = "fixed(2,2)"
dates_2026 = ["2026-02-02"]
dates_2027 = ["2027-02-02"]
dates_2028 = ["2028-02-02"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "¡Feliz Día de la Candelaria!"
greeting_en = "Happy Candlemas"
look = "candles"
source = ["https://www.elfinanciero.com.mx/food-and-drink/2022/01/30/como-inicio-la-tradicion-de-comer-tamales-el-2-de-febrero/", "https://www.meteored.mx/noticias/actualidad/por-que-el-dia-de-la-candelaria-se-celebra-con-tamales-en-mexico.html"]

[[holiday]]
country = "MX"
id = "media_asta_vicente_guerrero"
name = "Aniversario luctuoso de Vicente Guerrero (1831) y de Agustina Ramírez (1879) — bandera a media asta"
rule = "fixed(2,14)"
dates_2026 = ["2026-02-14"]
dates_2027 = ["2027-02-14"]
dates_2028 = ["2028-02-14"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Vicente Guerrero (1831) y de Agustina Ramírez (1879)"
greeting_en = "Remembering Vicente Guerrero"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_francisco_madero"
name = "Aniversario luctuoso de Francisco I. Madero (1913) — bandera a media asta"
rule = "fixed(2,22)"
dates_2026 = ["2026-02-22"]
dates_2027 = ["2027-02-22"]
dates_2028 = ["2028-02-22"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Francisco I. Madero (1913)"
greeting_en = "Remembering Francisco Madero"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "dia_de_la_bandera"
name = "Día de la Bandera (Flag Day)"
rule = "fixed(2,24)"
dates_2026 = ["2026-02-24"]
dates_2027 = ["2027-02-24"]
dates_2028 = ["2028-02-24"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "¡Feliz Día de la Bandera!"
greeting_en = "Happy Flag Day"
look = "flag"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_cuauhtemoc"
name = "Aniversario luctuoso de Cuauhtémoc (1525) — bandera a media asta"
rule = "fixed(2,28)"
dates_2026 = ["2026-02-28"]
dates_2027 = ["2027-02-28"]
dates_2028 = ["2028-02-28"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Cuauhtémoc (1525)"
greeting_en = "Remembering Cuauhtémoc"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_josefa_ortiz"
name = "Aniversario luctuoso de Josefa Ortiz (1829) — bandera a media asta"
rule = "fixed(3,2)"
dates_2026 = ["2026-03-02"]
dates_2027 = ["2027-03-02"]
dates_2028 = ["2028-03-02"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Josefa Ortiz (1829)"
greeting_en = "Remembering Josefa Ortiz"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_emiliano_zapata"
name = "Aniversario luctuoso de Emiliano Zapata (1919) — bandera a media asta"
rule = "fixed(4,10)"
dates_2026 = ["2026-04-10"]
dates_2027 = ["2027-04-10"]
dates_2028 = ["2028-04-10"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Emiliano Zapata (1919)"
greeting_en = "Remembering Emiliano Zapata"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_rosario_ibarra"
name = "Aniversario luctuoso de Rosario Ibarra (2022) — bandera a media asta"
rule = "fixed(4,16)"
dates_2026 = ["2026-04-16"]
dates_2027 = ["2027-04-16"]
dates_2028 = ["2028-04-16"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Rosario Ibarra (2022)"
greeting_en = "Remembering Rosario Ibarra"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_sor_juana"
name = "Aniversario luctuoso de Sor Juana Inés de la Cruz (1695) — bandera a media asta"
rule = "fixed(4,17)"
dates_2026 = ["2026-04-17"]
dates_2027 = ["2027-04-17"]
dates_2028 = ["2028-04-17"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Sor Juana Inés de la Cruz (1695)"
greeting_en = "Remembering Sor Juana"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_elvia_carrillo_puerto"
name = "Aniversario luctuoso de Elvia Carrillo Puerto (1965) — bandera a media asta"
rule = "fixed(4,18)"
dates_2026 = ["2026-04-18"]
dates_2027 = ["2027-04-18"]
dates_2028 = ["2028-04-18"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Elvia Carrillo Puerto (1965)"
greeting_en = "Remembering Elvia Carrillo"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_defensa_de_veracruz"
name = "Aniversario de la heroica defensa del Puerto de Veracruz (1914) — bandera a media asta"
rule = "fixed(4,21)"
dates_2026 = ["2026-04-21"]
dates_2027 = ["2027-04-21"]
dates_2028 = ["2028-04-21"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario de la heroica defensa del Puerto de Veracruz (1914)"
greeting_en = "Defence of Veracruz, 1914"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "dia_del_nino"
name = "Día del Niño (Children's Day)"
rule = "fixed(4,30)"
dates_2026 = ["2026-04-30"]
dates_2027 = ["2027-04-30"]
dates_2028 = ["2028-04-30"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "¡Feliz Día del Niño!"
greeting_en = "Happy Children's Day"
look = "none"
source = ["https://www.excelsior.com.mx/estilo-de-vida/origen-y-significado-dia-nino", "https://www.mexicodesconocido.com.mx/dia-del-nino-30-de-abril-mexico-por-que-se-festeja.html"]

[[holiday]]
country = "MX"
id = "media_asta_escuadron_201"
name = "Conmemoración de los pilotos caídos del Escuadrón 201 (1945) — bandera a media asta"
rule = "fixed(5,2)"
dates_2026 = ["2026-05-02"]
dates_2027 = ["2027-05-02"]
dates_2028 = ["2028-05-02"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Conmemoración de los pilotos caídos del Escuadrón 201 (1945)"
greeting_en = "Remembering Escuadrón 201"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "batalla_de_puebla"
name = "Aniversario de la Batalla de Puebla (Cinco de Mayo)"
rule = "fixed(5,5)"
dates_2026 = ["2026-05-05"]
dates_2027 = ["2027-05-05"]
dates_2028 = ["2028-05-05"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario de la Batalla de Puebla"
greeting_en = "Battle of Puebla Day"
look = "flag"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://www.officeholidays.com/countries/mexico/2026"]

[[holiday]]
country = "MX"
id = "dia_de_las_madres"
name = "Día de las Madres (Mother's Day)"
rule = "fixed(5,10)"
dates_2026 = ["2026-05-10"]
dates_2027 = ["2027-05-10"]
dates_2028 = ["2028-05-10"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "¡Feliz Día de las Madres!"
greeting_en = "Happy Mother's Day"
look = "none"
source = ["https://unamglobal.unam.mx/global_revista/dia-de-las-madres-origenes-y-significado/", "https://expansion.mx/tendencias/2026/05/08/por-que-se-celebra-10-de-mayo-dia-de-las-madres"]

[[holiday]]
country = "MX"
id = "dia_del_maestro"
name = "Día del Maestro (Teachers' Day)"
rule = "fixed(5,15)"
dates_2026 = ["2026-05-15"]
dates_2027 = ["2027-05-15"]
dates_2028 = ["2028-05-15"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "¡Feliz Día del Maestro!"
greeting_en = "Happy Teachers' Day"
look = "none"
source = ["https://www.gaceta.unam.mx/15-de-mayo-el-origen-del-dia-del-maestro/", "https://www.cndh.org.mx/sites/default/files/documentos/2025-05/FRN_MAY_15-1.pdf"]

[[holiday]]
country = "MX"
id = "media_asta_venustiano_carranza"
name = "Aniversario luctuoso de Venustiano Carranza (1920) — bandera a media asta"
rule = "fixed(5,21)"
dates_2026 = ["2026-05-21"]
dates_2027 = ["2027-05-21"]
dates_2028 = ["2028-05-21"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Venustiano Carranza (1920)"
greeting_en = "Remembering Carranza"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_mariano_escobedo"
name = "Aniversario luctuoso de Mariano Escobedo (1902) — bandera a media asta"
rule = "fixed(5,22)"
dates_2026 = ["2026-05-22"]
dates_2027 = ["2027-05-22"]
dates_2028 = ["2028-05-22"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Mariano Escobedo (1902)"
greeting_en = "Remembering Mariano Escobedo"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_juana_belen_gutierrez"
name = "Aniversario luctuoso de Juana Belén Gutiérrez (1942) — bandera a media asta"
rule = "fixed(7,13)"
dates_2026 = ["2026-07-13"]
dates_2027 = ["2027-07-13"]
dates_2028 = ["2028-07-13"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Juana Belén Gutiérrez (1942)"
greeting_en = "Remembering Juana Gutiérrez"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_cuca_garcia"
name = "Aniversario luctuoso de María Refugio «Cuca» García (1973) — bandera a media asta"
rule = "fixed(7,16)"
dates_2026 = ["2026-07-16"]
dates_2027 = ["2027-07-16"]
dates_2028 = ["2028-07-16"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de María Refugio «Cuca» García (1973)"
greeting_en = "Remembering Cuca García"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_alvaro_obregon"
name = "Aniversario luctuoso del General Álvaro Obregón (1928) — bandera a media asta"
rule = "fixed(7,17)"
dates_2026 = ["2026-07-17"]
dates_2027 = ["2027-07-17"]
dates_2028 = ["2028-07-17"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso del General Álvaro Obregón (1928)"
greeting_en = "Remembering Álvaro Obregón"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_benito_juarez"
name = "Aniversario luctuoso de Benito Juárez (1872) — bandera a media asta"
rule = "fixed(7,18)"
dates_2026 = ["2026-07-18"]
dates_2027 = ["2027-07-18"]
dates_2028 = ["2028-07-18"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Benito Juárez (1872)"
greeting_en = "Remembering Benito Juárez"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_miguel_hidalgo"
name = "Aniversario luctuoso de Miguel Hidalgo y Costilla (1811) — bandera a media asta"
rule = "fixed(7,30)"
dates_2026 = ["2026-07-30"]
dates_2027 = ["2027-07-30"]
dates_2028 = ["2028-07-30"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Miguel Hidalgo y Costilla (1811)"
greeting_en = "Remembering Miguel Hidalgo"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_sara_perez_romero"
name = "Aniversario luctuoso de Sara Pérez Romero (1952) — bandera a media asta"
rule = "fixed(7,31)"
dates_2026 = ["2026-07-31"]
dates_2027 = ["2027-07-31"]
dates_2028 = ["2028-07-31"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Sara Pérez Romero (1952)"
greeting_en = "Remembering Sara Pérez"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_leona_vicario"
name = "Aniversario luctuoso de Leona Vicario (1842) y de Carmen Serdán (1948) — bandera a media asta"
rule = "fixed(8,21)"
dates_2026 = ["2026-08-21"]
dates_2027 = ["2027-08-21"]
dates_2028 = ["2028-08-21"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Leona Vicario (1842) y de Carmen Serdán (1948)"
greeting_en = "Remembering Leona Vicario"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_batallon_de_san_patricio"
name = "Gesta heroica del Batallón de San Patricio (1847) — bandera a media asta"
rule = "fixed(9,12)"
dates_2026 = ["2026-09-12"]
dates_2027 = ["2027-09-12"]
dates_2028 = ["2028-09-12"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Gesta heroica del Batallón de San Patricio (1847)"
greeting_en = "San Patricio Battalion, 1847"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_ninos_heroes"
name = "Aniversario del sacrificio de los Niños Héroes de Chapultepec (1847) — bandera a media asta"
rule = "fixed(9,13)"
dates_2026 = ["2026-09-13"]
dates_2027 = ["2027-09-13"]
dates_2028 = ["2028-09-13"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario del sacrificio de los Niños Héroes de Chapultepec (1847)"
greeting_en = "Remembering the Niños Héroes"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_sismos_1985_2017"
name = "Conmemoración de los sismos de 1985 y 2017 — bandera a media asta"
rule = "fixed(9,19)"
dates_2026 = ["2026-09-19"]
dates_2027 = ["2027-09-19"]
dates_2028 = ["2028-09-19"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Conmemoración de los sismos de 1985 y 2017"
greeting_en = "1985 and 2017 earthquakes"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_laureana_wright"
name = "Aniversario luctuoso de Laureana Wright (1896) — bandera a media asta"
rule = "fixed(9,22)"
dates_2026 = ["2026-09-22"]
dates_2027 = ["2027-09-22"]
dates_2028 = ["2028-09-22"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Laureana Wright (1896)"
greeting_en = "Remembering Laureana Wright"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_tlatelolco_1968"
name = "Caídos en la lucha por la democracia en Tlatelolco (1968) — bandera a media asta"
rule = "fixed(10,2)"
dates_2026 = ["2026-10-02"]
dates_2027 = ["2027-10-02"]
dates_2028 = ["2028-10-02"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Caídos en la lucha por la democracia en Tlatelolco (1968)"
greeting_en = "Remembering Tlatelolco 1968"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_belisario_dominguez"
name = "Sacrificio del senador Belisario Domínguez (1913) — bandera a media asta"
rule = "fixed(10,7)"
dates_2026 = ["2026-10-07"]
dates_2027 = ["2027-10-07"]
dates_2028 = ["2028-10-07"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Sacrificio del senador Belisario Domínguez (1913)"
greeting_en = "Belisario Domínguez, 1913"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_gertrudis_bocanegra"
name = "Aniversario luctuoso de Gertrudis Bocanegra (1817) y de Rita Cetina Gutiérrez (1908) — bandera a media asta"
rule = "fixed(10,11)"
dates_2026 = ["2026-10-11"]
dates_2027 = ["2027-10-11"]
dates_2028 = ["2028-10-11"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Gertrudis Bocanegra (1817) y de Rita Cetina Gutiérrez (1908)"
greeting_en = "Gertrudis Bocanegra, 1817"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_dolores_jimenez_y_muro"
name = "Aniversario luctuoso de Dolores Jiménez y Muro (1925) — bandera a media asta"
rule = "fixed(10,15)"
dates_2026 = ["2026-10-15"]
dates_2027 = ["2027-10-15"]
dates_2028 = ["2028-10-15"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Dolores Jiménez y Muro (1925)"
greeting_en = "Remembering Dolores Jiménez"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_elena_arizmendi"
name = "Aniversario luctuoso de Elena Arizmendi (1949) — bandera a media asta"
rule = "fixed(11,3)"
dates_2026 = ["2026-11-03"]
dates_2027 = ["2027-11-03"]
dates_2028 = ["2028-11-03"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de Elena Arizmendi (1949)"
greeting_en = "Remembering Elena Arizmendi"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "media_asta_eliminacion_violencia_contra_la_mujer"
name = "Día Internacional de la Eliminación de la Violencia contra la Mujer — bandera a media asta"
rule = "fixed(11,25)"
dates_2026 = ["2026-11-25"]
dates_2027 = ["2027-11-25"]
dates_2028 = ["2028-11-25"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Día Internacional de la Eliminación de la Violencia contra la Mujer"
greeting_en = "End Violence Against Women"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]

[[holiday]]
country = "MX"
id = "virgen_de_guadalupe"
name = "Día de la Virgen de Guadalupe (Our Lady of Guadalupe)"
rule = "fixed(12,12)"
dates_2026 = ["2026-12-12"]
dates_2027 = ["2027-12-12"]
dates_2028 = ["2028-12-12"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = true
greeting = "¡Felices fiestas guadalupanas!"
greeting_en = "Our Lady of Guadalupe"
look = "candles"
source = ["https://en.wikipedia.org/wiki/Our_Lady_of_Guadalupe", "https://www.officeholidays.com/countries/mexico/2026"]

[[holiday]]
country = "MX"
id = "media_asta_jose_maria_morelos"
name = "Aniversario luctuoso de José María Morelos (1815) — bandera a media asta"
rule = "fixed(12,22)"
dates_2026 = ["2026-12-22"]
dates_2027 = ["2027-12-22"]
dates_2028 = ["2028-12-22"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Aniversario luctuoso de José María Morelos (1815)"
greeting_en = "Remembering Morelos"
look = "half_mast"
source = ["https://www.diputados.gob.mx/LeyesBiblio/pdf/213.pdf", "https://mley.mx/LEBHN/articulo/18/", "https://sdv.com.mx/compendio/ley-sobre-el-escudo-la-bandera-y-el-himno-nacionales/articulo-18/"]
```

Notes:

- **Basis.** Ley sobre el Escudo, la Bandera y el Himno Nacionales, art. 15: on dates declared
  solemn for the whole Nation the flag is raised "a toda o a media asta, según se trate de
  festividad o duelo"; art. 18 lists the dates, part I full mast (52 entries incl. 24 Feb
  Día de la Bandera and 5 May Batalla de Puebla) and part II half mast (32 entries). The list
  here is art. 18 part II as in force after the DOF reform of 21 Jul 2025 (recognition of women
  who transformed the country), which added several women's death anniversaries.
- **The two sources agree, but neither is primary.** diputados.gob.mx (the official
  consolidated PDF, LeyesBiblio/pdf/213.pdf) refused connections, the SEP-hosted PDF failed DNS
  twice and dof.gob.mx's 21 Jul 2025 index did not show the decree. The 32 dates were read on
  two independent legal portals, mley.mx and sdv.com.mx (both "sin reformas posteriores al
  21-07-2025"), which match entry for entry. **Re-check against the DOF text before shipping.**
- **Window:** art. 18 sets no hours, so treat each date as half-mast for the whole flag day.
- **Volume.** 32 half-mast days is a lot for an app look. If the owner wants a short list, the
  widely known ones are 22 Feb (Madero), 28 Feb (Cuauhtémoc), 10 Apr (Zapata), 18 Jul
  (Juárez), 30 Jul (Hidalgo), 13 Sep (Niños Héroes), 19 Sep (earthquakes), 2 Oct (Tlatelolco)
  and 22 Dec (Morelos).
- Shared dates: 14 Feb (Guerrero and Agustina Ramírez), 21 Aug (Leona Vicario and Carmen
  Serdán) and 11 Oct (Gertrudis Bocanegra and Rita Cetina) each carry two people in one entry;
  `name` and `greeting` say both, `greeting_en` names the first.
- `media_asta_benito_juarez` (18 Jul, death) is separate from group-c's `benito_juarez_birthday`
  (third Monday of March, a day off).
- Observances: Día de las Madres is always 10 May (not a Sunday rule); Día del Maestro dates
  from a 1917 decree first observed in 1918 (Gaceta UNAM); Día del Niño from 1924 (Excélsior,
  México Desconocido). Virgen de Guadalupe has `eve = true` for the night-before mañanitas.
  Cinco de Mayo is a holiday in Puebla state only.
- Considered and dropped: Las Posadas (16–24 Dec, covered by the global Christmas lights),
  Día de la Independencia eve (already `eve = true` in group-c), Día de la Raza (12 Oct,
  full-mast only, low recognition).

## Nigeria (NG)

3 observances, 0 half-mast days. Public holidays and Easter Sunday are in group-c.md.

```toml
[[holiday]]
country = "NG"
id = "armed_forces_remembrance_day"
name = "Armed Forces Remembrance Day"
rule = "fixed(1,15)"
dates_2026 = ["2026-01-15"]
dates_2027 = ["2027-01-15"]
dates_2028 = ["2028-01-15"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Armed Forces Remembrance Day"
greeting_en = "Armed Forces Remembrance"
look = "none"
source = ["https://thewillnews.com/afrd-2024-tinubu-honours-fallen-heroes/", "https://blueprint.ng/2022-celebrating-the-armed-forces-of-nigeria-amid-security-challenges/", "https://en.wikipedia.org/wiki/Armed_Forces_Remembrance_Day"]

[[holiday]]
country = "NG"
id = "mothering_sunday"
name = "Mothering Sunday (fourth Sunday of Lent)"
rule = "easter(-21)"
dates_2026 = ["2026-03-15"]
dates_2027 = ["2027-03-07"]
dates_2028 = ["2028-03-26"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Happy Mothering Sunday"
greeting_en = "Happy Mother's Day"
look = "none"
source = ["https://guardian.ng/sunday-magazine/mothering-sunday-celebration-2021/", "https://guardian.ng/sunday-magazine/uec-maiden-mothering-sunday-held-in-eight-superintendencies/"]

[[holiday]]
country = "NG"
id = "childrens_day"
name = "Children's Day"
rule = "fixed(5,27)"
dates_2026 = ["2026-05-27"]
dates_2027 = ["2027-05-27"]
dates_2028 = ["2028-05-27"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Happy Children's Day"
greeting_en = "Happy Children's Day"
look = "none"
source = ["https://gazettengr.com/childrens-day-fg-reaffirms-commitment-to-inclusive-education-child-development/", "https://nounnews.nou.edu.ng/index.php/node/2035"]
```

Notes:

- **No annual half-mast day found.** Half-mast in Nigeria is ordered by the President on a
  death or disaster. Armed Forces Remembrance Day (15 Jan; end of the civil war in 1970) is a
  wreath-laying at the National Arcade, Eagle Square, with parades in the 36 states and the
  emblem appeal launched the November before; no report of half-masting was found (2021–2025
  coverage in TheCable, The Will, Blueprint).
- **Children's Day** (27 May) is marked by federal and state education ministries (2026 theme
  "Future Now: Promoting Inclusion for Every Nigerian Child"). It is not in the Public Holidays
  Act, so it is an observance; whether schools close was not confirmed.
- **Mothering Sunday** follows the Anglican date, the fourth Sunday of Lent (Easter − 21).
  Catholic women's groups hold their own Mothers' Day (rescheduled in 2026 around Holy Week),
  so the date is not universal.
- Considered and dropped: Palm Sunday and Father's Day (no Nigerian source found), Islamic New
  Year (state holiday in some northern states only), Ramadan (could be added like extras.md, but
  Nigeria's moon-sighting dates were not researched here).

## Ethiopia (ET)

4 observances, 0 half-mast days. Public holidays (Proclamation 1334/2024) and Derg Downfall Day
are in group-b.md.

```toml
[[holiday]]
country = "ET"
id = "martyrs_day"
name = "Yekatit 12 / Ye Semaetat Qen (Martyrs' Day)"
rule = "listed"
dates_2026 = ["2026-02-19"]
dates_2027 = ["2027-02-19"]
dates_2028 = ["2028-02-20"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Yekatit 12"
greeting_en = "Martyrs' Day"
look = "none"
source = ["https://en.wikipedia.org/wiki/Yekatit_12", "https://www.fanabc.com/english/?p=57369", "https://www.fanabc.com/english/?p=28904"]

[[holiday]]
country = "ET"
id = "hosanna"
name = "Hosanna (Orthodox Palm Sunday)"
rule = "orthodox_easter(-7)"
dates_2026 = ["2026-04-05"]
dates_2027 = ["2027-04-25"]
dates_2028 = ["2028-04-09"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Melkam Hosanna"
greeting_en = "Happy Hosanna"
look = "none"
source = ["https://www.officeholidays.com/countries/ethiopia/2026", "https://time.et/holidays"]

[[holiday]]
country = "ET"
id = "national_flag_day"
name = "Ye Sendek Alama Qen (National Flag Day; first Monday of Tikimt)"
rule = "listed"
dates_2026 = ["2026-10-12"]
dates_2027 = ["2027-10-18"]
dates_2028 = ["2028-10-16"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Melkam Ye Sendek Alama Qen"
greeting_en = "Happy Flag Day"
look = "flag"
source = ["https://nbe.gov.et/nbe_news/national-flag-day-observed-at-the-national-bank-of-ethiopia/", "https://ecma.gov.et/2025/10/13", "https://en.wikipedia.org/wiki/Flag_of_Ethiopia"]

[[holiday]]
country = "ET"
id = "nations_nationalities_peoples_day"
name = "Nations, Nationalities and Peoples' Day (Hidar 29)"
rule = "listed"
dates_2026 = ["2026-12-08"]
dates_2027 = ["2027-12-09"]
dates_2028 = ["2028-12-08"]
tentative_2027 = true
tentative_2028 = false
observance = true
eve = false
greeting = "Melkam Ye Bihieroch Bihereseboch Qen"
greeting_en = "Nations and Peoples' Day"
look = "none"
source = ["https://allafrica.com/stories/202512080274.html", "https://fanabc.com/english/19th-nations-nationalities-day-celebrations-underway-in-arba-minch-city"]
```

Notes:

- **No annual half-mast day found.** The Flag Proclamation No. 654/2009 is online only as a PDF
  with an embedded Amharic/English font that could not be decoded here; Wikipedia's and FOTW's
  summaries of it mention no fixed half-mast day, and Yekatit 12 reports (Fana 2022, 2024) show
  wreath-laying at the Sidist Kilo monument, not half-masting. National mourning is declared
  case by case by the House of Peoples' Representatives or the government. Treat as
  unverified rather than proven.
- **Ethiopian-calendar conversions.** Meskerem 1 is 11 Sep, but 12 Sep in 2027 (the year
  before a Gregorian leap year), which shifts every date from Sep 2027 to Feb 2028 by one day:
  - Yekatit 12 is 19 Feb, but 20 Feb 2028 (Fana reported the 87th on 20 Feb 2024, the same
    pattern).
  - Flag Day is the first Monday of Tikimt (Tikimt 1 = 11 Oct, or 12 Oct in 2027): 12 Oct 2026,
    18 Oct 2027, 16 Oct 2028; it was 14 Oct 2024 and 13 Oct 2025 (NBE, ECMA), which fits. No
    2026 report was found, so 2026 is computed.
  - Nations, Nationalities and Peoples' Day is Hidar 29, reported as 8 Dec in 2024 and 2025.
    In 2027 Hidar 29 is 9 Dec; some sources say "8 December annually", so 2027 is tentative.
- Hosanna is Orthodox Easter − 7 (5 Apr 2026, 25 Apr 2027, 9 Apr 2028), from the Fasika dates
  in group-b.md.
- Considered and dropped: Irreecha (Oromo thanksgiving; dates set yearly by the Gadaa
  councils, early October), Buhe (13 Nehase, 19 Aug; regional), Ashenda/Shadey (regional,
  Aug), Derg Downfall Day (28 May; id already in group-b.md, excluded there as not a day off;
  could become an observance by flipping that entry instead of adding a new one).

---

## Summary for the app

**Counts**

| Country | Observances | Half-mast days | Notes |
|---|---|---|---|
| ID | 12 | 1 | 5 observances fly the flag by UU 24/2009 Penjelasan |
| IN | 8 | 0 | Flag Code: half-mast only on Government orders |
| CN | 8 | 1 | 13 Dec half-mast is at the Nanjing ceremony venue only |
| US | 5 | 5 | Peace Officers has no half-staff in 2027 (Armed Forces Day) |
| PK | 4 | 0 | 2025 flag instructions: half-mast only on PM's orders |
| NG | 3 | 0 | none found |
| BR | 6 | 0 | Lei 5.700 art. 18: only decreed mourning |
| BD | 2 | 2 | 14 Dec is one entry, counted as half-mast |
| RU | 8 | 1 | Decree 857/1996 |
| ET | 4 | 0 | flag proclamation text not decodable; treat as unverified |
| MX | 8 | 32 | every art. 18 part II date |
| JP | 5 | 1 | 15 Aug, government buildings |
| MY | 6 | 0 | none found |
| UK | 8 | 0 | DCMS: none; Remembrance Sunday is full mast |

**Every half-mast day**

| Country | id | Date rule | Window |
|---|---|---|---|
| ID | `g30s_remembrance` | `fixed(9,30)` | all day 30 Sep; full mast again 06:00 local on 1 Oct (the app raises it at midnight) |
| US | `fallen_firefighters_memorial` | `nth_weekday(5, sun, 1)` (tentative 2027–28) | all day; federal buildings |
| US | `peace_officers_memorial_day` | `listed`: 15 May, none in 2027 | all day; skipped when 15 May is Armed Forces Day |
| US | `memorial_day_half_mast` | `last_weekday(5, mon)` | sunrise to 12:00, then full staff (the app: 00:00 to 12:00) |
| US | `patriot_day` | `fixed(9,11)` | all day |
| US | `pearl_harbor_remembrance_day` | `fixed(12,7)` | all day |
| CN | `national_memorial_day` | `fixed(12,13)` | from the 08:00 flag ceremony; ceremony main venue only |
| JP | `war_dead_memorial_day` | `fixed(8,15)` | all day at government buildings; silence 12:00–12:01 |
| RU | `day_of_memory_and_sorrow` | `fixed(6,22)` | all day |
| BD | `language_martyrs_day_half_mast` | `fixed(2,21)` | all day (twin of group-a `language_martyrs_day`) |
| BD | `martyred_intellectuals_day` | `fixed(12,14)` | all day |
| MX | `media_asta_margarita_maza` | `fixed(1,2)` | all day |
| MX | `media_asta_ignacia_riesch` | `fixed(1,15)` | all day |
| MX | `media_asta_vicente_guerrero` | `fixed(2,14)` | all day |
| MX | `media_asta_francisco_madero` | `fixed(2,22)` | all day |
| MX | `media_asta_cuauhtemoc` | `fixed(2,28)` | all day |
| MX | `media_asta_josefa_ortiz` | `fixed(3,2)` | all day |
| MX | `media_asta_emiliano_zapata` | `fixed(4,10)` | all day |
| MX | `media_asta_rosario_ibarra` | `fixed(4,16)` | all day |
| MX | `media_asta_sor_juana` | `fixed(4,17)` | all day |
| MX | `media_asta_elvia_carrillo_puerto` | `fixed(4,18)` | all day |
| MX | `media_asta_defensa_de_veracruz` | `fixed(4,21)` | all day |
| MX | `media_asta_escuadron_201` | `fixed(5,2)` | all day |
| MX | `media_asta_venustiano_carranza` | `fixed(5,21)` | all day |
| MX | `media_asta_mariano_escobedo` | `fixed(5,22)` | all day |
| MX | `media_asta_juana_belen_gutierrez` | `fixed(7,13)` | all day |
| MX | `media_asta_cuca_garcia` | `fixed(7,16)` | all day |
| MX | `media_asta_alvaro_obregon` | `fixed(7,17)` | all day |
| MX | `media_asta_benito_juarez` | `fixed(7,18)` | all day |
| MX | `media_asta_miguel_hidalgo` | `fixed(7,30)` | all day |
| MX | `media_asta_sara_perez_romero` | `fixed(7,31)` | all day |
| MX | `media_asta_leona_vicario` | `fixed(8,21)` | all day |
| MX | `media_asta_batallon_de_san_patricio` | `fixed(9,12)` | all day |
| MX | `media_asta_ninos_heroes` | `fixed(9,13)` | all day |
| MX | `media_asta_sismos_1985_2017` | `fixed(9,19)` | all day |
| MX | `media_asta_laureana_wright` | `fixed(9,22)` | all day |
| MX | `media_asta_tlatelolco_1968` | `fixed(10,2)` | all day |
| MX | `media_asta_belisario_dominguez` | `fixed(10,7)` | all day |
| MX | `media_asta_gertrudis_bocanegra` | `fixed(10,11)` | all day |
| MX | `media_asta_dolores_jimenez_y_muro` | `fixed(10,15)` | all day |
| MX | `media_asta_elena_arizmendi` | `fixed(11,3)` | all day |
| MX | `media_asta_eliminacion_violencia_contra_la_mujer` | `fixed(11,25)` | all day |
| MX | `media_asta_jose_maria_morelos` | `fixed(12,22)` | all day |

**Half-mast candidates considered and dropped**

| Country | Day | Reason |
|---|---|---|
| US | National Korean War Veterans Armistice Day (27 Jul) | 36 U.S.C. §127 asks for half-staff only "until 2003"; the 2024 proclamation (No. 10788) does not mention the flag and 2025 had only a message. Full-staff display under 4 U.S.C. §6(d). |
| UK | Remembrance Sunday | Designated day at full mast (DCMS list; council policies say full mast). |
| IN | Martyrs' Day (30 Jan) | Two minutes' silence at 11:00, no half-masting; Flag Code allows half-mast only on Government orders. |
| PK | 11 Sep (Quaid), 21 Apr (Iqbal), 16 Oct (Liaquat) | ARY lists them; the 2025 Cabinet Division instructions require PM orders and list none. Sources disagree. |
| BD | 15 Aug (former National Mourning Day) | Cancelled by the interim government (Aug and Oct 2024), not restored for 2026. |
| BD | 25 Mar (Genocide Day) | One-minute blackout 22:30, no half-mast instruction found. |
| CN | 13 Dec nationwide | Half-mast is at the ceremony's main venue only; kept as a CN entry with that caveat. |
| JP | 11 Mar | No government memorial ceremony since 2021. |
| RU | 3 Sep (Solidarity against Terrorism) | No flag rule. |
| BR | Finados (2 Nov) | Lei 5.700 art. 18 has no fixed dates. |
| MY | Hari Pahlawan (31 Jul) | Parade and wreaths, no half-mast instruction found. |
| NG | Armed Forces Remembrance Day (15 Jan) | No half-mast reported 2021–2025. |
| ET | Yekatit 12 (Martyrs' Day) | Wreath-laying only in reports; flag proclamation unreadable here. Kept as an observance. |

**Observances considered and dropped**

| Country | Day | Reason |
|---|---|---|
| ID | Hari Anak Nasional (23 Jul) | 13th candidate; HUT TNI kept instead (cap 12) |
| US | Valentine's Day, Black Friday, Election Day | commercial / not yearly; Halloween is global already |
| UK | Burns Night (25 Jan) | Scotland only; cap 8 |
| IN | Chhath Puja, Lohri, Navratri, Gudi Padwa/Ugadi, National Youth Day | cap 8; regional |
| CN | Laba, Army Day (1 Aug), Martyrs' Day (30 Sep) | lower recognition; Army Day half day only for the military |
| JP | Mother's Day, White Day, Valentine's Day, Ōmisoka | commercial; Ōmisoka covered by the global New Year |
| RU | Old New Year, Radonitsa, 3 Sep | informal / regional / no greeting fit |
| PK | Chand Raat, Shab-e-Miraj, Youm-e-Istehsal, Kashmir Black Day, Iqbal and Liaquat death anniversaries | covered by Eid eve / Sindh only / protest days / low recognition |
| BD | Rabindra Jayanti, Nazrul Jayanti, Shab-e-Meraj | revised-calendar dates not verified / optional holiday |
| MY | Israk Mikraj, Awal Ramadan, Hari Wilayah, Bulan Kebangsaan | state only / Ramadan already in extras.md / no fixed dates |
| BR | Dia dos Namorados, Dia das Crianças, Dia dos Povos Indígenas | commercial / same day as Aparecida / lower recognition |
| MX | Las Posadas, Día de la Raza | covered by global Christmas lights / low recognition |
| NG | Palm Sunday, Father's Day, Islamic New Year, Ramadan | no Nigerian source / state only / dates not researched |
| ET | Irreecha, Buhe, Ashenda, Derg Downfall Day | yearly-set or regional dates; Derg Downfall already in group-b.md |

**Wiring notes for the generator** (not done here)

- `tools/world_holidays.py` loads only `group-*.md` and `extras.md`; this file needs adding to
  `load()`.
- `half_mast` is not yet in `LOOKS` / `ICON_BY_LOOK`. Several half-mast ids contain
  ICON_BY_WORD words (`memorial`, `remembrance`, `martyr`) and would get the flag icon.
- `peace_officers_memorial_day` has `dates_2027 = []`; the generator drops empty years, so it
  is tabled as `listed` for 2026 and 2028 only.
- Multi-day entries (`obon`, `maslenitsa`, `pesta_kaamatan`, `hari_gawai`) use rule arrays like
  group-c's Carnaval; the generator reads the first rule and the run length.
- Tentative and weak spots to re-check before shipping: the Mexican art. 18 list against the
  DOF text, Pakistan's half-mast claim, China's venue-only scope, Ethiopia's flag proclamation,
  US firefighters 2027–28 dates, and every lunar 2028 date.
