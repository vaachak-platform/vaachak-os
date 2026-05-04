#!/usr/bin/env bash
set -euo pipefail

OUT="/tmp/phase39h-reader-save-callsites.txt"

{
  echo "# Phase 39H Reader/App Save Callsite Candidates"
  echo
  echo "## progress/theme/bookmark/state save candidates"
  rg -n --hidden --glob '!target/**' --glob '!*.zip' \
    'progress|bookmark|theme|metadata|save|persist|write|BKMK|RECENT|SETTINGS|TITLES|PRG|THM|MTA|BKM|BMIDX|STATE' \
    vendor/pulp-os/src vendor/pulp-os/kernel target-xteink-x4/src 2>/dev/null || true
  echo
  echo "## likely reader runtime functions"
  rg -n --hidden --glob '!target/**' --glob '!*.zip' \
    'fn .*save|fn .*persist|fn .*bookmark|fn .*theme|fn .*progress|struct .*Reader|impl .*Reader|KernelHandle' \
    vendor/pulp-os/src vendor/pulp-os/kernel target-xteink-x4/src 2>/dev/null || true
} | tee "$OUT"

echo
echo "Wrote: $OUT"
