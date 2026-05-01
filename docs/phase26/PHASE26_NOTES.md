# Phase 26 Notes

Phase 26 intentionally mirrors the Phase 25 pattern: strengthen a Vaachak-owned contract first, then move physical behavior only after the contract is stable.

The input contract smoke validates pure metadata and semantic mappings. It does not read ADC values or touch hardware.

Future phases can use this contract to migrate one input concern at a time:

1. Observe-only ADC sampling diagnostics.
2. Vaachak-owned button role normalization.
3. Vaachak-owned debounce/repeat policy.
4. Reader action routing after behavior parity is proven.

Do not edit `vendor/pulp-os` or `vendor/smol-epub` in Phase 26.
