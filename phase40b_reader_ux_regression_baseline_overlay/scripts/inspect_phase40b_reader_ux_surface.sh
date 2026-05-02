#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase40b-reader-ux-surface.txt}"

{
  echo "# Phase 40B Reader UX Surface Inspection"
  echo
  echo "## app modules"
  find vendor/pulp-os/src/apps -maxdepth 2 -type f -name '*.rs' | sort | sed 's#^#- #'
  echo
  echo "## Home / Files / Reader entry points"
  rg -n 'struct .*Home|impl .*Home|Bookshelf|Books|Files|Library|struct .*Files|impl .*Files|struct .*Reader|impl .*Reader|ReaderApp|open|back|Back|Select|Stay|Footer|footer|button|Button' \
    vendor/pulp-os/src/apps target-xteink-x4/src 2>/dev/null || true
  echo
  echo "## active reader persistence and restore touchpoints"
  rg -n 'typed_state_wiring|persist_progress_records|persist_theme_preset|persist_meta_record|persist_bookmarks|persist_bookmarks_index|restore|progress|theme|bookmark|recent' \
    vendor/pulp-os/src/apps/reader target-xteink-x4/src/vaachak_x4/runtime 2>/dev/null || true
  echo
  echo "## EPUB title/display candidates"
  rg -n 'title|display|label|long|short|epub|EPU|TITLES|RECENT|file_name|filename' \
    vendor/pulp-os/src/apps vendor/pulp-os/kernel/src target-xteink-x4/src/vaachak_x4/runtime 2>/dev/null || true
  echo
  echo "marker=phase40b=x4-reader-ux-regression-baseline-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"
