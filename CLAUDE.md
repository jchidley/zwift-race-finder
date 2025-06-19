# CLAUDE.md - Zwift Race Finder

Behavioral instructions for safe AI assistance on this project.

## Core Principle

This project uses empirical data from 151 real races. All constants and behaviors are validated against actual race results.

## Zwift-Specific System Invariants

### Racing Algorithm Constants (From 151 Races)
- Pack Speed Category D: 30.9 km/h
- Solo Speed: 77% of pack speed  
- Drop State: BINARY (with pack OR dropped)
- Current Accuracy: 23.6% MAE

These constants are empirically validated. See parent CLAUDE.md for invariant handling rules.

### Why Drop State is Binary
Zwift's draft physics: You're either in the blob (30.9 km/h) or alone (23.8 km/h).
No gradual transition exists in the game engine.

### Behavioral Rules
1. ALWAYS use route_id as identifier (route names change)
2. ALWAYS include lead-in distance (0.2-5.7km) in total calculations
3. ALWAYS preserve exact duration formula behavior
4. ALWAYS run regression tests before claiming improvements
5. NEVER add complexity without regression evidence

### Racing Score Events
- Identified by: `distanceInMeters: 0` AND `rangeAccessLabel` field present
- Distance must be parsed from description text

## Testing Requirements

For Zwift-specific logic:
```bash
# Zwift-specific test commands:
cargo test zwift_api  # Test API integration
cargo test duration   # Test duration calculations
cargo test --test regression_tests -- --nocapture  # Verify against 151 races
```

Run regression tests in `tests/regression_tests.rs` before claiming any improvement to duration estimation.

## When Users Ask About...

**"Why is my time wrong?"**: 
- Check: Did you include --include-lead-in? (adds 0.2-5.7km)
- Check: Is your Racing Score current? (--zwift-score parameter)
- Show: Current accuracy is 23.6% MAE from 151 races

**"Route not found"**:
1. Run with --show-unknown-routes
2. Find route_id on ZwiftHacks.com/app/routes
3. Add to data/route_manifest.json using route_id (never name)

**"Import my data"**:
- ZwiftPower: See docs/guides/DATA_IMPORT.md#zwiftpower
- Route updates: Use scripts/import_routes.sh

**Running with their Racing Score**: Explain --zwift-score parameter with --duration and --tolerance
**Updating personal stats**: Direct to update_rider_stats.sh script

## Communication Standards

- When changing behavior: Cite specific regression test evidence
- When discussing times: Always clarify if including lead-in distance
- When logging work: Append to ZWIFT_API_LOG_RECENT.md
- When referencing domain concepts: Ensure accuracy per docs/reference/ZWIFT_DOMAIN.md

## Zwift-Specific Anti-Patterns

❌ Using route names as identifiers (use route_id)
❌ Ignoring lead-in distance in time calculations
❌ Modifying pack speed constants without new race data
❌ Assuming gradual drops (Zwift's draft is binary)

## Zwift-Specific Stop Signals

🛑 Changing pack speed constants → Check regression_tests.rs first
🛑 Route name in code → Use route_id (names change frequently)
🛑 Duration without lead-in → Always include (0.2-5.7km varies)

## Key Files for Zwift Logic
- `src/zwift/duration.rs` - Pack speed calculations
- `tests/regression_tests.rs` - 151 race validations
- `data/route_manifest.json` - Route definitions (by ID)
- `ZWIFT_API_LOG_RECENT.md` - API change tracking

## References

Parent standards: /home/jack/tools/CLAUDE.md (includes empirical development principles)
Project documentation: docs/ subdirectories