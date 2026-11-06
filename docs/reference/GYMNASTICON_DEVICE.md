# Gymnasticon card and live device

Historical Coach evidence retained during consolidation. Current state and ownership
are in the root [status](../../STATUS.md) and [documentation map](../README.md). Dated claims below are not current acceptance.

## Current board after hardware move

The card is now running on **Raspberry Pi Zero 2 Rev 1.0** (owner: Zero 2 W),
reachable at the supplied **10.0.1.61** when SSH is available. Existing gym SSH
host identity was preserved; use strict HostKeyAlias=gym for direct-IP checks.
Do not assume the old 10.0.1.148 or its Pi 3 A+ hardware is current.

Watchdog bot/helper hashes and explicit zero startup config match the installed
change below. [The BLE pulse/expiry test passed again](ZWIFT_BLE_PAIRING_TEST.md)
on this board. PID 583/NRestarts=2 were stable between checks, zero confirmed,
no throttling reported. Two initial boot errors were ENODEV and Bluetooth
poweredOff. Startup reported no ANT+ stick found, so ANT on this board remains
unqualified. SSH reachability was intermittent; no network repair was performed.
The Pi 3 A+ records below describe earlier hardware, not a current inventory.

## Subsequent Zero 2 reboot and ANT detection

After the owner reported a reboot, strict SSH at 10.0.1.61 verified boot ID
b9c901a2-b9d3-495c-9dce-419e339b2943 and short uptime. USB enumerated ANTUSB-m
0fcf:1009, and Gymnasticon reported ANT+ stick opened. PID 552 was active with
NRestarts=1 (initial Bluetooth ENODEV), zero output and throttled=0x0.
The earlier no-stick result above is historical, not current enumeration state.

Zwift's pre-ride selector discovered Powermeter 11234 over ANT and showed it
selected for power, with BLE Gymnasticon still connected for cadence. The mixed
pulse attempt stopped at the foreground guard before positive commands were
sent. This establishes discovery/initialization, not a successful live pulse or
simultaneous decoded-value parity test on this boot. No network settings or
router-forwarding policy were changed. The agent's earlier remote reboot
attempts failed before delivery; the observed reboot followed the owner's action.

## Zero 2 ANT live retest completed

[The subsequent guarded retry passed](ZWIFT_ANT_PAIRING_TEST.md): both power and
cadence were selected over ANT as Powermeter 11234 and displayed 0/0 → 150 W/85 rpm
→ 0/0 after producer silence, before cleanup zero. No ride started. This supersedes
the earlier pending-pulse status above. BLE separately passed on the Zero 2;
simultaneous reception and startup-race/reliability qualification remain open.

## Current watchdog update

The [subsequent receive-side watchdog change](GYMNASTICON_WATCHDOG.md) supersedes
earlier undefined-cadence/no-expiry observations below: installed 1.4.0 now starts
at 0 W / 0 rpm and expires accepted full samples after 2.5 s. Service restart and
ANT observations were tested. Sender-to-zero delay is not universally under 3 s;
BLE and process-crash behavior remain unqualified. Historical boot evidence below
is retained rather than rewritten as a current guarantee.

## Verified state

The owner booted the inspected Gymnasticon card in a Raspberry Pi 3 Model A+
and approved trust of its newly observed SSH host key.

- Hostname after first-run: **gym**, current DHCP address **10.0.1.148**.
- Login: `ssh jack@gym`; public-key login succeeded.
- Home: `/home/jack`; UID/GID 1000 preserved through the account rename.
- Raspbian 10 Buster; installed service uses **Gymnasticon 1.4.0** under
  `/opt/gymnasticon`. A separate 1.4.1 checkout is not the service installation.
- SSH, Bluetooth and Gymnasticon active during inspection.
- Gymnasticon running as `jack:jack`, UDP listener `0.0.0.0:3000`.
- Configured bot starts at 0 W. Live logs show 0 W and undefined cadence.
- Startup log reports an ANT+ stick opened. Actual RF output/profile correctness
  and Zwift pairing were not independently tested.
- Bluetooth controller up/running; no ACL connection during the sampled status.
- `vcgencmd get_throttled` returned 0x0 during inspection.

Address is a lease, not a reservation. It was previously used by the same Pi
hardware with the oldgarage card; do not infer card identity from IP/MAC alone.

## Approved card preparation

Using the reviewed Windows/WSL helpers, identified the 16 GB card by reader,
capacity and partition IDs. Inspected read-only first, with ext4 journal replay
disabled. Private backups precede configuration writes.

- Installed the managing machine's public SSH key, without copying private keys
  or replacing the card's host keys.
- Copied C2's SSID-specific derived key from pi5data.
- Copied C1_ASUS's own credential from WYK82Q.
- Installed both profiles in the root filesystem and the pending first-run
  script. Used a quoted heredoc to prevent shell expansion of configuration.
- Owner explicitly retained first-run: hostname gym, pi→jack account/home rename,
  SSH enablement, self-removal and reboot.
- Changed the existing Gymnasticon service's User/Group to jack to match that
  rename. Kept its existing executable, config and restart behavior.
- Normalized carriage returns in boot command line; retained first-run arguments.
- Bash syntax and exact file readbacks passed. No ARM program was executed on WSL.
- Unmounted and detached the card before owner boot.

Backups remain private under `~/.local/share/gymnasticon-card-backup.JrHFrl/`
and `~/.local/share/gymnasticon-firstrun-backup.lD1ZBN/`. They include credentials;
never copy them into Git or print them.

## Post-boot verification

First-run completed: hostname gym, user/home jack, script and boot invocation
removed. Public-key SSH authentication succeeded. Approved host key was saved
for gym and gym.chidley.home:

`SHA256:o/eJ0sihAGu4uh6x5He3GG85aC6Zo9+3oyAcsGYXDH8` (Ed25519).

Two initial service failures were `ENODEV` while Noble tried to bind the
Bluetooth HCI device. A subsequent automatic restart succeeded; MainPID and
restart count remained unchanged between two observations. This is a startup
readiness issue, not a demonstrated permanent fix. No service restart, bot
command or positive synthetic prescription was sent during verification.
The existing service itself initializes radio backends automatically.

## Subsequent native Windows ANT bench

[ANT receive tests](ANT_WINDOWS_RECEIVE_TEST.md) subsequently verified real
radio reception and controlled power/cadence steps on a native Windows receiver.
Explicit zero cadence was sent, but not persisted in startup config. Freshness
remains a gap; BLE and actual Zwift pairing are still untested. Earlier no-command
statements above refer to the initial boot inspection, not that later bench.

## Limits and next work

- Do not use synthetic output in public competitive events; keep normal clients
  unpaired during bench preparation.
- Bot listens on all interfaces without demonstrated authentication and retains
  the last accepted values. Inspected code has no producer-expiry watchdog.
- Cadence is currently unspecified. Define an explicit initial zero and validate
  missing/zero handling before useful output tests.
- Assess the Bluetooth startup race before relying on unattended boot.
- Validate ANT+/BLE output with an independent receiver before claiming parity
  or client compatibility. No hill/drafting/physics experiment has run.
- Old Buster/Node dependencies are retained, not endorsed as current/security-
  qualified. No upgrades were performed as part of this card preparation.
- Earlier script inspection accidentally exposed credential-bearing arguments
  in tool output. Do not copy that output into project documentation or share
  the transcript as-is.

## Separate oldgarage card

Oldgarage is a BME280 environmental-logger card, not this installation.
It successfully booted on this same Pi 3 A+ with working C2 and SSH, but its
BME280 service failed and a power warning was observed. The earlier Zero W
Wi-Fi errors are historical evidence from different hardware.

Its C1_ASUS profile was initially given a C2-derived key in error. Correction
remains pending because oldgarage ceased answering SSH when this Pi was switched
to gym. C2 works; do not claim C1_ASUS is qualified. Use the correct SSID-specific
credential on its next approved access; do not copy a derived C2 key across SSIDs.
