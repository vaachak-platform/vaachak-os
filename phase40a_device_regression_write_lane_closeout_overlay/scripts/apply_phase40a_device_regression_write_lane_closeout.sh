#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase40a_device_regression_write_lane_closeout_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_post_cleanup_runtime_surface_acceptance;' "$RUNTIME_MOD"; then
  echo "Phase 39P export missing; apply Phase 39P first." >&2
  exit 1
fi

if [ ! -f "$ROOT/vendor/pulp-os/src/apps/reader/typed_state_wiring.rs" ]; then
  echo "accepted typed_state_wiring helper missing; Phase 40A blocked." >&2
  exit 1
fi

mkdir -p "$RUNTIME_DIR"

for file in state_io_device_regression_write_lane_closeout.rs state_io_device_regression_write_lane_closeout_acceptance.rs; do
  SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/$file"
  DST="$RUNTIME_DIR/$file"
  if [ ! -f "$SRC" ]; then
    echo "missing overlay source: $SRC" >&2
    exit 1
  fi
  cp -v "$SRC" "$DST"
done

for export in \
  "pub mod state_io_device_regression_write_lane_closeout;" \
  "pub mod state_io_device_regression_write_lane_closeout_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
    echo "added $export to $RUNTIME_MOD"
  else
    echo "$export already present in $RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase40a_device_regression_write_lane_closeout.sh"

echo "phase40a=x4-device-regression-write-lane-closeout-ok"
echo "phase40a-acceptance=x4-device-regression-write-lane-closeout-report-ok"
echo "Phase 40A overlay applied."
echo ""
echo "Closeout flow:"
echo "  ./phase40a_device_regression_write_lane_closeout_overlay/scripts/record_phase40a_release_build_baseline.sh"
echo "  ./phase40a_device_regression_write_lane_closeout_overlay/scripts/print_phase40a_flash_commands.sh"
echo "  ./phase40a_device_regression_write_lane_closeout_overlay/scripts/inspect_phase40a_runtime_exports.sh"
echo "  SD=/media/mindseye73/C0D2-109E ./phase40a_device_regression_write_lane_closeout_overlay/scripts/inspect_phase40a_sd_persistence.sh"
echo "  SD=/media/mindseye73/C0D2-109E ./phase40a_device_regression_write_lane_closeout_overlay/scripts/capture_phase40a_sd_state_snapshot.sh"
echo "  DEVICE_REGRESSION_CONFIRMED=1 RESTORE_VERIFIED=1 SD=/media/mindseye73/C0D2-109E ./phase40a_device_regression_write_lane_closeout_overlay/scripts/write_phase40a_device_regression_report.sh"
echo "  DEVICE_REGRESSION_CONFIRMED=1 RESTORE_VERIFIED=1 SD=/media/mindseye73/C0D2-109E ./phase40a_device_regression_write_lane_closeout_overlay/scripts/accept_phase40a_device_regression_write_lane_closeout.sh"
