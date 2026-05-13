#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "repository hygiene failed: $*" >&2
  exit 1
}

[ -f Cargo.toml ] || fail "missing Cargo.toml"
[ -f target-xteink-x4/Cargo.toml ] || fail "missing target-xteink-x4/Cargo.toml"
[ -f target-xteink-x4/src/vaachak_x4/apps/home.rs ] || fail "missing Vaachak X4 Home app"
[ -f target-xteink-x4/src/vaachak_x4/x4_apps/apps/reader/mod.rs ] || fail "missing Vaachak X4 Reader app"
[ -f target-xteink-x4/src/vaachak_x4/network/mod.rs ] || fail "missing Vaachak X4 network module"

if find . -maxdepth 1 \
  \( -name '*.zip' \
     -o -name '*_repair' \
     -o -name '*_restore' \
     -o -name '*_cleanup' \
     -o -name '*_contract' \
     -o -name '*_reset' \
     -o -name '*_overlay' \
  \) -print | grep -q .; then
  find . -maxdepth 1 \
    \( -name '*.zip' \
       -o -name '*_repair' \
       -o -name '*_restore' \
       -o -name '*_cleanup' \
       -o -name '*_contract' \
       -o -name '*_reset' \
       -o -name '*_overlay' \
    \) -print >&2
  fail "generated root patch/deliverable artifacts remain"
fi

if find scripts -maxdepth 1 -type f \
  \( -name 'patch_*' -o -name 'apply_*' -o -name 'cleanup_*' \) -print | grep -q .; then
  find scripts -maxdepth 1 -type f \
    \( -name 'patch_*' -o -name 'apply_*' -o -name 'cleanup_*' \) -print >&2
  fail "generated patch/apply/cleanup scripts remain"
fi

if find scripts -maxdepth 1 -type f \
  \( -name 'validate_*_repair*' \
     -o -name 'validate_*_cleanup*' \
     -o -name 'validate_*_restore*' \
     -o -name 'validate_*_pack*' \
     -o -name 'validate_*_slice*' \
  \) -print | grep -q .; then
  find scripts -maxdepth 1 -type f \
    \( -name 'validate_*_repair*' \
       -o -name 'validate_*_cleanup*' \
       -o -name 'validate_*_restore*' \
       -o -name 'validate_*_pack*' \
       -o -name 'validate_*_slice*' \
    \) -print >&2
  fail "one-off repair/cleanup/feature-slice validators remain"
fi

if find . \
  \( -path './.git' -o -path './target' -o -path './vendor' \) -prune -o \
  \( -name '__pycache__' -o -name '*.pyc' -o -name '__MACOSX' -o -name '.DS_Store' \) -print | grep -q .; then
  find . \
    \( -path './.git' -o -path './target' -o -path './vendor' \) -prune -o \
    \( -name '__pycache__' -o -name '*.pyc' -o -name '__MACOSX' -o -name '.DS_Store' \) -print >&2
  fail "generated cache or OS metadata remains"
fi

if rg -n 'pulp_os::|package = "x4-os"|x4-kernel =' Cargo.toml target-xteink-x4/Cargo.toml target-xteink-x4/src >/tmp/vaachak_active_pulp_refs.txt; then
  cat /tmp/vaachak_active_pulp_refs.txt >&2
  fail "active old Pulp package references remain"
fi

if rg -n 'phase[[:space:]_-]*[0-9]|Phase[[:space:]_-]*[0-9]' \
  README.md SCOPE.md ROADMAP.md docs/architecture docs/development docs/operations \
  --glob '*.md' >/tmp/vaachak_phase_doc_refs.txt; then
  cat /tmp/vaachak_phase_doc_refs.txt >&2
  fail "phase-numbered delivery references remain in current-state docs"
fi

echo "repository_hygiene=ok"
