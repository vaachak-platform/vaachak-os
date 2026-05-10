#!/usr/bin/env bash
set -euo pipefail

if [ -d vendor/pulp-os ]; then
  echo "ERROR: vendor/pulp-os remains active after runtime retirement" >&2
  exit 1
fi

if rg -n 'use crate::fonts;|crate::fonts::bitmap' target-xteink-x4/src target-xteink-x4/build.rs; then
  echo "ERROR: migrated app/font code still uses old crate-root fonts path" >&2
  exit 1
fi

if ! rg -q 'smol-epub = \{ path = "\.\./vendor/smol-epub", features = \["async"\] \}' target-xteink-x4/Cargo.toml; then
  echo "ERROR: smol-epub async feature is not enabled for migrated EPUB reader" >&2
  exit 1
fi

if rg -n 'pulp_os::|pulp-os|vendor/pulp-os|package = "x4-os"|x4-kernel =' Cargo.toml target-xteink-x4/Cargo.toml target-xteink-x4/src; then
  echo "ERROR: retired Pulp runtime dependency reference remains" >&2
  exit 1
fi

echo "vaachak-runtime-vendor-retirement-build-wiring-ok"
