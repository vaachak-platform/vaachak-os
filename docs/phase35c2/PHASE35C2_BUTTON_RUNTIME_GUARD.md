# Phase 35C-2 — Button Runtime Guard

## Purpose

Phase 35C-2 locks in the hardware-validated rollback from the failed active
theme/metadata app-layer wrapper.

The Xteink X4 button path must continue through the known-good imported Pulp
runtime:

```text
InputDriver::new(board.input)
tasks::input_task(input)
ButtonMapper::new()
AppManager::new(...)
kernel.boot(&mut app_mgr)
kernel.run(&mut app_mgr)
```

## Guarded Regression

Phase 35C-1 attempted to wrap Pulp `AppManager` in a Vaachak `AppLayer`
adapter. That compiled, but hardware validation showed button input stopped
working.

Phase 35C-2 therefore forbids Vaachak-owned `AppLayer` wrappers in the active
runtime path until a smaller, hardware-safe hook is available.

## Current Ownership

Vaachak owns:

```text
reader state facade formats from Phase 35C-0
path-only storage state preflight from Phase 35B
guardrails preventing app-layer wrapper regressions
```

Imported Pulp still owns:

```text
physical input sampling
button mapping dispatch path
active app manager
active reader app
theme/metadata active persistence
```

Normal boot remains:

```text
vaachak=x4-runtime-ready
```
