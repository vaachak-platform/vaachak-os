# Hardware Runtime Executor Live Handoff

This document is the canonical note for the `hardware_runtime_executor_live_path_handoff` deliverable.

## Goal

Begin live runtime handoff into the Vaachak hardware executor path while preserving the existing working Pulp-compatible low-level hardware execution.

This is not a reader UX rewrite, file-browser UX rewrite, app-navigation rewrite, display driver rewrite, input driver rewrite, SD/MMC rewrite, or FAT algorithm rewrite.

## Vaachak-owned live handoff entrypoint

The live handoff surface is:

```text
 target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_live_handoff.rs
```

It exposes one Vaachak-owned runtime handoff layer for these live runtime boundaries:

```text
BootPreflight
ImportedPulpReaderRuntimeBoundary
StorageAvailabilityHandoff
DisplayRefreshHandoff
InputRuntimeHandoff
```

## Runtime-use acceptance dependency

Live handoff is allowed only after the accepted runtime-use layer reports ready:

```text
VaachakHardwareRuntimeExecutorRuntimeUse::active_runtime_preflight()
VaachakHardwareRuntimeExecutorRuntimeUse::runtime_use_ok()
VaachakHardwareRuntimeExecutorAcceptance::acceptance_ok()
```

This keeps the handoff path behind the accepted hardware executor stack.

## Pulp-compatible backend remains active

The active low-level backend remains:

```text
PulpCompatibility
```

The current working imported Pulp runtime remains responsible for physical SPI transfer, chip-select toggling, SD/MMC execution, FAT algorithms, SSD1677 draw/full/partial refresh algorithms, and input scan/debounce/navigation behavior.

## Runtime marker set

Expected serial/debug markers after flashing:

```text
hardware_runtime_executor_live_path_handoff=ok
hardware.executor.live_handoff.boot_preflight
hardware.executor.live_handoff.imported_pulp_reader_runtime_boundary
hardware.executor.live_handoff.storage_availability
hardware.executor.live_handoff.display_refresh
hardware.executor.live_handoff.input_runtime
hardware.executor.live_handoff.backend.pulp_compatible
hardware.executor.live_handoff.behavior.preserved
```

## Behavior preservation

The deliverable intentionally does not change:

```text
reader/file-browser UX
app navigation behavior
display rendering algorithms
input scan/debounce/navigation behavior
SD/MMC or FAT algorithms
physical SPI transfer or chip-select toggling
```

## Validation

```bash
cargo fmt --all
./scripts/validate_hardware_runtime_executor_runtime_use.sh
./scripts/validate_hardware_runtime_executor_runtime_use_cleanup.sh
./scripts/validate_hardware_runtime_executor_live_path_handoff.sh
cargo build
```

Expected marker:

```text
hardware_runtime_executor_live_path_handoff=ok
```
