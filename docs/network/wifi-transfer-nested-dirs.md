# Wi-Fi Transfer nested directory support

This update expands Vaachak-owned Wi-Fi Transfer path handling so Lua app folders can be uploaded without removing the SD card.

## Supported use case

The chunked transfer page can now create nested directories deeper than two levels, including paths such as:

```text
/VAACHAK/APPS/MANTRA/MAIN.LUA
/VAACHAK/APPS/MANTRA/MANTRAS.TXT
/VAACHAK/APPS/CALENDAR/EVENTS.TXT
/VAACHAK/APPS/PANCHANG/DATA/2026.TXT
```

## Safety rules

The transfer path normalizer still rejects traversal and unsafe separators. Directory creation remains bounded by a fixed component count and fixed stack buffers.

```text
maximum normalized transfer path buffer: 128 bytes
maximum path components: 8
rejected components: empty, ., .., backslash, colon
```

## Runtime ownership

This does not change Wi-Fi radio ownership, FAT implementation, SD mount/probe behavior, SPI arbitration, dashboard wiring, Lua VM behavior, or vendor code. It only extends the existing Vaachak Wi-Fi Transfer upload path handling and browser upload flow.

## Lua app uploads with uppercase 8.3 paths

For Xteink X4 Wi-Fi Transfer, Lua app uploads should use uppercase 8.3-safe physical paths. This avoids failures from long folder names such as `daily_mantra` and four-character extensions such as `app.toml`.

Upload Daily Mantra files to:

```text
/VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/MANTRA/MAIN.LUA
/VAACHAK/APPS/MANTRA/MANTRAS.TXT
```

Future sample apps should use the same convention:

```text
/VAACHAK/APPS/CALENDAR/APP.TOM
/VAACHAK/APPS/CALENDAR/MAIN.LUA
/VAACHAK/APPS/CALENDAR/EVENTS.TXT
/VAACHAK/APPS/PANCHANG/APP.TOM
/VAACHAK/APPS/PANCHANG/MAIN.LUA
/VAACHAK/APPS/PANCHANG/DATA/2026.TXT
```


## Lua app 8.3 upload aliases

The current X4 SD/FAT path is safest with uppercase FAT 8.3 physical names.
The logical app id can remain longer, for example `daily_mantra`, but the SD
folder used by Wi-Fi Transfer should be short and uppercase.

Daily Mantra physical upload paths:

```text
/VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/MANTRA/MAIN.LUA
/VAACHAK/APPS/MANTRA/MANTRAS.TXT
```

The Chunked Resume browser also normalizes stale local sample paths so this
upload target is repaired automatically:

```text
/VAACHAK/APPS/daily_mantra/app.toml -> /VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/daily_mantra/main.lua -> /VAACHAK/APPS/MANTRA/MAIN.LUA
/VAACHAK/APPS/daily_mantra/mantras.txt -> /VAACHAK/APPS/MANTRA/MANTRAS.TXT
```

## Lua app Wi-Fi Transfer 8.3 aliases

Lua app logical ids can remain descriptive, but the X4 SD/FAT upload path must
use uppercase FAT 8.3-safe physical names for now.

Daily Mantra aliases accepted by Chunked Resume:

```text
/VAACHAK/APPS/daily_mantra/app.toml   -> /VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/daily_mantras/app.toml  -> /VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/mantra/app.toml         -> /VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/MANTRA/app.toml         -> /VAACHAK/APPS/MANTRA/APP.TOM
```

When the target box is `/VAACHAK/APPS`, selected folders named
`daily_mantra`, `daily_mantras`, or `mantra` are preserved long enough for the
normalizer to map their children into `/VAACHAK/APPS/MANTRA/...`.

The initial mkdir pass also normalizes the target before creating folders, so
Chunked Resume must not attempt to create `/VAACHAK/APPS/daily_mantras`.


<!-- BEGIN LUA_APP_WIFI_UPLOADS -->
## Lua app uploads

Lua apps are deployed under `/VAACHAK/APPS` at the SD card root.
Use uppercase 8.3-safe physical names when uploading over Wi-Fi Transfer.

Recommended upload targets:

```text
/VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/MANTRA/MAIN.LUA
/VAACHAK/APPS/MANTRA/MANTRAS.TXT
/VAACHAK/APPS/CALENDAR/APP.TOM
/VAACHAK/APPS/CALENDAR/MAIN.LUA
/VAACHAK/APPS/CALENDAR/EVENTS.TXT
/VAACHAK/APPS/PANCHANG/APP.TOM
/VAACHAK/APPS/PANCHANG/MAIN.LUA
/VAACHAK/APPS/PANCHANG/DATA/Y2026.TXT
```

Do not upload Lua apps to long physical names like `/VAACHAK/APPS/daily_mantra/app.toml` while the embedded FAT layer is constrained to short names.
<!-- END LUA_APP_WIFI_UPLOADS -->

<!-- VAACHAK:LUA_DEPLOYMENT_CONTRACT:START -->
## Lua app upload paths

Use `/VAACHAK/APPS` as the Lua app root. Upload physical files using uppercase 8.3 names:

```text
/VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/MANTRA/MAIN.LUA
/VAACHAK/APPS/MANTRA/MANTRAS.TXT
/VAACHAK/APPS/CALENDAR/APP.TOM
/VAACHAK/APPS/CALENDAR/MAIN.LUA
/VAACHAK/APPS/CALENDAR/EVENTS.TXT
/VAACHAK/APPS/PANCHANG/APP.TOM
/VAACHAK/APPS/PANCHANG/MAIN.LUA
/VAACHAK/APPS/PANCHANG/DATA/Y2026.TXT
```

Do not rely on lower-case long physical paths such as `/VAACHAK/APPS/daily_mantra/app.toml` on the current embedded FAT path.
<!-- VAACHAK:LUA_DEPLOYMENT_CONTRACT:END -->
