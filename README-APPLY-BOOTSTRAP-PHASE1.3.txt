VaachakOS Bootstrap Phase 1.3 — Clippy Default Impl Fix

Purpose:
- Fix clippy::new_without_default failures introduced by bootstrap placeholder services.
- Adds Default implementations for:
  - PowerManager
  - StorageService
  - ActivityManager

Apply from the vaachak-os repo root:

  unzip -o /path/to/vaachak-os-bootstrap-phase1.3-clippy-defaults-fix.zip

Then run:

  cargo fmt --all
  cargo check --workspace --all-targets
  cargo test --workspace --all-targets
  cargo clippy --workspace --all-targets -- -D warnings

This pack only replaces:
- core/src/services/power_manager.rs
- core/src/services/storage_service.rs
- core/src/ui/activity.rs
