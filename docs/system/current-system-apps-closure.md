# Current System Apps Closure

This document records the expected behavior of system apps in the active X4 runtime.

## Wi-Fi Transfer

Expected behavior:

- normal transfer remains available for ordinary files.
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

## Manual validation

After flashing:

1. Open Wi-Fi Transfer and complete a normal upload where possible.
2. Complete or resume a large prepared-cache upload where possible.
3. Open Date & Time and exercise retry/back paths.
4. Change Settings, reboot, and confirm persistence.
5. Open Reader and verify settings still apply.
