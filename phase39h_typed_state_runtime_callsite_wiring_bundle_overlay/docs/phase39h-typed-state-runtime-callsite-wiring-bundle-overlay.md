# Phase 39H — Typed State Runtime Callsite Wiring Bundle Overlay

This phase wires all typed-state write entrypoints at once through the Phase 39G
runtime file API integration gate and Phase 39F runtime-owned writer.

Entry points added:

```text
phase39h_write_progress_state
phase39h_write_theme_state
phase39h_write_metadata_state
phase39h_write_bookmark_state
phase39h_append_bookmark_index
phase39h_replace_bookmark_index
phase39h_compact_bookmark_index
phase39h_write_all_typed_state
```

Scope:

```text
.PRG
.THM
.MTA
.BKM
BMIDX.TXT
```

This still keeps the actual reader/app callsite patching separate, but provides
the single facade those callsites should use. The included locator script finds
likely save callsites for the next patch.
