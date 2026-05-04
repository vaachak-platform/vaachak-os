# Phase 41H — Biscuit UI Acceptance Freeze and Commit Baseline

Phase 41H freezes the Biscuit UI baseline after the active Home dashboard and
Phase 41G polish.

## Frozen UI surfaces

```text
Home:
- Biscuit dashboard active
- old vertical list Home UI gone
- card fonts readable
- Reader card detail not badly clipped
- exactly one footer row

Files/Library:
- title display still correct
- TXT/MD names still come from _X4/TITLES.BIN
- EPUB/EPU metadata titles still work

Reader:
- open/back/restore still works
- pagination unchanged
- footer/input unchanged
```

## Non-goals

```text
- no new UI changes
- no route changes
- no title-cache changes
- no reader pagination changes
- no input calibration changes
- no write-lane changes
- no display geometry changes
```

marker=phase41h=x4-biscuit-ui-acceptance-freeze-ok
