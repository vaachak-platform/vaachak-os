# Current System Apps Closure

This document records expected behavior of system apps in the active X4 runtime.

## Category dashboard

Expected behavior:

- Home exposes Network, Productivity, Games, Reader, System, and Tools.
- Section titles remain visually distinct.
- Back navigation returns to the appropriate category or Home without requiring reboot.

## Wi-Fi Transfer

Expected behavior:

- normal transfer remains available for ordinary files.
- nested directory upload supports Lua app deployment under `/VAACHAK/APPS`.
- large prepared-cache folder transfer remains possible through the current runtime flow.
- browser/device UI must not expose saved Wi-Fi passwords.
- failed transfers should leave visible retry guidance.

## Date & Time

Expected behavior:

- Back can cancel or leave the network sync path without locking input.
- Date & Time reports Live, Cached, or Unsynced.
- failed retry preserves previously cached time.
- Select/retry can be used without reboot.

## Settings

Expected behavior:

- Reader-facing settings apply to Reader.
- sleep image mode persists.
- battery/header display remains consistent.
- settings persistence remains compatible with `/_X4/SETTINGS.TXT`.

## Reader

Expected behavior:

- TXT and EPUB open from Files.
- progress, bookmarks, title cache, and prepared cache metadata remain compatible with existing state paths.
- Bionic Reading, Guide Dots, and sunlight-fading mitigation remain available where configured.

## Lua apps

Expected behavior:

- Optional apps load from `/VAACHAK/APPS`.
- Physical app folders remain uppercase 8.3-safe.
- Calendar, Panchang, Daily Mantra, Dictionary, Unit Converter, and Games sample apps remain examples rather than replacements for native features.

## Manual validation

After flashing:

1. Open Wi-Fi Transfer and complete a normal upload where possible.
2. Complete or resume a large prepared-cache upload where possible.
3. Open Date & Time and exercise retry/back paths.
4. Change Settings, reboot, and confirm persistence.
5. Open Reader and verify settings still apply.
6. Open at least one Lua app from each populated category where sample files are present.
