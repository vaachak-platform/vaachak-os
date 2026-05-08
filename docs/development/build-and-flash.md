# X4 Build and Flash Notes

The repository has two relevant embedded build paths:

1. `target-xteink-x4`: the root workspace integration target.
2. `vendor/pulp-os`: the active Pulp-derived runtime that currently owns the working device behavior.

Both should build before an on-device reader smoke run.

## Build/static gate

From repo root:

```bash
./scripts/validate_on_device_reader_smoke.sh
```

This runs formatting, cleanup, static reader chrome checks, workspace checks, clippy for the embedded target, the `target-xteink-x4` release build, and the active Pulp-derived runtime release build.

## Flash target-xteink-x4

From repo root:

```bash
cargo run -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

The package-local `target-xteink-x4/build.rs` supplies the linker script only for the embedded target. Do not add `-Tlinkall.x` back to the root Cargo config because it leaks into nested builds.

## Flash active Pulp-derived runtime

From repo root:

```bash
cd vendor/pulp-os
cargo run --release
```

Use this path when validating the currently working reader behavior if the root integration target is not the flashed product for a given run.

## On-device acceptance

After flashing, validate:

- `YEARLY_H.TXT` opens with `Prep Pg ...`.
- mixed EPUB smoke opens with `Prep Pg ...`.
- no `err:OPEN` or temporary debug-only text appears on successful reads.
- a deliberately missing cache shows `Read cache:<BOOKID> err:<CODE>`.
- Back returns to Library/Home without state corruption.
- progress restore works for prepared TXT and EPUB.
- Reader settings still apply.
- Wi-Fi Transfer still shows Original Transfer and Chunked Resume.

# Build and Flash

The active X4 firmware path is the Pulp-derived runtime.

## Repository gate

```bash
cargo fmt --all --check
./scripts/check_no_milestone_artifacts.sh .
cargo check -p target-xteink-x4
cargo check --workspace --all-targets
cargo check --workspace --target riscv32imc-unknown-none-elf
cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

## Active firmware build

```bash
cd vendor/pulp-os
cargo build --release
```

## System apps closure gate

```bash
./scripts/validate_system_apps_closure.sh
```

Flash the active firmware build after the gate passes, then verify Wi-Fi Transfer, Date & Time, and Settings on the X4.
