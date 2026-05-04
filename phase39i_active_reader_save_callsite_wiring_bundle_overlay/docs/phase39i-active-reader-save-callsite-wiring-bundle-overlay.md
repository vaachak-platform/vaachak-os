# Phase 39I — Active Reader Save Callsite Wiring Bundle Overlay

The Phase 39H locator showed the real active save callsites are inside:

```text
vendor/pulp-os/src/apps/reader/mod.rs
```

Phase 39I therefore adds a Pulp-local facade:

```text
vendor/pulp-os/src/apps/reader/typed_state_wiring.rs
```

and rewrites active reader calls from:

```rust
k.ensure_app_subdir(reader_state::STATE_DIR)
k.write_app_subdir(...)
```

to:

```rust
typed_state_wiring::ensure_state_dir(k)
typed_state_wiring::write_app_subdir(k, ...)
```

This wires everything at once while keeping the existing filesystem behavior.
