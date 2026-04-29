VaachakOS Bootstrap Phase 6 — X4 Storage HAL Smoke

Apply from repo root:

  cd /home/mindseye73/Documents/projects/vaachak-os
  unzip -o /path/to/vaachak-os-bootstrap-phase6-x4-storage-hal-smoke.zip

Run host checks:

  cargo fmt --all
  cargo check --workspace --all-targets
  cargo test --workspace --all-targets
  cargo clippy --workspace --all-targets -- -D warnings

Build and flash X4:

  . "$HOME/export-esp.sh"
  cargo run -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf

Expected serial markers:

  phase6: sd init start
  phase6: sd mounted volume=0 root=open
  phase6: wrote state/VOSMOKE.TXT
  phase6: readback ok state/VOSMOKE.TXT
  phase6=x4-storage-hal-smoke-ok

The screen is not expected to change. This phase proves SD/FAT persistence only.
