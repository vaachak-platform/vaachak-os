Apply from the vaachak-os repository root:

  unzip -o sleep_settings_ui_overlay.zip
  chmod +x sleep_settings_ui_overlay/scripts/*.sh
  ./sleep_settings_ui_overlay/scripts/apply_sleep_settings_ui.sh .
  ./sleep_settings_ui_overlay/scripts/audit_sleep_settings_ui.sh .

Validate:

  cargo fmt --all --check
  cargo check --workspace --target riscv32imc-unknown-none-elf
  cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings
  cargo test -p vaachak-core --all-targets
  cargo test -p hal-xteink-x4 --all-targets
  cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
  ./scripts/check_no_milestone_artifacts.sh .

If the no-milestone guard scans the extracted overlay folder, remove it and rerun the guard:

  rm -rf sleep_settings_ui_overlay
  rm -f sleep_settings_ui_overlay.zip
  ./scripts/check_no_milestone_artifacts.sh .
