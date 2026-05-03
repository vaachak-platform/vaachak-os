#!/usr/bin/env bash
set -euo pipefail

SD="${SD:-/media/mindseye73/C0D2-109E}"
OUT="${OUT:-/tmp/phase40h-host-title-map-inspection.txt}"

{
  echo "# Phase 40H Host Title Map Inspection"
  echo
  echo "## source markers"
  rg -n 'phase40h|PHASE40H_TITLE_MAP_FILE|phase40h_load_host_title_map' vendor/pulp-os/kernel/src/kernel/dir_cache.rs target-xteink-x4/src/vaachak_x4/runtime 2>/dev/null || true
  echo
  echo "## TITLEMAP.TSV"
  if [ -f "$SD/_X4/TITLEMAP.TSV" ]; then
    echo "titlemap=$SD/_X4/TITLEMAP.TSV"
    wc -l "$SD/_X4/TITLEMAP.TSV"
    sed -n '1,80p' "$SD/_X4/TITLEMAP.TSV"
  else
    echo "missing=$SD/_X4/TITLEMAP.TSV"
  fi
  echo
  echo "## bad cached phrases"
  if [ -f "$SD/_X4/TITLES.BIN" ]; then
    strings -a "$SD/_X4/TITLES.BIN" | rg -n 'most other parts|world at no cost|Project Gutenberg|produced by|transcribed by' || true
  else
    echo "no active $SD/_X4/TITLES.BIN"
  fi
  echo
  echo "marker=phase40h=x4-host-title-map-txt-display-names-ok"
} | tee "$OUT"

echo "Wrote: $OUT"
