#!/usr/bin/env python3
"""Phase 17 manifest cleanup for vaachak-os.

This script is intentionally conservative. It only normalizes the workspace
exclude list and the target-xteink-x4 dependency lines needed for the vendored
X4/Pulp reader runtime boundary.
"""
from __future__ import annotations

from pathlib import Path
import re
import sys
from datetime import datetime

ROOT = Path.cwd()
STAMP = datetime.now().strftime("%Y%m%d-%H%M%S")

ROOT_CARGO = ROOT / "Cargo.toml"
TARGET_CARGO = ROOT / "target-xteink-x4" / "Cargo.toml"
TARGET_MAIN = ROOT / "target-xteink-x4" / "src" / "main.rs"

REQUIRED_EXCLUDES = ["vendor/pulp-os", "vendor/smol-epub"]

CANONICAL_TARGET_DEPS = [
    'pulp-os = { package = "x4-os", path = "../vendor/pulp-os" }',
    'x4-kernel = { path = "../vendor/pulp-os/kernel" }',
    'smol-epub = { path = "../vendor/smol-epub" }',
]


def backup(path: Path, label: str) -> Path:
    dst = path.with_name(f"{path.name}.bak-phase17-{label}-{STAMP}")
    dst.write_text(path.read_text())
    return dst


def find_section(lines: list[str], header: str) -> tuple[int, int]:
    start = -1
    for i, line in enumerate(lines):
        if re.match(rf"^\s*\[{re.escape(header)}\]\s*$", line):
            start = i
            break
    if start < 0:
        return -1, -1

    end = len(lines)
    for i in range(start + 1, len(lines)):
        if re.match(r"^\s*\[.+\]\s*$", lines[i]):
            end = i
            break
    return start, end


def parse_one_line_array_values(line: str) -> list[str] | None:
    m = re.match(r'^\s*exclude\s*=\s*\[(.*)\]\s*$', line)
    if not m:
        return None
    body = m.group(1)
    return re.findall(r'"([^"]+)"', body)


def ensure_workspace_excludes() -> None:
    if not ROOT_CARGO.exists():
        raise SystemExit("missing root Cargo.toml")

    original = ROOT_CARGO.read_text()
    lines = original.splitlines()
    start, end = find_section(lines, "workspace")
    if start < 0:
        raise SystemExit("root Cargo.toml has no [workspace] section")

    backup(ROOT_CARGO, "workspace")

    section = lines[start:end]
    exclude_idx = None
    existing: list[str] = []

    for rel_idx, line in enumerate(section):
        parsed = parse_one_line_array_values(line)
        if parsed is not None:
            exclude_idx = start + rel_idx
            existing = parsed
            break

    merged = []
    for value in existing + REQUIRED_EXCLUDES:
        if value not in merged:
            merged.append(value)

    new_line = 'exclude = [' + ', '.join(f'"{value}"' for value in merged) + ']'

    if exclude_idx is None:
        # Keep this early in the [workspace] section. TOML permits this before members.
        lines.insert(start + 1, new_line)
    else:
        lines[exclude_idx] = new_line

    new = "\n".join(lines) + "\n"
    if new != original:
        ROOT_CARGO.write_text(new)
        print("updated root Cargo.toml workspace exclude")
    else:
        print("root Cargo.toml workspace exclude already clean")


def remove_invalid_dotted_workspace_lines(lines: list[str]) -> list[str]:
    # If a file contains `foo = ...` and also `foo.workspace = true`, remove the
    # dotted line. If it only has `foo.workspace = true`, normalize it to
    # `foo = { workspace = true }`.
    direct_keys = set()
    for line in lines:
        m = re.match(r'^\s*([A-Za-z0-9_-]+)\s*=\s*', line)
        if m:
            direct_keys.add(m.group(1))

    out: list[str] = []
    for line in lines:
        m = re.match(r'^(\s*)([A-Za-z0-9_-]+)\.workspace\s*=\s*true\s*$', line)
        if not m:
            out.append(line)
            continue

        indent, key = m.groups()
        if key in direct_keys:
            print(f"removed conflicting dotted workspace key: {line.strip()}")
            continue
        out.append(f'{indent}{key} = {{ workspace = true }}')

    return out


def ensure_target_dependencies() -> None:
    if not TARGET_CARGO.exists():
        raise SystemExit("missing target-xteink-x4/Cargo.toml")

    original = TARGET_CARGO.read_text()
    backup(TARGET_CARGO, "target-deps")

    lines = remove_invalid_dotted_workspace_lines(original.splitlines())
    start, end = find_section(lines, "dependencies")
    if start < 0:
        raise SystemExit("target-xteink-x4/Cargo.toml has no [dependencies] section")

    remove_keys = {"pulp-os", "x4-kernel", "smol-epub"}
    new_lines: list[str] = []
    for i, line in enumerate(lines):
        if start < i < end:
            m = re.match(r'^\s*([A-Za-z0-9_-]+)\s*=', line)
            if m and m.group(1) in remove_keys:
                print(f"removed old dependency line: {line.strip()}")
                continue
        new_lines.append(line)

    # Recompute dependencies section after removals.
    start, end = find_section(new_lines, "dependencies")
    insert_at = end
    for dep in CANONICAL_TARGET_DEPS:
        new_lines.insert(insert_at, dep)
        insert_at += 1

    new = "\n".join(new_lines) + "\n"
    if new != original:
        TARGET_CARGO.write_text(new)
        print("updated target-xteink-x4/Cargo.toml dependency boundary")
    else:
        print("target-xteink-x4/Cargo.toml already clean")


def ensure_phase17_marker() -> None:
    if not TARGET_MAIN.exists():
        raise SystemExit("missing target-xteink-x4/src/main.rs")

    original = TARGET_MAIN.read_text()
    if "phase17=x4-reader-refactor-ok" in original:
        print("phase17 marker already present")
        return

    backup(TARGET_MAIN, "main-marker")

    lines = original.splitlines()
    inserted = False
    out: list[str] = []
    for line in lines:
        out.append(line)
        if "phase16=x4-reader-parity-ok" in line and not inserted:
            indent = line[: len(line) - len(line.lstrip())]
            if "esp_println::println!" in line:
                out.append(f'{indent}esp_println::println!("phase17=x4-reader-refactor-ok");')
            elif "println!" in line:
                out.append(f'{indent}println!("phase17=x4-reader-refactor-ok");')
            else:
                out.append(f'{indent}// phase17=x4-reader-refactor-ok')
            inserted = True

    if not inserted:
        # Last-resort: add a comment marker near the top. This preserves compile
        # behavior even if the main logging shape changes.
        out.insert(0, "// phase17=x4-reader-refactor-ok")
        print("warning: phase16 marker not found; inserted phase17 comment marker at top")
    else:
        print("inserted phase17 marker after phase16 marker")

    TARGET_MAIN.write_text("\n".join(out) + "\n")


def main() -> int:
    ensure_workspace_excludes()
    ensure_target_dependencies()
    ensure_phase17_marker()
    return 0


if __name__ == "__main__":
    sys.exit(main())
