# Display Backend Native Refresh Shell

`display_backend_native_refresh_shell` is the first native display backend migration slice after the hardware backend takeover bridge.

Cleanup checkpoint: [`display-backend-native-refresh-shell-cleanup.md`](display-backend-native-refresh-shell-cleanup.md)

## Goal

Move the display refresh command shell into the Vaachak `target-xteink-x4` layer while keeping the active SSD1677/e-paper execution path Pulp-compatible.

Vaachak now owns:

- display refresh command normalization
- full refresh intent mapping
- partial refresh intent mapping
- clear/sleep/render metadata intent mapping
- refresh handoff pre-routing before the Pulp-compatible executor

The active low-level backend remains `PulpCompatibility`.

## Active backend

| Area | Owner |
| --- | --- |
| Native refresh shell | `target-xteink-x4 Vaachak layer` |
| Backend selection | `VaachakDisplayNativeRefreshShellWithPulpExecutor` |
| Low-level refresh executor | `vendor/pulp-os imported runtime` |
| Active backend name | `PulpCompatibility` |

## What moved

The following display behavior is now represented by Vaachak-owned command/intent types:

- `FullRefresh`
- `PartialRefresh`
- `ClearFrame`
- `SleepFrame`
- `RenderFrameMetadata`

These are mapped to Pulp-compatible execution intents by `VaachakDisplayBackendNativeRefreshShell`.

## What did not move

This slice intentionally does not rewrite or move:

- SSD1677 draw buffer logic
- SSD1677 full-refresh algorithm
- SSD1677 partial-refresh algorithm
- waveform handling
- BUSY wait behavior
- physical SPI transfer
- chip-select GPIO toggling
- display rendering algorithms
- reader/file-browser UX
- app navigation behavior
- storage behavior
- input behavior

## Backend takeover integration

The existing hardware backend takeover path calls the native display refresh shell before routing to the Pulp-compatible display backend:

- `execute_display_full_refresh_handoff()` calls `VaachakDisplayBackendNativeRefreshShell::execute_full_refresh_handoff()`
- `execute_display_partial_refresh_handoff()` calls `VaachakDisplayBackendNativeRefreshShell::execute_partial_refresh_handoff()`

If the shell report is not safe, the route falls back to the existing `PulpCompatibility` backend call. The current expected path is behavior-preserving and should not change hardware output.

## Cleanup checkpoint

`display_backend_native_refresh_shell_cleanup=ok` is the final acceptance checkpoint for the display refresh shell. It verifies that refresh command normalization and intent mapping remain Vaachak-owned while SSD1677 algorithms, physical SPI transfer, chip-select behavior, storage, input, reader/file-browser UX, and app navigation behavior remain unchanged.

## Validation marker

```text
display_backend_native_refresh_shell=ok
```

## Hardware smoke expectation

After flashing, display behavior should look unchanged:

- normal boot display
- normal Home/category dashboard
- normal full refresh behavior
- normal partial refresh behavior
- no blank/stuck refresh regression
- no SD or input regression

## Display native refresh command executor

`display_backend_native_refresh_command_executor` moves refresh command selection behavior into the Vaachak `target-xteink-x4` layer. The selected low-level executor remains `PulpCompatibility`; SSD1677 draw, waveform, BUSY wait, physical SPI transfer, and chip-select behavior remain in the imported Pulp-compatible runtime.

## Display native refresh command executor cleanup checkpoint

The accepted `display_backend_native_refresh_command_executor_cleanup` checkpoint verifies that `VaachakDisplayRefreshCommandExecutorWithPulpExecutor` remains selected, `PulpCompatibility` remains the low-level SSD1677 executor fallback, the rustfmt repair has been folded into the main checkpoint, and no SSD1677 draw/full/partial refresh algorithm, waveform, BUSY wait, physical SPI transfer, chip-select, storage, input, reader/file-browser UX, or app-navigation behavior moved.

