Vaachak OS transfer + prepared reader diagnostic fix overlay

What this replaces:
- vendor/pulp-os/src/apps/reader/mod.rs
- vendor/pulp-os/src/apps/reader/prepared_txt.rs
- vendor/pulp-os/src/apps/upload.rs
- vendor/pulp-os/kernel/src/drivers/storage.rs
- vendor/pulp-os/assets/upload.html
- vendor/pulp-os/src/apps/home.rs

Scope:
- Adds prepared-cache error code display in Reader header.
- Keeps prepared TXT cache loading wired through PreparedTxtState::try_open().
- Shows errors such as MISSING, META, BOOK, INDEX, FONT_MISSING, FONT, PAGE, TOO_LARGE.
- Adds Wi-Fi Transfer page with two tabs:
  1. Original Transfer / SD Manager
  2. Chunked Resume for large /FCACHE/<BOOKID> uploads
- Adds /v2/stat, /v2/mkdir, and /v2/chunk endpoints.
- Adds nested file-size helpers for resumable cache uploads.
- Keeps USB Transfer hidden and suppresses its dead-code warning.

Apply:
  unzip -o vaachak_transfer_reader_fix_overlay.zip
  ./vaachak_transfer_reader_fix_overlay/scripts/apply_transfer_reader_fix.sh .

Validate:
  cargo fmt --all --check
  cargo check --workspace --target riscv32imc-unknown-none-elf
  cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings
  cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
  git diff --check

After flashing:
- Open YEARLY_H.TXT.
- If prepared cache opens: header should show Prep Pg ...
- If not: header should show Read cache:15D1296A err:<CODE> ...

Use Wi-Fi Transfer:
- Original Transfer tab: browse, upload, download, rename, create folder, delete.
- Chunked Resume tab: upload /FCACHE/15D1296A folder using 256-byte chunks and resume.
