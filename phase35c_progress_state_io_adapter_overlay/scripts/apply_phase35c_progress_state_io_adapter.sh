#!/usr/bin/env bash
set -euo pipefail

overlay_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
repo_root="${1:-$(pwd)}"

state_src="$overlay_root/replaceable/target-xteink-x4/src/vaachak_x4/state/progress_state_io_adapter.rs"
state_mod_src="$overlay_root/replaceable/target-xteink-x4/src/vaachak_x4/state/mod.rs"
state_dst_dir="$repo_root/target-xteink-x4/src/vaachak_x4/state"
state_dst="$state_dst_dir/progress_state_io_adapter.rs"
state_mod_dst="$state_dst_dir/mod.rs"
vaachak_mod="$repo_root/target-xteink-x4/src/vaachak_x4/mod.rs"

if [[ ! -d "$repo_root/target-xteink-x4/src/vaachak_x4" ]]; then
  echo "error: repo root does not look like vaachak-os: $repo_root" >&2
  echo "run this from the vaachak-os repo root, or pass the repo root as the first argument" >&2
  exit 1
fi

mkdir -p "$state_dst_dir"
cp -av "$state_src" "$state_dst"

if [[ -f "$state_mod_dst" ]]; then
  if ! grep -q 'pub mod progress_state_io_adapter;' "$state_mod_dst"; then
    printf '\n// Phase 35C — progress-state I/O adapter overlay.\npub mod progress_state_io_adapter;\n' >> "$state_mod_dst"
    echo "merged module export into $state_mod_dst"
  else
    echo "module export already present in $state_mod_dst"
  fi
else
  cp -av "$state_mod_src" "$state_mod_dst"
fi

if [[ -f "$vaachak_mod" ]]; then
  if ! grep -q 'pub mod state;' "$vaachak_mod"; then
    printf '\n// Phase 35C — Vaachak-owned progress state boundary.\npub mod state;\n' >> "$vaachak_mod"
    echo "merged state module export into $vaachak_mod"
  else
    echo "state module export already present in $vaachak_mod"
  fi
else
  echo "warning: $vaachak_mod not found; add 'pub mod state;' manually if needed" >&2
fi

bash "$overlay_root/scripts/check_phase35c_progress_state_io_adapter.sh" "$repo_root"

echo "Phase 35C overlay applied. Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo check --workspace --all-targets"
echo "  cargo test --workspace --all-targets"
echo "  cargo clippy --workspace --all-targets -- -D warnings"
