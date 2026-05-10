# Wi-Fi connection restore + settings UI repair

This repair replaces the earlier Wi-Fi profile/browser-editor direction.

## Source of truth

Wi-Fi credentials are stored only in the existing Vaachak/Pulp settings file:

```text
/_x4/SETTINGS.TXT
wifi_ssid=...
wifi_pass=...
```

There is no `/_x4/WIFI.TXT` profile file and no `/wifi`, `/wifi.txt`, or `/wifi/save` browser route.

## Reliability repair

The prior Wi-Fi Transfer change ran NTP before the upload server started. On networks where DNS/NTP is slow or blocked, that makes the device appear to have failed Wi-Fi connection even after DHCP succeeds.

This repair restores the connection-first behavior:

1. Join Wi-Fi using `wifi_ssid` / `wifi_pass` from `SETTINGS.TXT`.
2. Wait for DHCP.
3. Immediately show `http://x4.local/` and the IP address.
4. Start serving upload/download requests.

Date/time sync remains available in `System > Date & Time`. A future version can add non-blocking background time sync after the server is already available.

## On-device Wi-Fi setup UI

`Network > Wi-Fi Networks` now behaves as a first-install setup screen:

- Row mode:
  - `SSID`
  - `Password`
  - `Save to SETTINGS.TXT`
- Select on SSID or Password enters a focused text editor.
- Select on Save writes the current fields back to `/_x4/SETTINGS.TXT`.
- Password characters are never displayed; only length is shown.

## Controls

Row mode:

```text
Next / Prev       move rows
Left / Right      move rows
Select            edit selected field, or save
Back              return to Network category
```

Text edit mode:

```text
Next / Prev       change character
Left / Right      jump through characters faster
Select            add shown character
Long Select       delete one character
Long Left/Right   clear active field
Back              return to row mode, then Save
```

## Validation marker

```text
wifi-connection-restore-settings-ui-ok
```
