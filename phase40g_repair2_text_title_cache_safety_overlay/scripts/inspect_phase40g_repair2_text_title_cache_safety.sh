#!/usr/bin/env bash
set -euo pipefail

SD="${SD:-/media/mindseye73/C0D2-109E}"
OUT="${OUT:-/tmp/phase40g-repair2-text-title-cache-inspection.txt}"

{
  echo "# Phase 40G Repair 2 Text Title Cache Inspection"
  echo
  echo "## source markers"
  rg -n 'phase40g-repair2|phase40g_repair_extract_text_title|Title:' vendor/pulp-os/src/apps/files.rs || true
  echo
  echo "## bad cached phrases"
  if [ -f "$SD/_X4/TITLES.BIN" ]; then
    strings -a "$SD/_X4/TITLES.BIN" | rg -n 'most other parts|world at no cost|Project Gutenberg|produced by|transcribed by' || true
  else
    echo "no active $SD/_X4/TITLES.BIN"
  fi
  echo
  echo "marker=phase40g-repair2=x4-text-title-cache-safety-ok"
} | tee "$OUT"

echo "Wrote: $OUT"
