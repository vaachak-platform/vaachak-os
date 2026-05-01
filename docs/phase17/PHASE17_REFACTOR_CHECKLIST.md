# Phase 17 Reader Refactor Checklist

## Repo hygiene

- [ ] Old Phase 15B overlay folder is moved out of the repo root.
- [ ] Old Phase 16 overlay folder is moved out of the repo root.
- [ ] Compatibility symlink `phase15b_overlay` is removed if present.
- [ ] Old `target-xteink-x4/src/main.rs.phase15a-backup.*` files are moved out of the active source directory.
- [ ] Old `target-xteink-x4/src/main.rs.bak-phase*` files are moved out of the active source directory.
- [ ] `.gitignore` ignores future phase backup files.

## Manifest hygiene

- [ ] Root `Cargo.toml` excludes `vendor/pulp-os`.
- [ ] Root `Cargo.toml` excludes `vendor/smol-epub`.
- [ ] `target-xteink-x4/Cargo.toml` uses the `pulp-os` alias for the vendored `x4-os` package.
- [ ] `target-xteink-x4/Cargo.toml` has a direct `x4-kernel` path dependency.
- [ ] `target-xteink-x4/Cargo.toml` has a direct `smol-epub` path dependency.
- [ ] No dependency is represented both as `foo = "..."` and `foo.workspace = true`.

## Runtime boundary

- [ ] `target-xteink-x4/src/main.rs` tracks `vendor/pulp-os/src/bin/main.rs`.
- [ ] Allowed target-main differences are limited to crate alias and phase markers.
- [ ] `vendor/pulp-os/src/apps/reader/*` is not modified in Phase 17.
- [ ] `vendor/pulp-os/kernel/*` is not modified in Phase 17.
- [ ] `vendor/smol-epub/*` is not modified in Phase 17.

## Reader parity preservation

- [ ] TXT opens.
- [ ] EPUB opens using real rendered text.
- [ ] EPUB does not show raw `PK`/ZIP bytes.
- [ ] Back returns to library/files.
- [ ] Continue works.
- [ ] TXT progress works.
- [ ] EPUB progress works.
- [ ] TXT bookmarks work.
- [ ] EPUB bookmarks work.
- [ ] Theme/menu/footer behavior is unchanged from Phase 16.

## Required checks

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_reader_runtime_sync.sh
./scripts/check_phase17_reader_refactor.sh
```

## Expected marker

```text
phase17=x4-reader-refactor-ok
```
