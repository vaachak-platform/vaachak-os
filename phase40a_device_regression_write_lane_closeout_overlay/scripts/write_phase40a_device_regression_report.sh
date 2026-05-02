#!/usr/bin/env bash
set -euo pipefail

DEVICE_REGRESSION_CONFIRMED="${DEVICE_REGRESSION_CONFIRMED:-0}"
RESTORE_VERIFIED="${RESTORE_VERIFIED:-0}"
SD="${SD:-/media/mindseye73/C0D2-109E}"
OUT="${OUT:-/tmp/phase40a-device-regression-report.txt}"

status="ACCEPTED"
reason="DeviceRegressionConfirmed"

if [ "$DEVICE_REGRESSION_CONFIRMED" != "1" ]; then
  status="REJECTED"
  reason="DeviceRegressionNotConfirmed"
elif [ "$RESTORE_VERIFIED" != "1" ]; then
  status="REJECTED"
  reason="RestoreNotVerified"
fi

{
  echo "# Phase 40A Device Regression Report"
  echo "status=$status"
  echo "reason=$reason"
  echo "device_regression_confirmed=$DEVICE_REGRESSION_CONFIRMED"
  echo "restore_verified=$RESTORE_VERIFIED"
  echo "sd=$SD"
  echo "marker=phase40a=x4-device-regression-write-lane-closeout-ok"
  echo
  echo "## Manual checklist"
  echo "- Home opens"
  echo "- Files/Library opens"
  echo "- EPUB titles display correctly"
  echo "- Open EPUB"
  echo "- Scroll a few pages"
  echo "- Back returns to Library"
  echo "- Reopen EPUB and confirm progress restores"
  echo "- Change theme and confirm it persists after reopen"
  echo "- Add/remove bookmark and confirm bookmark list/index updates"
  echo "- No crash/reboot"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 6
fi
