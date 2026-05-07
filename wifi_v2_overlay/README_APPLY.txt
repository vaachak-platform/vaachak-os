Wi-Fi Transfer v2 — Chunked Folder Upload + Resume + USB option cleanup

Apply from the vaachak-os repository root:

  python3 scripts/apply_wifi_transfer_v2_cleanup_usb.py .
  cargo fmt --all

Validate:

  cargo fmt --all --check
  cargo check --workspace --target riscv32imc-unknown-none-elf
  cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings
  cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
  git diff --check

Expected after flashing:

  Apps > Tools:
  - File Browser
  - QR Generator

  USB Transfer should no longer be visible.

Use Wi-Fi Transfer v2:

  Apps > Network > Wi-Fi Transfer
  Open http://x4.local/

Browser workflow:

  1. Target folder: /FCACHE/15D1296A
  2. Select local folder: /tmp/FCACHE/15D1296A
  3. Chunk size: 1024
  4. Click Upload / Resume

Resume:

  If upload fails, keep the same target folder, select the same local folder again,
  and click Upload / Resume. The browser checks /v2/stat and resumes from the
  file size already present on SD.
