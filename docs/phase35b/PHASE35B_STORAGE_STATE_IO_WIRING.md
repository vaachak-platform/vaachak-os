# Phase 35B — Storage State IO Runtime Wiring

## Purpose

Phase 35B wires the Vaachak-owned storage state IO seam into the active runtime as a safe bridge.

This is not a persistence takeover.

Phase 35B wires a Vaachak-owned storage state IO seam into active runtime as a
path-only/no-op preflight.

## Baseline

Normal boot marker remains:

```text
vaachak=x4-runtime-ready
```

## What Phase 35B Adds

```text
target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs
```

The runtime bridge should exercise:

```text
Progress
Bookmark
Theme
Metadata
```

through the existing Vaachak storage state seam and storage path helpers.

The active imported runtime wrapper calls:

```text
VaachakStorageStateRuntimeBridge::active_runtime_preflight()
```

The call is silent and returns a boolean that is intentionally discarded. It
does not print `phase35=`, `phase35b=`, or any other phase marker.

## Persistence Ownership

Phase 35B does not replace progress/bookmark/theme persistence.

Physical SD/SPI/FAT IO remains owned by the imported Pulp runtime.

## What Phase 35B Does Not Move

```text
SD initialization
SPI bus setup
filesystem open/read/write/close
progress/bookmark/theme file IO
EPUB cache IO
reader app internals
```

## Vendor Rule

These paths stay untouched:

```text
vendor/pulp-os/**
vendor/smol-epub/**
```

`vendor/pulp-os` and `vendor/smol-epub` are untouched.

Normal boot remains `vaachak=x4-runtime-ready` only.
