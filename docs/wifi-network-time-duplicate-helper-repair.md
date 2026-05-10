# Wi-Fi network time duplicate helper repair

This is a small compile repair for the Wi-Fi connection restore/settings UI work.

## Problem

`vendor/pulp-os/src/apps/network_time.rs` can contain duplicated helper definitions after multiple Wi-Fi/time overlays have been applied in sequence:

- `sync_existing_stack`
- `save_time_result_to_sd`
- `save_time_error_to_sd`
- `load_time_cache_from_sd`

Rust rejects these duplicate function definitions with `E0428`.

## Repair

The repair script removes duplicate copies and leaves exactly one definition of each helper. It does not change Wi-Fi credential storage or the on-device Wi-Fi settings UI.

## Intended state

- Wi-Fi credentials remain in `/_x4/SETTINGS.TXT` via `wifi_ssid=` and `wifi_pass=`.
- There is no separate `/_x4/WIFI.TXT` file.
- Wi-Fi Transfer should connect, get DHCP, and host immediately.
- Date/time sync remains available through Date & Time, not as a blocking step before file transfer hosting.

## Validation marker

```text
wifi-network-time-duplicate-helper-repair-ok
```
