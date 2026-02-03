# AGENTS.md

## Overview
Zwift Race Finder predicts race durations from Zwift event data and a rider's Racing Score, then filters events by target duration.

## Commands
| Task | Command |
| --- | --- |
| Build release | `cargo build --release` |
| Run CLI (example) | `zwift-race-finder --zwift-score 195 --duration 30 --tolerance 15` |
| Show unknown routes | `zwift-race-finder --show-unknown-routes` |
| Import routes | `./scripts/import_routes.sh` |

## Zwift Invariants
- Always use `route_id` as the identifier. Route names change.
- Always include lead-in distance (0.2-5.7km) in total time calculations.
- Drop state is binary: in-pack or dropped.
- Pack-speed constants are empirical; change only with new regression evidence.
- Preserve duration formula behavior unless regression tests prove an improvement.

## Testing
| Purpose | Command |
| --- | --- |
| All tests | `cargo test` |
| Zwift API tests | `cargo test zwift_api` |
| Duration logic | `cargo test duration` |
| Regression suite (151 races) | `cargo test --test regression_tests -- --nocapture` |

## Troubleshooting
- Route not found: run `zwift-race-finder --show-unknown-routes`, find the `route_id` on ZwiftHacks, update `data/route_manifest.json`.
- Time looks wrong: verify lead-in distance and correct Racing Score input.

