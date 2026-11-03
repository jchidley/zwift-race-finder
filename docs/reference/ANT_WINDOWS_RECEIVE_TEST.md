# Native Windows ANT receive bench — 2026-10-30

## Result

**Gymnasticon on gym → ANT radio → native Windows receiver works** for the tested
Bicycle Power pages and controlled power/cadence steps. This is not a Zwift
pairing test, BLE test, exhaustive ANT profile conformance test or loss-free link.

| Part | Verified setup |
|---|---|
| Sender | Pi 3 A+, gym, Gymnasticon 1.4.0, ANT USB-m 0fcf:1009 |
| Sender profile | Bicycle Power, device number 11234, device type 11, transmission type 1; RF 57 / 2457 MHz; period 8182 |
| Receiver | Windows laptop, ANT USBStick2 0fcf:1008, USB instance serial 123 |
| Windows driver | Existing Dynastream 1.2.40.201, libusb-win32 device class |
| Windows stick response | AP2USB1.05; capability payload 08 03 00 ba 36 00 (8 channels, 3 networks) |
| Runtime | Existing uv-managed native Windows CPython 3.13.7; temporary uv dependency pyusb==1.3.1 |
| Transport | Existing System32 libusb0.dll; no USB/IP, WSL USB attachment or driver replacement |
| Client boundary | Only ZwiftLauncher observed, not a running Zwift game process |

This Windows stick differs in firmware from the NUC's USBStick2 (AJK1.04RAF).
Do not infer firmware from product ID alone.

## Tests executed

1. Open/reset/version/capability query on the Windows stick: succeeded.
2. Explicit zero command to gym over local UDP, then 10 s receive window:
   **37 standard power pages**, all **0 W / 0 rpm**, spanning approximately 9.49 s
   from first to last received page.
3. Parallel bounded command schedule and 26 s Windows receive window:
   **100 pages**, spanning approximately 25.22 s from first to last page.

| Command UTC on gym | Command | First observed UTC on Windows |
|---|---|---|
| 21:16:51.687 | 0 W / 0 rpm | 21:16:54.619 (receiver acquired signal after start) |
| 21:16:56.661 | 150 W / 80 rpm | 21:16:57.864 |
| 21:17:02.657 | 250 W / 95 rpm | 21:17:03.856 |
| 21:17:08.663 | 0 W / 0 rpm | 21:17:09.848 |

The roughly 1.2 s transition differences are observations from different hosts'
wall clocks, **not calibrated latency measurements**. Clock synchronization and
offset were not independently verified. Receiver records also contain its own
monotonic timestamps.

### Counters and loss

- 90 consecutive same-value pairs with event-count delta 1 had the expected
  accumulated-power increment modulo 65536. **Zero mismatches** in those checks.
- Four missing event counts across the step capture; do not call the link
  loss-free or assume their cause. No RF/error-location diagnosis was performed.
- No event-counter wrap occurred in that capture. Full rollover, long-duration
  reliability, additional pages and unavailable-value wire behavior remain open.

### Freshness limitation observed

Each positive prescription was sent once, with six seconds of silence until
the next command. The receiver continued seeing the positive value.
This verifies hold-last-value behavior under command silence, **not a working
producer watchdog**. It is not a process-crash or sensor-disconnect test.

A scheduled zero reset and remote shell exit trap sent zero at completion.
Post-test independent gym logs confirmed **0 W / 0 rpm**. Service stayed active
with NRestarts=2, unchanged from the earlier Bluetooth startup failures.

Cadence zero was set by an explicit temporary command, not a persistent config
fix. A future service restart can still begin with unspecified bot cadence.

## Diagnostic and checks

Source: [scripts/ant_power_receive.py](../../scripts/ant_power_receive.py).

The script opens one receive-only channel for the selected device number,
validates framing/checksums, decodes standard power pages, and closes/unassigns
the channel and releases USB in cleanup. It has no ANT broadcast/acknowledged/
burst data command path. It resets the receiver stick; do not use concurrently
with another application.

Initial receive attempt exposed a legacy-backend timeout quirk: libusb0 returns
generic USBError with backend_error_code=-116 and errno=None. A diagnostic
confirmed that error code; only that code and USBTimeoutError are treated as
quiet reads. The initial attempt released USB; the repeat succeeded.
This is not a driver install/change.

Self-checks cover command framing, fragmented/concatenated frames, checksum
corruption rejection, power/cadence parsing and zero versus unavailable values.
They use synthetic bytes, not personal recordings. Ruff checks and self-checks
were run; live tests establish the exercised Windows path separately.

Example native PowerShell 7 invocation (after selecting the installed managed
runtime and confirming no Zwift game or other stick consumer is running):

```powershell
$python = Join-Path $env:APPDATA 'uv\python\cpython-3.13.7-windows-x86_64-none\python.exe'
$script = '\\wsl.localhost\Debian4\home\jack\git\zwift-race-coach\scripts\ant_power_receive.py'
uv.exe run --no-project --python $python --with pyusb==1.3.1 python $script --seconds 10
```

Use the reviewed ps-exec helper when calling PowerShell from WSL; do not
substitute system Python, change drivers or launch Zwift as part of this command.
Runtime paths are evidence of this machine, not a portable installation recipe.

Private raw step results:
`~/.local/share/zwift-race-coach/bench-ant-2026-10-30/step-results.json`.
Keep generated captures outside Git. This document contains only test conditions
and summarized evidence, not a committed recording.

## Subsequent watchdog change

The [later installed watchdog](GYMNASTICON_WATCHDOG.md) supersedes the startup
cadence/freshness gaps for the running 1.4.0 bot. The initial hold-last-value
findings above remain historical evidence. Follow the later note for measured
timeouts, changed complete-sample input contract and end-to-end timing limits.
The receiver now also has an explicit bounded UDP watchdog-exercise mode;
ordinary receive mode remains ANT receive-only.

## Final state and next work

- Windows ANT device still reports OK with its original driver class.
- Receiver resources released; no process left holding its channel.
- Gym continues its existing zero-output service; no service restart or config
  change was made during tests.
- Positive synthetic output was exercised only in this local bench test,
  without a running Windows Zwift game client.
- Next: explicit startup cadence, fail-safe source expiry and controlled BLE
  reception/parity before multi-client or strategy experiments. Actual Windows
  Zwift pairing remains a separate test; do not extend results to public racing.
