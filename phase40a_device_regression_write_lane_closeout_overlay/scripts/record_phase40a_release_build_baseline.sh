#!/usr/bin/env bash
set -euo pipefail

OUT_DIR="${OUT_DIR:-/tmp/phase40a-release-build-baseline}"
mkdir -p "$OUT_DIR"

run_and_log() {
  local name="$1"
  shift
  echo "Running $name..."
  "$@" 2>&1 | tee "$OUT_DIR/$name.log"
}

run_and_log cargo-fmt cargo fmt --all
run_and_log cargo-test-core cargo test -p vaachak-core --all-targets
run_and_log cargo-check-hal cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf
run_and_log cargo-check-target cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
run_and_log cargo-clippy-hal cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
run_and_log cargo-clippy-target cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
run_and_log cargo-build-release cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf

cat > "$OUT_DIR/summary.txt" <<EOF
# Phase 40A Release Build Baseline

status=ACCEPTED
cargo_fmt=passed
cargo_test_core=passed
cargo_check_hal=passed
cargo_check_target=passed
cargo_clippy_hal=passed
cargo_clippy_target=passed
cargo_build_release=passed
marker=phase40a=x4-device-regression-write-lane-closeout-ok
EOF

cat "$OUT_DIR/summary.txt"
echo
echo "Wrote: $OUT_DIR"
