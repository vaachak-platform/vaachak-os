# Hardware Runtime Executor Boot Markers

Marker: `hardware_runtime_executor_boot_markers=ok`

This document is the runtime-evidence layer for the Vaachak hardware executor path.
It surfaces the already accepted executor, wiring, and observability selections during boot without changing hardware behavior.

## What moved

Vaachak now owns the boot/debug marker entrypoint for the consolidated hardware runtime executor path:

- `target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_boot_markers.rs`
- `VaachakBoot::emit_hardware_runtime_executor_boot_markers()`
- real boot path call in `target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs`

The boot path now emits a serial/debug marker set showing that the Vaachak executor, wiring, observability, SPI, storage, display, and input paths are selected.

## Runtime marker set

The boot/debug stream emits:

```text
hardware_runtime_executor_boot_markers=ok
hardware_runtime_executor_observability=ok
hardware.executor.layer.selected
hardware.executor.wiring.selected
hardware.executor.backend.pulp_compatible
hardware.executor.spi.paths.selected
hardware.executor.storage.paths.selected
hardware.executor.display.paths.selected
hardware.executor.input.paths.selected
hardware.executor.behavior.preserved
```

These are serial/debug boot markers only. They are not rendered on the e-paper display.

## What did not move

This deliverable does not move or rewrite:

- physical SPI transfer execution
- chip-select GPIO toggling
- SD/MMC low-level execution
- FAT implementation algorithms
- SSD1677 draw/full-refresh/partial-refresh algorithms
- button ADC scan/debounce/navigation behavior
- reader or file-browser UX behavior
- app navigation behavior

## Dependencies

This layer depends on the accepted hardware executor stack:

- `hardware_runtime_executor_extraction=ok`
- `hardware_runtime_executor_wiring=ok`
- `hardware_runtime_executor_observability=ok`

## Validation

Run:

```bash
cargo fmt --all
./scripts/validate_hardware_runtime_executor_boot_markers.sh
cargo build
```

Expected result:

```text
hardware_runtime_executor_boot_markers=ok
```

## Hardware smoke

After flashing, check the serial monitor for the marker set above, then confirm:

- device boots normally
- Home/category dashboard appears
- buttons/navigation work
- file browser opens
- SD files list
- TXT/EPUB still open
- display refresh looks unchanged
- no SD mount/probe regression
- no input freeze/regression
