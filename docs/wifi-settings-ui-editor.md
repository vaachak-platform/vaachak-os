# Wi-Fi Settings UI Editor

This overlay removes the separate `WIFI.TXT` profile/editor approach and keeps Wi-Fi credentials in the existing Vaachak/Pulp settings contract:

```text
/_x4/SETTINGS.TXT
wifi_ssid=...
wifi_pass=...
```

## User flow

From the category dashboard:

```text
Network -> Wi-Fi Networks
```

The X4 shows an on-device editor for:

```text
SSID
Password
Save to _x4/SETTINGS.TXT
```

Controls:

```text
Next / Prev        cycle character picker
Left / Right       switch field: SSID, Password, Save
Select             add selected character, or save when Save is selected
Long Select        delete one character from the active field
Long Left/Right    clear the active SSID or Password field
Back               return to Network category
```

Password text is masked on screen. It is saved only as the existing `wifi_pass=` key in `/_x4/SETTINGS.TXT`.

## Transfer behavior

`Network -> Wi-Fi Transfer` still reads `wifi_ssid` and `wifi_pass` from `/_x4/SETTINGS.TXT`.

After Wi-Fi connects and DHCP succeeds, transfer mode now also attempts NTP sync and writes the result to:

```text
/_x4/TIME.TXT
```

This means the user does not need to run the separate Date & Time sync path before transfer.

## Removed behavior

This overlay removes the previous separate profile file/browser editor path:

```text
/_x4/WIFI.TXT
http://x4.local/wifi
http://x4.local/wifi.txt
```

No SD mount/probe behavior, SPI arbitration, display refresh scheduler, EPUB loading, reader page turns, or storage layout behavior is changed.
