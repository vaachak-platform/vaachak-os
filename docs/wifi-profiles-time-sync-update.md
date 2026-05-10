# Wi-Fi profiles + transfer time sync update

This update keeps the existing Wi-Fi radio, transfer server, SD, display, and Date & Time special-mode behavior in the current runtime path. It adds a small saved-profile contract and reuses the already-connected transfer network stack to sync time before serving files.

## New profile file

Saved Wi-Fi profiles live at:

```text
/_x4/WIFI.TXT
```

Example:

```text
# Vaachak Wi-Fi profiles
# active can match profile or ssid
active=home

profile=home
ssid=YourSSID
pass=YourPassword

profile=office
ssid=OfficeSSID
pass=OfficePassword
```

The active profile is used first by Wi-Fi Transfer and Date & Time. If `/_x4/WIFI.TXT` is missing or has no complete active profile, the existing `/_x4/SETTINGS.TXT` `wifi_ssid` / `wifi_pass` fallback remains active.

## Editing profiles

Start Wi-Fi Transfer from the Network category and open:

```text
http://x4.local/wifi
```

The editor saves back to `/_x4/WIFI.TXT`. The current transfer session keeps using the already-connected network; the next transfer or Date & Time sync uses the updated active profile.

The raw profile file can also be downloaded from:

```text
http://x4.local/wifi.txt
```

## Transfer time sync

After Wi-Fi Transfer connects and DHCP succeeds, the transfer screen now performs an NTP sync over the same connected stack and writes the result to:

```text
/_x4/TIME.TXT
```

This means the user does not need to run Date & Time separately after starting Wi-Fi Transfer.

## Ownership boundaries

Changed:

```text
vendor/pulp-os/src/apps/wifi_profiles.rs
vendor/pulp-os/src/apps/manager.rs
vendor/pulp-os/src/apps/home.rs
vendor/pulp-os/src/apps/upload.rs
vendor/pulp-os/src/apps/network_time.rs
vendor/pulp-os/assets/upload.html
vendor/pulp-os/kernel/src/kernel/config.rs
```

Not changed:

```text
SSD1677/display refresh policy
scheduler redraw behavior
reader page-turn behavior
SD mount/probe behavior
SPI arbitration
Wi-Fi radio driver ownership
```
