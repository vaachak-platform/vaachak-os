# VaachakOS Skills Guide

This directory contains focused Codex skills for recurring workflows in the VaachakOS repository.

## Why these skills exist

The repository already uses `AGENTS.md` for repo-wide rules and source-of-truth ordering.  
These skills exist only for **narrow, repeatable workflows** where Codex is likely to drift, over-abstract, or hallucinate architecture if left unguided.

Use a skill when the task clearly matches that workflow.  
Do **not** use a skill just because its name sounds related.

## Skill invocation model

Codex may use these skills in two ways:

1. **Implicitly** — when the task closely matches the skill description
2. **Explicitly** — when the user or operator names the skill directly

Because of that, each skill should stay narrow and should not become a second `AGENTS.md`.

## Source-of-truth order

Before using any skill, follow this order:

1. `AGENTS.md`
2. `docs/architecture.md`
3. `docs/milestones.md`
4. `docs/sidecar-format.md` when sidecar work is involved
5. `IMPLEMENTATION.md`
6. `PLANS.md`

If a skill seems to conflict with these files, the higher-priority document wins.

---

## Available skills

### 1. `x4-bringup`

**Use when:**
- boot path is broken
- X4 display does not initialize correctly
- orientation, inversion, clipping, or polarity is wrong
- full or partial refresh is broken
- buttons are mis-mapped
- SD mount is broken at board level
- battery readout is wrong
- sleep/wake behavior is broken
- panic/crash logging needs board-level work

**Do not use when:**
- the task is about EPUB parsing
- the task is about cache schema
- the task is about sidecar import/export
- the task is about live sync or networking
- the task is about WS-397 support

**Primary goal:**  
Stabilize X4 hardware behavior without widening architecture.

---

### 2. `reader-cache`

**Use when:**
- EPUB or TXT loading is broken
- pagination needs work
- page turning is wrong
- reading progress does not save or resume
- cache layout needs adjustment
- recents or basic TOC work is needed
- reader settings affect rendering behavior
- cache invalidation or rebuild logic needs work

**Do not use when:**
- the problem is display-driver bring-up
- the problem is board wiring
- the task is about sidecar schema or merge rules
- the task is about server sync or auth
- the task is about second-board abstraction

**Primary goal:**  
Keep open/read/page-turn/resume stable on X4 without adding speculative fidelity or network complexity.

---

### 3. `sidecar-state`

**Use when:**
- offline reading-state exchange is being added or fixed
- sidecar schema is being defined or updated
- import/export behavior is being implemented
- progress merge rules are being changed
- bookmark merge rules are being changed
- sidecar validation and rejection behavior is needed
- an external helper tool needs to read or write sidecar files

**Do not use when:**
- the task mentions live sync server work
- the task mentions auth, crypto, tokens, or account login
- the task is about BLE or Wi-Fi except as dumb file transport
- the task is board bring-up
- the task is general reader rendering

**Primary goal:**  
Preserve the local-first v1 architecture and prevent drift back into fragile live sync work.

---

### 4. `board-boundaries`

**Use when:**
- refactoring file layout
- moving code between `core/`, `drivers/`, `boards/`, and `targets/`
- deciding whether a new file should exist
- reducing X4-only file sprawl
- checking whether a helper belongs in `drivers/` or `boards/x4/`
- reviewing whether a proposed abstraction is premature
- cleaning up architecture without widening scope

**Do not use when:**
- the task is a straightforward bugfix inside an already-correct file boundary
- the task is purely about EPUB behavior
- the task is purely about sidecar schema
- the task is a direct board bring-up bug with no refactor involved

**Primary goal:**  
Keep the codebase refactor-friendly by enforcing honest boundaries and avoiding unnecessary board-specific growth.

---

## Fast routing guide

Use this quick map:

- **Display wrong, buttons wrong, sleep broken, SD mount broken at board level**  
  -> `x4-bringup`

- **Book opens incorrectly, page turns wrong, progress not resuming, cache needs work**  
  -> `reader-cache`

- **Manual state exchange, import/export, merge rules, sidecar validation**  
  -> `sidecar-state`

- **Should this code live in `core`, `drivers`, or `boards/x4`? Should this file exist?**  
  -> `board-boundaries`

---

## Rules for combining skills

Only combine skills when the task genuinely crosses boundaries.

Common valid combinations:

- `x4-bringup` + `board-boundaries`  
  when board fixes require careful module cleanup

- `reader-cache` + `board-boundaries`  
  when reader logic has leaked into board files or vice versa

- `reader-cache` + `sidecar-state`  
  when progress/bookmark persistence interacts with import/export

Avoid combining skills just to be safe.  
Prefer the narrowest correct skill.

---

## What not to do

Do not:
- treat these skills as architecture permission slips
- widen scope beyond the current milestone
- add WS-397-driven abstractions during active X4 work
- reintroduce live sync because a task mentions “sync”
- create many new X4-only files without applying `board-boundaries`
- duplicate `AGENTS.md` inside the skills

---

## Practical operator guidance

When prompting Codex, it helps to name the skill explicitly if the task is high-risk.

Examples:

- “Use `x4-bringup` and fix the X4 inversion and partial refresh behavior.”
- “Use `reader-cache` and repair EPUB progress resume after reboot.”
- “Use `sidecar-state` and implement sidecar import validation for mismatched fingerprints.”
- “Use `board-boundaries` and review whether the new helper belongs in `drivers/` or `boards/x4/`.”

If the task is ambiguous, start with `board-boundaries` only when the main risk is architectural drift.  
Otherwise, prefer the workflow skill closest to the real task.

---

## Directory layout

```text
skills/
  README.md
  x4-bringup/
    SKILL.md
  reader-cache/
    SKILL.md
  sidecar-state/
    SKILL.md
  board-boundaries/
    SKILL.md
```

This file is only a routing guide.  
The actual behavior and constraints for each workflow live in that skill’s `SKILL.md`.