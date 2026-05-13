# Reader orientation landscape acceptance cleanup

This cleanup keeps the accepted reader orientation behavior and removes iterative trace output from the portrait/inverted and landscape bring-up.

## Accepted behavior

- Reader Orientation menu supports:
  - Portrait
  - Inverted
  - Landscape CW
  - Landscape CCW
- Portrait maps to SSD1677 rotation `Deg270`.
- Inverted maps to SSD1677 rotation `Deg90`.
- Landscape CW maps to SSD1677 rotation `Deg0`.
- Landscape CCW maps to SSD1677 rotation `Deg180`.
- Orientation changes force a full redraw.
- The AppManager and redraw-policy AppLayer bridges continue to delegate `desired_display_rotation()`.
- Landscape orientation uses `ReaderViewportModel` logical `800x480` geometry.
- Prepared page/cache identity includes orientation-specific suffixes.

## Stable monitor marker

```text
reader-orientation=x4-reader-landscape-render-ok
```

## Deferred

- Orientation-aware button remapping.
- Separate landscape-specific reader UX tuning beyond the initial viewport/reflow slice.
