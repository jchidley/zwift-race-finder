# Zwift Race Finder

Find upcoming Zwift races that fit your schedule, using estimated durations and your
Racing Score to select the appropriate event subgroup. **Personal alpha software:**
predictions are experimental, not guaranteed finish times.

## Quick Start

Use **Bash on Linux or WSL2**, from the root of an existing checkout
(`~/git/zwift-race-finder` on the maintainer's machine). Building requires Rust and
Cargo via `rustup` (stable toolchain; no minimum Rust version is declared) and,
on Debian/Ubuntu, `libssl-dev` and `pkg-config`. See the
[setup tutorial](docs/tutorial/getting-started.md) for obtaining a checkout.

Searches read the public Zwift API and can initialize/migrate the local SQLite
database and create a user-stat cache. A fresh database has **no route metadata**;
only event-provided or heuristic estimates are available until you complete the
[client-only route import](docs/howto/CLIENT_ROUTE_DATA.md). No route dataset is
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
| Understand why | [Explanation](docs/explanation/) — Zwift physics, racing tactics, testing philosophy |

The main CLI is in `src/main.rs`, reusable modules in `src/lib.rs`, secondary
import/analysis binaries in `src/bin/`, and tests in `tests/`. Optional OCR tools
have separate [setup requirements](tools/ocr/SETUP_GUIDE.md) and need the `ocr`
Cargo feature plus native Leptonica/Tesseract libraries.

[Project history](docs/project-history/) and [PROJECT_WISDOM.md](PROJECT_WISDOM.md)
preserve discoveries and the dated learning log. `archive/` is historical evidence,
not current operating guidance.

## About This Code

Almost all of this code is AI/LLM-generated. It's best used as a source of
inspiration for your own AI/LLM efforts rather than as a traditional library.

**This is personal alpha software.** If you want to use it:

- **Pin to a specific commit** — don't track `main`, it changes without warning
- **Use AI/LLM to adapt** — without AI assistance, this project is hard to use
- **Treat as inspiration** — build your own version rather than depending on mine

Suggestions welcome as inspiration for future improvements.

## License

[MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE)
