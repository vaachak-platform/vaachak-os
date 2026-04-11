---
name: board-boundaries
description: Use this skill when reorganizing files, introducing new modules, refactoring board code, moving logic between core/drivers/boards, or reviewing whether a proposed change creates unnecessary X4-only sprawl. Trigger when the task mentions refactor, file layout, module ownership, HAL-like boundaries, moving code, reusable drivers, board-specific code, or keeping the codebase future-friendly without over-abstracting. Do not use this skill for pure bugfixes that stay within an already-correct file boundary.
---

# Board Boundaries Skill

This skill prevents architecture drift and unnecessary board-specific file growth.

## Goal

Keep the codebase refactor-friendly by enforcing honest boundaries between:
- shared core logic
- reusable drivers
- board wiring
- boot targets

## Required source order

Before making changes, read these in this order:

1. `AGENTS.md`
2. `docs/architecture.md`
3. `docs/milestones.md`
4. `IMPLEMENTATION.md`
5. `PLANS.md`

If the requested refactor would widen scope beyond the active milestone, do not perform it unless the task explicitly requires it.

## Boundary test

For every proposed new file or moved function, ask:

1. Is this reader/product logic?
   - put it in `core/`

2. Is this reusable hardware behavior or protocol logic?
   - put it in `drivers/`

3. Is this board wiring, pin mapping, wake configuration, or board profile?
   - put it in `boards/x4/`

4. Is this only bootstrapping and runtime entry?
   - put it in `targets/x4/`

If none of those are clearly true, do not create a new file yet.

## Hard rules

- Default to fewer files, not more.
- Prefer improving boundaries in existing files over adding new X4-only files.
- Do not create abstraction layers "for future boards" unless the current task proves the need.
- Do not add WS-397-driven abstractions to active X4 work unless the task explicitly requires it.
- Do not create generic traits or interfaces just because they look cleaner.
- Do not move logic out of `core/` into board code to make a bug disappear.
- Do not move board-specific wiring into `core/` to reduce file count.

## Allowed reasons to create a new file

Create a new file only when at least one is true:
- the file groups a clearly separate reusable hardware behavior
- the file isolates board wiring that would otherwise pollute shared code
- the file significantly reduces confusion in an active milestone
- the file is explicitly required by the architecture documents

If the reason is only "future flexibility" or "cleaner abstraction," do not do it yet.

## Anti-patterns to avoid

Avoid these:

- many tiny `x4_*` files with one narrow helper each
- speculative `ws397` compatibility branches in shared code
- large HAL trees that become dumping grounds
- board modules that own reader behavior
- refactors that rename many files without reducing complexity
- introducing `dyn` dispatch or heap-based abstraction when explicit state would do

## Review checklist for refactors

Before finalizing a refactor, verify:
- Did file count grow? Why?
- Did any logic move closer to the wrong layer?
- Did this help the active milestone, or only future theory?
- Could the same improvement have been made with fewer files?
- Is the X4 path now clearer, not just more abstract?

## Expected output style

When you finish:
- list any files created, moved, or deleted
- justify each new file in one sentence
- explain how the boundary is now cleaner
- call out any deferred cleanup rather than hiding it

## Definition of done

A boundary/refactor change is only "done" when:
- it reduces confusion for the active milestone
- it does not widen scope
- it avoids unnecessary X4-only sprawl
- it preserves the board-thin, core-generic, driver-reusable structure