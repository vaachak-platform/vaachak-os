# Phase 39O — Guarded Review-Delete-Later Removal Patch

Phase 39O removes the 14 Phase 39N review-delete-later candidates from the runtime
build surface.

It does not destroy files. It moves them to:

```text
docs/archive/phase38-39-scaffolding/review-delete-later-runtime/
```

Protected path:

```text
vendor/pulp-os/src/apps/reader/mod.rs
  -> vendor/pulp-os/src/apps/reader/typed_state_wiring.rs
  -> KernelHandle
  -> _X4/state
  -> restore
```

Before removal the script checks:

```text
accepted write path guard
external candidate references outside the candidate group
```

After removal the acceptance script checks:

```text
candidate files are gone from runtime
candidate exports are gone from runtime.rs
archive contains all 14 files
accepted write path still passes
```
