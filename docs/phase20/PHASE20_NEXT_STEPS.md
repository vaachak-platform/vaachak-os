# Phase 20 Next Steps

Recommended next phase:

```text
Phase 21 — Display Boundary Extraction Smoke
```

Phase 21 should extract one narrow display-facing adapter at a time while keeping the imported Pulp display implementation callable and behavior-compatible.

Do not extract input and storage in the same phase. The X4 display, SD, and input paths are tightly coupled through timing, shared SPI ownership, and UI refresh behavior.
