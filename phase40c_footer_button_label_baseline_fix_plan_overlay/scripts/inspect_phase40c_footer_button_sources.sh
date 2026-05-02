#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase40c-footer-button-sources.txt}"

{
  echo "# Phase 40C Footer/Button Source Inspection"
  echo
  echo "## footer/rendering candidates"
  rg -n 'footer|Footer|button|Button|label|Label|Back|Select|Open|Stay|Home|Files|Library|Reader|draw.*text|text.*draw|status|nav|navigation' \
    vendor/pulp-os/src target-xteink-x4/src hal-xteink-x4/src 2>/dev/null || true
  echo
  echo "## likely footer constants and strings"
  rg -n '"Back"|"Select"|"Open"|"Stay"|"Home"|"Files"|"Library"|"Reader"|"Cancel"|"OK"|"Next"|"Prev"' \
    vendor/pulp-os/src target-xteink-x4/src hal-xteink-x4/src 2>/dev/null || true
  echo
  echo "## files containing footer/button words"
  rg -l 'footer|Footer|button|Button|Back|Select|Open|Stay' \
    vendor/pulp-os/src target-xteink-x4/src hal-xteink-x4/src 2>/dev/null | sort || true
  echo
  echo "marker=phase40c=x4-footer-button-label-baseline-fix-plan-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"
