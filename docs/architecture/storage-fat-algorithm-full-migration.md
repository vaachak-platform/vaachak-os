# Storage FAT Algorithm Full Migration

`storage_fat_algorithm_full_migration` moves FAT/path/list/open/read/write algorithm ownership from imported Pulp FAT behavior into the Vaachak `target-xteink-x4` layer.

## Accepted marker

```text
storage_fat_algorithm_full_migration=ok
```

## Active backend

| Layer | Value |
|---|---|
| Active FAT algorithm backend | `VaachakNativeFatAlgorithmDriver` |
| Backend owner | `target-xteink-x4 Vaachak layer` |
| Block-device backend | `VaachakNativeSdMmcPhysicalDriver` |
| Pulp FAT fallback | `false` |
| Imported Pulp FAT runtime active | `false` |

## Moved fully into Vaachak

- path normalization and path-role policy
- BPB / boot-sector parsing policy
- directory entry decoding
- long filename assembly policy
- FAT table / cluster-chain traversal policy
- file open/read/write request construction
- metadata update policy
- create/delete/rename authorization policy
- FAT operation result classification

## Still outside this boundary

- target HAL block I/O calls
- accepted Vaachak native SD/MMC physical block driver transport
- display behavior
- input behavior
- SPI behavior
- reader/file-browser UX
- app navigation screens

## Hardware validation

After flashing, validate carefully:

```text
- boots normally
- SD card initializes/mounts
- file browser opens
- SD file listing works
- long filename/title mapping still works
- TXT files open
- EPUB files open
- reader state/cache writes still behave as before
- Back navigation works
- display refresh unchanged
- buttons unchanged
- no new FAT/path errors
```
