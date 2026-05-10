# Consolidated Validation

## Final native hardware state

```bash
cargo fmt --all
cargo build
```

Expected markers include:

```text
vaachak_hardware_runtime_final_acceptance=ok
hardware_physical_full_migration_consolidation=ok
vendor_pulp_os_scope_reduction=ok
vaachak_docs_final_native_hardware_state=ok
```


Production hygiene check:

```bash
./scripts/check_repo_hygiene.sh
```
