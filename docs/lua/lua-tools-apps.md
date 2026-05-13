# Lua Tools Apps

Vaachak OS uses `/VAACHAK/APPS` as the canonical SD root for optional Lua apps.
Physical folders and files remain uppercase 8.3-safe.

## Added Tools

| Physical folder | Logical app id | Menu label | Data file |
| --- | --- | --- | --- |
| `DICT` | `dictionary` | Dictionary | `WORDS.TXT` |
| `UNITS` | `unit_converter` | Unit Converter | `UNITS.TXT` |

## SD layout

```text
/VAACHAK/APPS/DICT/APP.TOM
/VAACHAK/APPS/DICT/MAIN.LUA
/VAACHAK/APPS/DICT/WORDS.TXT
/VAACHAK/APPS/UNITS/APP.TOM
/VAACHAK/APPS/UNITS/MAIN.LUA
/VAACHAK/APPS/UNITS/UNITS.TXT
```

The current slice adds SD-loaded stub screens only. Search/conversion logic is intentionally deferred.

Marker: `vaachak-lua-tools-dictionary-unit-converter-stub-pack-ok`
