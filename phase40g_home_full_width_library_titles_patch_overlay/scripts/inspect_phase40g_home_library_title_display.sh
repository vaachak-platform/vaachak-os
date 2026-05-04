#!/usr/bin/env bash
set -euo pipefail
OUT="${OUT:-/tmp/phase40g-home-library-title-inspection.txt}"
PATCHED_LIST="${PATCHED_LIST:-/tmp/phase40g-patched-files.txt}"
{
  echo "# Phase 40G Home/Library Title Inspection"
  echo
  echo "## patched files"
  [ -f "$PATCHED_LIST" ] && cat "$PATCHED_LIST" || true
  echo
  echo "## markers"
  rg -n 'phase40g=x4-home-full-width-library-title-patch-ok|PHASE40G_HOME_RECENT_PREVIEW|next_untitled_reader_title|phase40g_extract_text_title|scan_one_reader_title' \
    vendor/pulp-os/src/apps/home.rs vendor/pulp-os/src/apps/files.rs vendor/pulp-os/kernel/src/kernel/dir_cache.rs target-xteink-x4/src/vaachak_x4/runtime 2>/dev/null || true
  echo
  echo "## forbidden old footer labels"
  rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay' \
    vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true
  echo "marker=phase40g=x4-home-full-width-library-title-patch-ok"
} | tee "$OUT"
