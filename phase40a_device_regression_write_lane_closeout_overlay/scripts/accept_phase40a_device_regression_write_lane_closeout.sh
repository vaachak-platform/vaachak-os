#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
SD="${SD:-/media/mindseye73/C0D2-109E}"
DEVICE_REGRESSION_CONFIRMED="${DEVICE_REGRESSION_CONFIRMED:-0}"
RESTORE_VERIFIED="${RESTORE_VERIFIED:-0}"

BUILD_SUMMARY="${BUILD_SUMMARY:-/tmp/phase40a-release-build-baseline/summary.txt}"
GUARD_OUT="${GUARD_OUT:-/tmp/phase40a-accepted-write-path-guard.txt}"
EXPORT_OUT="${EXPORT_OUT:-/tmp/phase40a-runtime-export-inventory.txt}"
SD_OUT="${SD_OUT:-/tmp/phase40a-sd-persistence-inspection.txt}"
REGRESSION_OUT="${REGRESSION_OUT:-/tmp/phase40a-device-regression-report.txt}"
OUT="${OUT:-/tmp/phase40a-device-regression-write-lane-closeout-acceptance.txt}"

"$ROOT/phase40a_device_regression_write_lane_closeout_overlay/scripts/guard_phase40a_accepted_write_path.sh" >/dev/null
"$ROOT/phase40a_device_regression_write_lane_closeout_overlay/scripts/inspect_phase40a_runtime_exports.sh" >/dev/null
SD="$SD" "$ROOT/phase40a_device_regression_write_lane_closeout_overlay/scripts/inspect_phase40a_sd_persistence.sh" >/dev/null
DEVICE_REGRESSION_CONFIRMED="$DEVICE_REGRESSION_CONFIRMED" RESTORE_VERIFIED="$RESTORE_VERIFIED" SD="$SD" \
  "$ROOT/phase40a_device_regression_write_lane_closeout_overlay/scripts/write_phase40a_device_regression_report.sh" >/dev/null

build_status="MISSING"
if [ -f "$BUILD_SUMMARY" ]; then
  build_status="$(grep '^status=' "$BUILD_SUMMARY" | head -1 | cut -d= -f2-)"
fi

guard_status="$(grep '^status=' "$GUARD_OUT" | head -1 | cut -d= -f2-)"
export_violations="$(grep '^export_violations=' "$EXPORT_OUT" | head -1 | cut -d= -f2-)"
sd_status="$(grep '^status=' "$SD_OUT" | head -1 | cut -d= -f2-)"
regression_status="$(grep '^status=' "$REGRESSION_OUT" | head -1 | cut -d= -f2-)"

status="ACCEPTED"
reason="WriteLaneClosed"

if [ "$build_status" != "ACCEPTED" ]; then
  status="REJECTED"
  reason="ReleaseBuildBaselineMissingOrFailed"
elif [ "$guard_status" != "ACCEPTED" ]; then
  status="REJECTED"
  reason="AcceptedPathGuardFailed"
elif [ "$export_violations" != "0" ]; then
  status="REJECTED"
  reason="RuntimeExportViolation"
elif [ "$sd_status" != "ACCEPTED" ]; then
  status="REJECTED"
  reason="SdPersistenceMissing"
elif [ "$regression_status" != "ACCEPTED" ]; then
  status="REJECTED"
  reason="DeviceRegressionMissing"
fi

{
  echo "# Phase 40A Device Regression and Write-Lane Closeout Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "build_status=$build_status"
  echo "guard_status=$guard_status"
  echo "export_violations=$export_violations"
  echo "sd_status=$sd_status"
  echo "device_regression_status=$regression_status"
  echo "device_regression_confirmed=$DEVICE_REGRESSION_CONFIRMED"
  echo "restore_verified=$RESTORE_VERIFIED"
  echo "marker=phase40a=x4-device-regression-write-lane-closeout-ok"
  echo "build_summary=$BUILD_SUMMARY"
  echo "guard=$GUARD_OUT"
  echo "runtime_exports=$EXPORT_OUT"
  echo "sd_persistence=$SD_OUT"
  echo "device_regression=$REGRESSION_OUT"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 7
fi
