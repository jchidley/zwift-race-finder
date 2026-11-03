# Find your first race

The finder uses your Racing Score to select an event subgroup/course, then uses
earlier personal race results to estimate duration. Fixed-time events use their
declared duration. Unknown estimates are omitted rather than filled with category
speeds. See [Quick Start](../../README.md#quick-start) for the shortest command path.

## Prepare

Use Bash on Linux/WSL. If you need a checkout/build, install stable Rust with
`rustup`; Debian/Ubuntu also need `libssl-dev` and `pkg-config`. Installation of
missing system packages is a separate privileged action.

```bash
git clone https://github.com/jchidley/zwift-race-finder.git
cd zwift-race-finder
cargo build --release --bin zwift-race-finder
```

A search calls the live API and can initialize/migrate local state. A fresh database
has no route metadata or measured history. Back up existing state and follow the
[client-route](../howto/CLIENT_ROUTE_DATA.md) and
[measured-history](../howto/ZWIFTPOWER_EXPORT_STEPS.md) procedures for personal
calculated estimates. Until then, fixed-duration events can still match.

## Search by available time

Replace the example score with your current score:

```bash
./target/release/zwift-race-finder --zwift-score 195 --duration 45 --tolerance 5
```

Success is a table of events estimated to take 40–50 minutes, or a clear no-match
result with unknown-duration omissions disclosed. `--debug` explains exclusions;
`--verbose` shows method/support. Score selects geometry, not your personal pace.
Increasing `--days` cannot extend the live API's limited event coverage.

## Save defaults or install on PATH

[Configuration](../howto/CONFIG_MANAGEMENT.md) covers score and search preferences;
weight, height and FTP are not model settings. [Installation](../howto/DEPLOYMENT.md)
covers copying the built executable to your user PATH. Public searches require no
credentials; [optional lookup credentials](../howto/SECRETS_SETUP.md) are separate.

## Compare predictions

Use race-only finishes, not saved activity durations that include other riding.
The [algorithm reference](../reference/ALGORITHMS.md) explains the active model;
the [baseline/recency comparison](../project-history/RECENCY_BASELINE_2026-11-03.md)
records measured examples and their limitations. Importing additional history may
change predictions without proving improved accuracy. [Status](../../STATUS.md)
and [Tasks](../../TASKS.md) identify current work.
