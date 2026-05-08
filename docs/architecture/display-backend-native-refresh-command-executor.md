# Display Backend Native Refresh Command Executor

`display_backend_native_refresh_command_executor` is the second native display backend migration slice after the native display refresh shell.

Cleanup checkpoint: [`display-backend-native-refresh-command-executor-cleanup.md`](display-backend-native-refresh-command-executor-cleanup.md)

## Goal

Move actual display refresh command selection behavior into the Vaachak `target-xteink-x4` layer while keeping the active SSD1677/e-paper execution path Pulp-compatible.

Vaachak now owns:

- refresh command selection behavior
- full refresh command handoff construction
- partial refresh command handoff construction
- partial-to-full escalation policy
- clear/sleep/render command classification
- Vaachak-owned display request construction

The active low-level backend remains `PulpCompatibility`.

## Active backend

| Area | Owner |
| --- | --- |
| Refresh command executor | `target-xteink-x4 Vaachak layer` |
| Backend selection | `VaachakDisplayRefreshCommandExecutorWithPulpExecutor` |
| Low-level SSD1677 executor | `vendor/pulp-os imported runtime` |
| Active backend name | `PulpCompatibility` |

## What moved

The following display command behavior is now Vaachak-owned:

- `FullRefreshRequested`
- `PartialRefreshRequested`
- `PartialRefreshUnsafeEscalatedToFull`
- `ClearRequested`
- `SleepRequested`
- `RenderMetadataRequested`

Unsafe partial refresh requests are escalated to full refresh in the Vaachak layer before being handed to the Pulp-compatible executor.

## What did not move

This slice intentionally does not rewrite or move:

- SSD1677 draw buffer algorithm
- SSD1677 waveform handling
- SSD1677 BUSY wait behavior
- physical SPI transfer
- chip-select GPIO toggling
- reader/file-browser UX
- app navigation behavior
- storage behavior
- input behavior

## Cleanup checkpoint

`display_backend_native_refresh_command_executor_cleanup=ok` is the final acceptance checkpoint for the display refresh command executor. It verifies that command selection and escalation remain Vaachak-owned, that the rustfmt repair has been folded into the checkpoint, and that SSD1677 algorithms, physical SPI transfer, chip-select behavior, storage, input, reader/file-browser UX, and app navigation behavior remain unchanged.

## Validation marker

```text
display_backend_native_refresh_command_executor=ok
```

## Hardware smoke expectation

After flashing, display behavior should look unchanged:

- normal boot display
- normal Home/category dashboard
- normal full refresh behavior
- normal partial/list refresh behavior
- no blank/stuck refresh regression
- no SD or input regression
