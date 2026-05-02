#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase40d-footer-label-rendering-inspection.txt}"
PATCHED_LIST="${PATCHED_LIST:-/tmp/phase40d-patched-files.txt}"

{
  echo "# Phase 40D Footer Label Rendering Inspection"
  echo
  echo "## patched files"
  if [ -f "$PATCHED_LIST" ]; then
    cat "$PATCHED_LIST"
  else
    echo "no patched-files list found"
  fi
  echo
  echo "## expected footer labels"
  rg -n 'Back.*Select.*Open.*Stay|Back.*Select.*open.*Stay|\["Back", "Select", "Open", "Stay"\]|\[b"Back", b"Select", b"Open", b"Stay"\]' \
    vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true
  echo
  echo "## forbidden old footer orders"
  rg -n 'Select.*Open.*Back.*Stay|Select.*open.*Back.*Stay|\["Select", "Open", "Back", "Stay"\]|\["Select", "open", "Back", "Stay"\]' \
    vendor/pulp-os/src/apps hal-xteink-x4/src/display_smoke.rs target-xteink-x4/src 2>/dev/null || true
  echo
  echo "marker=phase40d=x4-footer-button-label-rendering-patch-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"
