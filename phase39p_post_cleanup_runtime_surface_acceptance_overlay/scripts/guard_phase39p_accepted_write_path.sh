#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase39p-accepted-write-path-guard.txt}"
status="ACCEPTED"
reason="AcceptedPathPreserved"

fail() {
  status="REJECTED"
  reason="$1"
}

[ -f vendor/pulp-os/src/apps/reader/mod.rs ] || fail "MissingReaderMod"
[ -f vendor/pulp-os/src/apps/reader/typed_state_wiring.rs ] || fail "MissingTypedStateWiring"
[ -f target-xteink-x4/src/vaachak_x4/runtime/state_io_active_reader_save_callsite_wiring.rs ] || fail "MissingPhase39IActiveMetadata"
[ -f target-xteink-x4/src/vaachak_x4/runtime/state_io_runtime_state_write_verification.rs ] || fail "MissingPhase39JVerification"
[ -f target-xteink-x4/src/vaachak_x4/runtime/state_io_write_lane_cleanup_acceptance_freeze.rs ] || fail "MissingPhase39KFreeze"
[ -f target-xteink-x4/src/vaachak_x4/runtime/state_io_post_freeze_scaffolding_cleanup_plan.rs ] || fail "MissingPhase39LCleanupPlan"
[ -f target-xteink-x4/src/vaachak_x4/runtime/state_io_safe_scaffolding_archive_patch.rs ] || fail "MissingPhase39MArchivePatch"
[ -f target-xteink-x4/src/vaachak_x4/runtime/state_io_review_delete_later_removal_dry_run.rs ] || fail "MissingPhase39NDryRun"
[ -f target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_review_delete_later_removal_patch.rs ] || fail "MissingPhase39ORemovalPatch"

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
  echo "# Phase 39P Accepted Write Path Guard"
  echo "status=$status"
  echo "reason=$reason"
  echo "active_path=reader/mod.rs -> typed_state_wiring.rs -> KernelHandle -> _X4/state -> restore"
  echo "direct_reader_writes_present=$([ -n "$direct_reader_writes" ] && echo yes || echo no)"
  echo "missing_routing_checks=$missing_routing"
  echo "marker=phase39p=x4-post-cleanup-runtime-surface-acceptance-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 3
fi
