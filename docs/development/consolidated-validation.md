# Consolidated validation

Use this validation script after the pure-model extraction series:

```bash
cargo fmt --all
./scripts/validate_controlled_extraction_consolidation.sh
```

The script checks:

- formatting
- cleanup guard
- Vaachak-owned core model inventory
- hardware-free pure model boundary
- active `vendor/pulp-os` runtime markers
- consolidated docs
- `vaachak-core` tests
- host workspace checks
- embedded workspace checks
- clippy with warnings denied
- release build for `target-xteink-x4`
- active Pulp runtime release build

The script does not replace on-device testing. Use the manual checklist printed at the end of the script and the hardware migration readiness checklist before starting hardware-adjacent work.
