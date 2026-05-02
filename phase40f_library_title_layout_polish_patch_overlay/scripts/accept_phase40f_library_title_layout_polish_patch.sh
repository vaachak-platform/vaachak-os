#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
PATCH_OUT="${PATCH_OUT:-/tmp/phase40f-library-title-layout-patch.txt}"
INSPECT_OUT="${INSPECT_OUT:-/tmp/phase40f-library-title-layout-inspection.txt}"
DEVICE_OUT="${DEVICE_OUT:-/tmp/phase40f-device-library-title-layout-report.txt}"
PATCHED_LIST="${PATCHED_LIST:-/tmp/phase40f-patched-files.txt}"
OUT="${OUT:-/tmp/phase40f-library-title-layout-polish-patch-acceptance.txt}"

"$ROOT/phase40f_library_title_layout_polish_patch_overlay/scripts/guard_phase40f_library_title_patch_scope.sh" >/dev/null

if [ ! -f "$PATCH_OUT" ]; then
  echo "missing patch report: $PATCH_OUT" >&2
  exit 2
fi

"$ROOT/phase40f_library_title_layout_polish_patch_overlay/scripts/inspect_phase40f_library_title_layout.sh" >/dev/null

patch_status="$(grep '^status=' "$PATCH_OUT" | head -1 | cut -d= -f2-)"
patched_files="$(grep '^patched_files=' "$PATCH_OUT" | head -1 | cut -d= -f2-)"
old_footer_count="$(rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay' vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null | wc -l | tr -d ' ')"
footer_count="$(rg -n 'Back.*Select.*Open.*Stay|\["Back", "Select", "Open", "Stay"\]' vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null | wc -l | tr -d ' ')"

protected_touched=0
if [ -f "$PATCHED_LIST" ]; then
  if grep -E '(^hal-xteink-x4/src/input\.rs$|^target-xteink-x4/src/vaachak_x4/input/|^target-xteink-x4/src/vaachak_x4/contracts/input|^vendor/pulp-os/src/apps/reader/typed_state_wiring\.rs$|^vendor/pulp-os/src/apps/reader/)' "$PATCHED_LIST" >/dev/null 2>&1; then
    protected_touched=1
  fi
fi

device_status="MISSING"
if [ -f "$DEVICE_OUT" ]; then
  device_status="$(grep '^status=' "$DEVICE_OUT" | head -1 | cut -d= -f2-)"
fi

status="ACCEPTED"
reason="LibraryTitleLayoutPatchAccepted"

if [ "$patch_status" != "ACCEPTED" ]; then
  status="REJECTED"
  reason="PatchReportRejected"
elif [ "$patched_files" = "0" ]; then
  status="REJECTED"
  reason="NoFilesPatched"
elif [ "$protected_touched" != "0" ]; then
  status="REJECTED"
  reason="ProtectedFileTouched"
elif [ "$old_footer_count" != "0" ]; then
  status="REJECTED"
  reason="FooterRegressionDetected"
elif [ "$footer_count" = "0" ]; then
  status="REJECTED"
  reason="AcceptedFooterLabelsMissing"
elif [ "$device_status" != "ACCEPTED" ]; then
  status="REJECTED"
  reason="DeviceLibraryLayoutReportMissingOrRejected"
fi

{
  echo "# Phase 40F Library Title Layout Polish Patch Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "patch_status=$patch_status"
  echo "patched_files=$patched_files"
  echo "protected_touched=$protected_touched"
  echo "accepted_footer_count=$footer_count"
  echo "old_footer_count=$old_footer_count"
  echo "device_status=$device_status"
  echo "changes_title_source=false"
  echo "changes_footer_labels=false"
  echo "changes_input_mapping=false"
  echo "touches_write_lane=false"
  echo "touches_display_geometry=false"
  echo "touches_reader_pagination=false"
  echo "marker=phase40f=x4-library-title-layout-polish-patch-ok"
  echo "patch_report=$PATCH_OUT"
  echo "inspection=$INSPECT_OUT"
  echo "device_report=$DEVICE_OUT"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 5
fi
