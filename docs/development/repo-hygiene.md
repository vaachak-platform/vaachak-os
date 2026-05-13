# Repository Hygiene

The repository should not commit generated deliverable artifacts.

Do not commit:

- root-level zip files
- extracted patch/deliverable folders
- one-off `*_repair`, `*_restore`, `*_cleanup`, `*_contract`, or `*_reset` folders
- temporary apply scripts
- temporary patch scripts
- one-off repair/cleanup/feature-slice validator scripts
- Python bytecode/cache folders
- macOS metadata folders
- empty local build/status scratch files

The `scripts/` directory should contain production helper scripts only.

Keep examples, docs, host tools, and production helper scripts. Delete historical deliverable scripts once their behavior has been folded into source or documentation.

## Current production scripts

```text
scripts/activate_daily_mantra_sleep_image.sh
scripts/audit_remaining_pulp_runtime_dependencies.sh
scripts/check_no_milestone_artifacts.sh
scripts/check_repo_hygiene.sh
scripts/clear_sleep_image_cache_hint.sh
scripts/deploy/check_deploy_ready.sh
scripts/erase_x4_otadata_select_app0.sh
scripts/flash_x4_standard_partition_table.sh
scripts/flash_x4_vaachak_app0.sh
scripts/prepare_daily_mantra_sd_assets.sh
scripts/read_x4_partition_table.sh
scripts/validate_x4_flash_ota_slot_policy.sh
scripts/validate_x4_standard_partition_table_compatibility.py
scripts/validate_x4_standard_partition_table_compatibility.sh
scripts/validate_vaachak_wifi_runtime_ownership.py
scripts/validate_vaachak_wifi_runtime_ownership.sh
scripts/verify_active_sleep_image.sh
scripts/verify_daily_mantra_direct_sleep_files.sh
scripts/verify_daily_mantra_sd_assets.sh
scripts/verify_sleep_image_mode.sh
scripts/write_daily_mantra_today_file.sh
scripts/write_sleep_image_cache_hint.sh
scripts/write_sleep_image_mode.sh
```

Run before committing:

```bash
./scripts/check_repo_hygiene.sh
```

## Lua app cleanup and commit preparation

Before committing Lua app work, remove old overlay zip files, extracted overlay folders, and generated apply/patch/validator scripts from previous deliverables. Keep canonical docs and examples only.

Current canonical Lua sample app path root:

```text
examples/sd-card/VAACHAK/APPS
```

Current SD deployment root:

```text
/VAACHAK/APPS
```
