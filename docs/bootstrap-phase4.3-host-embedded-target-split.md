# VaachakOS Bootstrap Phase 4.3 — Host Checks / Embedded Target Split

Phase 4.2 made the X4 target an embedded firmware crate, but the Cargo config accidentally made `riscv32imc-unknown-none-elf` the global default target.

That caused normal workspace checks to compile unit tests for the embedded target:

```text
can't find crate for `test`
no global memory allocator found
#[panic_handler] function required
```

That is expected when `cargo test` is run for a `no_std` bare-metal target.

## Policy

Use host target for CI/dev checks:

```bash
cargo fmt --all
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

Use embedded target only for X4 firmware build/flash:

```bash
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
cargo run   -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

## What changed

`.cargo/config.toml` no longer has:

```toml
[build]
target = "riscv32imc-unknown-none-elf"
```

It keeps only the target-specific runner and linker flags:

```toml
[target.riscv32imc-unknown-none-elf]
runner = "espflash flash --monitor --chip esp32c3"
rustflags = [
  "-C", "link-arg=-Tlinkall.x",
  "-C", "force-frame-pointers",
]
```

This preserves the ESP32-C3 linker setup while keeping host tests useful.
