#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase40f_library_title_layout_polish_patch_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"
UI_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/ui"
UI_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/ui.rs"

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_reader_ux_polish_candidate_plan;' "$RUNTIME_MOD"; then
  echo "Phase 40E export missing; apply Phase 40E first." >&2
  exit 1
fi

mkdir -p "$RUNTIME_DIR" "$UI_DIR"

for file in state_io_library_title_layout_polish_patch.rs state_io_library_title_layout_polish_patch_acceptance.rs; do
  SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/$file"
  DST="$RUNTIME_DIR/$file"
  if [ ! -f "$SRC" ]; then
    echo "missing overlay source: $SRC" >&2
    exit 1
  fi
  cp -v "$SRC" "$DST"
done

cp -v "$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/ui/library_title_layout.rs" "$UI_DIR/library_title_layout.rs"

if [ ! -f "$UI_MOD" ]; then
  cat > "$UI_MOD" <<'EOF'
pub mod library_title_layout;
EOF
else
  if ! grep -Fxq "pub mod library_title_layout;" "$UI_MOD"; then
    printf '\n%s\n' "pub mod library_title_layout;" >> "$UI_MOD"
    echo "added library_title_layout export to $UI_MOD"
  fi
fi

for export in \
  "pub mod state_io_library_title_layout_polish_patch;" \
  "pub mod state_io_library_title_layout_polish_patch_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
    echo "added $export to $RUNTIME_MOD"
  else
    echo "$export already present in $RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase40f_library_title_layout_polish_patch.sh"
"$OVERLAY/scripts/guard_phase40f_library_title_patch_scope.sh"

echo "phase40f=x4-library-title-layout-polish-patch-ok"
echo "phase40f-acceptance=x4-library-title-layout-polish-patch-report-ok"
echo "Phase 40F metadata/helper applied."
echo ""
echo "Next:"
echo "  ./phase40f_library_title_layout_polish_patch_overlay/scripts/patch_phase40f_library_title_layout.sh"
echo "  ./phase40f_library_title_layout_polish_patch_overlay/scripts/inspect_phase40f_library_title_layout.sh"
