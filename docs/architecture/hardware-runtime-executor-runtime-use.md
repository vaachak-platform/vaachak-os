# Hardware Runtime Executor Runtime Use

`hardware_runtime_executor_runtime_use` is the first runtime-use adoption layer for the consolidated Vaachak hardware executor.

The prior hardware executor work established ownership, executor extraction, wiring, observability, boot markers, and acceptance cleanup. This deliverable begins using that accepted executor surface from selected boot/runtime call sites while keeping the existing Pulp-compatible low-level runtime active.

## Ownership move

Vaachak now owns the selected runtime-use entrypoint in:

```text
目标: target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_runtime_use.rs
owner: target-xteink-x4 Vaachak layer
marker: hardware_runtime_executor_runtime_use=ok
```

## Runtime-use call sites

The following selected runtime intents are routed through the Vaachak executor entrypoints:

```text
BootExecutorPreflight
BoardSpiOwnershipHandoff
DisplayInitHandoff
DisplayRefreshHandoff
StorageCardDetectHandoff
StorageMountHandoff
StorageDirectoryListingHandoff
ReaderFileOpenHandoff
InputDriverInitHandoff
InputTaskHandoff
```

These call sites map onto the accepted hardware executor wiring paths:

```text
BootStorageAvailability
SharedSpiStorageHandoff
SharedSpiDisplayHandoff
DisplayFullRefreshHandoff
LibraryDirectoryListing
ReaderFileOpenIntent
InputButtonScanHandoff
InputNavigationHandoff
```

## Active backend

The active backend remains:

```text
PulpCompatibility
```

This means the Vaachak runtime-use layer routes intent and ownership through the accepted executor entrypoints, but the currently working hardware executor remains the existing Pulp-compatible runtime underneath.

## Preserved behavior

This deliverable intentionally does not rewrite or move:

```text
physical SPI transfer
chip-select GPIO toggling
low-level SD/MMC execution
FAT implementation algorithms
SSD1677 display draw/full-refresh/partial-refresh algorithms
button ADC scan/debounce/navigation behavior
reader/file-browser UX behavior
app navigation behavior
```

## Runtime evidence

After flashing, the serial/debug boot stream should include the prior accepted hardware executor markers plus:

```text
hardware_runtime_executor_runtime_use=ok
hardware.executor.runtime_use.selected_call_sites=10
hardware.executor.runtime_use.backend.pulp_compatible
hardware.executor.runtime_use.behavior.preserved
```

## Validation

Run:

```bash
cargo fmt --all
./scripts/validate_hardware_runtime_executor_runtime_use.sh
cargo build
```

Expected:

```text
hardware_runtime_executor_runtime_use=ok
```

## Cleanup checkpoint

The accepted runtime-use layer is finalized by:

```text
hardware_runtime_executor_runtime_use_cleanup=ok
```

Canonical cleanup document:

```text
docs/architecture/hardware-runtime-executor-runtime-use-cleanup.md
```

The cleanup checkpoint folds the validator repair into the main runtime-use validator and removes temporary runtime-use overlay artifacts without changing hardware behavior.
