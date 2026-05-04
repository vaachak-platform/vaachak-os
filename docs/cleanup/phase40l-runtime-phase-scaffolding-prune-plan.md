# Phase 40L — Runtime Phase Scaffolding Prune Plan

Phase 40L is **plan-only**.

It prepares the repository for Biscuit UI work by identifying temporary runtime
phase scaffolding that can be removed later without touching accepted behavior.

## Frozen behavior surfaces

Do not change these in Phase 40L:

```text
- Home current-title layout
- Files/Library title display behavior
- TITLEMAP.TSV -> TITLES.BIN title-cache workflow
- EPUB/EPU metadata title support
- TXT/MD body-title scanning disabled
- Footer labels
- Input mapping
- Write lane
- Display geometry / rotation
- Reader pagination / restore
```

## Classification rules

```text
KEEP
- Runtime files used by active behavior.
- Public contracts, physical/runtime APIs, boot/runtime glue.
- Accepted title-cache and deploy workflow behavior.

PRUNE-CANDIDATE
- Pure marker/report/acceptance modules that are not used by behavior.
- Old repair marker files after their behavior has been absorbed.
- Plan/freeze report modules that only expose booleans and strings.

REVIEW
- Files with phase names but unclear runtime references.
- Files exported from runtime.rs and possibly used by scripts/tests.
- Anything under reader, files, input, display, storage, or write-lane paths.

DO-NOT-TOUCH
- vendor/pulp-os app behavior files.
- hal-xteink-x4 behavior files.
- title-cache host tools.
- scripts/deploy and scripts/repo-cleanup.
```

## Phase 40L output

The plan generates:

```text
/tmp/phase40l-runtime-scaffolding-inspection.txt
/tmp/phase40l-runtime-scaffolding-classification.tsv
/tmp/phase40l-runtime-scaffolding-prune-plan.md
/tmp/phase40l-runtime-prune-no-behavior-change-guard.txt
/tmp/phase40l-runtime-phase-scaffolding-prune-plan-acceptance.txt
```

## Future prune phase

Only after reviewing this plan and confirming build/device stability, run a
separate phase:

```text
Phase 40M — Guarded Runtime Scaffolding Prune Patch
```

Phase 40M may delete files, but Phase 40L does not.
