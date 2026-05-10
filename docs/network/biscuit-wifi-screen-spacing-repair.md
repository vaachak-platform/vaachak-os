# Biscuit Wi-Fi screen spacing repair

This overlay fixes the Xteink X4 Wi-Fi Networks screen layout after the larger-font update.

## Scope

- Remove the duplicate `Default/Profile` subtitle under the title.
- Draw the default profile selector once, as its own row.
- Increase vertical separation between title, default profile row, nearby networks label, and list rows.
- Show four network rows instead of five to keep the footer clear of the physical button labels.
- Move footer help text higher so it does not collide with the bottom button bar.
- Keep password text visible while typing.

## Non-goals

- No Wi-Fi scan behavior changes.
- No Wi-Fi Transfer changes.
- No NTP/date-time changes.
- No `_x4/WIFI.TXT` or browser Wi-Fi route changes.
- No SD/SPI/EPUB/display-driver changes.
