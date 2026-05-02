# Phase 35C — Progress State I/O Adapter Overlay

Archive: `phase35c_progress_state_io_adapter_overlay.zip`

Packaging note: this refreshed archive contains a top-level `phase35c_progress_state_io_adapter_overlay/` directory and includes the missing apply script.

Purpose:

- Add a Vaachak-owned progress-state I/O adapter boundary.
- Keep storage hardware / SD / FAT behavior outside this phase.
- Keep the state-path convention 8.3-safe: `state/<BOOKID>.PRG`.
- Add a boot/check marker: `phase35c=x4-progress-state-io-adapter-ok`.

Overlay contents:

```text
replaceable/
  target-xteink-x4/src/vaachak_x4/state/progress_state_io_adapter.rs
  target-xteink-x4/src/vaachak_x4/state/mod.rs
snippets/
  add_to_vaachak_x4_mod.rs.snippet
scripts/
  apply_phase35c_progress_state_io_adapter.sh
  check_phase35c_progress_state_io_adapter.sh
docs/
  phase35c-progress-state-io-adapter-overlay.md
```

Recommended application:

```bash
unzip -o phase35c_progress_state_io_adapter_overlay.zip
chmod +x phase35c_progress_state_io_adapter_overlay/scripts/*.sh
./phase35c_progress_state_io_adapter_overlay/scripts/apply_phase35c_progress_state_io_adapter.sh
cargo fmt --all
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

The apply script is non-destructive for existing module files: it copies the new adapter, creates `state/mod.rs` only if missing, and appends missing `pub mod` exports when needed.

Notes:

- `state/mod.rs` is intentionally tiny. If your local `state/mod.rs` already exists, do not overwrite it blindly; merge only:

```rust
pub mod progress_state_io_adapter;
```

- The adapter is pure Rust. It introduces no SD, SPI, display, input, or hardware behavior.
- The concrete runtime storage layer should implement `ProgressStateIo` later or in a separate phase.
