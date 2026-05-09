# Final Hardware Validation

Run this before committing and before pushing the full Vaachak hardware migration checkpoint.

## Static validation

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

Expected result:

```text
vaachak_hardware_runtime_final_acceptance=ok
hardware_physical_full_migration_consolidation=ok
hardware_physical_full_migration_cleanup=ok
pulp_hardware_reference_deprecation_audit=ok
pulp_hardware_dead_path_quarantine=ok
pulp_hardware_dead_path_removal=ok
vendor_pulp_os_scope_reduction=ok
vaachak_docs_and_artifact_cleanup=ok
Finished `dev` profile ...
```

## Device flash

```bash
cargo run --release
```

## Device smoke checklist

```text
[ ] Device boots normally
[ ] Display initializes
[ ] Home/category dashboard appears
[ ] Full refresh works
[ ] Partial/list refresh works
[ ] All buttons respond correctly
[ ] Direction mapping is unchanged
[ ] No missed press regression
[ ] No double press regression
[ ] Power button behavior is unchanged
[ ] SD card initializes
[ ] Storage availability state is correct
[ ] File browser opens
[ ] SD root listing works
[ ] Nested directory listing works if test files are available
[ ] Long filename/title mapping still works
[ ] TXT files open
[ ] EPUB files open
[ ] Progress/state/cache files still work
[ ] Back navigation works
[ ] No FAT/path/cluster-chain errors
```

## Regression notes

If a regression appears, classify it by subsystem:

```text
Input: button mapping, debounce, repeat, power button
Display: blank screen, stuck BUSY, ghosting, partial refresh movement
Storage physical: SD card init, media state, mount availability
FAT/filesystem: listing, path normalization, LFN/title mapping, cluster traversal
SPI: display and SD both failing, chip-select conflict symptoms
```

Do not delete `vendor/pulp-os` while debugging. It remains useful for non-hardware compatibility comparison until a separate non-hardware vendor retirement audit is complete.
