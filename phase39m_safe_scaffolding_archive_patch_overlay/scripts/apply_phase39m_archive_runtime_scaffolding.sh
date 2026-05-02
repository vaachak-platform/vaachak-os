#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
OVERLAY="$ROOT/phase39m_safe_scaffolding_archive_patch_overlay"
RUNTIME_DIR="$ROOT/target-xteink-x4/src/vaachak_x4/runtime"
RUNTIME_MOD="$ROOT/target-xteink-x4/src/vaachak_x4/runtime.rs"
ARCHIVE_DIR="$ROOT/docs/archive/phase38-39-scaffolding/runtime"
ARCHIVE_README="$ROOT/docs/archive/phase38-39-scaffolding/README.md"
OUT="${OUT:-/tmp/phase39m-archive-runtime-scaffolding.txt}"

"$OVERLAY/scripts/guard_phase39m_archive_patch.sh" >/dev/null

mkdir -p "$ARCHIVE_DIR"

archive_files=(
  state_io_guarded_persistent_backend_stub.rs
  state_io_guarded_read_before_write_stub.rs
  state_io_guarded_write_backend_adapter_acceptance.rs
  state_io_guarded_write_backend_adapter_shape.rs
  state_io_guarded_write_backend_binding.rs
  state_io_guarded_write_backend_dry_run_executor.rs
  state_io_guarded_write_backend_implementation_seam.rs
  state_io_guarded_write_dry_run_acceptance.rs
  state_io_pre_behavior_write_enablement_consolidation.rs
  state_io_shadow_write_acceptance.rs
  state_io_shadow_write_plan.rs
  state_io_write_design_consolidation.rs
  state_io_write_lane_entry_contract.rs
  state_io_write_lane_handoff_consolidation.rs
  state_io_write_plan_design.rs
)

moved=0
already_archived=0
missing=0

for file in "${archive_files[@]}"; do
  src="$RUNTIME_DIR/$file"
  dst="$ARCHIVE_DIR/$file"

  if [ -f "$src" ]; then
    mv -v "$src" "$dst"
    moved=$((moved + 1))
  elif [ -f "$dst" ]; then
    already_archived=$((already_archived + 1))
  else
    echo "missing archive candidate: $file" >&2
    missing=$((missing + 1))
  fi
done

python3 - "$RUNTIME_MOD" <<'PY'
from pathlib import Path
import sys

runtime_mod = Path(sys.argv[1])
archive_mods = [
    "state_io_guarded_persistent_backend_stub",
    "state_io_guarded_read_before_write_stub",
    "state_io_guarded_write_backend_adapter_acceptance",
    "state_io_guarded_write_backend_adapter_shape",
    "state_io_guarded_write_backend_binding",
    "state_io_guarded_write_backend_dry_run_executor",
    "state_io_guarded_write_backend_implementation_seam",
    "state_io_guarded_write_dry_run_acceptance",
    "state_io_pre_behavior_write_enablement_consolidation",
    "state_io_shadow_write_acceptance",
    "state_io_shadow_write_plan",
    "state_io_write_design_consolidation",
    "state_io_write_lane_entry_contract",
    "state_io_write_lane_handoff_consolidation",
    "state_io_write_plan_design",
]
lines = runtime_mod.read_text().splitlines()
remove = {f"pub mod {name};" for name in archive_mods}
new_lines = [line for line in lines if line.strip() not in remove]
runtime_mod.write_text("\n".join(new_lines).rstrip() + "\n")
PY

cat > "$ARCHIVE_README" <<'EOF'
# Phase 38/39 Write-Lane Scaffolding Archive

This archive was created by Phase 39M after Phase 39I/39J/39K/39L accepted the
active write path:

```text
reader/mod.rs -> typed_state_wiring.rs -> KernelHandle -> _X4/state -> restore
```

These files were design/dry-run/handoff scaffolding and are no longer exported
from `target-xteink-x4/src/vaachak_x4/runtime.rs`.

This archive intentionally does not include Phase 39L `REVIEW DELETE LATER`
candidates. Those remain in runtime for one more build/device regression cycle.
EOF

{
  echo "# Phase 39M Archive Runtime Scaffolding"
  echo "moved=$moved"
  echo "already_archived=$already_archived"
  echo "missing=$missing"
  echo "archive_dir=$ARCHIVE_DIR"
  echo "archive_readme=$ARCHIVE_README"
  echo "marker=phase39m=x4-safe-scaffolding-archive-patch-ok"
  echo
  echo "## archived files"
  for file in "${archive_files[@]}"; do
    echo "- $file"
  done
} | tee "$OUT"

echo
echo "Wrote: $OUT"

if [ "$missing" -ne 0 ]; then
  exit 4
fi
