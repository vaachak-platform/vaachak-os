# Phase 39M — Safe Scaffolding Archive Patch

Phase 39M archives only the Phase 39L `REVIEW ARCHIVE CANDIDATES`.

It does not archive or delete:

```text
vendor/pulp-os/src/apps/reader/mod.rs
vendor/pulp-os/src/apps/reader/typed_state_wiring.rs
target-xteink-x4/src/vaachak_x4/runtime/state_io_active_reader_save_callsite_wiring.rs
target-xteink-x4/src/vaachak_x4/runtime/state_io_runtime_state_write_verification*.rs
target-xteink-x4/src/vaachak_x4/runtime/state_io_write_lane_cleanup_acceptance_freeze*.rs
target-xteink-x4/src/vaachak_x4/runtime/state_io_post_freeze_scaffolding_cleanup_plan*.rs
```

It also leaves the Phase 39L `REVIEW DELETE LATER CANDIDATES` in place for one
more build/device regression cycle.

Archived files move to:

```text
docs/archive/phase38-39-scaffolding/runtime/
```

Before any archive action, the script runs the accepted-path guard.
