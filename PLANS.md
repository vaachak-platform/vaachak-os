# VaachakOS — Execution Plan

This document is the practical plan for implementing VaachakOS v1.

It is narrower than the architecture document and more concrete than the milestones document.

Use this when deciding what to build next.

---

## 1. Plan Summary

The plan is to deliver VaachakOS in three practical waves:

### Wave A — Make X4 boring

Focus:
- board bring-up
- display correctness
- input correctness
- SD correctness
- sleep/wake correctness

### Wave B — Make reading real

Focus:
- library
- TXT and EPUB open
- pagination
- progress persistence
- settings
- bookmarks

### Wave C — Make state portable

Focus:
- sidecar import/export
- state merge
- hardening
- selective fidelity upgrades

Everything else is deferred until these waves are complete.

---

## 2. Wave A — Make X4 Boring

### Objective

Remove hardware uncertainty first.

### Tasks

#### A1. Freeze structure
- trim repo to active X4 shape
- remove or clearly defer non-X4 paths
- confirm board/core/driver boundaries

#### A2. Display bring-up
- stable init
- correct geometry
- correct rotation/inversion
- full refresh
- partial refresh
- sleep/wake display handling

#### A3. Input bring-up
- ADC ladder decoding
- debounce
- repeat/hold behavior as needed
- stable mapping to logical buttons

#### A4. SD bring-up
- mount
- list files
- open/read test file
- basic write path for logs/state

#### A5. Battery and sleep
- battery sampling
- sleep request path
- wake path
- pre-sleep state flush hook

### Done when

- boot is reliable
- display is reliable
- buttons are reliable
- SD is reliable
- sleep/wake is reliable

---

## 3. Wave B — Make Reading Real

### Objective

Turn the stable hardware path into a stable local-first reader.

### Tasks

#### B1. Shell and screens
- fixed screen enum
- Home
- Library
- Reader
- Settings
- Debug

#### B2. TXT flow
- list TXT files
- open TXT
- paginate
- page turn
- save/load progress

#### B3. EPUB minimal flow
- identify EPUB
- extract metadata
- minimal text flow
- page turn
- save/load progress

#### B4. Persistence
- current book reference
- last position
- settings persistence
- recents

#### B5. Reader comfort
- font size
- margins
- line spacing
- refresh cadence
- basic bookmarks

### Done when

- books can be copied to SD and read reliably
- last position resumes
- settings persist
- bookmarks work at a basic level

---

## 4. Wave C — Make State Portable

### Objective

Allow state exchange without live sync.

### Tasks

#### C1. Sidecar schema lock
- finalize JSON field names
- finalize fingerprint rules
- finalize merge rules

#### C2. Export
- export progress to canonical path
- export bookmarks
- deterministic file shape

#### C3. Import
- parse file
- verify fingerprint
- verify schema version
- merge newer state
- reject mismatches cleanly

#### C4. Hardening
- corrupt sidecar handling
- corrupt settings recovery
- corrupt cache recovery
- long-session testing

### Done when

- sidecar files can move reading state by copy alone
- failures do not corrupt local reader state

---

## 5. Exact Priorities

When there is a choice, use this priority order:

1. fix display/input/storage/sleep bugs
2. fix reader correctness bugs
3. fix persistence bugs
4. simplify structure when it reduces fragility
5. add the smallest missing feature for the current milestone
6. defer everything else

---

## 6. Explicit Deferrals

Do not start these unless a task explicitly reopens them:

- live sync server integration
- crypto/auth flows
- WebDAV
- OPDS
- OTA
- large utility suite
- general app/activity framework
- second board implementation
- advanced typography polish
- highlight system

---

## 7. Task Breakdown for Codex

Codex should generally work in tasks small enough to verify immediately.

### Good task size examples

- “stabilize X4 display init and full refresh path”
- “add stable SD mount and file listing”
- “implement TXT progress persistence”
- “add sidecar export for progress only”

### Bad task size examples

- “implement full VaachakOS architecture”
- “add sync and portable state system”
- “generalize the core for future boards”
- “create all planned modules as stubs”

---

## 8. Plan Review Triggers

Revisit the plan if any of the following become true:

- X4 hardware behavior contradicts an architectural assumption
- screen model becomes the main blocker
- repo structure starts growing many board-specific logic files
- a later milestone becomes necessary to unblock the current one

If that happens, simplify rather than expand.

---

## 9. Acceptance Standard Per Patch

Each patch should answer:

- what current plan item it advances
- what files were touched and why
- what was deliberately not implemented
- what remains risky or unverified

If a patch cannot answer that clearly, it is probably too broad.

---

## 10. End State for v1

The plan succeeds when VaachakOS can do the following on X4:

- boot reliably
- render reliably
- browse books on SD
- open EPUB/TXT
- turn pages reliably
- save and restore position
- persist settings
- import/export state via sidecar files

And the codebase remains structured so that another board can be added later without ripping apart the reader core.
