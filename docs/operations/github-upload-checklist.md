# GitHub Upload Checklist

## 1. Confirm generated artifacts are gone

```bash
find . -maxdepth 1 \( -name '*.zip' -o -name '*_repair' -o -name '*_restore' -o -name '*_cleanup' -o -name '*_contract' -o -name '*_reset' \) -print
find scripts -maxdepth 1 -type f \( -name 'patch_*' -o -name 'apply_*' -o -name 'cleanup_*' \) -print
find . \( -name '__pycache__' -o -name '*.pyc' -o -name '__MACOSX' -o -name '.DS_Store' \) -not -path './.git/*' -print
```

Expected output: no files.

## 2. Run repository hygiene

```bash
./scripts/check_repo_hygiene.sh
```

## 3. Validate build state

```bash
cargo fmt --all
cargo test -p vaachak-core --all-targets
cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

For partition-sensitive changes, also run:

```bash
./scripts/validate_x4_standard_partition_table_compatibility.sh
./scripts/validate_x4_flash_ota_slot_policy.sh
```

## 4. Flash and smoke test

```bash
./scripts/flash_x4_vaachak_app0.sh /dev/cu.usbmodemXXXX
```

Use `docs/operations/final-hardware-validation.md` for the device checklist.

## 5. Review git status

```bash
git status --short
```

Expected categories:

- source changes for accepted functionality
- current-state docs
- production scripts only
- no generated zip/folder/script artifacts

## 6. Commit and push

```bash
git add README.md SCOPE.md ROADMAP.md docs scripts core hal-xteink-x4 target-xteink-x4 support examples tools partitions Cargo.toml Cargo.lock rust-toolchain.toml espflash.toml .github .cargo .gitignore AGENTS.md
git commit -m "Clean Vaachak OS repository artifacts and refresh docs"
git push
```

## Notes

Do not delete `vendor/pulp-os` in this cleanup checkpoint unless a separate dependency-removal audit confirms it is safe. Do not add new functionality there.
