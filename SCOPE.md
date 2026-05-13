# Vaachak OS Scope

## Current accepted scope

Vaachak OS is currently scoped as an Xteink X4 reader-first firmware with Vaachak-owned runtime code as the active development path.

In scope now:

- X4 boot, display, input, storage, reader, settings, transfer, network, and sleep-state validation.
- Accepted X4/CrossPoint-compatible partition table preservation.
- Reader support for TXT/EPUB, progress, bookmarks, title cache, prepared cache metadata, reader settings, Bionic Reading, Guide Dots, sunlight-fading mitigation, and font work.
- Category dashboard: Network, Productivity, Games, Reader, System, and Tools.
- Wi-Fi setup/scan, Wi-Fi Transfer, and network time integration.
- Optional Lua apps from `/VAACHAK/APPS` using uppercase 8.3-safe physical folders.
- Reader Home / Continue Reading and local library polish.
- Reader state model freeze for progress, bookmarks, highlights, per-book settings, and library entries.
- XTC compatibility as an import/open path.
- `.vchk` as the long-term Vaachak-native book package contract.
- Sync alignment after local reader state is stable.

## Repository hygiene scope

Previous patch deliverables must not be committed:

- root zip files
- extracted patch/deliverable folders
- temporary apply/patch scripts
- one-off repair/cleanup/feature-slice validator scripts
- generated `__pycache__`, `.pyc`, `.DS_Store`, and `__MACOSX` files

Keep only production helper scripts and current documentation.

## Retained vendor scope

`vendor/pulp-os` remains in the repository for scoped compatibility/reference use. It is not the place for new Vaachak OS functionality.

`vendor/smol-epub` remains as the EPUB dependency source.

## Deferred scope

- Palm/Tern compatibility as a product milestone.
- Broad app ecosystem beyond the optional Lua sample/app path.
- Plugin/runtime expansion beyond the bounded Lua app model.
- OPDS, analytics, achievements, and cloud-only flows.
- Waveshare/S3 profile implementation until the X4 reader path is stable.
- Sync transport features that require reader-state redesign.
