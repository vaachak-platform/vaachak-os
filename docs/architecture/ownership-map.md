# Ownership Map

## Hardware ownership

| Area | Vaachak owner | Pulp hardware fallback |
| --- | --- | --- |
| SPI | `VaachakNativeSpiPhysicalDriver` | Disabled |
| SSD1677 display | `VaachakNativeSsd1677PhysicalDriver` | Disabled |
| SD/MMC physical storage | `VaachakNativeSdMmcPhysicalDriver` | Disabled |
| FAT/filesystem algorithms | `VaachakNativeFatAlgorithmDriver` | Disabled |
| Input physical sampling | Vaachak native input sampling driver | Disabled for interpretation/classification; Pulp code remains only outside active hardware ownership |

## Runtime and product ownership

| Area | Current owner / direction |
| --- | --- |
| Reader Home | Next product work |
| Library index | Next product work |
| Reader state model | Must be frozen before XTC and `.vchk` work |
| XTC | Compatibility/open path, not native long-term package |
| `.vchk` | Long-term Vaachak-native package contract |
| Sync | Align after local state is stable |
| Waveshare/S3 | Future profile after X4 reader path is stable |
| Compatibility host | Future optional layer; not a v1 X4 milestone |

## Vendor scope

`vendor/pulp-os` is retained for non-hardware compatibility/import/reference scope. Any future use must be classified before it becomes product-critical.
