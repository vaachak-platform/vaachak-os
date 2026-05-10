#!/usr/bin/env bash
set -euo pipefail
python3 scripts/validate_x4_standard_partition_table_compatibility.py
python3 - <<'PY'
from pathlib import Path
text = Path('scripts/validate_x4_standard_partition_table_compatibility.py').read_text()
required = [
    'def iter_regression_scan_files()',
    'Only source/config',
    'scan_roots = [',
    'ROOT / "scripts"',
    'ROOT / "target-xteink-x4"',
]
missing = [item for item in required if item not in text]
if missing:
    raise SystemExit('x4 partition validator repair failed: patched validator missing ' + ', '.join(missing))
print('x4-standard-partition-validator-repair-ok')
PY
