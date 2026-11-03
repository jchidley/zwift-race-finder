# Configuration Reference

`src/config.rs` defines non-secret TOML configuration, environment overrides, and precedence. `config.example.toml` is the copyable example.

## Precedence

1. Supported environment variables override loaded fields.
2. The first existing file in this order is loaded:
   1. `./config.toml`
   2. the platform user config directory plus `zwift-race-finder/config.toml`
   3. the platform user data directory plus `zwift-race-finder/config.toml`
3. Built-in defaults fill a missing file or sections that use Serde defaults.

Configuration files are not layered or merged.

## TOML sections

| Section | Field | Type | Built-in default | Production use |
|---|---|---:|---:|---|
| `defaults` | `zwift_score` | optional integer | `195` | Used when CLI/cache/ZwiftPower do not provide a score |
| `defaults` | `category` | optional string | `"D"` | Fallback displayed category |
| `preferences` | `default_duration` | optional integer | `120` | Search duration in minutes |
| `preferences` | `default_tolerance` | optional integer | `30` | Duration tolerance in minutes |
| `preferences` | `default_days` | optional integer | `1` | Requested search horizon |

The current main CLI explicitly applies config preferences for duration, tolerance, and days only when the parsed CLI value equals that option's built-in default. It cannot distinguish “not supplied” from “explicitly supplied with the default value.”

## Environment overrides

| Variable | Field |
|---|---|
| `ZWIFT_SCORE` | `defaults.zwift_score` |
| `ZWIFT_CATEGORY` | `defaults.category` |
| `ZWIFT_DEFAULT_DURATION` | `preferences.default_duration` |
| `ZWIFT_DEFAULT_TOLERANCE` | `preferences.default_tolerance` |
| `ZWIFT_DEFAULT_DAYS` | `preferences.default_days` |

Values that fail numeric parsing are ignored.

## Secrets

`FullConfig` loads these separately from the environment; they are not TOML fields:

- `ZWIFTPOWER_PROFILE_ID`
- `ZWIFTPOWER_SESSION_ID`

`ZRF_ZP_DEBUG` enables additional ZwiftPower parsing diagnostics when present.

## Paths

On Linux, the usual resolved paths are:

- user config: `~/.config/zwift-race-finder/config.toml`
- user data config: `~/.local/share/zwift-race-finder/config.toml`

Other platforms use paths returned by the Rust `dirs` crate.

Use `--debug` for filtering diagnostics and `NO_COLOR` to disable colour. Weight,
height and FTP are not estimator configuration inputs.
