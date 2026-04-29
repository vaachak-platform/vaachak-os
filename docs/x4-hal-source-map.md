# X4 HAL Source Map

**Purpose:** Map proven `x4-reader-os-rs` hardware/runtime facts to future VaachakOS crates.

This is a source map, not a copy plan. Code should be moved only after the target trait and validation gate are ready.

---

## 1. Source repository roles

| Repository | Role |
|---|---|
| `x4-reader-os-rs` | Hardware proving-ground and runtime truth source |
| `vaachak-os` | Architecture-first OS repository |

---

## 2. Future crate ownership summary

| Future crate | Owns | Does not own |
|---|---|---|
| `core` | HAL traits, neutral models, reader state records | X4 GPIO/SPI/SSD1677/FAT implementation |
| `hal-xteink-x4` | X4 display/input/power/storage implementation | Reader/Home/Files app policy |
| `target-xteink-x4` | boot/executor/allocator/task wiring | hardware driver internals and app logic |

---

## 3. Display source map

### Candidate source areas in `x4-reader-os-rs`

```text
kernel/src/drivers/strip.rs
kernel/src/board/** display-related setup
SSD1677 driver module currently used by runtime
boot console display path
```

### Target destination

```text
core/src/hal/display.rs              # traits and neutral models
hal-xteink-x4/src/display.rs         # X4 SSD1677 implementation
target-xteink-x4/src/main.rs         # construction/wiring only
```

### Facts to preserve

- native panel is 800x480
- logical reader runtime is portrait-oriented
- no full framebuffer assumption
- strip rendering remains baseline
- refresh mode distinction must survive
- display and SD share SPI path
- SD probe/runtime speed sequence must not regress

### Do not port yet

- reader UI code
- app-specific draw order
- display workaround experiments without explanation

---

## 4. Input source map

### Candidate source areas in `x4-reader-os-rs`

```text
kernel/src/drivers/input/**
board/input setup code
button threshold diagnostics
power button interrupt setup
```

### Target destination

```text
core/src/hal/input.rs
hal-xteink-x4/src/input.rs
```

### Facts to preserve

- GPIO3 power button behavior
- ADC ladder row separation
- bottom-left cluster threshold mapping
- long-press and short-press remain runtime policy, not raw HAL policy, unless explicitly modeled as neutral event kinds
- power button precedence over ADC ladder events

### Do not port yet

- Reader action labels
- app menu shortcuts
- bookmark/theme action mapping

---

## 5. Power source map

### Candidate source areas in `x4-reader-os-rs`

```text
kernel/src/drivers/battery.rs
board power setup
sleep/wake notes
```

### Target destination

```text
core/src/hal/power.rs
hal-xteink-x4/src/power.rs
```

### Facts to preserve

- battery divider conversion
- percent estimation curve
- charging detect behavior if present
- future sleep hooks are target/HAL responsibilities, not Reader responsibilities

---

## 6. Storage source map

### Candidate source areas in `x4-reader-os-rs`

```text
kernel/src/drivers/sdcard.rs
kernel/src/drivers/storage.rs
reader_state.rs path behavior
state/*.BKM, BMIDX.TXT behavior
EPUB cache hit/load behavior
```

### Target destination

```text
core/src/hal/storage.rs
core/src/models/storage_layout.rs
hal-xteink-x4/src/storage.rs
```

### Facts to preserve

- SD card is on shared SPI
- probe speed and runtime speed both matter
- flat 8.3-compatible state path is currently proven
- nested path writes must be a capability, not assumed
- EPUB cache behavior should not be broken during HAL extraction

---

## 7. Target runtime source map

### Candidate source areas in `x4-reader-os-rs`

```text
src/bin/main.rs
src/lib.rs
Board::init path
executor setup
heap/static allocations
BootConsole
AppManager construction
```

### Target destination

```text
target-xteink-x4/src/main.rs
```

### Facts to preserve

- boot sequence order matters
- SD/display shared SPI order matters
- boot console is valuable for failure diagnosis
- target should construct HAL and hand off to core runtime

---

## 8. Leave behind in proving-ground initially

```text
src/apps/home.rs
src/apps/files.rs
src/apps/reader/**
EPUB parser/cache internals
UI experiments
reader footer/menu behavior
bookmark UX experiments
raw diagnostics
```

These can migrate later after HAL extraction and target boot are stable.
