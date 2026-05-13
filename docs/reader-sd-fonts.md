# Reader SD Fonts Contract

This slice adds the SD font pack contract and validators. It does not switch the active reader renderer to SD fonts yet.

Canonical SD layout:

```text
/VAACHAK/FONTS/MANIFEST.TXT
/VAACHAK/FONTS/<FONT>.VFN
```

Manifest row format:

```text
FONT|FONT_ID|Display Name|Script|Style|PixelSize|FILE.VFN|SizeBytes|CRC32HEX|GlyphCount
```

Rules:

- `FONT_ID` must be 8.3-safe: `A-Z`, `0-9`, `_`, max 8 chars.
- `FILE.VFN` must be 8.3-safe and end with `.VFN`.
- Maximum font file size is 512 KiB.
- Maximum glyph count is 2048.
- Supported scripts: `Latin`, `Devanagari`, `Gujarati`, `Symbols`.
- Supported styles: `Regular`, `Bold`, `Italic`, `BoldItalic`.

Validation:

```bash
python3 tools/validate_sd_font_pack.py examples/sd-card/VAACHAK/FONTS --manifest-only --allow-empty
python3 tools/validate_sd_font_pack.py /Volumes/<SD>/VAACHAK/FONTS
```

Manifest generation for existing `.VFN` files:

```bash
python3 tools/build_sd_font_manifest.py /Volumes/<SD>/VAACHAK/FONTS --script Latin --style Regular --pixel-size 14
```

No font binaries are included in this deliverable.
