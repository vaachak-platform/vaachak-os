# Phase 35C-2 Notes

## Why This Phase Exists

The failed active `AppLayer` wrapper showed that even a delegating wrapper can
disturb the real hardware input path. Phase 35C-2 records that lesson as an
automated check.

## Next Safe Extraction Approach

Future active theme/metadata work should avoid wrapping the whole app layer.
Safer options are:

```text
- a small upstreamable hook in the reader state methods
- a feature-gated Pulp-compatible trait boundary
- a copied Vaachak reader slice that owns a narrow lifecycle surface
```

Do not reintroduce an active Vaachak `AppLayer` wrapper unless hardware testing
explicitly accepts it.
