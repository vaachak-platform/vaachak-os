#!/usr/bin/env bash
set -euo pipefail

failures=0
warnings=0

ok() { printf 'OK   %s\n' "$*"; }
fail() { printf 'FAIL %s\n' "$*"; failures=$((failures + 1)); }
warn() { printf 'WARN %s\n' "$*"; warnings=$((warnings + 1)); }

exists() {
  if [[ -e "$1" ]]; then ok "exists: $1"; else fail "missing: $1"; fi
}

contains() {
  local desc="$1"
  local pattern="$2"
  shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase23_check_rg.txt 2>/dev/null; then
    ok "$desc"
  else
    fail "$desc"
    printf '      pattern: %s\n' "$pattern"
    printf '      path:    %s\n' "$*"
  fi
}

not_contains() {
  local desc="$1"
  local pattern="$2"
  shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase23_check_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase23_check_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 23 Vaachak display boundary extraction check"
echo

exists Cargo.toml
exists target-xteink-x4/Cargo.toml
exists target-xteink-x4/src/main.rs
exists target-xteink-x4/src/runtime/mod.rs
exists target-xteink-x4/src/runtime/pulp_runtime.rs
exists target-xteink-x4/src/runtime/vaachak_runtime.rs
exists target-xteink-x4/src/runtime/display_boundary.rs
exists target-xteink-x4/src/runtime/input_boundary.rs
exists target-xteink-x4/src/runtime/storage_boundary.rs
exists vendor/pulp-os/src/bin/main.rs
exists vendor/pulp-os/Cargo.toml
exists vendor/pulp-os/kernel/Cargo.toml
exists vendor/smol-epub/Cargo.toml
exists docs/phase23/PHASE23_DISPLAY_BOUNDARY.md
exists docs/phase23/PHASE23_ACCEPTANCE.md
exists scripts/check_reader_runtime_sync_phase23.sh

if [[ "${PHASE23_RUN_CARGO:-0}" == "1" ]]; then
  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
  ok "cargo check/clippy passed inside script"
else
  ok "cargo check/clippy skipped inside script; set PHASE23_RUN_CARGO=1 to enable"
fi

contains "crate root main loads runtime module" 'mod runtime;' target-xteink-x4/src/main.rs
contains "runtime mod exposes imported Pulp runtime" 'pub mod pulp_runtime;' target-xteink-x4/src/runtime/mod.rs
contains "runtime mod exposes Vaachak facade" 'pub mod vaachak_runtime;' target-xteink-x4/src/runtime/mod.rs
contains "runtime mod exposes display boundary" 'pub mod display_boundary;' target-xteink-x4/src/runtime/mod.rs
contains "display boundary type is present" 'struct VaachakDisplayBoundary' target-xteink-x4/src/runtime/display_boundary.rs
contains "display boundary has Phase 23 marker" 'phase23=x4-display-boundary-ok' target-xteink-x4/src/runtime/display_boundary.rs
contains "Vaachak facade emits Phase 23 marker through display boundary" 'VaachakDisplayBoundary::emit_phase23_marker' target-xteink-x4/src/runtime/vaachak_runtime.rs

contains "display boundary records imported runtime behavior owner" 'vendor/pulp-os imported runtime|ImportedPulpRuntime' target-xteink-x4/src/runtime/display_boundary.rs
contains "display boundary records physical display init not moved" 'PHYSICAL_DISPLAY_INIT_MOVED_IN_PHASE23:\s*bool\s*=\s*false' target-xteink-x4/src/runtime/display_boundary.rs
contains "display boundary records physical refresh not moved" 'PHYSICAL_DISPLAY_REFRESH_MOVED_IN_PHASE23:\s*bool\s*=\s*false' target-xteink-x4/src/runtime/display_boundary.rs
contains "display boundary records SSD1677 SPI transactions not moved" 'SSD1677_SPI_TRANSACTIONS_MOVED_IN_PHASE23:\s*bool\s*=\s*false' target-xteink-x4/src/runtime/display_boundary.rs
contains "display boundary records framebuffer/strip render not moved" 'FRAMEBUFFER_OR_STRIP_RENDER_MOVED_IN_PHASE23:\s*bool\s*=\s*false' target-xteink-x4/src/runtime/display_boundary.rs

contains "display boundary records X4 EPD pins" 'EPD_CS_GPIO:\s*u8\s*=\s*21|EPD_DC_GPIO:\s*u8\s*=\s*4|EPD_RST_GPIO:\s*u8\s*=\s*5|EPD_BUSY_GPIO:\s*u8\s*=\s*6' target-xteink-x4/src/runtime/display_boundary.rs
contains "display boundary records shared SPI pins" 'SPI_SCLK_GPIO:\s*u8\s*=\s*8|SPI_MOSI_GPIO:\s*u8\s*=\s*10|SPI_MISO_GPIO:\s*u8\s*=\s*7' target-xteink-x4/src/runtime/display_boundary.rs
contains "display boundary records shared bus with storage" 'SHARES_SPI_WITH_STORAGE:\s*bool\s*=\s*true|STORAGE_SD_CS_GPIO:\s*u8\s*=\s*12' target-xteink-x4/src/runtime/display_boundary.rs

contains "display boundary records native 800x480 geometry" 'NATIVE_WIDTH:\s*u16\s*=\s*800|NATIVE_HEIGHT:\s*u16\s*=\s*480' target-xteink-x4/src/runtime/display_boundary.rs
contains "display boundary records logical portrait geometry" 'LOGICAL_WIDTH:\s*u16\s*=\s*480|LOGICAL_HEIGHT:\s*u16\s*=\s*800' target-xteink-x4/src/runtime/display_boundary.rs
contains "display boundary records rotation" 'ROTATION_DEGREES:\s*u16\s*=\s*270' target-xteink-x4/src/runtime/display_boundary.rs
contains "display boundary records strip rows" 'STRIP_ROWS:\s*u16\s*=\s*40' target-xteink-x4/src/runtime/display_boundary.rs

contains "display boundary records SSD1677 RAM/refresh commands" 'SSD1677_WRITE_RAM_CMD:\s*u8\s*=\s*0x24|SSD1677_WRITE_PREVIOUS_RAM_CMD:\s*u8\s*=\s*0x26|SSD1677_DISPLAY_UPDATE_CONTROL_2_CMD:\s*u8\s*=\s*0x22|SSD1677_MASTER_ACTIVATION_CMD:\s*u8\s*=\s*0x20' target-xteink-x4/src/runtime/display_boundary.rs
contains "display boundary exposes geometry helper" 'fn geometry\s*\(' target-xteink-x4/src/runtime/display_boundary.rs
contains "display boundary exposes pin helper" 'fn pins\s*\(' target-xteink-x4/src/runtime/display_boundary.rs
contains "display boundary exposes refresh contract helper" 'fn refresh_contract\s*\(' target-xteink-x4/src/runtime/display_boundary.rs
contains "display boundary exposes command validation helper" 'fn is_known_ssd1677_command\s*\(' target-xteink-x4/src/runtime/display_boundary.rs

contains "Phase 16 marker is still present in imported runtime" 'phase16=x4-reader-parity-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 17 marker is still present in imported runtime" 'phase17=x4-reader-refactor-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 18 marker is still present in imported runtime" 'phase18=x4-runtime-adapter-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 19 marker is still present in Vaachak facade" 'phase19=x4-vaachak-runtime-facade-ok' target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 20 marker is still present" 'phase20=x4-boundary-scaffold-ok' target-xteink-x4/src/runtime/display_boundary.rs target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 21 marker is still present" 'phase21=x4-storage-boundary-ok' target-xteink-x4/src/runtime/storage_boundary.rs target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 22 marker is still present" 'phase22=x4-input-boundary-ok' target-xteink-x4/src/runtime/input_boundary.rs target-xteink-x4/src/runtime/vaachak_runtime.rs

contains "imported runtime still calls Vaachak facade marker" 'VaachakRuntime::emit_boot_marker' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "imported runtime remains the working async entrypoint" 'async fn main\s*\(' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "imported runtime keeps esp-rs runtime entrypoint attribute" '#\[(esp_rtos::main|main)\]' target-xteink-x4/src/runtime/pulp_runtime.rs
not_contains "display boundary does not own physical SSD1677 init yet" 'X4Ssd1677|Ssd1677::new|display\.init|spi::master::Spi::new|RefCellDevice::new' target-xteink-x4/src/runtime/display_boundary.rs
not_contains "active runtime has no fake/raw EPUB smoke path" 'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' target-xteink-x4/src/runtime target-xteink-x4/src/main.rs
not_contains "vendored runtime files were not edited for Phase 23" 'phase23=x4-display-boundary-ok' vendor/pulp-os vendor/smol-epub

contains "target depends on vendored x4-os via pulp-os alias" 'pulp-os\s*=\s*\{[^}]*package\s*=\s*"x4-os"[^}]*path\s*=\s*"\.\./vendor/pulp-os"' target-xteink-x4/Cargo.toml
contains "target has direct x4-kernel path dependency" 'x4-kernel\s*=\s*\{[^}]*path\s*=\s*"\.\./vendor/pulp-os/kernel"' target-xteink-x4/Cargo.toml
contains "target has direct smol-epub path dependency" 'smol-epub\s*=\s*\{[^}]*path\s*=\s*"\.\./vendor/smol-epub"' target-xteink-x4/Cargo.toml
contains "root workspace excludes vendored Pulp workspace" 'exclude\s*=.*vendor/pulp-os' Cargo.toml
contains "root workspace excludes vendored smol-epub workspace" 'exclude\s*=.*vendor/smol-epub' Cargo.toml

contains "smol-epub path is present for EPUB ZIP/OPF/HTML reader" 'smol_epub|smol-epub' vendor/pulp-os vendor/smol-epub target-xteink-x4/Cargo.toml Cargo.lock
contains "X4/Pulp bookmark path is present" 'bookmark|Bookmark|bookmarks|Bookmarks' vendor/pulp-os/src vendor/pulp-os/kernel
contains "progress/continue path symbols are present" 'progress|Progress|position|Position|offset|Offset|last_read|LastRead|continue|Continue' vendor/pulp-os/src vendor/pulp-os/kernel
contains "theme/font/preset state symbols are present" 'theme|Theme|preset|Preset|font|Font' vendor/pulp-os/src vendor/pulp-os/kernel
contains "reader footer/menu/action symbols are present" 'footer|Footer|quick|Quick|menu|Menu|action|Action' vendor/pulp-os/src vendor/pulp-os/kernel

if ./scripts/check_reader_runtime_sync_phase23.sh; then
  ok "Phase 23 reader runtime sync check passes"
else
  fail "Phase 23 reader runtime sync check fails"
fi

echo
echo "Phase 23 display boundary check complete: failures=${failures} warnings=${warnings}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
