#!/usr/bin/env bash
set -euo pipefail
ROOT="$(pwd)"
PATCH_OUT="${PATCH_OUT:-/tmp/phase40g-home-library-title-patch.txt}"
DEVICE_OUT="${DEVICE_OUT:-/tmp/phase40g-device-home-library-title-report.txt}"
PATCHED_LIST="${PATCHED_LIST:-/tmp/phase40g-patched-files.txt}"
OUT="${OUT:-/tmp/phase40g-home-library-title-patch-acceptance.txt}"
"$ROOT/phase40g_home_full_width_library_titles_patch_overlay/scripts/guard_phase40g_patch_scope.sh" >/dev/null
[ -f "$PATCH_OUT" ] || { echo "missing patch report: $PATCH_OUT" >&2; exit 2; }
"$ROOT/phase40g_home_full_width_library_titles_patch_overlay/scripts/inspect_phase40g_home_library_title_display.sh" >/dev/null
patch_status="$(grep '^status=' "$PATCH_OUT" | head -1 | cut -d= -f2-)"
patched_files="$(grep '^patched_files=' "$PATCH_OUT" | head -1 | cut -d= -f2-)"
old_footer_count="$((rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay' vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true) | wc -l | tr -d ' ')"
protected_touched=0
if [ -f "$PATCHED_LIST" ] && grep -E '(^hal-xteink-x4/src/input\.rs$|^target-xteink-x4/src/vaachak_x4/input/|^target-xteink-x4/src/vaachak_x4/contracts/input|^vendor/pulp-os/src/apps/reader/)' "$PATCHED_LIST" >/dev/null 2>&1; then protected_touched=1; fi
device_status="MISSING"; [ -f "$DEVICE_OUT" ] && device_status="$(grep '^status=' "$DEVICE_OUT" | head -1 | cut -d= -f2-)"
status="ACCEPTED"; reason="HomeLibraryTitlePatchAccepted"
[ "$patch_status" = ACCEPTED ] || { status="REJECTED"; reason="PatchReportRejected"; }
[ "$patched_files" != 0 ] || { status="REJECTED"; reason="NoFilesPatched"; }
[ "$protected_touched" = 0 ] || { status="REJECTED"; reason="ProtectedFileTouched"; }
[ "$old_footer_count" = 0 ] || { status="REJECTED"; reason="FooterRegressionDetected"; }
[ "$device_status" = ACCEPTED ] || { status="REJECTED"; reason="DeviceReportMissingOrRejected"; }
{
  echo "# Phase 40G Home/Library Title Patch Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "patch_status=$patch_status"
  echo "patched_files=$patched_files"
  echo "protected_touched=$protected_touched"
  echo "old_footer_count=$old_footer_count"
  echo "device_status=$device_status"
  echo "changes_home_title_layout=true"
  echo "changes_library_title_resolution=true"
  echo "changes_footer_labels=false"
  echo "changes_input_mapping=false"
  echo "touches_write_lane=false"
  echo "touches_display_geometry=false"
  echo "touches_reader_pagination=false"
  echo "marker=phase40g=x4-home-full-width-library-title-patch-ok"
} | tee "$OUT"
[ "$status" = ACCEPTED ] || exit 5
