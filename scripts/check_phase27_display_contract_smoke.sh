#!/usr/bin/env bash
set -euo pipefail

failures=0
warnings=0
ok(){ printf 'OK   %s\n' "$*"; }
fail(){ printf 'FAIL %s\n' "$*"; failures=$((failures+1)); }
warn(){ printf 'WARN %s\n' "$*"; warnings=$((warnings+1)); }
exists(){ [[ -e "$1" ]] && ok "exists: $1" || fail "missing: $1"; }
contains(){ local d="$1" p="$2"; shift 2; if rg -n -e "$p" "$@" >/tmp/phase27_rg.txt 2>/dev/null; then ok "$d"; else fail "$d"; printf '      pattern: %s\n      path:    %s\n' "$p" "$*"; fi; }
not_contains(){ local d="$1" p="$2"; shift 2; if rg -n -e "$p" "$@" >/tmp/phase27_rg.txt 2>/dev/null; then fail "$d"; cat /tmp/phase27_rg.txt; else ok "$d"; fi; }

echo "Phase 27 Vaachak display contract smoke check"
echo

exists Cargo.toml
exists target-xteink-x4/Cargo.toml
exists target-xteink-x4/src/main.rs
exists target-xteink-x4/src/runtime/mod.rs
exists target-xteink-x4/src/runtime/pulp_runtime.rs
exists target-xteink-x4/src/runtime/vaachak_runtime.rs
exists target-xteink-x4/src/runtime/display_boundary.rs
exists target-xteink-x4/src/runtime/display_contract_smoke.rs
exists vendor/pulp-os/src/bin/main.rs
exists vendor/pulp-os/Cargo.toml
exists vendor/pulp-os/kernel/Cargo.toml
exists vendor/smol-epub/Cargo.toml
exists docs/phase27/PHASE27_DISPLAY_CONTRACT_SMOKE.md
exists docs/phase27/PHASE27_ACCEPTANCE.md
exists scripts/check_reader_runtime_sync_phase27.sh

if [[ "${PHASE27_RUN_CARGO:-0}" == "1" ]]; then
  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
  ok "cargo check/clippy passed inside script"
else
  ok "cargo check/clippy skipped inside script; set PHASE27_RUN_CARGO=1 to enable"
fi

contains "crate root main loads runtime module" 'mod runtime;' target-xteink-x4/src/main.rs
contains "runtime mod exposes imported Pulp runtime" 'pub mod pulp_runtime;' target-xteink-x4/src/runtime/mod.rs
contains "runtime mod exposes Vaachak facade" 'pub mod vaachak_runtime;' target-xteink-x4/src/runtime/mod.rs
contains "runtime mod exposes display boundary" 'pub mod display_boundary;' target-xteink-x4/src/runtime/mod.rs
contains "runtime mod exposes display contract smoke" 'pub mod display_contract_smoke;' target-xteink-x4/src/runtime/mod.rs

contains "display contract smoke type is present" 'struct VaachakDisplayContractSmoke' target-xteink-x4/src/runtime/display_contract_smoke.rs
contains "Phase 27 marker is present" 'phase27=x4-display-contract-smoke-ok' target-xteink-x4/src/runtime/display_contract_smoke.rs
contains "Vaachak facade emits Phase 27 marker" 'VaachakDisplayContractSmoke::emit_boot_marker' target-xteink-x4/src/runtime/vaachak_runtime.rs

contains "display contract validates native 800x480" 'NATIVE_WIDTH:\s*u16\s*=\s*800|NATIVE_WIDTH.*800' target-xteink-x4/src/runtime/display_contract_smoke.rs
contains "display contract validates logical portrait 480x800" 'LOGICAL_WIDTH:\s*u16\s*=\s*480|LOGICAL_HEIGHT:\s*u16\s*=\s*800' target-xteink-x4/src/runtime/display_contract_smoke.rs
contains "display contract validates strip rows" 'STRIP_ROWS:\s*u16\s*=\s*40|is_valid_strip_contract' target-xteink-x4/src/runtime/display_contract_smoke.rs
contains "display contract validates rotation" 'ROTATION_DEGREES:\s*u16\s*=\s*270|rotation_degrees' target-xteink-x4/src/runtime/display_contract_smoke.rs
contains "display contract records X4 EPD pins" 'EPD_CS_GPIO:\s*u8\s*=\s*21|EPD_DC_GPIO:\s*u8\s*=\s*4|EPD_RST_GPIO:\s*u8\s*=\s*5|EPD_BUSY_GPIO:\s*u8\s*=\s*6' target-xteink-x4/src/runtime/display_contract_smoke.rs
contains "display contract records shared SPI pins" 'SPI_SCLK_GPIO:\s*u8\s*=\s*8|SPI_MOSI_GPIO:\s*u8\s*=\s*10|SPI_MISO_GPIO:\s*u8\s*=\s*7|SD_CS_GPIO:\s*u8\s*=\s*12' target-xteink-x4/src/runtime/display_contract_smoke.rs
contains "display contract records SSD1677 commands" '0x24|0x26|0x22|0x20|SSD1677_CURRENT_RAM_CMD|SSD1677_PREVIOUS_RAM_CMD' target-xteink-x4/src/runtime/display_contract_smoke.rs
contains "display contract has smoke_ok helper" 'smoke_ok\s*\(' target-xteink-x4/src/runtime/display_contract_smoke.rs
contains "display contract has command validation helper" 'is_supported_ssd1677_command' target-xteink-x4/src/runtime/display_contract_smoke.rs

contains "Phase 16 marker is still present" 'phase16=x4-reader-parity-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 17 marker is still present" 'phase17=x4-reader-refactor-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 18 marker is still present" 'phase18=x4-runtime-adapter-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 19 marker is still present" 'phase19=x4-vaachak-runtime-facade-ok' target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 20 marker is still present" 'phase20=x4-boundary-scaffold-ok' target-xteink-x4/src/runtime/display_boundary.rs target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 21 marker is still present" 'phase21=x4-storage-boundary-ok' target-xteink-x4/src/runtime/storage_boundary.rs target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 22 marker is still present" 'phase22=x4-input-boundary-ok' target-xteink-x4/src/runtime/input_boundary.rs target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 23 marker is still present" 'phase23=x4-display-boundary-ok' target-xteink-x4/src/runtime/display_boundary.rs target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 24 marker is still present" 'phase24=x4-boundary-contract-ok' target-xteink-x4/src/runtime/boundary_contract.rs target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 25 marker is still present" 'phase25=x4-storage-contract-smoke-ok' target-xteink-x4/src/runtime/storage_state_contract.rs target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 26 marker is still present" 'phase26=x4-input-contract-smoke-ok' target-xteink-x4/src/runtime/input_contract_smoke.rs target-xteink-x4/src/runtime/vaachak_runtime.rs

contains "imported runtime still calls Vaachak facade marker" 'VaachakRuntime::emit_boot_marker' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "imported runtime remains the working async entrypoint" 'async fn main\s*\(' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "imported runtime keeps esp-rs runtime entrypoint attribute" '#\[(esp_rtos::main|main)\]' target-xteink-x4/src/runtime/pulp_runtime.rs

not_contains "display contract does not own physical display behavior" \
  'esp_hal::spi|spi::master|Output::new|Input::new|RefCellDevice|draw_|refresh\(|init\(|write_cmd|write_data|set_ram|delay_ms' \
  target-xteink-x4/src/runtime/display_contract_smoke.rs
not_contains "active runtime has no fake/raw EPUB smoke path" 'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' target-xteink-x4/src/runtime target-xteink-x4/src/main.rs

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

if ./scripts/check_reader_runtime_sync_phase27.sh; then
  ok "Phase 27 reader runtime sync check passes"
else
  fail "Phase 27 reader runtime sync check fails"
fi

echo
echo "Phase 27 display contract smoke check complete: failures=${failures} warnings=${warnings}"
[[ "$failures" -eq 0 ]]
