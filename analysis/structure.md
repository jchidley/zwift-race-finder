# Structure Map + Dependencies + Clusters

## Scope
- Target: library crate + main binary (excluding `src/bin/`, OCR modules)
- 24 core files, ~9,500 LOC total (~5,500 production)
- 8 entry points (1 primary, 7 secondary binaries)
- Build: `cargo build --release`
- Test: `cargo test` (148 tests: 89 lib + 36 bin + 12 integration + 11 property)

## Production LOC by file (>100 only)

| File | Prod LOC | Test LOC | Pub fns | Notes |
|------|----------|----------|---------|-------|
| database.rs | 868 | 73 | 22 | SQLite ops — route CRUD, race results, discovery |
| event_display.rs | 804 | 475 | 12 | Verbose + table output formatting |
| main.rs | 629 | 888 | 0 | CLI orchestration, filter_events, 26 tests |
| route_discovery.rs | 573 | 193 | 4 | Web scraping for unknown routes |
| commands.rs | 431 | 0 | 6 | CLI subcommand handlers |
| config.rs | 334 | 0 | 14 | Config file loading, secrets |
| compatibility.rs | 308 | 0 | 10 | **DEAD MODULE — never imported** |
| event_filtering.rs | 292 | 541 | 9 | Duration matching, sport/time/tag filters |
| secure_storage.rs | 251 | 91 | 6 | **DEAD MODULE — never imported by production code** |
| errors.rs | 173 | 0 | 15 | User-facing error messages |
| zwift_offline_client.rs | 165 | 0 | 7 | Zwift-offline API client |
| parsing.rs | 137 | 188 | 5 | Distance/description parsing |
| ab_testing.rs | 133 | 64 | 6 | A/B test framework (used only by tests/) |
| zwiftpower.rs | 127 | 0 | 2 | ZwiftPower stats fetch |
| estimation.rs | 112 | 32 | 4 | Route lookup + duration bridge |
| category.rs | 107 | 107 | 4 | Score→category, category speeds |
| duration_estimation.rs | 100 | 229 | 4 | Core prediction algorithm |

## Dependency Graph (non-trivial)

```
main.rs → api, commands, zwiftpower, config, database, route_discovery
         + lib: category, constants, errors, estimation, event_analysis,
                event_display, event_filtering, formatting, models

commands.rs → api, database, route_discovery
             + lib: constants, estimation, formatting

zwiftpower.rs → config
               + lib: cache, category, models

event_display.rs → lib: category, constants, database, duration_estimation,
                        estimation, event_analysis, event_filtering,
                        formatting, models, parsing, route_discovery

event_filtering.rs → lib: category, constants, database, duration_estimation,
                          estimation, event_analysis, models, parsing

estimation.rs → lib: category, constants, database, duration_estimation, models

duration_estimation.rs → lib: category, constants

route_discovery.rs → (self-contained, uses reqwest)
```

## God Functions (>5 unique callees)

| Function | File | Callees |
|----------|------|---------|
| `event_display::prepare_event_row` | event_display.rs | ~10 (estimation, duration, category, formatting, parsing, event_analysis) |
| `event_filtering::event_matches_duration` | event_filtering.rs | ~8 (estimation, duration, category, parsing, event_analysis) |
| `event_display::display_duration_info` | event_display.rs | ~7 (calls 4 sub-display functions that each call estimation) |

## Clusters

| Cluster | Files | Cohesion | Notes |
|---------|-------|----------|-------|
| **core-estimation** | duration_estimation, estimation, category, constants | High | Pure computation, well-tested |
| **event-pipeline** | event_filtering, event_display, event_analysis, formatting, models, parsing | Medium | Filtering + display, heavily interconnected |
| **data** | database, config, cache | Medium | SQLite + config + cache |
| **cli** | main, commands, api, zwiftpower, errors | Medium | Binary-side orchestration |
| **discovery** | route_discovery, zwift_offline_client | High | Web scraping, self-contained |
| **framework** | ab_testing, compatibility, secure_storage, test_db, test_utils | Low | Utility/test infrastructure, some dead |
