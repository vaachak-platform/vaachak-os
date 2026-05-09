# Vaachak OS Hardware Runtime Architecture

## Accepted hardware state

```text
vaachak_hardware_runtime_final_acceptance=ok
hardware_physical_full_migration_consolidation=ok
```

Vaachak owns the X4 hardware runtime. Pulp hardware fallbacks are disabled from Vaachak integration.

| Surface | Active backend | Transport / dependency |
| --- | --- | --- |
| SPI physical driver | `VaachakNativeSpiPhysicalDriver` | ESP32-C3 target HAL boundary |
| SSD1677 display driver | `VaachakNativeSsd1677PhysicalDriver` | `VaachakNativeSpiPhysicalDriver` |
| SD/MMC physical driver | `VaachakNativeSdMmcPhysicalDriver` | `VaachakNativeSpiPhysicalDriver` |
| FAT algorithm driver | `VaachakNativeFatAlgorithmDriver` | `VaachakNativeSdMmcPhysicalDriver` |
| Input sampling | Vaachak native physical sampling driver | target HAL ADC/GPIO boundary |

## Pulp hardware status

Pulp hardware references have been audited, quarantined, and removed/disabled from Vaachak integration. `vendor/pulp-os` remains present only for scoped non-hardware compatibility/import/reference surfaces.

## Validation

```bash
cargo fmt --all
./scripts/validate_vaachak_hardware_runtime_final_acceptance.sh
./scripts/validate_hardware_physical_full_migration_consolidation.sh
./scripts/validate_pulp_hardware_reference_deprecation_audit.sh
./scripts/validate_pulp_hardware_dead_path_quarantine.sh
./scripts/validate_pulp_hardware_dead_path_removal.sh
./scripts/validate_vendor_pulp_os_scope_reduction.sh
cargo build
```
