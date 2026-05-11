# Lua Panchang manager-path reader

The SD manager page proves that the Panchang data file exists at:

```text
/VAACHAK/APPS/PANCHANG/DATA/Y2026.TXT
```

The Lua Panchang runtime should read that file through the same nested path traversal used by Wi-Fi Transfer listing/upload:

```text
path = VAACHAK/APPS/PANCHANG/DATA
name = Y2026.TXT
```

This avoids treating `DATA/Y2026.TXT` as a single filename and avoids one-off directory traversal behavior that can drift from the Wi-Fi Transfer manager.

The physical deployment contract remains 8.3-safe:

```text
/VAACHAK/APPS/PANCHANG/APP.TOM
/VAACHAK/APPS/PANCHANG/MAIN.LUA
/VAACHAK/APPS/PANCHANG/DATA/Y2026.TXT
```

No recursive SD scan, Lua app execution change, raw SD/FAT/SPI change, or `vendor/pulp-os` change is introduced by this repair.
