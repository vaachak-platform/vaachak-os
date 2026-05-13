# Current State Models

This document records current Vaachak-owned state models and compatibility paths.

## Vaachak-owned model surfaces

- reader preferences
- sleep image mode
- Date & Time cached/live/unsynced state
- Wi-Fi Transfer configuration shape
- reader progress records
- bookmark records
- title-cache records
- prepared-cache metadata
- storage/path helper constants
- SD font catalog/selection state
- reader viewport state

## Compatibility paths

Current runtime compatibility paths include:

```text
/_X4/SETTINGS.TXT
/SLPMODE.TXT
/TIME.TXT
/FCACHE/<BOOKID>
state/<BOOKID>.PRG
state/<BOOKID>.BKM
state/BMIDX.TXT
/_X4/TITLES.BIN
/VAACHAK/APPS
/VAACHAK/FONTS
```

## Reader preferences

The model mirrors current runtime keys:

```text
book_font=<0..4>
reading_theme=<0..3>
show_progress=<0|1>
prepared_font_profile=<0..2>
prepared_fallback_policy=<0..2>
```

## Date & Time state

Status behavior:

- Live: same-boot cached sync can advance from uptime.
- Cached: previous sync exists but live same-boot continuity is not trusted.
- Unsynced: no cached sync exists.

## Validation

Use repository-level validation:

```bash
cargo fmt --all
./scripts/check_repo_hygiene.sh
cargo test -p vaachak-core --all-targets
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```
