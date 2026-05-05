# Sleep Bitmap Prefetch Cache

This overlay optimizes sleep bitmap rendering by reading the 800x480 1bpp BMP pixel payload into a compact 48 KB heap buffer before the e-paper refresh begins.

If allocation or read fails, rendering falls back to the older streaming SD path.

Modes are preserved:

- `daily`
- `fast-daily`
- `static`
- `cached`
- `text`
- `off` / `no-redraw`

Timing logs include:

- `bmp_prefetch_ms`
- `bmp_draw_ms`
- `bmp_decode_ms`
- `epd_refresh_ms`
