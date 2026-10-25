#!/usr/bin/env python3
"""Record the README demo GIFs.

Runs weathrz in a pseudo-terminal for each scene, saves the output as an asciicast v2
file, then renders it with agg (https://github.com/asciinema/agg). Each scene gets its
own config and its place's time zone, so --date and event times are local there.

usage: python3 tools/record_demos.py --font-dir <dir with a Nerd Font> \
           [--bin weathrz] [--agg agg] [--out docs/demo] [--only day,storm]

Linux and macOS only (it needs the pty module).
"""
import argparse
import fcntl
import json
import os
import pty
import select
import signal
import struct
import subprocess
import tempfile
import termios
import time

COLS, ROWS = 120, 32
SETTLE_SECS = 1.5
FONTS = "JetBrainsMono Nerd Font Mono,DejaVu Sans Mono,DejaVu Sans,Unifont"
TOKYO_NIGHT = ",".join([
    "1a1b26", "c0caf5",
    "15161e", "f7768e", "9ece6a", "e0af68", "7aa2f7", "bb9af7", "7dcfff", "a9b1d6",
    "414868", "f7768e", "9ece6a", "e0af68", "7aa2f7", "bb9af7", "7dcfff", "c0caf5",
])

PLACES = {
    "lisbon": (38.7223, -9.1393, "Lisbon", "Europe/Lisbon"),
    "oslo": (59.9139, 10.7522, "Oslo", "Europe/Oslo"),
    "delhi": (28.6139, 77.2090, "New Delhi", "Asia/Kolkata"),
    "tokyo": (35.6762, 139.6503, "Tokyo", "Asia/Tokyo"),
    "seoul": (37.5665, 126.9780, "Seoul", "Asia/Seoul"),
    "bangkok": (13.7563, 100.5018, "Bangkok", "Asia/Bangkok"),
    "rio": (-22.9068, -43.1729, "Rio de Janeiro", "America/Sao_Paulo"),
    "addis": (9.0300, 38.7400, "Addis Ababa", "Africa/Addis_Ababa"),
    "kl": (3.1390, 101.6869, "Kuala Lumpur", "Asia/Kuala_Lumpur"),
    "sydney": (-33.8688, 151.2093, "Sydney", "Australia/Sydney"),
    "mexico": (19.4326, -99.1332, "Mexico City", "America/Mexico_City"),
}
PAINTED = {"painted": True}

SCENES = [
    ("day", "lisbon", ["--simulate", "clear", "--date", "2026-06-10T11:00", "--flyer", "kite"], {}),
    ("storm", "lisbon", ["--simulate", "thunderstorm", "--night"], PAINTED),
    ("snow", "oslo", ["--simulate", "snow", "--date", "2026-01-15T12:00"], {}),
    ("chuseok", "seoul", ["--event", "kr:chuseok", "--simulate", "clear"], PAINTED),
    ("songkran", "bangkok", ["--event", "th:songkran", "--simulate", "clear"], {}),
    ("carnival", "rio", ["--event", "br:carnival", "--simulate", "clear"], {}),
    ("meskel", "addis", ["--event", "et:meskel", "--simulate", "clear"], {}),
    ("diwali", "delhi", ["--event", "in:diwali", "--simulate", "clear"], {}),
    ("hari-raya", "kl", ["--event", "my:eid_al_fitr", "--simulate", "clear"], {}),
    ("childrens-day", "tokyo", ["--event", "jp:childrens_day", "--simulate", "clear"], {}),
    ("anzac-day", "sydney", ["--event", "au:anzac_day_half_mast", "--simulate", "clear"], {}),
    ("new-year", "lisbon", ["--event", "new_year", "--simulate", "clear"], {}),
    ("halloween", "lisbon", ["--event", "halloween", "--simulate", "clear", "--flyer", "witch"], {}),
    ("details", "mexico", ["--simulate", "partly-cloudy", "--date", "2026-09-16T10:00"], {"keys": "i"}),
]
KEYS_AFTER_SECS = 1.0


def config_for(place, extras):
    latitude, longitude, city, _ = PLACES[place]
    glyphs = "fine" if extras.get("painted") else "unicode"
    sky = "[sky]\npainted = true\n" if extras.get("painted") else ""
    return (
        f'theme = "tokyo-night"\nicons = "nerd"\nglyphs = "{glyphs}"\nsilent = true\n'
        f'[location]\nauto = false\nlatitude = {latitude}\nlongitude = {longitude}\n'
        f'city = "{city}"\ndisplay = "city"\n{sky}'
    )


def record(argv, env, secs, keys=""):
    pid, fd = pty.fork()
    if pid == 0:
        fcntl.ioctl(0, termios.TIOCSWINSZ, struct.pack("HHHH", ROWS, COLS, 0, 0))
        os.execvpe(argv[0], argv, env)
    events, start = [], time.monotonic()
    while time.monotonic() - start < secs + SETTLE_SECS:
        if keys and time.monotonic() - start > SETTLE_SECS + KEYS_AFTER_SECS:
            os.write(fd, keys.encode())
            keys = ""
        if not select.select([fd], [], [], 0.02)[0]:
            continue
        try:
            chunk = os.read(fd, 65536)
        except OSError:
            break
        if not chunk:
            break
        events.append((time.monotonic() - start, chunk))
    os.kill(pid, signal.SIGTERM)
    try:
        while select.select([fd], [], [], 0.3)[0] and os.read(fd, 65536):
            pass
    except OSError:
        pass
    os.waitpid(pid, 0)
    return events


def write_cast(events, path):
    pending = b""
    with open(path, "w", encoding="utf-8") as out:
        out.write(json.dumps({"version": 2, "width": COLS, "height": ROWS}) + "\n")
        for at, chunk in events:
            data = pending + chunk
            try:
                text, pending = data.decode("utf-8"), b""
            except UnicodeDecodeError as err:
                if err.reason != "unexpected end of data":
                    text, pending = data.decode("utf-8", errors="replace"), b""
                else:
                    text = data[: err.start].decode("utf-8", errors="replace")
                    pending = data[err.start:]
            if text:
                out.write(json.dumps([round(max(0.0, at - SETTLE_SECS), 4), "o", text]) + "\n")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--font-dir", required=True)
    parser.add_argument("--bin", default="weathrz")
    parser.add_argument("--agg", default="agg")
    parser.add_argument("--out", default="docs/demo")
    parser.add_argument("--secs", type=float, default=10)
    parser.add_argument("--only", default="")
    opts = parser.parse_args()
    wanted = set(filter(None, opts.only.split(",")))
    os.makedirs(opts.out, exist_ok=True)

    with tempfile.TemporaryDirectory() as scratch:
        for name, place, args, extras in SCENES:
            if wanted and name not in wanted:
                continue
            config_home = os.path.join(scratch, place)
            os.makedirs(os.path.join(config_home, "weathrz"), exist_ok=True)
            with open(os.path.join(config_home, "weathrz", "config.toml"), "w") as config:
                config.write(config_for(place, extras))
            env = dict(os.environ, TERM="xterm-256color", COLORTERM="truecolor",
                       LANG="C.UTF-8", TZ=PLACES[place][3], XDG_CONFIG_HOME=config_home,
                       XDG_CACHE_HOME=os.path.join(config_home, "cache"))
            cast = os.path.join(scratch, f"{name}.cast")
            write_cast(record([opts.bin, *args], env, opts.secs, extras.get("keys", "")), cast)
            gif = os.path.join(opts.out, f"{name}.gif")
            subprocess.run([opts.agg, "--quiet", "--font-dir", opts.font_dir,
                            "--text-font-family", FONTS, "--theme", TOKYO_NIGHT,
                            "--font-size", "14", "--fps-cap", "15", cast, gif], check=True)
            print(f"{gif}: {os.path.getsize(gif) // 1024} KiB")


if __name__ == "__main__":
    main()
