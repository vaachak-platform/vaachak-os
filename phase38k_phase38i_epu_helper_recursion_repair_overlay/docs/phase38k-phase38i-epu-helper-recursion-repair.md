# Phase 38K/38I Repair — EPU Helper Recursion

This repair replaces the accidentally recursive `phase38i_is_epub_or_epu_name`
helper with a direct byte-suffix predicate:

- `.EPUB`
- `.EPU`

It only patches source text in the two active Pulp files and prints a marker:

```text
phase38k-phase38i-repair=x4-epu-helper-recursion-fixed
```
