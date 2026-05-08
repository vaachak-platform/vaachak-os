# Hardware Runtime Executor Live Handoff Cleanup

This document is the canonical cleanup checkpoint for the Vaachak hardware runtime executor live handoff.

## Accepted live handoff stack

The cleanup checkpoint assumes the following stack is already accepted:

- `hardware_runtime_executor_runtime_use=ok`
- `hardware_runtime_executor_runtime_use_cleanup=ok`
- `hardware_runtime_executor_live_path_handoff=ok`

## Final cleanup ownership statement

Vaachak owns the live hardware executor handoff boundary in `target-xteink-x4`.
The active low-level executor remains the Pulp-compatible backend.

## Live handoff sites covered

- Boot preflight
- Imported Pulp reader runtime boundary
- Storage availability handoff
- Display refresh handoff
- Input runtime handoff

## Behavior preservation

This cleanup checkpoint does not change:

- physical SPI transfer execution
- chip-select GPIO toggling
- low-level SD/MMC execution
- FAT algorithms
- SSD1677 display algorithms
- button ADC scan, debounce, repeat, or navigation behavior
- reader/file-browser UX
- app navigation behavior

## Validation

Run:

```bash
cargo fmt --all
./scripts/validate_hardware_runtime_executor_runtime_use.sh
./scripts/validate_hardware_runtime_executor_runtime_use_cleanup.sh
./scripts/validate_hardware_runtime_executor_live_path_handoff.sh
./scripts/validate_hardware_runtime_executor_live_handoff_cleanup.sh
cargo build
```

Expected markers:

```text
hardware_runtime_executor_runtime_use=ok
hardware_runtime_executor_runtime_use_cleanup=ok
hardware_runtime_executor_live_path_handoff=ok
hardware_runtime_executor_live_handoff_cleanup=ok
```
