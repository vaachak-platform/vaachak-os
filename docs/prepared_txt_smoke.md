# Prepared TXT Smoke

This is a TXT-only visibility path for one mixed English and Devanagari book.
The X4 does not shape Indic text on device. Instead, a host tool prepares a
small cache of bitmap fonts and positioned glyph records, then the Reader uses
that cache when it opens the matching TXT file.

## Cache Layout

The Reader looks for this 8.3-friendly layout on the SD card:

```text
/FCACHE/<BOOKID>/
  META.TXT
  FONTS.IDX
  LAT18.VFN
  DEV22.VFN
  PAGES.IDX
  P000.VRN
```

`<BOOKID>` is the same path fingerprint policy used by the existing Reader
state files, without the `bk-` prefix and uppercased to eight hex characters.

`META.TXT` contains line-based metadata:

```text
book_id=<8HEX>
source=/<device TXT path>
title=Prepared TXT Smoke
page_count=1
latin_font=LAT18.VFN
devanagari_font=DEV22.VFN
pages=PAGES.IDX
```

`FONTS.IDX` maps scripts to VFNT files. `PAGES.IDX` lists one VRN page file per
line. The smoke renderer supports Latin and Devanagari font slots only.

## Synthetic Generator

The host generator lives at:

```bash
tools/prepared_txt_smoke/generate_prepared_txt_smoke.py
```

Example:

```bash
python3 tools/prepared_txt_smoke/generate_prepared_txt_smoke.py \
  --book tools/prepared_txt_smoke/MIXED.TXT \
  --device-path MIXED.TXT \
  --out /Volumes/X4SD/FCACHE
```

The `--device-path` value must match the path string the X4 Reader uses for the
TXT file. If the book is inside a folder on SD, pass that relative path, such
as `BOOKS/MIXED.TXT`.

The generator creates tiny synthetic VFNT assets and one VRN page. It does not
use Noto fonts, HarfBuzz, rustybuzz, network access, or generated binary assets
committed to the repository.

## Real VFNT Generator

The real-font host generator lives at:

```bash
tools/prepared_txt_real_vfnt
```

It is a separate Cargo project with its own workspace boundary, so it is not
compiled into the ESP32 firmware workspace. It uses `rustybuzz` for host-side
shaping and `fontdue` for host-side glyph rasterization. The firmware still
loads only prepared VFNT/VRN byte files and does not shape or rasterize fonts on
device.

Required local font files:

```text
NotoSans-Regular.ttf
NotoSansDevanagari-Regular.ttf
```

Do not commit these font files or generated cache output. Place the font files
somewhere on the host machine and pass their paths to the tool.

Example:

```bash
cd tools/prepared_txt_real_vfnt
cargo run --release -- \
  --book ../prepared_txt_smoke/MIXED.TXT \
  --device-path MIXED.TXT \
  --latin-font /path/to/NotoSans-Regular.ttf \
  --devanagari-font /path/to/NotoSansDevanagari-Regular.ttf \
  --out /Volumes/X4SD/FCACHE
```

Optional layout controls:

```text
--title <title>
--latin-size <px>
--devanagari-size <px>
--line-height <px>
--page-width <px>
--page-height <px>
--margin-x <px>
--margin-y <px>
```

The real generator writes the same active Reader cache layout:

```text
/FCACHE/<BOOKID>/
  META.TXT
  FONTS.IDX
  LAT18.VFN
  DEV22.VFN
  PAGES.IDX
  P000.VRN
```

The generated VFNT files contain only glyph IDs used by the prepared TXT pages.
The generated VRN files reference those VFNT glyph IDs and font slots:

```text
Latin -> LAT18.VFN
Devanagari -> DEV22.VFN
```

`rustybuzz` shapes Devanagari runs before VRN output, so matras and conjuncts
are represented as positioned glyph IDs rather than raw Unicode code points.

## Reader Detection

When the active Reader opens a TXT file, it computes the existing Reader book
id for the TXT path and checks `/FCACHE/<BOOKID>/META.TXT`. If the metadata,
font index, page index, VFNT assets, and VRN page validate, the Reader draws the
prepared glyph page. If the cache is missing or invalid, it clears the prepared
state and falls back to the existing TXT Reader path.

The prepared bridge is local to the active Reader because the target-owned text
foundation cannot be imported into `pulp-os` without reversing the existing
crate dependency direction.

## Not Included

This smoke path intentionally does not add:

- EPUB prepared rendering.
- On-device Indic shaping.
- Arbitrary TTF loading.
- General SD font discovery.
- Reader-wide custom font settings.
- Arbitrary TXT Indic rendering without a prepared cache.
- Full paragraph layout or EPUB/CSS layout.
- Daily Mantra, Sleep Screen, Home, Settings, or e-paper display changes.
