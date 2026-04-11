---
name: x4-bringup
description: Use this skill for Xteink X4 board bring-up, display correctness, input wiring, SD mount, battery readout, sleep/wake behavior, and low-level runtime stabilization. Trigger when the task mentions X4 boot, display init, inversion, rotation, refresh, buttons, SD, battery, sleep, wake, panic logs, or board-level debugging. Do not use this skill for EPUB layout, cache schema, sidecar import/export, or future WS-397 support.
---

# X4 Bring-Up Skill

This skill exists to keep X4 work narrow, practical, and non-hallucinatory.

## Goal

Deliver stable X4 board behavior without widening scope into generic frameworks, second-board abstractions, or product features that are not required for bring-up.

## Required source order

Before making changes, read these in this order:

1. `AGENTS.md`
2. `docs/architecture.md`
3. `docs/milestones.md`
4. `IMPLEMENTATION.md`
5. `PLANS.md`

If these sources disagree, follow them in that order and do not invent a hybrid design.

## Scope

In scope:
- X4 boot path
- board initialization
- display init
- geometry correctness
- inversion/polarity fixes
- refresh mode correctness
- button input
- SD mount
- battery reading
- sleep/wake behavior
- crash logging
- minimal board diagnostics

Out of scope:
- live sync
- sidecar schema changes
- EPUB parsing changes unless needed only to unblock bring-up
- generic activity frameworks
- second-board support
- refactors that create new architecture without an explicit task

## Hard rules

- Prefer fixing existing files over creating new ones.
- Do not create multiple new X4-only files unless the task explicitly requires it.
- Keep board-specific code in `boards/x4/` unless it is clearly reusable hardware behavior for `drivers/`.
- Do not move reader logic into board code.
- Do not add WS-397 code, placeholders, stubs, or conditional branches unless the task explicitly asks for them.
- Do not add network, sync, BLE, WebDAV, OPDS, or OTA code in bring-up tasks.
- Do not introduce a full framebuffer.
- Do not replace strip rendering with a new rendering model.
- Do not claim a fix is complete without naming the exact low-level behavior changed.

## File ownership guidance

Use these boundaries:

- `boards/x4/`
  - pin map
  - bus wiring
  - wake/sleep wiring
  - board profile
  - optional FFI bridge

- `drivers/`
  - SSD1677 behavior
  - ADC key scanning
  - battery sensing
  - refresh policy helpers
  - SD card helpers

- `core/`
  - never for X4-specific pin or board wiring

If unsure whether code belongs in `boards/x4/` or `drivers/`, ask:
"Would this still make sense on another board using the same chip or behavior?"
- If yes, put it in `drivers/`.
- If no, keep it in `boards/x4/`.

## Bring-up checklist

For any X4 bring-up task, check these in order:

1. Boot reaches the intended screen or test path
2. Display init sequence matches the intended panel behavior
3. Width, height, origin, and orientation are consistent
4. Black/white polarity is correct
5. Full refresh works
6. Partial refresh works or is safely disabled
7. Button mapping is correct
8. SD access works without breaking display access
9. Sleep/wake returns to a safe state
10. Crash path logs enough context for the next iteration

Do not jump ahead to higher-level UX polish until these are stable.

## Expected output style

When you finish:
- state what changed
- state which file boundaries were preserved
- state any unresolved hardware uncertainty
- state the exact validation commands or manual test steps

## Definition of done

A bring-up change is only "done" when:
- it stays within X4 scope
- it does not widen architecture
- it improves or preserves board stability
- it does not create unnecessary X4-only file sprawl
- it leaves the next debugging step obvious