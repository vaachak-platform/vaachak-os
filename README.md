# Vaachak OS

Vaachak OS is the Xteink X4 firmware/runtime track for the Vaachak Platform. The current target is the ESP32-C3 based Xteink X4 with SSD1677 4.26-inch e-paper display, SD-card backed local storage, physical button input, and reader-oriented runtime behavior.

This repository now treats `target-xteink-x4` as the Vaachak-owned hardware/runtime boundary. The historical `vendor/pulp-os` tree remains present only for non-hardware compatibility/import surfaces while the active hardware ownership has moved to Vaachak-native modules.

## Current checkpoint

The accepted checkpoint is:

```text
vaachak_hardware_runtime_final_acceptance=ok
```

The final hardware migration stack has been accepted for:

```text
- SPI physical driver ownership
- SSD1677 display physical driver ownership
- SD/MMC physical driver ownership
- FAT/filesystem algorithm ownership
- input physical sampling ownership
- native hardware behavior consolidation
- Pulp hardware reference audit/quarantine/removal
- vendor/pulp-os scope reduction
```

## Hardware ownership status

| Area | Active owner | Active backend | Pulp hardware fallback |
|---|---|---|---|
| SPI bus / transaction lifecycle | Vaachak `target-xteink-x4` | `VaachakNativeSpiPhysicalDriver` | Disabled |
| SSD1677 display lifecycle | Vaachak `target-xteink-x4` | `VaachakNativeSsd1677PhysicalDriver` | Disabled |
| SD/MMC physical lifecycle | Vaachak `target-xteink-x4` | `VaachakNativeSdMmcPhysicalDriver` | Disabled |
| FAT / filesystem algorithms | Vaachak `target-xteink-x4` | `VaachakNativeFatAlgorithmDriver` | Disabled |
| Input physical sampling | Vaachak `target-xteink-x4` | `VaachakPhysicalSamplingWithPulpAdcGpioReadFallback` | Limited to physical ADC/GPIO read fallback only |

`vendor/pulp-os` is intentionally not deleted yet. Its allowed role is now limited to non-hardware compatibility/import surfaces and documentation-only references captured in the deprecation and scope-reduction docs.

## Primary docs

Start here:

```text
docs/architecture/vaachak-os-hardware-runtime-architecture.md
docs/architecture/vaachak-hardware-runtime-final-acceptance.md
docs/architecture/hardware-physical-full-migration-consolidation.md
docs/architecture/hardware-physical-full-migration-cleanup.md
docs/architecture/vendor-pulp-os-scope-reduction.md
docs/operations/final-hardware-validation.md
docs/operations/github-upload-checklist.md
```

## Validation

Run the final hardware/runtime acceptance checks before committing or pushing:

```bash
cargo fmt --all
./scripts/validate_vaachak_hardware_runtime_final_acceptance.sh
./scripts/validate_hardware_physical_full_migration_consolidation.sh
./scripts/validate_hardware_physical_full_migration_cleanup.sh
./scripts/validate_pulp_hardware_reference_deprecation_audit.sh
./scripts/validate_pulp_hardware_dead_path_quarantine.sh
./scripts/validate_pulp_hardware_dead_path_removal.sh
./scripts/validate_vendor_pulp_os_scope_reduction.sh
./scripts/validate_docs_and_artifact_cleanup.sh
cargo build
```

## Hardware smoke

After a successful build, flash the device using the project’s normal flow:

```bash
cargo run --release
```

Validate on the Xteink X4:

```text
- device boots normally
- display initializes
- full refresh works
- partial/list refresh works
- all buttons respond correctly
- no missed/double button press regression
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
```

## Cleanup before GitHub upload

Remove generated overlay zip files, extracted overlay folders, temporary patch scripts, validator-fix scripts, and accidental `__pycache__` folders:

```bash
./scripts/cleanup_legacy_deliverable_artifacts.sh
```

The cleanup script intentionally preserves repository source files, architecture docs, canonical validators, `vendor/pulp-os`, `target-xteink-x4`, and Cargo project files.

After cleanup:

```bash
git status --short
```

## Suggested commit

```bash
git add README.md \
        docs/architecture \
        docs/operations \
        scripts/cleanup_legacy_deliverable_artifacts.sh \
        scripts/validate_docs_and_artifact_cleanup.sh

git commit -m "Update Vaachak hardware migration docs and cleanup artifacts"
```

## Current migration posture

Vaachak hardware ownership is complete at the architecture/runtime boundary level. The next safe work should focus on stabilization, removing remaining non-hardware vendor dependency only after a separate audit, and tightening runtime tests around hardware smoke paths.
