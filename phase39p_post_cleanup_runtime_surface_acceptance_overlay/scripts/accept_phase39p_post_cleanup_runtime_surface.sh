#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
GUARD_OUT="/tmp/phase39p-accepted-write-path-guard.txt"
INSPECT_OUT="/tmp/phase39p-runtime-surface-inspection.txt"
BUILD_SUMMARY="${BUILD_SUMMARY:-/tmp/phase39p-build-baseline/summary.txt}"
OUT="${OUT:-/tmp/phase39p-post-cleanup-runtime-surface-acceptance.txt}"

"$ROOT/phase39p_post_cleanup_runtime_surface_acceptance_overlay/scripts/guard_phase39p_accepted_write_path.sh" >/dev/null
"$ROOT/phase39p_post_cleanup_runtime_surface_acceptance_overlay/scripts/inspect_phase39p_runtime_surface.sh" >/dev/null

guard_status="$(grep '^status=' "$GUARD_OUT" | head -1 | cut -d= -f2-)"
export_violations="$(grep '^export_violations=' "$INSPECT_OUT" | head -1 | cut -d= -f2-)"
runtime_file_violations="$(grep '^runtime_file_violations=' "$INSPECT_OUT" | head -1 | cut -d= -f2-)"
build_status="MISSING"

if [ -f "$BUILD_SUMMARY" ]; then
  build_status="$(grep '^status=' "$BUILD_SUMMARY" | head -1 | cut -d= -f2-)"
fi

phase39j_scripts_present="no"
if [ -f "phase39j_runtime_state_write_verification_acceptance_overlay/scripts/inspect_phase39j_sd_state.sh" ] \
   && [ -f "phase39j_runtime_state_write_verification_acceptance_overlay/scripts/accept_phase39j_sd_persistence.sh" ]; then
  phase39j_scripts_present="yes"
fi

status="ACCEPTED"
reason="RuntimeSurfaceAccepted"

if [ "$guard_status" != "ACCEPTED" ]; then
  status="REJECTED"
  reason="AcceptedPathGuardFailed"
elif [ "$export_violations" != "0" ] || [ "$runtime_file_violations" != "0" ]; then
  status="REJECTED"
  reason="ArchivedScaffoldingStillOnRuntimeSurface"
elif [ "$phase39j_scripts_present" != "yes" ]; then
  status="REJECTED"
  reason="Phase39JVerificationScriptsMissing"
elif [ "$build_status" != "ACCEPTED" ]; then
  status="REJECTED"
  reason="BuildBaselineMissingOrFailed"
fi

{
  echo "# Phase 39P Post-Cleanup Runtime Surface Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "guard_status=$guard_status"
  echo "export_violations=$export_violations"
  echo "runtime_file_violations=$runtime_file_violations"
  echo "phase39j_scripts_present=$phase39j_scripts_present"
  echo "build_status=$build_status"
  echo "marker=phase39p=x4-post-cleanup-runtime-surface-acceptance-ok"
  echo "guard=$GUARD_OUT"
  echo "inspection=$INSPECT_OUT"
  echo "build_summary=$BUILD_SUMMARY"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 5
fi
