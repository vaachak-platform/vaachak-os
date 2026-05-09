# GitHub Upload Checklist

## 1. Clean generated artifacts

```bash
./scripts/cleanup_legacy_deliverable_artifacts.sh
```

Then inspect:

```bash
git status --short
```

## 2. Validate final checkpoint

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

## 3. Flash and smoke test

```bash
cargo run --release
```

Use `docs/operations/final-hardware-validation.md` for the device checklist.

## 4. Commit docs and cleanup scripts

```bash
git add README.md \
        docs/architecture \
        docs/operations \
        scripts/cleanup_legacy_deliverable_artifacts.sh \
        scripts/validate_docs_and_artifact_cleanup.sh

git commit -m "Update Vaachak hardware migration docs and cleanup artifacts"
```

## 5. Push

```bash
git push
```

## Notes

Do not delete `vendor/pulp-os` in this checkpoint. The current accepted state only reduces it to non-hardware compatibility/import surfaces.
