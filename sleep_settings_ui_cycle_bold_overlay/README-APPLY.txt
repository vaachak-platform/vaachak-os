Apply from the vaachak-os repo root:

  unzip -o sleep_settings_ui_cycle_bold_overlay.zip
  chmod +x sleep_settings_ui_cycle_bold_overlay/scripts/*.sh
  ./sleep_settings_ui_cycle_bold_overlay/scripts/apply_sleep_settings_ui_cycle_bold.sh .
  ./sleep_settings_ui_cycle_bold_overlay/scripts/audit_sleep_settings_ui_cycle_bold.sh .

Validate:

  cargo fmt --all --check
  cargo check --workspace --target riscv32imc-unknown-none-elf
  cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings
  cargo test -p vaachak-core --all-targets
  cargo test -p hal-xteink-x4 --all-targets
  cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
  ./scripts/check_no_milestone_artifacts.sh .
