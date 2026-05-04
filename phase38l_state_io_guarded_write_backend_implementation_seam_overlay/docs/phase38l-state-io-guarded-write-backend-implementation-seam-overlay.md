# Phase 38L — State I/O Guarded Write Backend Implementation Seam Overlay

This phase introduces a compile-time seam for the future typed-state mutation backend.

It covers:
- `.PRG`
- `.THM`
- `.MTA`
- `.BKM`
- `BMIDX.TXT`

The default behavior is still deny-by-default. The seam can plan a mutation, return a dry-run result, or declare that a future backend dispatch would be allowed by policy, but no live mutation is performed here.

Expected marker:

```text
phase38l=x4-state-io-guarded-write-backend-implementation-seam-ok
```
