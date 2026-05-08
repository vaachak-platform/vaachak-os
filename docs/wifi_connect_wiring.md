# Wi-Fi Connect wiring

The current implementation turns `Network > Wi-Fi Connect` from a placeholder into a local configuration/status screen.

Current behavior:

- Reads `_x4/SETTINGS.TXT` through the existing kernel config parser.
- Shows whether `wifi_ssid` is configured.
- Shows whether `wifi_pass` is present, without displaying the password.
- Keeps the radio idle; Wi-Fi runtime startup remains owned by the later transfer screen.
- `Select` refreshes the local config snapshot.
- `Back` returns to the Network category.

Expected settings keys:

```text
wifi_ssid=YourNetworkName
wifi_pass=YourPassword
```

This deliverable intentionally does not start Wi-Fi or host a transfer service yet. It prepares the user-facing network setup surface before Wi-Fi Transfer owns the radio lifecycle.
