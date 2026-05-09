# Vaachak Hardware Runtime Final Acceptance

This is the final acceptance gate for the Vaachak-owned Xteink X4 hardware runtime after full hardware migration.

## Accepted native hardware stack

The final acceptance checkpoint expects these migrations to already be accepted:

- SPI physical native driver
- SSD1677 display physical native driver
- SD/MMC physical native driver
- FAT algorithm native driver
- input physical sampling native driver
- full hardware physical migration consolidation
- full hardware physical migration cleanup
- Pulp hardware reference deprecation audit
- Pulp hardware dead-path quarantine
- Pulp hardware dead-path removal
- vendor/pulp-os scope reduction

## Final ownership state

| Area | Final accepted owner |
| --- | --- |
| SPI bus identity, pin map, transaction lifecycle, CS policy | Vaachak `target-xteink-x4` |
| SSD1677 display command sequencing and refresh lifecycle | Vaachak `target-xteink-x4` |
| SD/MMC card lifecycle and block-device policy | Vaachak `target-xteink-x4` |
| FAT/path/list/open/read/write algorithm ownership | Vaachak `target-xteink-x4` |
| X4 input ADC ladder interpretation and sampling classification | Vaachak `target-xteink-x4` |
| Remaining vendor/pulp-os scope | non-hardware compatibility/import surface only |

## What this checkpoint does

- Adds one final acceptance module.
- Adds one final smoke contract.
- Adds one final validator.
- Confirms `vendor/pulp-os` remains present but no unclassified active Pulp hardware fallback remains.
- Confirms full migration consolidation and cleanup are accepted.
- Requires final device smoke after flashing.

## What this checkpoint does not do

- It does not delete `vendor/pulp-os`.
- It does not change app behavior.
- It does not change reader/file-browser UX.
- It does not change display/input/storage/SPI runtime behavior.
- It does not remove non-hardware compatibility/import boundaries.

## Final validation

```bash
cargo fmt --all
./scripts/validate_vaachak_hardware_runtime_final_acceptance.sh
cargo build
```

Expected marker:

```text
vaachak_hardware_runtime_final_acceptance=ok
```

## Final hardware smoke

After the static/build checks pass, flash and validate on-device:

```bash
cargo run --release
```

Expected smoke:

- boots normally
- display initializes
- full refresh works
- partial/list refresh works
- all buttons respond correctly
- SD card initializes
- storage availability state is correct
- file browser opens
- SD root listing works
- nested directory listing works if available
- long filename/title mapping still works
- TXT files open
- EPUB files open
- progress/state/cache files still work
- Back navigation works
- no FAT/path/cluster-chain errors
- no blank/stuck display
- no input regression

## Next possible checkpoint

After this final acceptance passes and is committed, the next safe step is a separate repository hygiene deliverable that removes old validators that are no longer needed and consolidates documentation indexes. That should be separate from this acceptance gate.
