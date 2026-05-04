#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
DIR_CACHE="$ROOT/vendor/pulp-os/kernel/src/kernel/dir_cache.rs"
FILES_RS="$ROOT/vendor/pulp-os/src/apps/files.rs"

for f in "$DIR_CACHE" "$FILES_RS"; do
  if [ ! -f "$f" ]; then
    echo "missing required file: $f" >&2
    exit 1
  fi
done

python3 - "$DIR_CACHE" "$FILES_RS" <<'PY'
from pathlib import Path
import sys

replacement = """fn phase38i_is_epub_or_epu_name(name: &[u8]) -> bool {
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
"""

def replace_function(src: str, fn_name: str, fallback_anchor: str) -> tuple[str, bool]:
    needle = f"fn {fn_name}("
    start = src.find(needle)
    if start == -1:
        anchor = src.find(fallback_anchor)
        if anchor == -1:
            raise RuntimeError(f"could not find {needle!r} or fallback anchor {fallback_anchor!r}")
        return src[:anchor] + replacement + "\n" + src[anchor:], True

    brace = src.find("{", start)
    if brace == -1:
        raise RuntimeError(f"could not find opening brace for {needle!r}")

    depth = 0
    end = None
    for i in range(brace, len(src)):
        ch = src[i]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                end = i + 1
                break
    if end is None:
        raise RuntimeError(f"could not find closing brace for {needle!r}")

    return src[:start] + replacement + src[end:], False

paths = [Path(sys.argv[1]), Path(sys.argv[2])]
anchors = ["impl DirCache {", "fn scan_one_epub_title("]

for path, anchor in zip(paths, anchors):
    src = path.read_text()
    new_src, inserted = replace_function(src, "phase38i_is_epub_or_epu_name", anchor)
    if "if phase38i_is_epub_or_epu_name(name)" in new_src:
        raise RuntimeError(f"recursive helper call still present in {path}")
    if new_src != src:
        path.write_text(new_src)
        action = "inserted" if inserted else "replaced"
        print(f"{action} fixed phase38i helper in {path}")
    else:
        print(f"phase38i helper already fixed in {path}")
PY

if rg -n 'if phase38i_is_epub_or_epu_name\(name\)' \
  vendor/pulp-os/kernel/src/kernel/dir_cache.rs \
  vendor/pulp-os/src/apps/files.rs >/tmp/phase38i-recursion-left.txt; then
  cat /tmp/phase38i-recursion-left.txt >&2
  echo "recursive helper call still present" >&2
  exit 1
fi

echo "phase38k-phase38i-repair=x4-epu-helper-recursion-fixed"
echo "Next checks:"
echo "  cargo fmt --all"
echo "  cargo test -p vaachak-core --all-targets"
echo "  cargo check -p hal-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo check -p target-xteink-x4 --target riscv32imc-unknown-none-elf"
echo "  cargo clippy -p hal-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
echo "  cargo clippy -p target-xteink-x4 --target riscv32imc-unknown-none-elf -- -D warnings"
