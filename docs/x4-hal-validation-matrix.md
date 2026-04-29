# X4 HAL Validation Matrix

**Purpose:** Define what must be proven before each hardware seam is considered extracted.

---

## 1. Validation levels

| Level | Meaning |
|---|---|
| L0 | Host unit tests only |
| L1 | Compiles for embedded target |
| L2 | Minimal firmware boots on X4 |
| L3 | Hardware proof logs captured |
| L4 | Integrated with minimal VaachakOS runtime |
| L5 | Feature parity with proving-ground behavior |

Phase 2 aims only for L0 planning readiness. Later extraction branches move seams toward L3/L4.

---

## 2. Display validation

| Check | Required before driver migration? | Evidence |
|---|---:|---|
| Native geometry is 800x480 | Yes | unit test |
| Logical portrait geometry is stable | Yes | unit test + photo |
| Strip rendering interface does not require framebuffer | Yes | code review |
| Refresh modes are represented | Yes | unit test |
| Shared SPI speed sequence documented | Yes | boot log |
| Minimal boot/status screen renders | Later | on-device photo/log |
| Reader rendering parity | Later | manual test pass |

---

## 3. Input validation

| Check | Required before migration? | Evidence |
|---|---:|---|
| Row 1 thresholds captured | Yes | unit test |
| Row 2 thresholds captured | Yes | unit test |
| Bottom-left cluster mapping preserved | Yes | unit test |
| GPIO3 power event precedence | Yes | unit test |
| Short/long event policy defined | Yes | contract doc |
| Reader footer labels not inside HAL | Yes | code review |
| On-device event stream matches proving-ground | Later | serial log |

---

## 4. Power validation

| Check | Required before migration? | Evidence |
|---|---:|---|
| Battery divider math captured | Yes | unit test |
| Percentage curve captured | Yes | unit test |
| Charging detect modeled if available | Before L4 | serial log |
| Sleep API is trait-only first | Yes | code review |
| Real light/deep sleep proof | Later | device test |

---

## 5. Storage validation

| Check | Required before migration? | Evidence |
|---|---:|---|
| Storage lifecycle states modeled | Yes | unit test |
| SD probe/mount contract defined | Yes | trait review |
| 8.3 flat state layout preserved | Yes | unit test |
| Nested paths capability flag exists | Before real adapter | code review |
| Real SD read/write adapter works | Later | serial log |
| EPUB cache read path preserved | Later | real EPUB open test |
| No raw app policy in storage HAL | Yes | code review |

---

## 6. Target runtime validation

| Check | Required before target migration? | Evidence |
|---|---:|---|
| allocator/heap strategy documented | Yes | target plan |
| executor startup sequence documented | Yes | target plan |
| boot console path documented | Yes | target plan |
| HAL construction ownership documented | Yes | source map |
| minimal target builds | Later | cargo build |
| minimal target boots | Later | serial log |
| app runtime handoff works | Later | device test |

---

## 7. Exit criteria for real HAL migration

A HAL seam can move from plan to implementation only when:

- it has an owner file in `core/src/hal`
- it has an owner file in `hal-xteink-x4/src`
- its source map is written
- its host tests exist
- its risk is listed
- no Reader/Home/Files policy is mixed into the HAL
