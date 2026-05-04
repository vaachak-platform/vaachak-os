# Phase 39G — Runtime File API Integration Gate Overlay

This phase adds the gate between the Phase 39F runtime-owned writer binding and
the actual runtime/kernel file APIs.

It adds:

```text
Phase39gRuntimeFileApiProbe
Phase39gIntegrationRequest
Phase39gIntegrationGateReport
phase39g_execute_runtime_file_api_gate(...)
Phase39G acceptance/report layer
```

It also includes:

```text
scripts/find_phase39g_runtime_file_api_candidates.sh
```

That script searches the active Pulp/X4 code for likely file API call sites.

This phase does not wire reader save call sites yet. The next phase should wire
progress save first, then expand to theme, metadata, bookmarks, and bookmark index.
