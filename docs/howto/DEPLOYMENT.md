# Install and update

Use Bash on Linux/WSL from an existing checkout root. Build prerequisites and the
shortest useful search are in [Quick Start](../../README.md#quick-start). This is a
personal CLI, not a deployed service. The versioned baseline is `v0.3.0`; pin the
intended source version when reproducing predictions.

## Install a built version

This replaces an existing user-local binary:

```bash
cargo build --release --bin zwift-race-finder
mkdir -p ~/.local/bin
cp target/release/zwift-race-finder ~/.local/bin/
```

With `~/.local/bin` on PATH, verify the installed executable:

```bash
zwift-race-finder --version
zwift-race-finder --help
```

Updating source/builds does not import history or refresh client metadata. Back up
an existing database before any operation that can initialize/migrate schema or
apply data. Follow [client routes](CLIENT_ROUTE_DATA.md) and
[measured history](ZWIFTPOWER_EXPORT_STEPS.md) for preview/reconciliation.

## Configure and search

[Configuration](CONFIG_MANAGEMENT.md) owns defaults and precedence. Secrets come
only from environment variables; no credentials are needed for public events.
A normal search calls live services and may create/migrate local state:

```bash
zwift-race-finder --zwift-score 195 --duration 45 --tolerance 5
```

Replace the score with your own. Success is a list of matching events or an
explanation of exclusions/no matches. Personal estimates need supported measured
history and geometry; fixed durations remain usable without those inputs.

## Diagnose results

- Use `--debug` for exclusion reasons and `--show-unknown-routes` for metadata gaps.
- The public endpoint has limited event coverage; increasing `--days` cannot force
  it to return more events. No endpoint failover is implemented.
- Check subgroup distance/laps and input history before changing a formula.
  Recordings, titles and moving time do not establish race finish observations.
- Judge predictions against race-only outcomes, not full activity duration. See
  [Algorithms](../reference/ALGORITHMS.md) and the
  [baseline comparison](../project-history/RECENCY_BASELINE_2026-11-03.md).
- Build/test success and owner acceptance do not establish clean release gates.
  [Status](../../STATUS.md) records current limits; [testing](TESTING_GUIDE.md)
  describes validation without touching normal user state.
