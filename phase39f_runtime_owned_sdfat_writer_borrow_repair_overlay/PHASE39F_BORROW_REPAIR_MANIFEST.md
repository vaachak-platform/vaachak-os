# Phase 39F Borrow Repair Overlay

Repairs Phase 39F borrow-checker errors:

- `cannot borrow *self.ops as mutable more than once at a time`
- affected methods:
  - `write_direct`
  - `write_atomic`

Expected marker:
- phase39f-borrow-repair=x4-runtime-owned-sdfat-writer-borrow-repair-ok
