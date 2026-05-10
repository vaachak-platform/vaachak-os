# Biscuit Wi-Fi screen large layout

This overlay updates only the Biscuit-style Wi-Fi setup activity layout.

## Scope

- Use larger body fonts for the Wi-Fi network and password screens.
- Increase row height and spacing on the scan results list.
- Render the default profile selector as a bordered, readable row.
- Reduce footer text to two readable help lines above the physical button bar.
- Increase keyboard key height and spacing.
- Keep credentials in `_x4/SETTINGS.TXT`.
- Do not add `_x4/WIFI.TXT` or browser Wi-Fi editor routes.
- Do not change Wi-Fi Transfer, NTP/date-time sync, SD, SPI, EPUB, or display driver behavior.

## Expected device behavior

Open:

```text
Network -> Wi-Fi Networks
```

The screen should now show:

```text
Wi-Fi Networks
Default: Home   Profile: Home
[Home]   Work   Other
Nearby networks
<larger network rows>
```

The password screen should show a larger SSID/password area and larger keyboard keys.
