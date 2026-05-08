# Wi-Fi Transfer configuration ownership

This document records the Vaachak-owned pure model for Wi-Fi Transfer configuration.

## Runtime ownership boundary

The active X4 runtime still owns Wi-Fi connection logic, mDNS, HTTP server behavior, upload handling, SD writes, retry execution, and browser UI assets. This extraction only adds pure config, validation, and failure classification models in `vaachak-core`.

## Current compatibility points

The browser UI remains compatible with:

- `Original Transfer`
- `Chunked Resume`
- large prepared-cache folder uploads such as `/FCACHE/15D1296A`
- chunk-size default: `256`
- chunk-size bounds: `128..1536`
- chunk delay default: `250 ms`
- chunk delay bounds: `0..2000 ms`
- file delay default: `600 ms`
- file delay bounds: `0..3000 ms`

## Security rule

The transfer config model does not store or expose Wi-Fi credentials. Wi-Fi credentials remain owned by the existing runtime settings file and connection flow.

## Non-goals

This extraction does not move Wi-Fi connect logic, HTTP server/upload handling, mDNS behavior, SD I/O, browser UI behavior, or FCACHE file writes.
