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
  if rg -n -e "$pattern" "$@" >/tmp/phase20_check_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase20_check_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase20_check_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 20 Vaachak display/input/storage boundary scaffold check"
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
exists docs/phase20/PHASE20_BOUNDARY_SCAFFOLD.md
exists docs/phase20/PHASE20_ACCEPTANCE.md
exists scripts/check_reader_runtime_sync_phase20.sh

if [[ "${PHASE20_RUN_CARGO:-0}" == "1" ]]; then
  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
  ok "cargo check/clippy passed inside script"
else
  ok "cargo check/clippy skipped inside script; set PHASE20_RUN_CARGO=1 to enable"
fi

contains "crate root main loads runtime module" 'mod runtime;' target-xteink-x4/src/main.rs
contains "runtime mod exposes imported Pulp runtime" 'pub mod pulp_runtime;' target-xteink-x4/src/runtime/mod.rs
contains "runtime mod exposes Vaachak facade" 'pub mod vaachak_runtime;' target-xteink-x4/src/runtime/mod.rs
contains "runtime mod exposes display boundary" 'pub mod display_boundary;' target-xteink-x4/src/runtime/mod.rs
contains "runtime mod exposes input boundary" 'pub mod input_boundary;' target-xteink-x4/src/runtime/mod.rs
contains "runtime mod exposes storage boundary" 'pub mod storage_boundary;' target-xteink-x4/src/runtime/mod.rs

contains "display boundary scaffold type is present" 'struct VaachakDisplayBoundary' target-xteink-x4/src/runtime/display_boundary.rs
contains "input boundary scaffold type is present" 'struct VaachakInputBoundary' target-xteink-x4/src/runtime/input_boundary.rs
contains "storage boundary scaffold type is present" 'struct VaachakStorageBoundary' target-xteink-x4/src/runtime/storage_boundary.rs

contains "display boundary records X4 display pins" 'EPD_CS_GPIO|EPD_DC_GPIO|EPD_RST_GPIO|EPD_BUSY_GPIO|SPI_SCLK_GPIO|SPI_MOSI_GPIO|SPI_MISO_GPIO' target-xteink-x4/src/runtime/display_boundary.rs
contains "input boundary records X4 input pins" 'ROW1_ADC_GPIO|ROW2_ADC_GPIO|POWER_BUTTON_GPIO' target-xteink-x4/src/runtime/input_boundary.rs
contains "storage boundary records X4 SD pin and shared bus" 'SD_CS_GPIO|SHARES_DISPLAY_SPI_BUS' target-xteink-x4/src/runtime/storage_boundary.rs

contains "Phase 19 marker is still present in Vaachak facade" 'phase19=x4-vaachak-runtime-facade-ok' target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 20 marker is present in display boundary" 'phase20=x4-boundary-scaffold-ok' target-xteink-x4/src/runtime/display_boundary.rs
contains "Vaachak facade emits Phase 20 marker through boundary scaffold" 'VaachakDisplayBoundary::emit_scaffold_marker' target-xteink-x4/src/runtime/vaachak_runtime.rs

contains "Phase 16 marker is still present in imported runtime" 'phase16=x4-reader-parity-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 17 marker is still present in imported runtime" 'phase17=x4-reader-refactor-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 18 marker is still present in imported runtime" 'phase18=x4-runtime-adapter-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "imported runtime still calls Phase 19 facade marker" 'VaachakRuntime::emit_boot_marker' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "imported runtime remains the working async entrypoint" 'async fn main\s*\(' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "imported runtime keeps esp-rs runtime entrypoint attribute" '#\[(esp_rtos::main|main)\]' target-xteink-x4/src/runtime/pulp_runtime.rs

not_contains "boundary modules do not own hardware init yet" 'Spi::new|SdCard::new|X4Ssd1677|Adc::new|Input::new|Output::new' target-xteink-x4/src/runtime/display_boundary.rs target-xteink-x4/src/runtime/input_boundary.rs target-xteink-x4/src/runtime/storage_boundary.rs
not_contains "active runtime has no fake/raw EPUB smoke path" 'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' target-xteink-x4/src/runtime target-xteink-x4/src/main.rs
not_contains "vendored runtime files were not edited for Phase 20" 'phase20=x4-boundary-scaffold-ok|VaachakDisplayBoundary|VaachakInputBoundary|VaachakStorageBoundary' vendor/pulp-os vendor/smol-epub

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

if ./scripts/check_reader_runtime_sync_phase20.sh; then
  ok "Phase 20 reader runtime sync check passes"
else
  fail "Phase 20 reader runtime sync check fails"
fi

echo
echo "Phase 20 boundary scaffold check complete: failures=${failures} warnings=${warnings}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
