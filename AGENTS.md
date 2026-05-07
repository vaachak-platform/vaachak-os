# AGENTS.md — Vaachak OS

This file provides instructions for coding agents working in the `vaachak-os` repository.

Keep changes small, validated, and hardware-safe.

## Project Summary

Vaachak OS is an embedded Rust reader OS effort for the Xteink X4 / ESP32-C3 e-paper device.

Current accepted hardware target:

```text
Board/device: Xteink X4
MCU: ESP32-C3 revision v0.4
Crystal: 40 MHz
Flash: 16 MB
Display: SSD1677 e-paper, 800x480 native
Rust target: riscv32imc-unknown-none-elf
```

The current working reader runtime comes from imported X4/Pulp code under:

```text
vendor/pulp-os
vendor/smol-epub
```

Do not rewrite or casually refactor the imported reader runtime. It currently provides the known-good EPUB/TXT reader behavior.

## Current Development State

Latest accepted phase before Phase 30:

```text
Phase 29 — Storage Path Helpers
Accepted boot marker:
phase29=x4-storage-path-helpers-ok
```

Phase 30 goal:

```text
Phase 30 — Vaachak Runtime Ownership Consolidation
Expected boot marker:
vaachak=x4-runtime-ready
```

## Hard Rules

Do not modify:

```text
vendor/pulp-os/**
vendor/smol-epub/**
```

unless the prompt explicitly says to patch vendored code.

Do not change physical hardware behavior unless the prompt explicitly says so.

Do not move or rewrite these during Phase 30:

```text
SD/SPI initialization
filesystem reads/writes
EPUB cache IO
ADC sampling
button debounce/repeat handling
SSD1677 init
display refresh
strip rendering
reader app construction
EPUB parsing/rendering
bookmark/progress IO
theme IO
```

Do not reintroduce the fake EPUB smoke reader. These strings must never appear in active source:

```text
run_epub_reader_page_storage_smoke
ZIP container parsed
First readable bytes
ensure_pulp_dir_async
```

## Expected Target Structure After Phase 30

Preferred structure:

```text
target-xteink-x4/src/
  main.rs
  vaachak_x4/
    mod.rs
    boot.rs
    runtime.rs
    contracts/
      mod.rs
      boundary_contract.rs
      boundary_contract_smoke.rs
      storage.rs
      input.rs
      display.rs
      storage_state_contract.rs
      storage_path_helpers.rs
      input_contract_smoke.rs
      display_contract_smoke.rs
    imported/
      mod.rs
      pulp_reader_runtime.rs
```

The `vaachak_x4` namespace is Vaachak-owned.

The `vaachak_x4/imported/pulp_reader_runtime.rs` file is an imported-runtime wrapper and should remain behavior-equivalent to the previously accepted Pulp runtime.

## Boot Marker Policy

For Phase 30, normal boot should emit only:

```text
vaachak=x4-runtime-ready
```

Old phase markers may remain as constants or test fixtures, but they must not be printed during normal boot.

Old markers include:

```text
phase16=x4-reader-parity-ok
phase17=x4-reader-refactor-ok
phase18=x4-runtime-adapter-ok
phase19=x4-vaachak-runtime-facade-ok
phase20=x4-boundary-scaffold-ok
phase21=x4-storage-boundary-ok
phase22=x4-input-boundary-ok
phase23=x4-display-boundary-ok
phase24=x4-boundary-contract-ok
phase25=x4-storage-contract-smoke-ok
phase26=x4-input-contract-smoke-ok
phase27=x4-display-contract-smoke-ok
phase28=x4-boundary-contract-smoke-ok
phase29=x4-storage-path-helpers-ok
```

## Rust Validation Commands

Always run:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
```

For Phase 30 also run:

```bash
./scripts/check_imported_reader_runtime_sync.sh
./scripts/check_vaachak_x4_runtime.sh
```

Do not report success until these pass.

## Clippy Rules

`cargo clippy ... -- -D warnings` must pass.

Avoid adding broad crate-level allows. If scaffold code is intentionally unused, prefer one of:

```rust
#[allow(dead_code)]
```

on the specific module or item, or actually use the item in the contract smoke.

## Documentation Rules

When adding phase docs, use:

```text
docs/phaseXX/
```

For Phase 30, add:

```text
docs/phase30/PHASE30_RUNTIME_OWNERSHIP.md
docs/phase30/PHASE30_ACCEPTANCE.md
docs/phase30/PHASE30_NOTES.md
```

Docs should explain the boundary clearly:

```text
Vaachak owns the target namespace and contracts.
Imported Pulp owns the working physical reader behavior.
No hardware behavior moved in Phase 30.
```

## Script Rules

Scripts must be safe to rerun.

Scripts should create backups before destructive changes.

Scripts should fail clearly with actionable messages.

Do not depend on network access.

## Final Response Requirements

When done, report:

```text
- Files changed
- Validation commands run
- Whether vendor/pulp-os and vendor/smol-epub remained untouched
- Whether old phase boot markers are silenced
- Remaining risks or follow-up if any
```

Do not claim device success. The user performs flashing and hardware validation.

# AGENTS.md Addendum — Phase 31

Append this section to the repository `AGENTS.md`.

---

## Phase 31 — Active Storage Path Helper Adoption

### Accepted Baseline

Phase 30 is accepted on real Xteink X4 hardware.

Normal boot marker:

```text
vaachak=x4-runtime-ready
```

TXT and EPUB reader behavior is confirmed working after Phase 30.

### Phase 31 Goal

Phase 31 adopts Vaachak-owned pure storage path/name helpers in the active runtime path where safe.

This is not a physical storage extraction.

### Allowed Changes

Allowed:

```text
- Add or refine Vaachak-owned pure storage path helper functions.
- Add host-side tests for pure helper behavior.
- Update the imported runtime wrapper only to call Vaachak-owned pure helper functions where mechanically equivalent.
- Add Phase 31 docs and checks.
- Keep normal boot marker as vaachak=x4-runtime-ready.
```

### Forbidden Changes

Do not modify:

```text
vendor/pulp-os/**
vendor/smol-epub/**
```

Do not move or rewrite:

```text
SD card initialization
SPI bus setup
SPI bus sharing/arbitration
filesystem open/read/write/close
progress file IO
bookmark file IO
theme file IO
EPUB cache IO
ADC sampling
display refresh
reader app construction
EPUB parsing/rendering
TXT reader behavior
```

### Boot Marker Policy

Normal boot must continue to print only:

```text
vaachak=x4-runtime-ready
```

Do not reintroduce old phase marker logs.

Old phase strings may remain as constants or docs, but must not be actively printed during normal boot.

### Required Checks

Run:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings

./scripts/check_imported_reader_runtime_sync_phase31.sh
./scripts/check_phase31_storage_path_adoption.sh
```

### Fake EPUB Smoke Ban

Active source must not contain:

```text
run_epub_reader_page_storage_smoke
ZIP container parsed
First readable bytes
ensure_pulp_dir_async
```

### Vendor Rule

Before reporting success, verify:

```bash
git diff --quiet -- vendor/pulp-os vendor/smol-epub
```

No tracked vendor edits are allowed in Phase 31.

### Stop Conditions

Stop and report if adopting Vaachak helpers would require changing:

```text
filesystem IO
SD/SPI behavior
EPUB cache IO
reader behavior
vendor code
```

Phase 31 should move only pure deterministic path/name helper ownership.
# AGENTS.md Addendum — Phase 41G Biscuit Home Dashboard

## Mission

Implement a real, active, device-visible Biscuit-style Home dashboard for VaachakOS on Xteink X4.

The current Home screen still shows the old list-style UI:

- `Vaachak` header
- `x4` status on right
- `Last: <book title>`
- vertical list:
  - Continue
  - Files
  - Bookmarks
  - Settings
  - Upload

That is not acceptable for Phase 41G.

Phase 41G must replace the active Home render path with a Biscuit-inspired dashboard/card layout similar to the Biscuit reference screenshot.

## Critical Requirement

Do not only add helper modules.

The device-visible Home screen must actually change.

Trace and patch the active render path used by the X4 firmware. The likely active file is:

```text
vendor/pulp-os/src/apps/home.rs
```

If helper modules exist under:

```text
target-xteink-x4/src/vaachak_x4/ui/
```

they may be reused, but they are not sufficient unless the active Home renderer calls them or equivalent drawing logic is wired into the active path.

## Required Visual Result

The Home screen must become a dashboard.

Required elements:

```text
Header:
- Left: Vaachak
- Right: x4

Dashboard:
- Card/tile grid, preferably 2 columns
- Obvious selected card
- E-paper-friendly high contrast
- Large readable labels
- Placeholder apps visible

Cards:
- Reader
  - subtitle: Continue reading
  - show recent/current book snippet if available

- Library
  - subtitle: Browse books

- Bookmarks
  - subtitle: Saved places

- Settings
  - subtitle: Device & reader

- Sync
  - subtitle: Coming soon

- Upload
  - subtitle: Coming soon
```

The old vertical list must no longer be the Home UI.

## Behavior Requirements

Preserve working behavior:

```text
Reader card:
- Opens existing Continue / reader-entry flow.

Library card:
- Opens existing Files / Library browser.

Bookmarks card:
- Opens existing bookmarks flow if implemented.
- If not implemented, placeholder behavior is acceptable.

Settings card:
- Opens existing settings if implemented.
- If not implemented, placeholder behavior is acceptable.

Sync card:
- Placeholder only.

Upload card:
- Placeholder only unless existing Upload is implemented.
```

Navigation must remain usable with the existing X4 button/input path.

Preferred dashboard navigation:

```text
Left / Right:
- move horizontally between cards

Up / Down:
- move vertically between rows if supported by current input events

Select / OK:
- activate selected card

Back:
- existing Home-safe behavior
```

Do not change low-level button ladder calibration or input thresholds in this phase.

## Frozen Surfaces

Do not regress or intentionally change:

```text
- Files / Library listing behavior
- EPUB/EPU title display
- TXT/MD title display through _X4/TITLES.BIN
- TXT/MD body-title scanning disabled state
- Reader open/back/restore
- Reader pagination
- Bookmarks persistence
- footer label order outside Home unless absolutely necessary
- input mapping / ADC thresholds
- write lane / typed state / SD persistence
- display geometry / rotation
- SPI / SD / FAT runtime behavior
```

## Explicit Non-Goals

Do not do these in Phase 41G:

```text
- broad runtime cleanup
- runtime scaffolding pruning
- title-cache rewrite
- reader pagination rewrite
- font engine rewrite
- display geometry changes
- input calibration changes
- new SD write behavior
- app launcher architecture rewrite
```

## Implementation Guidance

1. Inspect active Home path first.

Check:

```bash
rg -n "struct Home|draw_menu|Continue|Bookmarks|Upload|Vaachak|Last:" vendor/pulp-os/src/apps/home.rs
rg -n "Home" vendor/pulp-os/src/apps target-xteink-x4/src
```

2. Identify the active draw function.

Likely candidates:

```text
draw_menu
draw_home
draw
render
```

3. Replace old list drawing with card dashboard drawing.

The dashboard should use existing drawing primitives already used in the project, such as:

```text
BitmapLabel
BitmapDynLabel
Region
Alignment
inverted(...)
```

4. Keep behavior dispatch intact where possible.

If the old `selected` index maps to existing actions, preserve the mapping but update labels/layout.

Recommended card order:

```text
0 Reader
1 Library
2 Bookmarks
3 Settings
4 Sync
5 Upload
```

If current actions expect old order:

```text
0 Continue
1 Files
2 Bookmarks
3 Settings
4 Upload
```

then map:

```text
Reader  -> old Continue action
Library -> old Files action
Bookmarks -> old Bookmarks action
Settings -> old Settings action
Upload -> old Upload action
Sync -> placeholder/no-op or safe message
```

5. Make visual difference unmistakable.

The output must not look like:

```text
> Continue
Files
Bookmarks
Settings
Upload
```

It should look like a dashboard with bordered/inverted cards.

## Required Files to Consider

Review and modify only what is needed.

Likely modified:

```text
vendor/pulp-os/src/apps/home.rs
target-xteink-x4/src/vaachak_x4/ui.rs
target-xteink-x4/src/vaachak_x4/ui/biscuit_home.rs
target-xteink-x4/src/vaachak_x4/ui/biscuit_home_apps.rs
```

Optional docs/scripts:

```text
docs/ui/phase41g-biscuit-home-dashboard.md
scripts/ui/check_phase41g_biscuit_home_dashboard.sh
scripts/ui/inspect_phase41g_biscuit_home_dashboard.sh
scripts/ui/write_phase41g_device_home_dashboard_report.sh
scripts/ui/accept_phase41g_biscuit_home_dashboard.sh
```

Do not modify unless necessary:

```text
vendor/pulp-os/src/apps/files.rs
vendor/pulp-os/src/apps/reader/mod.rs
vendor/pulp-os/kernel/src/kernel/dir_cache.rs
hal-xteink-x4/src/*
target-xteink-x4/src/vaachak_x4/input/*
target-xteink-x4/src/vaachak_x4/physical/*
target-xteink-x4/src/vaachak_x4/runtime/*
```

## Acceptance Criteria

Phase 41G is accepted only when:

```text
- Home screen visibly shows Biscuit-style dashboard/cards.
- Old Home vertical list is gone.
- Placeholder cards for non-reader apps are visible.
- Reader card opens existing reader/continue flow.
- Library card opens existing Files/Library flow.
- Files/Library title display still works.
- Reader open/back/restore still works.
- Footer labels remain sensible.
- Input navigation works.
- No crash/reboot.
- Build/check/clippy pass.
```

## Validation Commands

Run:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo test -p vaachak-core --all-targets
cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

Flash using the current detected port, for example:

```bash
espflash flash \
  --monitor \
  --chip esp32c3 \
  --port /dev/cu.usbmodem3101 \
  target/riscv32imc-unknown-none-elf/release/target-xteink-x4
```

## Deliverable Summary Required From Codex

At completion, report:

```text
- Files changed
- Active Home render path identified
- Why previous patch still showed old UI
- What now makes Home visibly Biscuit-style
- Behavior preserved
- Validation commands run
- Any manual device checks still needed
```

## Phase Marker

Use:

```text
phase41g=x4-biscuit-home-dashboard-active-ok
```
# AGENTS.md Addendum — Phase 41G Home Dashboard Polish

## Mission

Polish the active Biscuit-style Home dashboard on Xteink X4.

The dashboard is now visible on device, but the current version has UI issues:

```text
- card fonts are too large
- Reader card text clips
- footer is duplicated
- Home placeholder app routing needs to be explicit and safe

# AGENTS.md Addendum — Phase 41G Home Dashboard Polish

## Mission

Polish the active Biscuit-style Home dashboard on Xteink X4.

The dashboard is now visible on device, but the current version has UI issues:

```text
- card fonts are too large
- Reader card text clips
- footer is duplicated
- Home placeholder app routing needs to be explicit and safe
```

Phase 41G must improve the active Home dashboard without regressing the working Biscuit card layout.

## Active Path

The active Home renderer is:

```text
vendor/pulp-os/src/apps/home.rs
```

Patch the active renderer directly.

Do not only add unused helper modules under:

```text
target-xteink-x4/src/vaachak_x4/ui/
```

Helper modules may be used, but the device-visible Home path must change.

## Required UI Fixes

### Typography

The current card titles are too large. Fix card typography:

```text
- reduce card title font size
- reduce subtitle/detail density
- avoid text touching card borders
- prevent Reader recent-book text from clipping
- preserve readability on e-paper
```

Use existing fonts safely. Prefer smaller/body fonts inside cards instead of the largest heading font.

### Card Layout

Keep the 2-column dashboard:

```text
Reader      Library
Bookmarks   Settings
Sync        Upload
```

Cards should remain bordered and the selected card should remain visually obvious.

### Footer

The current Home screen shows two footer rows:

```text
Back Select Left Right
Back OK << >>
```

This must be fixed.

Only one footer row should be visible.

Prefer:

```text
Back    Select    Left    Right
```

But if the global footer is fixed by the shell, use the existing global footer and remove the Home custom footer.

Do not break Files/Reader footer behavior.

### Navigation

Card navigation should be predictable:

```text
Left   -> move to left column when possible
Right  -> move to right column when possible
Up     -> previous row when possible
Down   -> next row when possible
Select -> activate card
Back   -> safe Home behavior
```

Do not change low-level ADC/input calibration.

### Placeholder Routing

Routes:

```text
Reader:
- existing Continue / Reader flow
- fallback to Library/Files if no recent book

Library:
- existing Files/Library flow

Bookmarks:
- existing Bookmarks flow if present
- otherwise safe placeholder/no-op

Settings:
- existing Settings flow if present
- otherwise safe placeholder/no-op

Sync:
- placeholder/no-op

Upload:
- existing Upload if safe
- otherwise placeholder/no-op
```

No placeholder card may crash.

## Frozen Surfaces

Do not intentionally change:

```text
- Files/Library title source
- EPUB/EPU title display
- TXT/MD display from _X4/TITLES.BIN
- TXT/MD body-title scanning disabled state
- Reader pagination
- Reader restore
- bookmark persistence
- write lane
- typed state / SD persistence
- display geometry / rotation
- physical input thresholds
- SPI / SD / FAT runtime behavior
```

## Files To Inspect

```text
vendor/pulp-os/src/apps/home.rs
vendor/pulp-os/kernel/src/app/model.rs
vendor/pulp-os/src/apps/files.rs
vendor/pulp-os/src/apps/reader/mod.rs
target-xteink-x4/src/vaachak_x4/ui.rs
target-xteink-x4/src/vaachak_x4/ui/mod.rs
target-xteink-x4/src/vaachak_x4/ui/biscuit_tokens.rs
target-xteink-x4/src/vaachak_x4/ui/biscuit_layout.rs
target-xteink-x4/src/vaachak_x4/ui/biscuit_home.rs
target-xteink-x4/src/vaachak_x4/ui/biscuit_home_apps.rs
target-xteink-x4/src/vaachak_x4/contracts/input.rs
```

## Required Validation

Run:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo test -p vaachak-core --all-targets
cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

## Phase Marker

```text
phase41g=x4-biscuit-home-nav-polish-placeholder-routing-ok
```

## Acceptance Rule

Phase 41G is accepted only when the device-visible Home screen has:

```text
- active Biscuit dashboard still present
- improved card font sizing
- Reader card text not badly clipped
- exactly one footer row
- working navigation
- safe placeholder behavior
- no Files/Reader/title-cache/input/write/display regression
```
# AGENTS.md Addendum — Phase 42A App Shell Routing + Settings Implementation

## Mission

Implement Phase 42A as a code-first product increment.

Phase 42A combines app shell routing cleanup and real Settings implementation.

The goal is to move beyond Home dashboard polish and make the dashboard function like a real app shell.

## Phase

```text
Phase 42A — App Shell Routing + Settings Implementation
```

## Required Outcome

The Home dashboard must remain active and must route correctly:

```text
Reader      Library
Bookmarks   Settings
Sync        Upload
```

Settings must become a real screen/app, not a placeholder.

## Active Runtime Path

The active Home renderer is expected to be:

```text
vendor/pulp-os/src/apps/home.rs
```

Patch active runtime paths only.

Do not only add helper modules.

If a new Settings app is added, it must be wired into the active app/shell system.

## Implementation Requirements

### Home routing

Required card behavior:

```text
Reader:
- existing Continue/Reader flow
- fallback to Library/Files if no recent book

Library:
- existing Files/Library flow

Bookmarks:
- existing bookmarks flow if available
- otherwise safe placeholder/no-op

Settings:
- opens new Settings screen

Sync:
- safe placeholder/no-op

Upload:
- existing Upload only if safe
- otherwise safe placeholder/no-op
```

### Settings sections

Settings must visibly include:

```text
Reader
Display
Storage
Device
About
```

### Reader settings

```text
Font size: Small / Normal / Large
Line spacing: Compact / Normal / Relaxed
Margins: Compact / Normal / Wide
Show progress: On / Off
```

Phase 42A behavior:

```text
- in-memory selection is acceptable
- do not rewrite reader pagination
- do not persist unless existing safe persistence exists
```

### Display settings

```text
Refresh mode: Full / Balanced / Fast
Invert colors: Off / On
Contrast: Normal / High
```

Phase 42A behavior:

```text
- in-memory selection is acceptable
- do not change display geometry
- do not change rotation
- do not change low-level EPD driver behavior unless existing safe hook exists
```

### Storage settings

```text
SD status
Books count
Title cache status
Rebuild title cache: Host tool only / Coming soon
```

Phase 42A behavior:

```text
- read-only status is acceptable
- no on-device title-cache rebuild
- keep host TITLEMAP.TSV -> TITLES.BIN workflow unchanged
```

### Device settings

```text
Battery: placeholder/read-only if no safe source exists
Sleep timeout: 5 min / 10 min / 30 min / Never
Button test: Coming soon
```

Phase 42A behavior:

```text
- in-memory sleep timeout selection is acceptable
- do not change power manager behavior unless safe existing hook exists
```

### About section

Display:

```text
VaachakOS
Xteink X4
Build/profile info if available
Storage layout summary
```

Static build/profile text is acceptable for this first pass.

## UI Requirements

Use the existing e-paper drawing primitives already used by Home/Files/Reader.

Keep UI readable and simple:

```text
- no dense text
- one footer row only
- selected row/card obvious
- Back returns to Home from Settings
```

## Frozen Surfaces

Do not intentionally change:

```text
Files / Library:
- title source
- EPUB/EPU title display
- TXT/MD title display from _X4/TITLES.BIN
- TXT/MD body-title scanning disabled state

Reader:
- pagination
- restore
- bookmark persistence

System:
- write lane
- typed state / SD persistence
- display geometry / rotation
- physical input thresholds
- SPI / SD / FAT runtime behavior
- title-cache workflow
```

## Expected Files

Likely changed:

```text
vendor/pulp-os/src/apps/home.rs
vendor/pulp-os/src/apps/mod.rs
vendor/pulp-os/kernel/src/app/model.rs
```

Likely added:

```text
vendor/pulp-os/src/apps/settings.rs
docs/ui/phase42a-app-shell-settings.md
scripts/ui/check_phase42a_app_shell_settings.sh
scripts/ui/inspect_phase42a_app_shell_settings.sh
scripts/ui/write_phase42a_device_app_shell_settings_report.sh
scripts/ui/accept_phase42a_app_shell_settings.sh
```

Optional:

```text
target-xteink-x4/src/vaachak_x4/ui/biscuit_settings.rs
```

Avoid unless absolutely necessary:

```text
vendor/pulp-os/src/apps/files.rs
vendor/pulp-os/src/apps/reader/mod.rs
vendor/pulp-os/kernel/src/kernel/dir_cache.rs
hal-xteink-x4/src/*
target-xteink-x4/src/vaachak_x4/runtime/*
target-xteink-x4/src/vaachak_x4/input/*
target-xteink-x4/src/vaachak_x4/physical/*
```

## Phase Discipline

This is an implementation phase.

Do not create a documentation-only result.

Documentation should be minimal:

```text
docs/ui/phase42a-app-shell-settings.md
```

Scripts should be useful and lightweight.

## Validation

Run:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo test -p vaachak-core --all-targets
cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

## Acceptance

Manual device acceptance requires:

```text
HOME_DASHBOARD_STILL_ACTIVE=1
SETTINGS_CARD_OPENS_SETTINGS=1
SETTINGS_SECTIONS_VISIBLE=1
READER_SETTINGS_VISIBLE=1
DISPLAY_SETTINGS_VISIBLE=1
STORAGE_SETTINGS_VISIBLE=1
DEVICE_SETTINGS_VISIBLE=1
ABOUT_VISIBLE=1
SETTINGS_NAVIGATION_WORKS=1
SETTINGS_BACK_RETURNS_HOME=1
READER_CARD_STILL_WORKS=1
LIBRARY_CARD_STILL_WORKS=1
SYNC_UPLOAD_PLACEHOLDERS_SAFE=1
FILES_TITLES_STILL_OK=1
READER_RESTORE_STILL_OK=1
SINGLE_FOOTER_CONFIRMED=1
NO_INPUT_WRITE_GEOMETRY_REGRESSION=1
NO_CRASH_REBOOT=1
```

## Phase Marker

```text
phase42a=x4-app-shell-routing-settings-implementation-ok
```

## Reject Conditions

Reject work if:

```text
- Home dashboard regresses
- Settings card does not open Settings
- Settings screen lacks required sections
- settings selection crashes
- Sync/Upload crash
- Files title display regresses
- Reader restore regresses
- duplicate footer returns
- build/check/clippy fail
```
# AGENTS.md Addendum — Phase 42B Settings Persistence + Reader Preference Preview

## Mission

Implement Phase 42B as a code-first product increment.

Phase 42B makes Settings useful by persisting safe settings and applying only low-risk Reader preferences.

## Phase

```text
Phase 42B — Settings Persistence + Reader Preference Preview
```

## Required Outcome

Settings must no longer be only in-memory.

The device should support safe persisted preferences while preserving the stable Reader/Files/Home baseline.

## Active Runtime Paths

Likely active files:

```text
vendor/pulp-os/src/apps/settings.rs
vendor/pulp-os/src/apps/manager.rs
vendor/pulp-os/src/apps/reader/mod.rs
```

Patch active runtime paths only.

Do not only add helper modules.

## Settings Persistence Requirements

Persist these Settings values:

```text
Reader:
- font size
- line spacing
- margins
- show progress

Display:
- refresh mode
- invert colors
- contrast

Device:
- sleep timeout
```

Use existing safe Settings/config hooks if present.

If no safe hook exists, add a small isolated Settings persistence implementation.

Candidate path:

```text
_X4/SETTINGS.TXT
```

or the repository’s existing settings/config path if one exists.

Required persistence behavior:

```text
- missing settings file -> defaults
- corrupt settings file -> defaults
- save failure -> no crash
- values visible after navigating away/back
- values visible after reboot when save succeeds
```

## Reader Preference Preview

Safe to apply:

```text
Show progress:
- apply if existing reader chrome can safely hide/show progress/status

Font size:
- apply only if existing reader supports it without pagination rewrite
- otherwise persist only

Line spacing:
- apply only if existing reader supports it without pagination rewrite
- otherwise persist only

Margins:
- apply only if existing reader supports it without pagination rewrite
- otherwise persist only
```

Do not rewrite reader pagination in Phase 42B.

## Display Settings

Persist values:

```text
Refresh mode
Invert colors
Contrast
```

Do not change display geometry, rotation, waveform, or low-level EPD behavior unless an existing safe hook already exists.

Persist-only is acceptable.

## Device Settings

Persist:

```text
Sleep timeout
```

Battery and Button Test may remain read-only/placeholder.

Do not change power manager behavior unless an existing safe hook already exists.

## Frozen Surfaces

Do not intentionally change:

```text
Home:
- Biscuit dashboard
- card routing
- single footer behavior

Files/Library:
- title source
- EPUB/EPU metadata title display
- TXT/MD display from _X4/TITLES.BIN
- TXT/MD body-title scanning disabled state

Reader:
- pagination algorithm
- restore/progress file format
- bookmark persistence format

System:
- write lane
- display geometry / rotation
- physical input thresholds
- SPI / SD / FAT low-level behavior
- title-cache workflow
```

## Expected Files

Likely changed:

```text
vendor/pulp-os/src/apps/settings.rs
vendor/pulp-os/src/apps/manager.rs
vendor/pulp-os/src/apps/reader/mod.rs
```

Possibly changed:

```text
vendor/pulp-os/kernel/src/app/model.rs
```

Likely added:

```text
docs/ui/phase42b-settings-persistence-reader-preview.md
scripts/ui/check_phase42b_settings_persistence_reader_preview.sh
scripts/ui/inspect_phase42b_settings_persistence_reader_preview.sh
scripts/ui/write_phase42b_device_settings_persistence_report.sh
scripts/ui/accept_phase42b_settings_persistence_reader_preview.sh
```

Avoid unless absolutely necessary:

```text
vendor/pulp-os/src/apps/files.rs
vendor/pulp-os/kernel/src/kernel/dir_cache.rs
hal-xteink-x4/src/*
target-xteink-x4/src/vaachak_x4/runtime/*
target-xteink-x4/src/vaachak_x4/input/*
target-xteink-x4/src/vaachak_x4/physical/*
```

## Implementation Discipline

Keep implementation simple:

```text
- explicit SettingsPrefs struct
- explicit default values
- simple parser/serializer
- no broad config framework
- no new dependency unless already present
- no reader pagination rewrite
```

Documentation must be minimal.

This is not a planning-only phase.

## Validation

Run:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo test -p vaachak-core --all-targets
cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

## Acceptance

Manual device acceptance requires:

```text
HOME_DASHBOARD_STILL_ACTIVE=1
SETTINGS_CARD_OPENS_SETTINGS=1
SETTINGS_VALUES_CHANGE=1
SETTINGS_VALUES_SURVIVE_NAVIGATION=1
SETTINGS_VALUES_SURVIVE_REBOOT=1
SHOW_PROGRESS_SETTING_APPLIED_OR_SAFELY_DEFERRED=1
FONT_SIZE_SETTING_PERSISTED=1
LINE_SPACING_SETTING_PERSISTED=1
MARGINS_SETTING_PERSISTED=1
DISPLAY_SETTINGS_PERSISTED=1
SLEEP_TIMEOUT_SETTING_PERSISTED=1
READER_CARD_STILL_WORKS=1
LIBRARY_CARD_STILL_WORKS=1
FILES_TITLES_STILL_OK=1
READER_RESTORE_STILL_OK=1
READER_PAGINATION_NOT_REGRESSED=1
SINGLE_FOOTER_CONFIRMED=1
NO_INPUT_WRITE_GEOMETRY_REGRESSION=1
NO_CRASH_REBOOT=1
```

## Phase Marker

```text
phase42b=x4-settings-persistence-reader-preview-ok
```

## Reject Conditions

Reject work if:

```text
- settings persistence is claimed but values do not survive reboot
- missing/corrupt settings can crash
- reader pagination is rewritten unexpectedly
- reader restore regresses
- Files title display regresses
- duplicate footer returns
- build/check/clippy fail
```

# AGENTS.md Addendum — Phase 42B-R1 Settings / Reader Vocabulary Alignment

## Mission

Fix Settings so Reader preference options match the existing Reader app.

This is a repair phase for Phase 42B.

## Phase

```text
Phase 42B-R1 — Settings / Reader Preference Vocabulary Alignment
```

## Problem

Settings currently exposes Reader options that do not match Reader’s actual settings vocabulary.

Example mismatch:

```text
Settings page:
Small / Normal / Large

Reader settings:
xsmall / small / medium / large / ...
```

This must be corrected.

## Rule

Do not implement new Reader behavior in this phase.

Reader implementation changes belong to the next phase.

This phase must align Settings/config with what Reader already supports.

## Required Behavior

Settings Reader section must:

```text
- show exact Reader-supported values
- use same default as Reader
- persist Reader-compatible values
- avoid unsupported/misleading rows
```

If Reader does not support line spacing or margins yet, Settings must not present them as active settings.

Acceptable handling:

```text
- hide unsupported rows
- or mark them as Coming soon
- or show them as persisted-only / deferred if clearly labeled
```

## Files to Inspect

```text
vendor/pulp-os/src/apps/reader/mod.rs
vendor/pulp-os/src/apps/settings.rs
vendor/pulp-os/kernel/src/kernel/config.rs
vendor/pulp-os/src/apps/manager.rs
vendor/pulp-os/kernel/src/app/model.rs
```

Search:

```bash
rg -n "xsmall|small|medium|large|xlarge|font|Font|line|spacing|margin|theme|reader|Reader|book_font|reading_theme|show_progress|progress" \
  vendor/pulp-os/src/apps/reader \
  vendor/pulp-os/src/apps/settings.rs \
  vendor/pulp-os/kernel/src/kernel/config.rs \
  vendor/pulp-os/src/apps/manager.rs
```

## Frozen Surfaces

Do not intentionally change:

```text
- Reader pagination
- Reader restore
- Reader layout algorithm
- Files/Library title display
- title-cache workflow
- Home dashboard
- display geometry / rotation
- input thresholds
- write lane
- SPI / SD / FAT runtime
```

## Expected Files

Likely changed:

```text
vendor/pulp-os/src/apps/settings.rs
vendor/pulp-os/kernel/src/kernel/config.rs
```

Possibly changed:

```text
vendor/pulp-os/src/apps/manager.rs
```

Avoid unless required for exposing existing constants only:

```text
vendor/pulp-os/src/apps/reader/mod.rs
```

## Phase Marker

```text
phase42b-repair1=x4-settings-reader-vocabulary-alignment-ok
```

## Validation

Run:

```bash
. "$HOME/export-esp.sh"

cargo fmt --all
cargo test -p vaachak-core --all-targets
cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf
cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
```

## Acceptance

```text
SETTINGS_READER_FONT_OPTIONS_MATCH_READER=1
SETTINGS_READER_DEFAULT_MATCHES_READER=1
SETTINGS_VALUES_SURVIVE_NAVIGATION=1
SETTINGS_VALUES_SURVIVE_REBOOT=1
UNSUPPORTED_READER_SETTINGS_NOT_MISLEADING=1
READER_CARD_STILL_WORKS=1
READER_RESTORE_STILL_OK=1
READER_PAGINATION_NOT_REGRESSED=1
FILES_TITLES_STILL_OK=1
HOME_DASHBOARD_STILL_ACTIVE=1
SINGLE_FOOTER_CONFIRMED=1
NO_INPUT_WRITE_GEOMETRY_REGRESSION=1
NO_CRASH_REBOOT=1
```

## Vaachak OS Network UI and Wi-Fi Transfer Guardrails

These guardrails apply to Home/category dashboard, Network screens, Wi-Fi configuration, and Wi-Fi transfer work.

### Naming and generated artifact rules

- Use semantic names only.
- Do not add generated-delivery names, numbered delivery labels, temporary marker labels, or similar strings in source, docs, scripts, logs, symbols, comments, or filenames.
- Do not add boot marker strings or log marker strings for this work.
- Do not leave temporary overlay directories, generated archives, or apply/audit helper scripts in the repository.
- Existing repository validation must continue to pass, including `./scripts/check_no_milestone_artifacts.sh .`.

### Home and category dashboard rules

- The category dashboard is the main Vaachak OS Home page.
- Do not restore the older tile-based homepage.
- Home categories must remain:
  - Network
  - Productivity
  - Games
  - Reader
  - System
  - Tools
- Preserve existing working routes:
  - Productivity > Daily Mantra opens the existing Daily Mantra screen.
  - Reader > Continue Reading preserves existing reader continue behavior.
  - Reader > Library opens the existing file/library flow.
  - Reader > Bookmarks opens the existing bookmarks screen.
  - System > Settings opens the existing Settings app.
  - Tools > File Browser opens the existing file/library flow.

### Network category rules

Network items should be:

- Wi-Fi Connect
- Wi-Fi Transfer
- Network Status

Expected behavior:

- Wi-Fi Connect shows saved Wi-Fi configuration status and must not display the password.
- Network Status shows current network/device status.
- Wi-Fi Transfer is the user-facing transfer-server entry point.
- Wi-Fi Transfer must not remain a placeholder once transfer integration is implemented.

### Wi-Fi credential rules

- Use the existing settings path and settings parser.
- Expected keys:
  - `wifi_ssid`
  - `wifi_pass`
- Never show the saved password on screen.
- Never log the saved password.
- Screens may show password status as Saved or Missing.
- Missing credentials must produce a clear user-facing error and wait for Back.

### Wi-Fi radio and transfer-server rules

- Do not start the Wi-Fi radio while drawing Home or Network category screens.
- Start Wi-Fi only after explicit user action on Wi-Fi Transfer.
- Prefer the existing isolated special-mode path if `AppId::Upload` and `run_upload_mode` already exist.
- It is acceptable to keep internal `AppId::Upload` naming when it avoids risky refactoring, but the user-facing title should be Wi-Fi Transfer.
- Active transfer mode should:
  - connect as Wi-Fi client,
  - wait for DHCP,
  - show `http://x4.local/`,
  - show the numeric IP address when available,
  - serve the existing HTTP transfer page on port 80,
  - exit on Back.
- If mDNS is implemented, ensure the encoded hostname matches `x4.local`.

### Safety and preservation rules

- Keep SD helper scripts unchanged unless the user explicitly asks to update them.
- Keep power-release guard behavior unchanged.
- Keep reader state, bookmarks, library, settings, and sleep-image behavior unchanged.
- Avoid broad rewrites of `home.rs`; patch the current route and screen behavior as narrowly as possible.
- Prefer existing abstractions and existing upload/server code over duplicating network code.

### Required validation

Run these before reporting completion:

- `cargo fmt --all --check`
- `cargo check --workspace --target riscv32imc-unknown-none-elf`
- `cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings`
- `cargo test -p vaachak-core --all-targets`
- `cargo test -p hal-xteink-x4 --all-targets`
- `cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf`
- `./scripts/check_no_milestone_artifacts.sh .`

Report all command results and any physical-device testing still required.

## Vaachak OS Font Asset and Script Run Guardrails

These guardrails apply to custom font foundations, script detection, glyph asset contracts, prepared glyph run contracts, and future Indic text rendering work.

### Scope boundaries

- Keep custom font infrastructure as a shared Vaachak OS text service, not a reader-only feature.
- Shared text modules belong under:
  - `target-xteink-x4/src/vaachak_x4/text/`
- The first font-asset work must remain contract-oriented:
  - define font asset formats,
  - define prepared run formats,
  - split Unicode text into script runs,
  - define cache lookup contracts,
  - keep rendering placeholders explicit.
- Do not wire reader rendering to new custom font contracts until a separate renderer task is requested.
- Do not implement full Indic shaping in the contract task.
- Do not attempt arbitrary TTF loading on the X4 in the contract task.

### Naming and artifact rules

- Use semantic names only.
- Do not add generated-delivery names, numbered delivery labels, temporary marker labels, marker logs, or similar strings in source, docs, scripts, comments, tests, symbols, or filenames.
- Do not add boot marker strings for font work.
- Do not leave temporary overlay directories, generated archives, local patch scripts, or apply/audit helper scripts in the repository.
- Existing repository validation must continue to pass, including:
  - `./scripts/check_no_milestone_artifacts.sh .`

### Font asset contract rules

Vaachak compact font assets use the `.vfnt` concept.

The `.vfnt` contract should describe:

- asset magic and version,
- script/family metadata,
- pixel size,
- line metrics,
- glyph metrics table,
- bitmap index table,
- bitmap data region,
- bitmap format such as 1bpp, 2bpp, or 4bpp.

Rules:

- Prefer 1bpp as the initial e-paper-friendly bitmap format.
- Keep structs simple and no_std/alloc-friendly.
- Do not use unsafe binary parsing.
- Do not implement SD-card font loading until requested.
- Do not claim a font asset is renderable until an actual glyph bitmap renderer exists.
- Missing or unsupported fonts must have explicit fallback/missing statuses.

### Prepared run contract rules

Vaachak prepared text runs use the `.vrun` concept.

The `.vrun` contract should describe:

- asset magic and version,
- run count,
- glyph count,
- cluster count,
- positioned glyph records,
- source text cluster mapping.

Rules:

- `.vrun` represents a future host/mobile/server-prepared shaped run format.
- Do not implement shaping in the contract task.
- Do not reorder, normalize, or substitute Unicode text in the script run splitter.
- Keep cluster metadata explicit so future shaping can preserve source mapping.

### Script detection and run splitting rules

Script detection must support at least:

- Latin
- Devanagari
- Gujarati
- Unknown

Run splitting must:

- return every meaningful contiguous script run,
- preserve slices from the original input where possible,
- avoid excessive one-character punctuation runs,
- keep whitespace and punctuation attached to nearby strong script runs using a deterministic policy,
- return no runs for an empty string,
- avoid Unicode normalization,
- avoid shaping,
- avoid character reordering.

Required mixed-script examples:

- `नमस्ते दुनिया`
- `धर्मक्षेत्रे कुरुक्षेत्रे`
- `નમસ્તે દુનિયા`
- `Vaachak नमस्ते નમસ્તે`
- `ॐ नमः शिवाय - Om Namah Shivaya`

### Glyph cache contract rules

The glyph cache module must remain contract-only until a renderer task is requested.

Allowed:

- cache key structs,
- glyph bitmap reference structs,
- lookup trait,
- empty lookup implementation,
- explicit missing/unsupported status values.

Not allowed yet:

- SD-card cache storage,
- actual bitmap loading,
- actual glyph rendering,
- reader integration,
- sleep screen rendering integration,
- claims of Indic rendering correctness.

### Indic text rules

Hindi, Sanskrit, and Gujarati require shaping for correct rendering.

Until a shaping pipeline exists:

- Do not claim Hindi/Sanskrit/Gujarati are fully supported.
- Do not present raw codepoint drawing as correct Indic rendering.
- Keep language around Indic support precise:
  - script detection is supported,
  - font asset contracts are supported,
  - shaping is planned,
  - correct Indic rendering is not complete yet.

### Safety and preservation rules

- Do not change Wi-Fi Transfer behavior.
- Do not change Network dashboard behavior.
- Do not change reader state files, bookmarks, library flow, settings behavior, sleep image behavior, or daily mantra behavior unless explicitly requested.
- Do not touch Wi-Fi credentials or transfer-server code for font work.
- Keep changes narrowly focused on text/font contracts and guard-script correctness.

### Required validation

Run these before reporting completion:

- `cargo fmt --all --check`
- `cargo check --workspace --target riscv32imc-unknown-none-elf`
- `cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings`
- `cargo test -p vaachak-core --all-targets`
- `cargo test -p hal-xteink-x4 --all-targets`
- `cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf`
- `./scripts/check_no_milestone_artifacts.sh .`
- `git diff --check`

Also run any crate-specific text/font tests if the workspace command set does not execute them automatically.

Report all command results and clearly state what is intentionally not implemented.

## Vaachak OS VFNT Parser Guardrails

These guardrails apply to compact font asset parsing and glyph lookup work.

### Scope boundaries

The VFNT parser task is a foundation task only.

Allowed:

- parse `.vfnt` assets from byte slices,
- validate headers,
- validate table offsets and lengths,
- validate bitmap ranges,
- expose glyph metrics lookup,
- expose bitmap slice lookup,
- add parser error types,
- add unit tests for valid and malformed assets,
- update font asset documentation.

Not allowed in this task:

- reader renderer integration,
- Daily Mantra renderer integration,
- sleep screen renderer integration,
- SD-card font discovery,
- Wi-Fi Transfer changes,
- arbitrary TTF loading,
- Indic shaping,
- Unicode reordering,
- glyph bitmap drawing,
- EPUB CSS font-family support.

### Naming and artifact rules

- Use semantic names only.
- Do not add generated-delivery names, numbered delivery labels, temporary marker labels, marker logs, or similar strings in source, docs, scripts, comments, tests, symbols, or filenames.
- Do not add boot marker strings for font work.
- Do not leave temporary generated archives, helper scripts, or local test artifacts in the repository.
- Existing repository validation must continue to pass, including:
  - `./scripts/check_no_milestone_artifacts.sh .`

### Parser safety rules

The parser must be safe and deterministic.

Required:

- parse from `&[u8]`,
- use explicit little-endian reads,
- avoid `unsafe`,
- avoid `transmute`,
- avoid direct struct casting from bytes,
- use checked arithmetic for offset and length calculations,
- validate before slicing,
- reject malformed assets early,
- return clear typed errors,
- keep lookup allocation-free.

The parser must validate:

- magic,
- version,
- header length,
- bitmap format,
- glyph count,
- metrics table bounds,
- bitmap index table bounds,
- bitmap data bounds,
- every bitmap record range before returning a bitmap slice.

### VFNT lookup rules

Glyph lookup should support:

- metrics lookup by glyph id,
- bitmap index lookup by glyph id,
- combined glyph lookup returning metrics, bitmap record, and bitmap byte slice.

Rules:

- Linear lookup is acceptable initially.
- Do not require sorted glyph ids unless the contract explicitly defines a sorted table.
- Missing glyphs must return an explicit missing-glyph error.
- Returned bitmap slices must always be bounded by the original asset byte slice.
- Unsupported bitmap formats must be rejected or surfaced explicitly.
- Do not claim a glyph is renderable until a renderer task exists.

### Documentation language

Documentation must be precise:

- `.vfnt` parser support is available.
- Bounds-safe glyph lookup is available.
- Rendering is not implemented in this task.
- SD-card discovery is not implemented in this task.
- Indic shaping is not implemented in this task.
- Correct Hindi/Sanskrit/Gujarati rendering still requires a future shaping and rendering pipeline.

### Preservation rules

Do not change:

- Wi-Fi Transfer behavior,
- Wi-Fi Connect behavior,
- Network Status behavior,
- category dashboard behavior,
- reader state files,
- library flow,
- bookmarks,
- settings,
- sleep image,
- daily mantra behavior.

### Required validation

Run these before reporting completion:

- `cargo fmt --all --check`
- `cargo check --workspace --target riscv32imc-unknown-none-elf`
- `cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings`
- `cargo test -p vaachak-core --all-targets`
- `cargo test -p hal-xteink-x4 --all-targets`
- `cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf`
- `./scripts/check_no_milestone_artifacts.sh .`
- `git diff --check`

Also run any direct text-module test harness if normal host tests cannot execute target text tests due to ESP host-incompatible dependencies.

Report all command results and clearly state what remains intentionally unimplemented.

## Vaachak OS VFNT Asset Reader and Font Catalog Binding Guardrails

These guardrails apply to connecting parsed VFNT assets to the shared text font catalog.

### Scope boundaries

This task is still a foundation task.

Allowed:

- define read-only VFNT asset references,
- parse in-memory font bytes through the existing VFNT parser,
- define loaded font face structs,
- bind loaded VFNT faces to semantic font/catalog selection,
- select preferred fonts by ScriptClass,
- implement fallback policy for Latin, Devanagari, Gujarati, and Unknown,
- add unit tests using synthetic in-memory VFNT data,
- update documentation.

Not allowed in this task:

- reader renderer integration,
- Daily Mantra renderer integration,
- sleep screen renderer integration,
- Home or Settings UI integration,
- SD-card font scanning,
- Wi-Fi Transfer changes,
- arbitrary TTF loading,
- Indic shaping,
- Unicode reordering,
- glyph bitmap drawing,
- EPUB CSS font-family support,
- committing font binaries or generated font assets.

### Naming and artifact rules

- Use semantic names only.
- Do not add generated-delivery names, numbered delivery labels, temporary marker labels, marker logs, or similar strings in source, docs, scripts, comments, tests, symbols, or filenames.
- Do not add boot marker strings for font work.
- Do not leave temporary generated archives, helper scripts, or local test artifacts in the repository.
- Existing repository validation must continue to pass, including:
  - `./scripts/check_no_milestone_artifacts.sh .`

### Asset reader rules

The asset reader must be read-only and byte-slice based.

Required:

- accept `&[u8]` font asset bytes,
- parse through the existing `VfntFont` parser,
- borrow original asset bytes,
- avoid copying font data,
- avoid file IO,
- avoid SD-card scanning,
- avoid unsafe code,
- keep logic deterministic and bounded.

The asset reader may expose semantic types such as:

- `FontAssetRef`
- `LoadedFontFace`
- `LoadedFontSet`
- `FontAssetReadError`
- `FontAssetReader`

### Font catalog binding rules

Font catalog binding should map loaded font faces to scripts.

Supported script classes:

- Latin
- Devanagari
- Gujarati
- Unknown

Fallback policy:

- exact script match wins,
- Unknown prefers Latin when available,
- Devanagari falls back to Latin when Devanagari is missing,
- Gujarati falls back to Latin when Gujarati is missing,
- if Latin is missing but another loaded font exists, fallback may return the first loaded font,
- if no fonts are loaded, return no selected font or a clear missing-font error.

Important wording:

- Font fallback does not mean correct Indic rendering.
- Correct Hindi/Sanskrit/Gujarati rendering still requires shaping and glyph rendering.
- This task only chooses a loaded font face.

### Test rules

Tests should use small synthetic VFNT byte arrays.

Tests must not require:

- real font files,
- Noto font binaries,
- SD-card files,
- generated font assets,
- Wi-Fi or hardware.

Recommended coverage:

- load Latin VFNT face,
- load Devanagari VFNT face,
- load Gujarati VFNT face,
- reject invalid VFNT asset,
- select exact script font,
- fallback to Latin for Unknown,
- fallback to Latin when Devanagari is missing,
- fallback to first available when Latin is missing,
- no font selected when no assets exist,
- loaded face borrows original bytes.

### Preservation rules

Do not change:

- Wi-Fi Transfer behavior,
- Wi-Fi Connect behavior,
- Network Status behavior,
- category dashboard behavior,
- reader state files,
- library flow,
- bookmarks,
- settings,
- sleep image,
- daily mantra behavior.

### Required validation

Run these before reporting completion:

- `cargo fmt --all --check`
- `cargo check --workspace --target riscv32imc-unknown-none-elf`
- `cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings`
- `cargo test -p vaachak-core --all-targets`
- `cargo test -p hal-xteink-x4 --all-targets`
- `cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf`
- `./scripts/check_no_milestone_artifacts.sh .`
- `git diff --check`

Also run any direct text-module test harness if normal host tests cannot execute target text tests due to ESP host-incompatible dependencies.

Report all command results and clearly state what remains intentionally unimplemented.

## Vaachak OS Glyph Bitmap Renderer Guardrails

These guardrails apply to low-level VFNT glyph bitmap rendering work.

### Scope boundaries

This task is still a foundation task.

Allowed:

- define glyph bitmap renderer contracts,
- define an in-memory monochrome render target,
- render 1bpp VFNT glyph bitmaps into a borrowed in-memory target,
- support safe clipping,
- support row stride,
- support transparent and optionally opaque blits,
- add unit tests using synthetic glyph data,
- update documentation.

Not allowed in this task:

- e-paper display integration,
- reader renderer integration,
- Daily Mantra renderer integration,
- sleep screen renderer integration,
- Home or Settings UI integration,
- SD-card font scanning,
- Wi-Fi Transfer changes,
- arbitrary TTF loading,
- Indic shaping,
- Unicode reordering,
- text layout,
- baseline layout,
- EPUB CSS font-family support,
- committing font binaries or generated font assets.

### Naming and artifact rules

- Use semantic names only.
- Do not add generated-delivery names, numbered delivery labels, temporary marker labels, marker logs, or similar strings in source, docs, scripts, comments, tests, symbols, or filenames.
- Do not add boot marker strings for font work.
- Do not leave temporary generated archives, helper scripts, or local test artifacts in the repository.
- Existing repository validation must continue to pass, including:
  - `./scripts/check_no_milestone_artifacts.sh .`

### Renderer safety rules

The renderer must be safe and deterministic.

Required:

- no unsafe code,
- no direct hardware access,
- no file IO,
- no SD-card access,
- no heap allocation in production renderer code,
- validate target buffer bounds before pixel access,
- validate glyph bitmap length before reading,
- validate glyph row stride before reading,
- clip pixels outside target bounds,
- never panic on negative origins or oversized glyphs.

### Bitmap rules

Initial renderer support is limited to 1bpp VFNT glyph bitmaps.

Recommended bit order:

- most-significant bit first within each byte,
- bit 7 is the leftmost pixel,
- bit 0 is the rightmost pixel.

Rules:

- use `VfntGlyph.bitmap.row_stride` for source row stepping,
- reject row stride too small for glyph width,
- reject bitmap data too short for declared height and row stride,
- ignore unused trailing bits when glyph width is not divisible by 8,
- return an explicit unsupported-format error for non-1bpp glyphs.

### Target rules

The in-memory target should be borrowed and bounded.

Allowed target forms:

- borrowed byte slice with explicit width, height, and row stride,
- borrowed bool slice if that better matches current test style.

Rules:

- target constructor must validate buffer capacity,
- `set_pixel` and `pixel` must remain bounds-safe,
- out-of-bounds pixel access should be ignored or return a clear error depending on API style,
- tests must cover set/get round trips.

### Blit mode rules

Transparent mode:

- glyph 1 bits set target pixels,
- glyph 0 bits leave target unchanged.

Opaque mode, if implemented:

- glyph 1 bits set target pixels,
- glyph 0 bits clear target pixels within glyph bounds.

If Opaque mode is deferred, document that only Transparent mode is implemented.

### Preservation rules

Do not change:

- Wi-Fi Transfer behavior,
- Wi-Fi Connect behavior,
- Network Status behavior,
- category dashboard behavior,
- reader state files,
- library flow,
- bookmarks,
- settings,
- sleep image,
- daily mantra behavior.

### Language around Indic support

Do not claim Hindi, Sanskrit, or Gujarati rendering correctness from this task.

This task only renders glyph bitmaps that are already present. Correct Indic text still requires:

- script run splitting,
- font fallback,
- shaping,
- positioned glyph runs,
- app-level layout,
- renderer integration.

### Required validation

Run these before reporting completion:

- `cargo fmt --all --check`
- `cargo check --workspace --target riscv32imc-unknown-none-elf`
- `cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings`
- `cargo test -p vaachak-core --all-targets`
- `cargo test -p hal-xteink-x4 --all-targets`
- `cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf`
- `./scripts/check_no_milestone_artifacts.sh .`
- `git diff --check`

Also run any direct text-module test harness if normal host tests cannot execute target text tests due to ESP host-incompatible dependencies.

Report all command results and clearly state what remains intentionally unimplemented.

## Vaachak OS Glyph Run Renderer Guardrails

These guardrails apply to prepared glyph run rendering work.

### Scope boundaries

This task remains a foundation task.

Allowed:

- define glyph run renderer contracts,
- render multiple positioned glyph records into an in-memory target,
- reuse the existing VFNT parser,
- reuse the existing glyph bitmap renderer,
- define prepared-font lookup traits or structs,
- support single-font and optionally slice-backed multi-font lookup,
- add unit tests using synthetic in-memory font data,
- update documentation.

Not allowed in this task:

- e-paper display integration,
- Reader renderer integration,
- Daily Mantra renderer integration,
- Sleep Screen renderer integration,
- Home or Settings UI integration,
- SD-card font scanning,
- Wi-Fi Transfer changes,
- arbitrary TTF loading,
- Indic shaping,
- Unicode reordering,
- full text layout,
- baseline layout,
- EPUB CSS font-family support,
- committing font binaries or generated font assets.

### Naming and artifact rules

- Use semantic names only.
- Do not add generated-delivery names, numbered delivery labels, temporary marker labels, marker logs, or similar strings in source, docs, scripts, comments, tests, symbols, or filenames.
- Do not add boot marker strings for font work.
- Do not leave temporary generated archives, helper scripts, or local test artifacts in the repository.
- Existing repository validation must continue to pass, including:
  - `./scripts/check_no_milestone_artifacts.sh .`

### Renderer safety rules

The glyph run renderer must be safe and deterministic.

Required:

- no unsafe code,
- no direct hardware access,
- no file IO,
- no SD-card access,
- no heap allocation in production renderer code,
- validate font lookup results,
- map missing fonts to explicit errors,
- map missing glyphs to explicit errors,
- delegate clipping to the glyph bitmap renderer,
- never panic on empty runs or malformed records.

### Prepared glyph rules

Prepared glyph records represent already-positioned glyphs.

Rules:

- render glyph records in order,
- use prepared x/y positions directly,
- do not shape Unicode,
- do not reorder glyphs,
- do not apply OpenType features,
- do not do line breaking,
- do not do baseline layout unless the existing record contract already defines it clearly,
- preserve cluster/source metadata if present but do not rely on it for rendering yet.

### Font lookup rules

Allowed lookup forms:

- single-font lookup for simple smoke tests,
- borrowed slice-backed multi-font lookup for prepared runs with font ids.

Rules:

- lookup must be allocation-free,
- missing font id returns a clear error,
- missing glyph id returns a clear error,
- unsupported bitmap format returns a clear error,
- do not load fonts from disk,
- do not scan SD card.

### Language around Indic support

Do not claim Hindi, Sanskrit, or Gujarati rendering correctness from this task.

This task can render positioned glyphs that already exist. Correct Indic text still requires:

- script run splitting,
- font fallback,
- shaping,
- prepared positioned glyph generation,
- app-level layout,
- renderer integration.

### Preservation rules

Do not change:

- Wi-Fi Transfer behavior,
- Wi-Fi Connect behavior,
- Network Status behavior,
- category dashboard behavior,
- reader state files,
- library flow,
- bookmarks,
- settings,
- sleep image,
- daily mantra behavior.

### Required validation

Run these before reporting completion:

- `cargo fmt --all --check`
- `cargo check --workspace --target riscv32imc-unknown-none-elf`
- `cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings`
- `cargo test -p vaachak-core --all-targets`
- `cargo test -p hal-xteink-x4 --all-targets`
- `cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf`
- `./scripts/check_no_milestone_artifacts.sh .`
- `git diff --check`

Also run any direct text-module test harness if normal host tests cannot execute target text tests due to ESP host-incompatible dependencies.

Report all command results and clearly state what remains intentionally unimplemented.

## Vaachak OS Prepared TXT Cache Guardrails

These guardrails apply to prepared TXT book rendering and prepared font/run cache integration.

### Scope boundaries

Prepared TXT work is the first Reader-visible custom-font smoke path.

Allowed:

- support TXT-only prepared cache detection,
- add an offline prepared-cache generator,
- generate tiny synthetic VFNT and VRN-style cache files,
- detect prepared cache by existing book id,
- render prepared page glyph records on the X4,
- preserve existing TXT Reader fallback when cache is missing,
- add parser/cache tests,
- document generator and SD layout.

Not allowed in this task:

- EPUB support,
- on-device Indic shaping,
- arbitrary TTF loading,
- general SD-card font discovery,
- Reader-wide custom font settings,
- Daily Mantra renderer integration,
- Sleep Screen renderer integration,
- Home or Settings UI font integration,
- Wi-Fi Transfer changes,
- committing large font binaries,
- committing generated cache output unless it is tiny and required as a fixture.

### Naming and artifact rules

- Use semantic names only.
- Do not add generated-delivery names, numbered delivery labels, temporary marker labels, marker logs, or similar strings in source, docs, scripts, comments, tests, symbols, filenames, or runtime logs.
- Do not add boot marker strings for font or prepared text work.
- Do not leave temporary generated cache directories in the repository.
- Existing repository validation must continue to pass, including:
  - `./scripts/check_no_milestone_artifacts.sh .`

### Cache layout rules

Use an 8.3-friendly cache layout for embedded SD compatibility.

Recommended layout:

- `/FCACHE/<BOOKID>/META.TXT`
- `/FCACHE/<BOOKID>/FONTS.IDX`
- `/FCACHE/<BOOKID>/LAT18.VFN`
- `/FCACHE/<BOOKID>/DEV22.VFN`
- `/FCACHE/<BOOKID>/PAGES.IDX`
- `/FCACHE/<BOOKID>/P000.VRN`

Rules:

- `<BOOKID>` must use the existing deterministic book id/path id policy.
- `.VFN` files contain VFNT-magic data.
- `.VRN` files contain VRUN-magic or prepared-run data.
- Metadata must be simple enough to parse on X4.
- The cache must not introduce a second incompatible book id policy.

### Reader behavior rules

When opening a TXT file:

- compute the existing book id,
- check for a matching prepared cache,
- validate metadata and required files,
- render prepared pages if cache is valid,
- fall back to existing TXT Reader behavior if cache is missing,
- show a safe message or fallback if cache is invalid,
- preserve Back behavior.

Do not break:

- existing TXT reading,
- existing EPUB reading,
- progress files,
- bookmark files,
- library navigation.

### Renderer rules

Prepared TXT rendering may use the shared text renderer if the active Reader can import it safely.

If crate ownership prevents importing target text modules:

- do not create dependency cycles,
- prefer a small temporary active-reader bridge,
- keep binary contracts aligned with VFNT/VRUN docs,
- document the bridge and future consolidation path.

Renderer must:

- parse VFNT/VRN data bounds-safely,
- avoid unsafe code,
- avoid on-device shaping,
- render prepared glyph positions as-is,
- use existing display/page drawing paths,
- avoid changing SSD1677 low-level behavior.

### Offline generator rules

The generator should:

- live under a semantic tools directory,
- use no network access,
- not require Noto fonts for the first smoke,
- generate a tiny sample cache from a mixed English + Devanagari TXT,
- write output to a user-provided directory,
- not commit generated output by default,
- print copy-to-SD instructions.

The generator may use synthetic VFNT glyphs for the smoke proof. It must not claim full Hindi/Sanskrit correctness.

### Language around Indic support

Do not claim general Hindi/Sanskrit rendering support from this task.

Allowed wording:

- prepared TXT cache can render pre-positioned glyphs,
- mixed English + Devanagari smoke path works when a prepared cache exists,
- on-device shaping is not implemented,
- arbitrary Hindi/Sanskrit TXT without cache is not fully supported yet.

### Preservation rules

Do not change:

- Wi-Fi Transfer behavior,
- Wi-Fi Connect behavior,
- Network Status behavior,
- category dashboard behavior,
- settings,
- sleep image,
- daily mantra,
- bookmarks,
- normal library flow.

### Required validation

Run these before reporting completion:

- `cargo fmt --all --check`
- `cargo check --workspace --target riscv32imc-unknown-none-elf`
- `cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings`
- `cargo test -p vaachak-core --all-targets`
- `cargo test -p hal-xteink-x4 --all-targets`
- `cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf`
- `./scripts/check_no_milestone_artifacts.sh .`
- `git diff --check`

Also run:

- prepared TXT smoke generator into a temporary output directory,
- list the generated cache files,
- remove temporary generated output,
- any direct text-module harness if normal host tests are blocked.

Report all command results and clearly state what remains intentionally unimplemented.
## Vaachak OS Real VFNT Generator and Prepared TXT Devanagari Guardrails

These guardrails apply to host-side real font generation for prepared TXT cache rendering.

### Scope boundaries

This task is a TXT-only prepared-rendering improvement.

Allowed:

- add a host-side generator for real VFNT assets,
- use user-provided NotoSans-Regular.ttf and NotoSansDevanagari-Regular.ttf,
- use rustybuzz/HarfBuzz host-side shaping,
- rasterize used glyphs into compact VFNT files,
- generate VRN positioned glyph records,
- generate FCACHE output compatible with the active Reader prepared TXT bridge,
- update docs and tests.

Not allowed in this task:

- EPUB support,
- on-device Indic shaping,
- Reader-wide font settings,
- general SD-card font discovery,
- arbitrary TTF loading on X4,
- Daily Mantra renderer integration,
- Sleep Screen renderer integration,
- Home or Settings UI font integration,
- Wi-Fi Transfer changes,
- committing large font binaries,
- committing generated cache output by default.

### Naming and artifact rules

- Use semantic names only.
- Do not add generated-delivery names, numbered delivery labels, temporary marker labels, marker logs, or similar strings in source, docs, scripts, comments, tests, symbols, filenames, or runtime logs.
- Do not add boot marker strings for font or prepared text work.
- Do not leave temporary generated cache directories in the repository.
- Existing repository validation must continue to pass, including:
  - `./scripts/check_no_milestone_artifacts.sh .`

### Host tool dependency rules

Host-side shaping and rasterization dependencies must not enter firmware crates.

Allowed in the host tool:

- rustybuzz or HarfBuzz-equivalent shaping crate,
- fontdue or another host-side rasterizer,
- ttf-parser if needed,
- normal host `std` APIs.

Rules:

- keep host tool under a tools directory,
- avoid making host dependencies part of the ESP32 firmware build,
- do not break workspace target builds,
- do not require network access,
- do not automatically download fonts,
- do not commit Noto font files unless explicitly approved.

### Font input rules

The generator must accept user-provided font paths:

- `NotoSans-Regular.ttf`
- `NotoSansDevanagari-Regular.ttf`

Rules:

- fail clearly if either font path is missing,
- fail clearly if font parsing fails,
- do not display massive binary dumps,
- do not copy font files into the repository,
- do not copy font files onto SD for this task.

### Shaping rules

Devanagari text must be shaped host-side.

Required:

- use rustybuzz/HarfBuzz host-side shaping for Devanagari,
- use shaped glyph ids and positions,
- do not use raw Unicode codepoint-to-glyph mapping for Devanagari,
- preserve shaped glyph ids as VFNT glyph ids or maintain an explicit mapping,
- generate VRN glyph records referencing glyph ids available in the generated VFNT.

Allowed:

- shape Latin with the same shaping engine.
- simple LTR layout only for this smoke.

Not required:

- BiDi,
- full paragraph layout,
- EPUB/CSS layout,
- hyphenation,
- justification.

### VFNT generation rules

Generated VFNT assets must:

- use the existing VFNT magic/version,
- use 1bpp bitmap glyphs,
- use MSB-first bit order,
- include only glyphs needed by the prepared TXT pages,
- include valid glyph metrics and bitmap records,
- use row stride consistently,
- produce non-empty bitmaps for visible glyphs,
- be accepted by the existing Reader prepared TXT bridge.

Rules:

- do not commit large generated VFN files,
- use temporary output or SD output only,
- keep any test fixture tiny if one is necessary.

### VRN generation rules

Generated VRN pages must:

- use the existing prepared page/VRN contract consumed by Reader,
- include positioned glyph records,
- reference the correct font slot or font id,
- reference glyph ids that exist in the corresponding VFN,
- keep coordinates inside the Reader page region for the smoke case.

### Reader behavior rules

Prefer no Reader changes if generated assets fit the existing bridge.

Reader changes are allowed only to fix correctness for real generated assets, such as:

- larger glyph ids,
- realistic row strides,
- non-zero bitmap offsets,
- simple bearing/position interpretation,
- safe handling of invalid cache.

Do not break:

- normal TXT fallback,
- EPUB behavior,
- Back behavior,
- progress files,
- bookmark files,
- Library navigation.

### Language around Indic support

Allowed wording:

- the prepared TXT smoke can render a host-shaped English + Devanagari TXT with real glyphs,
- Devanagari shaping happens offline on the host,
- X4 renders prepared glyph runs,
- general on-device Hindi/Sanskrit rendering is not implemented.

Do not claim:

- arbitrary Hindi/Sanskrit TXT support without prepared cache,
- EPUB Indic support,
- on-device Indic shaping,
- complete typography/layout support.

### Required validation

Run these before reporting completion:

- `cargo fmt --all --check`
- `cargo check --workspace --target riscv32imc-unknown-none-elf`
- `cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings`
- `cargo test -p vaachak-core --all-targets`
- `cargo test -p hal-xteink-x4 --all-targets`
- `cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf`
- `./scripts/check_no_milestone_artifacts.sh .`
- `git diff --check`

Also run:

- host tool unit tests,
- host tool build,
- host tool generation into a temporary directory if fonts are available,
- list generated temporary output,
- remove generated temporary output if possible.

Report all command results and clearly state what remains intentionally unimplemented.
## Vaachak OS Network Time and Clock Screen Guardrails

These guardrails apply to NTP time sync, Date & Time screen, Home time/date display, and Network Status time sync integration.

### Scope boundaries

This task adds a network time foundation only.

Allowed:

- add System > Date & Time screen,
- sync time with NTP over existing Wi-Fi credentials,
- cache last successful time sync on SD,
- show cached time, day, date, timezone, sync status, and last sync result,
- add Time Sync status to Network Status,
- update Home/category dashboard to show time/date and battery instead of category/app count,
- add docs and pure helper tests.

Not allowed in this task:

- Calendar app,
- Hindu calendar,
- timezone Settings UI,
- automatic boot sync,
- browser SD card manager,
- Wi-Fi Transfer behavior changes,
- Reader behavior changes,
- Daily Mantra behavior changes,
- Sleep Image behavior changes,
- custom font work.

### Naming and artifact rules

- Use semantic names only.
- Do not add generated-delivery names, numbered milestone labels, temporary marker labels, marker logs, or similar strings in source, docs, scripts, comments, tests, symbols, filenames, or runtime logs.
- Do not add boot marker strings for time work.
- Do not create zip archives or overlay helper scripts.
- Do not leave temporary generated files in the repository.
- Existing repository validation must continue to pass, including:
  - `./scripts/check_no_milestone_artifacts.sh .`

### Wi-Fi and NTP rules

- Do not start Wi-Fi from Home rendering.
- Do not start Wi-Fi automatically at boot.
- Do not start the HTTP transfer server for time sync.
- NTP sync must happen only after explicit user action on Date & Time or another explicit sync control.
- Reuse existing Wi-Fi credential handling.
- Never display or log the Wi-Fi password.
- Missing credentials must show a clear error.
- NTP timeout, DNS failure, invalid packet, network failure, and cache write failure must show clear status.
- Keep sync bounded with timeouts.

### Time cache rules

Use a simple, robust, line-based SD cache.

Recommended path:

- `/_x4/TIME.TXT`

Recommended fields:

- `timezone=America/New_York`
- `last_sync_unix=<unix-seconds>`
- `last_sync_monotonic_ms=<device-ms-if-available>`
- `last_sync_ok=1`
- `last_sync_source=ntp`
- `last_sync_error=<short-error>`
- `display_offset_minutes=<offset>`

Rules:

- missing cache means unsynced,
- corrupt cache means unsynced or safe fallback,
- old cached time should remain available after sync failure,
- do not claim unsynced or stale time as authoritative,
- keep timezone handling centralized so Settings can configure it later.

### Timezone rules

- Use `America/New_York` for this deliverable.
- Centralize it as a constant/config object.
- Document DST behavior.
- If full DST rules are not implemented, state the fixed-offset limitation clearly.
- Future work may add a Settings timezone picker.

### UI rules

Date & Time screen:

- shows current cached/estimated time,
- shows day and date,
- shows timezone,
- shows sync status,
- shows last sync result,
- Select starts sync,
- Back returns to System category.

Network Status:

- adds a Time Sync line,
- does not perform sync automatically.

Home:

- shows time/date and battery status,
- replaces category/app count text,
- does not trigger Wi-Fi,
- handles missing time cache safely,
- handles missing battery status safely.

### Preservation rules

Do not change:

- Wi-Fi Transfer behavior,
- Wi-Fi Connect behavior,
- normal Network Status behavior beyond the added Time Sync line,
- category dashboard navigation,
- Reader state files,
- library flow,
- bookmarks,
- settings,
- sleep image,
- daily mantra behavior.

### Required validation

Run these before reporting completion:

- `cargo fmt --all --check`
- `cargo check --workspace --target riscv32imc-unknown-none-elf`
- `cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings`
- `cargo test -p vaachak-core --all-targets`
- `cargo test -p hal-xteink-x4 --all-targets`
- `cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf`
- `./scripts/check_no_milestone_artifacts.sh .`
- `git diff --check`

Report all command results and clearly state what remains intentionally unimplemented.