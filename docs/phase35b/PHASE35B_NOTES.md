# Phase 35B Notes

## Why This Phase Exists

Phase 35A created the storage state IO seam, but it was not connected to active runtime.

Phase 35B connects it safely as a path-only/no-op preflight.

Phase 35B wires a Vaachak-owned storage state IO seam into active runtime as a
path-only/no-op preflight.

Normal boot remains `vaachak=x4-runtime-ready` only.

`vendor/pulp-os` and `vendor/smol-epub` are untouched.

## Why Not Replace Persistence Yet

Persistence call sites live deeper in imported reader/kernel internals. Replacing them too early risks breaking:

```text
Continue
Bookmarks
Theme state
EPUB cache behavior
```

Phase 35B does not replace progress/bookmark/theme persistence.

Physical SD/SPI/FAT IO remains owned by the imported Pulp runtime.

## Future Work

Recommended next steps:

```text
Phase 35C — Read-only shadow probe for state files
Phase 35D — Feature-gated Vaachak state IO backend
Phase 36 — Input semantic active adoption
```
