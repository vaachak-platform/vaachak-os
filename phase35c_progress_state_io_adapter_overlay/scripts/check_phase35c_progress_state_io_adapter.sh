#!/usr/bin/env bash
set -euo pipefail

repo_root="${1:-.}"
module="$repo_root/target-xteink-x4/src/vaachak_x4/state/progress_state_io_adapter.rs"
mod_file="$repo_root/target-xteink-x4/src/vaachak_x4/state/mod.rs"

if [[ ! -f "$module" ]]; then
  echo "missing: $module" >&2
  exit 1
fi

if [[ ! -f "$mod_file" ]]; then
  echo "missing: $mod_file" >&2
  exit 1
fi

required_markers=(
  "PHASE_35C_PROGRESS_STATE_IO_ADAPTER_MARKER"
  "phase35c=x4-progress-state-io-adapter-ok"
  "ProgressStateIo"
  "ProgressStateIoAdapter"
  "state/"
  ".PRG"
  "PROGRESS_RECORD_LEN"
)

for marker in "${required_markers[@]}"; do
  if ! grep -q "$marker" "$module"; then
    echo "missing marker in progress_state_io_adapter.rs: $marker" >&2
    exit 1
  fi
done

if ! grep -q "pub mod progress_state_io_adapter" "$mod_file"; then
  echo "state/mod.rs does not export progress_state_io_adapter" >&2
  exit 1
fi

# Guardrail: this phase must not introduce direct hardware/storage behavior.
for forbidden in "embedded_sdmmc" "SdCard" "SpiDevice" "ssd1677" "GPIO" "PinDriver" "Fat"; do
  if grep -q "$forbidden" "$module"; then
    echo "forbidden physical/storage implementation symbol found: $forbidden" >&2
    exit 1
  fi
done

echo "phase35c=x4-progress-state-io-adapter-ok"
