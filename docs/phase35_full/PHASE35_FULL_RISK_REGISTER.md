# Phase 35 Full Risk Register

## High risk areas

```text
- SD/display shared SPI arbitration
- SSD1677 refresh and strip rendering
- ADC ladder thresholds and debounce
- Reader state persistence
- EPUB reader memory behavior
- App manager event loop behavior
```

## Mitigations

```text
- Preserve revert script.
- Keep vendored Pulp source untouched.
- Copy known-good code before adapting.
- Validate each ownership area with scripts.
- User performs final device validation.
```
