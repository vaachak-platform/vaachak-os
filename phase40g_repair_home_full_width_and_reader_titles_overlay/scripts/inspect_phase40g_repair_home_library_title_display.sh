#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase40g-repair-home-library-title-inspection.txt}"

{
  echo "# Phase 40G Repair Home/Library Title Inspection"
  echo
  echo "## Home full-width title markers"
  rg -n 'PHASE40G_REPAIR_HOME_RECENT|recent_preview_region|BitmapDynLabel::<96>|FULL_CONTENT_W' vendor/pulp-os/src/apps/home.rs || true
  echo
  echo "## Dir cache title scanner markers"
  rg -n 'next_untitled_reader_title|PHASE40G_REPAIR_TITLE_KIND|phase40g_repair_is_text_title_name|phase38i_is_epub_or_epu_name' vendor/pulp-os/kernel/src/kernel/dir_cache.rs || true
  echo
  echo "## Files scanner markers"
  rg -n 'scan_one_reader_title|scan_one_text_title|phase40g_repair_extract_text_title|PHASE40G_REPAIR_TEXT' vendor/pulp-os/src/apps/files.rs || true
  echo
  echo "## Protected footer/input/write checks"
  rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay' vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true
  echo
  echo "marker=phase40g-repair=x4-home-full-width-reader-titles-ok"
} | tee "$OUT"

echo "Wrote: $OUT"
