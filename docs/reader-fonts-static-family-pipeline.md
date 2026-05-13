# Reader/UI static font family pipeline

Vaachak now follows a CrossInk-style firmware-static model for custom fonts:
font files are rasterized by `target-xteink-x4/build.rs` into flash-resident
bitmap font assets at build time. Reader/UI draw paths do not read font files
from SD.

Supported family selections:

- Reader Font Source: Bookerly, Charis, Bitter, Lexend
- UI Font: Built-in, Inter, Lexend

Install local source fonts before building:

```bash
python3 tools/install_static_font_families_from_zips.py \
  --charis-zip ~/Downloads/Charis-7.000.zip \
  --families-zip ~/Downloads/Bitter,Inter,Lexend_Deca.zip \
  --clean

cargo clean -p target-xteink-x4
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf --features lua-vm
```

If a family is missing, build.rs emits zero-width stubs and runtime falls back
to Bookerly/Built-in fonts.

SD `/VAACHAK/FONTS/*.VFN` files are intentionally not used by the Reader/UI
runtime because they caused FAT stack pressure and EPUB reflow instability on
ESP32-C3.
