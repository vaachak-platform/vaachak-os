# X4 Keyboard CrossPoint Session Repair

This repair changes the Vaachak Wi-Fi keyboard integration to follow the CrossPoint-style ownership model:

- the keyboard/session owns the active text buffer while editing;
- settings reloads do not overwrite SSID/password bytes while the keyboard is open;
- keyboard navigation is RAM-only and clears stale pending redraws;
- OK/Select, delete, clear, done/back, and Save are the only paths that intentionally redraw or persist.

This is intentionally scoped to Wi-Fi settings keyboard behavior. It does not change Wi-Fi scan, Wi-Fi Transfer, NTP/date-time sync, EPUB rendering, display drivers, SD mount/probe, or SPI arbitration.
