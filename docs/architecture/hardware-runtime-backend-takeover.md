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

## Input native backend handoff

The input handoff now calls `VaachakInputBackendNativeExecutor` before continuing through the accepted hardware backend takeover bridge. The selected input-native backend is `VaachakInputNativeWithPulpSampling`; the low-level physical sampling fallback remains `PulpCompatibility`.

No ADC ladder sampling, debounce/repeat execution, navigation dispatch, display behavior, storage behavior, SPI transfer behavior, reader/file-browser UX, or app navigation behavior is rewritten by this handoff.

## Display native refresh shell

`display_backend_native_refresh_shell` adds the first Vaachak-native display backend slice. The backend takeover path now calls `VaachakDisplayBackendNativeRefreshShell` for full and partial refresh handoff pre-routing, while the active low-level executor remains `PulpCompatibility`.

## Display native refresh shell cleanup checkpoint

The accepted `display_backend_native_refresh_shell_cleanup` checkpoint verifies that `VaachakDisplayNativeRefreshShellWithPulpExecutor` remains selected, `PulpCompatibility` remains the refresh executor fallback, and no SSD1677 draw/full/partial refresh algorithm, physical SPI transfer, chip-select, storage, input, reader/file-browser UX, or app-navigation behavior moved.

## Input native event pipeline

`input_backend_native_event_pipeline` moves Vaachak-owned button event behavior into `target-xteink-x4` while physical ADC/GPIO sampling remains `PulpCompatibility`. The backend takeover path calls `VaachakInputBackendNativeEventPipeline` for scan and navigation handoff pre-routing before continuing through the existing Pulp-compatible backend.
## Input native event pipeline cleanup checkpoint

`input_backend_native_event_pipeline_cleanup` finalizes the first actual input behavior migration. The backend takeover path calls `VaachakInputBackendNativeEventPipeline` for input scan and navigation handoff pre-routing while physical ADC/GPIO sampling remains `PulpCompatibility`.

## Display native refresh command executor

`display_backend_native_refresh_command_executor` moves refresh command selection behavior into the Vaachak `target-xteink-x4` layer. The selected low-level executor remains `PulpCompatibility`; SSD1677 draw, waveform, BUSY wait, physical SPI transfer, and chip-select behavior remain in the imported Pulp-compatible runtime.

## Display native refresh command executor cleanup checkpoint

The accepted `display_backend_native_refresh_command_executor_cleanup` checkpoint verifies that `VaachakDisplayRefreshCommandExecutorWithPulpExecutor` remains selected, `PulpCompatibility` remains the low-level SSD1677 executor fallback, the rustfmt repair has been folded into the main checkpoint, and no SSD1677 draw/full/partial refresh algorithm, waveform, BUSY wait, physical SPI transfer, chip-select, storage, input, reader/file-browser UX, or app-navigation behavior moved.
## Storage native SD/MMC/FAT executor

`storage_backend_native_sd_mmc_fat_executor=ok` moves storage command decision behavior, probe/mount state interpretation, FAT operation classification, path-role policy, and destructive-operation denial into Vaachak while keeping the low-level SD/MMC block driver and FAT algorithms behind `PulpCompatibility`.

