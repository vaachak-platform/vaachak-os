# Phase 39L — Post-Freeze Scaffolding Cleanup Plan

Phase 39L is review-only. It does not delete code.

It protects the accepted write path:

```text
vendor/pulp-os/src/apps/reader/mod.rs
  -> vendor/pulp-os/src/apps/reader/typed_state_wiring.rs
  -> KernelHandle
  -> _X4/state
  -> restore
```

It also protects Phase 39J verification and Phase 39K freeze metadata.

Cleanup classification:

```text
Keep Active:
- reader/mod.rs
- reader/typed_state_wiring.rs

Keep Verification:
- Phase 39J runtime verification modules
- Phase 39J SD persistence scripts

Keep Freeze Metadata:
- Phase 39K freeze modules

Review Archive:
- Phase 38 design/write scaffolding
- shadow write prework

Review Delete Later:
- progress-only write adapters
- typed-record adapter experiments
- runtime-owned SD/FAT adapter lane
- target-side typed-state runtime wiring facade
```

Next phase can generate a deletion patch only after this plan is accepted.
