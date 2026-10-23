# Working on weathrz

weathrz is a fork of [Veirt/weathr](https://github.com/Veirt/weathr) (GPL-3.0-or-later,
see `LICENSE`). The [README](README.md) describes the app; this file covers how the repo
relates to weathr, pulling weathr's updates, adding a character to the sky, updating the
holiday tables, the known gaps and ideas for later.

## The repo and weathr

- `main` starts with weathr's own commits up to the fork point (`7d403a0`, 12 August
  2026), followed by the weathrz changes, so weathr's authorship stays in the history and
  its later commits merge in.
- Package and binary are `weathrz`; the library crate keeps the name `weathr`
  (`[lib]` in `Cargo.toml`) so the `use weathr::…` lines match upstream and merges stay
  small. The config folder is `weathrz`, falling back to `weathr` until it exists.
- Left out on purpose: upstream's release, Homebrew and Docker workflows, `install.sh`,
  the Dockerfile, `Cross.toml` and the nix files, which build and publish weathr itself,
  and upstream's demo GIFs in `docs/`, which show weathr (weathrz's are in `docs/demo/`).

## Pull updates from weathr

Add weathr once as a fetch-only remote. Never push to it: it is the original author's
repo.

```bash
git remote add upstream https://github.com/Veirt/weathr.git
git remote set-url --push upstream DISABLED
```

Then, for each sync:

```bash
git fetch upstream
git log --oneline main..upstream/main     # what weathr added since the last sync
git checkout -b sync-upstream main
git merge upstream/main                   # fix conflicts if any, then commit
cargo test --locked
git checkout main && git merge --ff-only sync-upstream
```

Conflicts are most likely in `src/app.rs`, `src/animation_manager.rs`, `src/cli.rs`,
`README.md`, `Cargo.toml` and `Cargo.lock`. Keep the package name `weathrz` and the
`[lib] name = "weathr"` lines; for `Cargo.lock`, take either side and run `cargo build`
to settle it. When upstream changes a file weathrz left out (packaging, see above),
keep it out with `git rm <file>`.

## Add your own character to the sky

Characters are built in: you add two text files and one entry in the code, then
rebuild. The example below adds a paper crane with the id `paper-crane`.

1. **Draw the art** in `src/animation/assets/characters/paper-crane.txt`:

   ```text
   # Paper crane, drawn by <you>
    __/\
   <__  >==
      \/
   ---
       /\
   <__  >==
    ~~\/
   ```

   - The first line is a `#` comment with the name and the credit. Lines starting with `#`
     are skipped.
   - Draw it facing right, or facing left if you register it as `left` in step 3.
   - A line holding only `---` starts the next frame. Frames play in turn, one every
     0.3 s. One frame is fine; two give a flap, a blink or a wheel turn.
   - Give every frame the same number of lines, and pad with empty lines if needed.
     Columns line up across frames, because one shared indent is trimmed from all of
     them.
   - Keep it 9 lines tall or less. Width is free; the long dragon is 59 columns.
   - Use plain ASCII. Trailing spaces do not matter.

2. **Colour it** in `src/animation/assets/characters/paper-crane.mask.txt`. The mask
   copies the art's shape: every non-space character in the art gets a letter, and every
   space stays a space, frame for frame and line for line.

   ```text
   # mask for paper-crane.txt
    ffff
   fff  fyy
      ff
   ---
       ff
   fff  fyy
    ccff
   ```

   Each letter picks a colour from the active theme:

   | Letter | Colour | Letter | Colour | Letter | Colour |
   |---|---|---|---|---|---|
   | `f` | foreground | `y` | yellow | `b` | blue |
   | `u` | muted | `g` | green | `B` | dark blue |
   | `s` | subtle | `G` | dark green | `m` | magenta |
   | `r` | red | `c` | cyan | `n` | brown |
   | `o` | orange | `t` | tan | `N` | dark brown |

3. **Register it** at the end of `CHARACTERS` in `src/animation/flyers/characters.rs`:

   ```rust
       FlyerDef {
           id: "paper-crane",
           credit: "drawn by <you>",
           right: Some(include_str!("../assets/characters/paper-crane.txt")),
           left: None,
           fill: Fill::Mirror,
           pace: Pace::Slow,
           when: When::Day,
           colouring: Colouring::Mask,
           kind: Kind::Character,
           mask: Some(include_str!("../assets/characters/paper-crane.mask.txt")),
       },
   ```

   - `right` / `left`: the direction the art faces. Set exactly one of them.
   - `fill`: `Mirror` flips the art for the other direction. It only works when every
     character has a mirror image, such as `/` and `\`, `(` and `)`, `<` and `>`, `[` and
     `]`, `{` and `}`, or symmetric ones like `-`, `_`, `=`, `|`, `.`, `o`, `~`, `^`. Letters
     such as `S` or `F` cannot flip, so use `None` (one direction only) for art with text.
     `Same` flies the same art both ways.
   - `pace`: `Slow`, `Normal` or `Fast`.
   - `when`: `Any`, `Day`, `Night`, `CalmDay` (day, wind under 20 km/h) or `MoonNight`
     (night with the moon showing; it then rides along the moon's row).
   - Keep `colouring: Colouring::Mask` and `kind: Kind::Character`.

4. **Update the roster tests** in `src/animation/flyers/catalogue.rs`:
   - in `characters_fly_their_pinned_directions`, add `("paper-crane", "both ways")` at the
     end (`"right"` or `"left"` for a one-way piece; `--list-flyers` shows which);
   - in `nine_characters`, change both `9`s to the new count, and optionally rename the test.

   Then update the `characters = []` comment in `README.md` with the new id.

5. **Test and preview:**

   ```bash
   cargo test --locked
   cargo run -- --list-flyers
   cargo run -- --simulate clear --flyer paper-crane
   ```

   The tests check the `#` header lines, that the mask matches the art cell for cell,
   that it uses known letters, the height limit, and that mirrored frames stay aligned.
   `the_sky_carries_no_borrowed_names` also turns away the names of well-known film and
   comic characters: draw your own rather than someone else's.
   For night pieces, add `--night`, or `--date 2026-10-26T21:00` for a full moon.

6. **Install:** `cargo install --path . --locked --force`.

About one flight in 12 is a character. To see yours more often, or to pick which
characters fly, set this in `~/.config/weathrz/config.toml`:

```toml
[sky]
characters = ["paper-crane", "ufo"]   # empty = all of them
specials = 3                           # about one flight in 3 is a character
every = "busy"                         # a flight every 20-40 s
```

Aircraft work the same way, in `CATALOGUE` in `src/animation/flyers/catalogue.rs`, with
`kind: Kind::Aircraft` and `colouring: Colouring::Airplane`. Their art lives in
`src/animation/assets/flyers/`, and they take colours from the theme's airplane tiers
instead of a mask.

## Refresh the README demos

`tools/record_demos.py` runs each scene in a pseudo-terminal at 120 × 32, saves it as an
asciicast and renders it to `docs/demo/<scene>.gif` with
[agg](https://github.com/asciinema/agg). It needs Linux or macOS, agg, and a folder
holding a Nerd Font (for example JetBrainsMono from
<https://github.com/ryanoasis/nerd-fonts/releases>):

```bash
cargo install --path . --locked --force
cargo install --git https://github.com/asciinema/agg --tag v1.9.0 --locked
python3 tools/record_demos.py --font-dir ~/fonts/JetBrainsMono          # every scene
python3 tools/record_demos.py --font-dir ~/fonts/JetBrainsMono --only day,storm
```

The scenes, their places and their flags are listed at the top of the script. Each one
runs with its own config and its place's time zone, so `--date` and event showcase
times are local there.

## Known gaps

Small things left for later:

- The moon biker flies only while the real moon sits in the sky band between the bar and
  the roofline; a low moon keeps it home, and so do small panes.
- A low moon (and the sun) can sit behind the house or the trees: they are drawn behind
  the scene, as they would be.
- The sun's compass direction and the providers' sunrise and sunset times are computed
  or fetched but not used yet.
- On panes 12–16 rows tall the tallest trees sink their trunks into the soil rows.
- Past 2028 the moon days are calculated (tentative, about a day either way) until the
  tables get the next year's dates (see "Update the holiday dates" below); Hindu
  festivals, India's Buddha Purnima and announced days (cuti bersama, Russia's New
  Year run, China's long blocks are kept in shape) go quiet. Rule-based days keep
  going. Every 2028 moon or
  announcement date is an estimate until the governments publish (none had by 21 Oct
  2026).
- The country comes from the nearest of about 1,000 places; in a border town (Johor
  Bahru vs Singapore, Amritsar vs Lahore) check `weathrz --list-events` and set
  `[events] country` if it guessed wrong. Hong Kong, Singapore and countries outside
  the 14 get the yearly events only.
- Japan's days are greetings only, and substitute days off (US observed days, UK
  substitute bank holidays, Japan's furikae kyujitsu) are not shown: each holiday shows
  on its own date.
- With `icons = "ascii"` on panes under about 90 columns, the bar drops a long holiday
  greeting to keep the weather; with Nerd icons it shrinks to the icon.
- Each flyer's hours are set by its `when` field in `src/animation/flyers/characters.rs`
  or `catalogue.rs`; the test `every_flyer_keeps_its_hours` lists them all.

## Update the holiday dates

Each September to December the governments publish next year's list. 2028 is in as
estimates; when a list comes out, replace that country's `dates_2028` and set
`tentative_2028 = false`. To add 2029:

1. For Indonesia, add one row per occurrence to `src/events/dates.rs` (`tentative` until
   the isbat) and bump `LAST_YEAR` there. For the other countries, add a
   `dates_2029 = [...]` line (and `tentative_2029 = true` when the list is not
   official yet) to each holiday in `docs/holidays/group-*.md`, and to `extras.md` for
   Ramadan, and add the Chinese New Year's Day to `LUNAR_NEW_YEAR` in
   `tools/world_holidays.py`. Rule-based days need nothing: they are computed.
2. Run `python3 tools/world_holidays.py` (it rewrites `src/events/world.rs` and runs
   rustfmt), then `cargo test --locked`. The test
   `every_researched_world_date_falls_inside_its_window` checks every listed date.
3. Rebuild and install (`cargo install --path . --locked --force`).

## Ideas

In rough order of value:

1. Swap estimates for official dates as governments publish them: next year's lists
   come out between September and December, and Islamic days in countries that sight the
   moon can still move a day (see the notes in `docs/holidays/`).
2. A painted sky: a time-of-day gradient behind the scene (the renderer has
   backgrounds), filled clouds, a glow from lit windows at night.
3. A details card on `i` (hourly outlook, sunrise and sunset, AQI band) and `--json`.
4. Observances that are not days off, per country, and a half-mast flag on days of
   mourning.
5. The rain overhaul: rain behind the house as well as in front, splashes on the roof
   and grass, and a few rain sheets.
6. More languages behind `language = "..."` (every word is in `src/text.rs` and the
   greetings in `src/events/`).
7. More countries: a country is a block of holidays in `docs/holidays/` plus a row in
   `src/events/country.rs`.
