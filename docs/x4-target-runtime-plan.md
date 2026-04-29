# X4 Target Runtime Plan

**Purpose:** Define how `target-xteink-x4` should eventually regain a real X4 boot path without dragging app policy into the target crate.

---

## 1. Target crate responsibility

`target-xteink-x4` owns:

- boot entrypoint
- allocator setup
- executor startup
- concrete X4 HAL construction
- boot console wiring
- task spawn wiring
- handoff into VaachakOS core runtime

It does not own:

- Reader UX
- Home/Files navigation
- book persistence record formats
- EPUB parsing
- bookmark/theme policy

---

## 2. Target boot stages

### Stage 0 — host placeholder

Current state:

- compiles on host
- no real hardware runtime
- useful for workspace health

### Stage 1 — embedded compile proof

Goal:

- build target crate for X4 embedded target
- no real driver integration required

### Stage 2 — boot marker proof

Goal:

- boot on device
- print serial marker
- optionally draw minimal display marker

### Stage 3 — HAL construction proof

Goal:

- construct X4 HAL components
- initialize storage/display safely
- emit capability summary

### Stage 4 — minimal runtime handoff

Goal:

- hand off to core runtime shell
- process neutral input events
- render minimal Home shell

### Stage 5 — parity rebuild

Goal:

- regain current proving-ground baseline:
  - Home
  - Library/Files
  - Reader
  - Continue
  - TXT
  - EPUB
  - bookmarks
  - reader theme/progress state

---

## 3. Boot order guardrail

The future target boot path must preserve lessons from the proving-ground:

1. initialize board/peripherals safely
2. establish serial logging early
3. initialize SPI conservatively
4. initialize SD card before relying on app state
5. speed up SPI only after probe/mount path is safe
6. initialize display without full framebuffer
7. construct core runtime only after HAL capabilities are known

---

## 4. Minimal target capability report

The first real target proof should log something like:

```text
vaachak-os target-xteink-x4 boot
hal.display.native=800x480
hal.display.logical=480x800
hal.display.strip=true
hal.storage.sd=mounted
hal.storage.nested_paths=false|true
hal.input.buttons=x4-adc-ladder+gpio3
hal.power.battery_mv=...
```

---

## 5. Do not do in target crate

Do not place these in `target-xteink-x4`:

- reader page calculations
- EPUB chapter caches
- bookmark serialization
- theme preset serialization
- UI row rendering
- app menu labels
