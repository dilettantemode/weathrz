# World holidays — group E: VN, KR, AU

Compiled 23 Oct 2026 for the weathrz holiday greetings and decorations (2026, 2027, 2028).
All three countries complete (VN, KR, AU); counts at the end.

**Schema conventions used below**

- Same as group C / group D. `rule` is one string from the fixed grammar; `listed` covers lunar,
  lunisolar and announced dates. Weekday tokens are `mon tue wed thu fri sat sun`.
- `dates_*` hold the holiday day itself; multi-day blocks list every day. Substitute / observed /
  bridge days are in Notes.
- `tentative_2027 = true` / `tentative_2028 = true` mean the **date** may still move or the
  official list for that year is not published yet; each country's Notes say which.
- `observance = true` marks a day that is not a day off (half-mast days, other widely marked days).
- `look = "half_mast"` is the new look for days the national flag is officially flown at
  half-mast. When a half-mast day is also a public holiday listed here, it is a separate entry with
  id `<holiday_id>_half_mast`, so the holiday keeps its own greeting and look.
- Easter Sunday is 2026-04-05, 2027-03-28 and 2028-04-16 (same as group C).
- Lunar New Year (China, generator `LUNAR_NEW_YEAR`) is 2026-02-17, 2027-02-06 and 2028-01-26.

**Primary sources checked**

| Country | Official source | Cross-check |
|---|---|---|
| VN | [Labour Code 2019 (45/2019/QH14) art. 112](https://vanban.chinhphu.vn/?pageid=27160&docid=198540); Resolution 28/2026/QH16 (24 Nov, via [Hà Tĩnh portal](https://hatinh.gov.vn/vi/bai-viet/ngay-van-hoa-viet-nam---2411-nguoi-lao-dong-duoc-nghi-huong-nguyen-luong) and [thuvienphapluat](https://thuvienphapluat.vn/van-ban/Van-hoa-Xa-hoi/Nghi-quyet-28-2026-QH16-phat-trien-van-hoa-Viet-Nam-704355.aspx)); MOHA notices 9441/TB-BNV (2026, via [Hà Tĩnh portal](https://hatinh.gov.vn/vi/bai-viet/lich-nghi-tet-nguyen-dan-binh-ngo-2026-va-cac-chi-dao-to-chuc-vui-xuan-don-tet-072633939)) and 11483/TB-BNV (2027, via [Hà Tĩnh portal](https://hatinh.gov.vn/vi/bai-viet/tet-nguyen-dan-2027-cong-chuc-vien-chuc-duoc-nghi-7-ngay-lien-tuc)); Government Office 10065/VPCP-KGVX via [VietnamPlus](https://www.vietnamplus.vn/chot-lich-nghi-dip-ngay-van-hoa-viet-nam-2026-tet-nguyen-dan-va-quoc-khanh-nam-2027-post1139707.vnp) (state news agency) | officeholidays.com [2026](https://www.officeholidays.com/countries/vietnam/2026) / [2027](https://www.officeholidays.com/countries/vietnam/2027); timeanddate.com [2028](https://www.timeanddate.com/holidays/vietnam/2028) via Wayback 2 Mar 2026; xemlicham.com lịch vạn niên day pages; own Meeus new-moon computation |
| KR | KASA (우주항공청) [2027 월력요항 release, 29 Jun 2026](https://www.kasa.go.kr/prog/plcyBrf/brief/kor/sub01_01_04/view.do?plcyBrfNo=431); KASI calendar data [2026](https://astro.kasi.re.kr/kor/life/post/calendarData?year=2026) (generated 2024) / [2027](https://astro.kasi.re.kr/kor/life/post/calendarData?year=2027) / [2028](https://astro.kasi.re.kr/kor/life/post/calendarData?year=2028) (pre-release, generated 30 Jun 2026); [MOIS national flag page](https://www.mois.go.kr/frt/sub/a06/b08/nationalIcon_2_3/screen.do) (flag days, half-mast) | officeholidays.com [2026](https://www.officeholidays.com/countries/south-korea/2026) / [2027](https://www.officeholidays.com/countries/south-korea/2027); timeanddate.com [2027](https://www.timeanddate.com/holidays/south-korea/2027) via Wayback 3 Mar 2026 (no 2028 snapshot); SBS Biz, Segye, Herald (law changes); own Meeus new-moon computation |
| AU | [Queensland Government public holidays](https://www.qld.gov.au/recreation/travel/holidays/public) (2026, 2027, 2028); [NSW Government public holidays](https://www.nsw.gov.au/about-nsw/public-holidays) (2026, 2027); PM&C Commonwealth Flag Network notices: [Anzac Day 2026](https://www.pmc.gov.au/honours-and-symbols/australian-national-symbols/australian-national-flag/flag-network/2026-04-25), [Remembrance Day 2025](https://www.pmc.gov.au/honours-and-symbols/australian-national-symbols/australian-national-flag/flag-network/2025-11-11); Fair Work Act 2009 s115 ([fairwork.gov.au public holidays](https://www.fairwork.gov.au/employment-conditions/public-holidays), not reachable, see below) | officeholidays.com [2026](https://www.officeholidays.com/countries/australia/2026) / [2027](https://www.officeholidays.com/countries/australia/2027); naidoc.org.au and council listings (NAIDOC Week); publicholidays.com.au (SA Proclamation Day) |

Fetch problems (23 Oct 2026): **fairwork.gov.au timed out** on every try (HTTP/2 stream errors,
then 60 s timeouts over HTTP/1.1 and WebFetch), so Australian dates rest on the NSW and Queensland
government pages (Queensland publishes 2028); business.vic.gov.au returned 403. pmc.gov.au blocks
scripted fetches (Incapsula) but WebFetch read the individual Flag Network notices cited; its
notice index and flag-protocol guidance page could not be read. law.go.kr renders by script and
returned an overload page, so Korean statutes are cited through the MOIS flag page, the KASA
release and news reports. The MOHA notice texts (Vietnam) are not on moha.gov.vn's indexed pages;
they were read as reproduced by the Hà Tĩnh provincial portal (hatinh.gov.vn) and VietnamPlus
(state news agency). vbpl.vn redirected (308) and moet.gov.vn reset the connection.
officeholidays.com has no 2028 pages (404 for VN, KR, AU). timeanddate.com was read through
Wayback snapshots: VN 2028 (2 Mar 2026) and KR 2027 (3 Mar 2026); there is no KR or AU 2028
snapshot. KASI's 2026 calendar data was generated in July 2024, before the April 2026 holiday
law changes, so it lacks 노동절 and shows 제헌절 as not a day off (see KR Notes).

---

## Vietnam (VN)

```toml
[[holiday]]
country = "VN"
id = "new_year"
name = "Tết Dương lịch (New Year's Day)"
rule = "fixed(1,1)"
dates_2026 = ["2026-01-01"]
dates_2027 = ["2027-01-01"]
dates_2028 = ["2028-01-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = true
greeting = "Chúc mừng năm mới"
greeting_en = "Happy New Year"
look = "fireworks_night"
source = ["https://vanban.chinhphu.vn/?pageid=27160&docid=198540", "https://baovanhoa.vn/doi-song/chinh-phu-dong-y-phuong-an-nghi-tet-duong-lich-2026-4-ngay-192438.html", "https://www.officeholidays.com/countries/vietnam/2027", "https://www.timeanddate.com/holidays/vietnam/2028"]

[[holiday]]
country = "VN"
id = "tet"
name = "Tết Nguyên đán / Tết Âm lịch (Lunar New Year)"
rule = "listed"
dates_2026 = ["2026-02-16", "2026-02-17", "2026-02-18", "2026-02-19", "2026-02-20"]
dates_2027 = ["2027-02-04", "2027-02-05", "2027-02-06", "2027-02-07", "2027-02-08"]
dates_2028 = ["2028-01-24", "2028-01-25", "2028-01-26", "2028-01-27", "2028-01-28"]
tentative_2027 = false
tentative_2028 = true
observance = false
eve = false
greeting = "Chúc mừng năm mới"
greeting_en = "Happy Lunar New Year · Year of the {animal}"
look = "lunar_new_year"
source = ["https://vanban.chinhphu.vn/?pageid=27160&docid=198540", "https://hatinh.gov.vn/vi/bai-viet/lich-nghi-tet-nguyen-dan-binh-ngo-2026-va-cac-chi-dao-to-chuc-vui-xuan-don-tet-072633939", "https://hatinh.gov.vn/vi/bai-viet/tet-nguyen-dan-2027-cong-chuc-vien-chuc-duoc-nghi-7-ngay-lien-tuc", "https://www.vietnamplus.vn/chot-lich-nghi-dip-ngay-van-hoa-viet-nam-2026-tet-nguyen-dan-va-quoc-khanh-nam-2027-post1139707.vnp", "https://hatinh.gov.vn/vi/bai-viet/thong-nhat-phuong-an-nghi-tet-at-ty-2025-lien-9-ngay", "https://www.timeanddate.com/holidays/vietnam/2028"]

[[holiday]]
country = "VN"
id = "hung_kings"
name = "Giỗ Tổ Hùng Vương (Hung Kings Commemoration Day)"
rule = "listed"
dates_2026 = ["2026-04-26"]
dates_2027 = ["2027-04-16"]
dates_2028 = ["2028-04-04"]
tentative_2027 = false
tentative_2028 = true
observance = false
eve = false
greeting = "Giỗ Tổ Hùng Vương"
greeting_en = "Hung Kings Commemoration"
look = "none"
source = ["https://vanban.chinhphu.vn/?pageid=27160&docid=198540", "https://www.officeholidays.com/countries/vietnam/2026", "https://www.officeholidays.com/countries/vietnam/2027", "https://www.timeanddate.com/holidays/vietnam/2028"]

[[holiday]]
country = "VN"
id = "victory_day"
name = "Ngày Chiến thắng / Ngày Giải phóng miền Nam, thống nhất đất nước (Victory Day / Reunification Day)"
rule = "fixed(4,30)"
dates_2026 = ["2026-04-30"]
dates_2027 = ["2027-04-30"]
dates_2028 = ["2028-04-30"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Mừng ngày thống nhất đất nước"
greeting_en = "{nth since 1975} Reunification Day"
look = "flag"
source = ["https://vanban.chinhphu.vn/?pageid=27160&docid=198540", "https://www.officeholidays.com/countries/vietnam/2027", "https://www.timeanddate.com/holidays/vietnam/2028"]

[[holiday]]
country = "VN"
id = "labour_day"
name = "Ngày Quốc tế Lao động (International Labour Day)"
rule = "fixed(5,1)"
dates_2026 = ["2026-05-01"]
dates_2027 = ["2027-05-01"]
dates_2028 = ["2028-05-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Mừng ngày Quốc tế Lao động"
greeting_en = "Happy Labour Day"
look = "none"
source = ["https://vanban.chinhphu.vn/?pageid=27160&docid=198540", "https://www.officeholidays.com/countries/vietnam/2027", "https://www.timeanddate.com/holidays/vietnam/2028"]

[[holiday]]
country = "VN"
id = "national_day"
name = "Quốc khánh (National Day)"
rule = "listed"
dates_2026 = ["2026-09-01", "2026-09-02"]
dates_2027 = ["2027-09-02", "2027-09-03"]
dates_2028 = ["2028-09-01", "2028-09-02"]
tentative_2027 = false
tentative_2028 = true
observance = false
eve = false
greeting = "Mừng Quốc khánh 2/9"
greeting_en = "Happy {nth since 1945} National Day, Vietnam"
look = "flag+fireworks_night"
source = ["https://vanban.chinhphu.vn/?pageid=27160&docid=198540", "https://hatinh.gov.vn/vi/bai-viet/lich-nghi-tet-nguyen-dan-binh-ngo-2026-va-cac-chi-dao-to-chuc-vui-xuan-don-tet-072633939", "https://hatinh.gov.vn/vi/bai-viet/tet-nguyen-dan-2027-cong-chuc-vien-chuc-duoc-nghi-7-ngay-lien-tuc", "https://www.vietnamplus.vn/chot-lich-nghi-dip-ngay-van-hoa-viet-nam-2026-tet-nguyen-dan-va-quoc-khanh-nam-2027-post1139707.vnp", "https://www.timeanddate.com/holidays/vietnam/2028"]

[[holiday]]
country = "VN"
id = "vietnam_culture_day"
name = "Ngày Văn hóa Việt Nam (Vietnam Culture Day)"
rule = "fixed(11,24)"
dates_2026 = ["2026-11-24"]
dates_2027 = ["2027-11-24"]
dates_2028 = ["2028-11-24"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Mừng Ngày Văn hóa Việt Nam"
greeting_en = "Vietnam Culture Day"
look = "none"
source = ["https://hatinh.gov.vn/vi/bai-viet/ngay-van-hoa-viet-nam---2411-nguoi-lao-dong-duoc-nghi-huong-nguyen-luong", "https://www.vietnamplus.vn/chot-lich-nghi-dip-ngay-van-hoa-viet-nam-2026-tet-nguyen-dan-va-quoc-khanh-nam-2027-post1139707.vnp", "https://thuvienphapluat.vn/van-ban/Van-hoa-Xa-hoi/Nghi-quyet-28-2026-QH16-phat-trien-van-hoa-Viet-Nam-704355.aspx", "https://www.officeholidays.com/countries/vietnam/2027"]
```

Observances (not days off):

```toml
[[holiday]]
country = "VN"
id = "kitchen_gods_day"
name = "Tết Ông Công Ông Táo (Kitchen Gods' Day, 23rd of the 12th lunar month)"
rule = "listed"
dates_2026 = ["2026-02-10"]
dates_2027 = ["2027-01-30"]
dates_2028 = ["2028-01-19"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Tiễn ông Công ông Táo về trời"
greeting_en = "Kitchen Gods' Day"
look = "none"
source = ["https://www.xemlicham.com/am-lich/nam/2027/thang/1/ngay/30", "https://www.xemlicham.com/am-lich/nam/2028/thang/1/ngay/19"]

[[holiday]]
country = "VN"
id = "international_womens_day"
name = "Ngày Quốc tế Phụ nữ (International Women's Day)"
rule = "fixed(3,8)"
dates_2026 = ["2026-03-08"]
dates_2027 = ["2027-03-08"]
dates_2028 = ["2028-03-08"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Chúc mừng ngày 8/3"
greeting_en = "Happy Women's Day"
look = "none"
source = ["https://vpcp.chinhphu.vn/cong-ttdt-chinh-phu-chuc-mung-ngay-quoc-te-phu-nu-8-3-11525634.htm", "https://www.vietnamplus.vn/ron-rang-khong-khi-chao-mung-ngay-quoc-te-phu-nu-83-post1015804.vnp"]

[[holiday]]
country = "VN"
id = "vesak"
name = "Lễ Phật đản (Vesak, 15th of the 4th lunar month)"
rule = "listed"
dates_2026 = ["2026-05-31"]
dates_2027 = ["2027-05-20"]
dates_2028 = ["2028-05-09"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Kính mừng Phật đản"
greeting_en = "Happy Vesak"
look = "lanterns"
source = ["https://www.xemlicham.com/am-lich/nam/2027/thang/5/ngay/20", "https://www.xemlicham.com/am-lich/nam/2028/thang/5/ngay/9"]

[[holiday]]
country = "VN"
id = "mid_autumn"
name = "Tết Trung thu (Mid-Autumn Festival, 15th of the 8th lunar month)"
rule = "listed"
dates_2026 = ["2026-09-25"]
dates_2027 = ["2027-09-15"]
dates_2028 = ["2028-10-03"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Trung thu vui vẻ"
greeting_en = "Happy Mid-Autumn Festival"
look = "lanterns+harvest_moon"
source = ["https://www.xemlicham.com/am-lich/nam/2027/thang/9/ngay/1", "https://www.xemlicham.com/am-lich/nam/2028/thang/10/ngay/3"]

[[holiday]]
country = "VN"
id = "vietnamese_womens_day"
name = "Ngày Phụ nữ Việt Nam (Vietnamese Women's Day)"
rule = "fixed(10,20)"
dates_2026 = ["2026-10-20"]
dates_2027 = ["2027-10-20"]
dates_2028 = ["2028-10-20"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Chúc mừng ngày Phụ nữ Việt Nam 20/10"
greeting_en = "Vietnamese Women's Day"
look = "none"
source = ["https://www.timeanddate.com/holidays/vietnam/2028"]

[[holiday]]
country = "VN"
id = "teachers_day"
name = "Ngày Nhà giáo Việt Nam (Vietnamese Teachers' Day)"
rule = "fixed(11,20)"
dates_2026 = ["2026-11-20"]
dates_2027 = ["2027-11-20"]
dates_2028 = ["2028-11-20"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Chúc mừng ngày Nhà giáo Việt Nam 20/11"
greeting_en = "Happy Teachers' Day"
look = "none"
source = ["https://xaydungchinhsach.chinhphu.vn/toan-van-quyet-dinh-so-167-hdbt-ve-ngay-nha-giao-viet-nam-119231120070916265.htm", "https://vanban.chinhphu.vn/default.aspx?pageid=27160&docid=6402"]
```

Half-mast days: **none** (see Country notes).

**Notes (VN)**

- **Legal basis.** Labour Code 2019 (Bộ luật Lao động, Law 45/2019/QH14) art. 112: 1 Jan (1 day),
  Tết (5 days), 10/3 lunar Hùng Kings (1), 30/4 (1), 1/5 (1), National Day 2/9 **plus one
  adjacent day before or after** (2) — 11 days. **New from 2026:** National Assembly Resolution
  28/2026/QH16 (passed 24 Apr 2026, in force 1 Jul 2026) makes **24 Nov "Ngày Văn hóa Việt Nam"**
  a day off with full pay — 12 days. A Labour Code amendment to write 24/11 into art. 112 was in
  appraisal (Ministry of Justice, Aug 2026) and is not needed for the day to apply. Each year the
  Prime Minister (in practice a Deputy PM, relayed by the Government Office) picks which Tết days
  and which National Day neighbour; the Ministry of Home Affairs (Bộ Nội vụ, MOHA) then issues the
  notice. Private employers choose their own Tết split from three MOHA options (see 2027).
- **Weekend rule.** A holiday on a weekly rest day is compensated on the next working day.
  Substitute days in 2026–2028: Hùng Kings **Mon 27 Apr 2026** (26 Apr is a Sunday);
  1 May 2027 is a Saturday → **Mon 3 May 2027**; New Year 2028 is a Saturday → **Mon 3 Jan 2028**;
  30 Apr 2028 is a Sunday → the 30/4 + 1/5 pair becomes **Mon 1 + Tue 2 May 2028**; National Day
  2 Sep 2028 is a Saturday → **Mon 4 Sep 2028**.
- **2026 (official).** New Year: 4-day break 1–4 Jan, with Fri 2 Jan swapped against Sat 10 Jan
  (Government Office 12729/VPCP-KGVX, 25 Dec 2025). Tết Bính Ngọ: the 5 statutory days are
  **Mon 16 Feb (29 tháng Chạp, the eve — the 12th month had 29 days) to Fri 20 Feb (mùng 4)**,
  i.e. 1 day before + 4 after; civil servants were off **Sat 14 – Sun 22 Feb** (9 days; MOHA notice
  9441/TB-BNV, 16 Oct 2025). National Day: **Tue 1 + Wed 2 Sep**, with Mon 31 Aug swapped against
  Sat 22 Aug → 5-day break 29 Aug – 2 Sep. Culture Day: **Tue 24 Nov**, a single day, no swap
  (Government Office 10065/VPCP-KGVX, 2 Oct 2026; MOHA had first proposed a 21–24 Nov block).
- **2027 (official for Tết and National Day).** MOHA notice **11483/TB-BNV, 9 Oct 2026**, after
  10065/VPCP-KGVX: Tết Đinh Mùi = 5 statutory days, **2 before + 3 after** — Thu 4 Feb (28 Chạp),
  Fri 5 Feb (29 Chạp, the eve), Sat 6 Feb (mùng 1), Sun 7, Mon 8 Feb; two of them fall on the
  weekend, so civil servants also get Tue 9 + Wed 10 Feb → 7-day break **4–10 Feb**. National Day:
  **Thu 2 + Fri 3 Sep** → 4-day break 2–5 Sep. Enterprises pick 1+4, 2+3 or 3+2 days around Tết
  and 1 or 3 Sep as the National Day neighbour. Culture Day 2027 is Wed 24 Nov; MOHA proposed no
  swap.
- **2028 (not published; tentative).** Tết Mậu Thân: mùng 1 = **Wed 26 Jan 2028**, eve Tue 25 Jan
  (29 Chạp; the 12th month again has 29 days). Exact precedent: in 2025 mùng 1 was also a
  Wednesday after a 29-day 12th month, and the 5 days were Mon 27 – Fri 31 Jan 2025 (2 before +
  3 after) inside a 9-day break. The same shape gives **Mon 24 – Fri 28 Jan 2028** (break Sat 22 –
  Sun 30 Jan), which is what is listed. **Disagreement:** timeanddate's 2028 page lists Tue 25 –
  Sun 30 Jan (1 before + 4 after). National Day: 2 Sep is a Saturday; the 2023 precedent (also a
  Saturday) used Fri 1 Sep as the neighbour with Mon 4 Sep in lieu, so **1–2 Sep** is listed;
  timeanddate lists only 2 Sep + 4 Sep. MOHA has to publish the 2028 notice by about Oct 2027.
- **Cross-check 2026/2027.** officeholidays.com agrees on every fixed day, Hùng Kings (shown on the
  observed Mon 27 Apr 2026) and Culture Day, and shows 31 Aug – 2 Sep 2026. **Disagreement:** its
  2027 Tết rows are Fri 5 – Tue 9 Feb (1 + 4), not MOHA's 4–8 Feb (2 + 3); MOHA is used.
- **Lunar dates and the meridian.** Vietnam's calendar is computed for UTC+7 (105°E), China's for
  UTC+8, so a new moon between 16:00 and 17:00 UTC starts the month a day earlier in Vietnam. Own
  computation (Meeus new moons, ΔT ≈ 70 s) for Dec 2025 – Jan 2029: **no new moon falls in that
  hour**, so in 2026–2028 every Vietnamese lunar date here is the **same as China's** (Tết 17 Feb
  2026 / 6 Feb 2027 / 26 Jan 2028 = generator `LUNAR_NEW_YEAR`; Mid-Autumn 25 Sep / 15 Sep / 3 Oct).
  Closest calls: 12 Aug 2026 17:36 UTC (00:36 on 13 Aug in Vietnam) and 31 Aug 2027 17:41 UTC
  (00:41 on 1 Sep) — both already past midnight in Vietnam, so no shift. 2028 has the same leap 5th
  month in both calendars. xemlicham.com confirms 1 Sep 2027 = 1/8 Đinh Mùi (Mid-Autumn
  15 Sep 2027; one blog table saying 14 Sep is wrong). A known past case of a different Tết is
  2007 (17 Feb in Vietnam, 18 Feb in China).
- **Hùng Kings** is 10/3 lunar: 3rd month starts 17 Apr 2026, 7 Apr 2027, 26 Mar 2028, giving
  26 Apr 2026 (Sun), 16 Apr 2027 (Fri), 4 Apr 2028 (Tue). officeholidays (2026/2027) and timeanddate
  (2028; also confirmed by lịch vạn niên rows 9 Mar 2028 = 14/2 and 3 May 2028 = 9/4) agree. The
  date itself will not move (the lunar calendar is computed, not proclaimed); 2028 is flagged
  tentative only because the Government's 2028 holiday arrangement is not out yet.
- **Observance dates.** Ông Công Ông Táo is 23/12 lunar (12th month starts 19 Jan 2026, 8 Jan 2027,
  28 Dec 2027). Phật đản follows the Vietnam Buddhist Sangha's 15/4 lunar (same as Singapore's
  Vesak). These lunar dates are deterministic, hence `tentative = false`; the xemlicham.com day
  pages cited confirm 30 Jan 2027 = 23/12, 19 Jan 2028 = 23/12, 20 May 2027 = 15/4,
  9 May 2028 = 15/4 and 3 Oct 2028 = 15/8.
- **Looks.** Tết uses `lunar_new_year`; at Giao thừa (midnight of the eve) every province fires an
  official fireworks show, which the look's night scene can carry. National Day gets
  `flag+fireworks_night` (Hanoi fires fireworks on the night of 2/9 most years). 30/4 gets `flag`
  (fireworks in Ho Chi Minh City are common but not every year). Phật đản gets `lanterns` for the
  floating lotus lanterns (hoa đăng); Mid-Autumn gets `lanterns` (children's star lanterns, lion
  dances). Tết has `eve = false` because the eve (29 Chạp, Giao thừa night) is already one of the
  listed days; the evening before the first listed day is not part of Tết.
- **Left out.** Vu Lan (15/7 lunar, parents' day), Tết Đoan Ngọ (5/5 lunar) and War Invalids and
  Martyrs' Day (27/7, candle-lighting at martyrs' cemeteries — a `candles` candidate if a seventh
  observance is wanted) are widely known but less universal. Ngày Gia đình Việt Nam (28/6) is
  minor. Valentine's Day is commercial.

### Country notes (VN)

- **Christmas / New Year:** **1 Jan is a public holiday** (Tết Dương lịch) and big cities hold
  countdown events, often with fireworks — keep the New Year countdown. **Christmas is not a
  holiday**, but it is widely marked in cities (Hanoi's St Joseph's Cathedral, Saigon's Notre-Dame,
  street lights and mall decorations; ~7% of the population is Catholic) — keeping the 15–26 Dec
  lights is fine.
- **Half-mast:** **none fixed.** The flag is flown at half-mast (cờ rủ, with a black mourning band,
  to 2/3 of the pole) only during a declared Quốc tang (state funeral, 2 days) under Decree
  105/2012/NĐ-CP art. 10, case by case. War Invalids and Martyrs' Day (27/7) and other
  commemorations are not half-mast days; on national holidays the flag is flown full-mast and
  households are asked to hang it.
- **Flag (cờ đỏ sao vàng):** 2:3, **red field (#DA251D) with one large five-pointed yellow star
  (#FFFF00) in the centre**, point up, the star spanning a bit over half the flag's height. For
  9×3: all 27 cells red, one yellow `★` in the middle cell (row 2, col 5); nothing else.
- **Geocoder language:** `vi`.
- **Anchor cities** (Vietnam is long and thin: 1,650 km north–south, as narrow as 50 km at Quảng
  Bình):
  - Lào Cai 22.49, 103.97 (north, China border)
  - Móng Cái 21.52, 107.97 (north-east, China border)
  - Điện Biên Phủ 21.39, 103.02 (north-west, Laos border)
  - Đồng Hới 17.47, 106.62 (narrow waist, Laos border 50 km west)
  - Huế 16.46, 107.59 (centre)
  - Buôn Ma Thuột 12.67, 108.04 (Central Highlands, towards Cambodia)
  - Châu Đốc 10.70, 105.12 (Mekong Delta, Cambodia border)
  - Cà Mau 9.18, 105.15 (southern tip; Phú Quốc island 10.22, 103.96 lies off Cambodia's coast)

---

## South Korea (KR)

```toml
[[holiday]]
country = "KR"
id = "new_year"
name = "신정 (New Year's Day)"
rule = "fixed(1,1)"
dates_2026 = ["2026-01-01"]
dates_2027 = ["2027-01-01"]
dates_2028 = ["2028-01-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = true
greeting = "새해 복 많이 받으세요"
greeting_en = "Happy New Year"
look = "fireworks_night"
source = ["https://www.kasa.go.kr/prog/plcyBrf/brief/kor/sub01_01_04/view.do?plcyBrfNo=431", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2027", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2028", "https://www.officeholidays.com/countries/south-korea/2027"]

[[holiday]]
country = "KR"
id = "seollal"
name = "설날 (Seollal, Korean Lunar New Year)"
rule = "listed"
dates_2026 = ["2026-02-16", "2026-02-17", "2026-02-18"]
dates_2027 = ["2027-02-06", "2027-02-07", "2027-02-08"]
dates_2028 = ["2028-01-26", "2028-01-27", "2028-01-28"]
tentative_2027 = false
tentative_2028 = true
observance = false
eve = false
greeting = "새해 복 많이 받으세요"
greeting_en = "Happy Lunar New Year · Year of the {animal}"
look = "lunar_new_year"
source = ["https://www.kasa.go.kr/prog/plcyBrf/brief/kor/sub01_01_04/view.do?plcyBrfNo=431", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2026", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2027", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2028", "https://www.officeholidays.com/countries/south-korea/2027"]

[[holiday]]
country = "KR"
id = "independence_movement_day"
name = "3·1절 (Samiljeol, Independence Movement Day)"
rule = "fixed(3,1)"
dates_2026 = ["2026-03-01"]
dates_2027 = ["2027-03-01"]
dates_2028 = ["2028-03-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "3·1절"
greeting_en = "Independence Movement Day"
look = "flag"
source = ["https://www.kasa.go.kr/prog/plcyBrf/brief/kor/sub01_01_04/view.do?plcyBrfNo=431", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2027", "https://www.mois.go.kr/frt/sub/a06/b08/nationalIcon_2_3/screen.do", "https://www.officeholidays.com/countries/south-korea/2027"]

[[holiday]]
country = "KR"
id = "labour_day"
name = "노동절 (Nodongjeol, Labour Day)"
rule = "fixed(5,1)"
dates_2026 = ["2026-05-01"]
dates_2027 = ["2027-05-01"]
dates_2028 = ["2028-05-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "노동절"
greeting_en = "Happy Labour Day"
look = "none"
source = ["https://www.kasa.go.kr/prog/plcyBrf/brief/kor/sub01_01_04/view.do?plcyBrfNo=431", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2027", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2028", "https://www.officeholidays.com/countries/south-korea/2026"]

[[holiday]]
country = "KR"
id = "childrens_day"
name = "어린이날 (Children's Day)"
rule = "fixed(5,5)"
dates_2026 = ["2026-05-05"]
dates_2027 = ["2027-05-05"]
dates_2028 = ["2028-05-05"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "어린이날을 축하합니다"
greeting_en = "Happy Children's Day"
look = "none"
source = ["https://www.kasa.go.kr/prog/plcyBrf/brief/kor/sub01_01_04/view.do?plcyBrfNo=431", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2027", "https://www.officeholidays.com/countries/south-korea/2027"]

[[holiday]]
country = "KR"
id = "buddhas_birthday"
name = "부처님오신날 (Buddha's Birthday, 8th of the 4th lunar month)"
rule = "listed"
dates_2026 = ["2026-05-24"]
dates_2027 = ["2027-05-13"]
dates_2028 = ["2028-05-02"]
tentative_2027 = false
tentative_2028 = true
observance = false
eve = false
greeting = "부처님 오신 날을 봉축합니다"
greeting_en = "Happy Buddha's Birthday"
look = "lanterns"
source = ["https://www.kasa.go.kr/prog/plcyBrf/brief/kor/sub01_01_04/view.do?plcyBrfNo=431", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2026", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2027", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2028", "https://www.officeholidays.com/countries/south-korea/2027"]

[[holiday]]
country = "KR"
id = "memorial_day"
name = "현충일 (Hyeonchungil, Memorial Day)"
rule = "fixed(6,6)"
dates_2026 = ["2026-06-06"]
dates_2027 = ["2027-06-06"]
dates_2028 = ["2028-06-06"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "현충일"
greeting_en = "Memorial Day"
look = "none"
source = ["https://www.kasa.go.kr/prog/plcyBrf/brief/kor/sub01_01_04/view.do?plcyBrfNo=431", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2027", "https://www.officeholidays.com/countries/south-korea/2027"]

[[holiday]]
country = "KR"
id = "constitution_day"
name = "제헌절 (Jeheonjeol, Constitution Day)"
rule = "fixed(7,17)"
dates_2026 = ["2026-07-17"]
dates_2027 = ["2027-07-17"]
dates_2028 = ["2028-07-17"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "제헌절"
greeting_en = "Constitution Day"
look = "flag"
source = ["https://www.kasa.go.kr/prog/plcyBrf/brief/kor/sub01_01_04/view.do?plcyBrfNo=431", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2027", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2028", "https://www.officeholidays.com/countries/south-korea/2026"]

[[holiday]]
country = "KR"
id = "liberation_day"
name = "광복절 (Gwangbokjeol, National Liberation Day)"
rule = "fixed(8,15)"
dates_2026 = ["2026-08-15"]
dates_2027 = ["2027-08-15"]
dates_2028 = ["2028-08-15"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "광복절을 축하합니다"
greeting_en = "Happy {nth since 1945} Liberation Day, Korea"
look = "flag"
source = ["https://www.kasa.go.kr/prog/plcyBrf/brief/kor/sub01_01_04/view.do?plcyBrfNo=431", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2027", "https://www.mois.go.kr/frt/sub/a06/b08/nationalIcon_2_3/screen.do", "https://www.officeholidays.com/countries/south-korea/2027"]

[[holiday]]
country = "KR"
id = "chuseok"
name = "추석 (Chuseok / Hangawi, Korean harvest festival)"
rule = "listed"
dates_2026 = ["2026-09-24", "2026-09-25", "2026-09-26"]
dates_2027 = ["2027-09-14", "2027-09-15", "2027-09-16"]
dates_2028 = ["2028-10-02", "2028-10-03", "2028-10-04"]
tentative_2027 = false
tentative_2028 = true
observance = false
eve = false
greeting = "풍성한 한가위 보내세요"
greeting_en = "Happy Chuseok"
look = "harvest_moon"
source = ["https://www.kasa.go.kr/prog/plcyBrf/brief/kor/sub01_01_04/view.do?plcyBrfNo=431", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2026", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2027", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2028", "https://www.officeholidays.com/countries/south-korea/2027"]

[[holiday]]
country = "KR"
id = "national_foundation_day"
name = "개천절 (Gaecheonjeol, National Foundation Day)"
rule = "fixed(10,3)"
dates_2026 = ["2026-10-03"]
dates_2027 = ["2027-10-03"]
dates_2028 = ["2028-10-03"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "개천절"
greeting_en = "National Foundation Day"
look = "flag"
source = ["https://www.kasa.go.kr/prog/plcyBrf/brief/kor/sub01_01_04/view.do?plcyBrfNo=431", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2027", "https://www.mois.go.kr/frt/sub/a06/b08/nationalIcon_2_3/screen.do", "https://www.officeholidays.com/countries/south-korea/2027"]

[[holiday]]
country = "KR"
id = "hangeul_day"
name = "한글날 (Hangeul Day, Korean Alphabet Day)"
rule = "fixed(10,9)"
dates_2026 = ["2026-10-09"]
dates_2027 = ["2027-10-09"]
dates_2028 = ["2028-10-09"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "한글날"
greeting_en = "Hangeul Day"
look = "flag"
source = ["https://www.kasa.go.kr/prog/plcyBrf/brief/kor/sub01_01_04/view.do?plcyBrfNo=431", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2027", "https://www.mois.go.kr/frt/sub/a06/b08/nationalIcon_2_3/screen.do", "https://www.officeholidays.com/countries/south-korea/2027"]

[[holiday]]
country = "KR"
id = "christmas"
name = "기독탄신일 / 성탄절 (Christmas Day)"
rule = "fixed(12,25)"
dates_2026 = ["2026-12-25"]
dates_2027 = ["2027-12-25"]
dates_2028 = ["2028-12-25"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "메리 크리스마스"
greeting_en = "Merry Christmas"
look = "none"
source = ["https://www.kasa.go.kr/prog/plcyBrf/brief/kor/sub01_01_04/view.do?plcyBrfNo=431", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2027", "https://www.officeholidays.com/countries/south-korea/2027"]

[[holiday]]
country = "KR"
id = "election_day"
name = "선거일 (Election Day: 전국동시지방선거 2026, 국회의원선거 2028)"
rule = "listed"
dates_2026 = ["2026-06-03"]
dates_2027 = []
dates_2028 = ["2028-04-12"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "투표하는 날"
greeting_en = "Election Day"
look = "none"
source = ["https://astro.kasi.re.kr/kor/life/post/calendarData?year=2026", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2028"]
```

Observances (not days off) and the half-mast day:

```toml
[[holiday]]
country = "KR"
id = "jeongwol_daeboreum"
name = "정월대보름 (Jeongwol Daeboreum, first full moon, 15th of the 1st lunar month)"
rule = "listed"
dates_2026 = ["2026-03-03"]
dates_2027 = ["2027-02-21"]
dates_2028 = ["2028-02-10"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "즐거운 정월대보름 보내세요"
greeting_en = "Happy Daeboreum"
look = "bonfire"
source = ["https://astro.kasi.re.kr/kor/life/post/calendarData?year=2026", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2027", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2028"]

[[holiday]]
country = "KR"
id = "parents_day"
name = "어버이날 (Parents' Day)"
rule = "fixed(5,8)"
dates_2026 = ["2026-05-08"]
dates_2027 = ["2027-05-08"]
dates_2028 = ["2028-05-08"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "어버이 은혜에 감사합니다"
greeting_en = "Happy Parents' Day"
look = "none"
source = ["https://astro.kasi.re.kr/kor/life/post/calendarData?year=2027", "https://www.timeanddate.com/holidays/south-korea/2027"]

[[holiday]]
country = "KR"
id = "teachers_day"
name = "스승의 날 (Teachers' Day)"
rule = "fixed(5,15)"
dates_2026 = ["2026-05-15"]
dates_2027 = ["2027-05-15"]
dates_2028 = ["2028-05-15"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "스승의 은혜에 감사합니다"
greeting_en = "Happy Teachers' Day"
look = "none"
source = ["https://astro.kasi.re.kr/kor/life/post/calendarData?year=2027", "https://www.timeanddate.com/holidays/south-korea/2027"]

[[holiday]]
country = "KR"
id = "memorial_day_half_mast"
name = "현충일 조기 게양 (Memorial Day, flag at half-mast)"
rule = "fixed(6,6)"
dates_2026 = ["2026-06-06"]
dates_2027 = ["2027-06-06"]
dates_2028 = ["2028-06-06"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "현충일 · 조기를 답시다"
greeting_en = "Memorial Day · half-mast"
look = "half_mast"
source = ["https://www.mois.go.kr/frt/sub/a06/b08/nationalIcon_2_3/screen.do", "https://ko.wikisource.org/wiki/%EB%8C%80%ED%95%9C%EB%AF%BC%EA%B5%AD%EA%B5%AD%EA%B8%B0%EB%B2%95"]

[[holiday]]
country = "KR"
id = "armed_forces_day"
name = "국군의 날 (Armed Forces Day; a statutory flag day)"
rule = "fixed(10,1)"
dates_2026 = ["2026-10-01"]
dates_2027 = ["2027-10-01"]
dates_2028 = ["2028-10-01"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "국군의 날"
greeting_en = "Armed Forces Day"
look = "flag"
source = ["https://www.mois.go.kr/frt/sub/a06/b08/nationalIcon_2_3/screen.do", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2027"]

[[holiday]]
country = "KR"
id = "pepero_day"
name = "빼빼로데이 (Pepero Day)"
rule = "fixed(11,11)"
dates_2026 = ["2026-11-11"]
dates_2027 = ["2027-11-11"]
dates_2028 = ["2028-11-11"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "해피 빼빼로데이"
greeting_en = "Happy Pepero Day"
look = "none"
source = ["https://www.koreatimes.co.kr/southkorea/society/20241107/how-pepero-day-got-started-in-korea"]

[[holiday]]
country = "KR"
id = "dongji"
name = "동지 (Dongji, winter solstice)"
rule = "listed"
dates_2026 = ["2026-12-22"]
dates_2027 = ["2027-12-22"]
dates_2028 = ["2028-12-21"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "동지 팥죽 드세요"
greeting_en = "Dongji · winter solstice"
look = "none"
source = ["https://astro.kasi.re.kr/kor/life/post/calendarData?year=2026", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2027", "https://astro.kasi.re.kr/kor/life/post/calendarData?year=2028"]
```

**Notes (KR)**

- **Legal basis.** 공휴일에 관한 법률 (Public Holidays Act, 2021) and 관공서의 공휴일에 관한 규정
  (the Presidential Decree on government-office holidays). The five 국경일 (national days under
  the 국경일에 관한 법률) are 3·1절, 제헌절, 광복절, 개천절, 한글날. **New in 2026:** in April 2026
  **노동절 (1 May)** and **제헌절 (17 Jul)** were designated government-office public holidays and
  apply **from 2026** (KASA's 2027 월력요항 release: "노동절·제헌절의 공휴일 지정(26.4.)으로 … 2026년부터
  공휴일로 적용"). 제헌절 had been dropped as a day off in 2008; 1 May was before that only a paid
  day off for private-sector employees ("근로자의 날"), renamed 노동절. Both were Fridays in 2026.
- **Official status.** 2026 and 2027 are official: the 2027 월력요항 was announced by KASA (우주항공청)
  on 29 Jun 2026 (공고 제20260078호; KASI's calendar data for 2027 regenerated 30 Jun 2026). The
  2028 월력요항 is due around June 2027; KASI already publishes **pre-release 2028 data** (generated
  30 Jun 2026, already including 노동절 and 제헌절), which is what dates_2028 uses. Lunar-dated 2028
  days are flagged tentative for that reason; fixed days are not.
- **Substitute holidays** (대체공휴일: for 설날/추석 when a day falls on a Sunday or another holiday;
  for the national days, 어린이날, 부처님오신날, 성탄절, 노동절 and 제헌절 when on a Saturday, Sunday or
  another holiday; never for 신정 or 현충일):
  - 2026: Mon 2 Mar (3·1절 on Sun), Mon 25 May (부처님오신날 on Sun), Mon 17 Aug (광복절 on Sat),
    Mon 5 Oct (개천절 on Sat). 현충일 on Sat 6 Jun and Chuseok's Sat 26 Sep get none.
  - 2027 (KASI/KASA): Tue 9 Feb (설날 on Sun 7 Feb), Mon 3 May (노동절 on Sat), Mon 19 Jul (제헌절 on
    Sat), Mon 16 Aug (광복절 on Sun), Mon 4 Oct (개천절 on Sun), Mon 11 Oct (한글날 on Sat), Mon 27 Dec
    (성탄절 on Sat). 현충일 on Sun 6 Jun gets none. 3-day-plus breaks: 10.
  - 2028 (pre-release): Thu 5 Oct, because 개천절 (Tue 3 Oct) is also 추석 day; 신정 on Sat 1 Jan gets
    none. KASI lists no other 2028 substitute.
- **Election days** are public holidays (공직선거법 art. 34 + the decree): the 9th local elections on
  **Wed 3 Jun 2026**, and the 23rd National Assembly election on **Wed 12 Apr 2028** (KASI 2028
  data). No national election falls in 2027 (next presidential and local elections 2030).
- **Temporary holidays** (임시공휴일) are set ad hoc by the Cabinet (e.g. 2 Oct 2023, 1 Oct 2024,
  27 Jan 2025); none was designated in 2026 (a rumoured 28 Sep 2026 bridge day was not adopted).
- **Lunar dates and the meridian — Korea DOES differ from China.** KASI computes the calendar for
  UTC+9 (135°E). A new moon between 15:00 and 16:00 UTC is before midnight in Beijing but after
  midnight in Seoul, so the Korean month starts a day later:
  - **Seollal 2027 = Sun 7 Feb** (new moon 6 Feb 15:56 UTC = 00:56 KST 7 Feb; China 6 Feb). The
    3-day block is 6–8 Feb, i.e. China's New Year's Day is Korea's 설 전날.
  - **Seollal 2028 = Thu 27 Jan** (new moon 26 Jan 15:12 UTC = 00:12 KST; China 26 Jan). Block 26–28 Jan.
  - Seollal 2026 = 17 Feb (same as China).
  - Daeboreum follows: 21 Feb 2027 and 10 Feb 2028 (China's Lantern Festival: 20 Feb, 9 Feb).
  - Chuseok (25 Sep 2026, 15 Sep 2027, 3 Oct 2028) and Buddha's Birthday months match China's;
    the 9th month of 2026 starts 11 Oct in Korea vs 10 Oct in China (no holiday affected).
  - 2028 has the same leap 5th month (윤5월 from 23 Jun) in both calendars.
  - **Generator caveat:** `LUNAR_NEW_YEAR` holds China's dates. KR `seollal` has `eve = false`, so
    nothing keys on it; with `eve = true` the generator would add an eve on 5 Feb 2027 and 25 Jan
    2028 (China's eve), which is wrong for Korea. The moon-calendar fallback for `seollal` and
    `chuseok` past 2028 needs UTC+9 new moons, not China's calendar.
- **Buddha's Birthday** is the 8th (not the 15th) of the 4th lunar month, so it is a week before
  Vesak elsewhere; it gets its own id `buddhas_birthday` rather than `vesak` so a full-moon
  fallback is not applied to it. Look `lanterns` for 연등회 (the Yeondeunghoe lantern parade,
  UNESCO-listed) and lanterns hung at temples and streets for weeks.
- **Cross-check.** officeholidays.com agrees on every 2026 date but omits the 3 Jun 2026 election
  day. **Disagreements (2027):** officeholidays labels Sat 6 Feb as "Korean New Year" and Sun 7 Feb
  as a "holiday" (KASA: 설날 is 7 Feb), shows Tue 9 Feb but omits the substitute days for 노동절,
  제헌절, 광복절, 한글날 and 성탄절 (only 4 Oct is shown). timeanddate's 2027 page (Wayback 3 Mar 2026)
  also calls 6 Feb "Seollal", drops 7 and 9 Feb, lists 노동절 as a bank holiday only and shows a
  single substitute (11 Oct); it predates the April 2026 designation. KASA/KASI are used. The
  observances (어버이 날 8 May, 스승의 날 15 May, 국군의 날 1 Oct) are in KASI's 기념일 list and on
  timeanddate; Pepero Day is not official (Lotte-promoted since the late 1990s) but is one of the
  biggest gift days of the year.
- **Looks.** 3·1절, 제헌절, 광복절, 개천절, 한글날 and 국군의 날 are the statutory flag days, hence `flag`.
  Seollal uses `lunar_new_year` (Korea has no lantern/fireworks tradition for Seollal, but the
  generic look reads fine). **Chuseok has `none`**: it is a full-moon harvest festival (songpyeon,
  ancestral rites, ganggangsullae) without lanterns; a `full_moon` look would fit it and
  Daeboreum. Daeboreum gets `bonfire` for 달집태우기 (burning the "moon house" bonfire). New Year has
  `fireworks_night` with `eve` for the 보신각 제야의 종 midnight bell-ringing countdown.
- **Left out.** 식목일 (Arbor Day, 5 Apr), 한식, 단오, Valentine's/White Day and the CSAT exam day are
  known but not universal enough; 4·19 and 5·18 commemorations are national memorials without a
  national half-mast order.

### Country notes (KR)

- **Christmas / New Year:** **25 Dec is a public holiday** (기독탄신일) and widely marked (street lights,
  a couples' holiday; ~28% Christian) — keep the Christmas lights. **1 Jan is a public holiday**
  (one day only; Seollal is the big New Year) and the 보신각 bell-ringing at midnight is the national
  countdown — keep the New Year countdown.
- **Half-mast:** **현충일 (6 Jun)** every year, **all day** (from the morning raising to lowering; flags
  may stay up 24 h if lit), on government buildings and households alike, under 대한민국국기법 art. 8
  and the 국기의 게양·관리 및 선양에 관한 규정 art. 7 (MOIS flag page). A nationwide siren at **10:00**
  marks one minute of silence. Otherwise half-mast only during a 국가장 (state funeral) or on days the
  Government designates. Half-mast in Korea means lowering the flag by the width of the flag.
- **Flag (태극기, Taegukgi):** 3:2, **white field**; centred **taegeuk** circle, **red (#CD2E3A) upper
  half and blue (#0047A0) lower half** (an S-shaped split); four **black trigrams** in the corners —
  ☰ geon top-left, ☵ gam top-right, ☲ ri bottom-left, ☷ gon bottom-right. For 9×3 on a white
  background: row 1 `☰` at cols 1–2 and `☵` at cols 8–9; row 2 the taegeuk at cols 4–6 as `▀` red
  on blue (red over blue), white elsewhere; row 3 `☲` at cols 1–2 and `☷` at cols 8–9, all black.
- **Geocoder language:** `ko`.
- **Anchor cities:**
  - Paju 37.76, 126.78 (north-west, DMZ)
  - Cheorwon 38.15, 127.31 (centre-north, DMZ)
  - Sokcho 38.21, 128.59 (north-east coast near the DMZ)
  - Incheon 37.46, 126.71 (west coast)
  - Ulleung 37.48, 130.90 (East Sea island)
  - Busan 35.18, 129.08 (south-east; Tsushima, Japan, 50 km)
  - Mokpo 34.81, 126.39 (south-west)
  - Jeju 33.50, 126.53 (southern island)

---

## Australia (AU)

National public holidays only (observed in every state and territory). State-only days are in
Notes.

```toml
[[holiday]]
country = "AU"
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
source = ["https://www.qld.gov.au/recreation/travel/holidays/public", "https://www.nsw.gov.au/about-nsw/public-holidays", "https://www.officeholidays.com/countries/australia/2027"]

[[holiday]]
country = "AU"
id = "australia_day"
name = "Australia Day"
rule = "fixed(1,26)"
dates_2026 = ["2026-01-26"]
dates_2027 = ["2027-01-26"]
dates_2028 = ["2028-01-26"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy Australia Day"
greeting_en = "Happy Australia Day"
look = "flag+fireworks_night"
source = ["https://www.qld.gov.au/recreation/travel/holidays/public", "https://www.nsw.gov.au/about-nsw/public-holidays", "https://www.officeholidays.com/countries/australia/2027"]

[[holiday]]
country = "AU"
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
source = ["https://www.qld.gov.au/recreation/travel/holidays/public", "https://www.nsw.gov.au/about-nsw/public-holidays", "https://www.officeholidays.com/countries/australia/2027"]

[[holiday]]
country = "AU"
id = "easter_monday"
name = "Easter Monday"
rule = "easter(1)"
dates_2026 = ["2026-04-06"]
dates_2027 = ["2027-03-29"]
dates_2028 = ["2028-04-17"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy Easter"
greeting_en = "Happy Easter"
look = "easter_eggs"
source = ["https://www.qld.gov.au/recreation/travel/holidays/public", "https://www.nsw.gov.au/about-nsw/public-holidays", "https://www.officeholidays.com/countries/australia/2027"]

[[holiday]]
country = "AU"
id = "anzac_day"
name = "Anzac Day"
rule = "fixed(4,25)"
dates_2026 = ["2026-04-25"]
dates_2027 = ["2027-04-25"]
dates_2028 = ["2028-04-25"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Lest we forget"
greeting_en = "Anzac Day · Lest we forget"
look = "none"
source = ["https://www.qld.gov.au/recreation/travel/holidays/public", "https://www.nsw.gov.au/about-nsw/public-holidays", "https://www.officeholidays.com/countries/australia/2027"]

[[holiday]]
country = "AU"
id = "christmas"
name = "Christmas Day"
rule = "fixed(12,25)"
dates_2026 = ["2026-12-25"]
dates_2027 = ["2027-12-25"]
dates_2028 = ["2028-12-25"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Merry Christmas"
greeting_en = "Merry Christmas"
look = "none"
source = ["https://www.qld.gov.au/recreation/travel/holidays/public", "https://www.nsw.gov.au/about-nsw/public-holidays", "https://www.officeholidays.com/countries/australia/2027"]

[[holiday]]
country = "AU"
id = "boxing_day"
name = "Boxing Day (Proclamation Day in South Australia)"
rule = "fixed(12,26)"
dates_2026 = ["2026-12-26"]
dates_2027 = ["2027-12-26"]
dates_2028 = ["2028-12-26"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy Boxing Day"
greeting_en = "Happy Boxing Day"
look = "none"
source = ["https://www.qld.gov.au/recreation/travel/holidays/public", "https://www.nsw.gov.au/about-nsw/public-holidays", "https://www.officeholidays.com/countries/australia/2027"]
```

Observances (not national days off) and half-mast days:

```toml
[[holiday]]
country = "AU"
id = "easter_sunday"
name = "Easter Sunday (a public holiday in most states)"
rule = "easter(0)"
dates_2026 = ["2026-04-05"]
dates_2027 = ["2027-03-28"]
dates_2028 = ["2028-04-16"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Happy Easter"
greeting_en = "Happy Easter"
look = "easter_eggs"
source = ["https://www.qld.gov.au/recreation/travel/holidays/public", "https://www.nsw.gov.au/about-nsw/public-holidays"]

[[holiday]]
country = "AU"
id = "anzac_day_half_mast"
name = "Anzac Day (flag at half-mast from dawn until noon)"
rule = "fixed(4,25)"
dates_2026 = ["2026-04-25"]
dates_2027 = ["2027-04-25"]
dates_2028 = ["2028-04-25"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Lest we forget · flags at half-mast until noon"
greeting_en = "Anzac Day · half-mast"
look = "half_mast"
source = ["https://www.pmc.gov.au/honours-and-symbols/australian-national-symbols/australian-national-flag/flag-network/2026-04-25", "https://www.pmc.gov.au/honours-and-symbols/australian-national-symbols/australian-national-flag/flag-network/2025-04-25"]

[[holiday]]
country = "AU"
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
source = ["https://www.officeholidays.com/countries/australia/2027"]

[[holiday]]
country = "AU"
id = "naidoc_week"
name = "NAIDOC Week (first Sunday of July to the following Sunday)"
rule = "nth_weekday(7, sun, 1)"
dates_2026 = ["2026-07-05", "2026-07-06", "2026-07-07", "2026-07-08", "2026-07-09", "2026-07-10", "2026-07-11", "2026-07-12"]
dates_2027 = ["2027-07-04", "2027-07-05", "2027-07-06", "2027-07-07", "2027-07-08", "2027-07-09", "2027-07-10", "2027-07-11"]
dates_2028 = ["2028-07-02", "2028-07-03", "2028-07-04", "2028-07-05", "2028-07-06", "2028-07-07", "2028-07-08", "2028-07-09"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Happy NAIDOC Week"
greeting_en = "Happy NAIDOC Week"
look = "none"
source = ["https://www.naidoc.org.au/about/naidoc-week", "https://www.cessnock.nsw.gov.au/Things-to-do/Whats-On/NAIDOC-Week-2026"]

[[holiday]]
country = "AU"
id = "fathers_day"
name = "Father's Day"
rule = "nth_weekday(9, sun, 1)"
dates_2026 = ["2026-09-06"]
dates_2027 = ["2027-09-05"]
dates_2028 = ["2028-09-03"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Happy Father's Day"
greeting_en = "Happy Father's Day"
look = "none"
source = ["https://www.officeholidays.com/countries/australia/2027"]

[[holiday]]
country = "AU"
id = "melbourne_cup"
name = "Melbourne Cup Day (\"the race that stops the nation\"; a holiday in metropolitan Melbourne only)"
rule = "nth_weekday(11, tue, 1)"
dates_2026 = ["2026-11-03"]
dates_2027 = ["2027-11-02"]
dates_2028 = ["2028-11-07"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Happy Melbourne Cup Day"
greeting_en = "Melbourne Cup Day"
look = "none"
source = ["https://www.officeholidays.com/countries/australia/2026", "https://www.officeholidays.com/countries/australia/2027"]

[[holiday]]
country = "AU"
id = "remembrance_day"
name = "Remembrance Day (flag at half-mast 10:30–11:02 am)"
rule = "fixed(11,11)"
dates_2026 = ["2026-11-11"]
dates_2027 = ["2027-11-11"]
dates_2028 = ["2028-11-11"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Lest we forget"
greeting_en = "Remembrance Day"
look = "candles"  # half-mast only 10:30–11:02, too short to draw
source = ["https://www.pmc.gov.au/honours-and-symbols/australian-national-symbols/australian-national-flag/flag-network/2025-11-11", "https://pmc.gov.au/honours-and-symbols/australian-national-symbols/australian-national-flag/flag-network/2024-11-11"]
```

**Notes (AU)**

- **What "national" means.** Public holidays are set by state and territory law; the Fair Work Act
  2009 s115 names the days every employee gets — 1 Jan, 26 Jan, Good Friday, Easter Monday,
  25 Apr, the King's Birthday, 25 Dec, 26 Dec — plus any day a state declares. Seven of those fall
  on the same date everywhere and are listed above. **King's Birthday** is national by statute but
  not by date, so it is left to Notes:
  - NSW, VIC, SA, TAS, ACT, NT: 2nd Monday in June — 8 Jun 2026, 14 Jun 2027, 12 Jun 2028.
  - QLD: 1st Monday in October — 5 Oct 2026, 4 Oct 2027, 2 Oct 2028 (qld.gov.au).
  - WA: proclaimed each year, usually the last Monday in September — 28 Sep 2026, 27 Sep 2027,
    probably 25 Sep 2028 (not yet proclaimed).
- **Substitute days** (from the NSW and QLD government pages; other states similar):
  - Anzac Day 2026 is a Saturday: Mon 27 Apr 2026 is an extra day in NSW, ACT and WA only (QLD,
    VIC, SA, TAS, NT none). Anzac Day 2027 is a Sunday: Mon 26 Apr 2027 in NSW and QLD (both pages) and in other states
    per officeholidays, which does not list them.
    The commemoration itself is always on 25 Apr.
  - Boxing Day 2026 (Sat) → Mon 28 Dec 2026. Christmas 2027 (Sat) → Mon 27 Dec, Boxing Day (Sun) →
    Tue 28 Dec 2027. New Year 2028 (Sat) → Mon 3 Jan 2028. Australia Day falls on weekdays
    2026–2028 (a weekend Australia Day moves to the Monday).
  - In South Australia 26 Dec is "Proclamation Day" (same day, same substitutes).
- **Big state-only holidays** (not listed as entries): Easter Saturday (all but TAS/WA) and Easter
  Sunday (NSW, VIC, QLD, WA, SA, ACT); Labour Day (WA 1st Mon Mar; VIC and TAS 2nd Mon Mar; QLD and
  NT 1st Mon May; NSW, ACT and SA 1st Mon Oct); Canberra Day (ACT) and Adelaide Cup (SA), 2nd Mon
  Mar; Reconciliation Day (ACT, Mon on/after 27 May); Western Australia Day (1st Mon Jun); Picnic Day
  (NT, 1st Mon Aug); Royal Queensland Show (Brisbane, Wed 12 Aug 2026 / 11 Aug 2027 / 16 Aug 2028);
  AFL Grand Final Friday (VIC, set each year); Melbourne Cup (metropolitan VIC, 1st Tue Nov);
  Easter Tuesday (TAS public service), Royal Hobart Regatta and Recreation Day (parts of TAS);
  Christmas Eve and New Year's Eve evenings (part-day in QLD, SA, NT). NSW's 1st-Monday-in-August
  "Bank Holiday" is not a public holiday.
- **2028.** All seven national dates are fixed or Easter-based and confirmed by Queensland's
  published 2028 table, so nothing is tentative. officeholidays.com has no 2028 page and no
  timeanddate 2028 snapshot exists.
- **Cross-check.** officeholidays.com 2026/2027 agrees on every national date. **Disagreements:**
  it puts NSW's Bank Holiday and NT's Picnic Day 2027 on Mon 9 Aug (the 1st Monday is 2 Aug, which
  the NSW page confirms for the Bank Holiday), and labels the 2027 Anzac Monday without states.
  timeanddate's 2027 page (Wayback 10 Mar 2026) was not needed.
- **Australia Day** is contested: many First Nations people and others mark 26 Jan as Invasion Day
  or Survival Day, with large rallies. The government greeting is "Happy Australia Day"; if a
  neutral line is preferred, use the plain name "Australia Day". Looks: `flag+fireworks_night`
  (evening fireworks in Sydney, Perth, Canberra and elsewhere; no eve).
- **Anzac Day** is solemn (dawn services, marches, two-up): greeting "Lest we forget", look `none`
  on the holiday, with the half-mast twin carrying the flag.
- **NAIDOC Week** (Aboriginal and Torres Strait Islander history and culture) runs from the first
  Sunday in July to the following Sunday; 2026 was 5–12 Jul. Its rule reproduces all three years.
- **Left out.** National Sorry Day (26 May) and National Reconciliation Week (27 May–3 Jun),
  Harmony Week, Vietnam Veterans' Day (18 Aug), Australian National Flag Day (3 Sep) and Valentine's
  Day are less universal; Halloween is handled globally.

### Country notes (AU)

- **Christmas / New Year:** **25 and 26 Dec are public holidays** and Christmas is the biggest
  holiday of the year (in midsummer: lights, carols by candlelight, beach barbecues) — keep the
  15–26 Dec lights. **1 Jan is a public holiday** and the Sydney Harbour midnight fireworks are the
  country's signature countdown — keep the New Year countdown.
- **Half-mast:** two fixed days a year under the Commonwealth Flag Network protocol (PM&C), asked
  of Australian Government buildings; states, councils and most organisations follow:
  - **Anzac Day, 25 Apr: dawn until noon**, then raised to the peak for the rest of the day.
  - **Remembrance Day, 11 Nov: 10:30 am to 11:02 am local time** (peak from about 8 am; lowered
    for the 11:00 minute of silence; back to the peak at 11:02).
  - Otherwise ad hoc, e.g. the National Day of Mourning for the Bondi Beach attack victims on
    22 Jan 2026 (all day) and the death of Queen Elizabeth II (22 Sep 2022).
- **Flag:** 1:2, **dark blue field (#012169, PMS 280)**; the **Union Jack** in the upper hoist
  quarter (red #E4002B and white crosses); a large white **seven-pointed Commonwealth Star** centred
  below the Union Jack; on the fly half the **Southern Cross**: four white seven-pointed stars in a
  kite (top, bottom, left, right) and one small five-pointed star. For 9×3 on blue: row 1 cols 1–4
  a Union Jack glyph block (e.g. `╳` / `✚` in white and red); row 3 col 2 a white `✶`
  (Commonwealth Star); fly cols 6–9 white stars: `·` row 1 col 7 (Gamma), `·` row 2 cols 6 and 9
  (Beta, Delta), `·` row 3 col 7 (Alpha), with a tiny `.` at row 2 col 8 (Epsilon).
- **Geocoder language:** `en`.
- **Anchor cities** (spread over the continent; Thursday Island is 150 km from Papua New Guinea):
  - Darwin -12.46, 130.84 (Top End)
  - Thursday Island -10.58, 142.22 (Torres Strait, far north)
  - Cairns -16.92, 145.77 (far north Queensland)
  - Broome -17.96, 122.24 (north-west)
  - Perth -31.95, 115.86 (west)
  - Alice Springs -23.70, 133.88 (centre)
  - Adelaide -34.93, 138.60 (south)
  - Hobart -42.88, 147.33 (Tasmania, far south)

---

## Counts

| Country | Holidays (days off) | Observances | Half-mast | 2027 status | 2028 status |
|---|---|---|---|---|---|
| VN | 7 (New Year, Tết ×5 days, Hùng Kings, 30/4, 1/5, National Day ×2, Culture Day 24/11 — new from 2026) | 6 (Kitchen Gods, 8/3, Phật đản, Mid-Autumn, 20/10, 20/11) | none fixed | official (MOHA 11483/TB-BNV for Tết and National Day; others by law) | not published; Tết days (24–28 Jan guess), National Day neighbour (1 Sep guess) and Hùng Kings flagged tentative |
| KR | 14 (incl. Seollal ×3, Chuseok ×3, 노동절 and 제헌절 — public holidays from 2026, election days 2026/2028) | 6 (Daeboreum, Parents' Day, Teachers' Day, Armed Forces Day, Pepero Day, Dongji) | 1 (Memorial Day 6 Jun, all day) | official (KASA 2027 월력요항, 29 Jun 2026) | KASI pre-release data; lunar days flagged tentative; Seollal 27 Jan (not China's 26 Jan) |
| AU | 7 national (New Year, Australia Day, Good Friday, Easter Monday, Anzac Day, Christmas, Boxing Day) | 5 (Easter Sunday, Mother's Day, NAIDOC Week, Father's Day, Melbourne Cup) | 2 (Anzac Day dawn–noon; Remembrance Day 10:30–11:02) | official (NSW, QLD) | official (QLD 2028 table); King's Birthday by state, in Notes |
