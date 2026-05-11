# Lua Calendar SD Runtime App v2

Calendar is the second SD-loaded Lua app after Daily Mantra. It follows the accepted Biscuit-style category model by appending an optional Lua app to the native Productivity category while keeping native apps authoritative.

## Physical SD layout

```text
/VAACHAK/APPS/CALENDAR/APP.TOM
/VAACHAK/APPS/CALENDAR/MAIN.LUA
/VAACHAK/APPS/CALENDAR/EVENTS.TXT
```

The physical folder and files remain uppercase 8.3-safe. The logical app id remains `calendar` in `APP.TOM`.

## EVENTS.TXT format

```text
YYYY-MM-DD|Title|Optional detail
```

The `|` separator is parsed and is not shown directly on the X4 screen.

## Optional VM field

```lua
vm_event_index_expression = "return 1 + 0"
```

When firmware is built with `--features lua-vm`, this expression selects the event record to render. Without `lua-vm`, Calendar uses the first event record and continues to build normally.

## Expected screen

```text
Calendar
Date: 2026-05-11
Event: Vaachak Lua Calendar - Second SD Lua app proof
under Productivity.
Back exits safely to Productivity
```

No vendor/pulp-os files are changed.
