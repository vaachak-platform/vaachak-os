#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase39l_post_freeze_scaffolding_cleanup_plan_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_write_lane_cleanup_acceptance_freeze;' "$RUNTIME_MOD"; then
  echo "Phase 39K export missing; apply Phase 39K first." >&2
  exit 1
fi

if [ ! -f "$ROOT/vendor/pulp-os/src/apps/reader/typed_state_wiring.rs" ]; then
  echo "Phase 39I typed_state_wiring helper missing; cleanup plan blocked." >&2
  exit 1
fi

mkdir -p "$RUNTIME_DIR"

for file in state_io_post_freeze_scaffolding_cleanup_plan.rs state_io_post_freeze_scaffolding_cleanup_plan_acceptance.rs; do
  SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/$file"
  DST="$RUNTIME_DIR/$file"
  if [ ! -f "$SRC" ]; then
    echo "missing overlay source: $SRC" >&2
    exit 1
  fi
  cp -v "$SRC" "$DST"
done

for export in \
  "pub mod state_io_post_freeze_scaffolding_cleanup_plan;" \
  "pub mod state_io_post_freeze_scaffolding_cleanup_plan_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
    echo "added $export to $RUNTIME_MOD"
  else
    echo "$export already present in $RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase39l_post_freeze_scaffolding_cleanup_plan.sh"

echo "phase39l=x4-post-freeze-scaffolding-cleanup-plan-ok"
echo "phase39l-acceptance=x4-post-freeze-scaffolding-cleanup-plan-report-ok"
echo "Phase 39L overlay applied. Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo ""
echo "Plan review:"
echo "  ./phase39l_post_freeze_scaffolding_cleanup_plan_overlay/scripts/guard_phase39l_accepted_write_path.sh"
echo "  ./phase39l_post_freeze_scaffolding_cleanup_plan_overlay/scripts/plan_phase39l_scaffolding_cleanup.sh"
echo "  ./phase39l_post_freeze_scaffolding_cleanup_plan_overlay/scripts/accept_phase39l_cleanup_plan.sh"
