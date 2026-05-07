# Network Time

## Date & Time Screen

`System > Date & Time` shows the clock state, cached time, date, timezone, sync status, last sync time, and last result. Pressing Select starts an explicit Wi-Fi client connection and NTP query in the isolated Time Sync mode. Back returns to the System category.

The Home screen never starts Wi-Fi or NTP. If the cache is missing or corrupt, Home and Date & Time show an unsynced fallback.

## Clock continuity

Vaachak OS stores the last successful NTP epoch together with the device uptime when that sync occurred. During the same boot session, the clock is treated as live and advances by adding elapsed uptime to the saved epoch.

After a reboot or full power loss, the device uptime resets. In that case the clock is shown as cached/stale rather than live. Date & Time shows a resync prompt, and Home shows a compact cached date. This avoids pretending the cached value is an accurate running clock when the device has no battery-backed RTC.

Clock states:

```text
Live
  NTP synced in the current boot; clock advances using uptime.

Cached
  A previous sync exists, but this boot cannot prove elapsed real time.
  Use Select on Date & Time to resync.

Unsynced
  No valid previous NTP sync exists.
```

## Cache File

The time cache is stored at:

```text
/_x4/TIME.TXT
```

Format:

```text
timezone=America/New_York
last_sync_unix=1714963320
last_sync_monotonic_ms=5000
last_sync_ok=1
last_sync_source=ntp
last_sync_error=
last_sync_ip=192.168.1.10
display_offset_minutes=-240
```

`last_sync_unix` is the NTP-derived Unix timestamp. `last_sync_monotonic_ms` is the device uptime captured during that same sync. If the current uptime is lower than the stored sync uptime, Vaachak treats the clock as cached after reboot.

## Wi-Fi Credentials

Date & Time uses the existing Wi-Fi settings from `/_x4/SETTINGS.TXT`:

```text
wifi_ssid=...
wifi_pass=...
```

The password is used only to configure the Wi-Fi client and is not displayed. The transfer server is not started for time sync.

## NTP

The sync mode sends a 48-byte NTP request over UDP port 123 and tries:

```text
pool.ntp.org
time.google.com
time.cloudflare.com
```

The NTP transmit timestamp is converted to Unix time by subtracting 2208988800 seconds.

## Timezone

Local display is centralized as `America/New_York`. The implementation applies the US Eastern DST transition rules used since 2007. A future Settings item can replace this constant with user-selectable timezone behavior.

## Network Status

Network Status includes a Time Sync line using the same cached state as Date & Time. It reports Live, Cached, Never synced, or Failed status. Network Status does not trigger Wi-Fi or NTP.

## Home

The Home category dashboard shows time/date and battery status in the header. It does not show battery on every card.

Examples:

```text
Wed May 6  9:42 AM    92% [battery icon]
Cached Wed May 6      92% [battery icon]
Time unsynced         92% [battery icon]
```

## Not included yet

```text
Calendar app
Hindu calendar
Timezone picker in Settings
Automatic boot sync
Browser SD card manager
