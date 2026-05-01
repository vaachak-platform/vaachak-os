#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
ARCHIVE_DIR="tools/phase-overlays"
BACKUP_DIR=".phase_backups/target-xteink-x4-src"
STAMP="$(date +%Y%m%d-%H%M%S)"

mkdir -p "$ARCHIVE_DIR" "$BACKUP_DIR"

move_path() {
  local path="$1"
  local base dest
  if [[ ! -e "$path" && ! -L "$path" ]]; then
    return 0
  fi

  base="$(basename "$path")"
  dest="$ARCHIVE_DIR/$base"
  if [[ -e "$dest" || -L "$dest" ]]; then
    dest="$ARCHIVE_DIR/${base}.${STAMP}"
  fi
  mv "$path" "$dest"
  echo "moved $path -> $dest"
}

# Old generated overlay folders should not stay in repo root after Phase 17.
move_path "phase15b_pulp_reader_overlay"
move_path "phase16_reader_parity_overlay"

# Compatibility symlink from Phase 15B is disposable.
if [[ -L "phase15b_overlay" ]]; then
  rm -f "phase15b_overlay"
  echo "removed phase15b_overlay symlink"
elif [[ -e "phase15b_overlay" ]]; then
  move_path "phase15b_overlay"
fi

# Keep the active source path clean so rg/check scripts do not find old fake EPUB code.
shopt -s nullglob
for f in \
  target-xteink-x4/src/main.rs.phase15a-backup.* \
  target-xteink-x4/src/main.rs.bak-phase* \
  target-xteink-x4/src/main.rs.Cargo.toml.bak-phase* \
  target-xteink-x4/src/*.bak-phase*
do
  if [[ -e "$f" ]]; then
    mv "$f" "$BACKUP_DIR/$(basename "$f").$STAMP"
    echo "archived $f -> $BACKUP_DIR"
  fi
done
shopt -u nullglob

# Add gitignore guardrails idempotently.
touch .gitignore
if ! rg -n '^# Vaachak OS phase generated artifacts$' .gitignore >/dev/null 2>&1; then
  cat >> .gitignore <<'EOF'

# Vaachak OS phase generated artifacts
*.bak-phase*
*.phase15a-backup.*
.phase_backups/
tools/phase-overlays/*.zip
EOF
  echo "updated .gitignore with phase artifact ignores"
else
  echo ".gitignore already has phase artifact block"
fi
