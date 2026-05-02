# Phase 35C — Progress State I/O Adapter Overlay

## Goal

Add a narrow Vaachak-owned adapter for reading and writing per-book progress records without moving physical storage behavior.

This phase is intentionally an overlay. It defines the shape of progress persistence and delegates actual file access to a small trait.

## Boundary decision

Owned in Phase 35C:

- Progress record format.
- 8.3-safe progress path construction.
- `state/<BOOKID>.PRG` convention.
- Input/output adapter trait shape.
- Pure encode/decode tests.
- Boot/check marker.

Not owned in Phase 35C:

- SD-card bus access.
- SPI arbitration.
- FAT implementation.
- Book discovery.
- EPUB parsing.
- Reader pagination behavior.

## State file convention

```text
state/<BOOKID>.PRG
```

`<BOOKID>` must be 8 ASCII characters for 8.3 safety. If a candidate ID is not 8.3-safe, the adapter derives an uppercase FNV-1a based 8-character hex ID.

## Record format

The record is a small fixed binary record:

```text
magic[4]              = VPRG
version[1]            = 1
unit[1]               = 0 page index, 1 byte offset, 2 slice index
flags[2]              = reserved
book_id[8]            = ASCII 8.3-safe book id
page_index[4]         = little-endian u32
logical_offset[4]     = little-endian u32
updated_epoch_secs[8] = little-endian u64
checksum[4]           = additive checksum over preceding bytes
```

The current size is `36` bytes.

## Apply

```bash
unzip -o phase35c_progress_state_io_adapter_overlay.zip
chmod +x phase35c_progress_state_io_adapter_overlay/scripts/*.sh
./phase35c_progress_state_io_adapter_overlay/scripts/apply_phase35c_progress_state_io_adapter.sh
```

The refreshed archive intentionally extracts under `phase35c_progress_state_io_adapter_overlay/`.

## Acceptance checks

```bash
bash scripts/check_phase35c_progress_state_io_adapter.sh .
cargo fmt --all
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

Expected marker:

```text
phase35c=x4-progress-state-io-adapter-ok
```
