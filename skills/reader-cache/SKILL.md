---
name: reader-cache
description: Use this skill for EPUB/TXT reader pipeline work, pagination, cache layout, page operations, progress save/load, and reader-facing storage behavior. Trigger when the task mentions library, reader, cache, page turns, pagination, EPUB, TXT, TOC, recents, progress resume, or reading-state persistence. Do not use this skill for board bring-up, display driver wiring, sidecar file exchange, or network sync.
---

# Reader Cache Skill

This skill keeps reader work focused on a stable X4-first reading flow.

## Goal

Implement or repair the reading pipeline so that books open reliably from SD, paginate predictably, render through strip-based drawing, and resume progress without introducing speculative features.

## Required source order

Before making changes, read these in this order:

1. `AGENTS.md`
2. `docs/architecture.md`
3. `docs/milestones.md`
4. `docs/sidecar-format.md`
5. `IMPLEMENTATION.md`
6. `PLANS.md`

If these disagree, do not invent a new plan. Follow the higher-priority file.

## Scope

In scope:
- book metadata extraction
- EPUB/TXT loading
- constrained pagination
- cache directory layout
- page operation model
- progress save/load
- recents
- basic TOC if already in milestone scope
- cache invalidation
- reader settings that affect rendering

Out of scope:
- board initialization
- display driver fixes
- sidecar import/export format changes
- live sync
- advanced typography beyond the milestone
- large CSS engines
- PDF support
- generic plugin or app systems

## Hard rules

- Optimize for readable and stable output before rich fidelity.
- Accept a constrained EPUB subset in v1.
- Do not add complex CSS support unless the task explicitly requires it and the milestone allows it.
- Do not introduce DOM-heavy parsing if a simpler streaming approach fits the milestone.
- Keep cache disposable and reading state durable.
- Do not key book identity by file path.
- Do not widen the page operation model without a concrete rendering need.
- Do not add networking to solve a reader-state problem.

## File ownership guidance

Use these boundaries:

- `core/model/`
  - book identity
  - progress model
  - settings model

- `core/reader/`
  - EPUB/TXT loading
  - pagination
  - page ops
  - cache schema
  - progress read/write hooks

- `core/storage/`
  - durable prefs and path helpers

- `boards/` and `drivers/`
  - not for reader business logic

Do not push reader logic into board or driver code.

## Reader pipeline rules

Preserve this basic flow:

book file on SD
-> metadata extraction
-> cache directory preparation
-> constrained parsing
-> layout into page model
-> page ops
-> strip rendering
-> progress save/load

If proposing a different flow, explain exactly why the current one fails.

## Cache rules

- Cache may be rebuilt.
- Progress must survive rebuilds.
- Cache directories should be deterministic from book fingerprint.
- Cache invalidation must be explicit when schema changes.
- Use versioning when practical rather than silently mixing old and new cache data.

## Book identity rules

Never use raw file path as the book identity.

Prefer:
1. reliable EPUB package identifier
2. normalized content hash
3. metadata-based fallback only if necessary

Any new reader work must preserve this rule.

## Expected output style

When you finish:
- name the exact reader behaviors changed
- name any cache schema changes
- state whether cache rebuild is required
- state the expected manual test path: open book, turn page, resume, reopen

## Definition of done

A reader/cache change is only "done" when:
- it improves or preserves open/read/resume stability
- it does not widen scope into sync or networking
- it does not shift reader business logic into board files
- it remains consistent with the X4 memory model