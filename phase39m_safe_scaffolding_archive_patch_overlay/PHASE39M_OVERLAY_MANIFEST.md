# Phase 39M Overlay Manifest

Phase 39M — Safe Scaffolding Archive Patch Overlay

Files added:
- target-xteink-x4/src/vaachak_x4/runtime/state_io_safe_scaffolding_archive_patch.rs
- target-xteink-x4/src/vaachak_x4/runtime/state_io_safe_scaffolding_archive_patch_acceptance.rs

Files changed:
- target-xteink-x4/src/vaachak_x4/runtime.rs
- docs/archive/phase38-39-scaffolding/runtime/*.rs
- docs/archive/phase38-39-scaffolding/README.md

Scripts added:
- scripts/guard_phase39m_archive_patch.sh
- scripts/apply_phase39m_archive_runtime_scaffolding.sh
- scripts/accept_phase39m_archive_patch.sh

Expected markers:
- phase39m=x4-safe-scaffolding-archive-patch-ok
- phase39m-acceptance=x4-safe-scaffolding-archive-patch-report-ok

Scope:
- Archive review-archive candidates only.
- Remove their runtime.rs exports.
- Do not touch accepted active reader path.
- Do not touch Phase 39J verification.
- Do not touch Phase 39K/39L freeze/plan metadata.
- Do not archive review-delete-later candidates yet.
