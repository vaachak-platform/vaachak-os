# Hardware Runtime Executor Wiring

Status marker:

```text
hardware_runtime_executor_wiring=ok
```

## Purpose

This deliverable wires selected internal runtime intents through the consolidated
Vaachak hardware runtime executor layer:

```text
target-xteink-x4/src/vaachak_x4/physical/hardware_runtime_executor_wiring.rs
```

It builds on the broad executor extraction layer documented in:

```text
docs/architecture/hardware-runtime-executor-extraction.md
```

The low-level physical executors remain Pulp-compatible so current boot,
display, input, SD/storage, reader, file-browser, and app navigation behavior
remain unchanged.

## Wired runtime paths

The wiring layer declares and validates these selected runtime paths:

| Runtime path | Vaachak executor domain | Active low-level backend |
| --- | --- | --- |
| `BootStorageAvailability` | SD probe/mount lifecycle | `PulpCompatibility` |
| `LibraryDirectoryListing` | FAT/storage | `PulpCompatibility` |
| `ReaderFileOpenIntent` | FAT/storage | `PulpCompatibility` |
| `ReaderFileChunkIntent` | FAT/storage | `PulpCompatibility` |
| `DisplayFullRefreshHandoff` | Display | `PulpCompatibility` |
| `DisplayPartialRefreshHandoff` | Display | `PulpCompatibility` |
| `InputButtonScanHandoff` | Input | `PulpCompatibility` |
| `InputNavigationHandoff` | Input | `PulpCompatibility` |
| `SharedSpiDisplayHandoff` | SPI bus | `PulpCompatibility` |
| `SharedSpiStorageHandoff` | SPI bus | `PulpCompatibility` |

Each path routes through `VaachakHardwareRuntimeExecutor::entry_for(...)` and
then to the domain bridge that was introduced in the executor extraction layer.

## What moved

Vaachak now owns a wiring entrypoint for selected runtime intent paths:

```text
VaachakHardwareRuntimeExecutorWiring
VaachakHardwareRuntimeWiringPulpBackend
```

The wiring layer verifies that runtime intent paths use the consolidated
Vaachak executor surface before reaching the Pulp-compatible backend descriptor.

## What did not move

This deliverable does not rewrite or move:

```text
physical SPI transfer execution
chip-select GPIO toggling
SD/MMC low-level execution
FAT implementation algorithms
SSD1677 draw/full/partial refresh algorithms
button ADC sampling, debounce, or navigation behavior
reader/file-browser UX behavior
app navigation behavior
```

## Static validation

Run:

```bash
./scripts/validate_hardware_runtime_executor_wiring.sh
```

Expected output:

```text
hardware_runtime_executor_wiring=ok
```

The validator checks that:

```text
Vaachak owns the wiring entrypoint
selected runtime paths are present
selected paths route through the consolidated hardware executor
Pulp-compatible backend remains active
SPI, storage, display, and input bridges remain referenced
reader/file-browser UX was not modified
app navigation was not modified
no display draw rewrite was introduced
no input debounce/navigation rewrite was introduced
no FAT destructive behavior was introduced
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
