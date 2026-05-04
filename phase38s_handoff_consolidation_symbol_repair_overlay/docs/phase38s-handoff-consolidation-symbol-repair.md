# Phase 38S Handoff Consolidation Symbol Repair

The original Phase 38S handoff consolidation referenced:

```rust
phase38c_live_writes_enabled
```

But the actual Phase 38C function is:

```rust
phase38c_writes_enabled
```

This repair patches the import and call site. It also adds a local dead-code allow to the Phase 38I `.EPU` / `.EPUB` helper in `dir_cache.rs` to remove the remaining warning.
