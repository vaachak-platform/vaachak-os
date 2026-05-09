# Pulp OS Scope After Hardware Migration

## Status

`vendor/pulp-os` remains in the repository, but its scope has been reduced after the Vaachak-native hardware migration.

## Allowed scope

The allowed remaining scope for Pulp references is:

```text
- non-hardware compatibility/import surfaces
- imported reader/runtime compatibility surfaces
- documentation-only historical references
- migration notes that explain previous ownership
```

## Disallowed scope

Pulp must not be treated as the active owner for:

```text
- SPI physical runtime
- SSD1677 display runtime
- SD/MMC physical runtime
- FAT/filesystem algorithm runtime
- input physical sampling interpretation
```

## Accepted checkpoints

The Pulp deprecation path is accepted through:

```text
pulp_hardware_reference_deprecation_audit=ok
pulp_hardware_dead_path_quarantine=ok
pulp_hardware_dead_path_removal=ok
vendor_pulp_os_scope_reduction=ok
```

## Removal policy

Do not delete `vendor/pulp-os` wholesale until a separate non-hardware dependency audit proves that imported reader/runtime compatibility is no longer needed.

Safe cleanup now includes:

```text
- generated overlay folders
- generated overlay zip files
- temporary validator-fix scripts
- temporary patch scripts
- accidental __pycache__ folders
```

Unsafe cleanup now includes:

```text
- deleting vendor/pulp-os wholesale
- deleting imported runtime boundaries without a separate audit
- deleting reader/file-browser paths
- deleting state/cache compatibility code
```
