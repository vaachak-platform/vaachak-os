#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase40e-reader-ux-polish-scope-guard.txt}"
status="ACCEPTED"
reason="ReaderUxPolishPlanScopeAllowed"

fail() {
  status="REJECTED"
  reason="$1"
}

[ -f target-xteink-x4/src/vaachak_x4/runtime/state_io_footer_button_label_rendering_patch.rs ] || fail "MissingPhase40DFooterPatch"
[ -f target-xteink-x4/src/vaachak_x4/runtime/state_io_reader_ux_regression_baseline.rs ] || fail "MissingPhase40BBaseline"
[ -f vendor/pulp-os/src/apps/reader/mod.rs ] || fail "MissingReaderMod"
[ -f vendor/pulp-os/src/apps/reader/typed_state_wiring.rs ] || fail "MissingTypedStateWiring"

if [ -f /tmp/phase40d-footer-button-label-rendering-patch-acceptance.txt ]; then
  if ! grep -q '^status=ACCEPTED' /tmp/phase40d-footer-button-label-rendering-patch-acceptance.txt; then
    fail "Phase40DAcceptanceRejected"
  fi
fi

direct_reader_writes="$(rg -n '\bk\s*\.\s*write_app_subdir\s*\(|\bk\s*\.\s*ensure_app_subdir\s*\(\s*reader_state::STATE_DIR\s*\)' vendor/pulp-os/src/apps/reader/mod.rs 2>/dev/null || true)"
old_footer_order="$(rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay' vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true)"

if [ -n "$direct_reader_writes" ]; then
  status="REJECTED"
  reason="DirectReaderWritesRemain"
elif [ -n "$old_footer_order" ]; then
  status="REJECTED"
  reason="OldFooterOrderFound"
fi

{
  echo "# Phase 40E Reader UX Polish Scope Guard"
  echo "status=$status"
  echo "reason=$reason"
  echo "phase40d_acceptance_artifact=$([ -f /tmp/phase40d-footer-button-label-rendering-patch-acceptance.txt ] && echo present || echo missing-ok)"
  echo "direct_reader_writes_present=$([ -n "$direct_reader_writes" ] && echo yes || echo no)"
  echo "old_footer_order_present=$([ -n "$old_footer_order" ] && echo yes || echo no)"
  echo "plan_only=true"
  echo "forbidden=footer-label-change,input-mapping,adc-thresholds,write-lane,display-geometry"
  echo "marker=phase40e=x4-reader-ux-polish-candidate-plan-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 3
fi
