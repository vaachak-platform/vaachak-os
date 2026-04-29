VaachakOS Bootstrap Phase 9.1 — Library List Clippy Fix

This pack fixes the Phase 9 host/clippy failure caused by the temporary
library-smoke renderer exceeding Clippy's too_many_arguments threshold.

It intentionally keeps the Phase 9 runtime behavior unchanged.

Apply:

  cd /home/mindseye73/Documents/projects/vaachak-os
  unzip -o /path/to/vaachak-os-bootstrap-phase9.1-library-clippy-fix.zip

Validate:

  cargo fmt --all
  cargo check --workspace --all-targets
  cargo test --workspace --all-targets
  cargo clippy --workspace --all-targets -- -D warnings

Flash:

  . "$HOME/export-esp.sh"
  cargo run -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf

Expected marker:

  phase9=x4-library-list-smoke-ok
