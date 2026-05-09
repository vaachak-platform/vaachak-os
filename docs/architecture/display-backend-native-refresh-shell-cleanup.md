# Display Backend Native Refresh Shell Cleanup

Status: `display_backend_native_refresh_shell_cleanup=ok`

This checkpoint finalizes the accepted `display_backend_native_refresh_shell` work before moving to deeper display-native or storage-native migration.

## What is accepted

Vaachak owns the display refresh command shell for:

- display refresh command normalization
- full refresh intent mapping
- partial refresh intent mapping
- clear/sleep/render metadata intent mapping
- refresh handoff pre-routing

The selected display-native backend remains:

```text
VaachakDisplayNativeRefreshShellWithPulpExecutor
```

The low-level refresh executor remains:

```text
PulpCompatibility
```

## Active backend relationship

```text
Vaachak hardware backend takeover
  -> VaachakDisplayNativeRefreshShellWithPulpExecutor
    -> PulpCompatibility SSD1677 refresh executor fallback
```

The cleanup verifies that the accepted backend takeover bridge, backend takeover cleanup, input-native cleanup, and display-native refresh shell checkpoints remain valid.

## Behavior preservation

This cleanup does not rewrite or move:

- SSD1677 draw buffer logic
- SSD1677 full refresh algorithm
- SSD1677 partial refresh algorithm
- waveform handling
- BUSY wait behavior
- physical SPI transfer
- chip-select GPIO toggling
- storage behavior
- input physical sampling or navigation behavior
- reader/file-browser UX
- app navigation behavior

## Cleanup behavior

The cleanup script removes old overlay artifacts such as:

```text
display_backend_native_refresh_shell
display_backend_native_refresh_shell.zip
display_backend_native_refresh_shell_validator_fix*
```

It does not remove repository source files, docs, or validators.

## Validation

Run:

```bash
cargo fmt --all
./scripts/validate_hardware_runtime_backend_takeover_bridge.sh
./scripts/validate_hardware_runtime_backend_takeover_cleanup.sh
./scripts/validate_input_backend_native_executor.sh
./scripts/validate_input_backend_native_executor_cleanup.sh
./scripts/validate_display_backend_native_refresh_shell.sh
./scripts/validate_display_backend_native_refresh_shell_cleanup.sh
cargo build
```

Expected markers:

```text
hardware_runtime_backend_takeover_bridge=ok
hardware_runtime_backend_takeover_cleanup=ok
input_backend_native_executor=ok
input_backend_native_executor_cleanup=ok
display_backend_native_refresh_shell=ok
display_backend_native_refresh_shell_cleanup=ok
```

## Next recommended slice

After hardware smoke confirms this checkpoint, the next native migration can be a deeper display refresh command bridge or the first storage-native lifecycle slice. Storage remains higher-risk because it can affect SD mount and FAT behavior.

## Display native refresh command executor

`display_backend_native_refresh_command_executor` moves refresh command selection behavior into the Vaachak `target-xteink-x4` layer. The selected low-level executor remains `PulpCompatibility`; SSD1677 draw, waveform, BUSY wait, physical SPI transfer, and chip-select behavior remain in the imported Pulp-compatible runtime.

## Display native refresh command executor cleanup checkpoint

The accepted `display_backend_native_refresh_command_executor_cleanup` checkpoint verifies that `VaachakDisplayRefreshCommandExecutorWithPulpExecutor` remains selected, `PulpCompatibility` remains the low-level SSD1677 executor fallback, the rustfmt repair has been folded into the main checkpoint, and no SSD1677 draw/full/partial refresh algorithm, waveform, BUSY wait, physical SPI transfer, chip-select, storage, input, reader/file-browser UX, or app-navigation behavior moved.

