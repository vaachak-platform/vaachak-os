Apply from vaachak-os repository root:

  unzip -o sleep_bitmap_prefetch_cache_overlay.zip
  chmod +x sleep_bitmap_prefetch_cache_overlay/scripts/*.sh
  ./sleep_bitmap_prefetch_cache_overlay/scripts/apply_sleep_bitmap_prefetch_cache.sh .
  ./sleep_bitmap_prefetch_cache_overlay/scripts/audit_sleep_bitmap_prefetch_cache.sh .

Validate:

  cargo fmt --all --check
  cargo check --workspace --target riscv32imc-unknown-none-elf
  cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings
  cargo test -p vaachak-core --all-targets
  cargo test -p hal-xteink-x4 --all-targets
  cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
  ./scripts/check_no_milestone_artifacts.sh .
