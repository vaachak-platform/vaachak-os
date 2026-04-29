# X4 HAL Porting Backlog

**Purpose:** Convert the extraction plan into concrete implementation tasks.

---

## Epic 1 — Power HAL extraction

### Task 1.1 — finalize `PowerHal` contract

Acceptance:

- exposes battery millivolts
- exposes battery percentage
- exposes charge state
- sleep hooks remain optional or no-op until proven

### Task 1.2 — port X4 battery conversion facts

Acceptance:

- divider math test passes
- percent interpolation test passes
- no app/runtime dependency

---

## Epic 2 — Input HAL extraction

### Task 2.1 — finalize neutral input event model

Acceptance:

- physical buttons map to neutral button IDs
- event kind supports press/release/short/long only if runtime contract requires it
- no Reader action names in HAL

### Task 2.2 — port ADC ladder thresholds

Acceptance:

- bottom-left cluster tests pass
- vertical buttons tests pass
- power button precedence test passes

### Task 2.3 — add input trace adapter

Acceptance:

- optional test utility can log raw ADC and decoded events
- trace utility does not enter core app logic

---

## Epic 3 — Storage HAL extraction

### Task 3.1 — refine `StorageHal` for app state use

Acceptance:

- read file
- write file
- exists
- mkdir or capability-gated mkdir
- list dir sink or iterator abstraction

### Task 3.2 — preserve X4 flat state layout

Acceptance:

- `state/<BOOKID>.PRG`
- `state/<BOOKID>.BKM`
- `state/<BOOKID>.THM`
- `state/<BOOKID>.MTA`
- `state/BMIDX.TXT`

### Task 3.3 — design SD/FAT adapter boundary

Acceptance:

- no real adapter copied until trait is stable
- errors are typed
- nested path support is tested before use

---

## Epic 4 — Display HAL extraction

### Task 4.1 — finalize display contract

Acceptance:

- begin frame
- draw strip
- end frame
- sleep
- refresh mode enum
- geometry model

### Task 4.2 — represent X4 display facts

Acceptance:

- native size 800x480
- logical portrait mapping
- SPI bus speed facts
- no framebuffer requirement

### Task 4.3 — port SSD1677 driver behind `DisplayHal`

Acceptance:

- minimal boot/status screen renders on X4
- no Reader renderer migration in this task
- serial logs and photo captured

---

## Epic 5 — Target X4 runtime shell

### Task 5.1 — document target boot sequence

Acceptance:

- heap setup
- executor startup
- HAL construction
- boot console
- task handoff

### Task 5.2 — create minimal target firmware proof

Acceptance:

- builds for embedded target
- boots on X4
- emits serial boot marker
- optionally draws boot marker

---

## Explicit deferrals

Do not schedule yet:

- Reader migration
- Home migration
- Files migration
- EPUB parser migration
- Sync
- Highlights
- WebDAV/OPDS
- Waveshare implementation
- desktop simulator
