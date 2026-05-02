# Phase 40F Overlay Manifest

Phase 40F — Library Title Layout Polish Patch Overlay

Files added:
- target-xteink-x4/src/vaachak_x4/runtime/state_io_library_title_layout_polish_patch.rs
- target-xteink-x4/src/vaachak_x4/runtime/state_io_library_title_layout_polish_patch_acceptance.rs
- target-xteink-x4/src/vaachak_x4/ui/library_title_layout.rs

Files changed:
- target-xteink-x4/src/vaachak_x4/runtime.rs
- target-xteink-x4/src/vaachak_x4/ui.rs if present, or creates it if needed
- vendor/pulp-os/src/apps/files.rs only if safe known title-layout pattern exists

Scripts added:
- scripts/guard_phase40f_library_title_patch_scope.sh
- scripts/patch_phase40f_library_title_layout.sh
- scripts/inspect_phase40f_library_title_layout.sh
- scripts/write_phase40f_device_library_title_report.sh
- scripts/accept_phase40f_library_title_layout_polish_patch.sh

Expected markers:
- phase40f=x4-library-title-layout-polish-patch-ok
- phase40f-acceptance=x4-library-title-layout-polish-patch-report-ok

Scope:
- Polish Files/Library title layout only.
- Preserve EPUB title source/cache behavior.
- Preserve footer labels: Back Select Open Stay.
- Preserve input mapping, write lane, display geometry, and reader pagination.
