#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase40k-detect-phase-runtime-scaffolding.txt}"

{
  echo "# Phase Runtime Scaffolding Detection"
  echo
  echo "## runtime files containing phase markers"
  rg -l 'phase3|phase4|PHASE_3|PHASE_4' target-xteink-x4/src/vaachak_x4/runtime 2>/dev/null | sort || true
  echo
  echo "## runtime exports containing phase/state_io markers"
  rg -n 'state_io_.*phase|state_io_.*repair|state_io_.*freeze|state_io_.*acceptance' target-xteink-x4/src/vaachak_x4/runtime.rs 2>/dev/null || true
  echo
  echo "marker=phase40k=x4-repository-cleanup-new-device-deploy-baseline-ok"
} | tee "$OUT"

echo "Wrote: $OUT"
echo "This script does not delete runtime files. Review before any prune."
