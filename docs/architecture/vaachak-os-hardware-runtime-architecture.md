# Vaachak OS Hardware Runtime Architecture

## Status

This document is the canonical architecture summary for the Vaachak OS Xteink X4 hardware runtime after the full hardware migration sequence.

Accepted final marker:

```text
vaachak_hardware_runtime_final_acceptance=ok
```

## Goal

The goal of the hardware migration was to move active hardware ownership out of Pulp OS and into the Vaachak-owned `target-xteink-x4` layer, while preserving the working reader/app behavior.

The migration is complete for the hardware stack:

```text
SPI
SSD1677 display
SD/MMC physical storage
FAT/filesystem algorithms
input physical sampling
```

## Ownership model

| Layer | Vaachak-owned responsibility | Active backend | Pulp role |
|---|---|---|---|
| Hardware runtime acceptance | Final native hardware acceptance gate | `VaachakHardwareRuntimeFinalAcceptance` | None for hardware |
| SPI physical driver | Bus identity, pins, transaction lifecycle, display/SD routing, transfer request/result construction | `VaachakNativeSpiPhysicalDriver` | Disabled |
| SSD1677 display driver | Command sequencing, full/partial/clear/sleep lifecycle, RAM-window state, BUSY/reset/DC policy | `VaachakNativeSsd1677PhysicalDriver` | Disabled |
| SD/MMC physical driver | Card lifecycle, media-state interpretation, probe/init/mount policy, block request construction | `VaachakNativeSdMmcPhysicalDriver` | Disabled |
| FAT algorithm driver | FAT/path/list/open/read/write algorithm ownership and operation classification | `VaachakNativeFatAlgorithmDriver` | Disabled |
| Input physical sampling | ADC ladder sample interpretation, oversample reduction, GPIO3 power-button interpretation | `VaachakPhysicalSamplingWithPulpAdcGpioReadFallback` | limited low-level physical sampling fallback only |

## Native behavior moved into Vaachak

The behavior migration completed before the physical-driver migration:

```text
- input event normalization and press/release/repeat classification
- button-to-navigation intent mapping
- display refresh command selection and escalation policy
- SD/MMC/FAT command decision behavior
- destructive FAT operation denial before handoff
```

## Physical drivers moved into Vaachak

The lower-level physical ownership migration is accepted for:

```text
- native SPI transaction lifecycle
- native SSD1677 display lifecycle policy
- native SD/MMC card lifecycle policy
- native FAT/filesystem algorithm ownership
- native input physical sample interpretation
```

## Remaining boundary

The remaining non-Vaachak boundary is the unavoidable target HAL/peripheral call surface:

```text
- ESP32-C3 SPI peripheral byte clocking
- ESP32-C3 ADC peripheral reads
- ESP32-C3 GPIO pin reads/writes
```

Those are not Pulp ownership. They are target hardware/HAL boundaries.

## Pulp OS status

`vendor/pulp-os` remains present but no longer owns active Vaachak hardware execution.

Allowed remaining roles:

```text
- non-hardware compatibility/import surfaces
- imported reader/runtime compatibility surfaces that have not been separately retired
- documentation-only references
```

Disallowed hardware roles:

```text
- active SPI hardware fallback
- active SSD1677 display fallback
- active SD/MMC physical fallback
- active FAT algorithm fallback
- active unclassified input hardware fallback
```

## Validation gates

The canonical final validation set is:

```bash
cargo fmt --all
./scripts/validate_vaachak_hardware_runtime_final_acceptance.sh
./scripts/validate_hardware_physical_full_migration_consolidation.sh
./scripts/validate_hardware_physical_full_migration_cleanup.sh
./scripts/validate_pulp_hardware_reference_deprecation_audit.sh
./scripts/validate_pulp_hardware_dead_path_quarantine.sh
./scripts/validate_pulp_hardware_dead_path_removal.sh
./scripts/validate_vendor_pulp_os_scope_reduction.sh
./scripts/validate_docs_and_artifact_cleanup.sh
cargo build
```

## Hardware smoke expectations

The firmware must still satisfy:

```text
- boot reaches Home/category dashboard
- display full and partial refresh work
- input mapping remains unchanged
- SD card initializes
- SD root listing works
- TXT and EPUB open paths work
- progress/state/cache files still work
- Back navigation works
- no new FAT/path/cluster-chain errors
```
