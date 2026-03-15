# Duration Estimation Algorithms

## Overview

Estimates how long a Zwift race will take for a specific rider based on route characteristics and Racing Score category.

## Production Algorithm: Category Speed Model

The algorithm used in production (`estimate_duration_for_category` in `src/duration_estimation.rs`):

```
duration_minutes = (distance_km / (category_speed * difficulty_multiplier)) × 60
```

### Category Speeds (km/h)

Empirical averages from real races. Already include draft benefit.

| Category | Score Range | Speed (km/h) |
|----------|-------------|--------------|
| E | 0–99 | 28.0 |
| D | 100–199 | 30.9 |
| C | 200–299 | 33.0 |
| B | 300–399 | 37.0 |
| A | 400–599 | 42.0 |
| A++ | 600+ | 45.0 |

Source: Cat D speed (30.9) calibrated from 151 real races. Others scaled from Cat D.

### Route Difficulty Multiplier

Based on elevation per kilometer (`get_route_difficulty_multiplier_from_elevation`):

| Elevation/km | Multiplier | Terrain |
|-------------|------------|---------|
| < 5 m/km | 1.0 | Flat |
| 5–10 m/km | ~1.05 | Rolling |
| 10–15 m/km | ~1.1 | Hilly |
| 15–20 m/km | ~1.15 | Mountainous |
| > 20 m/km | ~1.2 | Epic climbing |

When elevation data is unavailable, a name-based lookup (`get_route_difficulty_multiplier`) provides a fallback.

### Estimation Priority

`event_filtering.rs` tries these in order:

1. **Multi-lap with known route**: route distance × laps + lead-in → estimate with route data
2. **Known distance + known route**: use route difficulty data
3. **Known distance + unknown route**: category speed with name-based multiplier
4. **Known route only**: `estimate_duration_from_route_id` (route distance, no lead-in added)
5. **Racing Score event**: parse distance from description text
6. **Fallback**: category speed × parsed or estimated distance

Lead-in distance (0.2–5.7 km) is added to all route-based estimates and multi-lap calculations.

## Calibration

- **Current MAE**: 17.9% on 125 matched races
- **Target**: < 20% MAE
- **Threshold**: regression test asserts < 30% MAE
- Run: `cargo test --lib test_race_predictions_accuracy -- --nocapture`

### Key Findings

1. Pure physics models fail (127% error)
2. Draft benefit crucial — already baked into category speeds
3. High variance is inherent to racing (same route: 32–86 min depending on pack dynamics)
4. Binary state: with pack or dropped, no gradual separation
5. 82.6% of variance explained by drop dynamics

## Special Cases

| Case | Handling |
|------|----------|
| Time trials | No draft, use solo speed |
| Multi-lap | Route distance × laps + lead-in |
| Racing Score events | `distanceInMeters: 0` in API; parse from description |
| Unknown routes | Name-based difficulty multiplier fallback |
| Category E | Treated as separate category (28 km/h) |
