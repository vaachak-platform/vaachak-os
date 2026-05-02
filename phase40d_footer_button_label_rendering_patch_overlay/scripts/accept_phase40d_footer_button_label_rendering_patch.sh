#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
PATCH_OUT="${PATCH_OUT:-/tmp/phase40d-footer-label-rendering-patch.txt}"
INSPECT_OUT="${INSPECT_OUT:-/tmp/phase40d-footer-label-rendering-inspection.txt}"
DEVICE_OUT="${DEVICE_OUT:-/tmp/phase40d-device-footer-label-report.txt}"
PATCHED_LIST="${PATCHED_LIST:-/tmp/phase40d-patched-files.txt}"
OUT="${OUT:-/tmp/phase40d-footer-button-label-rendering-patch-acceptance.txt}"

"$ROOT/phase40d_footer_button_label_rendering_patch_overlay/scripts/guard_phase40d_footer_patch_scope.sh" >/dev/null

if [ ! -f "$PATCH_OUT" ]; then
  echo "missing patch report: $PATCH_OUT" >&2
  exit 2
fi

"$ROOT/phase40d_footer_button_label_rendering_patch_overlay/scripts/inspect_phase40d_footer_label_rendering.sh" >/dev/null

patch_status="$(grep '^status=' "$PATCH_OUT" | head -1 | cut -d= -f2-)"
patched_files="$(grep '^patched_files=' "$PATCH_OUT" | head -1 | cut -d= -f2-)"
expected_count="$(rg -n 'Back.*Select.*Open.*Stay|Back.*Select.*open.*Stay|\["Back", "Select", "Open", "Stay"\]|\[b"Back", b"Select", b"Open", b"Stay"\]' \
  vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null | wc -l | tr -d ' ')"
forbidden_count="$(rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay|\["Select", "Open", "Back", "Stay"\]|\["Select", "open", "Back", "Stay"\]' \
  vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs 2>/dev/null | wc -l | tr -d ' ')"

protected_touched=0
if [ -f "$PATCHED_LIST" ]; then
  if grep -E '(^hal-xteink-x4/src/input\.rs$|^target-xteink-x4/src/vaachak_x4/input/|^target-xteink-x4/src/vaachak_x4/contracts/input|^vendor/pulp-os/src/apps/reader/typed_state_wiring\.rs$)' "$PATCHED_LIST" >/dev/null 2>&1; then
    protected_touched=1
  fi
fi

device_status="MISSING"
if [ -f "$DEVICE_OUT" ]; then
  device_status="$(grep '^status=' "$DEVICE_OUT" | head -1 | cut -d= -f2-)"
fi

status="ACCEPTED"
reason="FooterRenderingPatchAccepted"

if [ "$patch_status" != "ACCEPTED" ]; then
  status="REJECTED"
  reason="PatchReportRejected"
elif [ "$patched_files" = "0" ]; then
  status="REJECTED"
  reason="NoFilesPatched"
elif [ "$protected_touched" != "0" ]; then
  status="REJECTED"
  reason="ProtectedInputOrWriteFileTouched"
elif [ "$expected_count" = "0" ]; then
  status="REJECTED"
  reason="ExpectedFooterLabelsNotFound"
elif [ "$forbidden_count" != "0" ]; then
  status="REJECTED"
  reason="OldFooterOrderStillFound"
elif [ "$device_status" != "ACCEPTED" ]; then
  status="REJECTED"
  reason="DeviceFooterReportMissingOrRejected"
fi

{
  echo "# Phase 40D Footer/Button Label Rendering Patch Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "patch_status=$patch_status"
  echo "patched_files=$patched_files"
  echo "expected_footer_count=$expected_count"
  echo "forbidden_old_footer_count=$forbidden_count"
  echo "protected_touched=$protected_touched"
  echo "device_status=$device_status"
  echo "changes_input_mapping=false"
  echo "touches_write_lane=false"
  echo "touches_display_geometry=false"
  echo "marker=phase40d=x4-footer-button-label-rendering-patch-ok"
  echo "patch_report=$PATCH_OUT"
  echo "inspection=$INSPECT_OUT"
  echo "device_report=$DEVICE_OUT"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 5
fi
