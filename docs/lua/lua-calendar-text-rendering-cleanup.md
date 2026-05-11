# Lua Calendar Text Rendering Cleanup

This cleanup makes `EVENTS.TXT` authoritative for Calendar display text.

## SD layout

```text
/VAACHAK/APPS/CALENDAR/APP.TOM
/VAACHAK/APPS/CALENDAR/MAIN.LUA
/VAACHAK/APPS/CALENDAR/EVENTS.TXT
```

## EVENTS.TXT format

```text
YYYY-MM-DD|Title|Optional detail
```

The `|` separator is parsed and is not displayed. Old script display text is ignored for the event body so stale `MAIN.LUA` fields cannot produce `Title|Detail` on screen.

## Expected screen

```text
Calendar
Date: 2026-05-11
Event: Vaachak Lua Calendar - Second SD Lua
app proof under Productivity.
Back exits safely to Productivity
```

No vendor/pulp-os files are changed.
