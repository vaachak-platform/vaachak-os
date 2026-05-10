# X4 New Device Deployment Runbook

This runbook prepares a fresh SD card and flashes a known-good Vaachak OS build to a new X4.

## 0. Build prerequisites

```bash
. "$HOME/export-esp.sh"

rustup target add riscv32imc-unknown-none-elf
cargo --version
espflash --version
```

## 1. Prepare SD card

Set the mount path:

```bash
export SD=/media/mindseye73/SD_CARD
```

Copy reader files to the SD root:

```bash
find "$SD" -maxdepth 1 -type f \( -iname '*.txt' -o -iname '*.epub' -o -iname '*.epu' -o -iname '*.md' \) -printf '%f\n' | sort
```

Prepare the title cache:

```bash
SD="$SD" ./tools/x4-title-cache/prepare_sd_title_cache.sh
```

Inspect the result:

```bash
SD="$SD" ./tools/x4-title-cache/inspect_title_cache.sh
```

Expected:
- `_X4/TITLEMAP.TSV` exists.
- `_X4/TITLES.BIN` exists.
- TXT/MD aliases are seeded into `_X4/TITLES.BIN`.
- No Project Gutenberg/body/license phrases are cached as titles.

## 2. Validate build

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo test -p vaachak-core --all-targets
cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

## 3. Flash

List ports:

```bash
espflash board-info --list-all-ports
```

Flash on macOS example:

```bash
espflash flash \
  --monitor \
  --chip esp32c3 \
  --port /dev/cu.usbmodem3101 \
  target/riscv32imc-unknown-none-elf/release/target-xteink-x4
```

## 4. Device smoke test

Confirm:
- Home page opens and title is not clipped.
- Files/Library opens.
- EPUB/EPU titles display correctly.
- TXT display names come from `_X4/TITLES.BIN`, not body text.
- Reader opens and Back returns to Files/Library.
- Reader restore still works.
- Footer labels remain accepted.
- No crash/reboot.

## X4-compatible partition table note

This repo includes `espflash.toml` and `partitions/xteink_x4_standard.bin` so the
X4 is flashed with the CrossPoint-compatible dual-OTA partition table used by
current X4 firmware tools. Validate it before flashing:

```bash
./scripts/validate_x4_standard_partition_table_compatibility.sh
```

If this X4 previously ran a Vaachak build with an incompatible Vaachak partition table,
do one full erase-and-flash migration first:

```bash
./scripts/flash_x4_standard_partition_table.sh /dev/cu.usbmodemXXXX
```
