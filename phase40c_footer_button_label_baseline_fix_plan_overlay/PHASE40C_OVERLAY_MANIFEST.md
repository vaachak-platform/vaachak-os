# Phase 40C Overlay Manifest

Phase 40C — Footer/Button Label Baseline and Fix Plan Overlay

Files added:
- target-xteink-x4/src/vaachak_x4/runtime/state_io_footer_button_label_baseline_fix_plan.rs
- target-xteink-x4/src/vaachak_x4/runtime/state_io_footer_button_label_baseline_fix_plan_acceptance.rs

Files changed:
- target-xteink-x4/src/vaachak_x4/runtime.rs

Scripts added:
- scripts/guard_phase40c_reader_ux_baseline.sh
- scripts/inspect_phase40c_footer_button_sources.sh
- scripts/inspect_phase40c_button_mapping_candidates.sh
- scripts/write_phase40c_expected_footer_labels_baseline.sh
- scripts/plan_phase40c_footer_button_label_fix.sh
- scripts/capture_phase40c_footer_button_plan_bundle.sh
- scripts/accept_phase40c_footer_button_label_baseline_fix_plan.sh

Expected markers:
- phase40c=x4-footer-button-label-baseline-fix-plan-ok
- phase40c-acceptance=x4-footer-button-label-baseline-fix-plan-report-ok

Scope:
- Plan-only.
- No rendering change.
- No input mapping change.
- No write-lane change.
- Capture expected labels and exact patch plan.
