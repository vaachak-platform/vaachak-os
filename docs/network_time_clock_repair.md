# Network Time Clock Repair

This overlay repairs two issues in the active Pulp OS Home and Date & Time path.

## Scope

- Move battery status out of every Home category card.
- Show battery percentage once in the Home header with a small drawn battery icon.
- Keep Home time/date display in the header.
- Remove the full-screen loading overlay that showed `Syncing time...20%`.
- Add a hard guard around the Date & Time NTP sync path so the device returns to the Date & Time screen instead of staying stuck indefinitely.
- Add bounded Wi-Fi start/connect timeouts inside the NTP sync helper.

## Runtime behavior

Home now shows:

```text
Vaachak                  Time/status   92% [battery]
```

Category cards show only their title and subtitle.

Date & Time sync is still explicitly started with Select. Home does not start Wi-Fi or NTP automatically.

If sync fails or times out, Date & Time should return with a safe failure status instead of leaving the device stuck on a loading screen.

## Validation

Run from the repository root:

```bash
cargo fmt --all --check
cargo check --workspace --target riscv32imc-unknown-none-elf
cargo clippy --workspace --target riscv32imc-unknown-none-elf -- -D warnings
cargo build -p target-xteink-x4 --release --target riscv32imc-unknown-none-elf
./scripts/check_no_milestone_artifacts.sh .
git diff --check
```

Then flash and confirm:

- Home category cards no longer show `Batt NN%` in every card.
- Home header shows time/date status plus battery percentage/icon.
- System > Date & Time opens.
- Select starts sync and returns with Synced or Failed/timeout status.
- Back returns to System after sync finishes or fails.
- Wi-Fi Transfer still works.
