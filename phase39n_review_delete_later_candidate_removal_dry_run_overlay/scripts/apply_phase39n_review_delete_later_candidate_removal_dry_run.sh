#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase39n_review_delete_later_candidate_removal_dry_run_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_safe_scaffolding_archive_patch;' "$RUNTIME_MOD"; then
  echo "Phase 39M export missing; apply Phase 39M first." >&2
  exit 1
fi

if [ ! -f "$ROOT/vendor/pulp-os/src/apps/reader/typed_state_wiring.rs" ]; then
  echo "Phase 39I typed_state_wiring helper missing; dry-run blocked." >&2
  exit 1
fi

mkdir -p "$RUNTIME_DIR"

for file in state_io_review_delete_later_removal_dry_run.rs state_io_review_delete_later_removal_dry_run_acceptance.rs; do
  SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/$file"
  DST="$RUNTIME_DIR/$file"
  if [ ! -f "$SRC" ]; then
    echo "missing overlay source: $SRC" >&2
    exit 1
  fi
  cp -v "$SRC" "$DST"
done

for export in \
  "pub mod state_io_review_delete_later_removal_dry_run;" \
  "pub mod state_io_review_delete_later_removal_dry_run_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
    echo "added $export to $RUNTIME_MOD"
  else
    echo "$export already present in $RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase39n_review_delete_later_removal_dry_run.sh"

echo "phase39n=x4-review-delete-later-candidate-removal-dry-run-ok"
echo "phase39n-acceptance=x4-review-delete-later-removal-dry-run-report-ok"
echo "Phase 39N overlay applied. Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo ""
echo "Dry-run plan:"
echo "  ./phase39n_review_delete_later_candidate_removal_dry_run_overlay/scripts/guard_phase39n_accepted_write_path.sh"
echo "  ./phase39n_review_delete_later_candidate_removal_dry_run_overlay/scripts/plan_phase39n_review_delete_later_removal_dry_run.sh"
echo "  ./phase39n_review_delete_later_candidate_removal_dry_run_overlay/scripts/accept_phase39n_review_delete_later_removal_dry_run.sh"
