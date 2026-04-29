VaachakOS Bootstrap Phase 4.4 — Serial Direct Boot Smoke

Apply from repo root:

  unzip -o /path/to/vaachak-os-bootstrap-phase4.4-serial-direct-boot-smoke.zip

Run host checks:

  cargo fmt --all
  cargo check --workspace --all-targets
  cargo test --workspace --all-targets
  cargo clippy --workspace --all-targets -- -D warnings

Build and flash X4:

  . "$HOME/export-esp.sh"

  cargo run -p target-xteink-x4 \
    --release \
    --target riscv32imc-unknown-none-elf

Important: the ePaper panel will still show the previous Home screen because
Phase 4 intentionally does not initialize or refresh the display. Validate this
phase by serial output only. Look for:

  phase4.4=serial-direct-print-ok
