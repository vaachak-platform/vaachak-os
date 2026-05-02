#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase40d_footer_button_label_rendering_patch_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_footer_button_label_baseline_fix_plan;' "$RUNTIME_MOD"; then
  echo "Phase 40C export missing; apply Phase 40C first." >&2
  exit 1
fi

mkdir -p "$RUNTIME_DIR"

for file in state_io_footer_button_label_rendering_patch.rs state_io_footer_button_label_rendering_patch_acceptance.rs; do
  SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/$file"
  DST="$RUNTIME_DIR/$file"
  if [ ! -f "$SRC" ]; then
    echo "missing overlay source: $SRC" >&2
    exit 1
  fi
  cp -v "$SRC" "$DST"
done

for export in \
  "pub mod state_io_footer_button_label_rendering_patch;" \
  "pub mod state_io_footer_button_label_rendering_patch_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
    echo "added $export to $RUNTIME_MOD"
  else
    echo "$export already present in $RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase40d_footer_button_label_rendering_patch.sh"
"$OVERLAY/scripts/guard_phase40d_footer_patch_scope.sh"

echo "phase40d=x4-footer-button-label-rendering-patch-ok"
echo "phase40d-acceptance=x4-footer-button-label-rendering-patch-report-ok"
echo "Phase 40D metadata applied."
echo ""
echo "Next:"
echo "  ./phase40d_footer_button_label_rendering_patch_overlay/scripts/patch_phase40d_footer_label_rendering.sh"
echo "  ./phase40d_footer_button_label_rendering_patch_overlay/scripts/inspect_phase40d_footer_label_rendering.sh"
