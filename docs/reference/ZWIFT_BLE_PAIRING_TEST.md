# Windows Zwift BLE pairing test — 2026-10-30

Historical Coach evidence retained during consolidation. Current state and ownership
are in the root [status](../../STATUS.md) and [documentation map](../README.md). Dated claims below are not current acceptance.

## Verified result

After the owner switched both Power Source and Cadence to BLE, direct inspection
of Zwift's **pre-ride Paired Devices screen** confirmed the Bluetooth icons,
**Gymnasticon** name and green CONNECTED indicators for both fields.

Repeated the same 150 W / 85 rpm prescription used in the
[ANT pairing test](ZWIFT_ANT_PAIRING_TEST.md). Inspected all three captured windows:

| Windows UTC timestamp after capture save | Power | Cadence |
|---|---:|---:|
| 22:42:30.416 | 0 W | 0 rpm |
| 22:42:33.969 | 150 W | 85 rpm |
| 22:42:44.887 | 0 W | 0 rpm |

The foreground-guarded Windows UDP producer sent full samples at roughly 500 ms
intervals for eight seconds, then stopped at 22:42:38.556. The zero capture was
saved about 6.33 seconds later, **before** the explicit cleanup zero sent at
22:42:44.893. Thus the observed return to zero followed command silence, not the
cleanup command. Both fields remained connected in the zero capture.

This is a snapshot check, not a measured BLE notification rate, exact display
latency, or a 3-second client-display guarantee. The source-side
[watchdog timing and limitations](GYMNASTICON_WATCHDOG.md) still apply.

No OK/Ride/event action was taken, no ride was started, and no physical resistance
control was paired or commanded. Final independent gym logs confirmed zero power
and cadence, active service, MainPID 2665 and NRestarts=0. Zwift was left on Paired
Devices with BLE selected. No service restart or implementation change was needed.

## Comparison and limits

ANT+ and BLE each displayed the prescribed **150 W / 85 rpm**, then returned to
zero on command silence, on the same Windows client. This establishes **basic
sequential displayed-value parity**, not simultaneous transport reception or
full profile conformance. The ANT identity is Powermeter 11234; BLE displays the
Gymnasticon name. A Bluetooth address/service dump was not captured, so the
name alone is not a globally unique identity.

Running profiles, FTMS/resistance control, independent GATT decoding, long-duration
reliability, radio disconnect/reconnect, rollover, multiple-client/identity
isolation, real sensor input and in-world physics remain unqualified. SkiErg
qualification remains parked until equipment is available.

## Subsequent hardware move

The owner later shut down gym and reported it running on new hardware. A directly
inspected pre-ride Zwift capture showed Gymnasticon connected over BLE for both
power/cadence at 0 W / 0 rpm. The board model and IP were not independently
established: DNS/router lease still identified the previous 10.0.1.148 entry,
and strict SSH returned No route to host. No positive pulse was sent after the
move, so the full test above must not be attributed to the new hardware yet.
Source control/network access is needed to repeat it. This observation is a
connected zero baseline, not independent verification of notification freshness.

## Pi Zero 2 W retest — passed

The owner supplied the new address 10.0.1.61. Strict SSH using the existing gym
host-key alias verified `Raspberry Pi Zero 2 Rev 1.0`, active service PID 583,
and the same patched bot/helper hashes recorded in the watchdog note. Persistent
bot power/cadence remain zero. No trust bypass, config change or reinstall occurred.

A fresh pre-ride BLE test sent directly from native Windows to 10.0.1.61:3000
produced directly inspected captures:

| Windows UTC timestamp after capture save | Power | Cadence |
|---|---:|---:|
| 23:07:56.556 | 0 W | 0 rpm |
| 23:08:00.204 | 150 W | 85 rpm |
| 23:08:11.161 | 0 W | 0 rpm |

Both fields showed Gymnasticon Bluetooth icons and CONNECTED indicators.
Heartbeats stopped at 23:08:04.822; the zero capture preceded explicit cleanup
zero at 23:08:11.166. This repeats the functional pulse/expiry result on the new
hardware, not a precise client timeout measurement. No ride was entered.

WSL SSH reached the board, then returned No route to host during the next check;
the direct Windows UDP-to-BLE test still passed. A later SSH retry succeeded.
The network/WSL cause was not isolated, and no network settings were changed.
At roughly ten minutes uptime, PID 583 and NRestarts=2 remained unchanged; source
logs independently confirmed zero and `get_throttled` reported 0x0.

Boot logs explain two initial automatic restarts: first ENODEV, then Bluetooth
adapter poweredOff, followed by successful operation. The startup-readiness race
therefore remains unresolved on this board. Boot also reported **no ANT+ stick
found**; ANT output has not been requalified on the Zero 2. Do not carry the old
Pi 3 A+ ANT result over to this board.

Window captures remain private in Windows Temp: `zwift-zero2-ble-baseline.png`,
`zwift-zero2-ble-positive.png`, `zwift-zero2-ble-expired.png`. Raw producer/capture
output is saved privately as `zwift-zero2-ble.json` alongside the prior bench
records. No generated captures were added to Git.

## Private evidence

Directly inspected window captures remain in Windows user Temp, outside Git:

- `zwift-ble-before.png` (initial pairing inspection)
- `zwift-ble-baseline.png`
- `zwift-ble-positive.png`
- `zwift-ble-expired.png`

Raw command output:
`~/.local/share/zwift-race-coach/bench-ant-2026-10-30/zwift-ble-pairing.json`.
No screenshots, personal profiles or generated recordings were copied into Git.
