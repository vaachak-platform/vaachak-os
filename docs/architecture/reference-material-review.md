# Reference Material Review

## Architecture split documents

Still useful:

- Shared core with device capability profiles.
- X4 as the disciplined reader-first profile.
- Waveshare/S3 as a later richer profile.
- Native app ABI and first-party system apps before compatibility layers.

Update needed:

- Hardware is no longer in bring-up or Pulp-active state. Vaachak-native hardware runtime is accepted.

## MVP implementation/program plan

Still useful:

- Reader-first scope.
- Home, Library, Reader, Settings, Transfer as first-party system surfaces.
- Weekly validation discipline.
- No Palm/Tern compatibility or broad plugin runtime in MVP.

Update needed:

- Treat hardware migration as completed. Move roadmap attention to Reader Home, data model, XTC, `.vchk`, and sync alignment.

## Roadmap document

Still useful and now primary for next work:

- Stabilize reader/home/library.
- Freeze reader data model.
- Add XTC compatibility.
- Freeze and implement `.vchk`.
- Align sync after local behavior is solid.

## `.vchk` draft

Still useful as planning contract.

Status:

- Draft package contract.
- Not yet an implemented feature.
- Should be frozen after reader state models are stable.
