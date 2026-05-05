#!/usr/bin/env python3
"""Generate daily mantra sleep BMP assets for Vaachak X4.

The preferred path uses Pillow and optional font files. If Pillow is not
available, the script creates valid 1-bit placeholder BMP files so the SD
layout and device path checks can still be verified.
"""
from __future__ import annotations

import argparse
import re
import struct
from dataclasses import dataclass
from pathlib import Path

WIDTH = 800
HEIGHT = 480
DAYS = [
    ("MONDAY", "mon", "Somvar"),
    ("TUESDAY", "tue", "Mangalvar"),
    ("WEDNESDAY", "wed", "Budhvar"),
    ("THURSDAY", "thu", "Guruvar"),
    ("FRIDAY", "fri", "Shukravar"),
    ("SATURDAY", "sat", "Shanivar"),
    ("SUNDAY", "sun", "Ravivar"),
]

@dataclass(frozen=True)
class MantraEntry:
    day: str
    key: str
    title: str
    dedication: str
    sanskrit: str
    hindi: str
    english: str


def parse_entries(path: Path) -> dict[str, MantraEntry]:
    text = path.read_text(encoding="utf-8")
    entries: dict[str, MantraEntry] = {}
    pattern = re.compile(r"^(MONDAY|TUESDAY|WEDNESDAY|THURSDAY|FRIDAY|SATURDAY|SUNDAY).*?-\s*(.*)$", re.MULTILINE)
    matches = list(pattern.finditer(text))
    for idx, match in enumerate(matches):
        day = match.group(1)
        dedication = match.group(2).strip()
        start = match.end()
        end = matches[idx + 1].start() if idx + 1 < len(matches) else len(text)
        block = text[start:end]
        fields = {}
        for label in ("Sanskrit", "Hindi", "English"):
            line_match = re.search(rf"^{label}:\s*(.*)$", block, re.MULTILINE)
            fields[label.lower()] = line_match.group(1).strip() if line_match else ""
        key = next(short for full, short, _ in DAYS if full == day)
        title = next(display for full, _, display in DAYS if full == day)
        entries[key] = MantraEntry(
            day=day.title(),
            key=key,
            title=title,
            dedication=dedication,
            sanskrit=fields["sanskrit"],
            hindi=fields["hindi"],
            english=fields["english"],
        )
    return entries


def write_manifest(output_dir: Path, entries: dict[str, MantraEntry]) -> None:
    lines = ["key\tfile\tday\ttitle\tdedication"]
    for _, key, _ in DAYS:
        entry = entries[key]
        lines.append(f"{key}\t{key}.bmp\t{entry.day}\t{entry.title}\t{entry.dedication}")
    lines.append("default\tdefault.bmp\tDefault\tDaily Mantra\tFallback image")
    (output_dir / "manifest.tsv").write_text("\n".join(lines) + "\n", encoding="utf-8")


def try_pillow_render(output_dir: Path, entries: dict[str, MantraEntry], args: argparse.Namespace) -> bool:
    try:
        from PIL import Image, ImageDraw, ImageFont
    except Exception:
        return False

    def load_font(path: str | None, size: int):
        if path:
            return ImageFont.truetype(path, size=size)
        try:
            return ImageFont.truetype("Arial Unicode.ttf", size=size)
        except Exception:
            return ImageFont.load_default()

    devanagari_large = load_font(args.devanagari_font, 52)
    devanagari_medium = load_font(args.devanagari_font, 30)
    latin_title = load_font(args.latin_font, 30)
    latin_body = load_font(args.latin_font, 24)

    for _, key, _ in DAYS:
        entry = entries[key]
        img = Image.new("1", (WIDTH, HEIGHT), 1)
        draw = ImageDraw.Draw(img)
        draw.rectangle((12, 12, WIDTH - 13, HEIGHT - 13), outline=0)
        draw.text((36, 30), f"{entry.day} / {entry.title}", font=latin_title, fill=0)
        draw.text((36, 72), entry.dedication, font=latin_body, fill=0)
        draw.line((36, 112, WIDTH - 36, 112), fill=0)
        draw.text((36, 146), entry.sanskrit, font=devanagari_large, fill=0)
        draw.text((36, 230), entry.hindi, font=devanagari_medium, fill=0)
        draw.text((36, 330), entry.english, font=latin_body, fill=0)
        draw.text((36, 420), "Vaachak Daily Mantra", font=latin_body, fill=0)
        img.save(output_dir / f"{key}.bmp", format="BMP")

    default = Image.new("1", (WIDTH, HEIGHT), 1)
    draw = ImageDraw.Draw(default)
    draw.rectangle((12, 12, WIDTH - 13, HEIGHT - 13), outline=0)
    draw.text((36, 40), "Daily Mantra", font=latin_title, fill=0)
    draw.text((36, 94), "Date is not set. Showing fallback sleep image.", font=latin_body, fill=0)
    default.save(output_dir / "default.bmp", format="BMP")
    return True


# Tiny fallback renderer: creates valid 1-bit BMP with border and simple bars.
def write_plain_bmp(path: Path, bar_index: int) -> None:
    row_bytes = ((WIDTH + 31) // 32) * 4
    pixel_bytes = row_bytes * HEIGHT
    header_size = 14 + 40 + 8
    file_size = header_size + pixel_bytes
    data = bytearray(pixel_bytes)

    def set_black(x: int, y: int) -> None:
        if not (0 <= x < WIDTH and 0 <= y < HEIGHT):
            return
        storage_y = HEIGHT - 1 - y
        offset = storage_y * row_bytes + (x // 8)
        bit = 7 - (x % 8)
        data[offset] |= 1 << bit

    for x in range(12, WIDTH - 12):
        set_black(x, 12)
        set_black(x, HEIGHT - 13)
    for y in range(12, HEIGHT - 12):
        set_black(12, y)
        set_black(WIDTH - 13, y)
    for y in range(60, 70):
        for x in range(40, WIDTH - 40):
            if (x + bar_index * 17) % 11 < 6:
                set_black(x, y)
    for row in range(7):
        y = 140 + row * 32
        length = 180 + ((bar_index + row) % 7) * 64
        for yy in range(y, y + 10):
            for x in range(60, min(WIDTH - 60, 60 + length)):
                set_black(x, yy)

    with path.open("wb") as f:
        f.write(b"BM")
        f.write(struct.pack("<IHHI", file_size, 0, 0, header_size))
        f.write(struct.pack("<IIIHHIIIIII", 40, WIDTH, HEIGHT, 1, 1, 0, pixel_bytes, 2835, 2835, 2, 0))
        f.write(bytes([255, 255, 255, 0, 0, 0, 0, 0]))
        f.write(data)


def write_placeholder_set(output_dir: Path) -> None:
    for idx, (_, key, _) in enumerate(DAYS):
        write_plain_bmp(output_dir / f"{key}.bmp", idx)
    write_plain_bmp(output_dir / "default.bmp", 8)


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate Vaachak X4 daily mantra sleep BMP assets")
    parser.add_argument("--source", default="assets/text/daily_hindu_mantras.txt")
    parser.add_argument("--output", required=True)
    parser.add_argument("--devanagari-font", default=None)
    parser.add_argument("--latin-font", default=None)
    parser.add_argument("--gujarati-font", default=None)
    args = parser.parse_args()

    source = Path(args.source)
    output_dir = Path(args.output)
    output_dir.mkdir(parents=True, exist_ok=True)
    entries = parse_entries(source)
    missing = [key for _, key, _ in DAYS if key not in entries]
    if missing:
        raise SystemExit(f"missing entries for: {', '.join(missing)}")

    rendered = try_pillow_render(output_dir, entries, args)
    if not rendered:
        write_placeholder_set(output_dir)
        print("Pillow not available; generated valid placeholder BMP files.")
        print("Install Pillow and pass font paths for rendered Devanagari/Latin text.")

    write_manifest(output_dir, entries)
    print(f"Wrote daily mantra sleep assets to {output_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
