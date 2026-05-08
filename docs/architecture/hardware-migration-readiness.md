# Hardware migration readiness checklist

Do not start SD/SPI/display behavior migration until this checklist is green.

## Build/readiness gate

Run:

```bash
cargo fmt --all --check
./scripts/check_no_milestone_artifacts.sh .
./scripts/validate_controlled_extraction_consolidation.sh
```

The validation script must pass before any hardware-adjacent extraction begins.

## Current runtime baseline

The active baseline is still `vendor/pulp-os`. Before moving behavior, confirm:

- `vendor/pulp-os` builds in release mode.
- `target-xteink-x4` builds in release mode for `riscv32imc-unknown-none-elf`.
- The device flashes and boots.
- Reader, Settings, Wi-Fi Transfer, Date & Time, title cache, sleep image, and display behavior are stable.

## On-device checklist

After flashing the current build:

1. Reader
   - Open a regular TXT file.
   - Open a regular EPUB file.
   - Open `YEARLY_H.TXT` with prepared cache.
   - Open the mixed prepared EPUB smoke file.
   - Confirm progress restores for each supported path.
   - Confirm EPUB bookmarks still work.

2. Settings
   - Confirm reader settings sync both ways.
   - Confirm sleep image mode persists.
   - Confirm title-cache action rows are selectable/actionable.
   - Confirm battery/header display remains consistent.

3. Wi-Fi Transfer
   - Confirm Original Transfer tab works.
   - Confirm Chunked Resume tab works.
   - Confirm large FCACHE upload notes remain visible.
   - Confirm credentials are not shown in browser UI.

4. Date & Time
   - Confirm Live/Cached/Unsynced status behavior.
   - Confirm Back/retry does not lock buttons.
   - Confirm cached time is preserved after failed retry.

5. Display and sleep
   - Confirm reader header/body/footer remain readable.
   - Confirm cache diagnostics do not pollute the header.
   - Confirm sleep image mode still works.

## Hardware migration entry criteria

Only start hardware-adjacent migration when all of these are true:

- Pure model tests are green.
- Active runtime release build is green.
- Flash and on-device smoke are green.
- There is one clearly scoped behavior to move.
- There is a rollback path to the Pulp-owned behavior.
- The migration does not combine SD, SPI, display, and input changes in one step.

## Recommended first hardware-adjacent candidates

Start with the lowest-risk adapters before moving drivers:

1. Storage path adapter wiring that calls Vaachak-owned path helpers but still uses Pulp SD I/O.
2. Input semantic adapter wiring that maps Pulp button events to Vaachak-owned semantic actions.
3. Display chrome geometry adapter wiring that uses Vaachak-owned layout constants while keeping Pulp drawing.

Do actual SD/SPI/display driver behavior last.
