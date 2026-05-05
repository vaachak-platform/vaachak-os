#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: scripts/verify_daily_mantra_sd_assets.sh /Volumes/SDCARD" >&2
  exit 2
fi

sd_root="$1"
asset_dir="$sd_root/sleep/daily"

python3 - "$asset_dir" <<'PY'
from pathlib import Path
import struct
import sys

asset_dir = Path(sys.argv[1])
expected = ["mon", "tue", "wed", "thu", "fri", "sat", "sun", "default"]
errors = []

if not asset_dir.exists():
    errors.append(f"missing directory: {asset_dir}")
else:
    for key in expected:
        path = asset_dir / f"{key}.bmp"
        if not path.exists():
            errors.append(f"missing {path}")
            continue
        data = path.read_bytes()[:30]
        if len(data) < 30 or data[:2] != b"BM":
            errors.append(f"not a BMP file: {path}")
            continue
        width, height = struct.unpack_from("<ii", data, 18)
        if (width, abs(height)) != (800, 480):
            errors.append(f"unexpected dimensions for {path}: {width}x{height}")

manifest = asset_dir / "manifest.tsv"
if not manifest.exists():
    errors.append(f"missing {manifest}")

if errors:
    for error in errors:
        print(f"error: {error}", file=sys.stderr)
    raise SystemExit(1)

print(f"Daily mantra sleep assets verified in {asset_dir}")
PY
