# Tutorial: Find Your First Zwift Race

Find a Zwift race that fits your schedule by predicting how long it will take you to finish.

## Before you start

You need:
- Linux or WSL2 (Windows Subsystem for Linux)
- Rust toolchain (`rustup` installed)
- Build dependencies: `sudo apt-get install -y libssl-dev pkg-config`
- Your Zwift Racing Score (find it on [ZwiftPower](https://zwiftpower.com) or in the Zwift Companion app)

## Step 1: Install

Clone and build the tool:

```bash
git clone https://github.com/jchidley/zwift-race-finder.git
cd zwift-race-finder
cargo build --release
cp target/release/zwift-race-finder ~/.local/bin/
```

You should see:

```
Finished `release` profile [optimized] target(s) in ...
```

## Step 2: Find races around 30 minutes

Run the tool with your Racing Score. We'll use 195 as an example (Category D):

```bash
zwift-race-finder --zwift-score 195 --duration 30 --tolerance 15
```

You should see a table of upcoming Zwift races estimated to last 15–45 minutes for a rider with your score. Each row shows the event name, start time, distance, and predicted duration.

If you see "No events found", try widening the search:

```bash
zwift-race-finder --zwift-score 195 --duration 60 --tolerance 30
```

## Step 3: Try different event types

By default, only races are shown. See all event types:

```bash
zwift-race-finder --zwift-score 195 --duration 60 --tolerance 30 --event-type all
```

Notice the event type summary at the top — it tells you how many races, group rides, fondos, and workouts were found.

## Step 4: Look further ahead

Search the next 3 days instead of just today:

```bash
zwift-race-finder --zwift-score 195 --duration 30 --tolerance 15 --days 3
```

Note: Zwift's API returns a maximum of 200 events (~12 hours). Multi-day searches may not cover the full range.

## Step 5: Save your defaults

Create a config file so you don't have to type your score every time:

```bash
mkdir -p ~/.config/zwift-race-finder
cp config.example.toml ~/.config/zwift-race-finder/config.toml
```

Edit `~/.config/zwift-race-finder/config.toml` and set your Racing Score, weight, and preferred duration.

Now you can run with just:

```bash
zwift-race-finder
```

## What next

- [How to import race data from Strava](../howto/DATA_IMPORT.md) — improve prediction accuracy with your actual race times
- [How to deploy and update](../howto/DEPLOYMENT.md) — install the tool permanently
- [Algorithm reference](../reference/ALGORITHMS.md) — understand how duration predictions work
- [About Zwift racing tactics](../explanation/ZWIFT_RACING_TACTICS.md) — pack dynamics, positioning, attack timing
