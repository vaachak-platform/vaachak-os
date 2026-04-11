---
name: sidecar-state
description: Use this skill for offline reading-state exchange, including sidecar schema, import/export logic, merge rules, validation, and book-fingerprint matching. Trigger when the task mentions sidecar files, reading-state import/export, manual sync, progress merge, bookmark merge, SD-based state transfer, or X4-friendly exchange format. Do not use this skill for live sync servers, auth, crypto, network APIs, or board bring-up.
---

# Sidecar State Skill

This skill exists to preserve the local-first v1 plan and prevent accidental drift back into fragile live sync work.

## Goal

Implement or refine offline reading-state exchange through sidecar files that can be copied to or from the X4 SD card.

## Required source order

Before making changes, read these in this order:

1. `AGENTS.md`
2. `docs/architecture.md`
3. `docs/sidecar-format.md`
4. `docs/milestones.md`
5. `IMPLEMENTATION.md`
6. `PLANS.md`

If any task request conflicts with the local-first architecture, do not silently add sync-server behavior.

## Scope

In scope:
- sidecar schema
- import/export logic
- fingerprint validation
- progress merge
- bookmark merge
- schema versioning
- rejection paths for mismatched books
- helper tooling that reads or writes the sidecar format

Out of scope:
- live sync
- server APIs
- account login
- tokens
- encryption
- conflict resolution for multi-device live sessions
- BLE or Wi-Fi transport unless the task explicitly says to use them as dumb file transport

## Hard rules

- Sidecar files are the v1 exchange mechanism.
- Do not add server endpoints, auth flows, or network assumptions.
- Do not reintroduce device accounts or cloud identity.
- Do not key sidecars by file path.
- Reject mismatched fingerprints rather than guessing.
- Keep merge rules explicit and simple.
- Prefer schema stability over feature breadth.
- Do not claim compatibility with mobile or future tools unless the schema actually supports it.

## Sidecar rules

Preserve these fields unless explicitly changing the schema version:
- `schema_version`
- `book_fingerprint`
- `source`
- `updated_at`
- `progress`
- optional `bookmarks`

Progress should merge by explicit freshness rule, not by guessed position semantics.

## Merge rules

For v1:
- newer `updated_at` wins for progress
- bookmarks merge by id if present
- mismatched fingerprints are rejected
- invalid schema version is rejected or migrated explicitly

If changing merge behavior, document:
- old behavior
- new behavior
- migration or fallback behavior

## File ownership guidance

Use these boundaries:

- `core/model/sidecar.rs`
  - schema types
  - validation rules

- `core/transfer/import.rs`
  - parse and validate incoming sidecar
  - merge into local state

- `core/transfer/export.rs`
  - build sidecar from local state

- `tools/sidecar-tool/`
  - desktop helper behavior

Do not scatter sidecar parsing across unrelated reader or board files.

## Expected output style

When you finish:
- show the exact schema or code path changed
- state whether the schema version changed
- state how merge behavior works now
- state at least one valid import case and one rejection case

## Definition of done

A sidecar-state change is only "done" when:
- it preserves the local-first architecture
- it does not add live sync dependencies
- it uses stable book fingerprinting
- it has explicit validation and merge behavior