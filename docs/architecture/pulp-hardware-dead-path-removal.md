# Pulp Hardware Dead Path Removal

`pulp_hardware_dead_path_removal=ok`

This checkpoint removes or disables only the quarantined, proven-dead Pulp hardware references from Vaachak-owned integration paths after full native hardware migration.

## What is removed from active hardware integration

- Dead legacy Pulp hardware fallback routes.
- Inactive Pulp hardware runtime selection paths.
- Generated overlay/scaffold artifacts when present.

## What remains intentionally kept

- vendor/pulp-os remains present.
- `vendor/pulp-os` remains present.
- Imported Pulp reader/runtime compatibility boundaries remain present.
- Documentation-only references remain present for migration history.
- Non-hardware runtime dependencies are not removed by this checkpoint.

## Native Vaachak hardware state

The active hardware stack remains Vaachak-native:

- `VaachakNativeSpiPhysicalDriver`
- `VaachakNativeSsd1677PhysicalDriver`
- `VaachakNativeSdMmcPhysicalDriver`
- `VaachakNativeFatAlgorithmDriver`
- `VaachakPhysicalSamplingWithPulpAdcGpioReadFallback`

## Guardrails

This checkpoint does not change:

- app behavior
- reader/file-browser UX
- display behavior
- input behavior
- storage behavior
- SPI behavior
- `vendor/pulp-os` contents

## Next step

After this removal checkpoint passes, the safe next step is `vendor_pulp_os_scope_reduction`, which should reduce the vendor tree only after verifying which remaining non-hardware imported runtime pieces are still required.
