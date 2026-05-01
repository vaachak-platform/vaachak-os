#!/usr/bin/env bash
set -euo pipefail

failures=0
warnings=0
ok() { printf 'OK   %s\n' "$*"; }
fail() { printf 'FAIL %s\n' "$*"; failures=$((failures + 1)); }
warn() { printf 'WARN %s\n' "$*"; warnings=$((warnings + 1)); }
exists() { if [[ -e "$1" ]]; then ok "exists: $1"; else fail "missing: $1"; fi; }
contains() {
  local desc="$1" pattern="$2"; shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase22_check_rg.txt 2>/dev/null; then ok "$desc"; else fail "$desc"; printf '      pattern: %s\n' "$pattern"; printf '      path:    %s\n' "$*"; fi
}
not_contains() {
  local desc="$1" pattern="$2"; shift 2
  if rg -n -e "$pattern" "$@" >/tmp/phase22_check_rg.txt 2>/dev/null; then fail "$desc"; cat /tmp/phase22_check_rg.txt; else ok "$desc"; fi
}

echo "Phase 22 Vaachak input boundary extraction check"
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
exists docs/phase22/PHASE22_INPUT_BOUNDARY.md
exists docs/phase22/PHASE22_ACCEPTANCE.md
exists scripts/check_reader_runtime_sync_phase22.sh

if [[ "${PHASE22_RUN_CARGO:-0}" == "1" ]]; then
  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf
  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings
  ok "cargo check/clippy passed inside script"
else
  ok "cargo check/clippy skipped inside script; set PHASE22_RUN_CARGO=1 to enable"
fi

contains "crate root main loads runtime module" 'mod runtime;' target-xteink-x4/src/main.rs
contains "runtime mod exposes imported Pulp runtime" 'pub mod pulp_runtime;' target-xteink-x4/src/runtime/mod.rs
contains "runtime mod exposes Vaachak facade" 'pub mod vaachak_runtime;' target-xteink-x4/src/runtime/mod.rs
contains "runtime mod exposes input boundary" 'pub mod input_boundary;' target-xteink-x4/src/runtime/mod.rs

contains "input boundary type is present" 'struct VaachakInputBoundary' target-xteink-x4/src/runtime/input_boundary.rs
contains "input boundary button role enum is present" 'enum VaachakButtonRole' target-xteink-x4/src/runtime/input_boundary.rs
contains "input boundary has Phase 22 marker" 'phase22=x4-input-boundary-ok' target-xteink-x4/src/runtime/input_boundary.rs
contains "Vaachak facade emits Phase 22 marker through input boundary" 'VaachakInputBoundary::emit_boot_marker' target-xteink-x4/src/runtime/vaachak_runtime.rs

contains "input boundary records imported runtime behavior owner" 'IMPLEMENTATION_OWNER.*vendor/pulp-os imported runtime' target-xteink-x4/src/runtime/input_boundary.rs
contains "input boundary records physical ADC reads not moved" 'PHYSICAL_ADC_READS_MOVED_IN_PHASE22.*false' target-xteink-x4/src/runtime/input_boundary.rs
contains "input boundary records ladder calibration not moved" 'BUTTON_LADDER_CALIBRATION_MOVED_IN_PHASE22.*false' target-xteink-x4/src/runtime/input_boundary.rs
contains "input boundary records debounce/repeat not moved" 'DEBOUNCE_REPEAT_HANDLING_MOVED_IN_PHASE22.*false' target-xteink-x4/src/runtime/input_boundary.rs
contains "input boundary records event routing not moved" 'BUTTON_EVENT_ROUTING_MOVED_IN_PHASE22.*false' target-xteink-x4/src/runtime/input_boundary.rs

contains "input boundary records row1 ADC GPIO" 'ROW1_ADC_GPIO.*1' target-xteink-x4/src/runtime/input_boundary.rs
contains "input boundary records row2 ADC GPIO" 'ROW2_ADC_GPIO.*2' target-xteink-x4/src/runtime/input_boundary.rs
contains "input boundary records power button GPIO" 'POWER_BUTTON_GPIO.*3' target-xteink-x4/src/runtime/input_boundary.rs

contains "input boundary records Back role" 'VaachakButtonRole::Back|Back' target-xteink-x4/src/runtime/input_boundary.rs
contains "input boundary records Select role" 'VaachakButtonRole::Select|Select' target-xteink-x4/src/runtime/input_boundary.rs
contains "input boundary records Up role" 'VaachakButtonRole::Up|Up' target-xteink-x4/src/runtime/input_boundary.rs
contains "input boundary records Down role" 'VaachakButtonRole::Down|Down' target-xteink-x4/src/runtime/input_boundary.rs
contains "input boundary records Left role" 'VaachakButtonRole::Left|Left' target-xteink-x4/src/runtime/input_boundary.rs
contains "input boundary records Right role" 'VaachakButtonRole::Right|Right' target-xteink-x4/src/runtime/input_boundary.rs
contains "input boundary records Power role" 'VaachakButtonRole::Power|Power' target-xteink-x4/src/runtime/input_boundary.rs

contains "input boundary exposes ADC ladder helper" 'is_adc_ladder_gpio' target-xteink-x4/src/runtime/input_boundary.rs
contains "input boundary exposes power GPIO helper" 'is_power_button_gpio' target-xteink-x4/src/runtime/input_boundary.rs
contains "input boundary exposes role helper" 'role_from_label' target-xteink-x4/src/runtime/input_boundary.rs
contains "input boundary exposes navigation helper" 'role_is_navigation' target-xteink-x4/src/runtime/input_boundary.rs
contains "input boundary exposes reader action helper" 'role_is_reader_action' target-xteink-x4/src/runtime/input_boundary.rs
contains "input boundary records reader footer action labels" 'READER_FOOTER_ACTION_LABELS' target-xteink-x4/src/runtime/input_boundary.rs

contains "Phase 16 marker is still present in imported runtime" 'phase16=x4-reader-parity-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 17 marker is still present in imported runtime" 'phase17=x4-reader-refactor-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 18 marker is still present in imported runtime" 'phase18=x4-runtime-adapter-ok' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "Phase 19 marker is still present in Vaachak facade" 'phase19=x4-vaachak-runtime-facade-ok' target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 20 marker is still present" 'phase20=x4-boundary-scaffold-ok' target-xteink-x4/src/runtime/display_boundary.rs target-xteink-x4/src/runtime/vaachak_runtime.rs
contains "Phase 21 marker is still present" 'phase21=x4-storage-boundary-ok' target-xteink-x4/src/runtime/storage_boundary.rs target-xteink-x4/src/runtime/vaachak_runtime.rs

contains "imported runtime still calls Vaachak facade marker" 'VaachakRuntime::emit_boot_marker' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "imported runtime remains the working async entrypoint" 'async fn main\s*\(' target-xteink-x4/src/runtime/pulp_runtime.rs
contains "imported runtime keeps esp-rs runtime entrypoint attribute" '#\[(esp_rtos::main|main)\]' target-xteink-x4/src/runtime/pulp_runtime.rs

not_contains "input boundary does not own physical ADC reads yet" 'read_oneshot|Adc<|ADC1|Input::new|X4Input::default|ingest_sample|\.poll\(' target-xteink-x4/src/runtime/input_boundary.rs
not_contains "active runtime has no fake/raw EPUB smoke path" 'run_epub_reader_page_storage_smoke|ZIP container parsed|First readable bytes|ensure_pulp_dir_async' target-xteink-x4/src/runtime target-xteink-x4/src/main.rs
not_contains "vendored runtime files were not edited for Phase 22" 'phase22=x4-input-boundary-ok' vendor/pulp-os vendor/smol-epub

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

if ./scripts/check_reader_runtime_sync_phase22.sh; then
  ok "Phase 22 reader runtime sync check passes"
else
  fail "Phase 22 reader runtime sync check fails"
fi

echo
echo "Phase 22 input boundary check complete: failures=${failures} warnings=${warnings}"
if [[ "$failures" -ne 0 ]]; then exit 1; fi
