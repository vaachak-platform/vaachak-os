#!/usr/bin/env bash
set -euo pipefail

SD="${SD:-/media/mindseye73/C0D2-109E}"
OUT="${OUT:-/tmp/phase40g-repair3-disable-txt-body-title-scanning-inspection.txt}"

{
  echo "# Phase 40G Repair 3 Inspection"
  echo
  echo "## source markers"
  rg -n 'phase40g-repair3|TXT/MD body-title scanning is disabled|next_untitled_reader_title|PHASE40G_REPAIR_TITLE_KIND_TEXT' vendor/pulp-os/kernel/src/kernel/dir_cache.rs || true
  echo
  echo "## bad cached phrases"
  if [ -f "$SD/_X4/TITLES.BIN" ]; then
    strings -a "$SD/_X4/TITLES.BIN" | rg -n 'most other parts|world at no cost|Project Gutenberg|produced by|transcribed by' || true
  else
    echo "no active $SD/_X4/TITLES.BIN"
  fi
  echo
  echo "marker=phase40g-repair3=x4-disable-txt-body-title-scanning-ok"
} | tee "$OUT"

echo "Wrote: $OUT"
