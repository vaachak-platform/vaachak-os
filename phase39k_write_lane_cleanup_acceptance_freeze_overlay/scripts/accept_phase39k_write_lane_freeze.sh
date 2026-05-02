#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase39k-write-lane-freeze-acceptance.txt}"

missing=0

check_file() {
  local file="$1"
  if [ ! -f "$file" ]; then
    echo "missing: $file" >&2
    missing=$((missing + 1))
  fi
}

check_file "vendor/pulp-os/src/apps/reader/mod.rs"
check_file "vendor/pulp-os/src/apps/reader/typed_state_wiring.rs"
check_file "target-xteink-x4/src/vaachak_x4/runtime/state_io_active_reader_save_callsite_wiring.rs"
check_file "target-xteink-x4/src/vaachak_x4/runtime/state_io_runtime_state_write_verification.rs"
check_file "target-xteink-x4/src/vaachak_x4/runtime/state_io_write_lane_cleanup_acceptance_freeze.rs"

if [ "$missing" -ne 0 ]; then
  exit 2
fi

direct_reader_writes="$(rg -n '\bk\s*\.\s*write_app_subdir\s*\(|\bk\s*\.\s*ensure_app_subdir\s*\(\s*reader_state::STATE_DIR\s*\)' vendor/pulp-os/src/apps/reader/mod.rs || true)"

status="ACCEPTED"
reason="ActivePathAndEvidenceAccepted"

if [ -n "$direct_reader_writes" ]; then
  status="REJECTED"
  reason="DirectReaderWritesRemain"
fi

{
  echo "# Phase 39K Write Lane Freeze Acceptance"
  echo "status=$status"
  echo "reason=$reason"
  echo "deletes_code_now=false"
  echo "adds_new_write_abstraction=false"
  echo "accepted_path=reader/mod.rs -> typed_state_wiring.rs -> KernelHandle -> _X4/state -> restore"
  echo "marker=phase39k=x4-write-lane-cleanup-acceptance-freeze-ok"
  echo
  if [ -n "$direct_reader_writes" ]; then
    echo "## direct reader writes still present"
    echo "$direct_reader_writes"
  else
    echo "## direct reader writes still present"
    echo "none"
  fi
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$status" != "ACCEPTED" ]; then
  exit 3
fi
