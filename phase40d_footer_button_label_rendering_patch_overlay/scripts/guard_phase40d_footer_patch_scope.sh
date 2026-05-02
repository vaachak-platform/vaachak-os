#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase40d-footer-patch-scope-guard.txt}"
status="ACCEPTED"
reason="FooterPatchScopeAllowed"

fail() {
  status="REJECTED"
  reason="$1"
}

[ -f target-xteink-x4/src/vaachak_x4/runtime/state_io_footer_button_label_baseline_fix_plan.rs ] || fail "MissingPhase40CPlan"
[ -f vendor/pulp-os/src/apps/reader/mod.rs ] || fail "MissingReaderMod"
[ -f vendor/pulp-os/src/apps/reader/typed_state_wiring.rs ] || fail "MissingTypedStateWiring"

if [ -f /tmp/phase40c-footer-button-label-baseline-fix-plan-acceptance.txt ]; then
  if ! grep -q '^status=ACCEPTED' /tmp/phase40c-footer-button-label-baseline-fix-plan-acceptance.txt; then
    fail "Phase40CAcceptanceFileRejected"
  fi
fi

direct_reader_writes="$(rg -n '\bk\s*\.\s*write_app_subdir\s*\(|\bk\s*\.\s*ensure_app_subdir\s*\(\s*reader_state::STATE_DIR\s*\)' vendor/pulp-os/src/apps/reader/mod.rs 2>/dev/null || true)"
if [ -n "$direct_reader_writes" ]; then
  status="REJECTED"
  reason="DirectReaderWritesRemain"
fi

{
  echo "# Phase 40D Footer Patch Scope Guard"
  echo "status=$status"
  echo "reason=$reason"
  echo "phase40c_acceptance_artifact=$([ -f /tmp/phase40c-footer-button-label-baseline-fix-plan-acceptance.txt ] && echo present || echo missing-ok)"
  echo "direct_reader_writes_present=$([ -n "$direct_reader_writes" ] && echo yes || echo no)"
  echo "allowed_change=footer-label-rendering-source-only"
  echo "forbidden=input-mapping,adc-thresholds,write-lane,display-geometry"
  echo "marker=phase40d=x4-footer-button-label-rendering-patch-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 3
fi
