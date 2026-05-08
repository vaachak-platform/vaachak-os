# Input Backend Native Event Pipeline

`input_backend_native_event_pipeline` is the first Vaachak OS hardware backend slice that moves actual input event behavior out of the imported Pulp runtime while keeping physical sampling Pulp-compatible.

Cleanup checkpoint: [`input-backend-native-event-pipeline-cleanup.md`](input-backend-native-event-pipeline-cleanup.md)

## Ownership moved to Vaachak

Vaachak now owns the input event pipeline in `target-xteink-x4`:

- raw sampled button state normalization
- stable button state tracking
- debounce window metadata
- press event generation
- release event generation
- repeat event generation
- button-to-navigation intent mapping
- scan handoff pre-routing
- navigation handoff pre-routing

The active native event pipeline is:

```text
VaachakNativeEventPipelineWithPulpSampling
```

## Still Pulp-compatible

The following remain in the existing Pulp-compatible low-level path:

- physical ADC ladder sampling
- GPIO polling
- physical hardware access
- final app navigation dispatch

`PulpCompatibility` remains the low-level sampling fallback while Vaachak owns event classification and intent mapping.

## Behavior preservation

This deliverable intentionally does not change:

- display behavior
- storage behavior
- SPI transfer or chip-select behavior
- reader/file-browser UX
- app navigation screens
- SD/FAT behavior

## Runtime integration

The backend takeover/live handoff path routes input scan and navigation handoffs through:

```text
VaachakInputBackendNativeEventPipeline::execute_scan_pipeline()
VaachakInputBackendNativeEventPipeline::execute_navigation_pipeline()
```

The handoff then continues to the Pulp-compatible backend for physical sampling and low-level execution.

The cleanup checkpoint verifies that the takeover-fix integration is folded into the main accepted input pipeline state.

## Acceptance markers

```text
input_backend_native_event_pipeline=ok
input_backend_native_event_pipeline_cleanup=ok
```
