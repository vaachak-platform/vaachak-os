# Network Time Sync Special-Mode Repair

This repair keeps the Home screen lightweight and moves the Date & Time NTP operation into an isolated Wi-Fi special mode, matching the same scheduler pattern used by Wi-Fi Transfer.

## What changed

- `System > Date & Time > Select` now enters `AppId::TimeSync` instead of running Wi-Fi/NTP inside the Home background loop.
- Time sync runs outside the normal Home input/render loop.
- Back can cancel while waiting for Wi-Fi start, Wi-Fi connect, DHCP, DNS, or NTP response.
- The old full-screen `Syncing time...20%` loading overlay is not used.
- On success, `/_x4/TIME.TXT` is written from the isolated sync mode.
- On failure, Date & Time returns with a clear cached failure status instead of leaving the header stuck on Syncing.
- Home still never starts Wi-Fi or NTP.
- Battery remains displayed in the Home header, not repeated in every category card.

## Runtime behavior

1. Open `System > Date & Time`.
2. Press Select.
3. The scheduler enters an isolated Date & Time Wi-Fi mode.
4. The mode connects to Wi-Fi, waits for DHCP, queries NTP, and writes `/_x4/TIME.TXT`.
5. The result screen waits for Back.
6. Returning reloads Date & Time from the cached status file.

If NTP is blocked by the network, the device should now return with an error such as DNS failed, DHCP timeout, or NTP timeout instead of getting stuck.

## Validation

```bash
cargo fmt --all --check
cargo check --workspace --target riscv32imc-unknown-none-elf
cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
./scripts/check_no_milestone_artifacts.sh .
git diff --check
```

## Device test

- Boot the X4.
- Confirm Home shows time/date or unsynced plus battery in the header.
- Open `System > Date & Time`.
- Press Select.
- Confirm a Date & Time sync screen appears.
- Press Back during sync to confirm the device is not stuck.
- Run sync again and wait for success or a clear failure.
- Confirm Wi-Fi Transfer still works.
