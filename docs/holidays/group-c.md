# World holidays — group C: US, UK, BR, MX, NG

Compiled 20 Oct 2026 for the weathr fork's holiday greetings and decorations. 2027 re-checked and
2028 added 21 Oct 2026.

**Schema conventions used below**

- `rule` is one string from the fixed grammar. For a holiday spanning several days, `rule` is an
  array with one rule per date (e.g. Carnaval `["easter(-48)", "easter(-47)"]`).
- Weekday tokens are `mon tue wed thu fri sat sun`.
- `dates_*` hold the holiday day itself. Substitute or observed days off are in Notes.
- `tentative_2027 = true` means the 2027 **date** itself may still move (Islamic days). Uncertainty
  about status only, such as a 2027 decree not yet published, is covered in Notes. `tentative_2028`
  works the same way.
- `look = "fireworks_night"` on Bonfire Night stands in for `bonfire` until that visual exists.
- Easter Sunday is 2026-04-05, 2027-03-28 and 2028-04-16. These dates match gov.uk (Good
  Friday/Easter Monday), Brazil's portaria (Paixão de Cristo) and ANBIMA's 2027 and 2028 lists.

**Primary sources checked**

| Country | Official source | Cross-check |
|---|---|---|
| US | [OPM federal holidays](https://www.opm.gov/policy-data-oversight/pay-leave/federal-holidays/) (lists 2026, 2027, 2028); [5 U.S.C. 6103](https://www.law.cornell.edu/uscode/text/5/6103) | officeholidays.com [2026](https://www.officeholidays.com/countries/usa/2026) / [2027](https://www.officeholidays.com/countries/usa/2027) |
| UK | [gov.uk bank-holidays.json](https://www.gov.uk/bank-holidays.json) (lists 2026, 2027, 2028) | officeholidays.com [2026](https://www.officeholidays.com/countries/united-kingdom/2026) / [2027](https://www.officeholidays.com/countries/united-kingdom/2027) |
| BR | [Portaria MGI nº 11.460/2025, DOU 30 Dec 2025](https://www.in.gov.br/web/dou/-/portaria-mgi-n-11.460-de-29-de-dezembro-de-2025-678388627) (2026 only) | ANBIMA [2026](https://www.anbima.com.br/feriados/fer_nacionais/2026.asp) / [2027](https://www.anbima.com.br/feriados/fer_nacionais/2027.asp) / [2028](https://www.anbima.com.br/feriados/fer_nacionais/2028.asp); officeholidays.com |
| MX | LFT art. 74 as quoted by [Profedet](https://www.gob.mx/profedet/articulos/sabes-cuales-son-los-dias-de-descanso-obligatorios-163134); art. 74 fr. VII reform, [DOF 30 Sep 2024](https://gaceta.diputados.gob.mx/PDF/66/2024/sep/DOF30sep2024-Art74LeyFedTrabajo.pdf); [STPS 2026 calendar (DOF)](https://sidof.segob.gob.mx/notas/docFuente/5776858) | officeholidays.com [2026](https://www.officeholidays.com/countries/mexico/2026) / [2027](https://www.officeholidays.com/countries/mexico/2027) |
| NG | [Public Holidays Act, Cap P40](https://placng.org/lawsofnigeria/print.php?sn=467); Ministry of Interior 2026 declarations, reported by FMINO, VON and Channels TV (linked per entry) | officeholidays.com [2026](https://www.officeholidays.com/countries/nigeria/2026) / [2027](https://www.officeholidays.com/countries/nigeria/2027); timeanddate.com [2028](https://www.timeanddate.com/holidays/nigeria/2028); IslamicFinder; AlAdhan Umm al-Qura |

Fetch problems: interior.gov.ng returned 503 ("temporarily unavailable") on 20 Oct 2026, so its
press-release URLs are cited from the search index. Their dates are confirmed by fmino.gov.ng (the
Federal Ministry of Information), VON (the state broadcaster) and Channels TV. diputados.gob.mx
refused connections, so the current consolidated LFT text was not read directly (see MX Notes).

21 Oct 2026: officeholidays.com has no 2028 pages yet (404 for all five countries). timeanddate.com
blocks scripted fetches (Cloudflare 403), so its Nigeria 2027 and 2028 tables were read from Wayback
snapshots of 8 Feb and 6 Mar 2026. The US, UK and BR 2028 dates were confirmed against OPM, gov.uk
and ANBIMA 2028. The MX 2028 dates come from the LFT rules alone.

---

## United States (US)

```toml
[[holiday]]
country = "US"
id = "new_year"
name = "New Year's Day"
rule = "fixed(1,1)"
dates_2026 = ["2026-01-01"]
dates_2027 = ["2027-01-01"]
dates_2028 = ["2028-01-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy New Year"
greeting_en = "Happy New Year"
look = "none"
source = ["https://www.opm.gov/policy-data-oversight/pay-leave/federal-holidays/", "https://www.law.cornell.edu/uscode/text/5/6103"]

[[holiday]]
country = "US"
id = "mlk_day"
name = "Birthday of Martin Luther King, Jr."
rule = "nth_weekday(1, mon, 3)"
dates_2026 = ["2026-01-19"]
dates_2027 = ["2027-01-18"]
dates_2028 = ["2028-01-17"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Martin Luther King Jr. Day"
greeting_en = "Martin Luther King Jr. Day"
look = "none"
source = ["https://www.opm.gov/policy-data-oversight/pay-leave/federal-holidays/", "https://www.law.cornell.edu/uscode/text/5/6103"]

[[holiday]]
country = "US"
id = "washingtons_birthday"
name = "Washington's Birthday"
rule = "nth_weekday(2, mon, 3)"
dates_2026 = ["2026-02-16"]
dates_2027 = ["2027-02-15"]
dates_2028 = ["2028-02-21"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy Presidents' Day"
greeting_en = "Happy Presidents' Day"
look = "none"
source = ["https://www.opm.gov/policy-data-oversight/pay-leave/federal-holidays/", "https://www.law.cornell.edu/uscode/text/5/6103"]

[[holiday]]
country = "US"
id = "easter_sunday"
name = "Easter Sunday"
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
source = ["https://www.officeholidays.com/countries/usa/2026", "https://www.officeholidays.com/countries/usa/2027"]

[[holiday]]
country = "US"
id = "memorial_day"
name = "Memorial Day"
rule = "last_weekday(5, mon)"
dates_2026 = ["2026-05-25"]
dates_2027 = ["2027-05-31"]
dates_2028 = ["2028-05-29"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Memorial Day"
greeting_en = "Memorial Day"
look = "none"
source = ["https://www.opm.gov/policy-data-oversight/pay-leave/federal-holidays/", "https://www.law.cornell.edu/uscode/text/5/6103"]

[[holiday]]
country = "US"
id = "juneteenth"
name = "Juneteenth National Independence Day"
rule = "fixed(6,19)"
dates_2026 = ["2026-06-19"]
dates_2027 = ["2027-06-19"]
dates_2028 = ["2028-06-19"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy Juneteenth"
greeting_en = "Happy Juneteenth"
look = "none"
source = ["https://www.opm.gov/policy-data-oversight/pay-leave/federal-holidays/", "https://www.law.cornell.edu/uscode/text/5/6103"]

[[holiday]]
country = "US"
id = "independence_day"
name = "Independence Day"
rule = "fixed(7,4)"
dates_2026 = ["2026-07-04"]
dates_2027 = ["2027-07-04"]
dates_2028 = ["2028-07-04"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy Fourth of July"
greeting_en = "Happy Fourth of July · America's {nth since 1776} birthday"
look = "fireworks_night+flag"
source = ["https://www.opm.gov/policy-data-oversight/pay-leave/federal-holidays/", "https://www.law.cornell.edu/uscode/text/5/6103"]

[[holiday]]
country = "US"
id = "labor_day"
name = "Labor Day"
rule = "nth_weekday(9, mon, 1)"
dates_2026 = ["2026-09-07"]
dates_2027 = ["2027-09-06"]
dates_2028 = ["2028-09-04"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy Labor Day"
greeting_en = "Happy Labor Day"
look = "none"
source = ["https://www.opm.gov/policy-data-oversight/pay-leave/federal-holidays/", "https://www.law.cornell.edu/uscode/text/5/6103"]

[[holiday]]
country = "US"
id = "columbus_day"
name = "Columbus Day"
rule = "nth_weekday(10, mon, 2)"
dates_2026 = ["2026-10-12"]
dates_2027 = ["2027-10-11"]
dates_2028 = ["2028-10-09"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Columbus Day"
greeting_en = "Columbus Day"
look = "none"
source = ["https://www.opm.gov/policy-data-oversight/pay-leave/federal-holidays/", "https://www.law.cornell.edu/uscode/text/5/6103"]

[[holiday]]
country = "US"
id = "veterans_day"
name = "Veterans Day"
rule = "fixed(11,11)"
dates_2026 = ["2026-11-11"]
dates_2027 = ["2027-11-11"]
dates_2028 = ["2028-11-11"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Veterans Day"
greeting_en = "Veterans Day"
look = "none"
source = ["https://www.opm.gov/policy-data-oversight/pay-leave/federal-holidays/", "https://www.law.cornell.edu/uscode/text/5/6103"]

[[holiday]]
country = "US"
id = "thanksgiving"
name = "Thanksgiving Day"
rule = "nth_weekday(11, thu, 4)"
dates_2026 = ["2026-11-26"]
dates_2027 = ["2027-11-25"]
dates_2028 = ["2028-11-23"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy Thanksgiving"
greeting_en = "Happy Thanksgiving"
look = "none"
source = ["https://www.opm.gov/policy-data-oversight/pay-leave/federal-holidays/", "https://www.law.cornell.edu/uscode/text/5/6103"]

[[holiday]]
country = "US"
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
source = ["https://www.opm.gov/policy-data-oversight/pay-leave/federal-holidays/", "https://www.law.cornell.edu/uscode/text/5/6103"]
```

**Notes (US)**

- Observed federal days off, per OPM: Independence Day 2026 falls on a Saturday, so it is
  observed **Fri 2026-07-03**. In 2027, Juneteenth is observed **Fri 2027-06-18**, Independence Day
  **Mon 2027-07-05**, Christmas **Fri 2027-12-24**, and New Year's Day 2028 **Fri 2027-12-31**.
  Veterans Day needs no substitute in 2026 or 2027. In 2028 it falls on a Saturday and is observed
  **Fri 2028-11-10**; no other 2028 holiday needs a substitute.
- Juneteenth has been a federal holiday since 2021. **2026-07-04 is the 250th anniversary**, so a
  special "Happy 250th" greeting may be worth adding.
- "Washington's Birthday" is the name in the statute. The greeting uses the everyday name,
  "Presidents' Day".
- Columbus Day keeps its statutory name. Many states and cities mark Indigenous Peoples' Day on the
  same Monday, so the greeting is the plain name.
- MLK Day, Memorial Day and Veterans Day use plain names because they are solemn.
- Executive orders sometimes close agencies on extra days: EO 14371 closed them on 24 and 26 Dec
  2025, and 24 Dec was given in 2024. These are not holidays, and none had been announced for 2026
  as of 20 Oct. Inauguration Day (DC area only) next falls in 2029.
- Easter Sunday is not a federal holiday; it is included as an observance. Mother's Day and
  Father's Day were left out on purpose. Halloween is handled globally.
- Cross-check: all 11 dates agree with officeholidays.com. Its Juneteenth rows are garbled (they
  show 2026-06-15 and 2027-06-21); OPM is authoritative.
- 2028: all 11 federal dates match OPM's 2028 table (read 21 Oct 2026). Easter is computed.

---

## United Kingdom (UK — England & Wales)

```toml
[[holiday]]
country = "UK"
id = "new_year"
name = "New Year's Day"
rule = "fixed(1,1)"
dates_2026 = ["2026-01-01"]
dates_2027 = ["2027-01-01"]
dates_2028 = ["2028-01-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy New Year"
greeting_en = "Happy New Year"
look = "none"
source = ["https://www.gov.uk/bank-holidays.json"]

[[holiday]]
country = "UK"
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
source = ["https://www.gov.uk/bank-holidays.json"]

[[holiday]]
country = "UK"
id = "easter_sunday"
name = "Easter Sunday"
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
source = ["https://www.officeholidays.com/countries/united-kingdom/2026", "https://www.officeholidays.com/countries/united-kingdom/2027"]

[[holiday]]
country = "UK"
id = "easter_monday"
name = "Easter Monday"
rule = "easter(+1)"
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
source = ["https://www.gov.uk/bank-holidays.json"]

[[holiday]]
country = "UK"
id = "early_may_bank_holiday"
name = "Early May bank holiday"
rule = "nth_weekday(5, mon, 1)"
dates_2026 = ["2026-05-04"]
dates_2027 = ["2027-05-03"]
dates_2028 = ["2028-05-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy bank holiday"
greeting_en = "Happy bank holiday"
look = "none"
source = ["https://www.gov.uk/bank-holidays.json"]

[[holiday]]
country = "UK"
id = "spring_bank_holiday"
name = "Spring bank holiday"
rule = "last_weekday(5, mon)"
dates_2026 = ["2026-05-25"]
dates_2027 = ["2027-05-31"]
dates_2028 = ["2028-05-29"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy bank holiday"
greeting_en = "Happy bank holiday"
look = "none"
source = ["https://www.gov.uk/bank-holidays.json"]

[[holiday]]
country = "UK"
id = "summer_bank_holiday"
name = "Summer bank holiday"
rule = "last_weekday(8, mon)"
dates_2026 = ["2026-08-31"]
dates_2027 = ["2027-08-30"]
dates_2028 = ["2028-08-28"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy bank holiday"
greeting_en = "Happy bank holiday"
look = "none"
source = ["https://www.gov.uk/bank-holidays.json"]

[[holiday]]
country = "UK"
id = "bonfire_night"
name = "Bonfire Night (Guy Fawkes Night)"
rule = "fixed(11,5)"
dates_2026 = ["2026-11-05"]
dates_2027 = ["2027-11-05"]
dates_2028 = ["2028-11-05"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Happy Bonfire Night"
greeting_en = "Happy Bonfire Night"
look = "fireworks_night+bonfire"
source = ["https://en.wikipedia.org/wiki/Guy_Fawkes_Night"]

[[holiday]]
country = "UK"
id = "remembrance_sunday"
name = "Remembrance Sunday"
rule = "nth_weekday(11, sun, 2)"
dates_2026 = ["2026-11-08"]
dates_2027 = ["2027-11-14"]
dates_2028 = ["2028-11-12"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Remembrance Sunday"
greeting_en = "Remembrance Sunday"
look = "none"
source = ["https://www.officeholidays.com/countries/united-kingdom/2026", "https://www.officeholidays.com/countries/united-kingdom/2027"]

[[holiday]]
country = "UK"
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
source = ["https://www.gov.uk/bank-holidays.json"]

[[holiday]]
country = "UK"
id = "boxing_day"
name = "Boxing Day"
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
source = ["https://www.gov.uk/bank-holidays.json"]
```

**Notes (UK)**

- Substitute days, per the gov.uk JSON:
  - Boxing Day 2026 (Sat) → **Mon 2026-12-28**.
  - Christmas 2027 (Sat) → **Mon 2027-12-27**; Boxing Day 2027 (Sun) → **Tue 2027-12-28**.
  - New Year's Day 2028 (Sat) → Mon 2028-01-03. No other 2028 E&W substitutes.
- 2028: all 8 E&W bank-holiday dates match gov.uk's JSON (read 21 Oct 2026).
- officeholidays.com agrees on every England & Wales date. Its UK page leaves out Easter Monday
  and the late-August bank holiday because Scotland does not share them.
- **Scotland only:**
  - 2 January: 2026-01-02; 2027 is substituted to 2027-01-04; 2028 (Sun) to Tue 2028-01-04.
  - St Andrew's Day: 2026-11-30, 2027-11-30 and 2028-11-30.
  - Summer bank holiday is the **first** Monday in August (2026-08-03, 2027-08-02, 2028-08-07),
    not the last.
  - Scotland has no Easter Monday bank holiday.
  - One-off **World Cup bank holiday, Mon 2026-06-15**, granted by Royal Proclamation
    ([gov.scot](https://www.gov.scot/news/world-cup-bank-holiday-confirmed/)).
- **Northern Ireland only:**
  - St Patrick's Day: 2026-03-17, 2027-03-17 and 2028-03-17.
  - Battle of the Boyne (Orangemen's Day), 12 Jul: in 2026 it falls on a Sunday, so the bank
    holiday is the substitute **Mon 2026-07-13**; in 2027 it is 2027-07-12 and in 2028
    2028-07-12.
- These days are not bank holidays: St David's Day (1 Mar), St George's Day (23 Apr), Mothering
  Sunday, and the Armistice Day two-minute silence on 11 Nov.
- Bonfire Night has had no legal status since 1859, when the Observance of 5th November Act was
  repealed. Many displays move to the nearest weekend. Switch the look to `bonfire` once it exists.
- Remembrance Sunday is the second Sunday in November. It is solemn, so the greeting is the plain
  name.

---

## Brazil (BR)

```toml
[[holiday]]
country = "BR"
id = "new_year"
name = "Confraternização Universal"
rule = "fixed(1,1)"
dates_2026 = ["2026-01-01"]
dates_2027 = ["2027-01-01"]
dates_2028 = ["2028-01-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Feliz Ano Novo"
greeting_en = "Happy New Year"
look = "none"
source = ["https://www.in.gov.br/web/dou/-/portaria-mgi-n-11.460-de-29-de-dezembro-de-2025-678388627", "https://www.anbima.com.br/feriados/fer_nacionais/2027.asp", "https://www.anbima.com.br/feriados/fer_nacionais/2028.asp"]

[[holiday]]
country = "BR"
id = "carnival"
name = "Carnaval"
rule = ["easter(-48)", "easter(-47)"]
dates_2026 = ["2026-02-16", "2026-02-17"]
dates_2027 = ["2027-02-08", "2027-02-09"]
dates_2028 = ["2028-02-28", "2028-02-29"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Bom Carnaval"
greeting_en = "Happy Carnival"
look = "carnival"
source = ["https://www.in.gov.br/web/dou/-/portaria-mgi-n-11.460-de-29-de-dezembro-de-2025-678388627", "https://www.anbima.com.br/feriados/fer_nacionais/2027.asp", "https://www.anbima.com.br/feriados/fer_nacionais/2028.asp", "https://www.officeholidays.com/countries/brazil/2027"]

[[holiday]]
country = "BR"
id = "good_friday"
name = "Paixão de Cristo"
rule = "easter(-2)"
dates_2026 = ["2026-04-03"]
dates_2027 = ["2027-03-26"]
dates_2028 = ["2028-04-14"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Sexta-feira Santa"
greeting_en = "Good Friday"
look = "none"
source = ["https://www.in.gov.br/web/dou/-/portaria-mgi-n-11.460-de-29-de-dezembro-de-2025-678388627", "https://www.anbima.com.br/feriados/fer_nacionais/2027.asp", "https://www.anbima.com.br/feriados/fer_nacionais/2028.asp"]

[[holiday]]
country = "BR"
id = "easter_sunday"
name = "Páscoa"
rule = "easter(0)"
dates_2026 = ["2026-04-05"]
dates_2027 = ["2027-03-28"]
dates_2028 = ["2028-04-16"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "Feliz Páscoa"
greeting_en = "Happy Easter"
look = "easter_eggs"
source = ["https://www.in.gov.br/web/dou/-/portaria-mgi-n-11.460-de-29-de-dezembro-de-2025-678388627", "https://www.anbima.com.br/feriados/fer_nacionais/2027.asp", "https://www.anbima.com.br/feriados/fer_nacionais/2028.asp"]

[[holiday]]
country = "BR"
id = "tiradentes"
name = "Tiradentes"
rule = "fixed(4,21)"
dates_2026 = ["2026-04-21"]
dates_2027 = ["2027-04-21"]
dates_2028 = ["2028-04-21"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Dia de Tiradentes"
greeting_en = "Tiradentes Day"
look = "none"
source = ["https://www.in.gov.br/web/dou/-/portaria-mgi-n-11.460-de-29-de-dezembro-de-2025-678388627", "https://www.anbima.com.br/feriados/fer_nacionais/2027.asp", "https://www.anbima.com.br/feriados/fer_nacionais/2028.asp"]

[[holiday]]
country = "BR"
id = "labour_day"
name = "Dia Mundial do Trabalho"
rule = "fixed(5,1)"
dates_2026 = ["2026-05-01"]
dates_2027 = ["2027-05-01"]
dates_2028 = ["2028-05-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Feliz Dia do Trabalhador"
greeting_en = "Happy Labour Day"
look = "none"
source = ["https://www.in.gov.br/web/dou/-/portaria-mgi-n-11.460-de-29-de-dezembro-de-2025-678388627", "https://www.anbima.com.br/feriados/fer_nacionais/2027.asp", "https://www.anbima.com.br/feriados/fer_nacionais/2028.asp"]

[[holiday]]
country = "BR"
id = "corpus_christi"
name = "Corpus Christi"
rule = "easter(+60)"
dates_2026 = ["2026-06-04"]
dates_2027 = ["2027-05-27"]
dates_2028 = ["2028-06-15"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Corpus Christi"
greeting_en = "Corpus Christi"
look = "none"
source = ["https://www.in.gov.br/web/dou/-/portaria-mgi-n-11.460-de-29-de-dezembro-de-2025-678388627", "https://www.anbima.com.br/feriados/fer_nacionais/2027.asp", "https://www.anbima.com.br/feriados/fer_nacionais/2028.asp", "https://www.officeholidays.com/countries/brazil/2027"]

[[holiday]]
country = "BR"
id = "independence_day"
name = "Independência do Brasil"
rule = "fixed(9,7)"
dates_2026 = ["2026-09-07"]
dates_2027 = ["2027-09-07"]
dates_2028 = ["2028-09-07"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Feliz Dia da Independência"
greeting_en = "Happy {nth since 1822} Independence Day, Brazil"
look = "flag"
source = ["https://www.in.gov.br/web/dou/-/portaria-mgi-n-11.460-de-29-de-dezembro-de-2025-678388627", "https://www.anbima.com.br/feriados/fer_nacionais/2027.asp", "https://www.anbima.com.br/feriados/fer_nacionais/2028.asp"]

[[holiday]]
country = "BR"
id = "nossa_senhora_aparecida"
name = "Nossa Senhora Aparecida"
rule = "fixed(10,12)"
dates_2026 = ["2026-10-12"]
dates_2027 = ["2027-10-12"]
dates_2028 = ["2028-10-12"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Nossa Senhora Aparecida · Feliz Dia das Crianças"
greeting_en = "Our Lady of Aparecida · Happy Children's Day"
look = "none"
source = ["https://www.in.gov.br/web/dou/-/portaria-mgi-n-11.460-de-29-de-dezembro-de-2025-678388627", "https://www.anbima.com.br/feriados/fer_nacionais/2027.asp", "https://www.anbima.com.br/feriados/fer_nacionais/2028.asp"]

[[holiday]]
country = "BR"
id = "finados"
name = "Finados"
rule = "fixed(11,2)"
dates_2026 = ["2026-11-02"]
dates_2027 = ["2027-11-02"]
dates_2028 = ["2028-11-02"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Dia de Finados"
greeting_en = "All Souls' Day"
look = "candles"
source = ["https://www.in.gov.br/web/dou/-/portaria-mgi-n-11.460-de-29-de-dezembro-de-2025-678388627", "https://www.anbima.com.br/feriados/fer_nacionais/2027.asp", "https://www.anbima.com.br/feriados/fer_nacionais/2028.asp"]

[[holiday]]
country = "BR"
id = "republic_day"
name = "Proclamação da República"
rule = "fixed(11,15)"
dates_2026 = ["2026-11-15"]
dates_2027 = ["2027-11-15"]
dates_2028 = ["2028-11-15"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Proclamação da República"
greeting_en = "Republic Proclamation Day"
look = "none"
source = ["https://www.in.gov.br/web/dou/-/portaria-mgi-n-11.460-de-29-de-dezembro-de-2025-678388627", "https://www.anbima.com.br/feriados/fer_nacionais/2027.asp", "https://www.anbima.com.br/feriados/fer_nacionais/2028.asp"]

[[holiday]]
country = "BR"
id = "black_consciousness_day"
name = "Dia Nacional de Zumbi e da Consciência Negra"
rule = "fixed(11,20)"
dates_2026 = ["2026-11-20"]
dates_2027 = ["2027-11-20"]
dates_2028 = ["2028-11-20"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Dia da Consciência Negra"
greeting_en = "Black Consciousness Day"
look = "none"
source = ["https://www.in.gov.br/web/dou/-/portaria-mgi-n-11.460-de-29-de-dezembro-de-2025-678388627", "https://www.anbima.com.br/feriados/fer_nacionais/2027.asp", "https://www.anbima.com.br/feriados/fer_nacionais/2028.asp", "https://www2.camara.leg.br/legin/fed/lei/2023/lei-14759-21-dezembro-2023-795091-publicacaooriginal-170522-pl.html"]

[[holiday]]
country = "BR"
id = "christmas"
name = "Natal"
rule = "fixed(12,25)"
dates_2026 = ["2026-12-25"]
dates_2027 = ["2027-12-25"]
dates_2028 = ["2028-12-25"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Feliz Natal"
greeting_en = "Merry Christmas"
look = "none"
source = ["https://www.in.gov.br/web/dou/-/portaria-mgi-n-11.460-de-29-de-dezembro-de-2025-678388627", "https://www.anbima.com.br/feriados/fer_nacionais/2027.asp", "https://www.anbima.com.br/feriados/fer_nacionais/2028.asp"]
```

**Notes (BR)**

- **Status, per Portaria MGI nº 11.460/2025**, read from the DOU text:
  - **Feriados nacionais (10):** Confraternização Universal, Paixão de Cristo, Tiradentes, Dia
    Mundial do Trabalho, Independência, Nossa Senhora Aparecida, Finados, Proclamação da República,
    Consciência Negra, Natal.
  - **Pontos facultativos (federal optional days), included as entries:** Carnaval (Mon 16 + Tue
    17 Feb 2026) and Corpus Christi (4 Jun 2026).
  - **Other 2026 pontos facultativos, not included as entries:** Quarta-feira de Cinzas, 18 Feb
    (until 14h); bridge days 20 Apr and 5 Jun; Dia do Servidor Público, 28 Oct; 24 Dec and 31 Dec
    (from 13h).
- **Disagreement:** ANBIMA's financial-market calendar lists Carnaval and Corpus Christi among
  "feriados nacionais". They are bank and market holidays, not legal national holidays, so the
  portaria's classification wins.
  - Locally, Carnaval is a state holiday in Rio de Janeiro, and Corpus Christi is a municipal
    holiday in many cities, São Paulo among them.
- **2027:** the portaria is not published yet; it normally comes out in late December. Every date
  is fixed by law or computed from Easter, and all of them match ANBIMA 2027 and
  officeholidays.com 2027, so `tentative_2027 = false`. Carnaval and Corpus Christi should again be
  pontos facultativos.
  - In 2027, May 1, Nov 20 and Dec 25 fall on a Saturday and Nov 15 on a Monday. Brazil has no
    substitute days.
  - Checked 21 Oct 2026: no change. The 2027 portaria is still unpublished, so the ponto
    facultativo status of Carnaval and Corpus Christi is still unconfirmed.
- **2028:** all dates are fixed by law or computed from Easter, and all of them match ANBIMA 2028.
  Carnaval falls on Mon 28 + Tue 29 Feb (a leap day), and 1 Jan is a Saturday. The status of
  Carnaval and Corpus Christi waits on the 2028 portaria, due about Dec 2027.
- Consciência Negra became a national holiday under Lei 14.759/2023; 2024 was its first national
  observance.
- Paixão de Cristo rests legally on Lei 9.093/1995 (religious days), but the federal portaria lists
  it as a feriado nacional.
- Páscoa is a Sunday and is not in the portaria. It is added as an observance for "Feliz Páscoa";
  chocolate eggs are the custom.
- **12 Oct greeting:** the holiday is officially Nossa Senhora Aparecida (Padroeira do Brasil), but
  it is also Dia das Crianças, and that is what most people greet on. For a religious greeting,
  switch to "Dia de Nossa Senhora Aparecida".
- Carnaval festivities really start on Friday or Saturday (easter(-51) or easter(-50)). Widen the
  window if wanted.
- Finados uses `candles` because families light candles at graves. Its greeting is solemn, so
  there is no "Feliz".

---

## Mexico (MX)

```toml
[[holiday]]
country = "MX"
id = "new_year"
name = "Año Nuevo"
rule = "fixed(1,1)"
dates_2026 = ["2026-01-01"]
dates_2027 = ["2027-01-01"]
dates_2028 = ["2028-01-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "¡Feliz Año Nuevo!"
greeting_en = "Happy New Year"
look = "none"
source = ["https://www.gob.mx/profedet/articulos/sabes-cuales-son-los-dias-de-descanso-obligatorios-163134", "https://sidof.segob.gob.mx/notas/docFuente/5776858", "https://www.officeholidays.com/countries/mexico/2027"]

[[holiday]]
country = "MX"
id = "constitution_day"
name = "Día de la Constitución"
rule = "nth_weekday(2, mon, 1)"
dates_2026 = ["2026-02-02"]
dates_2027 = ["2027-02-01"]
dates_2028 = ["2028-02-07"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Día de la Constitución"
greeting_en = "Constitution Day"
look = "none"
source = ["https://www.gob.mx/profedet/articulos/sabes-cuales-son-los-dias-de-descanso-obligatorios-163134", "https://sidof.segob.gob.mx/notas/docFuente/5776858", "https://www.officeholidays.com/countries/mexico/2027"]

[[holiday]]
country = "MX"
id = "benito_juarez_birthday"
name = "Natalicio de Benito Juárez"
rule = "nth_weekday(3, mon, 3)"
dates_2026 = ["2026-03-16"]
dates_2027 = ["2027-03-15"]
dates_2028 = ["2028-03-20"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Natalicio de Benito Juárez"
greeting_en = "Benito Juárez Day"
look = "none"
source = ["https://www.gob.mx/profedet/articulos/sabes-cuales-son-los-dias-de-descanso-obligatorios-163134", "https://sidof.segob.gob.mx/notas/docFuente/5776858", "https://www.officeholidays.com/countries/mexico/2027"]

[[holiday]]
country = "MX"
id = "labour_day"
name = "Día del Trabajo"
rule = "fixed(5,1)"
dates_2026 = ["2026-05-01"]
dates_2027 = ["2027-05-01"]
dates_2028 = ["2028-05-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "¡Feliz Día del Trabajo!"
greeting_en = "Happy Labour Day"
look = "none"
source = ["https://www.gob.mx/profedet/articulos/sabes-cuales-son-los-dias-de-descanso-obligatorios-163134", "https://sidof.segob.gob.mx/notas/docFuente/5776858", "https://www.officeholidays.com/countries/mexico/2027"]

[[holiday]]
country = "MX"
id = "independence_day"
name = "Día de la Independencia"
rule = "fixed(9,16)"
dates_2026 = ["2026-09-16"]
dates_2027 = ["2027-09-16"]
dates_2028 = ["2028-09-16"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = true
greeting = "¡Viva México!"
greeting_en = "Happy {nth since 1810} Independence Day, Mexico"
look = "fireworks_night+flag"
source = ["https://www.gob.mx/profedet/articulos/sabes-cuales-son-los-dias-de-descanso-obligatorios-163134", "https://sidof.segob.gob.mx/notas/docFuente/5776858", "https://www.officeholidays.com/countries/mexico/2027"]

[[holiday]]
country = "MX"
id = "dia_de_muertos"
name = "Día de Muertos"
rule = ["fixed(11,1)", "fixed(11,2)"]
dates_2026 = ["2026-11-01", "2026-11-02"]
dates_2027 = ["2027-11-01", "2027-11-02"]
dates_2028 = ["2028-11-01", "2028-11-02"]
tentative_2027 = false
tentative_2028 = false
observance = true
eve = false
greeting = "¡Feliz Día de Muertos!"
greeting_en = "Happy Day of the Dead"
look = "candles"
source = ["https://ich.unesco.org/en/RL/indigenous-festivity-dedicated-to-the-dead-00054", "https://www.officeholidays.com/countries/mexico/2026", "https://www.officeholidays.com/countries/mexico/2027"]

[[holiday]]
country = "MX"
id = "revolution_day"
name = "Día de la Revolución Mexicana"
rule = "nth_weekday(11, mon, 3)"
dates_2026 = ["2026-11-16"]
dates_2027 = ["2027-11-15"]
dates_2028 = ["2028-11-20"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Día de la Revolución"
greeting_en = "Revolution Day"
look = "none"
source = ["https://www.gob.mx/profedet/articulos/sabes-cuales-son-los-dias-de-descanso-obligatorios-163134", "https://sidof.segob.gob.mx/notas/docFuente/5776858", "https://www.officeholidays.com/countries/mexico/2027"]

[[holiday]]
country = "MX"
id = "christmas"
name = "Navidad"
rule = "fixed(12,25)"
dates_2026 = ["2026-12-25"]
dates_2027 = ["2027-12-25"]
dates_2028 = ["2028-12-25"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "¡Feliz Navidad!"
greeting_en = "Merry Christmas"
look = "none"
source = ["https://www.gob.mx/profedet/articulos/sabes-cuales-son-los-dias-de-descanso-obligatorios-163134", "https://sidof.segob.gob.mx/notas/docFuente/5776858", "https://www.officeholidays.com/countries/mexico/2027"]
```

**Notes (MX)**

- **LFT art. 74** lists these mandatory rest days:
  - I. 1 Jan
  - II. First Monday of February (for 5 Feb)
  - III. Third Monday of March (for 21 Mar)
  - IV. 1 May
  - V. 16 Sep
  - VI. Third Monday of November (for 20 Nov)
  - VII. 1 Oct every six years, when the federal executive changes hands
  - VIII. 25 Dec
  - IX. Election day, as set by electoral law
- **Transmisión del Poder Ejecutivo Federal** (fr. VII): the DOF of 30 Sep 2024 moved it from
  1 Dec to 1 Oct. It applied in 2024 and next applies in **2030**. 1 Oct is an ordinary workday
  in 2026 and 2027 ([El Imparcial](https://www.elimparcial.com/dinero/2026/09/30/habra-dias-de-descanso-obligatorio-en-octubre-de-2026-esto-dice-la-lft/)),
  so there is no entry for it.
- **2027 election day** (fr. IX): the federal midterm vote is **Sun 2027-06-06** (INE calendar).
  It is a legal rest day, but it is left out because it has no greeting. Add it as `listed` if
  wanted. No federal election falls in 2028; the next is in 2030.
- **2028:** the dates come from the art. 74 rules: Constitución Mon 7 Feb, Juárez Mon 20 Mar and
  Revolución Mon 20 Nov (which is also the real anniversary). officeholidays.com has no 2028 page
  yet to cross-check. El Grito is the night of 2028-09-15.
- **Monday-shifted days** (Constitución, Juárez, Revolución): the dates are the statutory Mondays.
  The real anniversaries are 5 Feb, 21 Mar and 20 Nov, and civic acts and the Revolution parade
  are often held on those days.
- Independence has `eve = true` for **El Grito**, the night of 15 Sep (2026-09-15, 2027-09-15).
  That night brings the "¡Viva México!" shout and fireworks.
- **Día de Muertos:** inscribed by UNESCO in 2008. Ofrendas go up from late October, and the main
  nights are 1 Nov (Todos Santos, for children) and 2 Nov. It is not an LFT day.
- **Customary days that are not LFT days:**
  - Jueves and Viernes Santo (2–3 Apr 2026; 25–26 Mar 2027; 13–14 Apr 2028).
  - 5 May.
  - 12 Dec, Virgen de Guadalupe: culturally huge, and the next candidate observance.
  - 10 May, Día de las Madres: left out on purpose.
  - 12 Oct, Día de la Nación Pluricultural: civic only.

  The STPS 2026 office calendar closes on Apr 2–3, May 5 and Nov 2. That is internal to STPS, not
  law. Mexico City made 11 Jun 2026 a local holiday for the World Cup opener.
- **Verification gap:** diputados.gob.mx refused connections, so the art. 74 text comes from
  Profedet's 2018 page, which predates the 2024 fr. VII change, together with reports of the 2024
  DOF reform. No 2025–2026 reform of art. 74 was found; the 2026 LFT reforms touched other
  articles. The 2026 dates (Feb 2, Mar 16, Nov 16) match the STPS 2026 calendar, Profedet's 2 Feb
  2026 note and officeholidays.com.

---

## Nigeria (NG)

```toml
[[holiday]]
country = "NG"
id = "new_year"
name = "New Year's Day"
rule = "fixed(1,1)"
dates_2026 = ["2026-01-01"]
dates_2027 = ["2027-01-01"]
dates_2028 = ["2028-01-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy New Year"
greeting_en = "Happy New Year"
look = "none"
source = ["https://www.thisdaylive.com/2025/12/23/fg-declares-dec-25-26-2025-and-jan-1-2026-public-holidays/", "https://www.officeholidays.com/countries/nigeria/2027"]

[[holiday]]
country = "NG"
id = "eid_al_fitr"
name = "Eid-ul-Fitr (Eid-el-Fitr)"
rule = "listed"
dates_2026 = ["2026-03-20"]
dates_2027 = ["2027-03-10"]
dates_2028 = ["2028-02-27"]
tentative_2027 = true
tentative_2028 = true
observance = false
eve = true
greeting = "Barka da Sallah"
greeting_en = "Happy Eid al-Fitr {era} AH"
look = "eid"
source = ["https://www.channelstv.com/2026/03/17/fg-declares-thursday-friday-public-holidays-for-eid-el-fitr/", "https://von.gov.ng/sultan-declares-friday-20-march-as-eid-al-fitr/", "https://interior.gov.ng/?p=1661", "https://www.khaleejtimes.com/uae/ramadan-eid-al-fitr-eid-al-adha-2027-likely-dates", "https://www.officeholidays.com/countries/nigeria/2027", "https://www.timeanddate.com/holidays/nigeria/2028", "https://www.islamicfinder.org/islamic-calendar/2028/February/", "https://api.aladhan.com/v1/gToH/26-02-2028?calendarMethod=UAQ"]

[[holiday]]
country = "NG"
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
source = ["https://www.channelstv.com/2026/04/01/fg-declares-friday-monday-public-holidays-for-easter/", "https://interior.gov.ng/?p=1693", "https://www.officeholidays.com/countries/nigeria/2027"]

[[holiday]]
country = "NG"
id = "easter_sunday"
name = "Easter Sunday"
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
look = "none"
source = ["https://www.channelstv.com/2026/04/01/fg-declares-friday-monday-public-holidays-for-easter/"]

[[holiday]]
country = "NG"
id = "easter_monday"
name = "Easter Monday"
rule = "easter(+1)"
dates_2026 = ["2026-04-06"]
dates_2027 = ["2027-03-29"]
dates_2028 = ["2028-04-17"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy Easter"
greeting_en = "Happy Easter"
look = "none"
source = ["https://www.channelstv.com/2026/04/01/fg-declares-friday-monday-public-holidays-for-easter/", "https://interior.gov.ng/?p=1693", "https://www.officeholidays.com/countries/nigeria/2027"]

[[holiday]]
country = "NG"
id = "workers_day"
name = "Workers' Day"
rule = "fixed(5,1)"
dates_2026 = ["2026-05-01"]
dates_2027 = ["2027-05-01"]
dates_2028 = ["2028-05-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy Workers' Day"
greeting_en = "Happy Workers' Day"
look = "none"
source = ["https://fmino.gov.ng/fg-declares-friday-1st-may-2026-as-public-holiday-to-mark-2026-workers-day-celebration/", "https://interior.gov.ng/?p=1744", "https://www.officeholidays.com/countries/nigeria/2027"]

[[holiday]]
country = "NG"
id = "eid_al_adha"
name = "Eid-ul-Adha (Eid-el-Kabir)"
rule = "listed"
dates_2026 = ["2026-05-27", "2026-05-28"]
dates_2027 = ["2027-05-17", "2027-05-18"]
dates_2028 = ["2028-05-05", "2028-05-06"]
tentative_2027 = true
tentative_2028 = true
observance = false
eve = true
greeting = "Barka da Sallah"
greeting_en = "Happy Eid al-Adha {era} AH"
look = "eid"
source = ["https://www.channelstv.com/2026/05/25/fg-declares-wednesday-thursday-public-holidays-for-eid-ul-adha-celebration/", "https://legit.ng/1710483-breaking-sultan-announces-dhul-hijjah-date-eid-al-adha-2026.html", "https://www.khaleejtimes.com/uae/ramadan-eid-al-fitr-eid-al-adha-2027-likely-dates", "https://www.officeholidays.com/countries/nigeria/2027", "https://www.timeanddate.com/holidays/nigeria/2028", "https://www.islamicfinder.org/islamic-calendar/2028/April/", "https://api.aladhan.com/v1/gToH/05-05-2028?calendarMethod=UAQ"]

[[holiday]]
country = "NG"
id = "democracy_day"
name = "Democracy Day"
rule = "fixed(6,12)"
dates_2026 = ["2026-06-12"]
dates_2027 = ["2027-06-12"]
dates_2028 = ["2028-06-12"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy Democracy Day"
greeting_en = "Happy Democracy Day"
look = "none"
source = ["https://fmino.gov.ng/federal-government-declares-june-12-2026-public-holiday-in-commemoration-of-democracy-day/", "https://www.channelstv.com/2026/06/11/june-12-fg-declares-friday-public-holiday/", "https://www.officeholidays.com/countries/nigeria/2027"]

[[holiday]]
country = "NG"
id = "mawlid"
name = "Eid-ul-Mawlid (Eid-el-Maulud)"
rule = "listed"
dates_2026 = ["2026-08-25"]
dates_2027 = ["2027-08-15"]
dates_2028 = ["2028-08-03"]
tentative_2027 = true
tentative_2028 = true
observance = false
eve = true
greeting = "Happy Eid-el-Maulud"
greeting_en = "Mawlid of the Prophet"
look = "eid"
source = ["https://interior.gov.ng/?p=1773", "https://www.legit.ng/nigeria/1725033-sultan-sokoto-declares-friday-august-14-day-rabiul-awwal-1448-ah/", "https://www.officeholidays.com/countries/nigeria/2027", "https://islamic.app/hijri/mawlid/2027", "https://www.timeanddate.com/holidays/nigeria/2028", "https://www.islamicfinder.org/islamic-calendar/2028/August/", "https://islamic.app/hijri/mawlid/2028"]

[[holiday]]
country = "NG"
id = "independence_day"
name = "Independence Day"
rule = "fixed(10,1)"
dates_2026 = ["2026-10-01"]
dates_2027 = ["2027-10-01"]
dates_2028 = ["2028-10-01"]
tentative_2027 = false
tentative_2028 = false
observance = false
eve = false
greeting = "Happy Independence Day"
greeting_en = "Happy {nth since 1960} Independence Day, Nigeria"
look = "flag"
source = ["https://fmino.gov.ng/?p=57849", "https://www.arise.tv/fg-declares-october-1-public-holiday-for-nigerias-66th-independence-anniversary/", "https://www.officeholidays.com/countries/nigeria/2027"]

[[holiday]]
country = "NG"
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
source = ["https://www.thisdaylive.com/2025/12/23/fg-declares-dec-25-26-2025-and-jan-1-2026-public-holidays/", "https://www.officeholidays.com/countries/nigeria/2026", "https://www.officeholidays.com/countries/nigeria/2027"]

[[holiday]]
country = "NG"
id = "boxing_day"
name = "Boxing Day"
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
source = ["https://www.thisdaylive.com/2025/12/23/fg-declares-dec-25-26-2025-and-jan-1-2026-public-holidays/", "https://www.officeholidays.com/countries/nigeria/2026", "https://www.officeholidays.com/countries/nigeria/2027"]
```

**Notes (NG)**

- **Legal basis:** the Public Holidays Act (Cap P40) schedule, plus a declaration by the Minister
  of Interior for every holiday, every year. In 2026 the minister is Olubunmi Tunji-Ojo and the
  statements are signed by Permanent Secretary Magdalene Ajani.
  - Democracy Day moved from 29 May to 12 June in 2019.
  - Boxing Day is missing from the online LFN 2004 schedule text but is declared every year.
- **2026 declarations (all verified):**
  - 1 Jan.
  - Eid-ul-Fitr: **Thu 19 + Fri 20 Mar**. The Sultan fixed 1 Shawwal as Fri 20 Mar; 19 Mar was
    30 Ramadan, so only 20 Mar is in `dates_2026`.
  - Easter: Fri 3 + Mon 6 Apr.
  - 1 May.
  - Eid-ul-Adha: **Wed 27 + Thu 28 May**. The Sultan fixed 1 Dhul Hijjah as 18 May, which puts
    Eid on 27 May.
  - 12 Jun.
  - Eid-ul-Mawlid: **Tue 25 Aug**. The Sultan fixed 1 Rabi' al-Awwal as 14 Aug.
  - 1 Oct, the 66th anniversary.
  - Christmas and Boxing Day 2026: not declared yet. The statement usually comes around 22 Dec.
- **Substitute days:**
  - Act s.5 says weekend holidays get **no** day in lieu, but the minister can use s.3 to move a
    day, and does.
  - 2020 had the same calendar as 2026, and Fri 25 + **Mon 28 Dec** were declared. Expect
    **Mon 2026-12-28** for Boxing Day.
  - 2021 had the same calendar as 2027, and **Mon 27 + Tue 28 Dec** were declared, plus **Mon 14 Jun
    2021** for Democracy Day.
  - Weekend holidays in 2027: May 1 (Sat), Jun 12 (Sat), Eid-ul-Adha ~16 May (Sun), Mawlid ~15 Aug
    (Sun), Dec 25 (Sat) and Dec 26 (Sun). Monday substitutes are likely but not guaranteed.
  - Weekend holidays in 2028: 1 Jan (Sat), Eid-ul-Fitr ~27 Feb (Sun), the second Eid-ul-Adha day
    ~6 May (Sat) and 1 Oct (Sun). timeanddate already lists Mon 28 Feb as an Eid holiday.
- **2027 Islamic estimates**, each ±1 day. The Sultan and the Ministry announce 1–3 days ahead.
  - **Eid-ul-Fitr 9 Mar**, per officeholidays.com and the UAE Astronomy Centre ("9 Mar if the
    crescent is sighted on 8 Mar, else 10 Mar"). New moon is 8 Mar 09:30 UTC, so the crescent is
    only ~8 h old at Nigerian sunset. **10 Mar is a real possibility.**
  - **Eid-ul-Adha 16 May (+17)**, per officeholidays.com and the IAC ("16 or 17 May"). New moon is
    6 May 10:58 UTC, so **17 May (+18)** is possible.
  - **Mawlid 15 Aug**, per officeholidays.com and a sighting-based estimate (new moon 2 Aug
    10:05 UTC). This **disagrees** with islamic.app's tabular calendar, which gives **14 Aug**.
  - New-moon times are my own Meeus calculation.
  - Checked 21 Oct 2026: no change. There is no official 2027 notice yet.
  - Disagreement: timeanddate (snapshot of 8 Feb 2026) gives **10 Mar** for Eid-ul-Fitr and
    **17 + 18 May** for Eid-ul-Adha. My Sokoto visibility check agrees: on 8 Mar and 6 May the
    crescent is 7–8 h old with an 11–12 min lag, the kind Nigeria rejected on 29 Mar 2025, when it
    held Eid on 31 Mar, a day after Saudi Arabia. officeholidays and Umm al-Qura give 9 Mar and
    16 May. The dates are kept, but 10 Mar and 17 May are the likelier Nigerian dates.
- **2028 Islamic estimates** (`tentative_2028 = true`). Each is the date at least two sources agree
  on. "My calc" is a Meeus new-moon and Yallop-q check at sunset in Sokoto.
  - **Eid-ul-Fitr 27 Feb**, per timeanddate and my calc. Ramadan starts 28 Jan (new moon 26 Jan
    15:12 UTC). On 25 Feb (29 Ramadan) the crescent is 7 h old with an 8-min lag (new moon 25 Feb
    10:37 UTC), so Ramadan runs to 30 days.
  - Disagreement: IslamicFinder and Umm al-Qura (AlAdhan) give **26 Feb**, a Saudi-style date.
  - **Eid-ul-Adha 5 May (+6)**, per IslamicFinder (1 Dhul Hijjah = 26 Apr), Umm al-Qura and my
    calc. The new moon is 24 Apr 19:47 UTC, after sunset. On 25 Apr the crescent is 22 h old with a
    43-min lag (Yallop B), which Nigeria accepts.
  - Disagreement: timeanddate gives **6 May (+7)**; it treats the 25 Apr crescent as not visible.
  - **Mawlid 3 Aug**, per timeanddate, IslamicFinder (1 Rabi' al-Awwal = 23 Jul) and Umm al-Qura.
  - Disagreement: islamic.app's tabular calendar gives **2 Aug**. My calc gives 3 Aug only if the
    22 Jul crescent is accepted: it is 15 h old with a 27-min lag (Yallop F; new moon 22 Jul 03:01
    UTC). Nigeria accepted a 14 h, 27-min crescent on 20 Apr 2023, but **4 Aug** is possible.
- Islamic days have `eve = true` because they begin at sunset.
- **Greetings:** "Barka da Sallah" (Hausa) is the most typically Nigerian Eid greeting and is used
  nationwide; "Eid Mubarak" is also common. Swap it in if preferred, and the same goes for
  "Barka da Maulud".
- **Looks:** Mawlid uses `eid` because Nigerians call it Eid-el-Maulud. Use `none` if the crescent
  should be kept for Fitr and Adha only. Independence has no fireworks tradition, so its look is
  `none` until `flag` exists.
- **Not federal holidays:** Children's Day (27 May, schools only); Islamic New Year (state holidays
  in several northern states); Isese Day (some south-western states); ad-hoc election-day work
  breaks.
- officeholidays.com 2026 lists 19 Mar as "Eid-el-fitri Sallah". It is a declared holiday, but it
  is not the Eid day itself.

---

## Counts

| Country | Holidays | Observances | 2027 status | 2028 status |
|---|---|---|---|---|
| US | 11 federal | 1 (Easter Sunday) | official (OPM lists 2027) | official (OPM lists 2028) |
| UK | 8 E&W bank holidays | 3 (Easter Sunday, Bonfire Night, Remembrance Sunday) | official (gov.uk lists 2027) | official (gov.uk lists 2028) |
| BR | 10 feriados + 2 pontos facultativos (Carnaval ×2 days, Corpus Christi) | 1 (Páscoa) | dates certain by law/Easter; 2027 portaria not yet published (checked 21 Oct 2026) | dates certain by law/Easter; match ANBIMA 2028 |
| MX | 7 LFT days | 1 (Día de Muertos ×2 days) | dates certain by LFT rules (no 2027 decree needed) | dates certain by LFT rules |
| NG | 11 (incl. 3 Islamic) | 1 (Easter Sunday) | fixed/Easter dates certain; 3 Islamic dates estimated; all still need yearly declaration | same; 3 Islamic dates estimated (`tentative_2028 = true`) |

## Review changes (21 Oct 2026)

- NG 2027: Eid al-Fitr 10 Mar and Eid al-Adha 17–18 May (timeanddate and a Sokoto crescent
  check; Nigeria rejected a similar crescent in 2025). Still tentative.
