# X4 Shared Keyboard + Wi-Fi Editor

This update replaces the Wi-Fi one-character picker with a reusable X4 on-device text keyboard.

## Scope

- Adds `vendor/pulp-os/src/apps/widgets/text_keyboard.rs`.
- Wires the keyboard into `Network -> Wi-Fi Networks` for SSID and password editing.
- Keeps credentials in `_x4/SETTINGS.TXT` only.
- Keeps multiple Wi-Fi profiles in `_x4/SETTINGS.TXT`:
  - `wifi_profile_0_*` = Home
  - `wifi_profile_1_*` = Work
  - `wifi_profile_2_*` = Other
  - `wifi_default` selects the profile used by Wi-Fi Transfer.
- Does not add `/_x4/WIFI.TXT`.
- Does not restore browser `/wifi` profile routes.
- Does not change NTP/date-time sync behavior that is already working.

## Keyboard controls

In `Network -> Wi-Fi Networks`, select `SSID` or `Password` to open the keyboard.

- Volume Up / Volume Down: move keyboard row
- Left / Right: move within the current row
- OK / Select: press selected key
- Hold OK / Select: delete one character
- `space`: insert a space
- `del`: delete one character
- `clear`: clear the active field
- `ABC` / `abc`: toggle upper/lowercase keyboard
- `123`: toggle symbols/numbers keyboard
- `done` or Back: return to the Wi-Fi profile list
- Save: writes `_x4/SETTINGS.TXT`

## Reuse by other apps

Future apps can reuse `text_keyboard.rs` by storing a layout byte and index byte in their app state, then using:

- `text_keyboard::draw(...)`
- `text_keyboard::move_horizontal(...)`
- `text_keyboard::move_vertical(...)`
- `text_keyboard::activate(...)`
- `TextKeyboardAction`

This keeps Search, Notes, Dictionary, account setup, and other text-entry screens consistent with Wi-Fi setup.
