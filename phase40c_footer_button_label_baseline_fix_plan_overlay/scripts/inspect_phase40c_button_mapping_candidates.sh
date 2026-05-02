#!/usr/bin/env bash
set -euo pipefail

OUT="${OUT:-/tmp/phase40c-button-mapping-candidates.txt}"

{
  echo "# Phase 40C Button Mapping Candidate Inspection"
  echo
  echo "## input/button mapping candidates"
  rg -n 'Button|button|Input|input|Key|key|ADC|adc|GPIO|gpio|ladder|Left|Right|Up|Down|Back|Select|Open|Stay|Event|Action|Nav|navigate' \
    vendor/pulp-os/src vendor/pulp-os/kernel/src target-xteink-x4/src hal-xteink-x4/src core/src 2>/dev/null || true
  echo
  echo "## action/event enums"
  rg -n 'enum .*Button|enum .*Input|enum .*Key|enum .*Action|struct .*Button|struct .*Input|impl .*Button|impl .*Input|match .*Button|match .*Input' \
    vendor/pulp-os/src vendor/pulp-os/kernel/src target-xteink-x4/src hal-xteink-x4/src core/src 2>/dev/null || true
  echo
  echo "marker=phase40c=x4-footer-button-label-baseline-fix-plan-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"
