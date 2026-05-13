# Dictionary Index Streaming Loader Repair

This repair fixes the Dictionary crash caused by allocating a full `INDEX.TXT` buffer on ESP32-C3.

## Problem

The Dictionary pack can have a large index. The observed pack has about 4,426 index rows and an `INDEX.TXT` around 79 KiB. The previous firmware-side loader allocated a 96 KiB heap buffer before opening Dictionary. On X4 this can fail with:

```text
memory allocation of 98304 bytes failed
```

## Fix

The Dictionary loader now keeps the 96 KiB maximum-size guard, but it does not allocate a 96 KiB buffer. It reads `/VAACHAK/APPS/DICT/INDEX.TXT` using a fixed 512-byte buffer and streams each line into `DictionaryIndexResolver`.

The resolver preserves the existing shard selection behavior:

- `GOOD` resolves to `DATA/GOO.JSN` when present.
- `DICTIONARY` resolves to `DATA/DIC.JSN` when present.
- Browse queries such as `G` resolve to the best matching `G*` shard.
- Deep shards and numbered fallback shards remain supported.

## Active files

```text
target-xteink-x4/src/vaachak_x4/apps/home.rs
target-xteink-x4/src/vaachak_x4/lua/dictionary.rs
target-xteink-x4/src/vaachak_x4/x4_kernel/kernel/handle.rs
examples/sd-card/VAACHAK/APPS/DICT
```

## Removed / inactive path

The old Rust `vaachak_x4/dictionary` prefix reader remains removed. The active Dictionary app is the Lua tool under `/VAACHAK/APPS/DICT` plus the firmware-side canonical Dictionary support in `vaachak_x4/lua/dictionary.rs`.

## Expected marker

```text
vaachak-lua-dictionary-pack-integrity-ok
```
