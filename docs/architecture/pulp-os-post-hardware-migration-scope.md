# Pulp OS Post-Hardware-Migration Scope

## Status

`vendor/pulp-os` remains present, but Pulp OS is not the active hardware owner.

Accepted gates:

```text
pulp_hardware_reference_deprecation_audit=ok
pulp_hardware_dead_path_quarantine=ok
pulp_hardware_dead_path_removal=ok
vendor_pulp_os_scope_reduction=ok
```

## Allowed retained scope

Allowed remaining uses of `vendor/pulp-os`:

- non-hardware compatibility/import surfaces
- historical reference while Vaachak reader features stabilize
- comparison source for behavior parity where explicitly documented
- temporary non-hardware scaffolding that is tracked for later removal

## Disallowed scope

Do not reintroduce Pulp ownership for:

- SPI physical runtime
- SSD1677 display runtime
- SD/MMC physical runtime
- FAT/filesystem algorithm runtime
- input sampling / event interpretation runtime

## Rule

Any new Pulp reference must be classified before merge. Unclassified Pulp hardware fallback is not allowed.
