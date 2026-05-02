# Phase 40F — Library Title Layout Polish Patch

Phase 40F applies the first polish candidate from Phase 40E: Library title layout
consistency.

It is intentionally narrow.

Allowed:
- add a pure title layout helper
- patch Files/Library title rendering only if a safe known pattern is found

Forbidden:
- changing EPUB title source/cache behavior
- changing footer labels
- changing input mapping or ADC thresholds
- changing write lane
- changing display geometry/rotation
- changing reader pagination

Expected footer remains:

```text
Back Select Open Stay
```

Run:

```bash
./phase40f_library_title_layout_polish_patch_overlay/scripts/guard_phase40f_library_title_patch_scope.sh
./phase40f_library_title_layout_polish_patch_overlay/scripts/patch_phase40f_library_title_layout.sh
./phase40f_library_title_layout_polish_patch_overlay/scripts/inspect_phase40f_library_title_layout.sh
```

Then build, flash, and confirm Library title layout on device.
