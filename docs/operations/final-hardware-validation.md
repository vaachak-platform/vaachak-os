# Final Hardware Validation

## Repository validation

```bash
cargo fmt --all
./scripts/check_repo_hygiene.sh
cargo test -p vaachak-core --all-targets
cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

## Flash

```bash
./scripts/flash_x4_vaachak_app0.sh /dev/cu.usbmodemXXXX
```

## Device smoke checklist

Validate on the Xteink X4:

```text
- boot completes
- display initializes
- Home/category dashboard renders
- full refresh works
- partial/list refresh works
- all buttons respond correctly
- Files opens
- SD root listing works
- TXT opens
- EPUB opens
- progress/state/cache files work
- bookmarks work where supported
- Back navigation works
- Settings opens and persists reader-visible settings
- Wi-Fi Networks opens without locking input
- Wi-Fi Transfer opens and returns safely
- Date & Time shows Live, Cached, or Unsynced without locking input
- optional Lua apps under /VAACHAK/APPS open where sample files are present
- no FAT/path/cluster-chain errors appear
```
