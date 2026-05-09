# Pulp Hardware Dead Path Quarantine

This checkpoint follows `pulp_hardware_reference_deprecation_audit` after the full Vaachak hardware migration.

The goal is to quarantine remaining classified dead Pulp hardware references without deleting `vendor/pulp-os` and without changing app, reader/file-browser, display, input, SPI, SD/MMC, or FAT behavior.

## Current state

Vaachak-native hardware backends are selected for:

- `VaachakNativeSpiPhysicalDriver`
- `VaachakNativeSsd1677PhysicalDriver`
- `VaachakNativeSdMmcPhysicalDriver`
- `VaachakNativeFatAlgorithmDriver`
- `VaachakPhysicalSamplingWithPulpAdcGpioReadFallback`

`vendor/pulp-os` remains present. vendor/pulp-os is not removed in this checkpoint because non-hardware runtime/import dependencies may still exist.

## Quarantine classifications

| Pulp reference class | Quarantine action |
| --- | --- |
| `StillRequiredRuntimeDependency` | Keep. Do not remove yet. |
| `CompatibilityImportBoundary` | Keep. Not an active hardware executor path. |
| `DeadLegacyHardwarePath` | `QuarantineDeadLegacyHardwarePath`: quarantine as inactive and eligible for later removal. |
| `DocumentationOnlyReference` | Keep only where documenting migration history. |
| `SafeToRemoveOverlayScaffoldArtifact` | Safe to remove as generated artifacts, not repo source. |

## Guardrails

This deliverable does not:

- remove `vendor/pulp-os`
- remove imported Pulp reader/runtime compatibility boundaries
- change app behavior
- change reader/file-browser UX
- change display behavior
- change input behavior
- change storage behavior
- change SPI behavior
- reactivate any Pulp hardware fallback

## Acceptance

`pulp_hardware_dead_path_quarantine=ok` means:

- the deprecation audit still passes
- full Vaachak physical migration consolidation still passes
- dead legacy Pulp hardware references are quarantined
- quarantined Pulp hardware paths are runtime-inactive
- no unclassified Pulp hardware path is active
- `vendor/pulp-os` is still present
