# Dictionary Runtime Current Path

The active Dictionary implementation uses the canonical SD Lua app path:

```text
/VAACHAK/APPS/DICT/APP.TOM
/VAACHAK/APPS/DICT/MAIN.LUA
/VAACHAK/APPS/DICT/INDEX.TXT
/VAACHAK/APPS/DICT/DATA/<PREFIX>.JSN
```

## Active files

```text
target-xteink-x4/src/vaachak_x4/lua/dictionary.rs
target-xteink-x4/src/vaachak_x4/lua/tool_stub_script.rs
target-xteink-x4/src/vaachak_x4/apps/home.rs
examples/sd-card/VAACHAK/APPS/DICT
```

## Repository hygiene

Old dictionary overlay folders, zips, generated docs, and generated validators should not be committed. Dictionary validation now uses the production repository/build checks.

## Validation

```bash
cargo fmt --all
./scripts/check_repo_hygiene.sh
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf --features lua-vm
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf --features lua-vm
```

Expected runtime marker on the Dictionary screen:

```text
vaachak-lua-dictionary-pack-integrity-ok
```
