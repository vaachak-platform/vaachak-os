#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase40b_reader_ux_regression_baseline_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"

if [ ! -f "$RUNTIME_MOD" ]; then
  echo "missing runtime module file: $RUNTIME_MOD" >&2
  exit 1
fi

if ! grep -q 'pub mod state_io_device_regression_write_lane_closeout;' "$RUNTIME_MOD"; then
  echo "Phase 40A export missing; apply Phase 40A first." >&2
  exit 1
fi

if [ ! -f "$ROOT/vendor/pulp-os/src/apps/reader/typed_state_wiring.rs" ]; then
  echo "accepted typed_state_wiring helper missing; Phase 40B blocked." >&2
  exit 1
fi

mkdir -p "$RUNTIME_DIR"

for file in state_io_reader_ux_regression_baseline.rs state_io_reader_ux_regression_baseline_acceptance.rs; do
  SRC="$OVERLAY/replaceable/target-xteink-x4/src/vaachak_x4/runtime/$file"
  DST="$RUNTIME_DIR/$file"
  if [ ! -f "$SRC" ]; then
    echo "missing overlay source: $SRC" >&2
    exit 1
  fi
  cp -v "$SRC" "$DST"
done

for export in \
  "pub mod state_io_reader_ux_regression_baseline;" \
  "pub mod state_io_reader_ux_regression_baseline_acceptance;"
do
  if ! grep -Fxq "$export" "$RUNTIME_MOD"; then
    printf '\n%s\n' "$export" >> "$RUNTIME_MOD"
    echo "added $export to $RUNTIME_MOD"
  else
    echo "$export already present in $RUNTIME_MOD"
  fi
done

"$OVERLAY/scripts/check_phase40b_reader_ux_regression_baseline.sh"

echo "phase40b=x4-reader-ux-regression-baseline-ok"
echo "phase40b-acceptance=x4-reader-ux-regression-baseline-report-ok"
echo "Phase 40B overlay applied."
echo ""
echo "Recommended validation:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo ""
echo "Baseline flow:"
echo "  ./phase40b_reader_ux_regression_baseline_overlay/scripts/guard_phase40b_write_lane_closed.sh"
echo "  ./phase40b_reader_ux_regression_baseline_overlay/scripts/inspect_phase40b_reader_ux_surface.sh"
echo "  SD=/media/mindseye73/C0D2-109E ./phase40b_reader_ux_regression_baseline_overlay/scripts/inspect_phase40b_epub_title_baseline.sh"
echo "  HOME_FILES_READER_CONFIRMED=1 FOOTER_LABELS_CONFIRMED=1 EPUB_TITLES_CONFIRMED=1 READER_RESTORE_CONFIRMED=1 NO_CRASH_REBOOT=1 ./phase40b_reader_ux_regression_baseline_overlay/scripts/write_phase40b_manual_device_ux_report.sh"
echo "  HOME_FILES_READER_CONFIRMED=1 FOOTER_LABELS_CONFIRMED=1 EPUB_TITLES_CONFIRMED=1 READER_RESTORE_CONFIRMED=1 NO_CRASH_REBOOT=1 SD=/media/mindseye73/C0D2-109E ./phase40b_reader_ux_regression_baseline_overlay/scripts/accept_phase40b_reader_ux_regression_baseline.sh"
