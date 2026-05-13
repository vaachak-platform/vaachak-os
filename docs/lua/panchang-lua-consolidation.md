# Panchang Lua consolidation

This repair leaves one visible Panchang app.

## User-facing behavior

- Productivity now contains `Panchang` instead of `Panchang Lite`.
- The Productivity `Panchang` entry opens the SD-backed Lua Panchang runtime.
- Tools no longer lists a duplicate `Lua Panchang` entry.
- The Lua Panchang screen now receives the former native Panchang Lite context:
  - cached/live Date & Time freshness
  - weekday
  - tithi
  - paksha
  - Hindu month estimate
  - configured location/timezone context
  - Day Mantra lookup when the mantra file is present

## Kept intact

- `/VAACHAK/APPS/PANCHANG/APP.TOM`
- `/VAACHAK/APPS/PANCHANG/MAIN.LUA`
- `/VAACHAK/APPS/PANCHANG/DATA/Y2026.TXT`
- SD app loading and existing Lua manifest contract
- Native Date & Time cache behavior
- Existing Calendar and Daily Mantra entries

## Removed from UI

- Duplicate Tools -> Lua Panchang route
- Productivity -> Panchang Lite route
