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