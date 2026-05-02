# Phase 40A Overlay Manifest

Phase 40A — Device Regression and Write-Lane Closeout Overlay

Files added:
- target-xteink-x4/src/vaachak_x4/runtime/state_io_device_regression_write_lane_closeout.rs
- target-xteink-x4/src/vaachak_x4/runtime/state_io_device_regression_write_lane_closeout_acceptance.rs

Files changed:
- target-xteink-x4/src/vaachak_x4/runtime.rs

Scripts added:
- scripts/guard_phase40a_accepted_write_path.sh
- scripts/record_phase40a_release_build_baseline.sh
- scripts/print_phase40a_flash_commands.sh
- scripts/inspect_phase40a_sd_persistence.sh
- scripts/capture_phase40a_sd_state_snapshot.sh
- scripts/inspect_phase40a_runtime_exports.sh
- scripts/write_phase40a_device_regression_report.sh
- scripts/accept_phase40a_device_regression_write_lane_closeout.sh

Expected markers:
- phase40a=x4-device-regression-write-lane-closeout-ok
- phase40a-acceptance=x4-device-regression-write-lane-closeout-report-ok

Scope:
- Release build baseline.
- Flash command helper.
- Device regression checklist.
- SD persistence verification.
- SD state snapshot.
- Runtime export inventory.
- Write-lane closeout acceptance.
