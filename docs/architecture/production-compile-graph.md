# Vaachak OS production compile graph

Status: production cleanup checkpoint.

The active target compile graph keeps only runtime-facing Vaachak modules and removes slice-era migration checkpoint code from `target-xteink-x4`.

Production-facing target modules retained:

- boot marker and imported X4 runtime entrypoint
- runtime-facing contracts for storage, input, display, and state
- `physical::spi_bus_runtime` bridge used by the imported runtime
- reader/library/UI/state/text/sleep/time modules

Removed from active compile graph:

- hardware migration smoke contracts
- final-acceptance marker-only modules
- hardware runtime executor/backend takeover scaffolding
- transition owner/bridge/read-only/fallback checkpoint modules
- one-off validate/apply/patch scripts from generated deliverables

Use production validation instead of slice validators:

```bash
cargo fmt --all
./scripts/check_repo_hygiene.sh
cargo test -p vaachak-core --all-targets
cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```
