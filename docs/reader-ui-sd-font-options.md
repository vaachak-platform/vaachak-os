# Reader/UI SD Font Options Pack

This deliverable adds local tooling to build Vaachak SD font options from font ZIP files that already exist on the developer machine.

It does not bundle any `.ttf`, `.otf`, `.woff`, `.woff2`, `.VFN`, or `.VFN` files.

## Reader mapping

The generated `/VAACHAK/FONTS/MANIFEST.TXT` is ordered for the current Reader `Font Source` menu:

```text
Built-in -> firmware built-in font
SD 1     -> Charis
SD 2     -> Bitter
SD 3     -> Lexend Deca
```

## UI staging

The generated `/VAACHAK/FONTS/UIFONTS.TXT` stages UI font options:

```text
UI 1 -> Inter UI
UI 2 -> Lexend Deca UI
```

The UI runtime still uses the built-in bitmap UI fonts until the UI font renderer selection slice is added. This pack prepares the SD-side assets and manifest contract for that follow-up.

## Build the SD font pack

From repo root:

```bash
python3 tools/build_x4_font_options_from_zips.py \
  --charis-zip ~/Downloads/Charis-7.000.zip \
  --families-zip ~/Downloads/Bitter,Inter,Lexend_Deca.zip \
  --out examples/sd-card/VAACHAK/FONTS \
  --clean
```

Validate reader font manifest:

```bash
python3 tools/validate_sd_font_pack.py examples/sd-card/VAACHAK/FONTS
```

Copy to actual SD card:

```bash
REAL_SD=/Volumes/<YOUR_SD_CARD_NAME>
rm -rf "$REAL_SD/VAACHAK/FONTS"
mkdir -p "$REAL_SD/VAACHAK"
cp -R examples/sd-card/VAACHAK/FONTS "$REAL_SD/VAACHAK/"
diskutil eject "$REAL_SD"
```

## Expected generated files

```text
/VAACHAK/FONTS/MANIFEST.TXT
/VAACHAK/FONTS/UIFONTS.TXT
/VAACHAK/FONTS/CHARIS18.VFN
/VAACHAK/FONTS/BITTER18.VFN
/VAACHAK/FONTS/LEXEND18.VFN
/VAACHAK/FONTS/INTER14.VFN
/VAACHAK/FONTS/LEXUI14.VFN
/VAACHAK/FONTS/README.TXT
```

## Device smoke test

```text
Reader -> Menu -> Font Source -> Built-in
Reader -> Menu -> Font Source -> SD 1
Reader -> Menu -> Font Source -> SD 2
Reader -> Menu -> Font Source -> SD 3
```

Expected marker when a valid SD font option is selected:

```text
reader-fonts=x4-reader-sd-font-selection-ok
```
