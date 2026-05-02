#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase39o_guarded_review_delete_later_removal_patch_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_review_delete_later_removal_dry_run;' "$RUNTIME_MOD"; then
  echo "Phase 39N export missing; apply Phase 39N first." >&2
  exit 1
fi

if [ ! -f "$ROOT/vendor/pulp-os/src/apps/reader/typed_state_wiring.rs" ]; then
  echo "Phase 39I typed_state_wiring helper missing; removal blocked." >&2
  exit 1
fi

mkdir -p "$RUNTIME_DIR"

for file in state_io_guarded_review_delete_later_removal_patch.rs state_io_guarded_review_delete_later_removal_patch_acceptance.rs; do
  SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/$file"
  DST="$RUNTIME_DIR/$file"
  if [ ! -f "$SRC" ]; then
    echo "missing overlay source: $SRC" >&2
    exit 1
  fi
  cp -v "$SRC" "$DST"
done

for export in \
  "pub mod state_io_guarded_review_delete_later_removal_patch;" \
  "pub mod state_io_guarded_review_delete_later_removal_patch_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
    echo "added $export to $RUNTIME_MOD"
  else
    echo "$export already present in $RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase39o_guarded_review_delete_later_removal_patch.sh"

echo "phase39o=x4-guarded-review-delete-later-removal-patch-ok"
echo "phase39o-acceptance=x4-guarded-review-delete-later-removal-report-ok"
echo "Phase 39O metadata applied."
echo ""
echo "To perform guarded removal:"
echo "  ./phase39o_guarded_review_delete_later_removal_patch_overlay/scripts/apply_phase39o_remove_review_delete_later_candidates.sh"
