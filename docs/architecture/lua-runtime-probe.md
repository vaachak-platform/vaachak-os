# Lua Runtime Probe Scaffold

## Purpose

This document defines the second Lua architecture slice for Vaachak OS: a
feature-gated runtime probe seam that prepares the firmware for a future Lua VM
without changing current device behavior.

The intent is to make the future Lua integration explicit, testable, and safe
before any SD-loaded app execution is added.

## Current boundary

Lua remains an optional app layer for:

- tools
- productivity apps
- experiments
- simple network features
- content-of-the-day apps such as Daily Mantra
- possible future Calendar and Panchang apps where feasible

Lua does not replace:

- native Reader
- native File browser
- native Settings
- native Wi-Fi Transfer
- native Date & Time sync
- native Sleep image modes
- display/input/storage/power behavior
- prepared cache, TXT, or EPUB behavior

## What this slice adds

This slice adds:

- `lua-runtime-probe` Cargo feature in `target-xteink-x4`
- a Vaachak-owned `target-xteink-x4/src/vaachak_x4/lua/` module
- a no-std compatible runtime probe contract
- a built-in probe script string used as the future VM smoke script
- a tiny native probe API model for `system.log` and `system.version`
- validation scripts proving the feature remains opt-in

## What this slice deliberately does not add

This slice does not add:

- a real Lua interpreter or VM dependency
- SD app discovery
- SD Lua script loading
- app execution from `/VAACHAK/APPS`
- network access from Lua
- filesystem writes from Lua
- dashboard UI wiring
- vendor or Pulp runtime changes

## Feature flag

The probe is compiled only when this feature is enabled:

```toml
[features]
default = []
lua-runtime-probe = []
```

Default firmware builds do not include the Lua probe module.

## Probe script

The built-in future smoke script is intentionally tiny:

```lua
system.log("lua-probe-start")
local version = system.version()
system.log("lua-probe-version:" .. version)
return version
```

The current implementation stores this as a contract string and models the
expected probe result natively. A later Lua VM slice can execute this exact
script through the real VM.

## Initial native API contract

The first future VM bridge should expose only:

```lua
system.log(message)
system.version()
```

No display, input, storage, network, or app lifecycle APIs are exposed in this
probe slice.

## Expected marker

The probe contract marker is:

```text
vaachak-lua-runtime-probe-ok
```

## Safety behavior

When a real VM is added later, the same probe boundary must preserve these rules:

- failure returns a diagnostic report instead of panicking
- the app/dashboard loop remains native
- Back/power behavior stays native
- Lua code cannot access storage directly
- Lua code cannot access display refresh policy directly
- Lua code cannot access Wi-Fi directly
- Lua app crashes return to the dashboard

## Relationship to SD app scaffold

The previous Lua app scaffold defines `/VAACHAK/APPS` layout and app manifests.
This probe slice is lower level: it prepares the native firmware integration
point for a future VM, but it does not load or execute files from the SD card.
