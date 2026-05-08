# Hardware Runtime Executor Extraction

Status marker:

```text
hardware_runtime_executor_extraction=ok
```

## Purpose

This deliverable moves a broad Vaachak-owned hardware executor layer into
`target-xteink-x4` without replacing the working low-level Pulp runtime.

The extraction clubs the accepted hardware ownership stack into one runtime
executor entrypoint:

```text
target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor.rs
```

The active low-level executor remains Pulp-compatible so current boot, display,
input, SD/storage, reader, file-browser, and app navigation behavior are
preserved while executor ownership is migrated safely.

## Moved into Vaachak

Vaachak now owns broad executor entrypoints for these domains:

| Domain | Vaachak entrypoint | Active low-level backend |
| --- | --- | --- |
| SPI bus runtime | `spi_executor_bridge.rs` | `PulpCompatibility` |
| SD probe/mount lifecycle | `storage_executor_bridge.rs` | `PulpCompatibility` |
| FAT/storage runtime boundary | `storage_executor_bridge.rs` | `PulpCompatibility` |
| Display runtime boundary | `display_executor_bridge.rs` | `PulpCompatibility` |
| Input runtime boundary | `input_executor_bridge.rs` | `PulpCompatibility` |

The shared backend descriptor is:

```text
target-xteink-x4/src/vaachak_x4/physical/hardware_executor_pulp_backend.rs
```

## SPI executor entrypoint

Vaachak owns SPI executor intent metadata for:

```text
transaction intent
display transaction ownership
SD transaction ownership
safe arbitration handoff
```

The existing Pulp-compatible backend still owns physical SPI transfer execution
and chip-select toggling.

## SD probe/mount lifecycle entrypoint

Vaachak owns lifecycle intent routing for:

```text
card-present intent
probe intent
mount intent
storage-available state
```

The low-level SD/MMC and FAT executor remains Pulp-compatible.

## FAT/storage runtime boundary

Vaachak owns executor entrypoints for the current safe storage surface:

```text
library/file metadata access
file open/read intent
directory listing intent
state/cache path resolution
```

This deliverable does not introduce destructive FAT/storage operations.
Existing Pulp FAT behavior remains active underneath.

## Display runtime boundary

Vaachak owns display executor intent metadata for:

```text
full refresh intent
partial refresh intent
clear/sleep/render intent metadata
```

SSD1677 draw algorithms, full-refresh execution, partial-refresh execution, and
busy-wait behavior are not rewritten in this deliverable.

## Input runtime boundary

Vaachak owns input executor intent metadata for:

```text
button scan intent
ADC ladder ownership metadata
debounce/repeat handoff
navigation handoff
```

Button ADC scan, debounce/repeat behavior, and app navigation routing remain in
the active Pulp-compatible executor.

## Explicit non-goals

This deliverable does not change:

```text
reader/file-browser UX behavior
app navigation behavior
SSD1677 draw algorithms
input debounce/navigation behavior
low-level SD/MMC behavior
physical SPI transfer behavior
chip-select toggling behavior
```

It also does not introduce destructive FAT/storage behavior.

## Static validation

Run:

```bash
./scripts/validate_hardware_runtime_executor_extraction.sh
```

Expected output:

```text
hardware_runtime_executor_extraction=ok
```

The validator checks that:

```text
Vaachak owns the consolidated hardware executor entrypoint
SPI, storage, display, and input executor bridges exist
Pulp-compatible backend remains active
reader/file-browser behavior was not modified
app navigation behavior was not modified
no display draw algorithm rewrite was introduced
no input debounce/navigation rewrite was introduced
no SD/FAT destructive behavior was introduced accidentally
existing hardware ownership modules are still referenced by the executor layer
```

## Hardware validation

After `cargo fmt --all`, validator, and `cargo build`, flash with the usual
workflow and verify:

```text
boot succeeds
Home/category dashboard appears
buttons/navigation still work
file browser opens
SD file listing still works
TXT/EPUB open paths still work
display refresh looks unchanged
no SD mount/probe regression
no input freeze/regression
```
