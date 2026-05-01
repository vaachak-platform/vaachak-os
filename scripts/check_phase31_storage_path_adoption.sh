#!/usr/bin/env bash
# scripts/check_phase31_storage_path_adoption.sh

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
  if rg -n -e "$pattern" "$@" >/tmp/phase31_check_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase31_check_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase31_check_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 31 storage path adoption check"
echo

exists Cargo.toml
exists target-xteink-x4/Cargo.toml
exists target-xteink-x4/src/main.rs
exists target-xteink-x4/src/vaachak_x4/mod.rs
exists target-xteink-x4/src/vaachak_x4/boot.rs
exists target-xteink-x4/src/vaachak_x4/runtime.rs
exists target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs
exists target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs

exists docs/phase31/PHASE31_STORAGE_PATH_ADOPTION.md
exists docs/phase31/PHASE31_ACCEPTANCE.md
exists docs/phase31/PHASE31_NOTES.md
exists scripts/check_imported_reader_runtime_sync_phase31.sh
exists scripts/check_phase31_storage_path_adoption.sh
exists scripts/revert_phase31_storage_path_adoption.sh

if cargo metadata --format-version 1 --no-deps >/tmp/phase31_metadata.json; then
  ok "cargo metadata works"
else
  fail "cargo metadata failed"
fi

if [[ "${PHASE31_RUN_CARGO:-0}" == "1" ]]; then
  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
  ok "cargo check/clippy passed inside script"
else
  ok "cargo check/clippy skipped inside script; set PHASE31_RUN_CARGO=1 to enable"
fi

contains "crate root loads vaachak_x4 module" \
  'mod vaachak_x4;' \
  target-xteink-x4/src/main.rs

contains "normal boot marker remains Vaachak runtime ready" \
  'vaachak=x4-runtime-ready' \
  target-xteink-x4/src/vaachak_x4 target-xteink-x4/src/main.rs

contains "active imported runtime adopts pure storage path helper probe" \
  'VaachakStoragePathHelpers::active_runtime_adoption_probe\(\)' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

contains "storage state contract delegates to Vaachak storage path helpers" \
  'VaachakStoragePathHelpers::(is_supported_state_extension|is_reserved_state_file|is_valid_upper_book_id|state_file_name_from_str)' \
  target-xteink-x4/src/vaachak_x4/contracts/storage_state_contract.rs

contains "storage path helpers define state directory" \
  'STATE_DIR|state' \
  target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs

contains "storage path helpers define progress extension" \
  'PRG|progress' \
  target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs

contains "storage path helpers define bookmark extension" \
  'BKM|bookmark' \
  target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs

contains "storage path helpers define theme extension" \
  'THM|theme' \
  target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs

contains "storage path helpers define metadata extension" \
  'MTA|metadata' \
  target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs

contains "storage path helpers define bookmark index" \
  'BMIDX\.TXT|BOOKMARK_INDEX' \
  target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs

contains "storage path helpers expose validation helpers" \
  'is_valid_book_id|is_valid_upper_book_id|is_supported_state_extension|is_reserved_state_file|state_path' \
  target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs

contains "storage path helpers expose Phase 31 adoption probe" \
  'phase31_adoption_report|active_runtime_adoption_probe|PHYSICAL_STORAGE_IO_MOVED_IN_PHASE31' \
  target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs

not_contains "active source has no fake/raw EPUB smoke path" \
  'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' \
  target-xteink-x4/src

not_contains "normal boot does not emit old phase markers" \
  'println!\("phase(16|17|18|19|20|21|22|23|24|25|26|27|28|29)=' \
  target-xteink-x4/src/vaachak_x4 target-xteink-x4/src/main.rs

not_contains "storage path helpers do not own physical SD/SPI/file IO" \
  'SdCard::new|open_raw_volume|open_file_in_dir|open_dir|read\(|write\(|close_file|spi::master|RefCellDevice|BlockDevice|AsyncVolumeManager' \
  target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs

contains "smol-epub path is still present for EPUB reader" \
  'smol_epub|smol-epub' \
  vendor/pulp-os vendor/smol-epub target-xteink-x4/Cargo.toml Cargo.lock

if git diff --quiet -- vendor/pulp-os vendor/smol-epub \
  && git diff --cached --quiet -- vendor/pulp-os vendor/smol-epub; then
  ok "vendor/pulp-os and vendor/smol-epub have no tracked edits"
else
  fail "vendor/pulp-os or vendor/smol-epub has tracked edits"
  git diff --stat -- vendor/pulp-os vendor/smol-epub || true
fi

if ./scripts/check_imported_reader_runtime_sync_phase31.sh; then
  ok "Phase 31 imported reader runtime sync check passes"
else
  fail "Phase 31 imported reader runtime sync check fails"
fi

echo
echo "Phase 31 storage path adoption check complete: failures=${failures} warnings=${warnings}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
