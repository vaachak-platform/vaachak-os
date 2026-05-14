# Release cleanup + X4 firmware workflow deliverable

Run `scripts/apply_release_cleanup_and_firmware_workflow.sh` from the repository root after unzipping this package.

The apply script removes generated patch artifacts and one-off patch scripts, installs the release firmware build helper, and leaves GitHub workflows that can build `dist/x4/firmware.bin`.
