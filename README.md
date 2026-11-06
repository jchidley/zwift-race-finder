# Zwift Racing

Find a race, prepare a plan, preserve predictions and review the result. The working
`zwift-race-finder` CLI selects upcoming events using personal duration estimates
and Racing Score subgroup geometry; planning and review are agent-assisted, not
a maintained coaching application. **Personal alpha software:**
predictions are experimental, not guaranteed finish times.

## Quick Start

Use **Bash on Linux or WSL2**, from the root of an existing checkout
(`~/git/zwift-racing` on the maintainer's machine). Building requires Rust and
Cargo via `rustup` (stable toolchain; no minimum Rust version is declared) and,
on Debian/Ubuntu, `libssl-dev` and `pkg-config`. See the
[setup tutorial](docs/tutorial/getting-started.md) for obtaining a checkout.

Searches read the public Zwift API and can initialize/migrate the local SQLite
database and create a user-stat cache. A fresh database has **no route metadata**;
fixed-time events can still match, but calculated personal estimates need
supported measured history and course geometry. Complete the
[client-only route import](docs/howto/CLIENT_ROUTE_DATA.md) and
[measured-history import](docs/howto/ZWIFTPOWER_EXPORT_STEPS.md). No route dataset is
bundled; existing database rows and race history are retained. Back up an existing
database before imports or schema upgrades.

Already built? Run either of the first two commands. Otherwise use the Cargo
command to build and run. These equivalent examples search for races estimated
to take **15–45 minutes**; replace Racing Score **195** with your own:

```bash
# Already built: short options
./target/release/zwift-race-finder -s 195 -d 30 -t 15

# Already built: long options
./target/release/zwift-race-finder --zwift-score 195 --duration 30 --tolerance 15

# Build if needed and run through Cargo
cargo run --release --bin zwift-race-finder -- --zwift-score 195 --duration 30 --tolerance 15
```

Success: the CLI reports the fetched event count and shows matching races with
start times and estimated durations, or explains that no events match. A live API
response is required; an empty result does not necessarily indicate a setup error.

### Optional PATH installation

To build separately and copy the executable to your user PATH directory (the copy
replaces any existing executable at that location):

```bash
cargo build --release
mkdir -p ~/.local/bin
cp target/release/zwift-race-finder ~/.local/bin/
```

With `~/.local/bin` on `PATH`, use
`zwift-race-finder -s 195 -d 30 -t 15` from any directory.

## Plan and review a race

From this same project session, supply an explicit path to an approved completed
activity outside Git and a question about the ride. Start with the
[first-review contract](docs/explanation/FIRST_RIDE_REVIEW.md): the first result
is a read-only field and timing-quality inventory, followed by a qualified review.
No live bridge, simulator, account login or coaching installation is required.

For an upcoming race, preserve a dated private plan and predictions before racing;
then compare the result and recording with those original expectations under the
[pacing and before/after standards](docs/explanation/PACING_AND_RACE_EXPERIMENTS.md).
Use the [shared-library review procedure](docs/howto/SHARED_LIBRARY_REVIEW.md)
for approved FitnessData inputs. Keep processing local and publication separate.

## Common commands

From the checkout root:

| Task | Command |
|------|---------|
| Show CLI options without fetching events | `./target/release/zwift-race-finder --help` |
| Show locally recorded unknown route IDs | `./target/release/zwift-race-finder --show-unknown-routes` |
| Explain filtering decisions | `./target/release/zwift-race-finder -s 195 -d 30 -t 15 --debug` |
| Build default-feature binaries | `cargo build --release` |

For development checks and test-state isolation, see [AGENTS.md](AGENTS.md), the
[testing guide](docs/howto/TESTING_GUIDE.md), and the
[test boundary reference](docs/reference/TEST_SUITE_SUMMARY.md). Duration-formula
changes require `cargo test --lib test_race_predictions_accuracy -- --nocapture`
and a before/after MAE on the same populated data, not an empty database.

### Export your latest ZwiftPower results

The exporter is **browser-console JavaScript, not a Bash or Node.js program**.
On the maintainer's WSL checkout, its full path is:

`/home/jack/git/zwift-racing/tools/import/zwiftpower/export_profile_history.js`

Display it from any Bash working directory (adjust the checkout path on another
machine):

```bash
cat /home/jack/git/zwift-racing/tools/import/zwiftpower/export_profile_history.js
```

1. Open **your own logged-in ZwiftPower profile/results page** and wait for the
   results table to load. Check that your latest race is listed.
2. Press **F12**, then select **Console**.
3. Copy the entire script shown by `cat`, paste it into the browser console,
   and press Enter. Do not paste the `cat` command itself.
4. The browser downloads `zwiftpower-history-<timestamp>.json`. Keep this private
   file outside Git; provide its full path when asking for analysis or import.

The script checks result ownership and that the complete table is loaded.
If it reports an error, do not bypass those checks. Exporting does not update the
finder's database. Follow the [backup, preview and import procedure](docs/howto/ZWIFTPOWER_EXPORT_STEPS.md)
when ready to apply it; do not share cookies or credentials.

## Configuration, data, and limitations

- Optional defaults: [config.example.toml](config.example.toml). On Linux, save
  non-secret settings to `~/.config/zwift-race-finder/config.toml`.
  [Configuration reference](docs/reference/CONFIGURATION.md) owns precedence:
  environment overrides, then the first existing file in the working directory,
  user config directory, or user data directory, then defaults. Files are not merged.
- Normal Linux storage: `~/.local/share/zwift-race-finder/races.db` and
  `~/.cache/zwift-race-finder/user_stats.json`. XDG overrides affect these paths.
- No credentials are required for a public event search. If configured, the
  optional ZwiftPower lookup can make an additional network request, even with
  `--zwift-score` supplied. Secrets come only from environment variables; see
  [secret setup](docs/howto/SECRETS_SETUP.md). Never commit credentials or private
  rider/client exports.
- The event API uses one regional endpoint with no failover and normally returns
  roughly 200 events (about 12 hours). Requesting more days does not guarantee
  coverage. Time trials need a declared duration to match a time window.
- Fixed-time events take priority. Supported ordinary races can use earlier
  personal history, including cross-course distance/climbing fitting. Unsupported
  durations remain unknown and are excluded from time-window matches; the CLI
  reports the omitted count. No category-speed duration is substituted. Racing Score selects subgroup geometry, not personal pace. See the
  [algorithm reference](docs/reference/ALGORITHMS.md) for qualifications.
- Duration filtering uses unrounded estimates; displayed durations round to whole
  minutes. An estimate just outside your window can round to its boundary but will
  still be excluded. Model evaluation uses unrounded predictions and measured seconds;
  see [evaluation instructions](docs/howto/TESTING_GUIDE.md#evaluate-saved-personal-predictions).
- Follow Zwift's Terms of Service when using or modifying this tool.

## Documentation and project map

Start at the [documentation map](docs/README.md) for maintained procedures and ownership.

| Need | Go to |
|------|-------|
| Resume current work | [Project status](STATUS.md) and [open tasks](TASKS.md) |
| Learn to use it | [Tutorial: Find your first race](docs/tutorial/getting-started.md) |
| Return after a break | [Generate a current Racing Score](docs/howto/RETURN_TO_RACING.md) |
| Import routes or measured history | [Client routes](docs/howto/CLIENT_ROUTE_DATA.md) and [data import](docs/howto/DATA_IMPORT.md) |
| Install, update, or troubleshoot setup | [Deployment guide](docs/howto/DEPLOYMENT.md) and [testing prerequisites](docs/howto/TESTING_GUIDE.md) |
| Change the implementation | [Agent operations](AGENTS.md), [code-truth map](docs/code-truth/README.md), and [architecture](docs/reference/ARCHITECTURE.md) |
| Look something up | [Reference](docs/reference/) — algorithms, architecture, database, configuration |
| Compare estimators | [Baseline and recency experiment](docs/project-history/RECENCY_BASELINE_2026-11-03.md) |

The main CLI is in `src/main.rs`, reusable modules in `src/lib.rs`, secondary
import/analysis binaries in `src/bin/`, and tests in `tests/`. Optional OCR tools
have separate [setup requirements](tools/ocr/SETUP_GUIDE.md) and need the `ocr`
Cargo feature plus native Leptonica/Tesseract libraries.

## Baseline and scope

`v0.3.0` preserves the usable personal-estimator baseline. Recency/current-form
experiments are compared against it; useful baseline acceptance does not stop
research or automatically promote a candidate. See [Status](STATUS.md) and
[Tasks](TASKS.md) for current work and evidence limits.

Race selection is implemented; planning and review currently use documented,
agent-assisted procedures. This is not a physics simulator or sensor application.
Fitness Bridge owns acquisition, transformations and BLE/ANT+ output; Life Fitness
owns hardware and physical control. Older investigations and learning logs are retrievable from Git at
`v0.3.0`; they are not current operating instructions. `archive/` is immutable
historical evidence. Pin a version when reproducing behaviour.

## License

[MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE). Imported Coach notices are
retained in [coach-licenses](docs/project-history/coach-licenses/); historical
Gymnasticon anchors retain their [upstream notice](tools/historical-coach/LICENSE-GYMNASTICON).
