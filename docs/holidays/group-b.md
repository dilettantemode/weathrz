# World holidays, group B: CN, JP, RU, ET (2026 + 2027 + 2028)

Compiled 2026-10-20; 2027 re-checked and 2028 added 2026-10-21. Every date was checked against at least two sources; the disagreements are listed in each country's Notes.

Status:
- 2027 JP: official (NAOJ 暦要項 of 2 Feb 2026; the Cabinet Office list agrees).
- 2027 RU: official (decree No. 1187 of 17 Sep 2026; consultant.ru's 2027 production calendar agrees).
- 2027 CN: checked 21 Oct 2026: still not published. The State Council notice is expected in November 2026; gov.cn's latest-policy feed (newest item 9 Oct 2026) has no 2027 notice.
- 2027 ET: the church and civic days are rule-based. Eid al-Fitr, Eid al-Adha and Mawlid: checked 21 Oct 2026: still not published; they are fixed by moon sighting a day or two ahead.
- 2028 (added 21 Oct 2026): no country has an official 2028 list yet. Statutory fixed and rule-based days have `tentative_2028 = false`. Tentative: every CN entry (no 2028 notice; lunar days are estimates), the RU New Year holidays (no 2028 transfer decree), the JP equinox days (NAOJ announces them in Feb 2027), and the three ET Islamic days.

Primary sources:
- CN: State Council notice 国办发明电〔2025〕7号, 4 Nov 2025 (gov.cn, mirrored on news.cn). The 2024 revision of 全国年节及纪念日放假办法 sets the statutory days. 2027 lunar dates cross-checked against officeholidays and Taiwan's 2027 DGPA calendar (PTS); Qingming against the NAOJ 暦要項.
- JP: Cabinet Office 国民の祝日 page (lists 2026 and 2027). NAOJ 暦要項 for 2026 (announced 3 Feb 2025) and 2027 (announced 2 Feb 2026).
- RU: Labour Code art. 112. Government decree No. 1466 of 24 Sep 2025 (2026 transfers) and No. 1187 of 17 Sep 2026 (2027 transfers), cross-checked against buh.ru, ppt.ru, kontur-extern and officeholidays.
- ET: Proclamation No. 1334/2024 (public holidays), plus officeholidays, calendarific, time.et and qppstudio. 2026 Islamic dates checked against Ethiopian press reports (ENA, The Reporter, FMC).
- 2028: CN lunar days and solar terms from the Hong Kong Observatory 2028 conversion table, cross-checked against calendarific (CN 2028) and calendarlabs (Taiwan 2028). JP equinoxes from the HKO table and ja.wikipedia's NAOJ-based prediction table; the other JP days from the Holidays Act (e-Gov). RU from Labour Code art. 112 (consultant.ru); the 2028 transfer decree is not out (allcontract.ru, and consultant.ru has no 2028 calendar yet). ET from calendarific (ET 2028) and time.et, Islamic days cross-checked against the Umm al-Qura tables (Wikipedia, qppstudio) and new-moon times computed with Meeus' algorithm.

Conventions:
- `rule` is the anchor rule of the holiday.
- `dates_*` are ISO strings. For multi-day CN and RU blocks they list every day of the block.
- Substitute and transfer days appear only in Notes.
- `eve = true` means the celebration starts the evening before the first listed date. For CN Spring Festival, the eve (Chuxi) is itself inside the block; see the CN notes.

## China (CN)

```toml
[[holiday]]
country = "CN"
id = "new_year"
name = "New Year's Day (Yuandan)"
rule = "fixed(1,1)"
dates_2026 = ["2026-01-01", "2026-01-02", "2026-01-03"]
dates_2027 = ["2027-01-01"]
dates_2028 = ["2028-01-01"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Yuandan kuaile"
greeting_en = "Happy New Year"
look = "none"
source = ["https://www.gov.cn/zhengce/content/202511/content_7047090.htm", "https://www.news.cn/politics/20241112/53da215f46c546a480f0a01d339dafa2/c.html", "https://calendarific.com/holidays/2028/CN"]

[[holiday]]
country = "CN"
id = "spring_festival"
name = "Spring Festival (Chunjie)"
rule = "listed"
dates_2026 = ["2026-02-15", "2026-02-16", "2026-02-17", "2026-02-18", "2026-02-19", "2026-02-20", "2026-02-21", "2026-02-22", "2026-02-23"]
dates_2027 = ["2027-02-05", "2027-02-06", "2027-02-07", "2027-02-08"]
dates_2028 = ["2028-01-25", "2028-01-26", "2028-01-27", "2028-01-28", "2028-01-29", "2028-01-30", "2028-01-31", "2028-02-01"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Chunjie kuaile"
greeting_en = "Happy Chinese New Year"
look = "lunar_new_year"
source = ["https://www.gov.cn/zhengce/content/202511/content_7047090.htm", "https://www.officeholidays.com/countries/china/2027", "https://news.pts.org.tw/article/809696", "https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2028e.txt", "https://calendarific.com/holidays/2028/CN", "https://www.calendarlabs.com/holidays/taiwan/2028"]

[[holiday]]
country = "CN"
id = "qingming"
name = "Qingming Festival / Tomb-Sweeping Day (Qingming jie)"
rule = "listed"
dates_2026 = ["2026-04-04", "2026-04-05", "2026-04-06"]
dates_2027 = ["2027-04-05"]
dates_2028 = ["2028-04-04"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Qingming jie"
greeting_en = "Qingming Festival"
look = "none"
source = ["https://www.gov.cn/zhengce/content/202511/content_7047090.htm", "https://eco.mtk.nao.ac.jp/koyomi/yoko/2027/rekiyou272.html", "https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2028e.txt", "https://calendarific.com/holidays/2028/CN"]

[[holiday]]
country = "CN"
id = "labour_day"
name = "Labour Day (Laodong jie)"
rule = "fixed(5,1)"
dates_2026 = ["2026-05-01", "2026-05-02", "2026-05-03", "2026-05-04", "2026-05-05"]
dates_2027 = ["2027-05-01", "2027-05-02"]
dates_2028 = ["2028-05-01", "2028-05-02"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Laodongjie kuaile"
greeting_en = "Happy Labour Day"
look = "none"
source = ["https://www.gov.cn/zhengce/content/202511/content_7047090.htm", "https://www.news.cn/politics/20241112/53da215f46c546a480f0a01d339dafa2/c.html", "https://calendarific.com/holidays/2028/CN"]

[[holiday]]
country = "CN"
id = "dragon_boat"
name = "Dragon Boat Festival (Duanwu jie)"
rule = "listed"
dates_2026 = ["2026-06-19", "2026-06-20", "2026-06-21"]
dates_2027 = ["2027-06-09"]
dates_2028 = ["2028-05-28"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Duanwu ankang"
greeting_en = "Happy Dragon Boat Festival"
look = "none"
source = ["https://www.gov.cn/zhengce/content/202511/content_7047090.htm", "https://www.officeholidays.com/countries/china/2027", "https://news.pts.org.tw/article/809696", "https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2028e.txt", "https://calendarific.com/holidays/2028/CN", "https://www.calendarlabs.com/holidays/taiwan/2028"]

[[holiday]]
country = "CN"
id = "mid_autumn"
name = "Mid-Autumn Festival (Zhongqiu jie)"
rule = "listed"
dates_2026 = ["2026-09-25", "2026-09-26", "2026-09-27"]
dates_2027 = ["2027-09-15"]
dates_2028 = ["2028-10-03"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Zhongqiu kuaile"
greeting_en = "Happy Mid-Autumn Festival"
look = "lanterns"
source = ["https://www.gov.cn/zhengce/content/202511/content_7047090.htm", "https://www.officeholidays.com/countries/china/2027", "https://news.pts.org.tw/article/809696", "https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2028e.txt", "https://calendarific.com/holidays/2028/CN", "https://www.calendarlabs.com/holidays/taiwan/2028"]

[[holiday]]
country = "CN"
id = "national_day"
name = "National Day (Guoqing jie)"
rule = "fixed(10,1)"
dates_2026 = ["2026-10-01", "2026-10-02", "2026-10-03", "2026-10-04", "2026-10-05", "2026-10-06", "2026-10-07"]
dates_2027 = ["2027-10-01", "2027-10-02", "2027-10-03"]
dates_2028 = ["2028-10-01", "2028-10-02", "2028-10-03"]
tentative_2027 = true
tentative_2028 = true
eve = false
greeting = "Guoqing kuaile"
greeting_en = "Happy National Day"
look = "flag"
source = ["https://www.gov.cn/zhengce/content/202511/content_7047090.htm", "https://www.news.cn/politics/20241112/53da215f46c546a480f0a01d339dafa2/c.html", "https://calendarific.com/holidays/2028/CN"]
```

Notes (CN):
- **2026 is official.** The make-up working days (调休上班) are Sun 4 Jan, Sat 14 Feb, Sat 28 Feb, Sat 9 May, Sun 20 Sep and Sat 10 Oct. Qingming, Dragon Boat and Mid-Autumn have no make-up days.
- **2027 notice not out yet.** As of 20 Oct 2026 there is no 2027 notice. The 2026 notice came on 4 Nov 2025 and the 2025 notice on 12 Nov 2024, so expect early-to-mid November 2026. Checked 21 Oct 2026: still not published.
- **What dates_2027 contains.** dates_2027 lists only the statutory days. These are fixed by the 2024 revision of the Measures, in force since 1 Jan 2025, and total 13 days:
  - New Year: 1 day
  - Spring Festival: 4 days (Chuxi plus the 1st–3rd of the first lunar month)
  - Qingming: 1 day
  - Labour Day: 2 days (1–2 May)
  - Dragon Boat: 1 day
  - Mid-Autumn: 1 day
  - National Day: 3 days (1–3 Oct)
- **Likely 2027 blocks (guesses, not official).** The real blocks will wrap these days in weekends and swapped days. Plausible shapes: New Year 1–3 Jan, Qingming 3–5 Apr, Labour Day around 1–5 May, National Day 1–7 Oct. Spring Festival will probably be an 8–9-day block containing 5–8 Feb. Dragon Boat (Wed 9 Jun) and Mid-Autumn (Wed 15 Sep) are midweek, so their blocks are hard to guess.
- **What dates_2028 contains.** The same as dates_2027: the 13 statutory days only, because no 2028 notice exists (it is due around November 2027). So the Spring Festival entry is the 4 statutory days, Chuxi plus the 1st–3rd of the first lunar month (25–28 Jan), in the same shape as 2027 and not the 9-day 2026 block.
- **Likely 2028 blocks (guesses, not official).**
  - New Year falls on Sat 1 Jan, so probably 1–3 Jan; calendarific forecasts Mon 3 Jan as the substitute.
  - Spring Festival: Chuxi falls on a Tuesday, as in 2025, whose official block was 28 Jan–4 Feb (8 days, Chuxi to the 7th day). The same shape gives 25 Jan–1 Feb 2028; calendarific's provisional block is 25–31 Jan.
  - Dragon Boat falls on Sun 28 May, so probably 27–29 May.
  - Mid-Autumn (Tue 3 Oct) falls inside National Day (1–3 Oct). The 2020 precedent, when the two coincided, gave an 8-day 1–8 Oct block; calendarific's provisional block is 1–6 Oct.
- **Spring Festival key days.**
  - 2026: Chuxi (New Year's Eve, the main night) is 16 Feb and Chunjie (first day of the first lunar month) is 17 Feb.
  - 2027: Chuxi is 5 Feb and Chunjie is 6 Feb.
  - 2028: Chuxi is 25 Jan (the 29th of a 29-day 12th month) and Chunjie is 26 Jan.
  - Show the lampions for the whole block and fireworks on Chuxi night. Lantern Festival (Yuanxiao) is not a day off: 3 Mar 2026, 20 Feb 2027, 9 Feb 2028 (could reuse the `lanterns` look).
- **Lunar and solar-term dates.**
  - 2027 lunar dates: officeholidays and Taiwan's 2027 calendar agree on Chunjie 6 Feb, Duanwu 9 Jun and Zhongqiu 15 Sep.
  - Qingming solar term: 5 Apr 2026 (Sun) and 5 Apr 2027. NAOJ gives 5 Apr 2027 09:17 JST, which is 08:17 Beijing time.
  - 2028: the HKO table, calendarific and calendarlabs' Taiwan 2028 list agree on Chunjie 26 Jan, Qingming 4 Apr, Duanwu 28 May and Zhongqiu 3 Oct. 2028 has a leap 5th month (from 23 Jun), which is why Mid-Autumn is so late.
- **Disagreement:** officeholidays' 2027 China page shows National Day as 1–5 Oct. That is its own guess, not an official schedule.
- **Disagreement (2028):** studycli lists Dragon Boat as 25–27 Jun 2028, which matches the 5th day of the leap 5th month (27 Jun), not the festival; HKO, calendarific and calendarlabs give 28 May.
- **Not used for 2028:** officeholidays has no 2028 China page yet, and timeanddate blocks automated fetches.
- **Greeting choices.**
  - Qingming is a solemn day, so the greeting is the plain name; some people now say "Qingming ankang".
  - For Duanwu, "Duanwu ankang" is the usual modern wish; "Duanwujie kuaile" is also common.

## Japan (JP)

```toml
[[holiday]]
country = "JP"
id = "new_year"
name = "New Year's Day (Ganjitsu)"
rule = "fixed(1,1)"
dates_2026 = ["2026-01-01"]
dates_2027 = ["2027-01-01"]
dates_2028 = ["2028-01-01"]
tentative_2027 = false
tentative_2028 = false
eve = true
greeting = "Akemashite omedetō"
greeting_en = "Happy New Year"
look = "none"
source = ["https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html", "https://eco.mtk.nao.ac.jp/koyomi/yoko/2027/rekiyou271.html", "https://laws.e-gov.go.jp/law/323AC1000000178"]

[[holiday]]
country = "JP"
id = "coming_of_age_day"
name = "Coming of Age Day (Seijin no Hi)"
rule = "nth_weekday(1,mon,2)"
dates_2026 = ["2026-01-12"]
dates_2027 = ["2027-01-11"]
dates_2028 = ["2028-01-10"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Seijin omedetō"
greeting_en = "Happy Coming of Age Day"
look = "none"
source = ["https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html", "https://eco.mtk.nao.ac.jp/koyomi/yoko/2027/rekiyou271.html", "https://laws.e-gov.go.jp/law/323AC1000000178"]

[[holiday]]
country = "JP"
id = "national_foundation_day"
name = "National Foundation Day (Kenkoku Kinen no Hi)"
rule = "fixed(2,11)"
dates_2026 = ["2026-02-11"]
dates_2027 = ["2027-02-11"]
dates_2028 = ["2028-02-11"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Kenkoku Kinen no Hi"
greeting_en = "National Foundation Day"
look = "flag"
source = ["https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html", "https://eco.mtk.nao.ac.jp/koyomi/yoko/2027/rekiyou271.html", "https://laws.e-gov.go.jp/law/323AC1000000178"]

[[holiday]]
country = "JP"
id = "emperors_birthday"
name = "The Emperor's Birthday (Tennō Tanjōbi)"
rule = "fixed(2,23)"
dates_2026 = ["2026-02-23"]
dates_2027 = ["2027-02-23"]
dates_2028 = ["2028-02-23"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Tennō Tanjōbi"
greeting_en = "The Emperor's Birthday"
look = "flag"
source = ["https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html", "https://eco.mtk.nao.ac.jp/koyomi/yoko/2027/rekiyou271.html", "https://laws.e-gov.go.jp/law/323AC1000000178"]

[[holiday]]
country = "JP"
id = "vernal_equinox_day"
name = "Vernal Equinox Day (Shunbun no Hi)"
rule = "equinox(spring)"
dates_2026 = ["2026-03-20"]
dates_2027 = ["2027-03-21"]
dates_2028 = ["2028-03-20"]
tentative_2027 = false
tentative_2028 = true
eve = false
greeting = "Shunbun no Hi"
greeting_en = "Vernal Equinox Day"
look = "none"
source = ["https://eco.mtk.nao.ac.jp/koyomi/yoko/2026/rekiyou261.html", "https://eco.mtk.nao.ac.jp/koyomi/yoko/2027/rekiyou271.html", "https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html", "https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2028e.txt", "https://ja.wikipedia.org/wiki/%E6%98%A5%E5%88%86%E3%81%AE%E6%97%A5"]

[[holiday]]
country = "JP"
id = "showa_day"
name = "Shōwa Day (Shōwa no Hi)"
rule = "fixed(4,29)"
dates_2026 = ["2026-04-29"]
dates_2027 = ["2027-04-29"]
dates_2028 = ["2028-04-29"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Shōwa no Hi"
greeting_en = "Showa Day"
look = "none"
source = ["https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html", "https://eco.mtk.nao.ac.jp/koyomi/yoko/2027/rekiyou271.html", "https://laws.e-gov.go.jp/law/323AC1000000178"]

[[holiday]]
country = "JP"
id = "constitution_day"
name = "Constitution Memorial Day (Kenpō Kinenbi)"
rule = "fixed(5,3)"
dates_2026 = ["2026-05-03"]
dates_2027 = ["2027-05-03"]
dates_2028 = ["2028-05-03"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Kenpō Kinenbi"
greeting_en = "Constitution Memorial Day"
look = "none"
source = ["https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html", "https://eco.mtk.nao.ac.jp/koyomi/yoko/2027/rekiyou271.html", "https://laws.e-gov.go.jp/law/323AC1000000178"]

[[holiday]]
country = "JP"
id = "greenery_day"
name = "Greenery Day (Midori no Hi)"
rule = "fixed(5,4)"
dates_2026 = ["2026-05-04"]
dates_2027 = ["2027-05-04"]
dates_2028 = ["2028-05-04"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Midori no Hi"
greeting_en = "Greenery Day"
look = "none"
source = ["https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html", "https://eco.mtk.nao.ac.jp/koyomi/yoko/2027/rekiyou271.html", "https://laws.e-gov.go.jp/law/323AC1000000178"]

[[holiday]]
country = "JP"
id = "childrens_day"
name = "Children's Day (Kodomo no Hi)"
rule = "fixed(5,5)"
dates_2026 = ["2026-05-05"]
dates_2027 = ["2027-05-05"]
dates_2028 = ["2028-05-05"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Kodomo no Hi"
greeting_en = "Happy Children's Day"
look = "koinobori"
source = ["https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html", "https://eco.mtk.nao.ac.jp/koyomi/yoko/2027/rekiyou271.html", "https://laws.e-gov.go.jp/law/323AC1000000178"]

[[holiday]]
country = "JP"
id = "marine_day"
name = "Marine Day (Umi no Hi)"
rule = "nth_weekday(7,mon,3)"
dates_2026 = ["2026-07-20"]
dates_2027 = ["2027-07-19"]
dates_2028 = ["2028-07-17"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Umi no Hi"
greeting_en = "Marine Day"
look = "none"
source = ["https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html", "https://eco.mtk.nao.ac.jp/koyomi/yoko/2027/rekiyou271.html", "https://laws.e-gov.go.jp/law/323AC1000000178"]

[[holiday]]
country = "JP"
id = "mountain_day"
name = "Mountain Day (Yama no Hi)"
rule = "fixed(8,11)"
dates_2026 = ["2026-08-11"]
dates_2027 = ["2027-08-11"]
dates_2028 = ["2028-08-11"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Yama no Hi"
greeting_en = "Mountain Day"
look = "none"
source = ["https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html", "https://eco.mtk.nao.ac.jp/koyomi/yoko/2027/rekiyou271.html", "https://laws.e-gov.go.jp/law/323AC1000000178"]

[[holiday]]
country = "JP"
id = "respect_for_the_aged_day"
name = "Respect for the Aged Day (Keirō no Hi)"
rule = "nth_weekday(9,mon,3)"
dates_2026 = ["2026-09-21"]
dates_2027 = ["2027-09-20"]
dates_2028 = ["2028-09-18"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Keirō no Hi"
greeting_en = "Respect for the Aged Day"
look = "none"
source = ["https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html", "https://eco.mtk.nao.ac.jp/koyomi/yoko/2027/rekiyou271.html", "https://laws.e-gov.go.jp/law/323AC1000000178"]

[[holiday]]
country = "JP"
id = "autumnal_equinox_day"
name = "Autumnal Equinox Day (Shūbun no Hi)"
rule = "equinox(autumn)"
dates_2026 = ["2026-09-23"]
dates_2027 = ["2027-09-23"]
dates_2028 = ["2028-09-22"]
tentative_2027 = false
tentative_2028 = true
eve = false
greeting = "Shūbun no Hi"
greeting_en = "Autumnal Equinox Day"
look = "none"
source = ["https://eco.mtk.nao.ac.jp/koyomi/yoko/2026/rekiyou261.html", "https://eco.mtk.nao.ac.jp/koyomi/yoko/2027/rekiyou271.html", "https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html", "https://www.hko.gov.hk/en/gts/time/calendar/text/files/T2028e.txt", "https://ja.wikipedia.org/wiki/%E7%A7%8B%E5%88%86%E3%81%AE%E6%97%A5"]

[[holiday]]
country = "JP"
id = "sports_day"
name = "Sports Day (Supōtsu no Hi)"
rule = "nth_weekday(10,mon,2)"
dates_2026 = ["2026-10-12"]
dates_2027 = ["2027-10-11"]
dates_2028 = ["2028-10-09"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Supōtsu no Hi"
greeting_en = "Sports Day"
look = "none"
source = ["https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html", "https://eco.mtk.nao.ac.jp/koyomi/yoko/2027/rekiyou271.html", "https://laws.e-gov.go.jp/law/323AC1000000178"]

[[holiday]]
country = "JP"
id = "culture_day"
name = "Culture Day (Bunka no Hi)"
rule = "fixed(11,3)"
dates_2026 = ["2026-11-03"]
dates_2027 = ["2027-11-03"]
dates_2028 = ["2028-11-03"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Bunka no Hi"
greeting_en = "Culture Day"
look = "none"
source = ["https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html", "https://eco.mtk.nao.ac.jp/koyomi/yoko/2027/rekiyou271.html", "https://laws.e-gov.go.jp/law/323AC1000000178"]

[[holiday]]
country = "JP"
id = "labour_thanksgiving_day"
name = "Labour Thanksgiving Day (Kinrō Kansha no Hi)"
rule = "fixed(11,23)"
dates_2026 = ["2026-11-23"]
dates_2027 = ["2027-11-23"]
dates_2028 = ["2028-11-23"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Kinrō Kansha no Hi"
greeting_en = "Labour Thanksgiving Day"
look = "none"
source = ["https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html", "https://eco.mtk.nao.ac.jp/koyomi/yoko/2027/rekiyou271.html", "https://laws.e-gov.go.jp/law/323AC1000000178"]
```

Notes (JP):
- **Equinox dates are official for both years.** The Holidays Act names no date for the equinox days. NAOJ computes them and publishes the next year's dates in the 暦要項 each February. 2026: Vernal 20 Mar, Autumnal 23 Sep (announced 3 Feb 2025). 2027: Vernal 21 Mar (05:25 JST), Autumnal 23 Sep (15:02 JST) (announced 2 Feb 2026). The Cabinet Office list agrees.
- **2028 equinox days are predictions.** The 2028 暦要項 is due in early February 2027, and the Cabinet Office says it will post the 2028 list then. The astronomical equinoxes are 20 Mar 2028 at about 11:17 JST and 22 Sep 2028 at about 20:45 JST, far from midnight, so the predicted days are Vernal Mon 20 Mar and Autumnal Fri 22 Sep. The HKO 2028 table and ja.wikipedia's NAOJ-based table agree.
- **2028 fixed and Happy Monday days** follow from the Holidays Act; the Cabinet Office page does not list 2028 yet.
- **Happy Monday rules:**
  - Seijin no Hi: 2nd Monday of January
  - Umi no Hi: 3rd Monday of July
  - Keirō no Hi: 3rd Monday of September
  - Supōtsu no Hi: 2nd Monday of October
- **Substitute holiday (振替休日, art. 3-2).** When a holiday falls on a Sunday, the next non-holiday day is off. That gives 2026-05-06 (Wed), for Constitution Day on a Sunday, and 2027-03-22 (Mon), for the Vernal Equinox on a Sunday. There are no others in 2026 or 2027. There are none in 2028: no holiday falls on a Sunday (1 Jan and 29 Apr fall on Saturdays, which get no substitute).
- **Citizens' holiday (国民の休日, art. 3-3).** A weekday sandwiched between two holidays is off. That gives 2026-09-22 (Tue), between Keirō no Hi (21st) and Shūbun no Hi (23rd), which makes the 19–23 Sep "Silver Week". There is none in 2027 or 2028.
- **Not national holidays.** The bank and office closure on 31 Dec and 2–3 Jan is not a national holiday. Neither is Golden Week as a block.
- **Eve.** New Year's eve (Ōmisoka, with joya no kane and hatsumōde at midnight) carries `eve = true`.
- **Looks.** None of the menu items is a natural fit for Japanese days except `flag` (hinomaru on 祝日), which is used sparingly.

## Russia (RU)

```toml
[[holiday]]
country = "RU"
id = "new_year_holidays"
name = "New Year Holidays (Novogodniye kanikuly)"
rule = "fixed(1,1)"
dates_2026 = ["2026-01-01", "2026-01-02", "2026-01-03", "2026-01-04", "2026-01-05", "2026-01-06", "2026-01-08"]
dates_2027 = ["2027-01-01", "2027-01-02", "2027-01-03", "2027-01-04", "2027-01-05", "2027-01-06", "2027-01-08"]
dates_2028 = ["2028-01-01", "2028-01-02", "2028-01-03", "2028-01-04", "2028-01-05", "2028-01-06", "2028-01-08"]
tentative_2027 = false
tentative_2028 = true
eve = true
greeting = "S Novym godom"
greeting_en = "Happy New Year"
look = "none"
source = ["https://buh.ru/news/pravitelstvo-utverdilo-poryadok-perenosa-vykhodnykh-dney-v-2026-godu.html", "https://www.kontur-extern.ru/info/normativ/document/1/508389-postanovlenie-pravitelstva-rf-ot-17-09-2026-n-1187", "https://www.consultant.ru/document/cons_doc_LAW_34683/98ef2900507766e70ff29c0b9d8e2353ea80a1cf/", "https://allcontract.ru/calendar/2028"]

[[holiday]]
country = "RU"
id = "orthodox_christmas"
name = "Christmas (Rozhdestvo Khristovo)"
rule = "fixed(1,7)"
dates_2026 = ["2026-01-07"]
dates_2027 = ["2027-01-07"]
dates_2028 = ["2028-01-07"]
tentative_2027 = false
tentative_2028 = false
eve = true
greeting = "S Rozhdestvom"
greeting_en = "Merry Christmas"
look = "candles"
source = ["https://ppt.ru/news/proizvodstvennyy-kalendar/pravitelstvo-utverdilo-perenosy-vykhodnykh-v-2026-godu", "https://ppt.ru/news/proizvodstvennyy-kalendar/utverdili-grafik-vyhodnyh-i-prazdnichnyh-dney-na-2027-god", "https://www.consultant.ru/document/cons_doc_LAW_34683/98ef2900507766e70ff29c0b9d8e2353ea80a1cf/"]

[[holiday]]
country = "RU"
id = "defender_of_the_fatherland_day"
name = "Defender of the Fatherland Day (Den' zashchitnika Otechestva)"
rule = "fixed(2,23)"
dates_2026 = ["2026-02-23"]
dates_2027 = ["2027-02-23"]
dates_2028 = ["2028-02-23"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "S 23 Fevralya"
greeting_en = "Happy Defender of the Fatherland Day"
look = "none"
source = ["https://ppt.ru/news/proizvodstvennyy-kalendar/pravitelstvo-utverdilo-perenosy-vykhodnykh-v-2026-godu", "https://www.kontur-extern.ru/info/normativ/document/1/508389-postanovlenie-pravitelstva-rf-ot-17-09-2026-n-1187", "https://www.consultant.ru/document/cons_doc_LAW_34683/98ef2900507766e70ff29c0b9d8e2353ea80a1cf/"]

[[holiday]]
country = "RU"
id = "womens_day"
name = "International Women's Day (Mezhdunarodny zhensky den')"
rule = "fixed(3,8)"
dates_2026 = ["2026-03-08"]
dates_2027 = ["2027-03-08"]
dates_2028 = ["2028-03-08"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "S 8 Marta"
greeting_en = "Happy Women's Day"
look = "none"
source = ["https://ppt.ru/news/proizvodstvennyy-kalendar/pravitelstvo-utverdilo-perenosy-vykhodnykh-v-2026-godu", "https://ppt.ru/news/proizvodstvennyy-kalendar/utverdili-grafik-vyhodnyh-i-prazdnichnyh-dney-na-2027-god", "https://www.consultant.ru/document/cons_doc_LAW_34683/98ef2900507766e70ff29c0b9d8e2353ea80a1cf/"]

[[holiday]]
country = "RU"
id = "spring_and_labour_day"
name = "Spring and Labour Day (Prazdnik Vesny i Truda)"
rule = "fixed(5,1)"
dates_2026 = ["2026-05-01"]
dates_2027 = ["2027-05-01"]
dates_2028 = ["2028-05-01"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "S Pervomayem"
greeting_en = "Happy Spring and Labour Day"
look = "none"
source = ["https://ppt.ru/news/proizvodstvennyy-kalendar/pravitelstvo-utverdilo-perenosy-vykhodnykh-v-2026-godu", "https://ppt.ru/news/proizvodstvennyy-kalendar/utverdili-grafik-vyhodnyh-i-prazdnichnyh-dney-na-2027-god", "https://www.consultant.ru/document/cons_doc_LAW_34683/98ef2900507766e70ff29c0b9d8e2353ea80a1cf/"]

[[holiday]]
country = "RU"
id = "victory_day"
name = "Victory Day (Den' Pobedy)"
rule = "fixed(5,9)"
dates_2026 = ["2026-05-09"]
dates_2027 = ["2027-05-09"]
dates_2028 = ["2028-05-09"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "S Dnyom Pobedy"
greeting_en = "Happy Victory Day"
look = "fireworks_night"
source = ["https://ppt.ru/news/proizvodstvennyy-kalendar/pravitelstvo-utverdilo-perenosy-vykhodnykh-v-2026-godu", "https://ppt.ru/news/proizvodstvennyy-kalendar/utverdili-grafik-vyhodnyh-i-prazdnichnyh-dney-na-2027-god", "https://www.consultant.ru/document/cons_doc_LAW_34683/98ef2900507766e70ff29c0b9d8e2353ea80a1cf/"]

[[holiday]]
country = "RU"
id = "russia_day"
name = "Russia Day (Den' Rossii)"
rule = "fixed(6,12)"
dates_2026 = ["2026-06-12"]
dates_2027 = ["2027-06-12"]
dates_2028 = ["2028-06-12"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "S Dnyom Rossii"
greeting_en = "Happy Russia Day"
look = "flag"
source = ["https://ppt.ru/news/proizvodstvennyy-kalendar/pravitelstvo-utverdilo-perenosy-vykhodnykh-v-2026-godu", "https://ppt.ru/news/proizvodstvennyy-kalendar/utverdili-grafik-vyhodnyh-i-prazdnichnyh-dney-na-2027-god", "https://www.consultant.ru/document/cons_doc_LAW_34683/98ef2900507766e70ff29c0b9d8e2353ea80a1cf/"]

[[holiday]]
country = "RU"
id = "unity_day"
name = "National Unity Day (Den' narodnogo edinstva)"
rule = "fixed(11,4)"
dates_2026 = ["2026-11-04"]
dates_2027 = ["2027-11-04"]
dates_2028 = ["2028-11-04"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "S Dnyom narodnogo edinstva"
greeting_en = "Happy Unity Day"
look = "flag"
source = ["https://ppt.ru/news/proizvodstvennyy-kalendar/pravitelstvo-utverdilo-perenosy-vykhodnykh-v-2026-godu", "https://www.kontur-extern.ru/info/normativ/document/1/508389-postanovlenie-pravitelstva-rf-ot-17-09-2026-n-1187", "https://www.consultant.ru/document/cons_doc_LAW_34683/98ef2900507766e70ff29c0b9d8e2353ea80a1cf/"]
```

Notes (RU):
- **Statutory holidays (Labour Code art. 112).** The New Year holidays are 1–6 and 8 Jan; Christmas is 7 Jan. The other holidays are 23 Feb, 8 Mar, 1 May, 9 May, 12 Jun and 4 Nov.
- **Transfer rule.** When one of the non-January holidays falls on a weekend, the next working day becomes a day off automatically. For weekend days that overlap the January holidays, the government decides by annual decree where they move.
- **2026 (decree No. 1466 of 24 Sep 2025, official).** Sat 3 Jan moves to Fri 9 Jan, and Sun 4 Jan moves to Thu 31 Dec 2026. Automatic transfers: 8 Mar (Sun) moves to Mon 9 Mar, and 9 May (Sat) moves to Mon 11 May. Resulting non-working stretches:
  - 31 Dec 2025 – 11 Jan 2026
  - 21–23 Feb
  - 7–9 Mar
  - 1–3 May
  - 9–11 May
  - 12–14 Jun
  - 4 Nov (single day)
  - 31 Dec 2026
- **2027 (decree No. 1187 of 17 Sep 2026, official).** Sat 2 Jan moves to Fri 5 Nov, Sun 3 Jan moves to Fri 31 Dec 2027, and Sat 20 Feb moves to Mon 22 Feb, so Sat 20 Feb becomes a working day. Automatic transfers: 1 May (Sat) moves to Mon 3 May, 9 May (Sun) moves to Mon 10 May, and 12 Jun (Sat) moves to Mon 14 Jun. Resulting non-working stretches:
  - 31 Dec 2026 – 10 Jan 2027
  - 21–23 Feb
  - 6–8 Mar
  - 1–3 May
  - 8–10 May
  - 12–14 Jun
  - 4–7 Nov
  - 31 Dec 2027
  - The Mintrud draft (Interfax, 30 Apr 2026) matched the signed decree.
- **2028 (no decree yet; checked 21 Oct 2026).** The transfer decree usually comes in September of the year before, so expect it around Sep 2027. Sat 1 Jan, Sun 2 Jan and Sat 8 Jan coincide with the New Year holidays, and the decree will move two of those weekend days to other dates in 2028. That is why `new_year_holidays` is tentative for 2028. Automatic transfer: 4 Nov (Sat) moves to Mon 6 Nov. Every other holiday falls on a weekday: 23 Feb (Wed), 8 Mar (Wed), 1 May (Mon), 9 May (Tue), 12 Jun (Mon). Labour Code art. 112 is unchanged as of its 25 May 2026 edition.
- **Disagreements.**
  - officeholidays' 2026 Russia page shows 2–3 Nov 2026 as holidays "compensated by Sun 1 Nov" and leaves out 4–5 Jan. Decree 1466 and the production calendars contradict it: 2 Nov is a normal working day and 3 Nov a shortened pre-holiday day. Ignore that page for 2026.
  - officeholidays' 2027 page matches decree 1187.
  - calendar.yoip.ru and calendar.mwmoskva.ru already show a 2028 transfer of 1 Jan to Mon 8 May (yoip says a government decree approved it). No 2028 decree exists: consultant.ru has no 2028 calendar and allcontract.ru says the decree is not published. Treat it as their guess.
- **Display.**
  - New Year fireworks belong to the night of 31 Dec→1 Jan only. Keep the greeting through 8 Jan, but don't show fireworks every night.
  - Christmas `eve` covers the Sochelnik (6 Jan) evening and midnight liturgy.
  - Victory Day salutes (salyut) are fired on the evening of 9 May itself.
- **Not federal days off.** Orthodox Easter (12 Apr 2026, 2 May 2027, 16 Apr 2028) is not a federal day off; some regions add their own days.

## Ethiopia (ET)

```toml
[[holiday]]
country = "ET"
id = "genna"
name = "Ethiopian Christmas (Genna / Lidet)"
rule = "listed"
dates_2026 = ["2026-01-07"]
dates_2027 = ["2027-01-07"]
dates_2028 = ["2028-01-07"]
tentative_2027 = false
tentative_2028 = false
eve = true
greeting = "Melkam Gena"
greeting_en = "Merry Christmas"
look = "candles"
source = ["https://www.officeholidays.com/countries/ethiopia/2026", "https://calendarific.com/holidays/2027/ET", "https://time.et/holidays", "https://calendarific.com/holidays/2028/ET", "https://www.fanamc.com/english/55"]

[[holiday]]
country = "ET"
id = "timkat"
name = "Epiphany (Timket / Timkat)"
rule = "listed"
dates_2026 = ["2026-01-19"]
dates_2027 = ["2027-01-19"]
dates_2028 = ["2028-01-20"]
tentative_2027 = false
tentative_2028 = false
eve = true
greeting = "Melkam Timket"
greeting_en = "Happy Timkat"
look = "none"
source = ["https://www.officeholidays.com/countries/ethiopia/2026", "https://calendarific.com/holidays/2027/ET", "https://time.et/holidays", "https://calendarific.com/holidays/2028/ET", "https://www.officeholidays.com/countries/ethiopia/2024"]

[[holiday]]
country = "ET"
id = "adwa_victory_day"
name = "Adwa Victory Day (Ye'Adwa Dil Beal)"
rule = "fixed(3,2)"
dates_2026 = ["2026-03-02"]
dates_2027 = ["2027-03-02"]
dates_2028 = ["2028-03-02"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Ye Adwa Dil Beal"
greeting_en = "Adwa Victory Day"
look = "flag"
source = ["https://www.metaappz.com/References/ethiopian_laws/federal/pr_1334_2024/en/txt", "https://www.officeholidays.com/countries/ethiopia/2026", "https://calendarific.com/holidays/2027/ET", "https://calendarific.com/holidays/2028/ET"]

[[holiday]]
country = "ET"
id = "eid_al_fitr"
name = "Eid al-Fitr (Id al-Fitr)"
rule = "listed"
dates_2026 = ["2026-03-20"]
dates_2027 = ["2027-03-09"]
dates_2028 = ["2028-02-27"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Melkam Eid"
greeting_en = "Happy Eid al-Fitr"
look = "eid"
source = ["https://allafrica.com/stories/202603200042.html", "https://www.officeholidays.com/countries/ethiopia/2026", "https://calendarific.com/holidays/2027/ET", "https://www.qppstudio.net/public-holidays/ethiopia.htm", "https://calendarific.com/holidays/2028/ET", "https://time.et/holidays", "https://en.wikipedia.org/wiki/Shawwal"]

[[holiday]]
country = "ET"
id = "siklet"
name = "Ethiopian Good Friday (Siklet)"
rule = "orthodox_easter(-2)"
dates_2026 = ["2026-04-10"]
dates_2027 = ["2027-04-30"]
dates_2028 = ["2028-04-14"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Siklet"
greeting_en = "Good Friday"
look = "none"
source = ["https://www.officeholidays.com/countries/ethiopia/2026", "https://www.officeholidays.com/countries/ethiopia/2027", "https://time.et/holidays", "https://calendarific.com/holidays/2028/ET"]

[[holiday]]
country = "ET"
id = "fasika"
name = "Ethiopian Easter (Fasika / Tinsae)"
rule = "orthodox_easter(+0)"
dates_2026 = ["2026-04-12"]
dates_2027 = ["2027-05-02"]
dates_2028 = ["2028-04-16"]
tentative_2027 = false
tentative_2028 = false
eve = true
greeting = "Melkam Fasika"
greeting_en = "Happy Easter"
look = "candles"
source = ["https://www.officeholidays.com/countries/ethiopia/2026", "https://calendarific.com/holidays/2027/ET", "https://time.et/holidays", "https://calendarific.com/holidays/2028/ET"]

[[holiday]]
country = "ET"
id = "labour_day"
name = "International Workers' Day (Ye Serategnoch Qen)"
rule = "fixed(5,1)"
dates_2026 = ["2026-05-01"]
dates_2027 = ["2027-05-01"]
dates_2028 = ["2028-05-01"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Ye Serategnoch Qen"
greeting_en = "Happy Labour Day"
look = "none"
source = ["https://www.metaappz.com/References/ethiopian_laws/federal/pr_1334_2024/en/txt", "https://www.officeholidays.com/countries/ethiopia/2026", "https://calendarific.com/holidays/2027/ET", "https://calendarific.com/holidays/2028/ET"]

[[holiday]]
country = "ET"
id = "patriots_victory_day"
name = "Patriots' Victory Day (Arbegnoch Qen)"
rule = "fixed(5,5)"
dates_2026 = ["2026-05-05"]
dates_2027 = ["2027-05-05"]
dates_2028 = ["2028-05-05"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Ye Arbegnoch Qen"
greeting_en = "Patriots' Victory Day"
look = "flag"
source = ["https://www.metaappz.com/References/ethiopian_laws/federal/pr_1334_2024/en/txt", "https://www.officeholidays.com/countries/ethiopia/2026", "https://calendarific.com/holidays/2027/ET", "https://calendarific.com/holidays/2028/ET"]

[[holiday]]
country = "ET"
id = "eid_al_adha"
name = "Eid al-Adha (Arefa)"
rule = "listed"
dates_2026 = ["2026-05-27"]
dates_2027 = ["2027-05-17"]
dates_2028 = ["2028-05-05"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Melkam Arefa"
greeting_en = "Happy Eid al-Adha"
look = "eid"
source = ["https://www.thereporterethiopia.com/50898/", "https://www.officeholidays.com/countries/ethiopia/2026", "https://calendarific.com/holidays/2027/ET", "https://www.qppstudio.net/public-holidays/ethiopia.htm", "https://calendarific.com/holidays/2028/ET", "https://time.et/holidays", "https://en.wikipedia.org/wiki/Dhu_al-Hijja"]

[[holiday]]
country = "ET"
id = "derg_downfall_day"
name = "Downfall of the Derg (Ginbot 20)"
rule = "fixed(5,28)"
dates_2026 = ["2026-05-28"]
dates_2027 = ["2027-05-28"]
dates_2028 = ["2028-05-28"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Ginbot 20"
greeting_en = "Downfall of the Derg"
look = "none"
source = ["https://www.officeholidays.com/countries/ethiopia/2026", "https://time.et/holidays", "https://calendarific.com/holidays/2027/ET", "https://calendarific.com/holidays/2028/ET"]

[[holiday]]
country = "ET"
id = "mawlid"
name = "Mawlid, the Prophet's Birthday (Mewlid)"
rule = "listed"
dates_2026 = ["2026-08-25"]
dates_2027 = ["2027-08-14"]
dates_2028 = ["2028-08-03"]
tentative_2027 = true
tentative_2028 = true
eve = true
greeting = "Melkam Mawlid"
greeting_en = "Mawlid of the Prophet"
look = "none"
source = ["https://www.officeholidays.com/countries/ethiopia/2026", "https://www.fanamc.com/english/pm-abiy-extends-mawlid-greetings-calls-on-ethiopians-to-embrace-purpose-sacrifice-and-service/", "https://calendarific.com/holidays/2027/ET", "https://www.qppstudio.net/public-holidays/ethiopia.htm", "https://calendarific.com/holidays/2028/ET", "https://time.et/holidays", "https://www.qppstudio.net/global-holidays-observances/mawlid-an-nabi-the-prophets-birthday.htm"]

[[holiday]]
country = "ET"
id = "enkutatash"
name = "Ethiopian New Year (Enkutatash)"
rule = "listed"
dates_2026 = ["2026-09-11"]
dates_2027 = ["2027-09-12"]
dates_2028 = ["2028-09-11"]
tentative_2027 = false
tentative_2028 = false
eve = false
greeting = "Melkam Addis Amet"
greeting_en = "Happy Ethiopian New Year"
look = "none"
source = ["https://time.et/holidays", "https://www.officeholidays.com/countries/ethiopia/2027", "https://calendarific.com/holidays/2027/ET", "https://calendarific.com/holidays/2028/ET", "https://www.officeholidays.com/countries/ethiopia/2024"]

[[holiday]]
country = "ET"
id = "meskel"
name = "Finding of the True Cross (Meskel)"
rule = "listed"
dates_2026 = ["2026-09-27"]
dates_2027 = ["2027-09-28"]
dates_2028 = ["2028-09-27"]
tentative_2027 = false
tentative_2028 = false
eve = true
greeting = "Melkam Meskel"
greeting_en = "Happy Meskel"
look = "bonfire"
source = ["https://time.et/holidays", "https://calendarific.com/holiday/ethiopia/finding-true-cross-day", "https://calendarific.com/holidays/2027/ET", "https://calendarific.com/holidays/2028/ET", "https://www.officeholidays.com/countries/ethiopia/2024"]
```

Notes (ET):
- **How the dates are derived.**
  - The church holidays are fixed in the Ethiopian calendar:
    - Genna: Tahsas 29
    - Timket: Tir 11
    - Enkutatash: Meskerem 1
    - Meskel: Meskerem 17
  - The civic days are fixed in the Gregorian calendar (Proclamation 1334/2024): Adwa 2 Mar, Labour Day 1 May, Patriots' Day 5 May.
- **Leap-year shift.** The Ethiopian leap day (Pagume 6) falls in the Gregorian year before a Gregorian leap year. It pushes Enkutatash to 12 Sep and Meskel to 28 Sep in that year, and Timket (and potentially Genna) to 20 Jan / 8 Jan in the following leap year.
  - 2026: Enkutatash 2019 E.C. is 11 Sep and Meskel is 27 Sep.
  - 2027: 2019 E.C. is a leap year, so Pagume 6 is 11 Sep 2027. Enkutatash 2020 E.C. is therefore **12 Sep** and Meskel **28 Sep**.
  - Genna (7 Jan) and Timket (19 Jan) are unaffected in both 2026 and 2027. The next shift is Jan 2028 (Timket 20 Jan).
  - 2028: 2020 E.C. follows the leap year 2019 E.C., as 2016 E.C. did in Jan 2024. Timket moves to **20 Jan** (Tir 11; officeholidays had 20 Jan 2024). Genna stays on **7 Jan** in practice: FBC reported Genna on 7 Jan 2024, and calendarific lists 7 Jan 2028. 2020 E.C. is not a leap year, so Enkutatash 2021 E.C. is **11 Sep 2028** and Meskel **27 Sep 2028**.
  - Siklet 14 Apr and Fasika 16 Apr 2028; Orthodox and Western Easter coincide that year.
- **Disagreement (Meskel 2027).** officeholidays' 2027 page lists Meskel on 27 Sep while giving Enkutatash 12 Sep, which is internally inconsistent. Calendarific gives 28 Sep, and the Meskerem 17 arithmetic gives 28 Sep. Use 28 Sep.
- **Disagreement (Genna 2028).** time.et lists Sat 8 Jan 2028, the plain Tahsas 29 arithmetic. Calendarific gives 7 Jan, and the 2024 analog was observed on 7 Jan. Use 7 Jan.
- **Islamic days in 2026.** All three have passed and match Ethiopian reports:
  - Eid al-Fitr, Fri 20 Mar (ENA, "1447th Eid al-Fitr", 20 Mar).
  - Eid al-Adha (Arefa), Wed 27 May (The Reporter: Addis prayers "on May 27th"). metaappz said 26 May, which is wrong.
  - Mawlid, Tue 25 Aug: officeholidays, timeanddate and TrueCalendar give 25 Aug, and the PM's greeting was dated 24 Aug. HolidayDB and calendarific list 26 Aug, so there is a slight doubt.
- **Islamic days in 2027 (moon sighting, ±1 day).** Ethiopia has tracked the Saudi dates in recent years. Majority of the listing sites (calendarific, qppstudio, time.et): Eid al-Fitr 10 Mar, Eid al-Adha 17 May, Mawlid 15 Aug. officeholidays has 9 Mar and 16 May. Checked 21 Oct 2026: still not published; officeholidays and time.et are unchanged.
  - The Umm al-Qura tables (Wikipedia) give 9 Mar, 17 May and 14 Aug 2027. Ethiopia's three 2026 days all matched Umm al-Qura, so 9 Mar and 14 Aug are as likely as the listed 10 Mar and 15 Aug.
- **Islamic days in 2028 (moon sighting, ±1 day).** Calendarific (ET 2028) and time.et agree: Eid al-Fitr Sun 27 Feb, Eid al-Adha Fri 5 May (the same day as Patriots' Victory Day), Mawlid Thu 3 Aug. Umm al-Qura agrees on Eid al-Adha (Dhu al-Hijjah 1449 starts 26 Apr) and Mawlid (Rabi' al-Awwal 1450 starts 23 Jul).
  - Disagreement (Eid al-Fitr): Umm al-Qura (Wikipedia's Shawwal table, publicholidays.ae, truecalendar) gives Sat 26 Feb. The new moon is 25 Feb 10:37 UTC, only about 5 hours before sunset in Addis and Mecca, so no crescent is visible that evening; a sighting-based Eid lands on 27 Feb. It is the same pattern as 2027's 9 vs 10 Mar.
  - Disagreement (Eid al-Adha): the tabular Hijri calendar (islamic.app) gives Thu 4 May.
  - Disagreement (Mawlid): the tabular calendar (islamic.app, calendardate) gives Wed 2 Aug. Strict local sighting could give Fri 4 Aug, because the moon is only about 13 hours old at sunset on 22 Jul.
- **Derg Downfall Day is doubtful.** Proclamation 1334/2024 (June 2024) lists 12 closed-office holidays: 4 national (11 Sep, 2 Mar, 1 May, 5 May) and 8 religious (Meskel, Genna, Timket, Siklet, Fasika, Mawlid, Eid al-Fitr, Eid al-Adha). Ginbot 20 is not among them. Aggregators (officeholidays, time.et, calendarific) still list 28 May. I kept the entry as you asked, but it probably no longer carries a day off; drop it or keep it as a greeting-only day.
- **Memorial days (offices open), not included.** Martyrs' Day (Yekatit 12, ~19/20 Feb) and Nations, Nationalities and Peoples' Day (9 Dec).
- **One-off 2026 holiday.** 1 Jun 2026 (7th general election day) was reportedly declared a public holiday (Remofirst; officeholidays shows an unnamed holiday). It does not recur and was not verified against a government notice.
- **Eve customs.**
  - Genna: overnight church vigil on 6 Jan.
  - Timket: Ketera on 18 Jan, the main procession taking the tabots to the water.
  - Fasika: Saturday-night vigil; the fast breaks after the midnight service.
  - Meskel: the Demera bonfire is lit on the evening of 26 Sep 2026 / 27 Sep 2027 / 26 Sep 2028.
  - Islamic days start at sunset.
  - Enkutatash eve torches (chibo) are regional, so it is set to `eve = false`.
- **Greetings.**
  - "Inkwan aderesachihu" is the all-purpose Amharic holiday greeting.
  - "Eid Mubarak" is equally common for both Eids.
  - "Melkam Timket" can be "Melkam Yetimket Beal" in full.
  - Siklet and Ginbot 20 use the plain name.

## Review changes (21 Oct 2026)

- CN Spring Festival 2028: the likely eight-day block, 25 Jan–1 Feb 2028 (tentative), instead
  of the four statutory days, until the State Council notice.
- ET 2027: Eid al-Fitr 9 Mar and Mawlid 14 Aug (Umm al-Qura, which matched every Ethiopian
  Islamic date in 2026); Eid al-Adha stays 17 May. Still tentative.
