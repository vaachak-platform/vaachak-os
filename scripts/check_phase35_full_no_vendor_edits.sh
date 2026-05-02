#!/usr/bin/env bash
set -euo pipefail

if git diff --quiet -- vendor/pulp-os vendor/smol-epub; then
  echo "OK   vendor/pulp-os and vendor/smol-epub have no tracked edits"
else
  echo "FAIL vendor/pulp-os or vendor/smol-epub has tracked edits"
  git diff --stat -- vendor/pulp-os vendor/smol-epub || true
  exit 1
fi
