# Current State Models

This document records Vaachak-owned state models and the imported runtime boundary.

## Ownership boundary

Vaachak-owned model surfaces:

- reader preferences
- sleep image mode
- Date & Time cached status
- Wi-Fi Transfer configuration shape
- reader progress records
- bookmark records
- title-cache records
- prepared-cache metadata
- storage/path helper constants

Still owned by the active imported runtime:

- actual SD reads/writes
- actual Settings UI behavior
- actual Reader behavior
- actual Wi-Fi server/upload handlers
- actual display drawing/refresh
- actual input scan/debounce

## Compatibility paths

Current runtime compatibility paths include:

```text
/_X4/SETTINGS.TXT
SLPMODE.TXT
TIME.TXT
/FCACHE/<BOOKID>
state/<BOOKID>.PRG
state/<BOOKID>.BKM
state/BMIDX.TXT
/_X4/TITLES.BIN
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
cargo build
./scripts/validate_documentation_refresh.sh
```
