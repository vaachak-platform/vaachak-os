# Ownership Map

## Active runtime ownership

| Area | Current owner | Notes |
| --- | --- | --- |
| Boot and scheduler | `vendor/pulp-os` | Active flashed runtime. |
| Display refresh | `vendor/pulp-os` | SSD1677 behavior stays in the proven path. |
| Button/input hardware | `vendor/pulp-os` | Ladder thresholds and event flow stay stable. |
| SD/FAT I/O | `vendor/pulp-os` | Physical storage behavior stays stable. |
| Home dashboard | `vendor/pulp-os` | Category dashboard is active here. |
| Reader | `vendor/pulp-os` | TXT, EPUB smoke, progress, bookmarks, prepared cache. |
| Wi-Fi Transfer | `vendor/pulp-os` | Original and chunked-resume tabs stay active. |
| Date & Time | `vendor/pulp-os` | Explicit isolated Wi-Fi sync mode. |
| Settings | `vendor/pulp-os` | Persists to `/_X4/SETTINGS.TXT`. |

## Vaachak-owned architecture

| Area | Current owner | Notes |
| --- | --- | --- |
| Reader/domain models | `core/` | Shared target-neutral vocabulary. |
| HAL seams | `hal-xteink-x4/` | X4 contracts and smoke helpers. |
| Adapter contracts | `target-xteink-x4/` | Future adoption surface. |
| Prepared cache tools | `tools/` | Host-side cache creation and inspection. |
| Title cache tools | `tools/x4-title-cache/` | Host-side title-map generation. |

## Rule for future work

Move pure deterministic logic first. Keep physical hardware behavior in the active runtime until the replacement path can be validated on the X4 without changing reader behavior.
