#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase40b-write-lane-closed-guard.txt}"
status="ACCEPTED"
reason="WriteLaneClosedAndAcceptedPathPreserved"

fail() {
  status="REJECTED"
  reason="$1"
}

[ -f vendor/pulp-os/src/apps/reader/mod.rs ] || fail "MissingReaderMod"
[ -f vendor/pulp-os/src/apps/reader/typed_state_wiring.rs ] || fail "MissingTypedStateWiring"
[ -f target-xteink-x4/src/vaachak_x4/runtime/state_io_device_regression_write_lane_closeout.rs ] || fail "MissingPhase40ACloseout"
[ -f target-xteink-x4/src/vaachak_x4/runtime/state_io_post_cleanup_runtime_surface_acceptance.rs ] || fail "MissingPhase39PAcceptance"
[ -f target-xteink-x4/src/vaachak_x4/runtime/state_io_runtime_state_write_verification.rs ] || fail "MissingPhase39JVerification"

direct_reader_writes="$(rg -n '\bk\s*\.\s*write_app_subdir\s*\(|\bk\s*\.\s*ensure_app_subdir\s*\(\s*reader_state::STATE_DIR\s*\)' vendor/pulp-os/src/apps/reader/mod.rs 2>/dev/null || true)"
if [ -n "$direct_reader_writes" ]; then
  status="REJECTED"
  reason="DirectReaderWritesRemain"
fi

missing_routing=0
for needle in \
  'typed_state_wiring::write_app_subdir' \
  'typed_state_wiring::ensure_state_dir' \
  'persist_progress_records' \
  'persist_theme_preset' \
  'persist_meta_record' \
  'persist_bookmarks' \
  'persist_bookmarks_index' \
  'ensure_bookmark_stub'
do
  if ! rg -n "$needle" vendor/pulp-os/src/apps/reader/mod.rs >/dev/null 2>&1; then
    missing_routing=$((missing_routing + 1))
  fi
done

if [ "$missing_routing" -ne 0 ]; then
  status="REJECTED"
  reason="MissingActiveReaderRouting"
fi

{
  echo "# Phase 40B Write-Lane Closed Guard"
  echo "status=$status"
  echo "reason=$reason"
  echo "active_path=reader/mod.rs -> typed_state_wiring.rs -> KernelHandle -> _X4/state -> restore"
  echo "direct_reader_writes_present=$([ -n "$direct_reader_writes" ] && echo yes || echo no)"
  echo "missing_routing_checks=$missing_routing"
  echo "marker=phase40b=x4-reader-ux-regression-baseline-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 3
fi
