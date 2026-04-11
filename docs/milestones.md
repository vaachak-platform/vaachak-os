# VaachakOS — Milestones

**Status:** Working plan aligned to the revised X4-first architecture  
**Primary target:** Xteink X4  
**Deferred targets:** Waveshare ESP32-S3 3.97" e-Paper, desktop simulator, live sync

---

## 1. Purpose

This document turns the revised architecture into a delivery sequence that is realistic for the X4.

The milestones are intentionally narrow. They exist to keep the project out of the pattern where many board-specific files and speculative abstractions appear before the basic reader path is stable.

The milestones are ordered by one rule:

**hardware stability first, reader usability second, optional platform features later.**

---

## 2. Delivery Principles

1. **Only one active board in implementation:** Xteink X4
2. **Only one active product path:** local-first e-reader
3. **No live sync in active milestones**
4. **No feature starts before the previous milestone exit criteria are met**
5. **No broad utility/app work before the reader is stable**
6. **No new X4-only files unless they are truly board wiring files**
7. **Prefer deletion and simplification over adding abstractions**

---

## 3. Milestone Overview

| Milestone | Name | Goal |
|---|---|---|
| M0 | Architecture Reset | Freeze structure and guardrails |
| M1 | X4 Foundation | Stabilize hardware bring-up |
| M2 | Reader Skeleton | Open books and resume progress |
| M3 | Reader MVP | Reach daily-usable reading |
| M4 | Stability Pass | Harden storage, sleep, and recovery |
| M5 | Offline Exchange | Import/export reading state via sidecar files |
| M6 | Quality Upgrade | Improve fidelity without destabilizing core |
| M7 | Second Board Extraction | Validate that the architecture generalizes |

---

## 4. M0 — Architecture Reset

### Goal

Freeze a repository shape and implementation approach that is realistic for X4 and resistant to over-abstraction.

### Deliverables

- Finalize repo layout around:
  - `core/`
  - `drivers/`
  - `boards/x4/`
  - `targets/x4/`
- Defer all non-X4 boards from active implementation
- Define `Screen` enum shell model
- Define offline sidecar format draft
- Remove live sync/server assumptions from the active path
- Document board/driver/core ownership clearly

### Exit Criteria

- X4 is the only production build target
- No live sync code is required for boot or reader flow
- No dynamic app-framework requirement blocks implementation
- The file layout is small enough that a new contributor can explain it in one page

### Anti-Goals

- No WebDAV
- No OPDS
- No OTA
- No auth/crypto stack
- No WS-397 active work
- No generalized plugin/app runtime

---

## 5. M1 — X4 Foundation

### Goal

Prove reliable hardware bring-up on the X4.

### Deliverables

- Display init path is stable
- Geometry is correct
- Rotation/inversion/polarity issues are resolved
- Full refresh works
- Partial refresh works well enough for shell screens
- Buttons are readable and debounced
- SD card mounts reliably
- Battery sampling works
- Sleep and wake path is stable
- Minimal crash log path exists

### Required Validation

- Cold boot repeatedly succeeds
- Shell redraw is stable over repeated navigation
- Buttons do not trigger ghost presses under normal use
- Sleep/wake does not corrupt the display or state
- SD card mount succeeds across repeated reboots

### Exit Criteria

- Device boots into a basic shell every time
- Display updates correctly with no persistent inversion/clipping issues
- Input is predictable
- Sleep/wake is usable enough for daily testing

### Anti-Goals

- No EPUB parsing yet if hardware still behaves inconsistently
- No settings UI beyond what is needed for bring-up
- No network work

---

## 6. M2 — Reader Skeleton

### Goal

Open books from SD and resume them after reboot.

### Deliverables

- Library screen can browse supported book files
- TXT open path works
- EPUB open path works in a constrained subset
- Minimal pagination is implemented
- Page turn works
- Progress save/load works
- Cache directory creation works
- Last-opened-book resume works

### Required Validation

- Open multiple TXT files from SD
- Open multiple EPUB files from SD
- Turn pages forward and backward
- Reboot and resume last position
- Sleep and resume inside a book

### Exit Criteria

- A user can copy a book to SD, open it, read it, and come back later to the same place
- Reader flow is stable enough to use without debug-only expectations

### Anti-Goals

- No advanced CSS fidelity work
- No highlights
- No sidecar import/export yet
- No cover generation unless it is nearly free

---

## 7. M3 — Reader MVP

### Goal

Turn the skeleton into a daily-usable reader.

### Deliverables

- Home screen with recents
- Reader settings:
  - font size
  - margins
  - line spacing
  - refresh cadence
- Settings screen for core device behavior
- Basic bookmarks
- Basic TOC if stable enough
- Footer labels and navigation polish
- Debug screen moved off the main path

### Required Validation

- Read across multiple sessions without corruption
- Change settings and keep them across reboot
- Add and remove bookmarks
- Open recent books from Home

### Exit Criteria

- Reading is comfortable enough for daily use
- The system feels like an e-reader, not a bring-up demo

### Anti-Goals

- No giant utilities bundle
- No sync semantics
- No advanced typography optimization yet

---

## 8. M4 — Stability Pass

### Goal

Harden the system against realistic failures and long-running use.

### Deliverables

- Versioned settings envelope
- Cache invalidation rules
- Corrupt-book handling
- Better crash logs
- Long-session soak testing
- Sleep/wake soak testing
- Reader recovery when cache is missing or invalid
- Graceful fallback to defaults on bad prefs

### Required Validation

- Bad settings payload does not brick the shell
- Corrupt cache can be rebuilt
- Corrupt EPUB fails with a user-visible error rather than panic
- Repeated sleep/wake cycles preserve state
- Multi-day testing does not show systematic state drift

### Exit Criteria

- Common failure modes recover predictably
- Tester confidence is high enough to start sidecar exchange and fidelity upgrades

### Anti-Goals

- No expansion of feature surface during stability milestone

---

## 9. M5 — Offline Exchange

### Goal

Move reading state between systems without live sync fragility.

### Deliverables

- Sidecar file schema v1 finalized
- Import sidecar from SD
- Export sidecar to SD
- Merge newer progress
- Merge bookmarks by id
- Reject mismatched book fingerprints
- Provide a small helper tool or documented workflow for external generation

### Required Validation

- Export X4 state to sidecar file
- Re-import same file safely
- Import newer state over older local state
- Reject unrelated sidecar for wrong book

### Exit Criteria

- Reading state can move by file copy alone
- No network dependency exists in the exchange path

### Anti-Goals

- No server sync
- No auth tokens
- No encryption requirement
- No Wi-Fi dependency

---

## 10. M6 — Quality Upgrade

### Goal

Improve reading fidelity without destabilizing the now-working core.

### Candidate Deliverables

- Better EPUB CSS subset support
- Chapter pre-layout cache
- Better typography
- Library covers or richer metadata presentation
- Footnotes
- Better chapter navigation

### Required Validation

- Improvements are measurable in reading quality
- Memory use stays within X4-safe bounds
- No regression in shell or reader reliability

### Exit Criteria

- Quality gains are visible and the core remains stable

### Anti-Goals

- No broad architecture rewrite during fidelity work

---

## 11. M7 — Second Board Extraction

### Goal

Validate that the architecture really generalizes and that the earlier separation work was honest.

### Deliverables

- Real `boards/ws397/` implementation
- Reuse `core/` directly where possible
- Reuse `drivers/` where possible
- Add board-specific wiring/profile only where needed
- Document differences in input/display/power assumptions

### Required Validation

- Second board can boot with limited changes outside board/wiring areas
- Core reader logic remains shared
- Board-specific work stays localized

### Exit Criteria

- The architecture supports another board without major churn in the reader core

### Anti-Goals

- No board-generalization effort before X4 milestones are complete

---

## 12. Recommended PR Sequence

This sequence is designed for Codex or a human contributor to follow without drifting.

1. Repo trim / structure freeze
2. X4 board bring-up stabilization
3. Shell boot + input + redraw
4. SD mount + file browsing
5. TXT reader path
6. EPUB minimal path
7. Progress persistence
8. Settings persistence
9. Home + recents + bookmarks
10. Stability hardening
11. Sidecar import/export
12. Quality upgrades
13. Second board extraction

---

## 13. Stop Conditions

Work should stop and be reviewed before proceeding if any of the following happen:

- more than two new X4-only files are added for logic that should live in `core/` or `drivers/`
- a milestone starts depending on a deferred feature
- a proposed refactor adds generic abstractions without removing existing complexity
- display stability regresses while unrelated features are being added
- network/sync code starts appearing before M5 explicitly authorizes it

---

## 14. Definition of Done for v1 Reader

VaachakOS v1 is successful when all of the following are true:

- it boots reliably on X4
- it displays correctly on X4
- it opens EPUB/TXT from SD card
- it turns pages reliably
- it resumes reading position after reboot or sleep
- it stores settings safely
- it can import/export reading state via sidecar files
- the codebase remains clean enough to support a later WS-397 extraction

That is the minimum outcome the project should optimize for.
