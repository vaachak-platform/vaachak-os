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
  if rg -n -e "$pattern" "$@" >/tmp/phase32_34_check_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase32_34_check_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase32_34_check_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 32-34 active helper adoption check"
echo

exists Cargo.toml
exists target-xteink-x4/Cargo.toml
exists target-xteink-x4/src/main.rs
exists target-xteink-x4/src/vaachak_x4/mod.rs
exists target-xteink-x4/src/vaachak_x4/boot.rs
exists target-xteink-x4/src/vaachak_x4/runtime.rs
exists target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs
exists target-xteink-x4/src/vaachak_x4/contracts/mod.rs
exists target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs
exists target-xteink-x4/src/vaachak_x4/contracts/storage_state_contract.rs
exists target-xteink-x4/src/vaachak_x4/contracts/input_semantics.rs
exists target-xteink-x4/src/vaachak_x4/contracts/display_geometry.rs
exists docs/phase32_34/PHASE32_34_ACTIVE_HELPER_ADOPTION.md
exists docs/phase32_34/PHASE32_34_ACCEPTANCE.md
exists docs/phase32_34/PHASE32_34_NOTES.md
exists scripts/check_imported_reader_runtime_sync_phase32_34.sh
exists scripts/check_phase32_34_active_helper_adoption.sh
exists scripts/revert_phase32_34_active_helper_adoption.sh

if cargo metadata --format-version 1 --no-deps >/tmp/phase32_34_metadata.json; then
  ok "cargo metadata works"
else
  fail "cargo metadata failed"
fi

if [[ "${PHASE32_34_RUN_CARGO:-0}" == "1" ]]; then
  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
  ok "cargo check/clippy passed inside script"
else
  ok "cargo check/clippy skipped inside script; set PHASE32_34_RUN_CARGO=1 to enable"
fi

contains "crate root loads vaachak_x4 module" 'mod vaachak_x4;' target-xteink-x4/src/main.rs
contains "normal boot marker remains Vaachak runtime ready" 'vaachak=x4-runtime-ready' target-xteink-x4/src/vaachak_x4 target-xteink-x4/src/main.rs

contains "active runtime calls Vaachak storage helper/probe" 'VaachakStoragePathHelpers::active_runtime_adoption_probe\(\)' target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs
contains "active runtime calls Vaachak input helper/probe" 'VaachakInputSemantics::active_runtime_adoption_probe\(\)' target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs
contains "active runtime calls Vaachak display helper/probe" 'VaachakDisplayGeometry::active_runtime_adoption_probe\(\)' target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

contains "storage path helpers expose path/name behavior" 'progress_path|bookmark_path|theme_path|metadata_path|bookmark_index|is_valid|state_file' target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs
contains "input semantics expose button/action behavior" 'Back|Select|Up|Down|Left|Right|Power|ReaderAction|InputSemantic|semantic' target-xteink-x4/src/vaachak_x4/contracts/input_semantics.rs
contains "display geometry exposes geometry/command behavior" '800|480|SSD1677|0x24|0x26|0x22|0x20|strip|rotation|geometry' target-xteink-x4/src/vaachak_x4/contracts/display_geometry.rs

not_contains "active source has no fake/raw EPUB smoke path" 'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' target-xteink-x4/src

not_contains "normal boot does not emit old phase markers" 'println!\("phase(16|17|18|19|20|21|22|23|24|25|26|27|28|29|30|31|32|33|34)=' target-xteink-x4/src/vaachak_x4 target-xteink-x4/src/main.rs

not_contains "storage helpers do not own physical SD/SPI/file IO" 'SdCard::new|open_raw_volume|open_file_in_dir|open_dir|read\(|write\(|close_file|spi::master|RefCellDevice|BlockDevice|AsyncVolumeManager' target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs target-xteink-x4/src/vaachak_x4/contracts/storage_state_contract.rs
not_contains "input helpers do not own ADC/debounce/event polling" 'Adc::new|read_oneshot|Input::new|debounce|repeat|poll\(|sample\(' target-xteink-x4/src/vaachak_x4/contracts/input_semantics.rs
not_contains "display helpers do not own SSD1677/SPI/refresh/rendering" 'esp_hal::spi|spi::master|Output::new|Input::new|RefCellDevice|draw_|refresh\(|init\(|write_cmd|write_data|set_ram|delay_ms' target-xteink-x4/src/vaachak_x4/contracts/display_geometry.rs

contains "smol-epub path is still present for EPUB reader" 'smol_epub|smol-epub' vendor/pulp-os vendor/smol-epub target-xteink-x4/Cargo.toml Cargo.lock

if git diff --quiet -- vendor/pulp-os vendor/smol-epub \
  && git diff --cached --quiet -- vendor/pulp-os vendor/smol-epub; then
  ok "vendor/pulp-os and vendor/smol-epub have no tracked edits"
else
  fail "vendor/pulp-os or vendor/smol-epub has tracked edits"
  git diff --stat -- vendor/pulp-os vendor/smol-epub || true
fi

if ./scripts/check_imported_reader_runtime_sync_phase32_34.sh; then
  ok "Phase 32-34 imported reader runtime sync check passes"
else
  fail "Phase 32-34 imported reader runtime sync check fails"
fi

echo
echo "Phase 32-34 active helper adoption check complete: failures=${failures} warnings=${warnings}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
