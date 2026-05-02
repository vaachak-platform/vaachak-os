# Phase 35F-0 - Display Geometry Runtime Facade

Phase 35F-0 adds a Vaachak-owned runtime facade for display geometry.

It validates:

- native SSD1677 panel bounds: `800x480`
- logical portrait bounds: `480x800`
- 270 degree logical-to-native rectangle mapping
- native strip count and strip window geometry
- reader page/text bounds used by the working runtime

The active runtime calls a silent, allocation-free preflight for this facade during early boot.

Phase 35F-0 does not move SSD1677 initialization, RAM writes, busy waits, refresh calls, strip rendering, or SPI/display ownership.

Normal boot remains:

```text
vaachak=x4-runtime-ready
```
