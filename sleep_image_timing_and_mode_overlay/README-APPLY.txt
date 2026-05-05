Apply from the vaachak-os repository root:

  unzip -o sleep_image_timing_and_mode_overlay.zip
  chmod +x sleep_image_timing_and_mode_overlay/scripts/*.sh
  ./sleep_image_timing_and_mode_overlay/scripts/apply_sleep_image_timing_and_mode.sh .
  ./sleep_image_timing_and_mode_overlay/scripts/audit_sleep_image_timing_and_mode.sh .

Validate:

  cargo fmt --all --check
  cargo check --workspace --target riscv32imc-unknown-none-elf
  cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings
  cargo test -p vaachak-core --all-targets
  cargo test -p hal-xteink-x4 --all-targets
  cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
  ./scripts/check_no_milestone_artifacts.sh .

Optional SD mode setup:

  scripts/write_sleep_image_mode.sh /Volumes/SD_CARD daily
  scripts/verify_sleep_image_mode.sh /Volumes/SD_CARD
