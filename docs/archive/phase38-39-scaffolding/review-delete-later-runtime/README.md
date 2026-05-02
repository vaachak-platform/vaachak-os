# Phase 39O Review-Delete-Later Runtime Archive

These files were moved out of the runtime build surface by Phase 39O after
Phase 39N dry-run acceptance.

Accepted active write path remains:

```text
vendor/pulp-os/src/apps/reader/mod.rs
  -> vendor/pulp-os/src/apps/reader/typed_state_wiring.rs
  -> KernelHandle
  -> _X4/state
  -> restore
```

These files were previously target-side adapter/facade experiments. The active
reader path uses the Pulp-local `typed_state_wiring.rs` facade.
