VaachakOS Bootstrap Phase 4.3 — Host Checks / Embedded Target Split

Apply from repo root:

  unzip -o /path/to/vaachak-os-bootstrap-phase4.3-host-embedded-target-split.zip

Then run host validation WITHOUT an explicit embedded target:

  cargo fmt --all
  cargo check --workspace --all-targets
  cargo test --workspace --all-targets
  cargo clippy --workspace --all-targets -- -D warnings

Then build embedded firmware WITH the explicit target:

  . "$HOME/export-esp.sh"

  cargo build -p target-xteink-x4 \
    --release \
    --target riscv32imc-unknown-none-elf

Then flash/monitor:

  cargo run -p target-xteink-x4 \
    --release \
    --target riscv32imc-unknown-none-elf

Why this fix exists:

  Phase 4.2 set the global Cargo build target to riscv32imc-unknown-none-elf.
  That made cargo check/test/clippy try to compile test binaries for a no_std
  bare-metal target, which fails with:

    can't find crate for `test`
    no global memory allocator found
    #[panic_handler] function required

  Phase 4.3 removes the global target and keeps only target-specific runner and
  linker flags for explicit ESP32-C3 builds.
