# Phase 38D — File Explorer Display Name Binding Overlay

## Goal

Fix the Files screen display-name path so file entries can render full human-readable names, for example:

```text
Alice's Adventures in Wonderland
```

rather than short initials such as:

```text
Al
```

## Strategy

This phase adds a side-effect-free normalizer that the Files screen can use when choosing a row label.

Priority order:

```text
1. Metadata title, usually from .MTA
2. Long file name, if provided by the runtime/browser entry
3. FAT 8.3 stem, as fallback
4. Empty/fallback outcome
```

## What this phase does not do

```text
writes: disabled
SD/FAT behavior: not moved
SPI behavior: not moved
display behavior: not moved
input behavior: not moved
power behavior: not moved
```

## Expected marker

```text
phase38d=x4-file-explorer-display-name-binding-ok
```

## Reference acceptance

The module includes a reference check for:

```text
metadata title: Alice's Adventures in Wonderland
FAT 8.3 name:  ALICE~1.TXT
```

The expected rendered label is:

```text
Alice's Adventures in Wonderland
```
