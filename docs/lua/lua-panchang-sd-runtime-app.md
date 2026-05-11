# Lua Panchang SD Runtime App

Panchang is the third SD-loaded Lua app after Daily Mantra and Calendar. It uses precomputed data rather than doing astronomical calculations on the ESP32-C3.

## Physical SD layout

```text
/VAACHAK/APPS/PANCHANG/APP.TOM
/VAACHAK/APPS/PANCHANG/MAIN.LUA
/VAACHAK/APPS/PANCHANG/DATA/Y2026.TXT
```

The physical folder and files remain uppercase 8.3-safe. The logical app id remains `panchang` in `APP.TOM`.

## DATA/Y2026.TXT format

```text
YYYY-MM-DD|Tithi|Nakshatra|Optional note
```

The `|` separator is parsed and is not shown directly on the X4 screen.

## Optional VM field

```lua
vm_panchang_index_expression = "return 1 + 0"
```

When firmware is built with `--features lua-vm`, this expression selects the Panchang record to render. Without `lua-vm`, Panchang uses the first record and continues to build normally.

## Expected screen

```text
Panchang
Date: 2026-05-11
Tithi: Krishna Ashtami
Nakshatra: Shravana
Note: Good day for reading and reflection.
```

No vendor/pulp-os files are changed.


## Panchang nested DATA read repair

The Panchang app keeps precomputed records under the 8.3-safe nested data folder:

```text
/VAACHAK/APPS/PANCHANG/DATA/Y2026.TXT
```

The runtime reads this with a fixed-depth read-only helper rather than by passing `DATA/Y2026.TXT` as one filename. This preserves the canonical `/VAACHAK/APPS` deployment contract, keeps the `DATA` subfolder, and avoids recursive SD scanning or raw SD/FAT/SPI changes.


## Panchang nested data reader v2

Panchang data is read from the fixed-depth path:

```text
/VAACHAK/APPS/PANCHANG/DATA/Y2026.TXT
```

The runtime now uses a dedicated read-only helper for the path segments
`VAACHAK / APPS / PANCHANG / DATA / Y2026.TXT`. This avoids passing
`DATA/Y2026.TXT` as one file name and keeps the Wi-Fi Transfer-visible path and
runtime path aligned.


## Panchang exact data path repair

Lua Panchang reads the canonical data file through an explicit fixed-depth path:

```text
/VAACHAK/APPS/PANCHANG/DATA/Y2026.TXT
```

The app loader opens `VAACHAK`, then `APPS`, then `PANCHANG`, then `DATA`, and
finally reads `Y2026.TXT`. This mirrors the path shown by Wi-Fi Transfer and
avoids passing `DATA/Y2026.TXT` as a single filename. A flat app-root fallback at
`/VAACHAK/APPS/PANCHANG/Y2026.TXT` is retained only as a manual recovery path.
