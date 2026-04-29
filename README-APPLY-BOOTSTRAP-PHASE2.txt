VaachakOS Bootstrap Phase 2 — Real X4 HAL Extraction Plan

Purpose
-------
This pack adds planning and governance documents for extracting the real Xteink X4 HAL from the x4-reader-os-rs proving-ground into vaachak-os.

It intentionally does not move runtime code yet.

Included files
--------------
- docs/x4-hal-extraction-plan.md
- docs/x4-hal-source-map.md
- docs/x4-hal-validation-matrix.md
- docs/x4-hal-porting-backlog.md
- docs/x4-target-runtime-plan.md
- docs/adr/0001-x4-hal-extraction-policy.md
- .github/ISSUE_TEMPLATE/x4-hal-extraction-task.md

Apply
-----
From the vaachak-os repo root:

  unzip -o /path/to/vaachak-os-bootstrap-phase2-x4-hal-extraction-plan.zip

Validate
--------
Because this pack adds docs only, the normal Phase 1 validation should still pass:

  cargo fmt --all
  cargo check --workspace --all-targets
  cargo test --workspace --all-targets
  cargo clippy --workspace --all-targets -- -D warnings

Recommended commit
------------------

  git add docs .github/ISSUE_TEMPLATE README-APPLY-BOOTSTRAP-PHASE2.txt
  git commit -m "Add X4 HAL extraction plan"

Next phase after this
---------------------
Bootstrap Phase 3 should extract one hardware seam at a time, starting with input or power, not display.
