# Hardware Runtime Executor Observability

`hardware_runtime_executor_observability=ok`

This document is the canonical observability note for the Vaachak-owned hardware runtime executor path on `target-xteink-x4`.

## Purpose

The previous hardware executor extraction and wiring layers establish a Vaachak-owned runtime path for SPI, storage, display, and input while keeping the low-level Pulp-compatible executors active. This observability slice adds metadata-only boot/debug markers that prove the selected executor and wired runtime paths are present.

## Active ownership model

| Area | Vaachak-owned layer | Active low-level executor |
| --- | --- | --- |
| Consolidated hardware executor | `hardware_runtime_executor.rs` | `PulpCompatibility` |
| Wired runtime paths | `hardware_runtime_executor_wiring.rs` | `PulpCompatibility` |
| Observability markers | `hardware_runtime_executor_observability.rs` | metadata only |
| Backend descriptor | `hardware_runtime_observability_pulp_backend.rs` | `vendor/pulp-os imported runtime` |

## Observed boot/debug markers

The observability layer exposes the following metadata marker texts:

- `hardware.executor.layer.selected`
- `hardware.executor.wiring.selected`
- `hardware.executor.backend.pulp_compatible`
- `hardware.executor.spi.paths.selected`
- `hardware.executor.storage.paths.selected`
- `hardware.executor.display.paths.selected`
- `hardware.executor.input.paths.selected`
- `hardware.executor.behavior.preserved`
- `hardware.executor.path.selected`

These markers are descriptors only. They do not print to the display, write to storage, sample input, or toggle SPI hardware.

## Runtime paths observed

The observability layer verifies the wired paths from `hardware-runtime-executor-wiring.md`:

- `BootStorageAvailability`
- `LibraryDirectoryListing`
- `ReaderFileOpenIntent`
- `ReaderFileChunkIntent`
- `DisplayFullRefreshHandoff`
- `DisplayPartialRefreshHandoff`
- `InputButtonScanHandoff`
- `InputNavigationHandoff`
- `SharedSpiDisplayHandoff`
- `SharedSpiStorageHandoff`

## Explicit non-goals

This slice does not change:

- physical SPI transfer execution
- chip-select GPIO toggling
- SD/MMC low-level execution
- FAT implementation algorithms
- SSD1677 draw/full-refresh/partial-refresh algorithms
- button ADC scan/debounce/navigation behavior
- reader/file-browser UX behavior
- app navigation behavior

## Validation

Run:

```bash
cargo fmt --all
./scripts/validate_hardware_runtime_executor_observability.sh
cargo build
```

Expected marker:

```text
hardware_runtime_executor_observability=ok
```
## Runtime boot marker surface

Runtime-visible boot markers are documented in `hardware-runtime-executor-boot-markers.md`.
## Acceptance cleanup

Final GitHub-readiness cleanup for this stack is documented in `hardware-runtime-executor-acceptance.md`.

