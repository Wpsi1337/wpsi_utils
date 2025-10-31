#!/usr/bin/env python3
# Source for biomes: https://poe2db.tw/us/Waystones#EndgameMaps (snapshot on 2025-10-30)

import os, re, sys, time, unicodedata, subprocess
from datetime import datetime
from typing import Optional, List, Dict, Tuple
def _bootstrap_pip() -> bool:
    try:
        print("Bootstrapping pip via ensurepip...", file=sys.stderr)
        subprocess.run(
            [sys.executable, "-m", "ensurepip", "--default-pip"],
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        return True
    except Exception as exc:  # pragma: no cover - platform specific
        print(f"ensurepip failed: {exc}", file=sys.stderr)
        return False

def _print_capture(stdout: Optional[bytes], stderr: Optional[bytes]):
    out_text = (stdout or b"").decode("utf-8", "ignore").strip()
    err_text = (stderr or b"").decode("utf-8", "ignore").strip()
    if out_text:
        print(out_text)
    if err_text:
        print(err_text, file=sys.stderr)

def _prompt_exit_if_needed(message: str):
    if os.name != "nt":
        return
    if sys.stdin is not None and sys.stdin.isatty():
        return
    print(message)
    try:
        import msvcrt  # type: ignore
        print("Press any key to close this window…")
        msvcrt.getch()
    except Exception:
        try:
            input("Press Enter to close this window…")
        except Exception:
            time.sleep(4)

def _install_windows_curses() -> bool:
    def _run(cmd: List[str]) -> subprocess.CompletedProcess:
        return subprocess.run(cmd, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

    try:
        print("Installing windows-curses with pip...", file=sys.stderr)
        result = _run([sys.executable, "-m", "pip", "install", "windows-curses"])
        _print_capture(result.stdout, result.stderr)
        return True
    except subprocess.CalledProcessError as err:
        stderr = err.stderr or b""
        stdout = err.stdout or b""
        combined = (stdout + b"\n" + stderr).decode("utf-8", "ignore")
        _print_capture(stdout, stderr)
        if "No module named pip" in combined or "No module named pip" in repr(err):
            if _bootstrap_pip():
                try:
                    result = _run([sys.executable, "-m", "pip", "install", "windows-curses"])
                    _print_capture(result.stdout, result.stderr)
                    return True
                except Exception as inner_exc:  # pragma: no cover - platform specific
                    print(f"pip install failed after bootstrapping: {inner_exc}", file=sys.stderr)
        else:
            print(f"pip install failed: {combined.strip()}", file=sys.stderr)
    except Exception as exc:  # pragma: no cover - platform specific
        print(f"Unexpected error while installing windows-curses: {exc}", file=sys.stderr)
    return False

def _load_curses():
    try:
        import curses as _curses
        return _curses
    except ImportError as exc:
        if os.name == "nt":
            print("Missing 'windows-curses'; attempting automatic installation...", file=sys.stderr)
            if _install_windows_curses():
                import importlib
                importlib.invalidate_caches()
                import curses as _curses  # type: ignore
                return _curses
            _prompt_exit_if_needed(
                "windows-curses installation failed. "
                "Install manually via:\n"
                "    py -m ensurepip --default-pip\n"
                "    py -m pip install windows-curses"
            )
            raise SystemExit(
                "windows-curses is required on Windows. Install it manually with:\n"
                "    py -m ensurepip --default-pip\n"
                "    py -m pip install windows-curses"
            ) from exc
        raise

curses = _load_curses()

def detect_default_log_path() -> str:
    override = os.environ.get("POE2_LOG_PATH")
    if override:
        return os.path.expanduser(os.path.expandvars(override))

    candidates = []
    if os.name == "nt":
        program_files_x86 = os.environ.get("PROGRAMFILES(X86)", r"C:\Program Files (x86)")
        program_files = os.environ.get("PROGRAMFILES", r"C:\Program Files")
        candidates.extend(
            [
                rf"{program_files_x86}\Steam\steamapps\common\Path of Exile 2\logs\Client.txt",
                rf"{program_files}\Steam\steamapps\common\Path of Exile 2\logs\Client.txt",
                r"%USERPROFILE%\Saved Games\Path of Exile 2\logs\Client.txt",
                r"%USERPROFILE%\Documents\My Games\Path of Exile 2\logs\Client.txt",
            ]
        )
    # Linux / macOS defaults
    candidates.extend(
        [
            "~/.steam/steam/steamapps/common/Path of Exile 2/logs/Client.txt",
            "~/Library/Application Support/Path of Exile 2/logs/Client.txt",
        ]
    )

    for candidate in candidates:
        path = os.path.expanduser(os.path.expandvars(candidate))
        parent = os.path.dirname(path)
        if os.path.exists(path) or (parent and os.path.isdir(parent)):
            return path

    # Fall back to the first candidate, even if the directory is missing.
    fallback = os.path.expanduser(os.path.expandvars(candidates[0]))
    return fallback

LOG_PATH = detect_default_log_path()

# ---------- Log patterns ----------
ENTERED_RE     = re.compile(r"You have entered (?P<zone>.+?)(?:\.)?$", re.IGNORECASE)
GENERATING_RE  = re.compile(r'Generating level \d+ area "(?P<area>[^"]+)"', re.IGNORECASE)
MESSAGE_SPLIT  = re.compile(r"]\s*:\s*(.*)$")

# ---------- Colors ----------
BIOME_COLOR = {
    "Swamp": "92", "Forest": "32", "Grass": "92",
    "Desert": "33", "Mountain": "90", "Water": "36",
    "Vaal City": "95", "Ezomyte City": "35", "Faridun City": "35",
    "Cemetery": "90", "Ruins": "90", "Unknown": "37"
}
DIM = "90"

def color(text: str, code: str) -> str:
    return f"\033[{code}m{text}\033[0m"

# ---------- Name normalization ----------
def normalize_zone(name: str) -> str:
    n = unicodedata.normalize("NFKC", name).replace("\r", "").strip().rstrip(".")
    n = " ".join(n.split())
    if n.lower().startswith("the "):
        n = n[4:]
    return " ".join(w[:1].upper() + w[1:] for w in n.split())

def token_to_zone(token: str) -> str:
    t = token.strip()
    if t.startswith("Map") and len(t) > 3:
        t = t[3:]
    t = re.sub(r"[_\-]+", " ", t)
    words = re.findall(r"[A-Z]?[a-z]+|[A-Z]+(?![a-z])|\d+", t)
    return normalize_zone(" ".join(words))

# ---------- STATIC BIOME MAP (from poe2db Waystones → Endgame Maps) ----------
# Multiple biomes are joined with " / " and duplicates removed.
BIOME_MAP = {
    # Unique/Atlas nodes
    "The Ziggurat Refuge": "Swamp / Grass / Forest",
    "The Stone Citadel": "Vaal City",
    "The Iron Citadel": "Ezomyte City",
    "The Copper Citadel": "Faridun City",

    # Water / coast
    "Castaway": "Water",
    "Untainted Paradise": "Water",
    "The Fractured Lake": "Water",
    "Crimson Shores": "Water",
    "Rockpools": "Water",
    "Wayward Isle": "Water",
    "The Jade Isles": "Water",
    "Stronghold": "Water",
    "Cliffside": "Water",
    "Sinkhole": "Water",
    "Caldera": "Water",

    # Trader variants
    "Merchant's Campsite": "Swamp / Forest / Desert / Grass",
    "Moment of Zen": "Water / Swamp / Forest",

    # Hideouts
    "Felled Hideout": "Forest",
    "Limestone Hideout": "Water",
    "Shrine Hideout": "Desert",
    "Canal Hideout": "Grass",
    "Farmlands Hideout": "Grass",
    "Prison Hideout": "Swamp",

    # Forest / grass / swamp clusters
    "Blooming Field": "Grass / Forest",
    "Savannah": "Grass / Desert",
    "Lost Towers": "Forest",
    "Mire": "Swamp / Forest",
    "Woodland": "Forest",
    "Sump": "Swamp",
    "Willow": "Forest",
    "Seepage": "Swamp / Grass / Forest",
    "Riverside": "Forest",
    "Steppe": "Grass",
    "Spider Woods": "Forest",
    "Bloodwood": "Forest",
    "Hidden Grotto": "Swamp / Grass / Forest / Mountain",
    "Augury": "Swamp / Grass / Forest",
    "Bastille": "Grass / Forest",
    "Creek": "Forest",
    "Decay": "Swamp / Grass / Forest",
    "Derelict Mansion": "Swamp / Grass / Forest",
    "Overgrown": "Forest",
    "Riverhold": "Grass / Forest",
    "Flotsam": "Grass / Forest",
    "Trenches": "Swamp / Forest",

    # Desert / city / mountains etc.
    "Fortress": "Desert",
    "Penitentiary": "Ezomyte City / Grass",
    "Sandspit": "Water",
    "Forge": "Grass / Desert / Mountain",
    "Sulphuric Caverns": "Swamp / Mountain / Desert",
    "Headland": "Faridun City / Mountain / Desert",
    "Lofty Summit": "Mountain",
    "Necropolis": "Ezomyte City / Forest",
    "Crypt": "Grass / Desert / Mountain",
    "Steaming Springs": "Grass / Forest / Mountain",
    "Slick": "Grass / Desert / Mountain",
    "Marrow": "Grass / Desert / Mountain",
    "Vaal City": "Vaal City / Swamp",
    "Cenotes": "Mountain / Swamp",
    "Ravine": "Mountain / Forest",
    "Alpine Ridge": "Mountain / Ezomyte City",
    "Grimhaven": "Ezomyte City / Grass",
    "Hive": "Desert",
    "Mineshaft": "Faridun City / Mountain / Desert",
    "Oasis": "Faridun City / Desert",
    "Outlands": "Faridun City",
    "Sinking Spire": "Vaal City / Swamp",
    "Vaal Village": "Vaal City / Swamp",
    "Rustbowl": "Desert",
    "Backwash": "Swamp / Forest",
    "Burial Bog": "Swamp",
    "Wetlands": "Swamp",
    "Sun Temple": "Vaal City / Swamp",
    "Channel": "Faridun City / Desert",
    "Vaal Foundry": "Vaal City / Mountain / Desert",
    "The Assembly": "Vaal City / Grass",
    "Mesa": "Faridun City / Desert",
    "Bluff": "Grass",
    "The Ezomyte Megaliths": "Grass / Forest",
    "Azmerian Ranges": "Mountain / Forest",
    "Frozen Falls": "Mountain",
    "Sealed Vault": "Mountain",
    "Sacred Reservoir": "Desert",
    "Ornate Chambers": "Mountain / Desert",
    "Canyon": "Mountain / Desert",
    "Confluence": "Desert",
    "Razed Fields": "Grass",
    "Rugosa": "Water",
    "Digsite": "Grass",
    "Ice Cave": "Mountain",
    "Rupture": "Swamp",
    "Spring": "Desert",

    # Boss/Unique nodes
    "Vaults of Kamasa": "Vaal City",
    "The Viridian Wildwood": "Swamp / Forest",
    "The Silent Cave": "Mountain / Desert",
}

# ---------- Biome loot notes ----------
BIOME_NOTES = {
    "Swamp":   "40% increased chance to drop Basic Currency",
    "Water":   "40% increased chance to drop Basic Currency",
    "Mountain":"40% increased chance to drop Gold",
    "Forest":  "40% increased chance to drop Jewels",
    "Desert":  "40% increased chance to drop Baryas and Inscribed Ultimatums",
    "Grass":   "40% increased chance to drop Socket Currency",
}

def resolve_biome(zone: str) -> str:
    key = normalize_zone(zone)
    return BIOME_MAP.get(key, "Unknown")

def split_biomes(biome_field: str) -> List[str]:
    return [b.strip() for b in biome_field.split("/")]

def biome_notes(biome_field: str) -> List[str]:
    notes = []
    seen = set()
    for b in split_biomes(biome_field):
        note = BIOME_NOTES.get(b)
        if note and note not in seen:
            seen.add(note)
            notes.append(note)
    return notes

# ---------- Search ----------
def search_biomes(term: str) -> Tuple[List[Tuple[str, str]], Dict[str, List[str]]]:
    query = term.strip()
    if not query:
        return [], {}

    q = query.lower()
    zone_matches: List[Tuple[str, str]] = []
    biome_matches: Dict[str, List[str]] = {}

    for zone, biome in sorted(BIOME_MAP.items()):
        if q in zone.lower():
            zone_matches.append((zone, biome))
        for token in split_biomes(biome):
            if q in token.lower():
                biome_matches.setdefault(token, []).append(zone)

    for token, zones in biome_matches.items():
        zones.sort()

    return zone_matches, biome_matches

# ---------- Log parsing ----------
def parse_zone(line: str) -> Optional[str]:
    line = line.replace("\r", "")
    m = MESSAGE_SPLIT.search(line)
    msg = m.group(1) if m else line
    msg = msg.strip()

    m1 = ENTERED_RE.search(msg)
    if m1:
        return normalize_zone(m1.group("zone"))

    m2 = GENERATING_RE.search(line)
    if m2:
        return token_to_zone(m2.group("area"))

    return None

class LogFollower:
    def __init__(self, path: str, poll_interval: float = 0.2):
        self.path = path
        self.poll_interval = poll_interval
        self._file = None
        self._last_open_attempt = 0.0
        self.last_error: Optional[str] = None

    def _try_open(self):
        if self._file is not None:
            return
        now = time.time()
        if now - self._last_open_attempt < self.poll_interval:
            return
        self._last_open_attempt = now
        try:
            f = open(self.path, "r", encoding="utf-8", errors="replace")
            f.seek(0, os.SEEK_END)
            self._file = f
            self.last_error = None
        except FileNotFoundError:
            self.last_error = f"waiting for log file {self.path}"

    def _check_rotation(self):
        if self._file is None:
            return
        try:
            st = os.stat(self.path)
        except FileNotFoundError:
            try:
                self._file.close()
            except Exception:
                pass
            self._file = None
            self.last_error = f"waiting for log file {self.path}"
            return
        if self._file.tell() > st.st_size:
            try:
                self._file.close()
            except Exception:
                pass
            self._file = None
            self.last_error = "log rotated, reopening…"

    def poll(self) -> List[str]:
        self._try_open()
        if self._file is None:
            return []

        lines = []
        while True:
            line = self._file.readline()
            if line:
                lines.append(line.rstrip("\n"))
            else:
                break

        self._check_rotation()
        return lines

# ---------- Output helpers ----------
def pick_color_for_biome(biome: str) -> str:
    token = biome.split("/")[0].strip()
    return BIOME_COLOR.get(token, "37")

ANSI_TO_CURSES = {
    "92": curses.COLOR_GREEN,
    "32": curses.COLOR_GREEN,
    "33": curses.COLOR_YELLOW,
    "90": curses.COLOR_CYAN,
    "36": curses.COLOR_CYAN,
    "95": curses.COLOR_MAGENTA,
    "35": curses.COLOR_MAGENTA,
    "37": curses.COLOR_WHITE,
}

class ZoneWatcherApp:
    def __init__(self, screen):
        self.screen = screen
        self.follower = LogFollower(LOG_PATH)
        self.events: List[Dict[str, object]] = []
        self.last_zone: Optional[str] = None
        self.status_message = ""
        self.search_query = ""
        self.zone_matches: List[Tuple[str, str]] = []
        self.biome_match_items: List[Tuple[str, List[str]]] = []
        self.max_events = 200
        self.has_colors = False
        self.color_pairs: Dict[str, int] = {}
        self.dim_attr = curses.A_DIM
        self.error_attr = curses.A_BOLD
        self._init_curses()

    def _init_curses(self):
        curses.curs_set(0)
        self.screen.nodelay(True)
        if curses.has_colors():
            self.has_colors = True
            curses.start_color()
            try:
                curses.use_default_colors()
            except curses.error:
                pass
            unique_codes = sorted(set(BIOME_COLOR.values()) | {DIM, "33"})
            pair_id = 1
            for code in unique_codes:
                fg = ANSI_TO_CURSES.get(code, curses.COLOR_WHITE)
                try:
                    curses.init_pair(pair_id, fg, -1)
                except curses.error:
                    curses.init_pair(pair_id, fg, 0)
                self.color_pairs[code] = curses.color_pair(pair_id)
                pair_id += 1
            self.dim_attr = self.color_pairs.get(DIM, curses.A_DIM)
            self.error_attr = self.color_pairs.get("33", curses.A_BOLD)

    def run(self):
        try:
            while True:
                self._consume_log()
                self._update_status()
                self._draw()
                ch = self.screen.getch()
                if ch != -1 and not self._handle_input(ch):
                    break
                time.sleep(0.02)
        except KeyboardInterrupt:
            pass

    def _consume_log(self):
        for line in self.follower.poll():
            zone = parse_zone(line)
            if not zone:
                continue
            if zone == self.last_zone:
                continue
            self.last_zone = zone
            biome = resolve_biome(zone)
            event = {
                "time": datetime.now(),
                "zone": zone,
                "biome": biome,
                "notes": biome_notes(biome),
            }
            self.events.append(event)
            if len(self.events) > self.max_events:
                self.events.pop(0)

    def _update_status(self):
        if self.follower.last_error:
            self.status_message = self.follower.last_error
        else:
            self.status_message = f"watching {LOG_PATH}"

    def _handle_input(self, ch: int) -> bool:
        if ch in (ord("q"), ord("Q")):
            return False
        if ch == curses.KEY_RESIZE:
            return True
        if ch in (curses.KEY_BACKSPACE, 127, 8):
            self._backspace_search()
        elif ch == 27:  # ESC
            self._clear_search()
        elif ch in (10, 13):
            pass
        elif 0 <= ch <= 255:
            char = chr(ch)
            if char.isprintable():
                self._append_search_char(char)
        return True

    def _append_search_char(self, char: str):
        self._update_search(self.search_query + char)

    def _backspace_search(self):
        if not self.search_query:
            return
        self._update_search(self.search_query[:-1])

    def _clear_search(self):
        self.search_query = ""
        self.zone_matches = []
        self.biome_match_items = []

    def _update_search(self, query: str):
        self.search_query = query.strip()
        if not self.search_query:
            self._clear_search()
            return
        zone_matches, biome_matches = search_biomes(self.search_query)
        self.zone_matches = zone_matches
        self.biome_match_items = sorted(biome_matches.items())

    def _draw(self):
        self.screen.erase()
        height, width = self.screen.getmaxyx()

        current_height = min(6, max(3, height // 5)) if height >= 5 else max(1, height - 8)
        current_height = max(3, current_height)
        if current_height >= height:
            current_height = max(3, height - 6)
        current_height = max(3, min(current_height, height))

        search_height = max(0, height - current_height)

        self._draw_current_zone(0, current_height, width)
        if search_height > 0:
            self._draw_search(current_height, search_height, width)

        self.screen.refresh()

    def _draw_current_zone(self, start: int, height: int, width: int):
        if height <= 0:
            return
        for idx in range(height):
            y = start + idx
            self.screen.move(y, 0)
            self.screen.clrtoeol()

        border = "=" * max(1, width)
        self._addstr(start, 0, self._truncate(border, width), self.dim_attr)
        if height == 1:
            return

        content_top = start + 1
        content_bottom = start + height - 1
        draw_bottom_border = height >= 3
        if draw_bottom_border:
            self._addstr(content_bottom, 0, self._truncate(border, width), self.dim_attr)
            content_bottom -= 1

        available_rows = max(0, content_bottom - content_top + 1)
        if available_rows <= 0:
            return

        latest = self.events[-1] if self.events else None
        if not latest:
            message = self.follower.last_error or "Waiting for zone event…"
            self._addstr(content_top, 0, self._truncate(message, width), self.dim_attr)
            return

        row = content_top
        zone_line = f"{latest['time'].strftime('%H:%M:%S')} — {latest['zone']}"
        self._addstr(row, 0, self._truncate(zone_line, width), curses.A_BOLD)
        row += 1

        if row <= content_bottom:
            tokens = split_biomes(latest["biome"])
            if tokens:
                self._render_biome_tokens(row, 0, tokens, width)
            else:
                attr = self._attr_for_code(pick_color_for_biome(latest["biome"])) | curses.A_BOLD
                self._addstr(row, 0, self._truncate(latest["biome"], width), attr)
            row += 1

        if row <= content_bottom and latest["notes"]:
            note_text = "Drops: " + " | ".join(latest["notes"])
            self._addstr(row, 0, self._truncate(note_text, width), self.dim_attr)

    def _draw_search(self, start: int, height: int, width: int):
        if height <= 0:
            return
        self._draw_separator(start, width)
        y = start + 1
        remaining = height - 1
        query_label = f"Search > {self.search_query or '(inactive)'}"
        attr = curses.A_BOLD if self.search_query else curses.A_DIM
        self._addstr(y, 0, self._truncate(query_label, width), attr)
        y += 1
        remaining -= 1

        if remaining <= 0:
            return

        if not self.search_query:
            hint = "Start typing to search maps/biomes. ESC clears."
            self._addstr(y, 0, self._truncate(hint, width))
            return

        if not self.zone_matches and not self.biome_match_items:
            self._addstr(y, 0, self._truncate("No matches found.", width), self.error_attr)
            return

        if self.zone_matches and remaining > 0:
            self._addstr(y, 0, self._truncate("Map matches:", width), curses.A_BOLD)
            y += 1
            remaining -= 1
            for zone, biome in self.zone_matches:
                if remaining <= 0:
                    break
                self._addstr(y, 2, "- ")
                x = 4
                tokens = split_biomes(biome)
                if tokens:
                    x = self._render_biome_tokens(y, x, tokens, width)
                else:
                    attr = self._attr_for_code(pick_color_for_biome(biome))
                    label = biome
                    avail = max(width - x, 0)
                    if avail > 0:
                        text = self._truncate(label, avail)
                        self._addstr(y, x, text, attr | curses.A_BOLD)
                        x += len(text)
                detail = f" — {zone}"
                if x < width:
                    self._addstr(y, x, self._truncate(detail, width - x))
                y += 1
                remaining -= 1

        if self.biome_match_items and remaining > 0:
            self._addstr(y, 0, self._truncate("Biome matches:", width), curses.A_BOLD)
            y += 1
            remaining -= 1
            for biome, zones in self.biome_match_items:
                if remaining <= 0:
                    break
                attr = self._attr_for_code(BIOME_COLOR.get(biome, "37"))
                self._addstr(y, 2, "- ")
                self._addstr(y, 4, self._truncate(biome, max(width - 4, 0)), attr | curses.A_BOLD)
                y += 1
                remaining -= 1
                for zone in zones:
                    if remaining <= 0:
                        break
                    self._addstr(y, 6, self._truncate(f"- {zone}", max(width - 6, 0)), curses.A_DIM)
                    y += 1
                    remaining -= 1

    def _draw_separator(self, y: int, width: int):
        if y < 0 or y >= curses.LINES:
            return
        line = "-" * max(1, width)
        self._addstr(y, 0, self._truncate(line, width), curses.A_DIM)

    def _addstr(self, y: int, x: int, text: str, attr: int = 0):
        if y < 0 or x < 0:
            return
        if not text:
            return
        try:
            self.screen.addstr(y, x, text, attr)
        except curses.error:
            pass

    def _attr_for_code(self, code: str) -> int:
        if self.has_colors and code in self.color_pairs:
            return self.color_pairs[code]
        if code == DIM:
            return curses.A_DIM
        return curses.A_NORMAL

    def _truncate(self, text: str, width: int) -> str:
        if width <= 0:
            return ""
        if len(text) <= width:
            return text
        return text[: max(0, width - 1)] + "…"

    def _render_biome_tokens(self, y: int, x: int, tokens: List[str], width: int) -> int:
        for idx, token in enumerate(tokens):
            if x >= width:
                return width
            if idx:
                if x < width:
                    self._addstr(y, x, " ")
                    x += 1
            chip = f"[{token}]"
            attr = self._attr_for_code(BIOME_COLOR.get(token, "37")) | curses.A_BOLD
            avail = width - x
            if avail <= 0:
                return width
            if len(chip) > avail:
                truncated = chip[: max(0, avail - 1)] + "…"
                self._addstr(y, x, truncated, attr)
                return width
            self._addstr(y, x, chip, attr)
            x += len(chip)
        return x

def main():
    curses.wrapper(lambda scr: ZoneWatcherApp(scr).run())

if __name__ == "__main__":
    main()
