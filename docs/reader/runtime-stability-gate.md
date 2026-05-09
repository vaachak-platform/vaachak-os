# Reader Runtime Stability Gate

This gate validates the current reader-first X4 runtime without claiming that hardware drivers have been migrated out of the imported runtime.

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
cargo build
./scripts/validate_documentation_refresh.sh
```

## Device gate

Use `docs/reader/on-device-reader-smoke.md` after flashing.

## Notes

The root Cargo config intentionally avoids adding embedded linker flags globally. Keep embedded linker setup package-local so nested `vendor/pulp-os` builds are not polluted by parent Cargo config.
