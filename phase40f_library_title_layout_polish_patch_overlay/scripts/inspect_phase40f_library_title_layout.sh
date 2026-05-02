#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase40f-library-title-layout-inspection.txt}"
PATCHED_LIST="${PATCHED_LIST:-/tmp/phase40f-patched-files.txt}"

{
  echo "# Phase 40F Library Title Layout Inspection"
  echo
  echo "## patched files"
  if [ -f "$PATCHED_LIST" ]; then
    cat "$PATCHED_LIST"
  else
    echo "no patched-files list found"
  fi
  echo
  echo "## Phase 40F markers"
  rg -n 'phase40f=x4-library-title-layout-polish-patch-ok|phase40f-helper=x4-library-title-layout-helper-ok' \
    vendor/pulp-os/src/apps/files.rs target-xteink-x4/src 2>/dev/null || true
  echo
  echo "## Library title/layout candidates"
  rg -n 'Files|Library|title|Title|display|Display|label|Label|row|Row|selected|selection|MAX.*TITLE|TITLE.*MAX|truncate|ellipsis|\.\.\.' \
    vendor/pulp-os/src/apps/files.rs target-xteink-x4/src/vaachak_x4/ui target-xteink-x4/src/vaachak_x4/runtime 2>/dev/null || true
  echo
  echo "## Guarded footer labels"
  rg -n 'Back.*Select.*Open.*Stay|\["Back", "Select", "Open", "Stay"\]' \
    vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true
  echo
  echo "## Forbidden old footer labels"
  rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay' \
    vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true
  echo
  echo "marker=phase40f=x4-library-title-layout-polish-patch-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"
