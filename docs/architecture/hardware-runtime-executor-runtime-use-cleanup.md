# Hardware Runtime Executor Runtime Use Cleanup

`hardware_runtime_executor_runtime_use_cleanup` is the GitHub-ready cleanup checkpoint for the accepted `hardware_runtime_executor_runtime_use` deliverable.

It folds the runtime-use validator repair into the main runtime-use files, adds a cleanup acceptance surface, and removes temporary runtime-use overlay artifacts. It does not change hardware behavior.

## Marker

```text
hardware_runtime_executor_runtime_use_cleanup=ok
```

## Ownership

```text
owner: target-xteink-x4 Vaachak layer
scope: GitHub-ready cleanup for hardware runtime executor runtime-use adoption
```

## What is consolidated

```text
hardware_runtime_executor_runtime_use.rs
hardware_runtime_executor_runtime_use_smoke.rs
validate_hardware_runtime_executor_runtime_use.sh
hardware_runtime_executor_runtime_use_cleanup.rs
hardware_runtime_executor_runtime_use_cleanup_smoke.rs
cleanup_hardware_runtime_executor_runtime_use_artifacts.sh
validate_hardware_runtime_executor_runtime_use_cleanup.sh
```

## Runtime-use surface preserved

The runtime-use checkpoint still routes these selected call sites through the accepted Vaachak executor entrypoints:

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

## Active backend

The active low-level executor remains:

```text
PulpCompatibility
```

## Behavior intentionally unchanged

This cleanup does not rewrite or move:

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

## Temporary artifacts removed by cleanup script

```text
hardware_runtime_executor_runtime_use/
hardware_runtime_executor_runtime_use.zip
hardware_runtime_executor_runtime_use_validator_fix/
hardware_runtime_executor_runtime_use_validator_fix.zip
hardware_runtime_executor_runtime_use_cleanup/              only with --include-current
hardware_runtime_executor_runtime_use_cleanup.zip           only with --include-current
```

## Validation

Run:

```bash
cargo fmt --all
./scripts/validate_hardware_runtime_executor_runtime_use.sh
./scripts/validate_hardware_runtime_executor_runtime_use_cleanup.sh
cargo build
```

Expected:

```text
hardware_runtime_executor_runtime_use=ok
hardware_runtime_executor_runtime_use_cleanup=ok
```

## Hardware smoke

After flashing, behavior should remain unchanged:

```text
- boots normally
- hardware executor boot/runtime markers appear in serial/debug output
- Home/category dashboard appears
- buttons/navigation work
- file browser opens
- SD files list
- TXT/EPUB still open
- display refresh unchanged
- no SD mount/probe regression
- no input freeze/regression
```
