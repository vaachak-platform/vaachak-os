# Display Backend Native Refresh Command Executor

`display_backend_native_refresh_command_executor` is the first display behavior move beyond the native refresh shell.

## Ownership moved into Vaachak

Vaachak now owns the refresh command executor behavior in `target-xteink-x4`:

- refresh command selection
- full refresh command execution handoff construction
- partial refresh command execution handoff construction
- unsafe partial-refresh escalation to full-refresh command
- clear / sleep / render-metadata command classification
- Vaachak-owned display request construction for the backend interface

## Still Pulp-compatible

The low-level executor remains `PulpCompatibility` through the imported Pulp runtime.

The following are intentionally not moved in this slice:

- SSD1677 draw buffer algorithm
- SSD1677 waveform handling
- SSD1677 BUSY wait behavior
- physical SPI transfer
- chip-select GPIO toggling
- reader/file-browser UX
- app navigation behavior
- storage or input behavior

## Runtime relationship

The previous shell remains the normalization layer:

- `display_backend_native_refresh_shell` maps external refresh intent into a Vaachak command.

The new command executor is the behavior layer:

- `display_backend_native_refresh_command_executor` selects the actual refresh command, including partial-to-full escalation policy.

The active low-level execution path remains:

- `PulpCompatibility`
- `vendor/pulp-os imported runtime`

## Acceptance

Expected validator marker:

```text
 display_backend_native_refresh_command_executor=ok
```

Expected hardware behavior:

- full refresh looks unchanged
- partial refresh/list movement looks unchanged
- unsafe partial refresh command escalates to full-refresh handoff metadata only
- no blank/stuck display
- no storage/input regression
