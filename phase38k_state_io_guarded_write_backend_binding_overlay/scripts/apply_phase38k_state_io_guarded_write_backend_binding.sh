#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPO="$(pwd)"

TARGET_DIR="$REPO/target-xteink-x4/src/vaachak_x4/runtime"
TARGET_FILE="$TARGET_DIR/state_io_guarded_write_backend_binding.rs"
RUNTIME_MOD="$REPO/target-xteink-x4/src/vaachak_x4/runtime.rs"
DIR_CACHE="$REPO/vendor/pulp-os/kernel/src/kernel/dir_cache.rs"
FILES_APP="$REPO/vendor/pulp-os/src/apps/files.rs"

mkdir -p "$TARGET_DIR"
cp -v "$ROOT/replaceable/target-xteink-x4/src/vaachak_x4/runtime/state_io_guarded_write_backend_binding.rs" "$TARGET_FILE"

if ! grep -q '^pub mod state_io_guarded_write_backend_binding;' "$RUNTIME_MOD"; then
  printf '\npub mod state_io_guarded_write_backend_binding;\n' >> "$RUNTIME_MOD"
  echo "added state_io_guarded_write_backend_binding export to $RUNTIME_MOD"
else
  echo "state_io_guarded_write_backend_binding export already present in $RUNTIME_MOD"
fi

python3 - "$DIR_CACHE" "$FILES_APP" <<'PYFIX'
from pathlib import Path
import re
import sys

HELPER = '''fn phase38i_is_epub_or_epu_name(name: &[u8]) -> bool {
    if name.len() >= 5
        && name[name.len() - 5] == b'.'
        && name[name.len() - 4..].eq_ignore_ascii_case(b"EPUB")
    {
        return true;
    }

    name.len() >= 4
        && name[name.len() - 4] == b'.'
        && name[name.len() - 3..].eq_ignore_ascii_case(b"EPU")
}
'''

OLD_PRED = re.compile(
    r"name\.len\(\)\s*>=\s*5\s*&&\s*name\[name\.len\(\)\s*-\s*5\]\s*==\s*b'.'\s*&&\s*name\[name\.len\(\)\s*-\s*4\.\.\]\.eq_ignore_ascii_case\(b\"EPUB\"\)",
    re.MULTILINE,
)
HELPER_RE = re.compile(
    r"fn\s+phase38i_is_epub_or_epu_name\s*\(\s*name\s*:\s*&\[u8\]\s*\)\s*->\s*bool\s*\{.*?\n\}",
    re.DOTALL,
)

for raw in sys.argv[1:]:
    path = Path(raw)
    if not path.exists():
        continue
    text = path.read_text()
    original = text

    text = text.replace('}\\n\\nfn phase38i_is_epub_or_epu_name', '}\n\nfn phase38i_is_epub_or_epu_name')
    text = text.replace('}\\n\\nfn scan_one_epub_title', '}\n\nfn scan_one_epub_title')
    text = text.replace('}\\n\\nimpl DirCache', '}\n\nimpl DirCache')

    if 'fn phase38i_is_epub_or_epu_name' in text:
        text = HELPER_RE.sub(HELPER.rstrip(), text, count=1)
    else:
        marker = 'impl DirCache {' if path.name == 'dir_cache.rs' else 'fn scan_one_epub_title'
        pos = text.find(marker)
        if pos >= 0:
            text = text[:pos] + HELPER + '\n' + text[pos:]

    text, replacements = OLD_PRED.subn('phase38i_is_epub_or_epu_name(name)', text)

    if text != original:
        path.write_text(text)
        print(f"repaired {path}: predicate replacements={replacements}")
    else:
        print(f"no phase38i repair needed for {path}")
PYFIX

"$ROOT/scripts/check_phase38k_state_io_guarded_write_backend_binding.sh"

echo "phase38k=x4-state-io-guarded-write-backend-binding-ok"
echo "Phase 38K overlay applied. Next recommended checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
