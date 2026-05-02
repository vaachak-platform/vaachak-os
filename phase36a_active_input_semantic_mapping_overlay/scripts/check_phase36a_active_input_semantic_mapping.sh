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
  if rg -n -e "$pattern" "$@" >/tmp/phase36a_check_rg.txt 2>/dev/null; then
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
  if rg -n -e "$pattern" "$@" >/tmp/phase36a_check_rg.txt 2>/dev/null; then
    fail "$desc"
    cat /tmp/phase36a_check_rg.txt
  else
    ok "$desc"
  fi
}

echo "Phase 36A active input semantic mapping check"
echo

exists Cargo.toml
exists target-xteink-x4/Cargo.toml
exists target-xteink-x4/src/main.rs
exists target-xteink-x4/src/vaachak_x4/mod.rs
exists target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs
exists target-xteink-x4/src/vaachak_x4/input/mod.rs
exists target-xteink-x4/src/vaachak_x4/input/input_semantics_runtime.rs
exists target-xteink-x4/src/vaachak_x4/input/active_semantic_mapper.rs
exists docs/phase36a/PHASE36A_ACTIVE_INPUT_SEMANTIC_MAPPING.md
exists docs/phase36a/PHASE36A_ACCEPTANCE.md
exists docs/phase36a/PHASE36A_NOTES.md
exists scripts/check_imported_reader_runtime_sync_phase36a.sh
exists scripts/check_phase36a_active_input_semantic_mapping.sh
exists scripts/check_phase36a_no_input_hardware_regression.sh
exists scripts/revert_phase36a_active_input_semantic_mapping.sh

if [[ -f "$HOME/export-esp.sh" ]]; then
  # shellcheck disable=SC1090
  . "$HOME/export-esp.sh"
fi

cargo metadata --format-version 1 --no-deps >/tmp/phase36a_metadata.json
ok "cargo metadata works"

if [[ "${PHASE36A_RUN_CARGO:-1}" == "1" ]]; then
  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
  ok "target-xteink-x4 cargo check passes"
  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
  ok "target-xteink-x4 clippy passes"
else
  ok "cargo check/clippy skipped inside script; set PHASE36A_RUN_CARGO=1 to enable"
fi

contains "normal boot marker remains Vaachak runtime ready" \
  'vaachak=x4-runtime-ready' \
  target-xteink-x4/src/vaachak_x4 target-xteink-x4/src/main.rs

contains "active semantic mapper type exists" \
  'VaachakActiveInputSemanticMapper' \
  target-xteink-x4/src/vaachak_x4/input/active_semantic_mapper.rs

contains "active semantic mapper exposes imported mapper factory" \
  'new_imported_button_mapper' \
  target-xteink-x4/src/vaachak_x4/input/active_semantic_mapper.rs

contains "active semantic mapper validates default mapping" \
  'map_button_default|default_mapping_ok|ImportedButton::Confirm|ImportedAction::Select' \
  target-xteink-x4/src/vaachak_x4/input/active_semantic_mapper.rs

contains "active semantic mapper validates swapped mapping" \
  'map_button_swapped|swapped_mapping_ok|set_swap|is_swapped' \
  target-xteink-x4/src/vaachak_x4/input/active_semantic_mapper.rs

contains "active semantic mapper validates event mapping" \
  'map_event|ImportedInputEvent|ImportedActionEvent|event_mapping_ok' \
  target-xteink-x4/src/vaachak_x4/input/active_semantic_mapper.rs

contains "input module exports active semantic mapper" \
  'pub mod active_semantic_mapper' \
  target-xteink-x4/src/vaachak_x4/input/mod.rs

contains "active imported runtime calls Phase 36A mapper preflight" \
  'active_semantic_mapper|VaachakActiveInputSemanticMapper::active_runtime_preflight' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

contains "active imported runtime uses Vaachak mapper factory" \
  'VaachakActiveInputSemanticMapper::new_imported_button_mapper' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

not_contains "active imported runtime does not directly import Pulp ButtonMapper" \
  'use pulp_os::board::action::ButtonMapper;' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

not_contains "active imported runtime does not directly construct ButtonMapper" \
  'ButtonMapper::new\(\)' \
  target-xteink-x4/src/vaachak_x4/imported/pulp_reader_runtime.rs

not_contains "active source has no fake/raw EPUB smoke path" \
  'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' \
  target-xteink-x4/src

not_contains "normal boot does not emit old phase markers" \
  'println!\("phase(16|17|18|19|20|21|22|23|24|25|26|27|28|29|30|31|32|33|34|35|35b|36a)=' \
  target-xteink-x4/src/vaachak_x4 target-xteink-x4/src/main.rs

if git diff --quiet -- vendor/pulp-os vendor/smol-epub; then
  ok "vendor/pulp-os and vendor/smol-epub have no tracked edits"
else
  fail "vendor/pulp-os or vendor/smol-epub has tracked edits"
  git diff --stat -- vendor/pulp-os vendor/smol-epub || true
fi

if ./scripts/check_imported_reader_runtime_sync_phase36a.sh; then
  ok "Phase 36A imported reader runtime sync check passes"
else
  fail "Phase 36A imported reader runtime sync check fails"
fi

echo
echo "Phase 36A active input semantic mapping check complete: failures=${failures} warnings=${warnings}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
