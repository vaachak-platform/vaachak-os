Apply from the vaachak-os repository root:

  unzip -o sleep_image_serial_timing_repair_overlay.zip
  chmod +x sleep_image_serial_timing_repair_overlay/scripts/*.sh

  ./sleep_image_serial_timing_repair_overlay/scripts/apply_sleep_image_serial_timing_repair.sh .
  ./sleep_image_serial_timing_repair_overlay/scripts/audit_sleep_image_serial_timing_repair.sh .

Then validate and flash:

  cargo fmt --all --check
  cargo check --workspace --target riscv32imc-unknown-none-elf
  cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings
  cargo test -p vaachak-core --all-targets
  cargo test -p hal-xteink-x4 --all-targets
  cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
  ./scripts/check_no_milestone_artifacts.sh .

After flashing, set /SLPMODE.TXT to daily/off/text/static and trigger sleep while watching the serial monitor.
