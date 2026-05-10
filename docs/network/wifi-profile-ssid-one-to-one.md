# Wi-Fi Profile / SSID One-to-One Update

This update replaces the previous default-profile/list layout with a direct profile-owned credential model.

Each profile owns exactly one SSID/password pair:

```text
Profile: Home / Work / Other
SSID: selected from scan results
Password: entered with on-device keyboard
Default: Yes / No
```

The only persistent source remains:

```text
/_x4/SETTINGS.TXT
```

No `/_x4/WIFI.TXT` file and no browser `/wifi` routes are added.

## On-device flow

```text
Network -> Wi-Fi Networks
```

Main setup screen:

```text
Profile: Home
SSID: Scan SSID and select one
Password: (empty) or set (N chars)
Default: Yes/No
```

Controls:

```text
Up/Down        move between Profile, SSID, Password, Default
Left/Right     change Profile; OK/Right opens SSID or Password rows
OK on SSID     opens scanned SSID picker
OK on Password opens keyboard after SSID is selected
OK on Default  makes selected profile default
Left on SSID   clears selected profile SSID/password
Left on Pass   clears selected profile password
Back           exits
```

SSID picker:

```text
Up/Down        choose scanned network
OK             assign SSID to selected profile
Back           return to setup
```

Password keyboard:

```text
Up/Down/Left/Right  move keyboard cursor
OK                  enter selected key
Hold OK             delete one character
done                save password to selected profile
Back                cancel password editing
```

## SETTINGS.TXT format

```text
wifi_ssid=<default profile ssid>
wifi_pass=<default profile pass>
wifi_default=0

wifi_profile_0_name=Home
wifi_profile_0_ssid=<home ssid>
wifi_profile_0_pass=<home password>
wifi_profile_1_name=Work
wifi_profile_1_ssid=<work ssid>
wifi_profile_1_pass=<work password>
wifi_profile_2_name=Other
wifi_profile_2_ssid=<other ssid>
wifi_profile_2_pass=<other password>
```
