Apply from the vaachak-os repository root:

  unzip -o daily_mantra_direct_sleep_loader_overlay.zip
  chmod +x daily_mantra_direct_sleep_loader_overlay/scripts/*.sh
  ./daily_mantra_direct_sleep_loader_overlay/scripts/apply_daily_mantra_direct_sleep_loader.sh .
  ./daily_mantra_direct_sleep_loader_overlay/scripts/audit_daily_mantra_direct_sleep_loader.sh .

Validate:

  cargo fmt --all --check
  cargo check --workspace --target riscv32imc-unknown-none-elf
  cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings
  cargo test -p vaachak-core --all-targets
  cargo test -p hal-xteink-x4 --all-targets
  cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
  ./scripts/check_no_milestone_artifacts.sh .

Prepare SD card:

  scripts/write_daily_mantra_today_file.sh /Volumes/SD_CARD
  scripts/verify_daily_mantra_direct_sleep_files.sh /Volumes/SD_CARD

Then eject, insert into X4, flash, and test sleep.
