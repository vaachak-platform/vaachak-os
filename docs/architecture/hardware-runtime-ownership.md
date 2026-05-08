# Hardware Runtime Ownership Consolidation

This is the canonical Vaachak OS hardware-runtime ownership map for the Xteink X4 target.

The purpose of this consolidation is to club the accepted ownership slices into one stable boundary before moving executor behavior out of the imported Pulp runtime.

## Consolidated owners

| Hardware area | Vaachak-owned authority | Active executor after this deliverable | Behavior moved now? |
| --- | --- | --- | --- |
| Shared SPI bus | `VaachakSpiBusRuntimeOwner` | `PulpCompatibility` / `vendor/pulp-os imported runtime` | No SPI transfer/arbitration executor moved |
| SD probe/mount lifecycle | `VaachakStorageProbeMountRuntimeOwner` | `PulpCompatibility` / `vendor/pulp-os imported runtime` | No card init, probe, or mount executor moved |
| SD/FAT read-only file access | `VaachakSdFatRuntimeReadonlyOwner` | `PulpCompatibility` / `vendor/pulp-os imported runtime` | No FAT executor moved; no writes introduced |
| Display runtime | `VaachakDisplayRuntimeOwner` | `PulpCompatibility` / `vendor/pulp-os imported runtime` | No SSD1677 draw/refresh executor moved |
| Input runtime | `VaachakInputRuntimeOwner` | `PulpCompatibility` / `vendor/pulp-os imported runtime` | No ADC scan/debounce/navigation executor moved |

## Canonical code entrypoint

```text
目标: target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_ownership.rs
marker: hardware_runtime_ownership_consolidation=ok
```

The consolidated entrypoint imports each accepted owner and reports whether the full hardware runtime ownership stack is coherent:

```text
VaachakHardwareRuntimeOwnership::consolidation_ok()
```

## Ownership state after this deliverable

Vaachak owns the runtime authority boundaries for:

```text
- shared SPI ownership metadata and device-user registration
- SD probe/mount lifecycle authority
- SD/FAT read-only runtime authority
- display runtime authority
- input runtime authority
```

Pulp remains the active executor for:

```text
- SPI transfer execution and chip-select toggling
- SD card initialization, probe, and mount execution
- FAT read/list/open behavior used by the current runtime
- SSD1677 draw/full-refresh/partial-refresh behavior
- button ADC sampling, debounce/repeat, and navigation dispatch
```

## Behavior guardrails

This consolidation does not change:

```text
- reader behavior
- file-browser behavior
- SD file listing behavior
- TXT/EPUB open paths
- display refresh behavior
- button navigation/debounce behavior
```

This consolidation does not add:

```text
- FAT write
- append
- delete
- rename
- mkdir
- format
- SD mount implementation
- SD probe implementation
- display draw implementation
- display refresh implementation
- input polling loop
- ADC sampling implementation
- navigation event routing implementation
```

## Dependency order

The hardware owner stack is intentionally ordered:

```text
1. SPI bus runtime owner
2. Storage probe/mount runtime owner
3. SD/FAT read-only runtime owner
4. Display runtime owner
5. Input runtime owner
6. Hardware runtime ownership consolidation
```

## Next migration step

After this consolidation passes host validation and hardware smoke, future deliverables can begin moving one executor at a time. The recommended first executor migration remains small and reversible:

```text
storage_probe_mount_executor_bridge
```

That future step would still avoid changing reader/file-browser behavior and should retain the Pulp compatibility backend until hardware smoke is clean.
