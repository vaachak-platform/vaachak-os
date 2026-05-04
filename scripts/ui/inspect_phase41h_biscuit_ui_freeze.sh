#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase41h-biscuit-ui-freeze-inspection.txt}"
SD="${SD:-/Volumes/NO NAME}"

{
  echo "# Phase 41H Biscuit UI Freeze Inspection"
  echo
  echo "## Home dashboard markers"
  rg -n 'phase41g|phase41h|Reader|Library|Bookmarks|Settings|Sync|Upload|Coming soon|draw.*card|dashboard|footer|ButtonFeedback|NextJump|PrevJump' vendor/pulp-os/src/apps/home.rs || true
  echo
  echo "## Files/Library title-cache protection markers"
  rg -n 'phase40g-repair3|phase40h-repair1|TITLEMAP|TITLES|body-title|scan_one_reader_title|scan_one_text_title' vendor/pulp-os/kernel/src/kernel/dir_cache.rs vendor/pulp-os/src/apps/files.rs || true
  echo
  echo "## Reader restore / pagination protection markers"
  rg -n 'restore|progress|pagination|page|Back|reader_state|phase41e|PHASE41E' vendor/pulp-os/src/apps/reader/mod.rs 2>/dev/null || true
  echo
  echo "## Footer duplicate / old-order scan"
  echo "custom_home_footer_count=$((rg -n 'Back[[:space:]]+Select[[:space:]]+Left[[:space:]]+Right|Back Select Left Right' vendor/pulp-os/src/apps/home.rs 2>/dev/null || true) | wc -l | tr -d ' ')"
  echo "old_footer_order_count=$((rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay' vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true) | wc -l | tr -d ' ')"
  echo
  echo "## SD title-cache status"
  if [ -d "$SD" ]; then
    echo "sd=$SD"
    if [ -f "$SD/_X4/TITLEMAP.TSV" ]; then
      echo "titlemap_status=PRESENT"
      echo "titlemap_lines=$(wc -l < "$SD/_X4/TITLEMAP.TSV" | tr -d ' ')"
    else
      echo "titlemap_status=MISSING"
    fi
    if [ -f "$SD/_X4/TITLES.BIN" ]; then
      echo "titles_status=PRESENT"
      echo "bad_phrase_lines=$((strings -a "$SD/_X4/TITLES.BIN" | rg -n 'most other parts|world at no cost|Project Gutenberg|produced by|transcribed by|START OF THE PROJECT GUTENBERG' || true) | wc -l | tr -d ' ')"
      echo "txt_title_lines=$((strings -a "$SD/_X4/TITLES.BIN" | rg -n '\.TXT|\.MD|POIROT~|THEMUR~|THESIG~|Poirot Investigates|Roger Ackroyd|Sign of the Four' || true) | wc -l | tr -d ' ')"
    else
      echo "titles_status=MISSING"
    fi
  else
    echo "sd=$SD"
    echo "sd_status=MISSING_OR_NOT_MOUNTED"
  fi
  echo
  echo "marker=phase41h=x4-biscuit-ui-acceptance-freeze-ok"
} | tee "$OUT"

echo "Wrote: $OUT"
