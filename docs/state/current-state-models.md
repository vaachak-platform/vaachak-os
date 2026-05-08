# Current state models

This document records the first controlled extraction of Vaachak-owned state models.
It does not move hardware behavior, SD I/O, Wi-Fi runtime, display drawing, or reader rendering out of the active Pulp-derived runtime.

## Ownership boundary

Vaachak-owned in `vaachak-core`:

- Settings/state model names and defaults
- Reader preference model
- Sleep image mode model
- Network time status model
- Wi-Fi Transfer configuration model
- Compatibility constants for current X4 SD files
- Parse/write helpers that are host-testable

Still owned by the active runtime:

- Actual SD reads and writes
- Actual Settings screen behavior
- Actual Reader behavior
- Actual Wi-Fi server and HTTP upload handlers
- Actual display drawing and refresh behavior
- Actual input scan/debounce behavior

## Compatibility paths

Current runtime compatibility paths are preserved:

```text
_X4/SETTINGS.TXT
SLPMODE.TXT
TIME.TXT
/FCACHE/<BOOKID>
/FCACHE/15D1296A
```

The model constants live in `core/src/models/state.rs`:

```text
X4_SETTINGS_COMPAT_PATH = "_X4/SETTINGS.TXT"
X4_SLEEP_IMAGE_MODE_FILE = "SLPMODE.TXT"
X4_TIME_STATE_FILE = "TIME.TXT"
X4_FCACHE_ROOT = "/FCACHE"
X4_DEFAULT_FCACHE_TARGET = "/FCACHE/15D1296A"
```

## Reader preferences

The core model mirrors current runtime keys:

```text
book_font=<0..4>
reading_theme=<0..3>
show_progress=<0|1>
prepared_font_profile=<0..2>
prepared_fallback_policy=<0..2>
```

The model clamps out-of-range values to current runtime limits.

## Sleep image mode

The core model accepts the current persisted values:

```text
daily
fast-daily
static
cached
text
no-redraw
off
```

`off` is accepted as a compatibility alias for `no-redraw`.

## Date & Time state

The core model mirrors the current time cache keys:

```text
timezone=America/New_York
last_sync_unix=<unix seconds>
last_sync_monotonic_ms=<uptime milliseconds>
last_sync_ok=<0|1>
last_sync_source=ntp
last_sync_error=<short text>
last_sync_ip=<ip text>
display_offset_minutes=<minutes>
```

Status behavior:

- `Live`: cached sync succeeded and same-boot uptime can advance the clock.
- `Cached`: a previous sync exists but live same-boot continuity is not currently trusted.
- `Unsynced`: no cached sync exists.

Retry failures preserve cached time instead of clearing it.

## Wi-Fi Transfer config

The core model owns safe defaults for the already-working transfer flow:

```text
target_folder=/FCACHE/15D1296A
chunk_size_bytes=256
delay_between_chunks_ms=250
delay_between_files_ms=600
max_retries=20
```

The transfer configuration model does not store or display a Wi-Fi password.
Credential storage remains a runtime concern through the existing settings file.

## Validation

Run:

```bash
./scripts/validate_state_model_ownership.sh
```

This checks formatting, cleanup guard, core tests, host checks, embedded checks, the root release build, and the active Pulp-derived firmware build.
