#!/usr/bin/env bash
set -euo pipefail

if [ -d vendor/pulp-os ]; then
  echo "ERROR: vendor/pulp-os remains" >&2
  exit 1
fi

if rg -n "pulp_os::|pulp-os|vendor/pulp-os|package = \"x4-os\"|x4-kernel =" Cargo.toml target-xteink-x4/Cargo.toml target-xteink-x4/src; then
  echo "ERROR: retired Pulp runtime dependency reference remains" >&2
  exit 1
fi

echo "vaachak-runtime-vendor-retirement-audit-ok"
