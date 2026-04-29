# VaachakOS Bootstrap Phase 8.5 — X4 Input Poll-Shape Fix

Phase 8.4 confirmed the target was receiving the same calibrated ADC values as the
working `x4-reader-os-rs` input path:

- idle around 2968 / 2968
- row1: 3, 1113, 1984, 2556 buckets
- row2: 3, 1659 buckets
- GPIO3 power-low event

The remaining bug was not threshold mapping. It was the target-loop shape.

## Root cause

The Phase 8 loop did this in each cycle:

1. ingest ADC sample into the debounce state
2. drain queued events
3. call `tick()` immediately in the same cycle

For a new button press, `candidate` became the decoded button while `stable` was still
`None`. The immediate timer-only tick fed `stable` back into the step function and reset
`candidate` to `None` before the candidate could remain stable for the 15ms debounce
window.

## Fix

This phase mirrors the proven `x4-reader-os-rs` input cadence more closely:

- poll every 10ms
- ingest one oversampled ADC snapshot per poll
- no timer-only tick during the navigation smoke
- Press/Release navigation only

LongPress/Repeat are intentionally deferred until VaachakOS has a dedicated input task
like the proving-ground runtime.
