# Phase 40B Overlay Manifest

Phase 40B — Reader UX Regression Baseline Overlay

Files added:
- target-xteink-x4/src/vaachak_x4/runtime/state_io_reader_ux_regression_baseline.rs
- target-xteink-x4/src/vaachak_x4/runtime/state_io_reader_ux_regression_baseline_acceptance.rs

Files changed:
- target-xteink-x4/src/vaachak_x4/runtime.rs

Scripts added:
- scripts/guard_phase40b_write_lane_closed.sh
- scripts/inspect_phase40b_reader_ux_surface.sh
- scripts/inspect_phase40b_epub_title_baseline.sh
- scripts/write_phase40b_manual_device_ux_report.sh
- scripts/capture_phase40b_reader_ux_baseline_bundle.sh
- scripts/accept_phase40b_reader_ux_regression_baseline.sh

Expected markers:
- phase40b=x4-reader-ux-regression-baseline-ok
- phase40b-acceptance=x4-reader-ux-regression-baseline-report-ok

Scope:
- Freeze current Home -> Files -> Reader behavior.
- Capture current footer/button labels.
- Capture EPUB title-display behavior.
- Capture reader restore behavior.
- Add no new UX feature.
