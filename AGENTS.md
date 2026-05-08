# Repository Working Notes

## Current priority

Keep the active X4 runtime stable while cleaning the repository and documenting the true ownership map.

## Active runtime

Use `vendor/pulp-os` as the device firmware source of truth. Do not rewrite working display, SD, input, scheduler, Reader, Home, Wi-Fi Transfer, Date & Time, Settings, or sleep-image behavior unless the requested change is explicitly scoped and device-testable.

## Vaachak-owned crates

Use the root workspace for target-neutral models, contracts, and adapters:

- `core/`
- `hal-xteink-x4/`
- `target-xteink-x4/`

Keep these crates clean, semantic, and free of generated delivery labels.

## Naming discipline

Use semantic names based on domain behavior, not development chronology. Do not add generated archives, temporary patch directories, local backup folders, or historical delivery labels to source control.

## Validation

For repo cleanup work, run the cleanup guard from the repository root. For device firmware work, build and flash from `vendor/pulp-os` and validate on the X4.
