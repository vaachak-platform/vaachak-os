# Current Architecture and Roadmap

## Architecture baseline

Vaachak OS now has an accepted X4-native hardware runtime. The architecture should be described as:

- Vaachak-native hardware ownership for SPI, display, SD/MMC, FAT, and input.
- X4-first reader product path.
- Shared-core friendly contracts and models.
- `vendor/pulp-os` retained only for scoped non-hardware compatibility/import/reference surfaces.
- Future board expansion through capability profiles, not forks.

## What changed from the original plan

The original plans assumed an X4-first bring-up and staged extraction path. The repo has moved past hardware bring-up into accepted Vaachak-native hardware ownership. The next work is no longer hardware migration; it is reader product architecture.

## Current roadmap

1. Reader Home + Continue Reading.
2. Reader data model freeze.
3. Library index polish.
4. XTC compatibility.
5. `.vchk` spec freeze.
6. `.vchk` read/open support.
7. `.vchk` mutable state.
8. Sync alignment.

## Architecture rule

Every new abstraction must improve the X4 reader path now, or it moves to the later profile/backlog track.
