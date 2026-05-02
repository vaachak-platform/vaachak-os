# Phase 35B — Storage State IO Runtime Wiring

## Purpose

Phase 35B wires the Vaachak-owned storage state IO seam into the active runtime as a safe bridge.

This is not a persistence takeover.

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
