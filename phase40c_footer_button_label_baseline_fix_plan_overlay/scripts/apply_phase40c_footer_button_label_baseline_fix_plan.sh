#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase40c_footer_button_label_baseline_fix_plan_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_reader_ux_regression_baseline;' "$RUNTIME_MOD"; then
  echo "Phase 40B export missing; apply Phase 40B first." >&2
  exit 1
fi

mkdir -p "$RUNTIME_DIR"

for file in state_io_footer_button_label_baseline_fix_plan.rs state_io_footer_button_label_baseline_fix_plan_acceptance.rs; do
  SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/$file"
  DST="$RUNTIME_DIR/$file"
  if [ ! -f "$SRC" ]; then
    echo "missing overlay source: $SRC" >&2
    exit 1
  fi
  cp -v "$SRC" "$DST"
done

for export in \
  "pub mod state_io_footer_button_label_baseline_fix_plan;" \
  "pub mod state_io_footer_button_label_baseline_fix_plan_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
    echo "added $export to $RUNTIME_MOD"
  else
    echo "$export already present in $RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase40c_footer_button_label_baseline_fix_plan.sh"

echo "phase40c=x4-footer-button-label-baseline-fix-plan-ok"
echo "phase40c-acceptance=x4-footer-button-label-baseline-fix-plan-report-ok"
echo "Phase 40C overlay applied."
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
echo "  ./phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/guard_phase40c_reader_ux_baseline.sh"
echo "  ./phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/inspect_phase40c_footer_button_sources.sh"
echo "  ./phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/inspect_phase40c_button_mapping_candidates.sh"
echo "  EXPECTED_FILES_FOOTER='Back Select Open Stay' EXPECTED_READER_FOOTER='Back Select Open Stay' ./phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/write_phase40c_expected_footer_labels_baseline.sh"
echo "  ./phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/plan_phase40c_footer_button_label_fix.sh"
echo "  ./phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/accept_phase40c_footer_button_label_baseline_fix_plan.sh"
