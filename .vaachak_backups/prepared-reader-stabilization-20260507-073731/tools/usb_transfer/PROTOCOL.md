# USB Serial SD Bulk Transfer Protocol

This is the staged replacement for large browser uploads.

## Goals

```text
- Transfer large /FCACHE/<BOOKID> folders over USB cable
- Avoid browser upload timeouts
- Avoid relying on macOS mounting the SD card
- Keep SD path traversal protections
- Verify file integrity with CRC32
```

## Host command

```bash
python3 tools/usb_transfer/send_folder.py \
  --port /dev/cu.usbmodemXXXX \
  --source /tmp/FCACHE/15D1296A \
  --target /FCACHE/15D1296A
```

Dry run:

```bash
python3 tools/usb_transfer/send_folder.py \
  --source /tmp/FCACHE/15D1296A \
  --target /FCACHE/15D1296A \
  --dry-run \
  --manifest /tmp/usb-transfer-manifest.json
```

## Frame format

```text
magic:   5 bytes  "VUSB1"
type:    u8
length:  u32 little-endian
payload: length bytes
crc32:   u32 little-endian over magic+type+length+payload
```

## Frame types

```text
1 HELLO
2 MKDIR
3 BEGIN
4 CHUNK
5 END
6 DONE
7 ABORT
```

## ACK

Device replies with:

```text
OK\n
```

or:

```text
ERR\n
```

## Path rules

Allowed:

```text
/
/FCACHE
/FCACHE/15D1296A
/FCACHE/15D1296A/P000.VRN
```

Rejected:

```text
../
./
folder/../../x
backslash
colon
empty path component
too many path components
```

## Staging note

The host tool is usable for dry-run and framing immediately. The device-side receiver is currently a skeleton for the next firmware integration step.
