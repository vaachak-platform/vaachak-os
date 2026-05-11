# Lua App Deployment Contract

This document records the accepted Vaachak OS Lua app deployment model.

## Canonical SD root

All optional Lua apps are deployed under the SD-card root path:

```text
/VAACHAK/APPS
```

Native Vaachak OS features remain authoritative. Lua apps are optional additions for tools, productivity, experiments, and simple network features.

## Physical naming rule

The current X4 SD/FAT path is safest with uppercase 8.3 physical names. Use:

```text
NAME.EXT
```

where `NAME` is at most 8 characters and `EXT` is at most 3 characters. Keep folder names uppercase. Keep long logical identity inside `APP.TOM`.

## Final physical-to-logical map

```text
/VAACHAK/APPS/MANTRA   -> id = "daily_mantra" -> Tools -> Daily Mantra
/VAACHAK/APPS/CALENDAR -> id = "calendar"     -> Productivity -> Calendar
/VAACHAK/APPS/PANCHANG -> id = "panchang"     -> Tools -> Panchang
```

## Required sample layout

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

## Manifest rule

`APP.TOM` uses logical app identity and dashboard metadata:

```text
id = "daily_mantra"
name = "Daily Mantra"
category = "Tools"
type = "activity"
version = "0.1.0"
entry = "MAIN.LUA"
capabilities = ["display", "input", "storage", "time"]
```

The physical folder is short and uppercase. The logical app id remains descriptive snake_case.

## Wi-Fi Transfer rule

Wi-Fi Transfer should upload to uppercase 8.3 physical paths. For example:

```text
/VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/MANTRA/MAIN.LUA
/VAACHAK/APPS/MANTRA/MANTRAS.TXT
```

Avoid uploading lower-case long-name folders such as `/VAACHAK/APPS/daily_mantra` while the embedded FAT path layer is short-name oriented.

## Commit hygiene

Old overlay zip files, extracted overlay folders, and temporary generated apply/patch/validator scripts should not be committed. Keep only source files, docs, examples, and the current final validator needed for repository hygiene.
