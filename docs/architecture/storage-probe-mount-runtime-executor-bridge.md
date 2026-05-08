# Storage Probe/Mount Runtime Executor Bridge

Acceptance marker:

```text
storage_probe_mount_runtime_executor_bridge=ok
```

## Purpose

This document is the canonical architecture note for the first narrow SD
probe/mount lifecycle executor bridge in the Vaachak Xteink X4 target.

The bridge moves the **lifecycle execution entrypoint** for SD probe/mount intent
into the Vaachak-owned `target-xteink-x4` layer while keeping the working
low-level SD/MMC, mount, and FAT implementation Pulp-compatible.

## Ownership map

| Area | Owner after this slice | Active executor |
| --- | --- | --- |
| SD probe/mount lifecycle intent entrypoint | Vaachak `target-xteink-x4` | Vaachak routes intent |
| Low-level card detect implementation | Pulp-compatible imported runtime | `vendor/pulp-os imported runtime` |
| Low-level card identification implementation | Pulp-compatible imported runtime | `vendor/pulp-os imported runtime` |
| Low-level mount implementation | Pulp-compatible imported runtime | `vendor/pulp-os imported runtime` |
| FAT read/write/list implementation | Pulp-compatible imported runtime | `vendor/pulp-os imported runtime` |
| SPI arbitration metadata | Vaachak | `spi_bus_arbitration_runtime_owner` |
| Display draw/refresh implementation | Pulp-compatible imported runtime | unchanged |
| Reader/file-browser behavior | Existing app path | unchanged |

## Public entrypoints

```text
target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_runtime_executor_bridge.rs
target-xteink-x4/src/vaachak_x4/physical/storage_probe_mount_executor_pulp_backend.rs
```

The Vaachak entrypoint is:

```text
VaachakStorageProbeMountRuntimeExecutorBridge::execute_lifecycle_intent(...)
```

This function is intentionally a routing/ownership entrypoint. It validates the
accepted storage probe/mount owner and SPI arbitration owner, then routes the
lifecycle intent to the Pulp compatibility executor descriptor.

## Lifecycle intents routed through Vaachak

```text
DetectCard
IdentifyCardAtSafeSpeed
ObserveCardAvailability
ObserveFatVolumeAvailability
```

All four intents route through Vaachak first and then resolve to the active
Pulp-compatible executor owner.

## Dependencies

This bridge depends on the accepted ownership layers:

```text
VaachakStorageProbeMountRuntimeOwner::ownership_ok()
VaachakSpiBusArbitrationRuntimeOwner::runtime_owner_ok()
VaachakStorageProbeMountExecutorPulpBackend::backend_ok()
```

## Explicit non-goals

This slice does not move or add:

```text
- low-level SD/MMC card initialization
- FAT read implementation
- FAT write implementation
- FAT list implementation
- file open/close behavior
- write, append, delete, rename, mkdir, truncate, or format behavior
- SPI physical transfer execution
- chip-select GPIO toggling
- display draw/refresh/partial-refresh behavior
- reader behavior
- file-browser behavior
```

## Static guardrails

The validator checks that:

```text
- Vaachak has the new lifecycle executor bridge entrypoint
- the Pulp compatibility executor remains active
- lifecycle intents route through Vaachak before the backend descriptor
- FAT read/write/list behavior did not move
- display behavior did not move
- reader/file-browser behavior did not move
- no vendor/pulp-os files are modified by the overlay
- no app/UI/storage consumer path is modified by the overlay
```

Run:

```bash
cargo fmt --all
./scripts/validate_storage_probe_mount_runtime_executor_bridge.sh
cargo build
```

Expected:

```text
storage_probe_mount_runtime_executor_bridge=ok
```
