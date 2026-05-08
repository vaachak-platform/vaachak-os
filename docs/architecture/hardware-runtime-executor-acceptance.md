# Hardware Runtime Executor Acceptance Cleanup

This document is the GitHub-readiness checkpoint for the Vaachak OS hardware runtime executor extraction stack.

Acceptance marker:

```text
hardware_runtime_executor_acceptance_cleanup=ok
```

## Accepted stack covered by this cleanup

The cleanup layer consolidates the accepted hardware ownership and executor path without changing behavior:

| Layer | Accepted marker |
| --- | --- |
| Hardware runtime ownership consolidation | `hardware_runtime_ownership_consolidation=ok` |
| SPI bus arbitration runtime owner | `spi_bus_arbitration_runtime_owner=ok` |
| Storage probe/mount runtime executor bridge | `storage_probe_mount_runtime_executor_bridge=ok` |
| Hardware runtime executor extraction | `hardware_runtime_executor_extraction=ok` |
| Hardware runtime executor wiring | `hardware_runtime_executor_wiring=ok` |
| Hardware runtime executor observability | `hardware_runtime_executor_observability=ok` |
| Hardware runtime executor boot markers | `hardware_runtime_executor_boot_markers=ok` |

## Ownership status

Vaachak now owns the consolidated hardware runtime executor acceptance surface in `target-xteink-x4`.

The active low-level backend remains Pulp-compatible to preserve working behavior:

- physical SPI transfer and chip-select execution remain unchanged
- SD/MMC low-level execution remains unchanged
- FAT implementation behavior remains unchanged
- SSD1677 draw/full-refresh/partial-refresh behavior remains unchanged
- ADC/button scan, debounce, repeat, and navigation behavior remain unchanged
- reader/file-browser UX behavior remains unchanged
- app navigation behavior remains unchanged

## New acceptance files

```text
target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_acceptance.rs
target-xteink-x4/src/vaachak_x4/contracts/hardware_runtime_executor_acceptance_smoke.rs
scripts/validate_hardware_runtime_executor_acceptance_cleanup.sh
scripts/cleanup_hardware_runtime_executor_artifacts.sh
docs/architecture/hardware-runtime-executor-acceptance.md
```

## Cleanup script

After applying and validating this deliverable, remove old overlay folders and zip files from the repository root:

```bash
./scripts/cleanup_hardware_runtime_executor_artifacts.sh --include-current
```

The cleanup script only removes known overlay package directories and zip files. It does not remove source files, docs, validator scripts, `vendor/`, `target-xteink-x4/`, or `.git/`.

## Validation

Run:

```bash
cargo fmt --all
./scripts/validate_hardware_runtime_executor_acceptance_cleanup.sh
cargo build
```

Expected:

```text
hardware_runtime_executor_acceptance_cleanup=ok
```

## Hardware smoke after acceptance

Flash as usual:

```bash
cargo run --release
```

Confirm:

```text
- device boots normally
- runtime executor boot markers appear on serial/debug stream
- Home/category dashboard appears
- all buttons and navigation still work
- file browser opens
- SD files still list
- TXT/EPUB files still open
- display refresh behavior looks unchanged
- no SD mount/probe regression
- no input freeze/regression
```
## Runtime use adoption

Selected boot/runtime hardware intent call sites are now routed through `hardware-runtime-executor-runtime-use.md` while Pulp-compatible low-level execution remains active.

