VaachakOS Bootstrap Phase 9 — X4 Minimal Library List Smoke

Apply from repo root:

  cd /home/mindseye73/Documents/projects/vaachak-os
  unzip -o /path/to/vaachak-os-bootstrap-phase9-x4-library-list-smoke.zip

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

Expected serial markers:

  phase9=x4-library-list-smoke-ready
  phase9: input event #... button=Down kind=Press
  phase9: redraw selected=1 file=...
  phase9=x4-library-list-smoke-ok
  phase9: select file=... idx=... size=...

Expected behavior:

  - Scans /BOOKS first, root fallback if /BOOKS is missing or empty.
  - Shows up to five TXT/MD/EPU/EPUB files.
  - Up/Down/Left/Right move selection.
  - Select logs the selected file but does not open Reader yet.
