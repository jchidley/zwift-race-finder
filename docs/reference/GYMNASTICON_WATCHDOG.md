# Gymnasticon startup zero and receive-side watchdog

Implemented on gym on 2026-10-30 after the
[initial ANT bench](ANT_WINDOWS_RECEIVE_TEST.md).

## Installed change

The installed Gymnasticon **1.4.0** bot now uses a small local input helper.
The separate 1.4.1 checkout was not changed. No replacement bridge, daemon,
controller, radio driver, package upgrade or competing serial reader was added.

- Persistent config: `bot-power: 0`, **`bot-cadence: 0`**.
- Bot adapter independently forces initial state to **0 W / 0 rpm**, even if
  passed nonzero initial arguments; a positive command must arrive over UDP.
- Complete JSON samples required: `{"power":180,"cadence":85}`.
- Power: integer 0..32767 W (shared ANT/BLE safe range; BLE encodes signed int16);
  cadence: integer 0..254 rpm. Protocol unavailable
  sentinels, negatives, fractional values, strings, malformed JSON, null, arrays,
  and partial updates are rejected, not silently clamped.
- No smoothing, interpolation, gain or randomization added.
- A valid complete packet refreshes arrival freshness, including an explicit
  zero packet. Invalid packets do not modify values or refresh that deadline.
- Expiry is **2500 ms since local acceptance**, using a monotonic clock.
  A 100 ms poll immediately emits zero on expiry. Periodic stats also recheck age.
  The 500 ms budget before the 3 s acceptance target allows polling and ANT
  scheduling, but is not a hard-real-time guarantee.
- A valid fresh sample can recover output after expiry.
- The socket-error handler publishes zero to the application state and cancels
  bot timers before the existing disconnect event. On-air behavior during
  process exit/restart remains unqualified.

Partial UDP updates were deliberately removed: updating power alone must not
refresh an old cadence (or vice versa). Producers must send both fields, normally
at least once per second; the bench used 500 ms heartbeats. A once-only positive
prescription no longer persists indefinitely.

UDP still binds the pre-existing `0.0.0.0:3000` endpoint and has no authentication.
This change does not secure it against another sender or replay. Do not expose
it to an untrusted network or treat it as a production control API.

## Deployment and rollback evidence

New original source files in this repository:

- [Input state/validation/expiry](../../scripts/gymnasticon_bot_input.cjs)
- [Candidate generator for inspected installed bot](../../scripts/patch_gymnasticon_bot.cjs)
- [Deterministic state tests](../../scripts/test_gymnasticon_bot_input.cjs)
- [Radio-free adapter integration tests](../../scripts/test_gymnasticon_bot_adapter.cjs)

The generator produces a candidate rather than overwriting its source, and
rejects an already-patched or structurally unexpected bot. The actual deployment
also required an exact original-file SHA-256 match. No full third-party bot source
was copied into this repository. This local adaptation does not change the
upstream application's license or transfer third-party rights.

On gym:

- Adapter: `/opt/gymnasticon/node/lib/node_modules/gymnasticon/lib/bikes/bot.js`
- Helper: same directory, `bot-input.cjs`
- Config: `/etc/gymnasticon.json`
- Private original bot/config backup:
  `/var/lib/gymnasticon-watchdog-backup.TZJkfl/`
- Staged candidate/tests: `/tmp/gym-watchdog.h0cJ7L/` (temporary, not durable storage)

SHA-256:

| File | Hash |
|---|---|
| Original installed bot | ec4d93a69372506b3724481923caa3ba13ec3c243dfb80450c0c6e3fd450cc46 |
| Installed patched bot | 60655ee7b5299246434fa407d6296f5979c82f7ce075c3d09cc2769fec3794b2 |
| Installed helper | 2b1d762bd5c83f148434fef9c2772049a7d7703635648f22420959f9933855ba |

Backups were taken before stopping the service. Candidate/config/helper exact
readback and Node syntax checks passed. Only Gymnasticon was stopped/started.
Post-start logs show explicit zero cadence/power, ANT startup and no automatic
restart (final MainPID 2665, NRestarts=0). This counter reset follows the intentional
service stop/start; it does not erase the earlier boot-readiness race.

Rollback, if needed: stop Gymnasticon, restore `bot.js.before` and
`gymnasticon.json.before` from the private backup to their original paths while
preserving metadata, then start it. The unused helper can remain inert.
**Rollback removes the watchdog and restores unspecified startup cadence.**
Do not do it during a positive-output experiment. A package reinstall/update can
also overwrite the adapter; re-audit rather than assuming the protection remains.

## Validation

### Deterministic and radio-free checks

Passed on gym's actual **Node v12.18.3**, before installation:

- Startup zero, valid samples and wire-range boundaries.
- Just-before versus exactly-at expiry; invalid traffic cannot extend the lease.
- Complete-sample requirement, malformed/null/array/string/negative/fractional/
  unavailable-value rejection.
- Heartbeat refresh, explicit zero and recovery.
- Adapter immediate emission, watchdog publication independent of periodic stats,
  and socket-error state-zero/timer cleanup.

The adapter test executes the generated/installed adapter with a fake UDP socket,
fake timer and controlled monotonic clock, without loading radio backends.
Local input tests, Python receiver self-checks and Ruff checks also passed.

### Independent Windows ANT observation

Used the same existing native Windows stick/driver as the initial bench; no
running Zwift game process was found before positive output.

The receiver's explicit `--watchdog-test-host gym` mode uses a fixed bounded UDP
exercise, records commands and received pages on **one Windows monotonic clock**,
and attempts zero in cleanup. It waits for an actual power page before starting
the schedule; acquisition has its own 20 s bound. The schedule tests malformed/
partial traffic, null/out-of-range traffic, periodic heartbeats, silence,
recovery and explicit zero.

A preliminary run missed the first pulse because reception had not acquired.
It verified later cases but is not evidence for the missed case. The corrected
acquisition-gated run received **110 pages** over approximately **27.96 s**, with
all expected pairs: 0/0, 180/85, 220/90, 150/80 and 120/70.

| Case | Windows command-send → observed ANT zero | gym accepted sample → published zero |
|---|---:|---:|
| Single 180/85, malformed and partial packets afterward | 2.746 s | 2.520 s |
| Single 220/90, null and unavailable-cadence packets afterward | **3.745 s** | 2.593 s |
| 150/80 heartbeats, then producer stops | 2.746 s from last send | 2.553 s from last acceptance |

The source-side figures use gym's monotonic journal timestamps of immediate
valid-sample publication and subsequent zero publication; no cross-host clock
subtraction is used. For the heartbeat case, the last accepted heartbeat is the
non-periodic stats entry at journal monotonic 2122.862947 s; zero is 2125.416378 s.
These are measured application/log timings, not a hard-real-time proof.

The on-air captures used the first watchdog revision (PID 2202). A final audit
then found the installed BLE encoder's signed int16 power field and narrowed
accepted power from the ANT-only ceiling of 65534 to 32767 W. Input boundary tests,
installed-adapter tests and startup checks were repeated after that scoped
revision (PID 2665). The timer and all exercised low-power sample behavior were
unchanged; the full on-air sequence was not repeated merely for that range fix.
Original and first-watchdog backups were retained separately.

**Do not summarize the entire Windows-send-to-zero test as passing 3 s.**
One case exceeded it. The watchdog's acceptance-to-zero timing passed locally,
while command delivery/dispatch contributed extra delay upstream of acceptance;
the exact network/OS cause was not isolated. ANT first-positive-to-first-zero
spans for the two single-pulse cases were approximately 2.497 s and 2.746 s.

Twelve received pages strictly inside the heartbeat-maintained window were all
150 W / 80 rpm. Subsequent fresh input recovered to 120 W / 70 rpm. Explicit zero
then appeared on ANT about 0.25 s after its Windows send. Final independent gym
logs confirmed zero, active service and no additional automatic restarts.

Private raw diagnostics, including the inconclusive first attempt and the
send-side deadline failure, are under:
`~/.local/share/zwift-race-coach/bench-ant-2026-10-30/`.
Generated captures remain outside Git.

## Boundaries and remaining work

This fixes **receive-side arrival freshness while the bot process is running**.
The legacy two-field protocol has no producer timestamp or sequence number:
delayed/replayed packets or another sender can refresh the lease. End-to-end
producer-age guarantees require a separately designed input contract, not a
smaller timer chosen just to pass this capture.

No hard-real-time, frozen-host, process-crash, full-network-queue, power-loss or
radio-firmware fail-safe has been established. A stalled/dead process cannot
execute this software watchdog. No host reboot was requested or tested.

Startup zero was verified through service restart, not full cold boot. BLE
receives the same application state changes in code but was not independently
received/tested. Zwift pairing, BLE parity, radio counter rollover, source
provenance recording, identity isolation and physical/client safety remain
separate qualification tasks. Synthetic output remains for controlled
experiments, never public competition.
