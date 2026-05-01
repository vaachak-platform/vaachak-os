# Phase 32–34 Notes

## Why These Phases Are Bundled

The project has already proven the storage, input, and display contract smoke layers.

Phase 32–34 can be bundled because each step is limited to pure helper adoption and does not move hardware behavior.

The active runtime uses silent helper probes rather than replacing physical
runtime paths. This gives Vaachak ownership of deterministic contracts without
touching the proven Pulp hardware/runtime flow.

## Why Physical Behavior Still Stays Imported

The imported Pulp runtime is proven on the real Xteink X4.

Risky paths remain imported for now:

```text
shared SPI bus arbitration
SSD1677 refresh/strip rendering
ADC ladder/debounce
EPUB cache IO
reader app internals
```

Normal boot continues to emit only:

```text
vaachak=x4-runtime-ready
```

## Future Work

After this phase, move one behavior path at a time.

Likely next:

```text
Phase 35 — Active Storage State Path Wiring
```
