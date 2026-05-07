# USB Serial SD Bulk Transfer

Use this for large prepared cache folders when Wi-Fi Transfer is too slow or unreliable.

## Current status

```text
- Host tool exists.
- Device receiver state-machine skeleton exists.
- Runtime wiring is intentionally deferred to the next integration step.
```

## Dry run

```bash
python3 tools/usb_transfer/send_folder.py \
  --source /tmp/FCACHE/15D1296A \
  --target /FCACHE/15D1296A \
  --dry-run \
  --manifest /tmp/usb-transfer-manifest.json
```

## Future serial transfer

```bash
python3 tools/usb_transfer/send_folder.py \
  --port /dev/cu.usbmodemXXXX \
  --source /tmp/FCACHE/15D1296A \
  --target /FCACHE/15D1296A
```

## Typical FCACHE use case

```text
Host:
  /tmp/FCACHE/15D1296A/
    META.TXT
    FONTS.IDX
    PAGES.IDX
    LAT18.VFN
    DEV22.VFN
    P000.VRN
    ...

SD target:
  /FCACHE/15D1296A/
```

## Why this exists

Browser Wi-Fi upload can fail for large prepared caches with many page files. USB serial transfer gives us:

```text
- deterministic framing
- CRC32 checks
- retry/resume hooks
- direct SD write path
- no dependency on macOS mounting the SD card
```


## Device runtime integration

This deliverable adds the compiled device-side runtime layer:

```text
vendor/pulp-os/src/apps/usb_transfer/
  mod.rs
  receiver_skeleton.rs
  runtime.rs
```

The runtime layer handles:

```text
- HELLO
- MKDIR
- BEGIN
- CHUNK
- END
- DONE
- ABORT
- path validation
- chunk CRC validation
- file-level CRC validation
- progress tracking
```

The final hardware binding is still a separate board/runtime step:

```text
ESP32-C3 USB_SERIAL_JTAG bytes
  -> frame accumulator
  -> UsbTransferRuntime::accept_raw_frame()
  -> SdTransferTarget implementation
  -> SD card writes
```

Do not use serial mode for real transfer until the board-specific byte-stream binding is enabled.


## X4 App + Byte Stream Binding Scaffold

This deliverable adds:

```text
Apps > Tools > USB Transfer
```

and compiled modules:

```text
vendor/pulp-os/src/apps/usb_transfer/
  binding.rs
  mod.rs
  receiver_skeleton.rs
  runtime.rs
  screen.rs
```

Current behavior:

```text
- User can open Tools > USB Transfer.
- Screen explains that host dry-run is ready.
- Protocol/runtime modules compile.
- ESP32-C3 USB_SERIAL_JTAG ownership is not taken yet.
```

The board-specific byte binding is still next:

```text
USB_SERIAL_JTAG RX bytes
  -> frame accumulator
  -> UsbTransferRuntime::accept_raw_frame()
  -> SdTransferTarget implementation
  -> SD writes
  -> OK/ERR response frames
```


## Real X4 USB_SERIAL_JTAG binding

This deliverable wires:

```text
Tools > USB Transfer
  -> AppId::UsbTransfer special mode
  -> esp_hal::usb_serial_jtag::UsbSerialJtag
  -> framed VUSB1 protocol
  -> SD writes under /FCACHE/<BOOKID>
```

Usage:

```bash
python3 tools/usb_transfer/send_folder.py \
  --port /dev/cu.usbmodemXXXX \
  --source /tmp/FCACHE/15D1296A \
  --target /FCACHE/15D1296A
```

Important:

```text
Do not run espflash monitor during transfer.
The USB Serial/JTAG CDC stream is used by the host transfer protocol.
```


## Status: experimental and hidden

USB bulk transfer was tested as a custom USB Serial/JTAG protocol. It is not exposed in the X4 Tools menu now.

Use Wi-Fi Transfer v2 for large prepared cache uploads:

```text
Apps > Network > Wi-Fi Transfer
Browser page: X4 Wi-Fi Transfer v2
```

Reason:

```text
- X4/ESP32-C3 does not support normal USB mass-storage file transfer.
- The custom USB serial path conflicts with monitor/console use and is still experimental.
- Chunked Wi-Fi upload with resume is the supported large-transfer path.
```
