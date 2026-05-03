#!/usr/bin/env bash
set -euo pipefail

SD="${SD:-/media/mindseye73/SD_CARD}"
OUT="${OUT:-/tmp/phase40h-repair1-titles-bin-seed-inspection.txt}"

{
  echo "# Phase 40H Repair 1 TITLES.BIN Seed Inspection"
  echo
  echo "## TITLEMAP"
  if [ -f "$SD/_X4/TITLEMAP.TSV" ]; then
    wc -l "$SD/_X4/TITLEMAP.TSV"
    sed -n '1,120p' "$SD/_X4/TITLEMAP.TSV"
  else
    echo "missing=$SD/_X4/TITLEMAP.TSV"
  fi
  echo
  echo "## TITLES.BIN"
  if [ -f "$SD/_X4/TITLES.BIN" ]; then
    strings -a "$SD/_X4/TITLES.BIN" | sed -n '1,220p'
  else
    echo "missing=$SD/_X4/TITLES.BIN"
  fi
  echo
  echo "## TXT alias/title checks"
  if [ -f "$SD/_X4/TITLES.BIN" ]; then
    strings -a "$SD/_X4/TITLES.BIN" | rg -n 'POIROT~|THEMUR~|THESIG~|Poirot Investigates|Roger Ackroyd|Sign of the Four' || true
  fi
  echo
  echo "## bad cached phrases"
  if [ -f "$SD/_X4/TITLES.BIN" ]; then
    strings -a "$SD/_X4/TITLES.BIN" | rg -n 'most other parts|world at no cost|Project Gutenberg|produced by|transcribed by' || true
  fi
  echo
  echo "marker=phase40h-repair1=x4-seed-txt-titlemap-into-titles-bin-ok"
} | tee "$OUT"

echo "Wrote: $OUT"
