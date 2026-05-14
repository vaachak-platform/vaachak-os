# Vaachak OS User Guide

## Build locally

Install the Rust toolchain from `rust-toolchain.toml`, then run:

```bash
cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

## Flash over USB

Normal development flashing:

```bash
scripts/flash_x4_vaachak_app0.sh /dev/cu.usbmodemXXXX
```

Recovery or first-time partition-table migration:

```bash
scripts/flash_x4_standard_partition_table.sh /dev/cu.usbmodemXXXX
```

Erase only OTA selector data to return boot selection to `app0`:

```bash
scripts/erase_x4_otadata_select_app0.sh /dev/cu.usbmodemXXXX
```

## Build a GitHub-style install artifact locally

```bash
cargo install espflash --locked
scripts/build_x4_firmware_artifacts.sh dist/x4
```

Install the generated full image on a new X4:

```bash
espflash write-bin --chip esp32c3 --port /dev/cu.usbmodemXXXX 0 dist/x4/vaachak-os-x4-full.bin
```

## Prepare an SD card

Start with the example layout:

```bash
rsync -a examples/sd-card/ /Volumes/X4SD/
```

Optional Lua apps are under:

```text
/Volumes/X4SD/VAACHAK/APPS
```

Recommended app folders:

```text
CALENDAR
DICT
MANTRA
PANCHANG
```

## Dictionary data

Build the dictionary SD pack from a source `dictionary.json`:

```bash
python3 tools/build_dictionary_sd_pack.py dictionary.json /Volumes/X4SD/VAACHAK/APPS/DICT
python3 tools/check_dictionary_sd_layout.py /Volumes/X4SD/VAACHAK/APPS/DICT
```

The firmware uses `INDEX.TXT` to resolve prefix shards from `DATA/*.JSN`.

## Calendar events

The combined Calendar screen uses a native month grid plus event files from:

```text
/VAACHAK/APPS/CALENDAR/EVENTS.TXT
/VAACHAK/APPS/CALENDAR/US2026.TXT
/VAACHAK/APPS/CALENDAR/HINDU26.TXT
```

Copy or edit the sample files under `examples/sd-card/VAACHAK/APPS/CALENDAR` when present.

## Daily Mantra and Panchang

Daily Mantra and Panchang read SD app data and use the device Date & Time cache where relevant. Missing or invalid data files remain visible as user-facing errors.

Daily mantra sleep-image assets can be prepared with:

```bash
scripts/prepare_daily_mantra_sd_assets.sh /Volumes/X4SD
scripts/activate_daily_mantra_sleep_image.sh /Volumes/X4SD
```

Sleep image mode helpers:

```bash
scripts/write_sleep_image_mode.sh /Volumes/X4SD daily
scripts/verify_sleep_image_mode.sh /Volumes/X4SD
scripts/write_sleep_image_cache_hint.sh /Volumes/X4SD
scripts/clear_sleep_image_cache_hint.sh /Volumes/X4SD
```

## On-device UI

Home remains a category dashboard. Internal screens use CrossInk-style chrome.

Reader tabs:

```text
Recent | Books | Files | Bookmarks
```

Network tabs:

```text
Wi-Fi | Transfer | Time | Status
```

Settings tabs:

```text
Display | Reader | Controls | System
```

Footer labels are screen-specific and reserve safe space so reader text and game boards do not overlap the button-hint area.
