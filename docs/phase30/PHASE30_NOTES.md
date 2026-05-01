# Phase 30 Notes

## Why Phase 30 Exists

Phases 18–29 built Vaachak-owned runtime boundaries and contracts while the proven imported Pulp reader runtime remained intact.

Phase 30 consolidates those modules into a durable Vaachak namespace.

## Why Not Move Hardware Yet

The imported runtime is known-good on the physical Xteink X4.

The riskiest behavior paths are:

```text
shared SPI bus arbitration
SSD1677 refresh/strip rendering
ADC ladder/debounce
EPUB cache IO
reader app internals
```

Phase 30 avoids those risks.

## Future Work

After Phase 30, move one behavior path at a time.

Suggested next phase:

```text
Phase 31 — Make active runtime call Vaachak storage path helpers
```
