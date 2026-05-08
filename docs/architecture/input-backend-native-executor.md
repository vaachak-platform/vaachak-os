# Input Backend Native Executor

`input_backend_native_executor` is the first native backend migration after the hardware runtime backend takeover bridge.

## What moved into Vaachak

Vaachak now owns the native input backend executor shell for:

- button event normalization
- input intent mapping
- scan handoff pre-routing
- navigation handoff pre-routing

The selected native input backend is:

```text
VaachakInputNativeWithPulpSampling
```

## What remains Pulp-compatible

The current low-level input executor remains active underneath:

```text
PulpCompatibility
```

The following behavior is intentionally not moved in this deliverable:

- physical ADC ladder sampling
- GPIO polling
- button debounce/repeat execution
- navigation dispatch behavior
- app navigation behavior

## Behavior preservation

This deliverable does not change display behavior, storage behavior, SPI transfer or chip-select behavior, reader/file-browser UX, or app navigation behavior.

## Backend takeover relationship

The accepted backend takeover bridge remains the callable hardware backend boundary. This deliverable adds a Vaachak-owned input-native pre-handoff before existing input scan/navigation handoffs continue through the Pulp-compatible backend.

```text
Vaachak live handoff
  -> Vaachak hardware backend takeover
    -> VaachakInputNativeWithPulpSampling
      -> PulpCompatibility physical sampling fallback
```

## Acceptance marker

```text
input_backend_native_executor=ok
```
