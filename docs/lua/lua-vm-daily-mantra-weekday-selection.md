# Lua VM Daily Mantra Weekday Selection

This slice extends the first on-device Lua VM proof so Daily Mantra can use a VM-derived weekday to select the matching `MANTRAS.TXT` record.

## Physical layout

```text
/VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/MANTRA/MAIN.LUA
/VAACHAK/APPS/MANTRA/MANTRAS.TXT
```

## MAIN.LUA VM fields

```lua
vm_expression = "return 108 + 0"
vm_day_expression = "return 'Monday'"
```

The VM still runs a tiny constrained subset. `vm_day_expression` supports a string return such as `return 'Monday'` and is used to select the matching `MANTRAS.TXT` record.

## Expected screen

```text
Daily Mantra
Day: Monday
Mantra: Om Namah Shivaya - A steady mind turns
...
VM result: 108
```

If `lua-vm` is not enabled, the existing fallback/subset parser remains active.
