#!/usr/bin/env bash
set -euo pipefail

failures=0
warnings=0
ok(){ printf 'OK   %s\n' "$*"; }
fail(){ printf 'FAIL %s\n' "$*"; failures=$((failures+1)); }
warn(){ printf 'WARN %s\n' "$*"; warnings=$((warnings+1)); }
exists(){ [[ -e "$1" ]] && ok "exists: $1" || fail "missing: $1"; }
contains(){ local d="$1" p="$2"; shift 2; if rg -n -e "$p" "$@" >/tmp/phase29_rg.txt 2>/dev/null; then ok "$d"; else fail "$d"; printf '      pattern: %s\n      path: %s\n' "$p" "$*"; fi; }
not_contains(){ local d="$1" p="$2"; shift 2; if rg -n -e "$p" "$@" >/tmp/phase29_rg.txt 2>/dev/null; then fail "$d"; cat /tmp/phase29_rg.txt; else ok "$d"; fi; }

echo "Phase 29 Vaachak storage path helper extraction check"
echo

exists Cargo.toml
exists target-xteink-x4/Cargo.toml
exists target-xteink-x4/src/main.rs
exists target-xteink-x4/src/runtime/mod.rs
exists target-xteink-x4/src/runtime/pulp_runtime.rs
exists target-xteink-x4/src/runtime/vaachak_runtime.rs
exists target-xteink-x4/src/runtime/storage_path_helpers.rs
exists vendor/pulp-os/src/bin/main.rs
exists vendor/pulp-os/Cargo.toml
exists vendor/smol-epub/Cargo.toml
exists docs/phase29/PHASE29_STORAGE_PATH_HELPERS.md
exists docs/phase29/PHASE29_ACCEPTANCE.md
exists scripts/check_reader_runtime_sync_phase29.sh

if [[ "${PHASE29_RUN_CARGO:-0}" == "1" ]]; then
  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
  ok "cargo check/clippy passed inside script"
else
  ok "cargo check/clippy skipped inside script; set PHASE29_RUN_CARGO=1 to enable"
fi

contains "runtime mod exposes storage path helpers" 'pub mod storage_path_helpers;' target-xteink-x4/src/runtime/mod.rs
contains "storage path helper type is present" 'struct VaachakStoragePathHelpers' target-xteink-x4/src/runtime/storage_path_helpers.rs
contains "Phase 29 marker is present" 'phase29=x4-storage-path-helpers-ok' target-xteink-x4/src/runtime/storage_path_helpers.rs
contains "Vaachak facade emits only Phase 29 helper marker" 'VaachakStoragePathHelpers::emit_phase29_marker' target-xteink-x4/src/runtime/vaachak_runtime.rs

contains "progress path helper exists" 'progress_path' target-xteink-x4/src/runtime/storage_path_helpers.rs
contains "bookmark path helper exists" 'bookmark_path' target-xteink-x4/src/runtime/storage_path_helpers.rs
contains "theme path helper exists" 'theme_path' target-xteink-x4/src/runtime/storage_path_helpers.rs
contains "metadata path helper exists" 'metadata_path' target-xteink-x4/src/runtime/storage_path_helpers.rs
contains "bookmark index path helper exists" 'bookmark_index_path' target-xteink-x4/src/runtime/storage_path_helpers.rs
contains "book id validation helper exists" 'is_valid_book_id' target-xteink-x4/src/runtime/storage_path_helpers.rs
contains "state extension validation helper exists" 'is_supported_state_extension' target-xteink-x4/src/runtime/storage_path_helpers.rs
contains "state path constants include state dir" 'STATE_DIR' target-xteink-x4/src/runtime/storage_path_helpers.rs
contains "state path constants include PRG/BKM/THM/MTA" 'PROGRESS_EXTENSION|BOOKMARK_EXTENSION|THEME_EXTENSION|METADATA_EXTENSION' target-xteink-x4/src/runtime/storage_path_helpers.rs

not_contains "previous phase boot print calls removed from active boot path" 'esp_println::println!\(\s*"phase(16|17|18|19|20|21|22|23|24|25|26|27|28)=' target-xteink-x4/src/runtime/pulp_runtime.rs target-xteink-x4/src/runtime/vaachak_runtime.rs
not_contains "storage path helpers do not own physical SD/SPI/file IO" 'SdCard|open_file|open_dir|read\(|write\(|RefCellDevice|spi::|Output::new|Input::new|embedded_sdmmc' target-xteink-x4/src/runtime/storage_path_helpers.rs
not_contains "active runtime has no fake/raw EPUB smoke path" 'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' target-xteink-x4/src/runtime target-xteink-x4/src/main.rs

contains "imported runtime still calls Vaachak facade marker" 'VaachakRuntime::emit_boot_marker' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "imported runtime remains the working async entrypoint" 'async fn main\s*\(' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "target depends on vendored x4-os via pulp-os alias" 'pulp-os\s*=\s*\{[^}]*package\s*=\s*"x4-os"[^}]*path\s*=\s*"\.\./vendor/pulp-os"' target-xteink-x4/Cargo.toml
contains "target has direct smol-epub path dependency" 'smol-epub\s*=\s*\{[^}]*path\s*=\s*"\.\./vendor/smol-epub"' target-xteink-x4/Cargo.toml
contains "root workspace excludes vendored Pulp workspace" 'exclude\s*=.*vendor/pulp-os' Cargo.toml

if ./scripts/check_reader_runtime_sync_phase29.sh; then ok "Phase 29 reader runtime sync check passes"; else fail "Phase 29 reader runtime sync check fails"; fi

echo
echo "Phase 29 storage path helpers check complete: failures=${failures} warnings=${warnings}"
[[ "$failures" -eq 0 ]]
