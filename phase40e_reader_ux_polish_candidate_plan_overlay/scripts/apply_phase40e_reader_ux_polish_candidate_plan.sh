#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase40e_reader_ux_polish_candidate_plan_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_footer_button_label_rendering_patch;' "$RUNTIME_MOD"; then
  echo "Phase 40D export missing; apply Phase 40D first." >&2
  exit 1
fi

mkdir -p "$RUNTIME_DIR"

for file in state_io_reader_ux_polish_candidate_plan.rs state_io_reader_ux_polish_candidate_plan_acceptance.rs; do
  SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/$file"
  DST="$RUNTIME_DIR/$file"
  if [ ! -f "$SRC" ]; then
    echo "missing overlay source: $SRC" >&2
    exit 1
  fi
  cp -v "$SRC" "$DST"
done

for export in \
  "pub mod state_io_reader_ux_polish_candidate_plan;" \
  "pub mod state_io_reader_ux_polish_candidate_plan_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
    echo "added $export to $RUNTIME_MOD"
  else
    echo "$export already present in $RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase40e_reader_ux_polish_candidate_plan.sh"
"$OVERLAY/scripts/guard_phase40e_reader_ux_polish_scope.sh"

echo "phase40e=x4-reader-ux-polish-candidate-plan-ok"
echo "phase40e-acceptance=x4-reader-ux-polish-candidate-plan-report-ok"
echo "Phase 40E overlay applied."
echo ""
echo "Recommended validation:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo ""
echo "Plan flow:"
echo "  ./phase40e_reader_ux_polish_candidate_plan_overlay/scripts/inspect_phase40e_reader_ux_sources.sh"
echo "  ./phase40e_reader_ux_polish_candidate_plan_overlay/scripts/write_phase40e_polish_candidate_backlog.sh"
echo "  ./phase40e_reader_ux_polish_candidate_plan_overlay/scripts/plan_phase40e_reader_ux_polish_candidates.sh"
echo "  ./phase40e_reader_ux_polish_candidate_plan_overlay/scripts/accept_phase40e_reader_ux_polish_candidate_plan.sh"
