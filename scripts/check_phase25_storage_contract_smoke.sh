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
  if rg -n -e "$pattern" "$@" >/tmp/phase25_check_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase25_check_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase25_check_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 25 Vaachak storage state contract smoke check"
echo

exists Cargo.toml
exists target-xteink-x4/Cargo.toml
exists target-xteink-x4/src/main.rs
exists target-xteink-x4/src/runtime/mod.rs
exists target-xteink-x4/src/runtime/pulp_runtime.rs
exists target-xteink-x4/src/runtime/vaachak_runtime.rs
exists target-xteink-x4/src/runtime/storage_boundary.rs
exists target-xteink-x4/src/runtime/storage_state_contract.rs
exists vendor/pulp-os/src/bin/main.rs
exists vendor/pulp-os/Cargo.toml
exists vendor/pulp-os/kernel/Cargo.toml
exists vendor/smol-epub/Cargo.toml
exists docs/phase25/PHASE25_STORAGE_CONTRACT_SMOKE.md
exists docs/phase25/PHASE25_ACCEPTANCE.md
exists scripts/check_reader_runtime_sync_phase25.sh

if [[ "${PHASE25_RUN_CARGO:-0}" == "1" ]]; then
  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
  ok "cargo check/clippy passed inside script"
else
  ok "cargo check/clippy skipped inside script; set PHASE25_RUN_CARGO=1 to enable"
fi

contains "crate root main loads runtime module" 'mod runtime;' target-xteink-x4/src/main.rs
contains "runtime mod exposes imported Pulp runtime" 'pub mod pulp_runtime;' target-xteink-x4/src/runtime/mod.rs
contains "runtime mod exposes Vaachak facade" 'pub mod vaachak_runtime;' target-xteink-x4/src/runtime/mod.rs
contains "runtime mod exposes storage boundary" 'pub mod storage_boundary;' target-xteink-x4/src/runtime/mod.rs
contains "runtime mod exposes storage state contract" 'pub mod storage_state_contract;' target-xteink-x4/src/runtime/mod.rs

contains "storage state contract type is present" 'struct VaachakStorageStateContract' target-xteink-x4/src/runtime/storage_state_contract.rs
contains "storage state contract has Phase 25 marker" 'phase25=x4-storage-contract-smoke-ok' target-xteink-x4/src/runtime/storage_state_contract.rs
contains "Vaachak facade emits Phase 25 marker through storage state contract" 'VaachakStorageStateContract::emit_contract_marker' target-xteink-x4/src/runtime/vaachak_runtime.rs

contains "storage state contract records state directory" 'STATE_DIR.*state' target-xteink-x4/src/runtime/storage_state_contract.rs
contains "storage state contract records PRG extension" 'PROGRESS_EXT.*PRG' target-xteink-x4/src/runtime/storage_state_contract.rs
contains "storage state contract records BKM extension" 'BOOKMARK_EXT.*BKM' target-xteink-x4/src/runtime/storage_state_contract.rs
contains "storage state contract records THM extension" 'THEME_EXT.*THM' target-xteink-x4/src/runtime/storage_state_contract.rs
contains "storage state contract records MTA extension" 'METADATA_EXT.*MTA' target-xteink-x4/src/runtime/storage_state_contract.rs
contains "storage state contract records bookmark index file" 'BOOKMARK_INDEX_FILE.*BMIDX\.TXT' target-xteink-x4/src/runtime/storage_state_contract.rs
contains "storage state contract records EPUB cache owner" 'EPUB_CACHE_OWNER.*vendor/pulp-os' target-xteink-x4/src/runtime/storage_state_contract.rs

contains "storage state contract validates known extensions" 'is_known_state_extension' target-xteink-x4/src/runtime/storage_state_contract.rs
contains "storage state contract validates reserved files" 'is_reserved_state_file' target-xteink-x4/src/runtime/storage_state_contract.rs
contains "storage state contract validates book IDs" 'is_valid_book_id' target-xteink-x4/src/runtime/storage_state_contract.rs
contains "storage state contract validates state file names" 'is_valid_state_file_name' target-xteink-x4/src/runtime/storage_state_contract.rs
contains "storage state contract builds 8.3 file names" 'state_file_name' target-xteink-x4/src/runtime/storage_state_contract.rs
contains "storage state contract exposes smoke validator" 'smoke_validate_contract' target-xteink-x4/src/runtime/storage_state_contract.rs

contains "Phase 16 marker is still present in imported runtime" 'phase16=x4-reader-parity-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 17 marker is still present in imported runtime" 'phase17=x4-reader-refactor-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 18 marker is still present in imported runtime" 'phase18=x4-runtime-adapter-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 19 marker is still present in Vaachak facade" 'phase19=x4-vaachak-runtime-facade-ok' target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 20 marker is still present" 'phase20=x4-boundary-scaffold-ok' target-xteink-x4/src/runtime/display_boundary.rs target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 21 marker is still present" 'phase21=x4-storage-boundary-ok' target-xteink-x4/src/runtime/storage_boundary.rs target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 22 marker is still present" 'phase22=x4-input-boundary-ok' target-xteink-x4/src/runtime/input_boundary.rs target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 23 marker is still present" 'phase23=x4-display-boundary-ok' target-xteink-x4/src/runtime/display_boundary.rs target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 24 marker is still present" 'phase24=x4-boundary-contract-ok' target-xteink-x4/src/runtime/boundary_contract.rs target-xteink-x4/src/runtime/vaachak_runtime.rs

contains "imported runtime still calls Vaachak facade marker" 'VaachakRuntime::emit_boot_marker' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "imported runtime remains the working async entrypoint" 'async fn main\s*\(' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "imported runtime keeps esp-rs runtime entrypoint attribute" '#\[(esp_rtos::main|main)\]' target-xteink-x4/src/runtime/pulp_runtime.rs

contains "storage state contract records physical SD IO not moved" 'PHYSICAL_SD_IO_MOVED_IN_PHASE25.*false' target-xteink-x4/src/runtime/storage_state_contract.rs
contains "storage state contract records EPUB cache IO not moved" 'EPUB_CACHE_IO_MOVED_IN_PHASE25.*false' target-xteink-x4/src/runtime/storage_state_contract.rs
not_contains "storage state contract does not perform physical SD/FAT IO" 'SdCard::new|open_raw_volume|open_file_in_dir|write\(|read\(|with_dma|SPI2|GPIO12' target-xteink-x4/src/runtime/storage_state_contract.rs
not_contains "active runtime has no fake/raw EPUB smoke path" 'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' target-xteink-x4/src/runtime target-xteink-x4/src/main.rs
not_contains "vendored runtime files were not edited for Phase 25" 'phase25=x4-storage-contract-smoke-ok' vendor/pulp-os vendor/smol-epub

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

if ./scripts/check_reader_runtime_sync_phase25.sh; then
  ok "Phase 25 reader runtime sync check passes"
else
  fail "Phase 25 reader runtime sync check fails"
fi

echo
echo "Phase 25 storage state contract smoke check complete: failures=${failures} warnings=${warnings}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
