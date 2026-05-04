#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase40e-reader-ux-source-inspection.txt}"

{
  echo "# Phase 40E Reader UX Source Inspection"
  echo
  echo "## Library / Files candidates"
  rg -n 'Files|Library|Book|title|Title|display|label|row|selected|selection|cursor|scroll|page|footer|Footer|Back|Select|Open|Stay' \
    vendor/pulp-os/src/apps/files.rs vendor/pulp-os/src/apps target-xteink-x4/src/vaachak_x4 2>/dev/null || true
  echo
  echo "## Reader candidates"
  rg -n 'Reader|page|Page|progress|restore|bookmark|theme|font|line|spacing|footer|Footer|header|status|title|draw|render' \
    vendor/pulp-os/src/apps/reader target-xteink-x4/src/vaachak_x4 2>/dev/null || true
  echo
  echo "## Rendering/typography candidates"
  rg -n 'font|Font|bitmap|draw_text|draw.*text|text.*draw|line_height|spacing|rect|fill|stroke|highlight' \
    vendor/pulp-os/src hal-xteink-x4/src target-xteink-x4/src/vaachak_x4 2>/dev/null || true
  echo
  echo "## Guarded accepted footer labels"
  rg -n 'Back.*Select.*Open.*Stay|\["Back", "Select", "Open", "Stay"\]' \
    vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true
  echo
  echo "marker=phase40e=x4-reader-ux-polish-candidate-plan-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"
