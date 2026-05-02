# Phase 35H-0 - SPI Bus Arbitration Facade

Phase 35H-0 adds a Vaachak-owned pure facade for the Xteink X4 shared SPI bus contract.

It models:

- SPI pins: SCLK GPIO8, MOSI GPIO10, MISO GPIO7.
- Display chip-select: EPD CS GPIO21.
- Storage chip-select: SD CS GPIO12.
- SD probe frequency: 400 kHz.
- Operational frequency: 20 MHz.
- DMA channel and buffer sizes: channel 0, 4096 byte TX/RX buffers.
- The rule that display and SD chip-selects must not be selected together.
- The rule that SD initialization happens before display traffic on a clean slow bus.

Phase 35H-0 does not initialize SPI, configure DMA, create bus devices, initialize SD, initialize display, or move transaction ownership.

Normal boot remains:

```text
vaachak=x4-runtime-ready
```
