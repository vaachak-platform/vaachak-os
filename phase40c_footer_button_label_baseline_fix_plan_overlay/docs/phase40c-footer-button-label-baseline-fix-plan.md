# Phase 40C — Footer/Button Label Baseline and Fix Plan

Phase 40C is plan-only.

It does not change rendering or input behavior. It inspects the current footer and
button mapping surface, records expected labels, and produces an exact patch plan
for Phase 40D.

Target issue to investigate:

```text
Footer labels should match physical button behavior and screen context.
Known expected reader/footer ordering from prior device feedback:
Back Select Open Stay
```

Recommended flow:

```bash
./phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/guard_phase40c_reader_ux_baseline.sh
./phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/inspect_phase40c_footer_button_sources.sh
./phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/inspect_phase40c_button_mapping_candidates.sh
EXPECTED_FILES_FOOTER="Back Select Open Stay" EXPECTED_READER_FOOTER="Back Select Open Stay" ./phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/write_phase40c_expected_footer_labels_baseline.sh
./phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/plan_phase40c_footer_button_label_fix.sh
./phase40c_footer_button_label_baseline_fix_plan_overlay/scripts/accept_phase40c_footer_button_label_baseline_fix_plan.sh
```
