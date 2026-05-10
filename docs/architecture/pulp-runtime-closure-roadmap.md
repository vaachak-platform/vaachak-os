# Pulp runtime closure roadmap

The active Vaachak X4 Wi-Fi path is now Vaachak-owned. The following areas still use imported Pulp compatibility code and should be migrated in separate hardware-safe slices:

```text
board/peripheral ownership
kernel scheduler and KernelHandle
SSD1677 display driver and strip buffer
input ADC/button driver
SD/FAT storage driver
Files app
Reader app
Settings app
reader_state compatibility helpers
font/widget rendering helpers
```

Rules going forward:

```text
- Do not add new functionality under vendor/pulp-os.
- Add new X4 runtime functionality under target-xteink-x4/src/vaachak_x4/**.
- Keep the X4/CrossPoint-compatible partition table unchanged.
- Migrate one active runtime surface at a time and keep validation scripts with each slice.
```

Useful audit command:

```bash
./scripts/audit_remaining_pulp_runtime_dependencies.sh
```
