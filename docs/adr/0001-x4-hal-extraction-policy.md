# ADR 0001 — X4 HAL Extraction Policy

**Status:** Accepted for Bootstrap Phase 2  
**Date:** 2026-04-29

---

## Context

VaachakOS was created after the X4 proving-ground reached a useful reader/runtime baseline. The proving-ground has real hardware truth, while VaachakOS is the architecture-first repository.

Moving all X4 code at once would risk losing the stability already earned in the proving-ground.

---

## Decision

Real X4 HAL extraction will be incremental and seam-driven.

The order is:

1. power
2. input
3. storage contract
4. display contract
5. display implementation
6. target boot/runtime shell
7. app/runtime parity

The project will not move Reader/Home/Files app code during the HAL extraction planning phase.

---

## Consequences

### Positive

- VaachakOS gains clean architecture without breaking the X4 baseline.
- Hardware seams become testable.
- Each extracted piece has clear ownership.
- The proving-ground remains useful for risky hardware validation.

### Negative

- VaachakOS will not immediately run the full reader UI.
- Some duplicate concepts may exist temporarily between the two repos.
- More discipline is required to avoid premature feature work.

---

## Guardrails

- No Reader policy in HAL.
- No board GPIO constants in core.
- No full framebuffer assumption.
- No sync/highlights/network work during HAL extraction.
- Real driver movement requires source map + validation matrix + tests.
