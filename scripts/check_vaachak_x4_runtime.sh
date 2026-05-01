#!/usr/bin/env bash
set -euo pipefail

failures=0
warnings=0

ok() { printf 'OK   %s\n' "$*"; }
fail() { printf 'FAIL %s\n' "$*"; failures=$((failures + 1)); }
warn() { printf 'WARN %s\n' "$*"; warnings=$((warnings + 1)); }

exists() {
  if [[ -e "$1" ]]; then
    ok "exists: $1"
  else
    fail "missing: $1"
  fi
}

contains() {
  local desc="$1"
  local pattern="$2"
  shift 2

  if rg -n -e "$pattern" "$@" >/tmp/phase30_check_rg.txt 2>/dev/null; then
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

  if rg -n -e "$pattern" "$@" >/tmp/phase30_check_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase30_check_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 30 Vaachak X4 runtime ownership check"
echo

exists Cargo.toml
exists target-xteink-x4/Cargo.toml
exists target-xteink-x4/src/main.rs

exists target-xteink-x4/src/vaachak_x4/mod.rs
exists target-xteink-x4/src/vaachak_x4/boot.rs
exists target-xteink-x4/src/vaachak_x4/runtime.rs
exists target-xteink-x4/src/vaachak_x4/imported/mod.rs
exists target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs
exists target-xteink-x4/src/vaachak_x4/contracts/mod.rs
exists target-xteink-x4/src/vaachak_x4/contracts/boundary_contract.rs
exists target-xteink-x4/src/vaachak_x4/contracts/boundary_contract_smoke.rs
exists target-xteink-x4/src/vaachak_x4/contracts/storage.rs
exists target-xteink-x4/src/vaachak_x4/contracts/input.rs
exists target-xteink-x4/src/vaachak_x4/contracts/display.rs
exists target-xteink-x4/src/vaachak_x4/contracts/storage_state_contract.rs
exists target-xteink-x4/src/vaachak_x4/contracts/storage_path_helpers.rs
exists target-xteink-x4/src/vaachak_x4/contracts/input_contract_smoke.rs
exists target-xteink-x4/src/vaachak_x4/contracts/display_contract_smoke.rs

exists docs/phase30/PHASE30_RUNTIME_OWNERSHIP.md
exists docs/phase30/PHASE30_ACCEPTANCE.md
exists docs/phase30/PHASE30_NOTES.md

exists scripts/check_imported_reader_runtime_sync.sh
exists scripts/check_vaachak_x4_runtime.sh

if cargo metadata --format-version 1 --no-deps >/tmp/phase30_metadata.json; then
  ok "cargo metadata works"
else
  fail "cargo metadata failed"
fi

if [[ "${PHASE30_RUN_CARGO:-0}" == "1" ]]; then
  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
  ok "cargo check/clippy passed inside script"
else
  ok "cargo check/clippy skipped inside script; set PHASE30_RUN_CARGO=1 to enable"
fi

contains "crate root loads vaachak_x4 module" 'mod vaachak_x4;' target-xteink-x4/src/main.rs
contains "Vaachak runtime-ready marker exists" 'vaachak=x4-runtime-ready' target-xteink-x4/src/vaachak_x4
contains "imported runtime wrapper emits active Vaachak marker" \
  'VaachakBoot::emit_runtime_ready_marker\(\)' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

not_contains "active source has no fake/raw EPUB smoke path" \
  'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' \
  target-xteink-x4/src

not_contains "normal boot does not emit old phase markers" \
  'println!\("phase(16|17|18|19|20|21|22|23|24|25|26|27|28|29)=' \
  target-xteink-x4/src/vaachak_x4 target-xteink-x4/src/main.rs

contains "smol-epub path is present for EPUB reader" \
  'smol_epub|smol-epub' \
  vendor/pulp-os vendor/smol-epub target-xteink-x4/Cargo.toml Cargo.lock

contains "target depends on vendored x4-os via pulp-os alias" \
  'pulp-os\s*=\s*\{[^}]*package\s*=\s*"x4-os"[^}]*path\s*=\s*"\.\./vendor/pulp-os"' \
  target-xteink-x4/Cargo.toml

contains "target has direct x4-kernel path dependency" \
  'x4-kernel\s*=\s*\{[^}]*path\s*=\s*"\.\./vendor/pulp-os/kernel"' \
  target-xteink-x4/Cargo.toml

contains "target has direct smol-epub path dependency" \
  'smol-epub\s*=\s*\{[^}]*path\s*=\s*"\.\./vendor/smol-epub"' \
  target-xteink-x4/Cargo.toml

contains "root workspace excludes vendored Pulp workspace" \
  'exclude\s*=.*vendor/pulp-os' Cargo.toml

contains "root workspace excludes vendored smol-epub workspace" \
  'exclude\s*=.*vendor/smol-epub' Cargo.toml

if git diff --quiet -- vendor/pulp-os vendor/smol-epub \
  && git diff --cached --quiet -- vendor/pulp-os vendor/smol-epub; then
  ok "vendor/pulp-os and vendor/smol-epub have no tracked edits"
else
  fail "vendor/pulp-os or vendor/smol-epub has tracked edits"
fi

not_contains "contract modules do not own physical SD/SPI/display/input behavior" \
  'SdCard::new|open_raw_volume|spi::master|RefCellDevice|Adc::new|read_oneshot|Output::new|Input::new|draw_|refresh\(|write_cmd|write_data|delay_ms' \
  target-xteink-x4/src/vaachak_x4/contracts

if ./scripts/check_imported_reader_runtime_sync.sh; then
  ok "imported reader runtime sync check passes"
else
  fail "imported reader runtime sync check fails"
fi

echo
echo "Phase 30 Vaachak X4 runtime ownership check complete: failures=${failures} warnings=${warnings}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
