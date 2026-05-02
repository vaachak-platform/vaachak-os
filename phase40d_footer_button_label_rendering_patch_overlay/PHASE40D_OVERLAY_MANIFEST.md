# Phase 40D Overlay Manifest

Phase 40D — Footer/Button Label Rendering Patch Overlay

Files added:
- target-xteink-x4/src/vaachak_x4/runtime/state_io_footer_button_label_rendering_patch.rs
- target-xteink-x4/src/vaachak_x4/runtime/state_io_footer_button_label_rendering_patch_acceptance.rs

Files changed:
- target-xteink-x4/src/vaachak_x4/runtime.rs
- footer/rendering source file(s) containing old footer label order

Scripts added:
- scripts/guard_phase40d_footer_patch_scope.sh
- scripts/patch_phase40d_footer_label_rendering.sh
- scripts/inspect_phase40d_footer_label_rendering.sh
- scripts/write_phase40d_device_footer_label_report.sh
- scripts/accept_phase40d_footer_button_label_rendering_patch.sh

Expected markers:
- phase40d=x4-footer-button-label-rendering-patch-ok
- phase40d-acceptance=x4-footer-button-label-rendering-patch-report-ok

Scope:
- Patch footer label order only.
- Expected visible order: Back Select Open Stay.
- Do not change input mapping.
- Do not change ADC thresholds.
- Do not change write lane.
- Do not change display geometry/rotation.
