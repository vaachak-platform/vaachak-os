# Display Backend Native Refresh Command Executor Cleanup

Status: `display_backend_native_refresh_command_executor_cleanup=ok`

This checkpoint finalizes the accepted `display_backend_native_refresh_command_executor` behavior migration and folds in the rustfmt repair that converted the display refresh shell dead-code allowance from inner-attribute style to rustfmt-safe outer-attribute style.

## What is accepted

Vaachak owns display refresh command behavior for:

- refresh command selection
- full refresh command handoff construction
- partial refresh command handoff construction
- partial-to-full escalation policy
- clear/sleep/render command classification
- Vaachak-owned display request construction

The selected native display command backend remains:

```text
VaachakDisplayRefreshCommandExecutorWithPulpExecutor
```

The low-level refresh executor remains:

```text
PulpCompatibility
```

## Active backend relationship

```text
Vaachak hardware backend takeover
  -> VaachakDisplayNativeRefreshShellWithPulpExecutor
    -> VaachakDisplayRefreshCommandExecutorWithPulpExecutor
      -> PulpCompatibility SSD1677 executor fallback
```

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
display_backend_native_refresh_command_executor
display_backend_native_refresh_command_executor.zip
display_backend_native_refresh_command_executor_fmt_fix
display_backend_native_refresh_command_executor_fmt_fix.zip
```

It also removes the temporary repository validator:

```text
scripts/validate_display_backend_native_refresh_command_executor_fmt_fix.sh
```

The canonical validators after this checkpoint are:

```text
scripts/validate_display_backend_native_refresh_command_executor.sh
scripts/validate_display_backend_native_refresh_command_executor_cleanup.sh
```

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
./scripts/validate_input_backend_native_event_pipeline.sh
./scripts/validate_input_backend_native_event_pipeline_cleanup.sh
./scripts/validate_display_backend_native_refresh_command_executor.sh
./scripts/validate_display_backend_native_refresh_command_executor_cleanup.sh
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
input_backend_native_event_pipeline=ok
input_backend_native_event_pipeline_cleanup=ok
display_backend_native_refresh_command_executor=ok
display_backend_native_refresh_command_executor_cleanup=ok
```

## Hardware smoke expectation

After flashing, display behavior should look unchanged:

- normal boot display
- normal Home/category dashboard
- normal full refresh behavior
- normal partial/list refresh behavior
- no blank/stuck refresh regression
- no SD/input regression
