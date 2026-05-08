# Vaachak OS Roadmap

## Current stance

Vaachak OS is an X4-first reader OS. The active firmware remains the Pulp-derived runtime while Vaachak-owned crates provide shared models, contracts, and extraction targets.

## Immediate stabilization

1. Keep the active runtime buildable and flashable.
2. Keep Reader reliable for TXT, prepared TXT, and mixed EPUB smoke files.
3. Keep Wi-Fi Transfer usable for normal files and large prepared-cache uploads.
4. Keep Settings, Date & Time, and sleep-image flows consistent and recoverable.
5. Keep repository checks clean so generated or historical delivery files do not return.

## Near-term product closure

- Reader: progress restore, title display, prepared-cache open success, and compact failure diagnostics.
- Settings: clear persistence model and reader-preference application.
- Network: Wi-Fi credentials, transfer server, chunked resume, and time sync isolation.
- Device: battery/header consistency, sleep-image mode persistence, and safe wake/resume behavior.

## Extraction order

Extract low-risk pure logic before hardware behavior:

1. Settings and state models.
2. Reader progress and bookmark state I/O.
3. Book identity and title-cache helpers.
4. Prepared-cache metadata parsing.
5. Input semantic mapping.
6. Storage path helpers.
7. Wi-Fi Transfer configuration models.
8. Display drawing abstractions.
9. SPI, SD, display, and low-level input behavior only after the product baseline is stable.

## Deferred expansion

Games, QR utilities, cloud sync, multi-device support, alternate boards, and richer EPUB rendering should wait until the X4 reader baseline is stable.
