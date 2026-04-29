# VaachakOS Bootstrap Phase 4.4 — Serial Direct Boot Smoke

Phase 4.3/4.2 produced a valid ESP32-C3 image and flashed it, but the monitor did
not show VaachakOS application logs after the ESP-IDF bootloader messages.

This phase switches the embedded boot-smoke path from the `log` facade to direct
`esp_println::println!` output.

## Important display note

The Xteink X4 ePaper display keeps its last image without power. Phase 4 does
not initialize or refresh the SSD1677 display. Seeing the old Home screen from
`x4-reader-os-rs` is expected and does not mean the old firmware is still
running.

The Phase 4.4 acceptance signal is serial output only.

## Expected serial markers

```text
VaachakOS X4 boot smoke starting
phase=bootstrap-phase4-x4-target-boot-smoke
note=display is intentionally not initialized in Phase 4
heap=16K boot-smoke only
display logical=480x800 native=800x480 rot=Deg270 strip_rows=40
bus shared_sd_epd=true probe=400kHz runtime=20MHz
storage state=Probed card_bytes=Some(...)
power battery_mv=4100 pct=...
VaachakOS X4 boot smoke complete
phase4.4=serial-direct-print-ok
```

## Out of scope

- SSD1677 display initialization
- SD/FAT real storage adapter
- Home/Files/Reader runtime
- real ADC sampling
- real battery/charge GPIO wiring
