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
  if rg -n -e "$pattern" "$@" >/tmp/phase35b_check_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase35b_check_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase35b_check_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 35B storage state IO runtime wiring check"
echo

exists Cargo.toml
exists target-xteink-x4/Cargo.toml
exists target-xteink-x4/src/main.rs
exists target-xteink-x4/src/vaachak_x4/mod.rs
exists target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs
exists target-xteink-x4/src/vaachak_x4/io/mod.rs
exists target-xteink-x4/src/vaachak_x4/io/storage_state.rs
exists target-xteink-x4/src/vaachak_x4/io/storage_state_adapter.rs
exists target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs

exists docs/phase35b/PHASE35B_STORAGE_STATE_IO_WIRING.md
exists docs/phase35b/PHASE35B_ACCEPTANCE.md
exists docs/phase35b/PHASE35B_NOTES.md
exists docs/phase35b/PHASE35B_WIRING_OPTIONS.md

exists scripts/check_imported_reader_runtime_sync_phase35b.sh
exists scripts/check_phase35b_storage_state_io_wiring.sh
exists scripts/check_phase35b_no_vendor_or_hardware_regression.sh
exists scripts/revert_phase35b_storage_state_io_wiring.sh

if [[ -f "$HOME/export-esp.sh" ]]; then
  # shellcheck disable=SC1090
  . "$HOME/export-esp.sh"
fi

cargo metadata --format-version 1 --no-deps >/tmp/phase35b_metadata.json
ok "cargo metadata works"

cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
ok "target-xteink-x4 cargo check passes"

cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
ok "target-xteink-x4 clippy passes"

contains "normal boot marker remains Vaachak runtime ready" \
  'vaachak=x4-runtime-ready' \
  target-xteink-x4/src/vaachak_x4 target-xteink-x4/src/main.rs

contains "storage state runtime bridge type exists" \
  'VaachakStorageStateRuntimeBridge|StorageStateRuntimeBridge' \
  target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs

contains "storage state runtime bridge exposes active preflight" \
  'active_runtime_preflight|runtime_preflight|preflight' \
  target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs

contains "storage state runtime bridge references Progress state" \
  'Progress' \
  target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs

contains "storage state runtime bridge references Bookmark state" \
  'Bookmark' \
  target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs

contains "storage state runtime bridge references Theme state" \
  'Theme' \
  target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs

contains "storage state runtime bridge references Metadata state" \
  'Metadata' \
  target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs

contains "storage state runtime bridge uses seam paths or adapter" \
  'VaachakStorageStatePaths|VaachakStorageStateIoAdapter|VaachakStorageStateIo' \
  target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs

contains "active imported runtime calls Phase 35B storage state runtime bridge" \
  'storage_state_runtime|VaachakStorageStateRuntimeBridge|active_runtime_preflight' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

contains "io module exports storage state runtime bridge" \
  'pub mod storage_state_runtime' \
  target-xteink-x4/src/vaachak_x4/io/mod.rs

not_contains "active source has no fake/raw EPUB smoke path" \
  'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' \
  target-xteink-x4/src

not_contains "normal boot does not emit old phase markers" \
  'println!\("phase(16|17|18|19|20|21|22|23|24|25|26|27|28|29|30|31|32|33|34|35|35b)=' \
  target-xteink-x4/src/vaachak_x4 target-xteink-x4/src/main.rs

not_contains "active imported runtime does not call old phase marker emitters" \
  'emit_phase[0-9]|emit_contract_marker|emit_boot_marker|emit_all_boundary_markers|PHASE[0-9_]*MARKER|phase35=|phase35b=' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

not_contains "storage state runtime bridge does not own physical SD/SPI/FAT IO" \
  'SdCard::new|open_raw_volume|open_file_in_dir|open_dir|read\(|write\(|close_file|spi::master|RefCellDevice|BlockDevice|AsyncVolumeManager|VolumeManager|FileMode' \
  target-xteink-x4/src/vaachak_x4/io/storage_state_runtime.rs

not_contains "active imported runtime wrapper does not add physical storage IO for Phase 35B" \
  'VaachakStorageStatePathIo|read_state_path|write_state_path|open_file_in_dir|open_dir|close_file|AsyncVolumeManager|VolumeManager|FileMode' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

if git diff --quiet -- vendor/pulp-os vendor/smol-epub; then
  ok "vendor/pulp-os and vendor/smol-epub have no tracked edits"
else
  fail "vendor/pulp-os or vendor/smol-epub has tracked edits"
  git diff --stat -- vendor/pulp-os vendor/smol-epub || true
fi

if ./scripts/check_imported_reader_runtime_sync_phase35b.sh; then
  ok "Phase 35B imported reader runtime sync check passes"
else
  fail "Phase 35B imported reader runtime sync check fails"
fi

echo
echo "Phase 35B storage state IO runtime wiring check complete: failures=${failures} warnings=${warnings}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
