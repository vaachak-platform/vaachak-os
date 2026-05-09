# Consolidated Validation

## Final native hardware state

```bash
cargo fmt --all
./scripts/validate_vaachak_hardware_runtime_final_acceptance.sh
./scripts/validate_hardware_physical_full_migration_consolidation.sh
./scripts/validate_hardware_physical_full_migration_cleanup.sh
./scripts/validate_pulp_hardware_reference_deprecation_audit.sh
./scripts/validate_pulp_hardware_dead_path_quarantine.sh
./scripts/validate_pulp_hardware_dead_path_removal.sh
./scripts/validate_vendor_pulp_os_scope_reduction.sh
./scripts/validate_vaachak_docs_final_native_hardware_state.sh
cargo build
```

Expected markers include:

```text
vaachak_hardware_runtime_final_acceptance=ok
hardware_physical_full_migration_consolidation=ok
vendor_pulp_os_scope_reduction=ok
vaachak_docs_final_native_hardware_state=ok
```
