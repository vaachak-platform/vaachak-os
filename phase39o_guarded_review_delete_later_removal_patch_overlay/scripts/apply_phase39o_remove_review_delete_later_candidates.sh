#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase39o_guarded_review_delete_later_removal_patch_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"
ARCHIVE_DIR="$ROOT/docs/archive/phase38-39-scaffolding/review-delete-later-runtime"
ARCHIVE_README="$ROOT/docs/archive/phase38-39-scaffolding/review-delete-later-runtime/README.md"
OUT="${OUT:-/tmp/phase39o-remove-review-delete-later-candidates.txt}"

"$OVERLAY/scripts/guard_phase39o_accepted_write_path.sh" >/dev/null
"$OVERLAY/scripts/check_phase39o_external_candidate_refs.sh" >/dev/null

mkdir -p "$ARCHIVE_DIR"

candidate_files=(
  state_io_progress_write_backend_binding.rs
  state_io_progress_write_callback_backend.rs
  state_io_progress_write_lane_acceptance.rs
  state_io_progress_write_lane.rs
  state_io_runtime_file_api_integration_gate_acceptance.rs
  state_io_runtime_file_api_integration_gate.rs
  state_io_runtime_owned_sdfat_writer_acceptance.rs
  state_io_runtime_owned_sdfat_writer.rs
  state_io_typed_record_sdfat_adapter_acceptance.rs
  state_io_typed_record_sdfat_adapter.rs
  state_io_typed_record_write_lane_acceptance.rs
  state_io_typed_record_write_lane.rs
  state_io_typed_state_runtime_callsite_wiring_acceptance.rs
  state_io_typed_state_runtime_callsite_wiring.rs
)

moved=0
already_archived=0
missing=0

for file in "${candidate_files[@]}"; do
  src="$RUNTIME_DIR/$file"
  dst="$ARCHIVE_DIR/$file"

  if [ -f "$src" ]; then
    mv -v "$src" "$dst"
    moved=$((moved + 1))
  elif [ -f "$dst" ]; then
    already_archived=$((already_archived + 1))
  else
    echo "missing removal candidate: $file" >&2
    missing=$((missing + 1))
  fi
done

python3 - "$RUNTIME_MOD" <<'PY'
from pathlib import Path
import sys

runtime_mod = Path(sys.argv[1])
candidate_mods = [
    "state_io_progress_write_backend_binding",
    "state_io_progress_write_callback_backend",
    "state_io_progress_write_lane_acceptance",
    "state_io_progress_write_lane",
    "state_io_runtime_file_api_integration_gate_acceptance",
    "state_io_runtime_file_api_integration_gate",
    "state_io_runtime_owned_sdfat_writer_acceptance",
    "state_io_runtime_owned_sdfat_writer",
    "state_io_typed_record_sdfat_adapter_acceptance",
    "state_io_typed_record_sdfat_adapter",
    "state_io_typed_record_write_lane_acceptance",
    "state_io_typed_record_write_lane",
    "state_io_typed_state_runtime_callsite_wiring_acceptance",
    "state_io_typed_state_runtime_callsite_wiring",
]
remove = {f"pub mod {name};" for name in candidate_mods}
lines = runtime_mod.read_text().splitlines()
new_lines = [line for line in lines if line.strip() not in remove]
runtime_mod.write_text("\n".join(new_lines).rstrip() + "\n")
PY

cat > "$ARCHIVE_README" <<'EOF'
# Phase 39O Review-Delete-Later Runtime Archive

These files were moved out of the runtime build surface by Phase 39O after
Phase 39N dry-run acceptance.

Accepted active write path remains:

```text
vendor/pulp-os/src/apps/reader/mod.rs
  -> vendor/pulp-os/src/apps/reader/typed_state_wiring.rs
  -> KernelHandle
  -> _X4/state
  -> restore
```

These files were previously target-side adapter/facade experiments. The active
reader path uses the Pulp-local `typed_state_wiring.rs` facade.
EOF

"$OVERLAY/scripts/guard_phase39o_accepted_write_path.sh" >/dev/null

{
  echo "# Phase 39O Remove Review-Delete-Later Candidates"
  echo "moved=$moved"
  echo "already_archived=$already_archived"
  echo "missing=$missing"
  echo "archive_dir=$ARCHIVE_DIR"
  echo "archive_readme=$ARCHIVE_README"
  echo "marker=phase39o=x4-guarded-review-delete-later-removal-patch-ok"
  echo
  echo "## moved files"
  for file in "${candidate_files[@]}"; do
    echo "- $file"
  done
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$missing" -ne 0 ]; then
  exit 5
fi
