# Current Runtime State

## Status

Vaachak-native X4 hardware runtime is accepted.

```text
vaachak_hardware_runtime_final_acceptance=ok
hardware_physical_full_migration_consolidation=ok
vendor_pulp_os_scope_reduction=ok
```

Pulp OS is not the active hardware runtime. The active hardware ownership lives in Vaachak target modules.

## Active hardware ownership

| Surface | Active owner |
| --- | --- |
| SPI physical driver | `VaachakNativeSpiPhysicalDriver` |
| SSD1677 display driver | `VaachakNativeSsd1677PhysicalDriver` |
| SD/MMC physical driver | `VaachakNativeSdMmcPhysicalDriver` |
| FAT/filesystem algorithm driver | `VaachakNativeFatAlgorithmDriver` |
| Input sampling and event pipeline | Vaachak-native input physical sampling and event pipeline |

## Retained Pulp scope

`vendor/pulp-os` remains present only for scoped non-hardware compatibility, import, reference, or historical comparison surfaces. It should not be described as the active SPI, display, SD/MMC, FAT, or input runtime.

## Current product work

The next product work is Reader Home, Continue Reading, reader data model freeze, XTC compatibility, `.vchk`, and sync alignment.
