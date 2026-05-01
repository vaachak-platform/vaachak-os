# Phase 31 Notes

## Why This Phase Exists

Phase 30 gave Vaachak ownership of the target namespace.

Phase 31 begins using Vaachak-owned logic in a low-risk way by adopting pure storage path/name helpers.

## Why Only Path Helpers

Path/name helpers are deterministic and host-testable.

They do not require hardware access and do not change SD/SPI behavior.

The active runtime has no safe local state-file construction call site to
replace without reaching into imported Pulp IO. Phase 31 therefore uses the
fallback approach: a pure Vaachak adoption probe is called from the active
wrapper, and Vaachak's storage-state contract delegates to the path helper
module. Physical IO remains imported.

## Future Work

After Phase 31, possible next steps:

```text
Phase 32 — State/progress/bookmark path ownership in active runtime
Phase 33 — Input semantic mapping ownership
Phase 34 — Display geometry helper ownership
```

Physical behavior should still move one path at a time.
