# Duration Estimation Algorithms

## Overview

Estimates how long a Zwift race will take for a specific rider based on route characteristics and Racing Score category.

## Production Algorithm: Category Speed Model

```
duration_minutes = (distance_km / (category_speed × difficulty_multiplier)) × 60
```

### Category Speeds (km/h)

Empirical averages from real races. Already include draft benefit.

| Category | Score Range | Speed (km/h) | Source |
|----------|-------------|--------------|--------|
| E | 0–99 | 28.0 | Estimated ~10% slower than Cat D |
| D | 100–199 | 30.9 | Calibrated from 151 real races |
| C | 200–299 | 33.0 | Scaled from Cat D |
| B | 300–399 | 37.0 | Scaled from Cat D |
| A | 400–599 | 42.0 | Scaled from Cat D |
| A++ | 600+ | 45.0 | Estimated ~7% faster than Cat A |

Score 600+ maps to "A++" (45.0 km/h). The detailed category function splits this further: 590–649 = "A+", 650+ = "A++".

### Route Difficulty Multiplier

Multiplied with category speed. Values >1.0 mean faster (flat), <1.0 mean slower (hilly).

**Elevation-based** (`get_route_difficulty_multiplier_from_elevation_and_category`):

Uses piecewise linear interpolation across 9 breakpoints for smooth transitions, with a category-aware penalty on steep climbs.

| Elevation/km | Base Multiplier | Terrain Example |
|-------------|-----------------|-----------------|
| 0 m/km | 1.10 | Tempus Fugit (~1 m/km) |
| 5 m/km | 1.05 | Most Watopia routes |
| 10 m/km | 1.00 | Rolling (transition zone) |
| 15 m/km | 0.93 | Rolling hills |
| 20 m/km | 0.85 | Mountain 8 (~21 m/km) |
| 30 m/km | 0.70 | Very hilly |
| 40 m/km | 0.55 | Mountain climbs |
| 60 m/km | 0.45 | Road to Sky (~60 m/km) |
| 100 m/km | 0.30 | Theoretical maximum |

Values between breakpoints are linearly interpolated.

**Category climbing penalty** (applied above 15 m/km):

On flat terrain, all categories ride near their empirical category speed (ratio ≈ 1.0). On steep climbs, lower categories are disproportionately slower because w/kg matters more than raw watts. This was measured from race data: Cat D achieves 48% of its flat speed on >20 m/km climbs, while Cat C achieves 54%.

| Category | Climbing Factor | Effect |
|----------|----------------|--------|
| A / A++ | 1.00 | No penalty |
| B | 0.97 | Minor penalty |
| C | 0.93 | Moderate penalty |
| D | 0.85 | Significant penalty |
| E | 0.75 | Large penalty |

The final multiplier on a climb is `base_multiplier × category_factor`. For example, Road to Sky (60 m/km) for Cat D: `0.45 × 0.85 = 0.38`, giving an effective speed of `30.9 × 0.38 = 11.7 km/h` — close to the observed 9.6 km/h from actual race data.

**Name-based fallback** (`get_route_difficulty_multiplier`) — used only when no elevation data is available:

| Route name contains | Multiplier |
|---------------------|------------|
| "alpe", "ventoux" | 0.7 |
| "epic", "mountain" | 0.8 |
| "flat", "tempus" | 1.1 |
| (anything else) | 1.0 |

### Which Multiplier Is Used Where

| Function | Multiplier Used | When Called |
|----------|----------------|------------|
| `estimate_duration_from_route_id` | Elevation + category | Route known, no distance override |
| `estimate_duration_with_distance` | Elevation + category | Route known, distance provided (multi-lap) |
| `estimate_duration_for_category` | Name-based only | No route lookup possible (fallback) |
| `event_filtering::estimate_event` | Resolves the ordered fallback below | Shared by duration filtering, compact rows, and verbose duration output |

### Estimation Priority

`event_filtering::estimate_event()` resolves one `EventEstimate` (minutes, available distance/elevation, and fixed-versus-estimated status). `event_matches_duration()` compares those minutes to the target with an inclusive tolerance. Both compact and verbose display use the same resolver; display does not add a separate surface multiplier or extra lead-in. An unestimable event is not a match.

The resolver tries these methods in order, returning the first available estimate:

1. **Fixed duration**: If event has `duration_in_minutes` or `duration_in_seconds`, use directly
2. **Multi-lap with known route**: Subgroup has lap count → `lead_in + (route_distance × laps)` → estimate
3. **Known route + known distance**: Route_id resolves, event/subgroup has distance → `estimate_duration_with_distance`
4. **Known distance + unknown route**: Distance available but route unknown → `estimate_duration_for_category` (name-based multiplier)
5. **Known route only**: Route_id resolves, no distance → `estimate_duration_from_route_id` (adds lead-in) + multi-lap DB check
6. **Racing Score + route**: Route_id known, parse distance from description → estimate
7. **Fallback: event distance**: `distance_in_meters > 0` → `estimate_duration_for_category`
8. **Fallback: Racing Score description**: Parse distance from description text
9. **Fallback: name guess**: `estimate_distance_from_name` heuristic
10. **Selected subgroup fallback**: fixed minutes or positive distance from the first subgroup selected by `find_user_subgroup`; it is not an any-subgroup match.

Fixed event minutes take precedence over event seconds and subgroup lap/distance metadata. For example, an API event declaring 2,700 seconds displays 45 minutes even if its lap-based estimate would differ. Available fixed-event distance/lap metadata is retained without overriding that duration.

Description and name fallbacks are estimates, not verified route facts. In particular, the existing description parser takes the first matching distance and does not select the relevant stage from multi-stage text; shared display makes this visible but does not resolve that data-quality issue.

### Lead-in Distance Handling

- **Multi-lap (step 2)**: Lead-in added once by the shared resolver in `event_filtering.rs`
- **Route-only (step 5)**: Lead-in added inside `estimate_duration_from_route_id` in `estimation.rs`
- **Explicit event/subgroup distance**: Treated as total distance; no extra lead-in is added by filtering or display.
- **Other fallbacks**: Lead-in is not added. Historical DB multi-lap metadata uses lap distance × laps plus one lead-in, then estimates that total, rather than multiplying a single-lap time and repeating its lead-in.

### Elevation knowledge and totals

Route and event lead-in climbing are optional: unknown is not zero. SQLite knowledge flags preserve legacy positive values while treating legacy zeros as unknown. Explicitly stored zero is known flat. The primary source is selected installed-client route metadata, then previously extracted client XML for missing IDs. These are the only supported route metadata inputs; see [client-only import](../howto/CLIENT_ROUTE_DATA.md). Client imports reconcile legacy metadata by numeric ID, atomically preserve history, and persist source provenance. Community imports and website scraping are retired; existing database rows remain available until explicitly updated. No route dataset is bundled.

- **Lap-based totals:** fractional lap elevation × laps + fractional event lead-in elevation once, rounded only for display. Both must be known; otherwise the total displays as unknown. Missing lead-in distance is also preserved as unknown rather than assumed zero. Route-only/lap distance estimates require a known lead-in distance.
- **Distance-limited totals:** proportional lap climbing for (total distance − lead-in distance), plus known lead-in climbing. Without a full elevation profile this is approximate and displays with a `~` prefix. Distances shorter than the lead-in do not get invented climbing.
- **Duration inputs:** when event climbing is available, apply the existing category/elevation multiplier to total gain over total distance. If lead-in gain is unknown, retain the known lap gradient; if lap gain is also unknown, use the existing name-based fallback. Category speeds and difficulty breakpoints are unchanged.
- **Fixed durations:** climbing/distance metadata never overrides declared time.

### Route ID Alias Resolution

Zwift uses different internal route IDs for the same physical route depending on context (event-only vs free-ride). The `route_aliases` table maps these alternative IDs to canonical DB route IDs. `Database::get_route()` checks aliases transparently: direct lookup → alias lookup → None. See [Database Reference](DATABASE.md).

## Calibration

- **Latest recorded MAE**: 16.6% on 125 matched races in the owner's local dataset
- **Product target**: < 20% MAE
- **Test threshold**: the focused regression asserts < 30% MAE
- **Run**: `cargo test --lib test_race_predictions_accuracy -- --nocapture`

The result depends on the local `race_results` database and was not reproduced by this documentation pass.

### Error by Terrain Type

| Terrain | MAE | n | Notes |
|---------|-----|---|-------|
| Flat (<10 m/km) | ~19% | 104 | Bias ≈ 0%, model well-calibrated |
| Hilly (10–20 m/km) | ~10% | 16 | Best accuracy segment |
| Climbing (>20 m/km) | <20% | 5 | Fixed from 43.8% by category-aware multiplier |

### Key Findings

1. Pure physics models fail (127% error)
2. Draft benefit crucial — already baked into category speeds
3. High variance is inherent to racing (same route: 32–86 min depending on pack dynamics)
4. Binary state: with pack or dropped, no gradual separation
5. Lower categories are disproportionately slower on climbs (w/kg effect)
6. Weight/height are **not** direct model inputs — the effect is captured via the category × elevation interaction

## Special Cases

| Case | Handling |
|------|----------|
| Fixed duration events | Duration used directly from event data (no estimation) |
| Time trials | No specific handling — uses same category speeds (known inaccuracy) |
| Multi-lap | Route distance × laps + lead-in |
| Racing Score events | `distanceInMeters: 0` in API; parse from description |
| Unknown routes | Name-based difficulty multiplier fallback |
| Category E | Separate category at 28 km/h |
