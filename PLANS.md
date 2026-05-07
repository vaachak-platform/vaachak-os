# Vaachak OS Plan: Network Wi-Fi Transfer Server Integration

## Goal

Make Network > Wi-Fi Transfer a real transfer-server entry point in the Vaachak OS category dashboard.

The category dashboard remains the main Home page. Wi-Fi Transfer should stop being a placeholder and should connect to the existing isolated Wi-Fi upload/server runtime.

## Current baseline

Working behavior to preserve:

- Home is the category dashboard.
- Network > Wi-Fi Connect displays saved Wi-Fi configuration status.
- Network > Network Status displays device/network status.
- Productivity > Daily Mantra opens the existing Daily Mantra screen.
- Reader > Continue Reading preserves existing reader continue behavior.
- Reader > Library opens the existing file/library flow.
- Reader > Bookmarks opens the existing bookmarks screen.
- System > Settings opens the existing Settings app.
- Tools > File Browser opens the existing file/library flow.

## Target behavior

Network should contain:

- Wi-Fi Connect
- Wi-Fi Transfer
- Network Status

Wi-Fi Transfer should:

- show user-facing title `Wi-Fi Transfer`,
- read saved Wi-Fi credentials through the existing settings model,
- never display or log the saved password,
- show missing-credential errors safely,
- start Wi-Fi only after explicit user action,
- use the existing isolated upload/server special-mode path when present,
- connect as Wi-Fi client,
- wait for DHCP,
- show `http://x4.local/`,
- show the numeric IP address when available,
- serve the existing HTTP transfer page on port 80,
- exit active transfer mode on Back.

## Implementation approach

### 1. Inspect current routing

Review:

- `vendor/pulp-os/src/apps/home.rs`
- `vendor/pulp-os/src/apps/manager.rs`
- `vendor/pulp-os/src/apps/upload.rs`
- `vendor/pulp-os/src/apps/settings.rs`
- `vendor/pulp-os/src/apps/mod.rs`

Confirm:

- where Network category item selection is handled,
- whether Wi-Fi Transfer currently routes to a placeholder,
- whether `AppId::Upload` exists,
- whether `AppManager::needs_special_mode` detects `AppId::Upload`,
- whether `run_upload_mode` is already used for Wi-Fi upload/server mode.

### 2. Route Wi-Fi Transfer

Patch the Network category route so that:

- Wi-Fi Connect remains unchanged,
- Network Status remains unchanged,
- Wi-Fi Transfer opens transfer-start behavior or directly enters the existing upload special mode.

Preferred low-risk path:

- keep internal `AppId::Upload` if already present,
- route Wi-Fi Transfer to `AppId::Upload`,
- update user-facing strings to `Wi-Fi Transfer`.

Optional start-screen path:

- add a Home state for Wi-Fi Transfer status/start,
- Back returns to Network category,
- Select enters `AppId::Upload`.

Use this only if it fits cleanly into the current Home state machine.

### 3. Align upload/server UI

Patch `vendor/pulp-os/src/apps/upload.rs` so user-facing text says:

- `Wi-Fi Transfer`
- `Connecting...`
- `No Wi-Fi credentials`
- `Press Back to exit`
- `http://x4.local/`

Keep password hidden.

Do not duplicate server code.

### 4. Fix mDNS hostname if needed

If the code comments or UI say `x4.local` but the DNS wire-format hostname encodes another name, update the wire-format name.

Expected wire-format bytes for `x4.local`:

- `[2, b'x', b'4', 5, b'l', b'o', b'c', b'a', b'l', 0]`

Update any hostname length constants if required.

### 5. Preserve existing settings behavior

Use the existing settings object and Wi-Fi config object.

Expected settings keys remain:

- `wifi_ssid`
- `wifi_pass`

Do not add a new settings file format in this work.

### 6. Add or update documentation

Add or update a concise document such as:

- `docs/wifi_transfer_server_wiring.md`

Document:

- the route from Network > Wi-Fi Transfer,
- the settings keys,
- missing credential behavior,
- server endpoint behavior,
- mDNS hostname,
- validation commands,
- physical-device test checklist.

Do not include generated-delivery labels or temporary markers.

## Acceptance criteria

Build and static checks:

- `cargo fmt --all --check` passes.
- `cargo check --workspace --target riscv32imc-unknown-none-elf` passes.
- `cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings` passes.
- `cargo test -p vaachak-core --all-targets` passes.
- `cargo test -p hal-xteink-x4 --all-targets` passes.
- `cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf` passes.
- `./scripts/check_no_milestone_artifacts.sh .` passes.

Runtime behavior:

- Home shows the category dashboard.
- Network opens the Network category items.
- Wi-Fi Connect still works.
- Network Status still works.
- Wi-Fi Transfer is no longer a placeholder.
- Missing credentials show a safe error screen.
- Saved password is never displayed.
- With valid credentials, Wi-Fi Transfer starts the radio/server path.
- DHCP result shows numeric IP when available.
- UI shows `http://x4.local/`.
- Browser can open the transfer page.
- Back exits active transfer mode.
- Existing Reader, Library, Bookmarks, Settings, Daily Mantra, and File Browser routes still work.

## Manual device test

Prepare SD settings:

- Ensure the existing settings file contains:
  - `wifi_ssid=your-network-name`
  - `wifi_pass=your-network-password`

Flash and test:

- Boot the X4.
- Open Network.
- Open Wi-Fi Connect and confirm SSID/password status is shown safely.
- Back to Network.
- Open Network Status and confirm it still renders.
- Back to Network.
- Open Wi-Fi Transfer.
- Start transfer mode if a start screen is shown.
- Wait for DHCP.
- Confirm the screen shows `http://x4.local/` and/or numeric IP.
- Open the URL from a browser on the same network.
- Confirm the transfer page loads.
- Press Back on the X4.
- Confirm transfer mode exits and returns safely.

## Notes

- The on-device SSID/password editor is not part of this work.
- Wi-Fi credentials are still expected to be prepared through the existing settings file path.
- A future task can add a button-driven Wi-Fi credential editor.

# Vaachak OS Plan: Font Asset Contract and Script Run Splitter

## Goal

Create the foundation for custom fonts and future Indic text rendering without changing the active reader renderer yet.

This work defines the Vaachak compact font asset contract, prepared glyph run contract, script run splitter, and glyph cache lookup contract.

## Non-goals

This work does not include:

- reader renderer integration,
- full Indic shaping,
- arbitrary TTF loading on the X4,
- SD-card font discovery,
- glyph bitmap rendering,
- EPUB CSS font-family support,
- Wi-Fi Transfer changes,
- Daily Mantra rendering changes,
- Sleep Screen rendering changes.

## Current baseline

The repository already has a shared text area under:

- `target-xteink-x4/src/vaachak_x4/text/`

The active product baseline to preserve:

- Category dashboard is the Home page.
- Network > Wi-Fi Connect works.
- Network > Wi-Fi Transfer works and can transfer files.
- Network > Network Status works.
- Reader, Library, Bookmarks, Settings, Daily Mantra, Sleep Image, and File Browser routes should remain unchanged.

## Planned files

Add:

- `target-xteink-x4/src/vaachak_x4/text/font_assets.rs`
- `target-xteink-x4/src/vaachak_x4/text/text_run.rs`
- `target-xteink-x4/src/vaachak_x4/text/glyph_cache.rs`

Update:

- `target-xteink-x4/src/vaachak_x4/text/mod.rs`
- `target-xteink-x4/src/vaachak_x4/text/script.rs` if needed for script classification helpers
- `target-xteink-x4/src/vaachak_x4/text/glyph_run.rs` if it is the cleanest home for prepared glyph run structs
- `scripts/check_no_milestone_artifacts.sh`
- `docs/font_asset_contract.md` or another suitable documentation file

## Font asset contract

Vaachak compact font assets use the `.vfnt` concept.

The `.vfnt` contract describes a compact e-paper-friendly bitmap font asset that can later be stored on SD or generated per book.

Recommended initial constants:

- `VFNT_MAGIC`
- `VFNT_VERSION`

Recommended structs:

- `VfntHeader`
- `VfntGlyphMetrics`
- `VfntGlyphBitmap`
- `VfntAssetInfo`
- `FontBitmapFormat`

Recommended fields for `VfntHeader`:

- magic
- version
- header length
- flags
- pixel size
- line height
- ascent
- descent
- glyph count
- metrics offset
- bitmap index offset
- bitmap data offset
- bitmap data length
- script
- bitmap format

Recommended fields for `VfntGlyphMetrics`:

- glyph id
- advance x
- advance y
- bearing x
- bearing y
- width
- height

Recommended fields for `VfntGlyphBitmap`:

- glyph id
- bitmap offset
- bitmap length
- row stride

Validation helpers:

- expected magic check
- supported version check
- supported bitmap format check
- combined supported-contract check

Implementation rules:

- no unsafe parsing,
- no file IO,
- no SD-card loading,
- no renderer claims,
- no shaping claims.

## Prepared run contract

Vaachak prepared text runs use the `.vrun` concept.

The `.vrun` contract represents future host/mobile/server-prepared shaped glyph runs.

Recommended initial constants:

- `VRUN_MAGIC`
- `VRUN_VERSION`

Recommended structs:

- `VrunHeader`
- `PositionedGlyph`
- `TextCluster`
- `PreparedGlyphRun`

Recommended fields for `VrunHeader`:

- magic
- version
- run count
- glyph count
- cluster count

Recommended fields for `PositionedGlyph`:

- font asset id
- glyph id
- x
- y
- advance x
- advance y
- cluster index

Recommended fields for `TextCluster`:

- source start
- source length
- first glyph
- glyph count

Validation helpers:

- expected magic check
- supported version check
- combined supported-contract check

Implementation rules:

- no shaping,
- no Unicode reordering,
- no serialization requirement yet unless the repository already has a preferred pattern,
- keep source-to-glyph mapping explicit.

## Script run splitter

Add full run splitting in:

- `target-xteink-x4/src/vaachak_x4/text/text_run.rs`

Required API:

- `TextRun`
- `split_script_runs`

Required supported script classes:

- Latin
- Devanagari
- Gujarati
- Unknown

Behavior:

- Empty input returns no runs.
- Strong script ranges are grouped into contiguous runs.
- Whitespace and neutral punctuation attach to nearby strong script runs using a deterministic policy.
- The splitter must return all meaningful runs, not just the first run.
- The splitter must preserve original text slices where possible.
- The splitter must not shape, reorder, or normalize text.

Required test strings:

- `नमस्ते दुनिया`
- `धर्मक्षेत्रे कुरुक्षेत्रे`
- `નમસ્તે દુનિયા`
- `Vaachak नमस्ते નમસ્તે`
- `ॐ नमः शिवाय - Om Namah Shivaya`

Expected mixed run behavior for:

- `Vaachak नमस्ते નમસ્તે`

Expected strong-script runs:

- Latin: `Vaachak `
- Devanagari: `नमस्ते `
- Gujarati: `નમસ્તે`

## Glyph cache contract

Add:

- `target-xteink-x4/src/vaachak_x4/text/glyph_cache.rs`

Purpose:

- Define future glyph lookup contracts.
- Do not implement real caching yet.

Recommended concepts:

- `FontAssetId`
- `GlyphCacheKey`
- `GlyphBitmapRef`
- `GlyphCacheStatus`
- `GlyphCacheLookup`
- `EmptyGlyphCache`

Implementation rules:

- `EmptyGlyphCache` should return a deterministic missing status.
- Do not add SD-card storage.
- Do not add glyph bitmap decoding.
- Do not wire reader, sleep screen, or daily text rendering yet.

## Guard script fix

Fix:

- `scripts/check_no_milestone_artifacts.sh`

Expected behavior:

- exits non-zero when forbidden delivery artifacts are found,
- prints matching lines for debugging,
- exits zero only when the repo is clean,
- does not hide important source/docs/scripts paths,
- does not always pass.

Manual check:

- confirm the script fails on a controlled forbidden artifact,
- remove any controlled test artifact,
- confirm the script passes on the clean repository.

## Tests

Add semantic tests such as:

- `detects_devanagari_script_for_hindi_text`
- `detects_devanagari_script_for_sanskrit_text`
- `detects_gujarati_script`
- `splits_mixed_latin_devanagari_gujarati_text`
- `keeps_mantra_text_as_stable_script_runs`
- `vfnt_header_rejects_wrong_magic`
- `vfnt_header_accepts_supported_contract`
- `vrun_header_rejects_wrong_version`
- `empty_glyph_cache_reports_missing_glyph`

## Validation commands

Run:

- `cargo fmt --all --check`
- `cargo check --workspace --target riscv32imc-unknown-none-elf`
- `cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings`
- `cargo test -p vaachak-core --all-targets`
- `cargo test -p hal-xteink-x4 --all-targets`
- `cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf`
- `./scripts/check_no_milestone_artifacts.sh .`
- `git diff --check`

Also run any crate-specific target text/font tests if the workspace command set does not execute them automatically.

## Acceptance criteria

Accept the work only when:

- the guard script correctly fails on forbidden delivery artifacts,
- the guard script passes on a clean repository,
- `.vfnt` contract structs and validation helpers exist,
- `.vrun` contract structs and validation helpers exist,
- glyph cache contract exists and remains placeholder-only,
- `split_script_runs` returns all runs for mixed Latin/Devanagari/Gujarati text,
- the reader renderer is not wired to the new contracts,
- Indic shaping is not claimed as complete,
- Wi-Fi, reader, library, bookmarks, settings, sleep image, daily mantra, and category dashboard behavior are untouched,
- all validation commands pass.

## Follow-up work

Future tasks can add:

- SD-card font discovery,
- `.vfnt` binary parser,
- host-side `.vfnt` generator,
- host/mobile/server `.vrun` generator using a shaping engine,
- X4 bitmap glyph renderer,
- reader integration,
- Daily Mantra and Sleep Screen rendering through prepared glyph runs,
- upload/manage fonts through Wi-Fi Transfer.

# Vaachak OS Plan: VFNT Parser and Bounds-Safe Glyph Lookup

## Goal

Add a safe parser for Vaachak compact font assets and expose bounds-checked glyph lookup.

This work makes the existing `.vfnt` contract usable by future font loaders and renderers while keeping the active reader and UI rendering paths unchanged.

## Non-goals

This work does not include:

- reader renderer integration,
- Daily Mantra renderer integration,
- sleep screen renderer integration,
- SD-card font discovery,
- glyph bitmap rendering,
- Indic shaping,
- arbitrary TTF loading,
- EPUB CSS font-family support,
- Wi-Fi Transfer changes,
- settings UI changes.

## Current baseline

The repository already has a shared text area under:

- `target-xteink-x4/src/vaachak_x4/text/`

Existing font foundation modules may include:

- `font_assets.rs`
- `text_run.rs`
- `glyph_cache.rs`
- `script.rs`
- `font_catalog.rs`
- `glyph_run.rs`
- `layout.rs`

The product behavior to preserve:

- Category dashboard is the Home page.
- Network > Wi-Fi Connect works.
- Network > Wi-Fi Transfer works and can transfer files.
- Network > Network Status works.
- Reader, Library, Bookmarks, Settings, Daily Mantra, Sleep Image, and File Browser routes remain unchanged.

## Planned implementation

Prefer updating:

- `target-xteink-x4/src/vaachak_x4/text/font_assets.rs`

Optional new module if the parser makes `font_assets.rs` too large:

- `target-xteink-x4/src/vaachak_x4/text/vfnt.rs`

If a new module is added, update:

- `target-xteink-x4/src/vaachak_x4/text/mod.rs`

Update documentation:

- `docs/font_asset_contract.md`

## Parser contract

Add a safe parser over byte slices.

Recommended public types:

- `VfntParseError`
- `VfntFont`
- `VfntGlyph`

Recommended methods:

- `VfntFont::parse`
- `VfntFont::header`
- `VfntFont::glyph_count`
- `VfntFont::metrics_for_glyph`
- `VfntFont::bitmap_for_glyph`
- `VfntFont::glyph`

The parser should:

- read from `&[u8]`,
- parse all multi-byte values as little-endian,
- avoid unsafe code,
- avoid direct struct casting,
- validate all offsets before slicing,
- use checked arithmetic,
- reject malformed assets early,
- return clear typed errors.

## Required validation

The parser must validate:

- expected magic,
- supported version,
- minimum header length,
- supported bitmap format,
- glyph count,
- metrics table range,
- bitmap index table range,
- bitmap data range,
- bitmap record ranges inside bitmap data.

Malformed assets should never cause panic or out-of-bounds slice access.

## Glyph lookup

Bounds-safe glyph lookup should support:

- metrics lookup by glyph id,
- bitmap index lookup by glyph id,
- combined glyph lookup returning:
  - metrics,
  - bitmap index record,
  - bitmap byte slice.

Rules:

- Linear lookup is acceptable initially.
- Missing glyphs return an explicit error.
- Bitmap slices must always be bounded by the parsed asset.
- Unsupported bitmap formats must be rejected or surfaced as an explicit error.
- Do not claim rendering support yet.

## Suggested errors

Recommended parser errors:

- `TruncatedHeader`
- `InvalidMagic`
- `UnsupportedVersion`
- `UnsupportedBitmapFormat`
- `InvalidHeaderLength`
- `InvalidTableOffset`
- `InvalidTableLength`
- `InvalidGlyphCount`
- `InvalidBitmapRange`
- `InvalidMetricsRecord`
- `InvalidBitmapRecord`
- `GlyphNotFound`

Names may be adjusted to match repository style, but errors should remain clear and specific.

## Tests

Add tests for:

- empty input,
- wrong magic,
- unsupported version,
- unsupported bitmap format,
- invalid header length,
- short metrics table,
- short bitmap index table,
- bitmap record outside bitmap data,
- missing glyph lookup,
- valid minimal font parse,
- known glyph metrics lookup,
- known glyph bitmap slice lookup,
- at least two glyph records in a valid asset.

Suggested test names:

- `vfnt_parse_rejects_empty_input`
- `vfnt_parse_rejects_wrong_magic`
- `vfnt_parse_rejects_unsupported_version`
- `vfnt_parse_rejects_unsupported_bitmap_format`
- `vfnt_parse_rejects_short_metrics_table`
- `vfnt_parse_rejects_short_bitmap_index`
- `vfnt_parse_rejects_bitmap_range_outside_data`
- `vfnt_lookup_returns_missing_for_unknown_glyph`
- `vfnt_lookup_returns_metrics_and_bitmap_for_known_glyph`
- `vfnt_parse_accepts_valid_minimal_font`

## Documentation update

Update the font asset documentation to state:

- `.vfnt` parsing is now byte-slice based,
- header and table bounds are validated,
- bitmap slices are returned only after bounds checks,
- rendering is intentionally not implemented,
- SD-card discovery is intentionally not implemented,
- Indic shaping is intentionally not implemented.

## Validation commands

Run:

- `cargo fmt --all --check`
- `cargo check --workspace --target riscv32imc-unknown-none-elf`
- `cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings`
- `cargo test -p vaachak-core --all-targets`
- `cargo test -p hal-xteink-x4 --all-targets`
- `cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf`
- `./scripts/check_no_milestone_artifacts.sh .`
- `git diff --check`

Also run any direct text-module test harness if normal host tests cannot execute target text tests due to ESP host-incompatible dependencies.

## Acceptance criteria

Accept the work only when:

- `.vfnt` parser exists,
- parser uses no unsafe code,
- parser reads explicit little-endian values,
- parser validates all header and table bounds,
- parser validates bitmap ranges before returning bitmap slices,
- metrics lookup by glyph id works,
- bitmap lookup by glyph id works,
- malformed assets return typed errors,
- valid minimal asset tests pass,
- renderer remains placeholder-only,
- reader rendering is not wired,
- SD-card font discovery is not added,
- Indic shaping is not implemented or claimed,
- existing Wi-Fi, reader, library, bookmarks, settings, sleep image, daily mantra, and category dashboard behavior is untouched,
- all validation commands pass.

## Follow-up work

Future tasks can add:

- SD-card font discovery,
- font catalog entries backed by parsed `.vfnt` assets,
- glyph bitmap renderer,
- `.vrun` parser,
- host-side `.vfnt` generator,
- host/mobile/server prepared-run generator,
- reader integration,
- Daily Mantra and Sleep Screen integration,
- font upload and management through Wi-Fi Transfer.

# Vaachak OS Plan: VFNT Asset Reader and Font Catalog Binding

## Goal

Bind parsed VFNT font assets to the shared Vaachak text font catalog without changing active rendering behavior.

This work introduces a read-only asset reader and semantic loaded-font types so future renderers can select fonts by script.

## Non-goals

This work does not include:

- reader renderer integration,
- Daily Mantra renderer integration,
- sleep screen renderer integration,
- Home or Settings UI integration,
- SD-card font discovery,
- glyph bitmap rendering,
- Indic shaping,
- arbitrary TTF loading,
- EPUB CSS font-family support,
- Wi-Fi Transfer changes,
- committing font binaries.

## Current baseline

The repository already has a shared text area under:

- `target-xteink-x4/src/vaachak_x4/text/`

Existing font foundation modules may include:

- `font_assets.rs`
- `font_catalog.rs`
- `text_run.rs`
- `glyph_cache.rs`
- `script.rs`
- `glyph_run.rs`
- `layout.rs`

The product behavior to preserve:

- Category dashboard is the Home page.
- Network > Wi-Fi Connect works.
- Network > Wi-Fi Transfer works and can transfer files.
- Network > Network Status works.
- Reader, Library, Bookmarks, Settings, Daily Mantra, Sleep Image, and File Browser routes remain unchanged.

## Planned files

Prefer adding:

- `target-xteink-x4/src/vaachak_x4/text/font_asset_reader.rs`

Update:

- `target-xteink-x4/src/vaachak_x4/text/mod.rs`
- `target-xteink-x4/src/vaachak_x4/text/font_catalog.rs`
- `docs/font_asset_contract.md`

Optional:

- `docs/font_catalog_binding.md`

## Asset reader design

The asset reader should parse in-memory VFNT bytes through the existing bounds-safe parser.

Recommended concepts:

- `FontAssetRef`
- `LoadedFontFace`
- `LoadedFontSet`
- `FontAssetReadError`
- `FontAssetReader`
- `StaticFontAssetReader`

`FontAssetRef` should describe:

- semantic asset name,
- borrowed font bytes.

`LoadedFontFace` should describe:

- semantic name,
- script class,
- parsed `VfntFont`.

Rules:

- borrow original font bytes,
- do not copy font data,
- do not load from disk,
- do not scan SD card,
- do not render glyphs,
- do not shape text.

## Catalog binding design

The binding layer should select loaded font faces by script.

Supported script classes:

- Latin
- Devanagari
- Gujarati
- Unknown

Fallback policy:

- exact script match wins,
- Unknown falls back to Latin when Latin exists,
- Devanagari falls back to Latin when Devanagari is missing,
- Gujarati falls back to Latin when Gujarati is missing,
- if Latin is missing but another font exists, fallback can return the first available font,
- if no fonts are loaded, return no font or a clear missing-font error.

This binding only selects a font face. It does not guarantee correct Indic rendering.

## Tests

Tests should use synthetic in-memory VFNT byte arrays.

Required coverage:

- load Latin VFNT face from bytes,
- load Devanagari VFNT face from bytes,
- load Gujarati VFNT face from bytes,
- reject invalid VFNT asset,
- select exact script font,
- fallback to Latin for Unknown,
- fallback to Latin when Devanagari is missing,
- fallback to first available when Latin is missing,
- return no font when no fonts are loaded,
- confirm loaded face borrows original asset bytes.

Tests must not require:

- SD card,
- real font files,
- Noto font binaries,
- generated assets,
- Wi-Fi,
- hardware.

## Documentation update

Update docs to state:

- VFNT parser exists,
- VFNT asset reader can parse byte slices into loaded font faces,
- Font catalog binding can select fonts by ScriptClass,
- no SD-card discovery exists yet,
- no renderer exists yet,
- no reader wiring exists yet,
- no Indic shaping exists yet.

Future pipeline:

- static or SD asset bytes,
- VFNT parser,
- LoadedFontFace,
- FontCatalogBinding,
- glyph renderer,
- app-specific text renderer.

## Validation commands

Run:

- `cargo fmt --all --check`
- `cargo check --workspace --target riscv32imc-unknown-none-elf`
- `cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings`
- `cargo test -p vaachak-core --all-targets`
- `cargo test -p hal-xteink-x4 --all-targets`
- `cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf`
- `./scripts/check_no_milestone_artifacts.sh .`
- `git diff --check`

Also run any direct text-module test harness if normal host tests cannot execute target text tests due to ESP host-incompatible dependencies.

## Acceptance criteria

Accept the work only when:

- VFNT asset reader abstraction exists,
- loaded font face types exist,
- loaded face borrows original VFNT bytes,
- parsed header script maps to the loaded font script,
- font catalog binding selects exact script fonts,
- fallback behavior is deterministic and tested,
- no unsafe code is added,
- no file IO or SD scanning is added,
- no font binaries are committed,
- renderer remains placeholder-only,
- reader rendering is not wired,
- Daily Mantra and Sleep Screen rendering are not wired,
- Indic shaping is not implemented or claimed,
- existing Wi-Fi, reader, library, bookmarks, settings, sleep image, daily mantra, and category dashboard behavior is untouched,
- all validation commands pass.

## Follow-up work

Future tasks can add:

- SD-card font discovery,
- font settings UI,
- glyph bitmap renderer,
- VRUN parser,
- host-side VFNT generator,
- host/mobile/server prepared-run generator,
- Reader integration,
- Daily Mantra integration,
- Sleep Screen integration,
- font upload and management through Wi-Fi Transfer.

# Vaachak OS Plan: Glyph Bitmap Renderer Contract and In-Memory Render Smoke

## Goal

Add a safe low-level renderer for VFNT glyph bitmaps.

This work renders parsed 1bpp VFNT glyph bitmap data into an in-memory monochrome target only. It is a foundation primitive for future text rendering.

## Non-goals

This work does not include:

- e-paper display integration,
- Reader renderer integration,
- Daily Mantra renderer integration,
- Sleep Screen renderer integration,
- Home or Settings UI integration,
- SD-card font discovery,
- glyph cache storage,
- text layout,
- baseline layout,
- Indic shaping,
- arbitrary TTF loading,
- EPUB CSS font-family support,
- Wi-Fi Transfer changes,
- committing font binaries.

## Current baseline

The repository already has a shared text area under:

- `target-xteink-x4/src/vaachak_x4/text/`

Existing font foundation modules may include:

- `font_assets.rs`
- `font_asset_reader.rs`
- `font_catalog.rs`
- `text_run.rs`
- `glyph_cache.rs`
- `script.rs`
- `glyph_run.rs`
- `layout.rs`

The product behavior to preserve:

- Category dashboard is the Home page.
- Network > Wi-Fi Connect works.
- Network > Wi-Fi Transfer works and can transfer files.
- Network > Network Status works.
- Reader, Library, Bookmarks, Settings, Daily Mantra, Sleep Image, and File Browser routes remain unchanged.

## Planned files

Add:

- `target-xteink-x4/src/vaachak_x4/text/glyph_bitmap_renderer.rs`

Update:

- `target-xteink-x4/src/vaachak_x4/text/mod.rs`
- `docs/font_asset_contract.md`

## Renderer design

Add a low-level glyph bitmap renderer that consumes parsed VFNT glyph data.

Recommended concepts:

- `GlyphRenderError`
- `GlyphBlitMode`
- `GlyphPoint`
- `GlyphRenderRequest`
- `MonochromeRenderTarget`
- `MonoBitmapViewMut`
- `GlyphBitmapRenderer`

The renderer should:

- render 1bpp VFNT glyphs,
- use declared source row stride,
- support safe clipping,
- handle negative origins,
- avoid out-of-bounds target writes,
- avoid out-of-bounds source reads,
- allocate nothing in production code,
- perform no file IO,
- perform no hardware IO.

## Bitmap behavior

Initial support is 1bpp only.

Recommended source bit order:

- most-significant bit first within each byte,
- bit 7 is the leftmost pixel,
- bit 0 is the rightmost pixel.

Rules:

- row stride comes from the VFNT bitmap record,
- row stride must be large enough for glyph width,
- bitmap data must be large enough for `row_stride * height`,
- unused trailing bits are ignored,
- unsupported bitmap formats return an explicit error.

## Target behavior

The in-memory render target should be borrowed.

Recommended target shape:

- width,
- height,
- row stride,
- borrowed mutable byte slice.

Rules:

- constructor validates byte slice capacity,
- set/get pixel is bounds-safe,
- target bit order is documented and tested,
- target writes outside bounds are clipped or rejected by API design,
- renderer must not panic on negative origins.

## Blit modes

Transparent mode:

- glyph bit 1 sets target pixel,
- glyph bit 0 leaves target pixel unchanged.

Opaque mode, if implemented:

- glyph bit 1 sets target pixel,
- glyph bit 0 clears target pixel inside glyph bounds.

If Opaque mode is not implemented, document that only Transparent mode exists.

## Tests

Use synthetic glyph bitmap data.

Required coverage:

- render a simple 1bpp glyph,
- x/y placement with non-zero origin,
- right-edge clipping,
- left-edge clipping with negative x,
- bottom-edge clipping,
- row stride larger than minimum,
- transparent mode preserves background for zero bits,
- opaque mode clears background for zero bits if implemented,
- reject row stride too small,
- reject short bitmap data,
- reject unsupported bitmap format,
- target set/get pixel round trip.

Tests must not require:

- SD card,
- real font files,
- Noto font binaries,
- generated font assets,
- Wi-Fi,
- hardware.

## Documentation update

Update docs to state:

- glyph bitmap renderer exists,
- renderer is in-memory only,
- renderer supports 1bpp VFNT glyph bitmaps,
- renderer supports clipping,
- renderer does not perform text layout,
- renderer does not perform baseline positioning,
- renderer does not perform shaping,
- renderer is not wired to active UI or e-paper display.

Future pipeline:

- static or SD asset bytes,
- VFNT parser,
- LoadedFontFace,
- FontCatalogBinding,
- shaped or prepared glyph runs,
- glyph bitmap renderer,
- app-specific text renderer,
- e-paper display integration.

## Validation commands

Run:

- `cargo fmt --all --check`
- `cargo check --workspace --target riscv32imc-unknown-none-elf`
- `cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings`
- `cargo test -p vaachak-core --all-targets`
- `cargo test -p hal-xteink-x4 --all-targets`
- `cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf`
- `./scripts/check_no_milestone_artifacts.sh .`
- `git diff --check`

Also run any direct text-module test harness if normal host tests cannot execute target text tests due to ESP host-incompatible dependencies.

## Acceptance criteria

Accept the work only when:

- `glyph_bitmap_renderer.rs` exists,
- the module is exported from `text/mod.rs`,
- 1bpp VFNT glyph bitmap rendering works into an in-memory target,
- renderer clips safely,
- renderer handles negative origin safely,
- renderer uses row stride correctly,
- renderer rejects unsupported bitmap formats,
- renderer rejects invalid source bitmap data,
- production renderer code uses no heap allocation,
- no unsafe code is added,
- no file IO or SD scanning is added,
- no e-paper display wiring is added,
- Reader rendering is not wired,
- Daily Mantra and Sleep Screen rendering are not wired,
- Indic shaping is not implemented or claimed,
- existing Wi-Fi, reader, library, bookmarks, settings, sleep image, daily mantra, and category dashboard behavior is untouched,
- all validation commands pass.

## Follow-up work

Future tasks can add:

- glyph run renderer over multiple positioned glyphs,
- VRUN parser,
- layout integration,
- prepared-run rendering smoke tests,
- Daily Mantra rendering through prepared glyph runs,
- Sleep Screen rendering through prepared glyph runs,
- Reader integration,
- SD-card font discovery,
- font upload and management through Wi-Fi Transfer.

# Vaachak OS Plan: Glyph Run Renderer and Prepared Run In-Memory Smoke

## Goal

Add an in-memory renderer for prepared positioned glyph runs.

This work proves the `.vrun`-style rendering path at the memory-buffer level by rendering multiple positioned glyphs using parsed VFNT font data and the existing glyph bitmap renderer.

## Non-goals

This work does not include:

- e-paper display integration,
- Reader renderer integration,
- Daily Mantra renderer integration,
- Sleep Screen renderer integration,
- Home or Settings UI integration,
- SD-card font discovery,
- glyph cache storage,
- text shaping,
- Unicode reordering,
- full text layout,
- baseline layout,
- arbitrary TTF loading,
- EPUB CSS font-family support,
- Wi-Fi Transfer changes,
- committing font binaries.

## Current baseline

The repository already has a shared text area under:

- `target-xteink-x4/src/vaachak_x4/text/`

Existing font foundation modules may include:

- `font_assets.rs`
- `font_asset_reader.rs`
- `font_catalog.rs`
- `text_run.rs`
- `glyph_cache.rs`
- `glyph_bitmap_renderer.rs`
- `script.rs`
- `glyph_run.rs`
- `layout.rs`

The product behavior to preserve:

- Category dashboard is the Home page.
- Network > Wi-Fi Connect works.
- Network > Wi-Fi Transfer works and can transfer files.
- Network > Network Status works.
- Reader, Library, Bookmarks, Settings, Daily Mantra, Sleep Image, and File Browser routes remain unchanged.

## Planned files

Add:

- `target-xteink-x4/src/vaachak_x4/text/glyph_run_renderer.rs`

Update:

- `target-xteink-x4/src/vaachak_x4/text/mod.rs`
- `docs/font_asset_contract.md`

Optional updates:

- `target-xteink-x4/src/vaachak_x4/text/glyph_cache.rs`
- `target-xteink-x4/src/vaachak_x4/text/glyph_run.rs`

Only update optional files if needed to reuse existing positioned glyph or prepared-run contracts cleanly.

## Renderer design

Add a low-level glyph run renderer that consumes prepared positioned glyph records.

Recommended concepts:

- `GlyphRunRenderError`
- `GlyphRunRenderOptions`
- `GlyphRunRenderRequest`
- `PreparedFontLookup`
- `SingleFontLookup`
- `LoadedPreparedFont`
- `SliceFontLookup`
- `GlyphRunRenderer`

The renderer should:

- render multiple positioned glyph records,
- use prepared x/y positions directly,
- use `VfntFont` glyph lookup,
- use `GlyphBitmapRenderer` for individual glyph blits,
- render into a `MonochromeRenderTarget`,
- support transparent and opaque blit modes,
- render glyphs in order,
- return clear errors,
- allocate nothing in production code,
- perform no file IO,
- perform no hardware IO.

## Prepared glyph behavior

Prepared glyph records are assumed to already have:

- font id,
- glyph id,
- x position,
- y position,
- optional advance values,
- optional cluster/source mapping.

Rules:

- do not shape text,
- do not reorder glyphs,
- do not do line breaking,
- do not normalize Unicode,
- do not apply font fallback,
- do not interpret clusters for rendering yet,
- use x/y as low-level draw positions for this smoke renderer.

## Font lookup behavior

Initial support may include:

- `SingleFontLookup` for simple one-font prepared runs,
- `SliceFontLookup` for multiple fonts if straightforward.

Rules:

- lookup is borrowed and allocation-free,
- missing font returns explicit error,
- missing glyph returns explicit error,
- unsupported glyph format returns explicit error,
- no SD-card loading,
- no font file discovery.

## Error handling

Recommended errors:

- missing font,
- missing glyph,
- unsupported bitmap format,
- invalid run,
- invalid glyph record,
- glyph render failure.

Error mapping:

- glyph not found from VFNT lookup maps to missing glyph,
- unsupported bitmap format maps to unsupported bitmap format,
- other glyph bitmap renderer failures map to render failure or a more specific error.

## Tests

Use synthetic in-memory VFNT bytes and prepared glyph records.

Required coverage:

- render multiple positioned glyphs into memory,
- render glyphs in order,
- empty run is no-op,
- missing font returns error,
- missing glyph returns error,
- unsupported bitmap format returns error,
- transparent mode preserves background,
- opaque mode clears zero-bit background,
- each glyph position is honored,
- clipping works through the bitmap renderer,
- single-font lookup returns only matching font,
- slice-backed lookup selects requested font if implemented.

Tests must not require:

- SD card,
- real font files,
- Noto font binaries,
- generated font assets,
- Wi-Fi,
- hardware.

## Documentation update

Update docs to state:

- glyph run renderer exists,
- renderer is in-memory only,
- renderer consumes prepared positioned glyph records,
- renderer uses VFNT fonts and the glyph bitmap renderer,
- renderer does not shape text,
- renderer does not do full layout,
- renderer is not wired to active UI or e-paper display.

Future pipeline:

- Unicode text,
- script runs,
- font fallback,
- shaping/preparation,
- positioned glyph records,
- glyph run renderer,
- app-specific text renderer,
- e-paper display integration.

## Validation commands

Run:

- `cargo fmt --all --check`
- `cargo check --workspace --target riscv32imc-unknown-none-elf`
- `cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings`
- `cargo test -p vaachak-core --all-targets`
- `cargo test -p hal-xteink-x4 --all-targets`
- `cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf`
- `./scripts/check_no_milestone_artifacts.sh .`
- `git diff --check`

Also run any direct text-module test harness if normal host tests cannot execute target text tests due to ESP host-incompatible dependencies.

## Acceptance criteria

Accept the work only when:

- `glyph_run_renderer.rs` exists,
- the module is exported from `text/mod.rs`,
- multiple positioned glyphs render into an in-memory target,
- renderer reuses the VFNT parser,
- renderer reuses the glyph bitmap renderer,
- empty runs are safe no-ops,
- missing fonts return clear errors,
- missing glyphs return clear errors,
- unsupported bitmap formats return clear errors,
- production renderer code uses no heap allocation,
- no unsafe code is added,
- no file IO or SD scanning is added,
- no e-paper display wiring is added,
- Reader rendering is not wired,
- Daily Mantra and Sleep Screen rendering are not wired,
- Indic shaping is not implemented or claimed,
- existing Wi-Fi, reader, library, bookmarks, settings, sleep image, daily mantra, and category dashboard behavior is untouched,
- all validation commands pass.

## Follow-up work

Future tasks can add:

- `.vrun` byte parser,
- prepared-run cache reader,
- app-level prepared text renderer,
- font diagnostic screen,
- Daily Mantra rendering through prepared glyph runs,
- Sleep Screen rendering through prepared glyph runs,
- Reader integration,
- SD-card font discovery,
- host-side font and run generator,
- font upload and management through Wi-Fi Transfer.

# Vaachak OS Plan: Prepared TXT Book Smoke

## Goal

Add the first Reader-visible prepared text rendering path for a mixed English + Devanagari TXT book.

The prepared cache is generated offline. The X4 Reader detects the prepared cache and renders prepared pages from VFNT/VRUN-style assets.

## Non-goals

This work does not include:

- EPUB support,
- on-device Indic shaping,
- arbitrary TTF loading,
- general SD-card font discovery,
- Reader-wide custom font settings,
- Daily Mantra renderer integration,
- Sleep Screen renderer integration,
- Home or Settings font integration,
- Wi-Fi Transfer changes,
- committing large font binaries.

## Current baseline

The product behavior to preserve:

- Category dashboard is the Home page.
- Network > Wi-Fi Connect works.
- Network > Wi-Fi Transfer works and can transfer files.
- Network > Network Status works.
- Reader, Library, Bookmarks, Settings, Daily Mantra, Sleep Image, and File Browser routes work.

Existing font foundation may include:

- VFNT parser,
- VFNT asset reader,
- FontCatalog binding,
- glyph bitmap renderer,
- prepared glyph run renderer,
- script run splitter.

## Planned cache layout

Use an 8.3-friendly SD layout:

- `/FCACHE/<BOOKID>/META.TXT`
- `/FCACHE/<BOOKID>/FONTS.IDX`
- `/FCACHE/<BOOKID>/LAT18.VFN`
- `/FCACHE/<BOOKID>/DEV22.VFN`
- `/FCACHE/<BOOKID>/PAGES.IDX`
- `/FCACHE/<BOOKID>/P000.VRN`

`<BOOKID>` must match the existing deterministic book id/path id used by Reader state where possible.

`.VFN` files contain VFNT-magic font data.

`.VRN` files contain VRUN-magic prepared positioned glyph data.

## Metadata format

`META.TXT` should be simple line-based metadata:

- `book_id=<8HEX>`
- `source=<book path>`
- `title=<title>`
- `page_count=<number>`
- `latin_font=LAT18.VFN`
- `devanagari_font=DEV22.VFN`
- `pages=PAGES.IDX`

`FONTS.IDX` should map scripts to files:

- `Latin=LAT18.VFN`
- `Devanagari=DEV22.VFN`

`PAGES.IDX` should list prepared pages:

- `P000.VRN`
- `P001.VRN`

## Offline generator

Add a semantic offline generator under:

- `tools/prepared_txt_smoke/`

The generator should:

- take a mixed English + Devanagari TXT path,
- compute or accept the matching book id,
- generate the prepared cache directory,
- generate tiny synthetic VFNT font files,
- generate one or more prepared VRN page files,
- use no network access,
- require no real Noto font binaries,
- write output to a user-provided destination,
- not commit generated output by default.

Suggested sample text:

- `Prepared TXT Smoke`
- `Om Namah Shivaya`
- `ॐ नमः शिवाय`
- `Dharma and devotion`

The smoke generator can use synthetic glyph shapes. This proves the prepared rendering path, not full Devanagari shaping correctness.

## Reader detection

When opening a TXT file:

1. Compute the existing book id for the TXT.
2. Check `/FCACHE/<BOOKID>/META.TXT`.
3. Validate metadata.
4. Load and validate `FONTS.IDX`.
5. Load and validate required `.VFN` font files.
6. Load `PAGES.IDX`.
7. Load the first `.VRN` page.
8. Render prepared page glyph records.
9. Preserve Back behavior.
10. If cache is missing, use existing TXT Reader behavior.

Invalid cache behavior:

- Prefer safe fallback to existing TXT Reader if possible.
- Otherwise show a clear error with Back.

## Prepared page rendering

Prepared page rendering should:

- parse page records bounds-safely,
- use VFNT fonts,
- draw positioned glyphs,
- render one page at a time,
- preserve existing Reader navigation semantics,
- avoid low-level SSD1677 behavior changes,
- avoid on-device shaping.

If shared target text modules are not importable from the active Reader:

- do not create dependency cycles,
- use a small temporary active-reader bridge,
- keep contracts aligned with VFNT/VRN docs,
- document future consolidation.

## Tests

Required coverage where practical:

- detects existing prepared cache by book id,
- missing prepared cache falls back to TXT Reader,
- rejects mismatched book id,
- parses page index,
- parses font index for Latin and Devanagari,
- rejects missing font asset,
- rejects invalid VFNT asset,
- parses prepared page with multiple glyphs,
- rejects bad prepared page magic,
- rejects truncated prepared glyph records,
- generator outputs expected files.

Tests must not require:

- SD card hardware,
- real font files,
- Noto font binaries,
- network access,
- Wi-Fi,
- e-paper display hardware.

## Physical-device test

After flashing:

1. Copy the mixed TXT book to SD.
2. Run the offline generator for that TXT.
3. Copy `/FCACHE/<BOOKID>/...` to SD.
4. Boot X4.
5. Open Reader > Library.
6. Open the TXT book.
7. Confirm the prepared page renders on X4.
8. Press Back and confirm return to Library.
9. Rename or remove `/FCACHE/<BOOKID>`.
10. Reopen the TXT and confirm existing TXT Reader fallback still works.

## Validation commands

Run:

- `cargo fmt --all --check`
- `cargo check --workspace --target riscv32imc-unknown-none-elf`
- `cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings`
- `cargo test -p vaachak-core --all-targets`
- `cargo test -p hal-xteink-x4 --all-targets`
- `cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf`
- `./scripts/check_no_milestone_artifacts.sh .`
- `git diff --check`

Also run:

- offline generator into a temporary output directory,
- list generated cache files,
- remove generated temporary output,
- direct text-module harness if needed.

## Acceptance criteria

Accept the work only when:

- prepared TXT cache generator exists,
- cache layout is 8.3-friendly,
- Reader detects valid prepared cache for a TXT file,
- Reader renders at least one prepared page on X4,
- missing cache preserves existing TXT Reader fallback,
- Back returns to Library/Files,
- EPUB support is not added,
- on-device Indic shaping is not added,
- arbitrary TTF loading is not added,
- general SD-card font discovery is not added,
- large font binaries are not committed,
- unsafe code is not added,
- existing Wi-Fi, dashboard, settings, bookmarks, daily mantra, sleep image, and normal reader behavior are not broken,
- all validation commands pass.

## Follow-up work

Future tasks can add:

- real host-side VFNT generation from Noto fonts,
- host-side Devanagari shaping,
- prepared EPUB smoke,
- SD-card font discovery,
- Reader font settings,
- Daily Mantra prepared rendering,
- Sleep Screen prepared rendering,
- font upload and management through Wi-Fi Transfer.

# Vaachak OS Plan: Real VFNT Generator and Prepared TXT Devanagari Smoke

## Goal

Replace the synthetic prepared TXT smoke font generation with real host-generated VFNT assets and shaped VRN glyph runs.

The target smoke case is one mixed English + Devanagari TXT book. The X4 Reader should render Latin text with real Latin glyphs and Devanagari text with real Devanagari glyphs from the prepared cache.

## Non-goals

This work does not include:

- EPUB support,
- on-device Indic shaping,
- arbitrary TTF loading on X4,
- general SD-card font discovery,
- Reader-wide font settings,
- Daily Mantra renderer integration,
- Sleep Screen renderer integration,
- Home or Settings font UI,
- Wi-Fi Transfer changes,
- committing large font binaries,
- committing generated cache output by default.

## Current baseline

The product behavior to preserve:

- Category dashboard is the Home page.
- Network > Wi-Fi Connect works.
- Network > Wi-Fi Transfer works and can transfer files.
- Network > Network Status works.
- Reader, Library, Bookmarks, Settings, Daily Mantra, Sleep Image, and File Browser routes work.
- Prepared TXT cache detection already exists.
- Active Reader can load `/FCACHE/<BOOKID>/META.TXT`.
- Active Reader can load `FONTS.IDX`, `PAGES.IDX`, `.VFN` files, and `.VRN` page files.
- Active Reader can render prepared page glyph records.

## Planned host tool

Add a host-side generator under:

- `tools/prepared_txt_real_vfnt/`

Preferred implementation:

- Rust host tool,
- rustybuzz for shaping,
- fontdue or another host rasterizer,
- ttf-parser if needed.

The host tool must not become an ESP32 firmware dependency.

## Host tool inputs

Required arguments:

- `--book <path to TXT>`
- `--device-path <path as seen by X4 Reader>`
- `--latin-font <path to NotoSans-Regular.ttf>`
- `--devanagari-font <path to NotoSansDevanagari-Regular.ttf>`
- `--out <path to FCACHE output directory>`

Optional arguments:

- `--title <title>`
- `--latin-size <px>`
- `--devanagari-size <px>`
- `--line-height <px>`
- `--page-width <px>`
- `--page-height <px>`
- `--margin-x <px>`
- `--margin-y <px>`

Example:

```bash
cd tools/prepared_txt_real_vfnt
cargo run --release -- \
  --book ../prepared_txt_smoke/MIXED.TXT \
  --device-path MIXED.TXT \
  --latin-font /path/to/NotoSans-Regular.ttf \
  --devanagari-font /path/to/NotoSansDevanagari-Regular.ttf \
  --out /Volumes/X4SD/FCACHE


  # Vaachak OS Plan: Network Time Foundation and Clock Screen

## Goal

Add a network-backed time foundation for Vaachak OS.

The user can open System > Date & Time, view cached time/date/day, and manually sync time through Wi-Fi + NTP. Network Status shows time sync status. Home shows time/date and battery status instead of category/app count.

## Non-goals

This work does not include:

- Calendar app,
- Hindu calendar,
- timezone Settings UI,
- automatic boot sync,
- browser SD card manager,
- Reader changes,
- Wi-Fi Transfer changes,
- Daily Mantra changes,
- Sleep Image changes,
- custom font changes.

## Current baseline

The product behavior to preserve:

- Category dashboard is the Home page.
- Network > Wi-Fi Connect works.
- Network > Wi-Fi Transfer works and can transfer files.
- Network > Network Status works.
- Reader, Library, Bookmarks, Settings, Daily Mantra, Sleep Image, and File Browser routes work.

## User-visible behavior

### System > Date & Time

The screen should show:

- current cached or estimated time,
- weekday,
- date,
- timezone,
- sync status,
- last sync result.

Controls:

- Select starts Wi-Fi + NTP sync.
- Back returns to the System category.

### Network Status

Add a Time Sync status line, such as:

- `Time Sync: Never synced`
- `Time Sync: Synced 10:42 PM`
- `Time Sync: Failed: timeout`

Network Status must not start sync automatically.

### Home

Replace current category/app count text with time/date and battery status.

Examples:

- `Tue May 5  10:42 PM`
- `Batt 87%`

Fallback:

- `Time unsynced`
- `Batt --`

Home must not start Wi-Fi or NTP.

## Time cache

Recommended SD path:

- `/_x4/TIME.TXT`

Recommended line-based format:

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
- sync failure should not destroy previous successful cached time,
- Wi-Fi password must never be written or logged.

## Timezone

Use:

- `America/New_York`

For this deliverable, keep timezone centralized as a constant or config object.

Document DST behavior. If full DST support is not implemented, clearly state that the display uses a fixed offset until timezone settings are added.

## NTP design

Use existing Wi-Fi credentials.

NTP behavior:

- connect to Wi-Fi only after explicit sync request,
- query NTP over UDP port 123,
- parse a 48-byte NTP response,
- convert NTP timestamp to Unix epoch by subtracting 2208988800 seconds,
- update `TIME.TXT` on success,
- report clear errors on failure.

Suggested NTP servers:

- `pool.ntp.org`
- `time.google.com`
- `time.cloudflare.com`

Use fallback servers only if existing DNS/network code supports it cleanly.

## State model

Recommended sync states:

- Unsynced
- Cached
- Syncing
- Synced
- Failed

The display should never block indefinitely while syncing.

## Tests

Add pure helper tests where practical:

- parse valid `TIME.TXT`,
- handle missing/corrupt cache,
- format known epoch as weekday/date/time,
- convert NTP timestamp to Unix timestamp,
- reject short NTP responses,
- reject invalid NTP transmit timestamp,
- preserve cached time after sync error,
- format Network Status line for synced and unsynced cases,
- format Home status line for synced and unsynced cases.

## Physical-device test

After flashing:

1. Boot X4.
2. Confirm Home dashboard appears.
3. Confirm Home shows time/date fallback and battery.
4. Open System > Date & Time.
5. Confirm unsynced/cached status displays.
6. Press Select.
7. Confirm Wi-Fi connects using saved credentials.
8. Confirm NTP sync completes or clear error displays.
9. Confirm Date & Time shows day/date/time.
10. Back to System.
11. Open Network > Network Status.
12. Confirm Time Sync line.
13. Return Home.
14. Confirm Home shows time/date and battery.
15. Reboot.
16. Confirm cached time state loads.
17. Confirm Wi-Fi Transfer still works.
18. Confirm Reader, Settings, Daily Mantra, Bookmarks, Sleep Image routes still work.

## Validation commands

Run:

- `cargo fmt --all --check`
- `cargo check --workspace --target riscv32imc-unknown-none-elf`
- `cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings`
- `cargo test -p vaachak-core --all-targets`
- `cargo test -p hal-xteink-x4 --all-targets`
- `cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf`
- `./scripts/check_no_milestone_artifacts.sh .`
- `git diff --check`

## Acceptance criteria

Accept the work only when:

- System > Date & Time exists,
- Date & Time shows cached time/date/day,
- Date & Time shows sync status and last result,
- Select starts Wi-Fi + NTP sync,
- Back returns to System category,
- Network Status includes Time Sync line,
- Home shows time/date and battery instead of category/app count,
- Home does not trigger Wi-Fi,
- missing/corrupt time cache is safe,
- Wi-Fi password is never displayed or logged,
- timezone is centralized as America/New_York for now,
- existing Wi-Fi Transfer and Reader behavior are not broken,
- Calendar and Hindu calendar are not added yet,
- all validation commands pass.

## Follow-up work

Future tasks can add:

- timezone setting,
- automatic optional sync policy,
- Gregorian Calendar app,
- Hindu Calendar app,
- browser SD card manager,
- time display on sleep screen,
- festival calendar integration.