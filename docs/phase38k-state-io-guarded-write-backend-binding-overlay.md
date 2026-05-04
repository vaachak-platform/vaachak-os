# Phase 38K — State I/O Guarded Write Backend Binding Overlay

This phase starts the write/backend lane after the read-only and write-design phases.

It adds a guarded write backend binding scaffold with:

- typed record kinds for `.PRG`, `.THM`, `.MTA`, `.BKM`, and `BMIDX.TXT`
- write operation modeling
- write gate states
- request validation
- a denying backend implementation

This phase intentionally keeps writes disabled. It does not bind SD/FAT, does not call SPI, and does not perform filesystem mutation.

Expected marker:

```text
phase38k=x4-state-io-guarded-write-backend-binding-ok
```

Next lane: guarded backend implementation, then enabling one write kind at a time.
