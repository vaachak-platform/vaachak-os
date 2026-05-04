# Phase 39N Overlay Manifest

Phase 39N — Review-Delete-Later Candidate Removal Dry Run Overlay

Files added:
- target-xteink-x4/src/vaachak_x4/runtime/state_io_review_delete_later_removal_dry_run.rs
- target-xteink-x4/src/vaachak_x4/runtime/state_io_review_delete_later_removal_dry_run_acceptance.rs

Files changed:
- target-xteink-x4/src/vaachak_x4/runtime.rs

Scripts added:
- scripts/guard_phase39n_accepted_write_path.sh
- scripts/plan_phase39n_review_delete_later_removal_dry_run.sh
- scripts/accept_phase39n_review_delete_later_removal_dry_run.sh

Expected markers:
- phase39n=x4-review-delete-later-candidate-removal-dry-run-ok
- phase39n-acceptance=x4-review-delete-later-removal-dry-run-report-ok

Scope:
- Dry-run only.
- No deletion.
- No moving/archive.
- Protect active Phase 39I write path.
- Protect Phase 39J/39K/39L/39M metadata.
