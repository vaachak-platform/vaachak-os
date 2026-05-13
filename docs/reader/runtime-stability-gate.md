# Reader Runtime Stability Gate

This gate validates the current reader-first X4 runtime.

## Scope

- Validate prepared TXT open path.
- Validate mixed EPUB smoke path.
- Keep successful prepared-cache chrome clean.
- Keep cache-failure diagnostics only on failure.
- Confirm progress restore.
- Confirm Back returns to Library/Home.
- Confirm Reader settings still apply.
- Confirm Wi-Fi Transfer can still upload large `/FCACHE/<BOOKID>` folders.

## Static gate

```bash
cargo fmt --all
./scripts/check_repo_hygiene.sh
cargo test -p vaachak-core --all-targets
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

## Device gate

Use `docs/reader/on-device-reader-smoke.md` after flashing.

## Notes

The root Cargo config intentionally avoids adding embedded linker flags globally. Keep embedded linker setup package-local so nested or host-side builds are not polluted by parent Cargo config.
