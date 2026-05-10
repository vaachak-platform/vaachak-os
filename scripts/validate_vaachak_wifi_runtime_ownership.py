#!/usr/bin/env python3
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]

REQUIRED = [
    "target-xteink-x4/src/vaachak_x4/network/mod.rs",
    "target-xteink-x4/src/vaachak_x4/network/upload.rs",
    "target-xteink-x4/src/vaachak_x4/network/network_time.rs",
    "target-xteink-x4/src/vaachak_x4/network/wifi_scan.rs",
    "target-xteink-x4/src/vaachak_x4/network/biscuit_wifi.rs",
    "target-xteink-x4/src/vaachak_x4/network/time_status.rs",
    "target-xteink-x4/src/vaachak_x4/apps/home.rs",
    "target-xteink-x4/src/vaachak_x4/apps/manager.rs",
    "target-xteink-x4/src/vaachak_x4/imported/x4_reader_runtime.rs",
    "target-xteink-x4/src/vaachak_x4/x4_apps/apps/files.rs",
    "target-xteink-x4/src/vaachak_x4/x4_apps/apps/reader/mod.rs",
    "target-xteink-x4/src/vaachak_x4/x4_apps/apps/settings.rs",
    "target-xteink-x4/src/vaachak_x4/x4_kernel/kernel/scheduler.rs",
]

errors: list[str] = []
for rel in REQUIRED:
    if not (ROOT / rel).is_file():
        errors.append(f"missing required Vaachak-owned runtime file: {rel}")

if (ROOT / "vendor/pulp-os").exists():
    errors.append("vendor/pulp-os must not remain after Vaachak runtime migration")

boot = (ROOT / "target-xteink-x4/src/vaachak_x4/imported/x4_reader_runtime.rs").read_text()
if "use crate::vaachak_x4::apps::home::HomeApp;" not in boot:
    errors.append("boot runtime is not using Vaachak-owned HomeApp")
if "use crate::vaachak_x4::apps::manager::AppManager;" not in boot:
    errors.append("boot runtime is not using Vaachak-owned AppManager")
if "VaachakWifiRuntimeOwnership::marker" not in boot:
    errors.append("boot runtime does not emit Vaachak Wi-Fi ownership marker")

manager = (ROOT / "target-xteink-x4/src/vaachak_x4/apps/manager.rs").read_text()
for name in ["upload", "network_time", "wifi_scan", "biscuit_wifi"]:
    needle = f"crate::vaachak_x4::network::{name}::"
    if needle not in manager:
        errors.append(f"special-mode dispatch does not call Vaachak network module: {name}")

home = (ROOT / "target-xteink-x4/src/vaachak_x4/apps/home.rs").read_text()
if "crate::vaachak_x4::network::{time_status, wifi_scan}" not in home:
    errors.append("Vaachak HomeApp does not use Vaachak network status/scan modules")

main_rs = (ROOT / "target-xteink-x4/src/main.rs").read_text()
if 'extern crate alloc;' not in main_rs:
    errors.append("target-xteink-x4 crate root must declare extern crate alloc")

cargo_text = (ROOT / "target-xteink-x4/Cargo.toml").read_text()
for token in ["pulp-os", "x4-kernel", "package = \"x4-os\""]:
    if token in cargo_text:
        errors.append(f"target Cargo.toml still references retired vendor runtime token: {token}")

for path in (ROOT / "target-xteink-x4/src").rglob("*.rs"):
    text = path.read_text(errors="ignore")
    rel = path.relative_to(ROOT).as_posix()
    for token in ["pulp_os::", "vendor/pulp-os", "pulp-os", "pulp_reader_runtime"]:
        if token in text:
            errors.append(f"retired vendor runtime token remains in {rel}: {token}")

partition = (ROOT / "partitions/xteink_x4_standard.csv").read_text()
required_partition_tokens = [
    "app0,       app,  ota_0,   0x10000,  0x640000,",
    "app1,       app,  ota_1,   0x650000, 0x640000,",
    "spiffs,     data, spiffs,  0xc90000, 0x360000,",
    "coredump,   data, coredump,0xff0000, 0x10000,",
]
for token in required_partition_tokens:
    if token not in partition:
        errors.append(f"partition table no longer matches accepted X4 layout: {token}")

for pattern in ["*.zip", "*.bak", "*.bak.*"]:
    for path in ROOT.rglob(pattern):
        rel = path.relative_to(ROOT).as_posix()
        if "/.git/" in f"/{rel}/" or rel.startswith("target/"):
            continue
        if path.is_file():
            errors.append(f"stale generated/backup artifact remains: {rel}")

if errors:
    for e in errors:
        print(f"ERROR: {e}")
    sys.exit(1)

print("vaachak-wifi-runtime-ownership-ok")
