# Phase 17 Manifest Notes

## Root workspace

The root `Cargo.toml` must exclude vendored workspaces so Cargo does not detect multiple workspace roots:

```toml
[workspace]
exclude = ["vendor/pulp-os", "vendor/smol-epub"]
```

The exact location of `exclude` inside the `[workspace]` table is not important, but it must be inside the `[workspace]` section.

## Target dependencies

The target crate should depend on the vendored Pulp package by aliasing package `x4-os` as dependency key `pulp-os`:

```toml
pulp-os = { package = "x4-os", path = "../vendor/pulp-os" }
x4-kernel = { path = "../vendor/pulp-os/kernel" }
smol-epub = { path = "../vendor/smol-epub" }
```

This allows Rust code to use the crate name:

```rust
pulp_os::...
```

while Cargo resolves the vendored package named:

```text
x4-os
```

## Invalid dependency shape to avoid

Do not mix direct dependency values with dotted workspace keys:

```toml
log = "0.4"
log.workspace = true
```

Use only one style:

```toml
log = "0.4"
```

or:

```toml
log = { workspace = true }
```
