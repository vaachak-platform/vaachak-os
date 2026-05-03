#!/usr/bin/env bash
set -euo pipefail

SD="${SD:-/media/mindseye73/SD_CARD}"
OUT="${OUT:-/tmp/phase40i-title-cache-workflow-inspection.txt}"

bad_phrase_count=0
txt_title_lines=0
epub_title_lines=0
titlemap_lines=0

if [ -f "$SD/_X4/TITLEMAP.TSV" ]; then
  titlemap_lines="$(wc -l < "$SD/_X4/TITLEMAP.TSV" | tr -d ' ')"
fi

if [ -f "$SD/_X4/TITLES.BIN" ]; then
  bad_phrase_count="$((strings -a "$SD/_X4/TITLES.BIN" | rg -n 'most other parts|world at no cost|Project Gutenberg|produced by|transcribed by|START OF THE PROJECT GUTENBERG' || true) | wc -l | tr -d ' ')"
  txt_title_lines="$((strings -a "$SD/_X4/TITLES.BIN" | rg -n '\.TXT|\.MD|POIROT~|THEMUR~|THESIG~|Poirot Investigates|Roger Ackroyd|Sign of the Four' || true) | wc -l | tr -d ' ')"
  epub_title_lines="$((strings -a "$SD/_X4/TITLES.BIN" | rg -n '\.EPU|\.EPUB|Alice|Dracula|Sherlock|Baskervilles' || true) | wc -l | tr -d ' ')"
fi

{
  echo "# Phase 40I Title Cache Workflow Inspection"
  echo "sd=$SD"
  echo "titlemap_present=$([ -f "$SD/_X4/TITLEMAP.TSV" ] && echo yes || echo no)"
  echo "titlemap_lines=$titlemap_lines"
  echo "titles_bin_present=$([ -f "$SD/_X4/TITLES.BIN" ] && echo yes || echo no)"
  echo "txt_title_lines=$txt_title_lines"
  echo "epub_title_lines=$epub_title_lines"
  echo "bad_phrase_count=$bad_phrase_count"
  echo "marker=phase40i=x4-title-cache-workflow-freeze-ok"
  echo
  echo "## TITLEMAP sample"
  if [ -f "$SD/_X4/TITLEMAP.TSV" ]; then
    sed -n '1,80p' "$SD/_X4/TITLEMAP.TSV"
  else
    echo "missing=$SD/_X4/TITLEMAP.TSV"
  fi
  echo
  echo "## TITLES.BIN strings"
  if [ -f "$SD/_X4/TITLES.BIN" ]; then
    strings -a "$SD/_X4/TITLES.BIN" | sed -n '1,220p'
  else
    echo "missing=$SD/_X4/TITLES.BIN"
  fi
} | tee "$OUT"

echo "Wrote: $OUT"
