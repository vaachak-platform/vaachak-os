#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
GUARD="/tmp/phase40e-reader-ux-polish-scope-guard.txt"
INSPECT="/tmp/phase40e-reader-ux-source-inspection.txt"
BACKLOG="/tmp/phase40e-polish-candidate-backlog.txt"
OUT="${OUT:-/tmp/phase40e-reader-ux-polish-candidate-plan.md}"

"$ROOT/phase40e_reader_ux_polish_candidate_plan_overlay/scripts/guard_phase40e_reader_ux_polish_scope.sh" >/dev/null

if [ ! -f "$INSPECT" ]; then
  "$ROOT/phase40e_reader_ux_polish_candidate_plan_overlay/scripts/inspect_phase40e_reader_ux_sources.sh" >/dev/null
fi

if [ ! -f "$BACKLOG" ]; then
  "$ROOT/phase40e_reader_ux_polish_candidate_plan_overlay/scripts/write_phase40e_polish_candidate_backlog.sh" >/dev/null
fi

candidate_files="$(rg -n 'Files|Library|title|Title|display|label|row|selected|selection' vendor/pulp-os/src/apps/files.rs vendor/pulp-os/src/apps target-xteink-x4/src/vaachak_x4 2>/dev/null \
  | sed -E 's/^([^:]+):.*/\1/' | sort -u | head -40 || true)"

{
  echo "# Phase 40E Reader UX Polish Candidate Plan"
  echo
  echo "Status: PLAN ONLY"
  echo
  echo "No UX behavior was changed in Phase 40E."
  echo
  echo "## Baseline preserved"
  echo
  echo "- Home -> Files/Library -> Reader -> Back -> Files/Library -> Reopen Reader"
  echo "- Footer labels: \`Back Select Open Stay\`"
  echo "- Input mapping unchanged"
  echo "- Write lane closed"
  echo "- Display geometry/rotation unchanged"
  echo
  echo "## Candidate backlog"
  echo
  cat "$BACKLOG"
  echo
  echo "## Recommended Phase 40F"
  echo
  echo "\`Phase 40F — Library Title Layout Polish Patch\`"
  echo
  echo "Scope:"
  echo
  echo "- Polish only Files/Library title row layout."
  echo "- Do not change title source or cache behavior."
  echo "- Do not change footer labels."
  echo "- Do not change input mapping."
  echo "- Do not change reader pagination."
  echo "- Add before/after device acceptance for Files/Library only."
  echo
  echo "## Candidate source files for Phase 40F"
  if [ -n "$candidate_files" ]; then
    echo "$candidate_files" | sed 's/^/- /'
  else
    echo "- vendor/pulp-os/src/apps/files.rs"
  fi
  echo
  echo "## Phase 40F acceptance criteria"
  echo
  echo "- Files/Library opens."
  echo "- EPUB titles remain correct."
  echo "- Selected row remains obvious."
  echo "- Row text is not clipped unexpectedly."
  echo "- Footer remains \`Back Select Open Stay\`."
  echo "- Reader open/back/restore still passes."
  echo "- No write-lane regression."
  echo
  echo "## Defer"
  echo
  echo "- Long title wrapping if it requires variable row height."
  echo "- Reader typography if it changes pagination."
  echo "- Any input ladder or ADC threshold work."
  echo
  echo "marker=phase40e=x4-reader-ux-polish-candidate-plan-ok"
} | tee "$OUT"

echo
echo "Wrote: $OUT"
