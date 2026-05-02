#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase40c-reader-ux-baseline-guard.txt}"
status="ACCEPTED"
reason="ReaderUxBaselinePresent"

fail() {
  status="REJECTED"
  reason="$1"
}

[ -f target-xteink-x4/src/vaachak_x4/runtime/state_io_reader_ux_regression_baseline.rs ] || fail "MissingPhase40BBaseline"
[ -f target-xteink-x4/src/vaachak_x4/runtime/state_io_device_regression_write_lane_closeout.rs ] || fail "MissingPhase40ACloseout"
[ -f vendor/pulp-os/src/apps/reader/mod.rs ] || fail "MissingReaderMod"
[ -f vendor/pulp-os/src/apps/reader/typed_state_wiring.rs ] || fail "MissingTypedStateWiring"

if [ -f /tmp/phase40b-reader-ux-regression-baseline-acceptance.txt ]; then
  if ! grep -q '^status=ACCEPTED' /tmp/phase40b-reader-ux-regression-baseline-acceptance.txt; then
    fail "Phase40BAcceptanceFileRejected"
  fi
else
  # Allow source-level guard to pass even if /tmp artifacts were cleaned.
  :
fi

direct_reader_writes="$(rg -n '\bk\s*\.\s*write_app_subdir\s*\(|\bk\s*\.\s*ensure_app_subdir\s*\(\s*reader_state::STATE_DIR\s*\)' vendor/pulp-os/src/apps/reader/mod.rs 2>/dev/null || true)"
if [ -n "$direct_reader_writes" ]; then
  status="REJECTED"
  reason="DirectReaderWritesRemain"
fi

{
  echo "# Phase 40C Reader UX Baseline Guard"
  echo "status=$status"
  echo "reason=$reason"
  echo "phase40b_acceptance_artifact=$([ -f /tmp/phase40b-reader-ux-regression-baseline-acceptance.txt ] && echo present || echo missing-ok)"
  echo "direct_reader_writes_present=$([ -n "$direct_reader_writes" ] && echo yes || echo no)"
  echo "marker=phase40c=x4-footer-button-label-baseline-fix-plan-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 3
fi
