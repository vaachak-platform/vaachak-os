# Phase 35F-0 Notes

This phase is geometry-only.

The active display path remains:

```text
Board::init -> DisplayDriver::new -> epd.init -> Kernel display scheduling -> imported strip rendering
```

The facade exists so future extraction phases can replace helper use before moving physical SSD1677 behavior.

The later display extraction should stay split:

- geometry helper use
- strip range mapping
- SSD1677 command ownership
- busy wait and refresh sequencing
- shared SPI transaction ownership
