#!/usr/bin/env python3
"""
USB Serial SD Bulk Transfer host tool.

This is the host-side sender for the Vaachak/X4 USB transfer protocol.

Current status:
- Supports dry-run manifest generation.
- Supports serial framing if pyserial is installed.
- Sends folders/files as path-safe framed records.
- Device receiver integration is intentionally separate and staged.

Example:

python3 tools/usb_transfer/send_folder.py \
  --port /dev/cu.usbmodemXXXX \
  --source /tmp/FCACHE/15D1296A \
  --target /FCACHE/15D1296A

Dry run:

python3 tools/usb_transfer/send_folder.py \
  --source /tmp/FCACHE/15D1296A \
  --target /FCACHE/15D1296A \
  --dry-run
"""

from __future__ import annotations

import argparse
import binascii
import json
import os
import struct
import sys
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable


MAGIC = b"VUSB1"
DEFAULT_BAUD = 921600
DEFAULT_CHUNK_SIZE = 2048
MAX_TARGET_DEPTH = 4

FRAME_HELLO = 1
FRAME_MKDIR = 2
FRAME_BEGIN = 3
FRAME_CHUNK = 4
FRAME_END = 5
FRAME_DONE = 6
FRAME_ABORT = 7

ACK_OK = b"OK\n"
ACK_ERR = b"ERR\n"


@dataclass(frozen=True)
class TransferFile:
    source: Path
    target: str
    size: int
    crc32: int


def normalize_target_path(path: str) -> str:
    path = path.strip().replace("\\", "/")

    if not path:
        raise ValueError("target path is empty")

    if not path.startswith("/"):
        path = "/" + path

    while "//" in path:
        path = path.replace("//", "/")

    if len(path) > 1 and path.endswith("/"):
        path = path.rstrip("/")

    parts = [part for part in path.split("/") if part]

    if len(parts) > MAX_TARGET_DEPTH:
        raise ValueError(f"target path too deep: {path}")

    for part in parts:
        validate_path_component(part)

    return "/" + "/".join(parts) if parts else "/"


def validate_path_component(part: str) -> None:
    if not part:
        raise ValueError("empty path component")

    if part in {".", ".."}:
        raise ValueError(f"invalid path component: {part}")

    if "/" in part or "\\" in part or ":" in part:
        raise ValueError(f"path separators are not allowed in component: {part}")

    if len(part.encode("utf-8")) > 32:
        raise ValueError(f"path component too long: {part}")


def iter_transfer_files(source: Path, target: str) -> list[TransferFile]:
    source = source.resolve()
    target = normalize_target_path(target)

    if not source.exists():
        raise FileNotFoundError(source)

    files: list[TransferFile] = []

    if source.is_file():
        file_target = target
        if target.endswith("/") or target == "/":
            file_target = normalize_target_path(target + "/" + source.name)
        files.append(build_transfer_file(source, file_target))
        return files

    if not source.is_dir():
        raise ValueError(f"not a regular file or directory: {source}")

    for path in sorted(source.rglob("*")):
        if not path.is_file():
            continue

        rel = path.relative_to(source)
        rel_parts = [part for part in rel.parts]
        for part in rel_parts:
            validate_path_component(part)

        file_target = normalize_target_path(target + "/" + "/".join(rel_parts))
        files.append(build_transfer_file(path, file_target))

    return files


def build_transfer_file(source: Path, target: str) -> TransferFile:
    data_crc = 0
    size = 0

    with source.open("rb") as fh:
        while True:
            chunk = fh.read(1024 * 1024)
            if not chunk:
                break
            size += len(chunk)
            data_crc = binascii.crc32(chunk, data_crc)

    return TransferFile(
        source=source,
        target=normalize_target_path(target),
        size=size,
        crc32=data_crc & 0xFFFFFFFF,
    )


def parent_dirs_for(files: Iterable[TransferFile]) -> list[str]:
    dirs: set[str] = set()

    for item in files:
        parts = [part for part in item.target.split("/") if part]
        for i in range(1, len(parts)):
            dirs.add("/" + "/".join(parts[:i]))

    return sorted(dirs)


def manifest(files: list[TransferFile]) -> dict:
    return {
        "version": 1,
        "files": [
            {
                "source": str(item.source),
                "target": item.target,
                "size": item.size,
                "crc32": f"{item.crc32:08X}",
            }
            for item in files
        ],
    }


def frame(frame_type: int, payload: bytes = b"") -> bytes:
    header = MAGIC + struct.pack("<BI", frame_type, len(payload))
    crc = binascii.crc32(header + payload) & 0xFFFFFFFF
    return header + payload + struct.pack("<I", crc)


def json_payload(obj: dict) -> bytes:
    return json.dumps(obj, sort_keys=True, separators=(",", ":")).encode("utf-8")


def open_serial(port: str, baud: int):
    try:
        import serial  # type: ignore
    except ImportError as exc:
        raise SystemExit(
            "pyserial is required for serial mode. Install with: python3 -m pip install pyserial"
        ) from exc

    return serial.Serial(port=port, baudrate=baud, timeout=5, write_timeout=10)


def write_frame(ser, frame_type: int, payload: bytes = b"", wait_ack: bool = True) -> None:
    ser.write(frame(frame_type, payload))
    ser.flush()

    if not wait_ack:
        return

    ack = ser.readline()
    if ack != ACK_OK:
        raise RuntimeError(f"device rejected frame: {ack!r}")


def send_file(ser, item: TransferFile, chunk_size: int) -> None:
    begin = {
        "path": item.target,
        "size": item.size,
        "crc32": f"{item.crc32:08X}",
    }
    write_frame(ser, FRAME_BEGIN, json_payload(begin))

    offset = 0
    with item.source.open("rb") as fh:
        while True:
            data = fh.read(chunk_size)
            if not data:
                break

            chunk_crc = binascii.crc32(data) & 0xFFFFFFFF
            chunk_header = {
                "path": item.target,
                "offset": offset,
                "length": len(data),
                "crc32": f"{chunk_crc:08X}",
            }
            payload = json_payload(chunk_header) + b"\n" + data
            write_frame(ser, FRAME_CHUNK, payload)

            offset += len(data)

    write_frame(ser, FRAME_END, json_payload(begin))


def send(
    files: list[TransferFile],
    port: str,
    baud: int,
    chunk_size: int,
    skip_mkdir: bool = False,
) -> None:
    print(f"opening serial port {port} at {baud}")
    print("note: Device runtime byte-stream binding must be enabled on X4 before serial mode will complete")
    with open_serial(port, baud) as ser:
        time.sleep(0.5)

        write_frame(
            ser,
            FRAME_HELLO,
            json_payload(
                {
                    "tool": "vaachak-usb-transfer",
                    "version": 1,
                    "file_count": len(files),
                }
            ),
        )

        if skip_mkdir:
            print("skipping MKDIR frames; assuming target directories already exist")
        else:
            for directory in parent_dirs_for(files):
                print(f"mkdir {directory}")
                try:
                    write_frame(ser, FRAME_MKDIR, json_payload({"path": directory}))
                except RuntimeError as exc:
                    print(f"warning: MKDIR rejected by device, continuing: {exc}")

        for idx, item in enumerate(files, start=1):
            print(f"[{idx}/{len(files)}] {item.source} -> {item.target} ({item.size} bytes)")
            send_file(ser, item, chunk_size)

        write_frame(ser, FRAME_DONE, json_payload({"file_count": len(files)}), wait_ack=True)


def run() -> int:
    parser = argparse.ArgumentParser(description="Vaachak/X4 USB Serial SD Bulk Transfer")
    parser.add_argument("--source", required=True, type=Path, help="File or folder to send")
    parser.add_argument("--target", required=True, help="Target path on SD, e.g. /FCACHE/15D1296A")
    parser.add_argument("--port", help="Serial port, e.g. /dev/cu.usbmodemXXXX")
    parser.add_argument("--baud", type=int, default=DEFAULT_BAUD)
    parser.add_argument("--chunk-size", type=int, default=DEFAULT_CHUNK_SIZE)
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument(
        "--skip-mkdir",
        action="store_true",
        help="Do not send MKDIR frames; use when target directories already exist",
    )
    parser.add_argument("--manifest", type=Path, help="Write transfer manifest JSON")
    args = parser.parse_args()

    files = iter_transfer_files(args.source, args.target)

    if not files:
        print("no files to transfer")
        return 1

    data = manifest(files)

    print(f"files={len(files)}")
    print(f"directories={len(parent_dirs_for(files))}")
    print(f"bytes={sum(item.size for item in files)}")

    for item in files[:20]:
        print(f"  {item.source} -> {item.target} size={item.size} crc32={item.crc32:08X}")

    if len(files) > 20:
        print(f"  ... {len(files) - 20} more files")

    if args.manifest:
        args.manifest.parent.mkdir(parents=True, exist_ok=True)
        args.manifest.write_text(json.dumps(data, indent=2, sort_keys=True), encoding="utf-8")
        print(f"manifest={args.manifest}")

    if args.dry_run:
        print("dry-run complete")
        return 0

    if not args.port:
        print("ERROR: --port is required unless --dry-run is used", file=sys.stderr)
        return 2

    if args.chunk_size < 128 or args.chunk_size > 8192:
        print("ERROR: --chunk-size must be between 128 and 8192", file=sys.stderr)
        return 2

    send(files, args.port, args.baud, args.chunk_size, skip_mkdir=args.skip_mkdir)
    print("transfer complete")
    return 0


if __name__ == "__main__":
    raise SystemExit(run())
