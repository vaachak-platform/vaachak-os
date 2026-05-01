# Phase 29 — Storage Path Helpers

## Goal

Phase 29 is the first real behavior extraction after the Vaachak boundary contract smoke phases. It moves only pure, deterministic storage path/name helper logic into Vaachak-owned code.

It does **not** move physical SD/SPI/filesystem behavior away from the imported X4/Pulp runtime.

## Added module

```text
target-xteink-x4/src/runtime/storage_path_helpers.rs
```

The module owns pure helpers for:

```text
state/<BOOKID>.PRG
state/<BOOKID>.BKM
state/<BOOKID>.THM
state/<BOOKID>.MTA
state/BMIDX.TXT
```

## Behavior intentionally not moved

The following remain owned by `vendor/pulp-os`:

```text
SD card initialization
SPI bus ownership
shared SD/display SPI arbitration
filesystem open/read/write/close
EPUB cache IO
reader progress/bookmark persistence behavior
```

## Logging change

Phase 29 quiets earlier development-phase boot logging. The active boot path should emit only:

```text
phase29=x4-storage-path-helpers-ok
```

Earlier phase marker constants and helper methods may remain in code for compatibility, but the active boot path should not print them.
