# Input Physical Sampling Native Driver Cleanup

Marker: `input_physical_sampling_native_driver_cleanup=ok`

This checkpoint finalizes the accepted Vaachak-native input physical sampling driver as a GitHub-ready migration slice.

## Accepted behavior move

Vaachak now owns these input physical-sampling behaviors in `target-xteink-x4`:

- X4 ADC ladder sample interpretation
- 4-sample oversample reduction
- GPIO3 low-active power-button level interpretation
- conversion into the accepted Vaachak native input event pipeline

## Preserved Pulp-compatible behavior

The cleanup checkpoint intentionally keeps these lower-level execution surfaces unchanged:

- actual ADC peripheral reads
- actual GPIO polling
- physical hardware access timing
- final app navigation dispatch
- display behavior
- storage behavior
- SPI transfer / chip-select behavior
- reader/file-browser UX
- app navigation screens

Active driver selection remains `VaachakPhysicalSamplingWithPulpAdcGpioReadFallback`, with physical read fallback `PulpCompatibility`.

## Validation

Run:

```bash
cargo fmt --all
./scripts/validate_hardware_runtime_backend_takeover_bridge.sh
./scripts/validate_hardware_runtime_backend_takeover_cleanup.sh
./scripts/validate_input_backend_native_executor.sh
./scripts/validate_input_backend_native_executor_cleanup.sh
./scripts/validate_input_backend_native_event_pipeline.sh
./scripts/validate_input_backend_native_event_pipeline_cleanup.sh
./scripts/validate_hardware_native_behavior_consolidation.sh
./scripts/validate_hardware_native_behavior_consolidation_cleanup.sh
./scripts/validate_physical_driver_migration_plan.sh
./scripts/validate_input_physical_sampling_native_driver.sh
./scripts/validate_input_physical_sampling_native_driver_cleanup.sh
cargo build
```

Expected:

```text
input_physical_sampling_native_driver=ok
input_physical_sampling_native_driver_cleanup=ok
```

## Hardware smoke

After flashing, validate:

- device boots normally
- Home/category dashboard appears
- all buttons respond
- up/down/select/back mapping remains unchanged
- no missed press
- no double press
- long press/repeat behavior remains equivalent
- power button behavior remains unchanged
- file browser opens
- SD files list
- TXT/EPUB still open
- display refresh remains unchanged
