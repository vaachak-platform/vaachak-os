# Storage Physical SD/MMC Full Migration

`storage_physical_sd_mmc_full_migration` moves SD/MMC physical-driver ownership from the imported Pulp runtime into the Vaachak `target-xteink-x4` layer.

## Accepted marker

```text
storage_physical_sd_mmc_full_migration=ok
```

## Active backend

| Layer | Value |
|---|---|
| Active storage physical backend | `VaachakNativeSdMmcPhysicalDriver` |
| Backend owner | `target-xteink-x4 Vaachak layer` |
| Transport backend | `VaachakNativeSpiPhysicalDriver` |
| Pulp SD/MMC fallback | `false` |
| Imported Pulp SD/MMC runtime active | `false` |
| FAT algorithm migration | deferred to a later dedicated migration |

## Moved into Vaachak

- SD/MMC card lifecycle sequencing
- card present / media-state interpretation
- probe and init command policy
- mount lifecycle policy
- storage availability state
- block-device read/write request construction
- SD chip-select and storage transaction use through the Vaachak native SPI driver

## Still intentionally outside this slice

- FAT algorithm migration is not part of this slice.
- Reader/file-browser UX is unchanged.
- App navigation behavior is unchanged.
- Display behavior is unchanged.
- Input behavior is unchanged.
- Target HAL pin/SPI peripheral calls remain the hardware boundary.

## Hardware validation

After flashing, validate carefully:

```text
- boots normally
- SD card initializes/probes/mounts
- file browser opens
- SD file listing works
- long filename/title mapping still works
- TXT files open
- EPUB files open
- Back navigation works
- display refresh unchanged
- buttons unchanged
- no new FAT/path errors
```
