VaachakOS Bootstrap Phase 8.4 — X4 calibrated ADC input path fix

This fix replaces target-xteink-x4/src/main.rs only.

Why:
Phase 8.3 normalized esp-hal AdcCalCurve readings into a synthetic 0..4095 range.
The working x4-reader-os-rs input path uses AdcCalCurve readings directly with
thresholds around:
  row1: 3, 1113, 1984, 2556
  row2: 3, 1659

Your logs show the VaachakOS target is seeing those same calibrated values:
  idle: row1=row2 ~= 2968
  row2: 5 and 1653
  row1: 4, 1116, 1991, 2567

So this patch removes normalization and feeds calibrated values directly into
hal-xteink-x4::X4Input.

Apply:
  unzip -o vaachak-os-bootstrap-phase8.4-x4-calibrated-adc-input-fix.zip

Validate:
  cargo fmt --all
  cargo check --workspace --all-targets
  cargo test --workspace --all-targets
  cargo clippy --workspace --all-targets -- -D warnings

Flash:
  . "$HOME/export-esp.sh"
  cargo run -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf

Expected serial:
  phase8.4: calibrated ADC idle row1=... row2=... model=direct-x4-reader-os-rs
  phase8: input event #1 button=Down kind=Press
  phase8: redraw selected=1 item=Library
  phase8=x4-input-navigation-smoke-ok
