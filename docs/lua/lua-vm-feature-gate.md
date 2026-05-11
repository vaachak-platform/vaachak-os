# Lua VM Feature Gate

This slice introduces a disabled-by-default Lua VM feature gate for `target-xteink-x4`.

## Feature

```text
lua-vm
```

The feature is intentionally not part of `default`. Normal firmware builds must continue to work without it.

## Dependency boundary

The feature enables the Vaachak-owned no-std Lua VM smoke dependency:

```text
support/vaachak-lua-vm
```

The crate executes a tiny in-memory Lua script first:

```lua
return 1 + 2
```

The expected result is `3`, with marker:

```text
vaachak-lua-vm-feature-gate-ok
```

## Current limits

This is not wired to SD apps, Daily Mantra, Calendar, Panchang, dashboard routing, or file execution.
It only proves that a VM dependency can be gated, compiled, and smoke-run independently.

## Guardrails

- no SD app execution
- no dashboard behavior change by default
- no vendor/pulp-os changes
- no native app replacement
- no default feature enablement
