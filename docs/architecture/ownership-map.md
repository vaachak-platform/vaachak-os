# Ownership Map

## Active code ownership

| Area | Current owner/path | Rule |
| --- | --- | --- |
| Shared models/contracts | `core/**` | Keep platform-neutral. |
| X4 HAL seams | `hal-xteink-x4/**` | Keep board-facing and testable. |
| X4 firmware target | `target-xteink-x4/**` | Active X4 runtime and app path. |
| Optional Lua VM support | `support/vaachak-lua-vm/**` | Feature-gated support crate. |
| SD sample assets/apps | `examples/sd-card/**` | Canonical sample deployment tree. |
| Host asset tooling | `tools/**` | Host-only generation/smoke tools. |
| Production helper scripts | `scripts/**` | Keep only reusable production helpers. |
| Documentation | `docs/**` | Current-state docs plus retained design references. |
| Pulp reference | `vendor/pulp-os/**` | Reference/compatibility only; do not add new functionality. |
| EPUB dependency | `vendor/smol-epub/**` | Dependency source, excluded from workspace. |

## Product ownership

| Area | Current state |
| --- | --- |
| Home/category dashboard | Active: Network, Productivity, Games, Reader, System, Tools. |
| Files | Active local SD file browser. |
| Reader | Active TXT/EPUB reader path with progress/bookmark/settings support. |
| Reader display aids | Bionic Reading, Guide Dots, and sunlight-fading mitigation accepted. |
| Wi-Fi | Vaachak-owned setup/scan/transfer/time code under the X4 target. |
| Lua apps | Optional `/VAACHAK/APPS` path, uppercase 8.3-safe physical folders. |
| Sleep images | Daily/Fast Daily/Static/Cached/Text/No Redraw helper path. |
| XTC | Planned compatibility/open path. |
| `.vchk` | Planned Vaachak-native package contract. |
| Sync | Planned after local reader state is stable. |
| Waveshare/S3 | Later capability profile after X4 reader path is stable. |

## New work rule

New code should go into Vaachak-owned paths. Do not add new behavior to `vendor/pulp-os`.
