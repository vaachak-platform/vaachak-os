# Dictionary Canonical App Loader

The firmware-side Dictionary tool loads from the canonical Lua app path:

```text
/VAACHAK/APPS/DICT/APP.TOM
/VAACHAK/APPS/DICT/MAIN.LUA
/VAACHAK/APPS/DICT/INDEX.TXT
/VAACHAK/APPS/DICT/DATA/<PREFIX>.JSN
```

The dictionary path supports splitter output with prefix shards such as `GOO.JSN`, `GALA.JSN`, or numbered shards such as `GRAN1.JSN`.

## Runtime behavior

- Tool manifest and script reads use `/VAACHAK/APPS/<APP>/...` directly.
- Dictionary startup reads `INDEX.TXT` into a bounded heap buffer.
- The shard resolver chooses the best prefix shard from `INDEX.TXT` before reading `DATA/*.JSN`.
- Typing letters, `DEL`, `CLR`, `GO`, or `*` reloads the selected shard when needed.
- `*` is treated as a prefix-search action, not as a literal query character.
- The screen marker is `vaachak-lua-dictionary-pack-integrity-ok`.

## Validation

```bash
cargo fmt --all
./scripts/check_repo_hygiene.sh
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf --features lua-vm
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf --features lua-vm
```

Then copy the firmware to the X4 and make sure the SD card contains the canonical `DICT` folder above.
