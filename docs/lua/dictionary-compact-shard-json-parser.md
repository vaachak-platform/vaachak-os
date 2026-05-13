# Dictionary Compact Shard JSON Parser

The Dictionary screen loads the canonical `/VAACHAK/APPS/DICT` app and supports compact splitter-generated shard values.

Runtime marker:

```text
vaachak-lua-dictionary-pack-integrity-ok
```

Supported rich schema:

```json
{
  "WORD": {
    "MEANINGS": [["Verb", "definition", [], []]],
    "SYNONYMS": [],
    "ANTONYMS": []
  }
}
```

Supported compact schema:

```json
{
  "GOOD": [
    {"def": "Possessing desirable qualities...", "pos": ""},
    {"def": "Possessing moral excellence...", "pos": ""}
  ]
}
```

## Current behavior

- Streaming `INDEX.TXT` is preserved; no large fixed allocation is required.
- Deep shards resolve through `INDEX.TXT`, such as `GOO.JSN` and `DIC.JSN`.
- Native parser supports rich object schema with `MEANINGS`, compact array schema with `{ "def": ..., "pos": ... }`, and string definitions.
- Empty-state help text points to `DATA/*.JSN`.

## Validation

```bash
cargo fmt --all
./scripts/check_repo_hygiene.sh
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf --features lua-vm
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf --features lua-vm
```

After flashing, Dictionary should show the pack-integrity marker and load shard results from SD when the matching dictionary files are present.
