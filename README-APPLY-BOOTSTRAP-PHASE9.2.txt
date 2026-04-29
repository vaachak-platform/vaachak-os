VaachakOS Bootstrap Phase 9.2 — Library List Export Fix

This replacement-file pack fixes the embedded compile error where target-xteink-x4 imports:

  hal_xteink_x4::{LibraryListItem, X4_LIBRARY_MAX_ITEMS, X4Input, X4Ssd1677Smoke}

but hal-xteink-x4 did not re-export LibraryListItem and X4_LIBRARY_MAX_ITEMS from display_smoke.

Files replaced:

  hal-xteink-x4/src/lib.rs

Apply from the vaachak-os repository root:

  unzip -o /path/to/vaachak-os-bootstrap-phase9.2-library-export-fix.zip

Validate:

  cargo fmt --all
  cargo check --workspace --all-targets
  cargo test --workspace --all-targets
  cargo clippy --workspace --all-targets -- -D warnings

Flash:

  . "$HOME/export-esp.sh"
  cargo run -p target-xteink-x4 \
    --release \
    --target riscv32imc-unknown-none-elf

Expected marker:

  phase9=x4-library-list-smoke-ready
  phase9=x4-library-list-smoke-ok
