# Pulp Hardware Reference Deprecation Audit

This checkpoint audits remaining Pulp references after the Vaachak-native hardware migration stack has been accepted.

It is intentionally non-destructive:

- vendor/pulp-os is not removed.
- App behavior is unchanged.
- Reader/file-browser UX is unchanged.
- Display/input/storage behavior is unchanged.
- The audit classifies references before quarantine or removal.

## Required native hardware state

The audit requires the full hardware migration consolidation to remain accepted:

| Domain | Required active backend |
|---|---|
| SPI | `VaachakNativeSpiPhysicalDriver` |
| SSD1677 display | `VaachakNativeSsd1677PhysicalDriver` |
| SD/MMC physical storage | `VaachakNativeSdMmcPhysicalDriver` |
| FAT filesystem algorithm | `VaachakNativeFatAlgorithmDriver` |
| Input sampling | `VaachakPhysicalSamplingWithPulpAdcGpioReadFallback` |

For SPI, display, SD/MMC, and FAT, imported Pulp hardware runtime activity must remain disabled. Input physical sampling still keeps a Pulp-compatible ADC/GPIO read fallback until the final target HAL read executor is wired, but Vaachak owns sample interpretation and event-pipeline behavior.

## Classification model

Remaining Pulp references are classified into these buckets:

1. `StillRequiredRuntimeDependency`
   - Example: `vendor/pulp-os` itself.
   - Action: keep until non-hardware Pulp runtime dependencies are separated.

2. `CompatibilityImportBoundary`
   - Example: `target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs`.
   - Action: keep as a reader/runtime compatibility boundary until reader app migration.

3. `DeadLegacyHardwarePath`
   - Example: inactive historical fallback constants such as `PULP_*_FALLBACK_ENABLED` and `IMPORTED_PULP_*_RUNTIME_ACTIVE`.
   - Action: quarantine before removal. Active values must remain false.

4. `DocumentationOnlyReference`
   - Example: migration documentation under `docs/architecture`.
   - Action: retain only where documenting migration history and deprecation state.

5. `SafeToRemoveOverlayScaffoldArtifact`
   - Example: generated overlay folders/zips and validator-fix artifacts.
   - Action: remove generated artifacts only; never remove repo source/docs/scripts.

## Guardrails

This audit must not:

- delete `vendor/pulp-os`;
- remove imported reader/runtime compatibility code;
- change app behavior;
- change reader/file-browser UX;
- change display/input/storage behavior;
- delete source, docs, or validators that are now part of the repo.

## Acceptance

Run:

```bash
cargo fmt --all
./scripts/validate_pulp_hardware_reference_deprecation_audit.sh
cargo build
```

Expected marker:

```text
pulp_hardware_reference_deprecation_audit=ok
```
