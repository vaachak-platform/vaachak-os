# Hardware Physical Full Migration Cleanup

This is the final cleanup checkpoint for the Vaachak OS hardware physical migration stack on Xteink X4.

## Accepted native physical stack

The cleanup checkpoint assumes the following migrations are accepted and consolidated:

- SPI physical native driver
- SSD1677 display physical native driver
- SD/MMC physical native driver
- FAT algorithm native driver
- Input physical sampling native driver

## Cleanup scope

This deliverable removes old overlay zip files and extracted overlay folders from previous deliverables. It does not remove source files, docs, or validators that are now part of the repository.

The cleanup script removes only top-level generated overlay artifacts that contain both:

- `MANIFEST.txt`
- `README-APPLY.md`

Zip files are removed only when their contents include the generated overlay manifest/readme pair.

## Preserved behavior

No runtime behavior is changed by this cleanup checkpoint.

Preserved:

- reader/file-browser UX
- app navigation screens
- native SPI physical driver
- native SSD1677 physical driver
- native SD/MMC physical driver
- native FAT algorithm driver
- native input physical sampling driver

## Acceptance marker

```text
hardware_physical_full_migration_cleanup=ok
```
