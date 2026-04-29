VaachakOS Bootstrap Phase 7.2 — Minimal Home Test/Clippy Fix

Apply from repo root:

  unzip -o /path/to/vaachak-os-bootstrap-phase7.2-minimal-home-test-clippy-fix.zip

Then run:

  cargo fmt --all
  cargo check --workspace --all-targets
  cargo test --workspace --all-targets
  cargo clippy --workspace --all-targets -- -D warnings

This replacement fixes:
- Clippy redundant closure in render_smoke_strip().
- A brittle Home status pixel assertion that pointed at a blank glyph pixel.
