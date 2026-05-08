# Input Runtime Ownership

Status marker: `input_runtime_owner=ok`

This document is the canonical ownership note for the Xteink X4 input runtime slice.

## Ownership decision

Vaachak now owns the input runtime ownership authority in `target-xteink-x4`:

```text
INPUT_RUNTIME_OWNERSHIP_AUTHORITY_MOVED_TO_VAACHAK = true
ACTIVE_BACKEND = PulpCompatibility
```

The active button/ADC/input executor remains the existing imported Pulp runtime:

```text
vendor/pulp-os imported runtime
```

## What moved

The following moved into Vaachak-owned metadata and ownership entrypoints:

- input runtime identity: `xteink-x4-button-adc-input-runtime`
- ADC/button pin map metadata
- button ladder metadata
- timing policy metadata
- shell/input boundary dependency metadata
- active backend selection: `PulpCompatibility`
- ownership safety report and smoke contract

## Current input hardware map

| Input path | X4 detail | Vaachak ownership status |
|---|---:|---|
| Row 1 ADC ladder | GPIO1 | metadata owned by Vaachak |
| Row 2 ADC ladder | GPIO2 | metadata owned by Vaachak |
| Power button | GPIO3 | metadata owned by Vaachak |
| Row 1 right | 3 mV center | metadata owned by Vaachak |
| Row 1 left | 1113 mV center | metadata owned by Vaachak |
| Row 1 confirm | 1984 mV center | metadata owned by Vaachak |
| Row 1 back | 2556 mV center | metadata owned by Vaachak |
| Row 2 volume down | 3 mV center | metadata owned by Vaachak |
| Row 2 volume up | 1659 mV center | metadata owned by Vaachak |

## Current timing policy metadata

| Setting | Value |
|---|---:|
| oversample count | 4 |
| debounce window | 15 ms |
| long press window | 1000 ms |
| repeat interval | 150 ms |

## Dependency on existing shell/input boundary

This ownership slice documents dependency on the current input stack:

- `VaachakInputBoundary`
- `VaachakActiveInputSemanticMapper`
- `VaachakInputAdcRuntimeBridge`
- Pulp `AppManager` input dispatch remains active

The ownership entrypoint is:

```text
target-xteink-x4/src/vaachak_x4/physical/input_runtime_owner.rs
```

The compatibility backend is:

```text
target-xteink-x4/src/vaachak_x4/physical/input_pulp_backend.rs
```

## What did not move

This slice intentionally does not move active runtime behavior:

- no ADC sampling executor moved to Vaachak
- no button scan loop moved to Vaachak
- no debounce/repeat executor moved to Vaachak
- no navigation dispatch moved to Vaachak
- no reader/file-browser behavior changed
- no display behavior changed
- no storage behavior changed

The active backend remains `PulpCompatibility` and continues to preserve the working imported behavior.

## Validation

Run:

```bash
cargo fmt --all
./scripts/validate_input_runtime_owner.sh
cargo build
```

Expected:

```text
input_runtime_owner=ok
```

## Hardware smoke

After flashing, behavior should be unchanged:

- device boots normally
- Home/category dashboard appears
- buttons still navigate normally
- reader navigation still works
- file browser still opens
- SD file listing still works
- display refresh looks unchanged
- no button lock-up or repeated input regression
