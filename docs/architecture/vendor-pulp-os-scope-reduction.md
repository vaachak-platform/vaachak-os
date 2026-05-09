# Vendor Pulp OS Scope Reduction

`vendor_pulp_os_scope_reduction=ok`

This checkpoint narrows the role of `vendor/pulp-os` after the accepted Vaachak-native hardware migration.

## Intent

`vendor/pulp-os` remains present, but its allowed role is now reduced to non-hardware compatibility and historical reference surfaces. It is not the active hardware implementation owner for SPI, display, storage, FAT, or input sampling.

`vendor/pulp-os remains present`. This deliverable does not delete `vendor/pulp-os` because the current firmware can still depend on non-hardware imported runtime boundaries while the reader/runtime migration is completed separately.

## Retained vendor scope

| Surface | Classification | Reason |
|---|---|---|
| Imported Pulp reader runtime boundary | compatibility/import boundary | kept until reader/runtime migration is complete |
| Architecture documentation | documentation-only reference | kept for migration history and deprecation state |
| Vendor tree | non-hardware runtime dependency | retained until non-hardware dependencies are audited |

## Excluded from vendor scope

| Hardware surface | Vaachak owner |
|---|---|
| SPI hardware runtime | `VaachakNativeSpiPhysicalDriver` |
| SSD1677 display hardware runtime | `VaachakNativeSsd1677PhysicalDriver` |
| SD/MMC hardware runtime | `VaachakNativeSdMmcPhysicalDriver` |
| FAT/filesystem algorithm runtime | `VaachakNativeFatAlgorithmDriver` |
| Input physical sampling interpretation | `VaachakPhysicalSamplingWithPulpAdcGpioReadFallback` |
| Dead legacy hardware fallback paths | quarantined and removed from Vaachak integration metadata |
| Generated overlay/scaffold artifacts | safe to remove through cleanup scripts |

## Guardrails

This checkpoint does not change:

- app behavior
- reader/file-browser UX
- display behavior
- input behavior
- storage behavior
- SPI behavior
- `vendor/pulp-os` contents

## Acceptance

Run:

```bash
cargo fmt --all
./scripts/validate_pulp_hardware_reference_deprecation_audit.sh
./scripts/validate_pulp_hardware_dead_path_quarantine.sh
./scripts/validate_pulp_hardware_dead_path_removal.sh
./scripts/validate_vendor_pulp_os_scope_reduction.sh
cargo build
```

Expected:

```text
vendor_pulp_os_scope_reduction=ok
```
