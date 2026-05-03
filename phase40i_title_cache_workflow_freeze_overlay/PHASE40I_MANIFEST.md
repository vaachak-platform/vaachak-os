# Phase 40I — Title Cache Workflow Freeze and Regression Baseline

Purpose:
- Freeze the accepted Phase 40G/40H title-cache workflow.
- Capture a reproducible SD/title-cache baseline.
- Guard against reintroducing TXT body-title scanning.
- Guard against bad Project Gutenberg/body/license phrases in `_X4/TITLES.BIN`.

Files added:
- `target-xteink-x4/src/vaachak_x4/runtime/state_io_title_cache_workflow_freeze.rs`
- `target-xteink-x4/src/vaachak_x4/runtime/state_io_title_cache_workflow_freeze_acceptance.rs`

Files changed:
- `target-xteink-x4/src/vaachak_x4/runtime.rs`

Expected markers:
- `phase40i=x4-title-cache-workflow-freeze-ok`
- `phase40i-acceptance=x4-title-cache-workflow-freeze-report-ok`

No UX behavior changes are introduced.
