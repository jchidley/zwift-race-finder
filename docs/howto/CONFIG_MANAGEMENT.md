# How to Configure Zwift Race Finder

Save non-secret defaults in TOML and use environment variables for secrets or temporary overrides.

## Configuration precedence

Environment overrides have highest priority. Otherwise the first existing project, user-config, or user-data TOML file wins. See the [Configuration Reference](../reference/CONFIGURATION.md) for the exact fields, defaults, environment names, and precedence.

## Create a user configuration

Prefer the user configuration directory:

```bash
mkdir -p ~/.config/zwift-race-finder
cp config.example.toml ~/.config/zwift-race-finder/config.toml
```

Edit the copied file. Supported sections include:

```toml
[defaults]
zwift_score = 250
category = "C"
weight_kg = 75.0
height_m = 1.75
ftp_watts = 250

[preferences]
default_duration = 60
default_tolerance = 15
default_days = 1

[display]
use_colors = true
debug = false

[import]
windows_username = "YOUR_USERNAME"
```

Weight, height, and FTP can be stored but are not direct inputs to the production duration estimator. See [Physical Stats and Duration Estimation](../reference/PHYSICAL_STATS.md).

## Apply temporary overrides

```bash
ZWIFT_SCORE=250 ZWIFT_DEFAULT_DURATION=90 zwift-race-finder
```

The [Configuration Reference](../reference/CONFIGURATION.md#environment-overrides) lists every supported variable. Command-line values override loaded duration, tolerance, days, and score when they differ from the CLI defaults. Because the current implementation infers whether some options were explicitly supplied by comparing them with default values, explicitly passing the default value may still allow the config preference to win.

## Configure ZwiftPower access

Do not put session or profile credentials in TOML. Load them from the environment:

- `ZWIFTPOWER_PROFILE_ID`
- `ZWIFTPOWER_SESSION_ID`

Use the reviewed local secrets process in [How to Set Up Secrets](SECRETS_SETUP.md).

## Use a project-local configuration

Create `config.toml` at the repository root only for project-specific testing. It overrides either user configuration location while the command runs from this checkout. Do not commit personal values.

## Troubleshoot configuration

- Invalid TOML makes `FullConfig::load()` fail; the main CLI warns and falls back to all built-in defaults.
- A valid but partial section may leave omitted optional fields unset. Copy `config.example.toml` when you want a complete starting point.
- If an unexpected file wins, check the three paths in precedence order and inspect relevant environment variables.
