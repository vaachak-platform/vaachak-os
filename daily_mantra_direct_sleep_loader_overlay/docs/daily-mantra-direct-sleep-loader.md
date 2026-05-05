# Daily Mantra Direct Sleep Loader

This deliverable wires the active deep-sleep path to render a prepared BMP from the SD card before entering display and MCU deep sleep.

The device reads `/sleep/daily/today.txt` to choose the weekday image:

```text
tue
```

Then it tries:

```text
/sleep/daily/tue.bmp
/sleep/daily/default.bmp
/sleep.bmp
hardcoded text fallback
```

The current X4 firmware does not have a battery-backed wall-clock source. A sync/mobile/host step must update `today.txt` until a trusted on-device date source is added.

BMP constraints:

```text
800x480
1-bit monochrome
uncompressed BMP
```
