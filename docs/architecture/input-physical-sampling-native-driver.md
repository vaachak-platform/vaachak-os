# Input Physical Sampling Native Driver

## Marker

`input_physical_sampling_native_driver=ok`

## Purpose

This deliverable performs the first lower-level physical driver migration below the accepted native behavior consolidation. Vaachak now owns the Xteink X4 input physical sampling interpretation layer while keeping actual ADC/GPIO peripheral reads available through the Pulp-compatible fallback.

## What moved into Vaachak

- ADC ladder sample interpretation for Xteink X4 buttons.
- Oversample reduction for the four-sample ADC window.
- GPIO3 power-button low-active level interpretation.
- Conversion from physical sample results into the accepted Vaachak native input event pipeline.
- Validation of the existing X4 ladder centers:
  - GPIO1 row: Right, Left, Confirm, Back.
  - GPIO2 row: VolDown, VolUp.
  - GPIO3: Power button handoff.

## What remains Pulp-compatible

- Actual ADC peripheral reads.
- Actual GPIO polling.
- Hardware access timing around the existing input task.
- Final app navigation dispatch.
- Display, storage, and SPI behavior.

## Active backend

`VaachakPhysicalSamplingWithPulpAdcGpioReadFallback`

The active low-level fallback remains `PulpCompatibility` until the next slice explicitly migrates ADC/GPIO peripheral read execution.

## Safety guardrails

This deliverable must not change:

- reader/file-browser UX;
- app navigation screens;
- display refresh behavior;
- storage/FAT behavior;
- SPI transfer or chip-select behavior.

## Hardware smoke

After flashing, validate:

- all buttons respond;
- direction mapping is unchanged;
- Back still works;
- no missed or double presses;
- file browser opens;
- SD files list;
- TXT/EPUB open;
- display refresh remains unchanged.
