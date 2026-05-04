#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase40l-runtime-scaffolding-inspection.txt}"
RUNTIME_DIR="${RUNTIME_DIR:-target-xteink-x4/src/vaachak_x4/runtime}"
RUNTIME_MOD="${RUNTIME_MOD:-target-xteink-x4/src/vaachak_x4/runtime.rs}"

{
  echo "# Phase 40L Runtime Scaffolding Inspection"
  echo "runtime_dir=$RUNTIME_DIR"
  echo "runtime_mod=$RUNTIME_MOD"
  echo

  echo "## runtime phase files"
  find "$RUNTIME_DIR" -maxdepth 1 -type f \( \
    -name '*phase*.rs' -o \
    -name 'state_io_*acceptance*.rs' -o \
    -name 'state_io_*repair*.rs' -o \
    -name 'state_io_*freeze*.rs' -o \
    -name 'state_io_*plan*.rs' \
  \) -print | sort || true
  echo

  echo "## runtime exports with phase/repair/freeze/plan/acceptance"
  if [ -f "$RUNTIME_MOD" ]; then
    rg -n 'state_io_.*(phase|repair|freeze|plan|acceptance)|phase[0-9]|Phase[0-9]' "$RUNTIME_MOD" 2>/dev/null || true
  else
    echo "missing=$RUNTIME_MOD"
  fi
  echo

  echo "## marker/report modules"
  rg -n 'MARKER|marker\(\)|accepted\(|Acceptance|Report|PLAN_ONLY|CHANGES_.*false|TOUCHES_.*false' "$RUNTIME_DIR" 2>/dev/null || true
  echo

  echo "## active behavior suspicion keywords"
  rg -n 'write_|read_|load_|save_|render|draw|input|button|footer|title|TITLES|TITLEMAP|reader|restore|progress|sd|fat|display' "$RUNTIME_DIR" 2>/dev/null || true
  echo

  echo "marker=phase40l=x4-runtime-phase-scaffolding-prune-plan-ok"
} | tee "$OUT"

echo "Wrote: $OUT"
