#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
GUARD="/tmp/phase40c-reader-ux-baseline-guard.txt"
FOOTER="/tmp/phase40c-footer-button-sources.txt"
MAPPING="/tmp/phase40c-button-mapping-candidates.txt"
EXPECTED="/tmp/phase40c-expected-footer-labels-baseline.txt"
OUT="${OUT:-/tmp/phase40c-footer-button-label-fix-plan.md}"

"$ROOT/phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/guard_phase40c_reader_ux_baseline.sh" >/dev/null

if [ ! -f "$FOOTER" ]; then
  "$ROOT/phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/inspect_phase40c_footer_button_sources.sh" >/dev/null
fi

if [ ! -f "$MAPPING" ]; then
  "$ROOT/phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/inspect_phase40c_button_mapping_candidates.sh" >/dev/null
fi

if [ ! -f "$EXPECTED" ]; then
  "$ROOT/phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/write_phase40c_expected_footer_labels_baseline.sh" >/dev/null
fi

footer_files="$(grep -E 'vendor/pulp-os/src|target-xteink-x4/src|hal-xteink-x4/src' "$FOOTER" 2>/dev/null | sed -E 's/^([^:]+):.*/\1/' | sort -u | head -40 || true)"
mapping_files="$(grep -E 'vendor/pulp-os/src|vendor/pulp-os/kernel/src|target-xteink-x4/src|hal-xteink-x4/src|core/src' "$MAPPING" 2>/dev/null | sed -E 's/^([^:]+):.*/\1/' | sort -u | head -40 || true)"

{
  echo "# Phase 40C Footer/Button Label Fix Plan"
  echo
  echo "Status: PLAN ONLY"
  echo
  echo "No source rendering/input behavior was changed in Phase 40C."
  echo
  echo "## Accepted baseline"
  echo
  cat "$EXPECTED"
  echo
  echo "## Candidate footer/rendering files"
  if [ -n "$footer_files" ]; then
    echo "$footer_files" | sed 's/^/- /'
  else
    echo "- No footer source candidates found by rg scan; inspect rendering modules manually."
  fi
  echo
  echo "## Candidate input/button mapping files"
  if [ -n "$mapping_files" ]; then
    echo "$mapping_files" | sed 's/^/- /'
  else
    echo "- No input mapping candidates found by rg scan; inspect HAL/input modules manually."
  fi
  echo
  echo "## Mismatch hypothesis"
  echo
  echo "The visible footer label order may be generated independently from the physical button-action order."
  echo "The fix should update labels at the screen footer rendering layer, not the physical input mapping, unless inspection proves mapping is wrong."
  echo
  echo "## Phase 40D exact patch plan"
  echo
  echo "1. Identify the single footer-label source used by Files/Library and Reader screens."
  echo "2. Add a small screen-local footer-label model if labels are currently hard-coded inline."
  echo "3. Set Files/Library footer order to:"
  echo "   \`Back Select Open Stay\`"
  echo "4. Set Reader footer order to:"
  echo "   \`Back Select Open Stay\`"
  echo "5. Do not change physical button mapping in Phase 40D unless the source scan proves label strings are correct and mapping is wrong."
  echo "6. Add an acceptance marker proving only footer label strings/layout changed."
  echo "7. Device-test Files/Library and Reader:"
  echo "   - label order matches physical behavior"
  echo "   - Back returns to previous screen"
  echo "   - Select changes selection"
  echo "   - Open opens selected file/book"
  echo "   - Stay does not unexpectedly navigate"
  echo "8. Re-run Phase 40B baseline acceptance after the patch."
  echo
  echo "## Guardrails"
  echo
  echo "- Do not touch \`vendor/pulp-os/src/apps/reader/typed_state_wiring.rs\`."
  echo "- Do not touch SD persistence."
  echo "- Do not touch archived Phase 38/39 scaffolding."
  echo "- Do not change display geometry/rotation."
  echo "- Do not change input ladder thresholds in this phase."
  echo
  echo "marker=phase40c=x4-footer-button-label-baseline-fix-plan-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"
