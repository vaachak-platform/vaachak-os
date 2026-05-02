#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase39m_safe_scaffolding_archive_patch_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_post_freeze_scaffolding_cleanup_plan;' "$RUNTIME_MOD"; then
  echo "Phase 39L export missing; apply Phase 39L first." >&2
  exit 1
fi

mkdir -p "$RUNTIME_DIR"

for file in state_io_safe_scaffolding_archive_patch.rs state_io_safe_scaffolding_archive_patch_acceptance.rs; do
  SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/$file"
  DST="$RUNTIME_DIR/$file"
  if [ ! -f "$SRC" ]; then
    echo "missing overlay source: $SRC" >&2
    exit 1
  fi
  cp -v "$SRC" "$DST"
done

for export in \
  "pub mod state_io_safe_scaffolding_archive_patch;" \
  "pub mod state_io_safe_scaffolding_archive_patch_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
    echo "added $export to $RUNTIME_MOD"
  else
    echo "$export already present in $RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase39m_safe_scaffolding_archive_patch.sh"

echo "phase39m=x4-safe-scaffolding-archive-patch-ok"
echo "phase39m-acceptance=x4-safe-scaffolding-archive-patch-report-ok"
echo "Phase 39M metadata applied."
echo ""
echo "To archive review-archive candidates:"
echo "  ./phase39m_safe_scaffolding_archive_patch_overlay/scripts/apply_phase39m_archive_runtime_scaffolding.sh"
echo ""
echo "Then run:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
