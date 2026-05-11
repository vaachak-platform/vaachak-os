# Vaachak OS Lua App Architecture

## Purpose

Lua support is an optional SD-loaded app layer for Vaachak OS. It is intended for small tools, productivity apps, experiments, content-of-the-day apps, and simple network features. It is not a replacement for the native firmware runtime.

## Non-goals

The scaffold does not add a Lua VM, does not add a Lua crate dependency, and does not change runtime behavior. It also does not move reader, display, storage, input, power, Wi-Fi, sleep image, or prepared-cache behavior out of native Vaachak OS.

Native Vaachak OS remains responsible for:

```text
- Home/category dashboard
- Reader and file browser
- Reader settings and progress
- TXT and EPUB rendering paths
- Prepared cache handling
- Display refresh policy
- Button/input sampling
- Storage mounting and SD/FAT behavior
- Wi-Fi Transfer
- Network time sync
- Sleep image modes
- Power/recovery behavior
```

Lua apps are allowed to request narrow, capability-gated services from the native runtime only after a future VM integration step.

## SD layout

Lua apps live under `/VAACHAK/APPS` on the SD card.

```text
/VAACHAK/APPS/
  daily_mantra/
    app.toml
    main.lua
    mantras.txt
  calendar/
    app.toml
    main.lua
    events.txt
  panchang/
    app.toml
    main.lua
    data/
      2026.txt
  state/
    <app_id>/
  cache/
    <app_id>/
  lib/
```

Rules:

```text
- Each app is a folder with app.toml and an entry Lua file.
- App IDs are lowercase ASCII with underscores only.
- Apps may read files inside their own folder.
- App state writes are limited to /VAACHAK/APPS/state/<app_id>/.
- App cache writes are limited to /VAACHAK/APPS/cache/<app_id>/.
- Shared libraries under /VAACHAK/APPS/lib are not enabled in the first runtime slice.
- Apps cannot access reader progress, bookmarks, Wi-Fi credentials, or raw SD paths unless a future native capability exposes a narrow read-only view.
```

## Manifest schema

Each app folder must include `app.toml`.

```toml
id = "daily_mantra"
name = "Daily Mantra"
category = "Tools"
type = "activity"
version = "0.1.0"
entry = "main.lua"
description = "Shows a daily local mantra from SD content."
capabilities = ["display.text", "input.navigation", "storage.app_read", "storage.app_state", "system.date"]
state_scope = "app"
cache_scope = "app"
network = false
```

Required fields:

| Field | Meaning |
| --- | --- |
| `id` | Stable app ID. Must match the app folder name. |
| `name` | User-visible name. |
| `category` | Dashboard category. |
| `type` | App execution type. |
| `version` | App version for compatibility and debugging. |
| `entry` | Lua file loaded by the app manager. |
| `description` | Short user-visible description. |
| `capabilities` | Native APIs requested by the app. |
| `state_scope` | Must be `app` for first scaffold. |
| `cache_scope` | Must be `app` for first scaffold. |
| `network` | `true` only for future network-capable apps. |

## Categories

The dashboard category must be one of:

```text
Network
Productivity
Games
Reader
System
Tools
```

Recommended usage:

```text
Network       Lightweight network experiments such as RSS or fetch-only tools.
Productivity  Calendar, notes, timers, Pomodoro, simple planning tools.
Games         Small turn-based or puzzle apps.
Reader        Reader helpers only. Native Reader remains the reading engine.
System        Native/trusted diagnostics and app manager surfaces only.
Tools         Daily Mantra, Panchang, dictionary, converters, small utilities.
```

## App types

Initial app types:

```text
activity  A visible foreground app opened from the dashboard.
reader    Future read-only helper or metadata tool. Not a replacement EPUB/TXT reader.
service   Future constrained helper. No long-running background execution in the first runtime slice.
```

The first real apps should use `activity`.

## Safe initial Lua API

The first runtime integration should expose only narrow APIs. Names below are the contract target, not current firmware code.

### system

```lua
system.date()              -- returns current date from native Date & Time model
system.time_status()       -- "live", "cached", or "unsynced"
system.battery_percent()   -- read-only battery percentage
system.millis()            -- monotonic uptime milliseconds
system.log(message)        -- bounded native log message
system.exit()              -- request safe app exit
```

### display

```lua
display.clear()
display.text(x, y, text)
display.rect(x, y, w, h)
display.refresh()
```

Rules:

```text
- Lua requests drawing; native code owns clipping and refresh policy.
- Lua does not control SPI or EPD commands.
- Native code may coalesce refreshes to protect e-paper behavior.
```

### input

```lua
input.up()
input.down()
input.select()
input.back()
input.next()
```

Rules:

```text
- Back must always be honored by native code.
- Power remains a native special button.
- Apps cannot make Back non-exiting.
```

### storage

```lua
storage.read_app_file(path)
storage.read_app_state(key)
storage.write_app_state(key, value)
```

Rules:

```text
- read_app_file reads only inside the app folder.
- read_app_state and write_app_state are scoped to /VAACHAK/APPS/state/<app_id>/.
- Raw filesystem APIs are not exposed.
- Path traversal is rejected by native code.
```

### network

Network APIs are not part of the first VM slice. A later network capability can expose constrained fetch-only helpers, for example:

```lua
net.http_get(url, max_bytes)
```

Rules for future network APIs:

```text
- No raw sockets in Lua.
- No access to Wi-Fi credentials.
- Native runtime owns connection state and timeout policy.
- Requests must be bounded by size and time.
```

## Lifecycle

A foreground Lua app follows this lifecycle:

```text
1. Native dashboard discovers app.toml.
2. User opens the app.
3. Native app manager validates manifest and capabilities.
4. Native runtime creates a restricted Lua state.
5. Native runtime loads the entry script.
6. app.on_open(ctx) is called if present.
7. Native loop sends button events to app.on_event(event).
8. App updates internal state.
9. app.on_draw(ctx) is called when redraw is required.
10. Back or app exit request triggers app.on_close(ctx) if present.
11. Native runtime destroys the Lua state.
12. User returns to the Vaachak dashboard.
```

Suggested Lua module shape:

```lua
local app = {}

function app.on_open(ctx)
end

function app.on_event(ctx, event)
end

function app.on_draw(ctx)
end

function app.on_close(ctx)
end

return app
```

## Crash and exit behavior

Native Vaachak OS must treat Lua as untrusted app code.

Required behavior:

```text
- Lua app error returns to dashboard after showing a bounded crash message.
- Back exits any Lua activity.
- Power behavior remains native.
- Native code owns watchdog/yield policy.
- Native code owns display refresh policy.
- Lua app state writes are bounded and scoped.
- The app manager must reject path traversal in manifests and app file access.
- Failure to load one Lua app must not prevent native Vaachak OS from booting.
```

The crash screen should show:

```text
App failed
<app name>
<short error class>
Back: Home
```

The full error should be logged only if logging is enabled and bounded.

## Initial app candidates

### Daily Mantra

Best first app because it is local, text-only, and date-driven.

```text
- Reads mantras.txt from the app folder.
- Uses system.date for default daily selection.
- Uses Up/Down for previous/next.
- Uses Select to mark favorite later.
- Uses Back to exit.
```

### Calendar

Good second app with local SD data only.

```text
- Month or agenda view.
- Reads events.txt from the app folder.
- Uses native date model.
- No external calendar sync initially.
```

### Panchang

Good staged app if driven by precomputed data first.

```text
- Reads data/<year>.txt from the app folder.
- Shows today, next day, previous day.
- Does not run astronomical calculations in Lua initially.
- Location-specific packs can be added later.
```

## Runtime rollout plan

Recommended sequence:

```text
1. Architecture scaffold and static examples.
2. Feature-gated Lua VM probe with one built-in script string.
3. Restricted API probe with system.log and system.version only.
4. SD manifest discovery without executing apps.
5. Daily Mantra as first SD app.
6. Calendar as second SD app.
7. Panchang using precomputed SD data.
8. Optional bounded network app experiments.
```

## Validation

Run:

```bash
./scripts/validate_lua_app_architecture_scaffold.sh
```

The validator checks docs, sample manifests, sample entry files, capability declarations, and that no Lua VM dependency has been added to active Cargo manifests.

## Lua SD app discovery model

The discovery model is a Vaachak-owned core contract. It models how app records found under `/VAACHAK/APPS` are interpreted, but it does not scan the SD card, mount storage, start a Lua VM, wire apps into the dashboard, or change runtime behavior.

Discovery input is deliberately separated from physical SD access. A future embedded adapter may provide records shaped like:

```text
app_folder = "daily_mantra"
manifest_text = contents of /VAACHAK/APPS/MANTRA/APP.TOM
files = ["app.toml", "main.lua", ...]
```

The core discovery model then validates the record and produces a `LuaAppRegistryModel`.

Rules:

```text
- Discovery root is /VAACHAK/APPS.
- Only direct child app folders are modeled in this slice.
- Folder names are safe lowercase relative names using a-z, 0-9, underscore, or hyphen.
- Reserved folders are rejected as apps: state, cache, lib.
- Each app folder must provide app.toml text.
- Manifest id must match the app folder name.
- Manifest entry must be present in the discovered file list.
- Entry paths remain safe relative .lua paths.
- Duplicate app ids are rejected.
- The output registry contains only valid apps.
- Diagnostics contain rejected app records and reasons.
```

Diagnostics modeled by `LuaAppDiscoveryDiagnosticKindModel`:

```text
missing app.toml
invalid manifest
missing entry file
duplicate app id
unsafe folder path
unsupported category
unsupported app type
unsupported capability
app id/folder mismatch
registry full
```

This slice remains static/model-only:

```text
- No actual SD scanning
- No Lua VM integration
- No dashboard wiring
- No runtime behavior change
- No vendor/pulp-os changes
```

## Lua host API permission contract

The host API contract is a Vaachak-owned `vaachak-core` model. It defines which Lua API namespaces and functions are allowed by manifest capabilities, but it does not bind a real Lua VM, scan SD apps, wire dashboard entries, or change embedded runtime behavior.

Namespace permission rules:

```text
system   -> baseline namespace, no manifest capability required
display  -> requires display capability
input    -> requires input capability
storage  -> requires storage capability
time     -> requires time capability
settings -> requires settings capability
network  -> requires network capability
```

Initial safe API descriptors:

```text
system.log(message)
system.version()
system.exit()
system.battery_percent()

display.clear()
display.text(x, y, text)
display.rect(x, y, w, h)
display.refresh()

input.next()
input.up()
input.down()
input.select()
input.back()

storage.read_app_file(path)
storage.read_app_state(key)
storage.write_app_state(key, value)

time.date()
time.time_status()

settings.read(key)
settings.write(key, value)

network.status()
network.fetch_text(url)
```

Validation order:

```text
1. Parse namespace.
2. Parse function within namespace.
3. Validate exact argument count.
4. Check manifest capability for non-system namespaces.
5. Optionally check current runtime support.
```

Error kinds modeled by `LuaHostApiErrorModel`:

```text
capability denied
unknown namespace
unknown function
invalid argument count
unsupported in current runtime
```

`LuaHostApiRuntimeModel::ContractOnly` represents the current no-binding state. Known and permitted calls can be described and permission-checked, but runtime execution remains unsupported until a future feature-gated Lua VM binding slice.

This slice remains static/model-only:

```text
- No real Lua VM integration
- No SD scanning
- No dashboard wiring
- No runtime behavior change
- No vendor/pulp-os changes
```



## Lua app runtime state model

The Lua app runtime state model is a contract-only model in `vaachak-core`. It defines how optional SD-loaded Lua apps move through lifecycle states before any real Lua VM, SD scanner, dashboard wiring, or host bindings are connected.

Runtime states:

```text
Discovered
LaunchPending
Running
Suspended
Exiting
Crashed
Disabled
```

Lifecycle events:

```text
discover
launch
suspend
resume
exit
crash
disable
```

Allowed transitions:

```text
Discovered + discover -> Discovered
Discovered + launch -> LaunchPending
LaunchPending + launch -> Running
Running + suspend -> Suspended
Suspended + resume -> Running
Discovered/LaunchPending/Running/Suspended + exit -> Exiting
Discovered/LaunchPending/Running/Suspended + crash -> Crashed
Discovered/LaunchPending/Running/Suspended/Exiting/Crashed/Disabled + disable -> Disabled
Crashed + exit -> Exiting
Exiting + exit -> Exiting
Disabled + discover/exit/disable -> Disabled
```

Invalid transitions are rejected by the model. Examples include `Running + launch`, `Discovered + resume`, and `Disabled + launch`.

Back button policy:

```text
Back always maps to the Lua lifecycle exit event.
The safe return target is the Vaachak dashboard.
```

Crash and exit diagnostics must preserve:

```text
app id
reason
last lifecycle state
safe return target
```

The state model does not execute Lua, discover SD files, draw UI, read storage, or alter native Vaachak runtime behavior. It is a contract layer used to keep future Lua runtime integration safe and testable.


## Lua app storage sandbox model

Lua app storage is sandboxed per app id. The storage sandbox is a contract-only model in `vaachak-core`; it does not perform real filesystem access, SD scanning, Lua VM binding, dashboard wiring, or embedded runtime behavior.

Per-app storage roots:

```text
/VAACHAK/APPS/state/<app_id>/
/VAACHAK/APPS/cache/<app_id>/
/VAACHAK/APPS/data/<app_id>/
```

Root ownership rules:

```text
state/<app_id>/  app-owned persistent state, readable and writable by that app
cache/<app_id>/  app-owned cache, readable and writable by that app
data/<app_id>/   optional bundled app data, read-only to the app
```

Allowed storage operations:

```text
read_app_file   -> data/<app_id>/, read
read_app_state  -> state/<app_id>/, read
write_app_state -> state/<app_id>/, write
read_app_cache  -> cache/<app_id>/, read
write_app_cache -> cache/<app_id>/, write
list_app_data   -> data/<app_id>/, list
```

All storage operations require the manifest capability:

```toml
capabilities = ["storage"]
```

Safe app-relative paths must be non-empty and relative. They must not contain traversal, absolute prefixes, backslashes, repeated separators, hidden segments, parent segments, spaces, or unsafe characters. Valid examples:

```text
mantras.txt
2026/may.txt
render/page_001.bin
```

Invalid examples:

```text
../settings.txt
/VAACHAK/APPS/state/other_app/prefs.txt
.hidden/file.txt
folder/.hidden
bad name.txt
bad\name.txt
```

Diagnostics covered by the storage sandbox model:

```text
unsafe path
capability denied
read-only violation
app id mismatch
unsupported operation
```

An app may only resolve paths for its own manifest id. For example, an app with `id = "daily_mantra"` may resolve `/VAACHAK/APPS/state/daily_mantra/...`, but not `/VAACHAK/APPS/state/calendar/...`.

The storage sandbox model intentionally does not expose raw SD paths, native reader progress files, native bookmarks, settings internals, SPI, FAT, or display behavior. Future host bindings must pass every Lua storage request through this model before touching real storage.

## Lua App Sample Pack Contract

The SD-only sample pack is contract data for future Lua host integration. It must remain safe to copy to an SD card image without changing firmware behavior.

Canonical sample layout:

```text
/VAACHAK/APPS/
  daily_mantra/
    app.toml
    main.lua
  calendar/
    app.toml
    main.lua
  panchang/
    app.toml
    main.lua
  data/
    daily_mantra/mantras.txt
    calendar/events_2026.txt
    panchang/nyc_2026.txt
  state/
    daily_mantra/favorites.txt
    calendar/view.txt
    panchang/day_offset.txt
  cache/
    daily_mantra/README.txt
    calendar/README.txt
    panchang/README.txt
```

Sample app rules:

```text
- Sample manifests must parse with the Vaachak manifest model.
- Manifest id must match the app folder name.
- Manifest entry must exist in the app folder.
- Sample app API usage is declared with comments of the form:
  -- vaachak-api: namespace.function(argument_count)
- Declared manifest capabilities must exactly match the non-system API namespaces used by those comments.
- Storage usage is declared with comments of the form:
  -- vaachak-storage: operation relative/path.txt
- Storage paths must remain app-relative and sandbox-safe.
- read_app_file paths resolve under /VAACHAK/APPS/data/<app_id>/.
- read_app_state and write_app_state paths resolve under /VAACHAK/APPS/state/<app_id>/.
- Cache directories are present as examples but no sample app uses cache writes yet.
```

The sample Lua files are not firmware-integrated and are not executed by Vaachak OS yet. They are API contract examples for the future host binding layer.

## Lua App Dashboard Catalog Model

The dashboard catalog model converts validated Lua SD app registry entries into dashboard-safe catalog entries. It does not wire apps into the live X4 dashboard yet.

Catalog rules:

```text
- Native Vaachak apps remain authoritative.
- Lua apps are optional additions only.
- Lua apps are grouped under the existing dashboard categories:
  - Network
  - Productivity
  - Games
  - Reader
  - System
  - Tools
- Category ordering follows the current dashboard order:
  Network, Productivity, Games, Reader, System, Tools.
- Display labels are derived from the validated manifest name.
- Sort keys are deterministic: category order, lowercase display label, app id.
- Disabled Lua apps are hidden from visible catalog entries.
- Crashed Lua apps are hidden from visible catalog entries.
- Hidden apps produce catalog diagnostics so the System/App Manager screen can explain why they are unavailable later.
- Duplicate display names within the same category produce diagnostics.
- Duplicate display names across different categories are allowed.
```

Catalog diagnostics covered by the model:

```text
hidden disabled app
hidden crashed app
duplicate display name
unsupported dashboard category
```

This model intentionally does not perform SD scanning, execute Lua, mutate dashboard state, or change native app routing. Future dashboard integration must treat the catalog output as advisory data and keep native app entries as the source of truth for built-in Reader, Settings, Wi-Fi Transfer, Date & Time, Sleep Image, and other working Vaachak features.

## Lua App Built-in Catalog Bridge

The built-in catalog bridge is the first target-side model bridge for Lua app catalog data. It is compiled only when the `lua-runtime-probe` feature is enabled.

Rules:

- Native dashboard apps remain authoritative.
- Lua apps are optional additions derived from already-discovered registry model records.
- The bridge does not scan the SD card.
- The bridge does not execute Lua.
- The bridge does not wire entries into the visible dashboard UI.
- The bridge does not add a Lua VM dependency.
- The bridge emits only the feature-gated diagnostic marker `vaachak-lua-catalog-bridge-ok` through the model probe contract.

Native snapshot:

```text
Reader       -> Reader category
Daily Mantra -> Tools category
```

The bridge converts `LuaAppRegistryModel` into `LuaAppDashboardCatalogModel` using the accepted catalog rules. Later runtime slices may feed real SD discovery results into this bridge, but dashboard wiring must remain a separate explicit deliverable.

## Lua App SD Manifest Reader Bridge

The SD manifest reader bridge is the first target-side bridge from known SD app manifest paths into the accepted Lua discovery/catalog models.

Rules:

- The bridge is compiled only with the `lua-runtime-probe` feature.
- It reads only known sample manifest paths:
  - `/VAACHAK/APPS/MANTRA/APP.TOM`
  - `/VAACHAK/APPS/CALENDAR/APP.TOM`
  - `/VAACHAK/APPS/PANCHANG/APP.TOM`
- It does not recursively scan `/VAACHAK/APPS`.
- It does not execute Lua.
- It does not wire entries into the dashboard UI.
- It does not add a Lua VM dependency.
- It does not touch raw SD, FAT, SPI, or `vendor/pulp-os` behavior.
- Missing manifests are converted into discovery diagnostics rather than panics.

The diagnostic marker for this feature-gated bridge is:

```text
vaachak-lua-sd-manifest-reader-ok
```

This bridge accepts manifest text from a read-only storage abstraction and converts it into `LuaAppDiscoveryOutcomeModel` plus `LuaAppDashboardCatalogModel`. A later explicit runtime slice may implement the abstraction using the existing storage path, but recursive scanning and dashboard UI wiring remain separate deliverables.



## First working app proof

The first on-device Lua proof is `daily_mantra` behind the existing
`lua-runtime-probe` feature. It is intentionally a safe Lua declaration subset,
not the final full Lua VM. The firmware reads:

```text
/VAACHAK/APPS/MANTRA/MAIN.LUA
```

and evaluates display fields such as:

```lua
display_title = "Lua Daily Mantra"
display_subtitle = "This screen is loaded from SD main.lua"
display_line1 = "Om Namah Shivaya"
display_line2 = "Edit this file and reopen to change text"
display_line3 = "No full Lua VM dependency yet"
display_footer = "Back exits safely to Tools"
```

When `lua-runtime-probe` is enabled, the Tools category gets a `Lua Daily Mantra`
entry. Opening it renders the SD script values on device and Back returns to the
Tools list. The diagnostic marker is:

```text
vaachak-lua-daily-mantra-app-ok
```

This proves the app UX, screen rendering, safe exit, and SD script load path
before investing flash/RAM budget in the full Lua VM binding.

## Lua Daily Mantra SD runtime app

The first on-device Lua app proof uses `/VAACHAK/APPS` at the SD root as the canonical app base. The Daily Mantra proof requires all three files below before the screen reports `Source: SD Lua`:

```text
/VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/MANTRA/MAIN.LUA
/VAACHAK/APPS/MANTRA/MANTRAS.TXT
```

The app remains feature-gated by `lua-runtime-probe` and uses a small safe Lua declaration subset instead of a full Lua VM. This proves SD upload/edit/reopen behavior before committing flash/RAM budget to an interpreter. If any file is missing or invalid UTF-8, the app shows a clear diagnostic screen and Back returns safely to Tools.

## X4 SD physical naming for Lua apps

The logical Lua app identity remains stable, for example `id = "daily_mantra"` in the manifest text. On the Xteink X4 SD card, physical app folders and file names should use 8.3-safe uppercase names because the active embedded FAT path uses short-file-name operations.

Canonical Daily Mantra runtime files:

```text
/VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/MANTRA/MAIN.LUA
/VAACHAK/APPS/MANTRA/MANTRAS.TXT
```

`APP.TOM` contains the same manifest fields as the earlier `app.toml` contract. The shorter physical name avoids long-filename and four-character-extension failures during Wi-Fi Transfer. The current Daily Mantra runtime reads `APP.TOM`, `MAIN.LUA`, and `MANTRAS.TXT` from `/VAACHAK/APPS/MANTRA` and reports `Source: SD Lua` only when all three files load successfully.


## Daily Mantra default Tools entry

The first accepted on-device Lua proof app is Daily Mantra. It is now visible from the normal Tools category without requiring the `lua-runtime-probe` feature. This does not add a full Lua VM dependency. The app still uses the small safe declaration-subset runner and reads the 8.3-safe SD files from:

```text
/VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/MANTRA/MAIN.LUA
/VAACHAK/APPS/MANTRA/MANTRAS.TXT
```

Native apps remain authoritative. Daily Mantra is the only Lua app promoted to the default Tools catalog in this slice. Future Lua apps should still be introduced behind explicit app-level validation before being shown by default.


<!-- BEGIN LUA_APP_DEPLOYMENT_CONTRACT -->
## Lua App Deployment Contract

`/VAACHAK/APPS` is the canonical root for all optional SD-loaded Lua apps.
Native Vaachak OS features remain authoritative; Lua apps are optional additions to the existing Biscuit-style category dashboard.

### Physical SD naming

The current X4 SD/FAT path is short-name oriented, so Lua app folders and files must use uppercase 8.3-safe physical names:

```text
/VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/MANTRA/MAIN.LUA
/VAACHAK/APPS/MANTRA/MANTRAS.TXT
/VAACHAK/APPS/CALENDAR/APP.TOM
/VAACHAK/APPS/CALENDAR/MAIN.LUA
/VAACHAK/APPS/CALENDAR/EVENTS.TXT
/VAACHAK/APPS/PANCHANG/APP.TOM
/VAACHAK/APPS/PANCHANG/MAIN.LUA
/VAACHAK/APPS/PANCHANG/DATA/Y2026.TXT
```

### Physical-to-logical app map

```text
MANTRA   -> daily_mantra -> Daily Mantra -> Tools
CALENDAR -> calendar     -> Calendar     -> Productivity
PANCHANG -> panchang     -> Panchang     -> Tools
```

The physical folder is constrained for SD compatibility. The logical app id remains descriptive snake_case inside `APP.TOM`.

### Manifest file

The physical manifest file is named `APP.TOM` to remain 8.3-safe. Its contents use the existing Vaachak app manifest syntax:

```toml
id = "daily_mantra"
name = "Daily Mantra"
category = "Tools"
type = "activity"
version = "0.1.0"
entry = "MAIN.LUA"
capabilities = ["display", "input", "storage", "time"]
```

### Deployment rule

Upload app folders with Wi-Fi Transfer into `/VAACHAK/APPS`. Use the physical folder names above, not long lowercase folders such as `daily_mantra`.
<!-- END LUA_APP_DEPLOYMENT_CONTRACT -->


## Lua Calendar SD runtime app

Calendar is the second on-device Lua app proof and the first Lua app shown under the Productivity category. It preserves the native Calendar entry. The Lua Calendar entry is an optional app loaded from the canonical app root:

```text
/VAACHAK/APPS/CALENDAR/APP.TOM
/VAACHAK/APPS/CALENDAR/MAIN.LUA
/VAACHAK/APPS/CALENDAR/EVENTS.TXT
```

The app uses the same safe declaration-subset runner as Daily Mantra. It does not add a full Lua VM dependency yet. The runner validates the manifest id `calendar`, reads `MAIN.LUA`, reads the first event from `EVENTS.TXT`, renders `Source: SD Lua` when all files are present, and returns safely to Productivity on Back.

## Lua VM Daily Mantra execution bridge

Daily Mantra can optionally run one constrained VM expression from `/VAACHAK/APPS/MANTRA/MAIN.LUA` when the `lua-vm` feature is enabled. The existing subset parser remains the fallback path, and the default build behavior is unchanged. See `docs/lua/lua-vm-daily-mantra-execution-bridge.md`.

## Lua Daily Mantra text rendering cleanup

The Daily Mantra app keeps the 8.3-safe SD layout:

```text
/VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/MANTRA/MAIN.LUA
/VAACHAK/APPS/MANTRA/MANTRAS.TXT
```

`MANTRAS.TXT` records use `|` as a field separator. The separator is parsed, not displayed.

Supported record examples:

```text
Monday|Om Namah Shivaya|A steady mind turns every page into practice.
Om Shanti Shanti Shanti|Peace in thought, word, and action.
```

If the first field is a weekday, the screen shows it as `Day: <weekday>`. If no weekday is present, the screen shows `Day: Today`. The mantra text is wrapped across the existing Daily Mantra content lines so long records do not run into the right edge of the X4 display.


## Lua Calendar SD runtime app v2

Calendar is deployed physically at `/VAACHAK/APPS/CALENDAR` and logically identified as `calendar`. It appends to Productivity; native apps remain authoritative.


## Lua Calendar text rendering cleanup

Calendar display is driven by `/VAACHAK/APPS/CALENDAR/EVENTS.TXT`. The `|` separator is parsed and never rendered directly on screen.


## Lua Panchang SD runtime app

Panchang is deployed physically at `/VAACHAK/APPS/PANCHANG` and logically identified as `panchang`. It uses precomputed data under `DATA/Y2026.TXT` and appends to Tools; native apps remain authoritative.


## Panchang DATA folder runtime read

Lua Panchang keeps precomputed records at `/VAACHAK/APPS/PANCHANG/DATA/Y2026.TXT`. The target runtime reads this with a fixed-depth read-only helper instead of passing a slash-containing filename.


## Panchang nested data reader v2

Runtime reads `/VAACHAK/APPS/PANCHANG/DATA/Y2026.TXT` through a fixed-depth read-only path helper so the path shown in Wi-Fi Transfer matches the app loader.


## Panchang exact data path repair

Lua Panchang reads `/VAACHAK/APPS/PANCHANG/DATA/Y2026.TXT` through a dedicated fixed-depth helper and keeps a flat `/VAACHAK/APPS/PANCHANG/Y2026.TXT` fallback only for manual recovery.

<!-- VAACHAK:LUA_DEPLOYMENT_CONTRACT:START -->
## Final Lua app deployment model

Lua apps are deployed under `/VAACHAK/APPS` on the SD card. Physical folders and files use uppercase 8.3 names for the current X4 FAT path. Logical app ids remain descriptive snake_case in `APP.TOM`.

```text
MANTRA   -> daily_mantra
CALENDAR -> calendar
PANCHANG -> panchang
```

The accepted physical sample layout is:

```text
/VAACHAK/APPS/MANTRA/APP.TOM
/VAACHAK/APPS/MANTRA/MAIN.LUA
/VAACHAK/APPS/MANTRA/MANTRAS.TXT
/VAACHAK/APPS/CALENDAR/APP.TOM
/VAACHAK/APPS/CALENDAR/MAIN.LUA
/VAACHAK/APPS/CALENDAR/EVENTS.TXT
/VAACHAK/APPS/PANCHANG/APP.TOM
/VAACHAK/APPS/PANCHANG/MAIN.LUA
/VAACHAK/APPS/PANCHANG/DATA/Y2026.TXT
```

Native apps remain authoritative. Lua apps append to existing dashboard categories and must keep Back as a safe exit path.
<!-- VAACHAK:LUA_DEPLOYMENT_CONTRACT:END -->
