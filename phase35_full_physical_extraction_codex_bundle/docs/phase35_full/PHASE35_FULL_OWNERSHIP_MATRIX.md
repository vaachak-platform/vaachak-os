# Phase 35 Full Ownership Matrix

| Area | Must be active Vaachak-owned? | Accepted evidence |
|---|---:|---|
| Storage state IO | Yes | Vaachak active state IO module handles PRG/BKM/THM/MTA/BMIDX reads/writes |
| Input semantic mapping | Yes | Active runtime maps raw input to reader/nav actions through Vaachak module |
| Display geometry helper usage | Yes | Active display/render layout uses Vaachak geometry helpers |
| Input ADC/debounce | Yes | Active GPIO1/GPIO2/GPIO3 sampling/debounce/repeat lives in Vaachak module |
| SD/SPI arbitration | Yes | Active shared SPI transaction ownership lives in Vaachak module |
| SSD1677 refresh/strip rendering | Yes | Active SSD1677 init/RAM/refresh/strip path lives in Vaachak module |
| Reader app internals | Yes | Active Home/Files/Reader/Settings/AppManager lives in Vaachak module |

Vendor source must remain unchanged.
