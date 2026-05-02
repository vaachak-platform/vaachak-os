#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase39p_post_cleanup_runtime_surface_acceptance_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_guarded_review_delete_later_removal_patch;' "$RUNTIME_MOD"; then
  echo "Phase 39O export missing; apply Phase 39O first." >&2
  exit 1
fi

if [ ! -f "$ROOT/vendor/pulp-os/src/apps/reader/typed_state_wiring.rs" ]; then
  echo "accepted typed_state_wiring helper missing; Phase 39P blocked." >&2
  exit 1
fi

mkdir -p "$RUNTIME_DIR"

for file in state_io_post_cleanup_runtime_surface_acceptance.rs state_io_post_cleanup_runtime_surface_acceptance_report.rs; do
  SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/$file"
  DST="$RUNTIME_DIR/$file"
  if [ ! -f "$SRC" ]; then
    echo "missing overlay source: $SRC" >&2
    exit 1
  fi
  cp -v "$SRC" "$DST"
done

for export in \
  "pub mod state_io_post_cleanup_runtime_surface_acceptance;" \
  "pub mod state_io_post_cleanup_runtime_surface_acceptance_report;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
    echo "added $export to $RUNTIME_MOD"
  else
    echo "$export already present in $RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase39p_post_cleanup_runtime_surface_acceptance.sh"

echo "phase39p=x4-post-cleanup-runtime-surface-acceptance-ok"
echo "phase39p-acceptance=x4-post-cleanup-runtime-surface-report-ok"
echo "Phase 39P overlay applied."
echo ""
echo "Acceptance flow:"
echo "  ./phase39p_post_cleanup_runtime_surface_acceptance_overlay/scripts/guard_phase39p_accepted_write_path.sh"
echo "  ./phase39p_post_cleanup_runtime_surface_acceptance_overlay/scripts/inspect_phase39p_runtime_surface.sh"
echo "  ./phase39p_post_cleanup_runtime_surface_acceptance_overlay/scripts/record_phase39p_build_baseline.sh"
echo "  ./phase39p_post_cleanup_runtime_surface_acceptance_overlay/scripts/accept_phase39p_post_cleanup_runtime_surface.sh"
