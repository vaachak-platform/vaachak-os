# CLAUDE.md

@AGENTS.md

## Claude Code
- This file keeps Claude aligned with the repo-root `AGENTS.md` contract instead of branching into a Claude-only lane.
- This repo also ships committed Claude project assets under `.claude/`, including `.claude/CLAUDE.md`; use them for Claude-native commands, hooks, rules, subagents, and the auto-memory bridge.
- Keep this file, the `.claude/` tree, and the scoped `odylith/**/CLAUDE.md` companions aligned with the same Odylith contract.
- First-match help route: if the user says `Odylith, help`, use the CLI help surface and print stdout only. Do not run install, status, intervention, or launcher diagnostics first.
- First-match demo route: if the user says `Odylith, show me what you can do` or asks what Odylith can do for this repo, use the advisory `odylith show` demo. Do not run install, status, intervention, or launcher diagnostics first.
- Capability inventory route: if the user asks to list Odylith capabilities, engines, product architecture, or the capability map, run `odylith capabilities` and print stdout only. Do not infer the taxonomy from `odylith --help`, `odylith show`, Claude Code capability prose, or any host-model surface.
- Claude Code is a first-class Odylith delegation host. Codex emits routed `spawn_agent` payloads subject to active host policy; Claude Code executes the same bounded delegation contract through Task-tool subagents and the checked-in `.claude/` project assets.
