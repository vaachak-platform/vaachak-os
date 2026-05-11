# Wi-Fi Transfer Page Repair v2

This repair restores the Wi-Fi Transfer page with a self-contained, browser-safe script after the previous page repair failed static validation.

The page keeps the existing server endpoints:

- `/files`, `/upload`, `/mkdir`, `/rename`, `/delete`, `/download` for Original Transfer
- `/v2/mkdir`, `/v2/stat`, `/v2/chunk` for Chunked Resume

Lua app uploads are normalized to uppercase FAT 8.3-safe paths under `/VAACHAK/APPS`:

```text
/VAACHAK/APPS/daily_mantra/app.toml  -> /VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/daily_mantras/app.toml -> /VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/mantra/main.lua        -> /VAACHAK/APPS/MANTRA/MAIN.LUA
```

The page must not attempt to create `/VAACHAK/APPS/daily_mantras` or `/VAACHAK/APPS/DAILY_MANTRAS`.
