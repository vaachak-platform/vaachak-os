# Input Backend Native Executor

This checkpoint introduced the Vaachak-native input backend shell for button event normalization and input intent mapping while keeping Pulp-compatible physical sampling available.

The accepted cleanup checkpoint remains:

```text
docs/architecture/input-backend-native-executor-cleanup.md
```

The next accepted behavior move is documented in:

```text
docs/architecture/input-backend-native-event-pipeline.md
```

## Current native input layers

- `input_backend_native_executor`: Vaachak-owned native input executor shell.
- `input_backend_native_executor_cleanup`: cleanup checkpoint for the native executor shell.
- `input_backend_native_event_pipeline`: actual Vaachak-owned input event behavior for normalization, stable state tracking, press/release/repeat classification, and navigation intent mapping.

## Active selection

```text
VaachakInputNativeWithPulpSampling
PulpCompatibility
```

## Still Pulp-compatible

The following remain Pulp-compatible:

- physical ADC ladder sampling
- GPIO polling
- physical hardware access
- final app navigation dispatch

## Behavior guardrails

This stack must not change display, storage, SPI, reader/file-browser UX, or app navigation screens.
