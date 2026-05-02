# Phase 35D-2 - Boot Preflight Allocation Guard

Phase 35D-2 records and checks the boot-order rule learned from hardware validation:

- pre-heap runtime preflight must be allocation-free
- allocation-using reader-state preflight must run only after `esp_alloc::heap_allocator!`
- active persistence remains owned by the imported Pulp runtime
- button/input runtime remains on the direct imported Pulp path

The current split is:

- `VaachakStorageStateRuntimeBridge::active_runtime_preflight()` runs before heap setup and must stay allocation-free.
- `VaachakStorageStateRuntimeBridge::active_runtime_alloc_preflight()` runs after heap setup and may reach `VaachakReaderStateRuntimeBridge`.

Normal boot remains:

```text
vaachak=x4-runtime-ready
```

No phase marker is printed during normal boot.
