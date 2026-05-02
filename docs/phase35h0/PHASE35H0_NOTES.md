# Phase 35H-0 Notes

This phase is arbitration-contract-only.

The active physical bus path remains:

```text
Board::init -> imported SPI2/DMA setup -> imported SD init -> imported EPD device
```

Future SD/SPI extraction should be hardware-gated and split into small steps:

- Vaachak-owned pin/device contract only
- Vaachak-owned bus manager type without active ownership
- active SD transaction ownership
- active display transaction ownership
- refresh-time background SD arbitration
