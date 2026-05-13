# Current Architecture and Roadmap

## Architecture baseline

Vaachak OS is currently an X4-first reader firmware. The active development path is Vaachak-owned runtime code under `target-xteink-x4/src/vaachak_x4/**`, with shared models in `core/**`, X4 HAL seams in `hal-xteink-x4/**`, optional Lua VM support in `support/**`, and host tooling in `tools/**`.

Current architecture commitments:

- X4/CrossPoint-compatible partition table remains fixed.
- X4 reader behavior is protected before broader platform expansion.
- New functionality goes into Vaachak-owned paths, not `vendor/pulp-os`.
- `vendor/pulp-os` may remain as scoped compatibility/reference material.
- `vendor/smol-epub` remains the EPUB dependency source.
- Lua apps are optional SD-loaded apps under `/VAACHAK/APPS` and do not replace native reader/runtime features.
- Future board expansion should use capability profiles rather than forks.

## Current product surfaces

- Home/category dashboard: Network, Productivity, Games, Reader, System, Tools.
- Files and Reader path for TXT/EPUB.
- Reader state: progress, bookmarks, title cache, prepared cache metadata, and settings.
- Reader display aids: Bionic Reading, Guide Dots, and sunlight-fading mitigation.
- Wi-Fi setup/scan, Wi-Fi Transfer, and network time.
- Optional Lua apps: Calendar, Panchang, Daily Mantra, Dictionary, Unit Converter, and Games.
- Sleep image mode helpers and SD asset tooling.

## Current repository hygiene state

The repository should remain free of generated patch artifacts:

- no root zip files
- no extracted deliverable folders
- no generated apply/patch scripts
- no one-off repair/cleanup validator scripts
- no Python bytecode/cache folders
- no macOS metadata folders

Use `./scripts/check_repo_hygiene.sh` before committing.

## Current roadmap

1. Reader Home + Continue Reading polish.
2. Reader data model freeze.
3. Library index and title-cache polish.
4. XTC compatibility.
5. `.vchk` spec freeze.
6. `.vchk` read/open support.
7. `.vchk` mutable state.
8. Sync alignment.

## Architecture rule

Every new abstraction must improve the X4 reader path now, or it moves to the later profile/backlog track.
