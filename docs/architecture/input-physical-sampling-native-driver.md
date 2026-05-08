# Input Physical Sampling Native Driver

Marker: `input_physical_sampling_native_driver=ok`

Canonical cleanup checkpoint: `docs/architecture/input-physical-sampling-native-driver-cleanup.md`

This deliverable moved the first lower-level input physical behavior into Vaachak while keeping physical hardware reads Pulp-compatible.

## Vaachak-owned behavior

- X4 ADC ladder sample interpretation
- 4-sample oversample reduction
- GPIO3 power-button low-active interpretation
- handoff to the Vaachak native input event pipeline

## Pulp-compatible fallback retained

- actual ADC peripheral reads
- actual GPIO polling
- physical hardware access timing
- final app navigation dispatch

Active driver: `VaachakPhysicalSamplingWithPulpAdcGpioReadFallback`

Fallback backend: `PulpCompatibility`

## Cleanup status

The cleanup checkpoint is accepted when:

```text
input_physical_sampling_native_driver_cleanup=ok
```
