# Hardware Runtime Backend Takeover Bridge

Status: `hardware_runtime_backend_takeover_bridge=ok`

Cleanup checkpoint: [`hardware-runtime-backend-takeover-cleanup.md`](hardware-runtime-backend-takeover-cleanup.md)

This checkpoint turns the accepted live hardware executor handoff into a real Vaachak-owned backend interface layer.

## Ownership

| Area | Owner after this deliverable | Active low-level executor |
| --- | --- | --- |
| Backend traits | `target-xteink-x4 Vaachak layer` | N/A |
| Request/result structs | `target-xteink-x4 Vaachak layer` | N/A |
| Backend selection | `target-xteink-x4 Vaachak layer` | `PulpCompatibility` |
| Physical SPI transfer | Pulp-compatible imported runtime | `vendor/pulp-os imported runtime` |
| SD/MMC/FAT algorithms | Pulp-compatible imported runtime | `vendor/pulp-os imported runtime` |
| SSD1677 draw/refresh algorithms | Pulp-compatible imported runtime | `vendor/pulp-os imported runtime` |
| Button ADC/debounce/navigation algorithms | Pulp-compatible imported runtime | `vendor/pulp-os imported runtime` |

## Vaachak-owned backend traits

The bridge adds Vaachak-owned traits for:

- `VaachakSpiTransactionExecutor`
- `VaachakStorageProbeMountExecutor`
- `VaachakStorageFatAccessExecutor`
- `VaachakDisplayExecutor`
- `VaachakInputExecutor`

It also adds the combined `VaachakHardwareRuntimeBackend` interface.

## Vaachak-owned request/result structs

The bridge adds request/result structs for:

- SPI display transaction handoff
- SPI storage transaction handoff
- SD card availability/probe/mount handoff
- storage directory listing handoff
- file open/read handoff
- state/cache path resolution handoff
- display full refresh handoff
- display partial refresh handoff
- input scan handoff
- input navigation handoff

## Active backend

The active backend remains explicit:

```text
active backend = PulpCompatibility
backend owner = target-xteink-x4 Vaachak layer
low-level executor = vendor/pulp-os imported runtime
```

## What did not move

This checkpoint does not rewrite or move:

- physical SPI transfer
- chip-select GPIO toggling
- SD/MMC card initialization
- FAT algorithms
- SSD1677 draw/full/partial refresh algorithms
- button ADC sampling
- input debounce/navigation algorithms
- reader/file-browser UX
- app navigation behavior

## Live handoff integration

`hardware_runtime_executor_live_handoff.rs` now references `VaachakHardwareRuntimeBackendTakeover` and calls the backend takeover layer from its live handoff adoption functions.

That means the live handoff path now goes through Vaachak-owned backend traits while preserving the PulpCompatibility low-level executor.

## Cleanup checkpoint

`hardware_runtime_backend_takeover_cleanup=ok` is the final acceptance checkpoint for this backend takeover bridge. It verifies that the Vaachak-owned backend traits, PulpCompatibility implementation, live handoff integration, runtime-use cleanup, and behavior-preservation guards remain aligned.
