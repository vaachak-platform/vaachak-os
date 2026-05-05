# Sleep Image Timing and Mode

This deliverable adds sleep-image timing logs and an SD-card controlled mode file.

Mode file:

```text
/SLPMODE.TXT
```

Supported values:

```text
daily   - use /sleep/daily/today.txt -> /sleep/daily/<weekday>.bmp, then fallbacks
static  - use /sleep.bmp only
text    - skip bitmap lookup and render built-in text fallback
off     - skip the display update before MCU deep sleep
```

If `/SLPMODE.TXT` is missing or invalid, firmware defaults to `daily`.

Timing logs include:

```text
sleep image: mode=<mode> mode_read_ms=<n>
sleep image: bitmap resolved resolve_ms=<n>
display: sleep bitmap rendered render_ms=<n>
display: fallback sleep screen rendered render_ms=<n>
```
