# Phase 40G Repair 2 — Text Title Cache Safety

The screenshot shows TXT entries displayed as body/license text:

```text
most other parts of the world at no cost ...
```

This repair makes TXT/MD title extraction strict:

```text
Accept only lines beginning with: Title:
Do not use arbitrary first body line.
Do not save Project Gutenberg license/body text as title.
```

After applying and flashing, rebuild the title cache:

```bash
SD=/media/mindseye73/C0D2-109E \
./phase40g_repair2_text_title_cache_safety_overlay/scripts/reset_phase40g_repair2_bad_title_cache_on_sd.sh
```
