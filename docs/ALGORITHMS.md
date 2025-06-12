# Duration Estimation Algorithms

## Overview

The core challenge is estimating how long it will take a specific rider to complete a Zwift race. This involves route characteristics, rider capabilities, and pack dynamics.

## Primary Algorithm: Dual-Speed Model

### Core Concept
Races have two states:
- **With Pack**: ~33% power savings from draft, higher speed
- **Solo/Dropped**: No draft benefit, ~77% of pack speed

### Implementation

```rust
// 1. Calculate pack speed based on category
let pack_speed_kmh = match category {
    "A" => 37.5,
    "B" => 35.0,
    "C" => 32.5,
    "D" => 30.9,
    _ => 30.0,
};

// 2. Calculate drop probability based on elevation
let drop_probability = calculate_drop_probability(
    elevation_m,
    weight_kg,
    category
);

// 3. Weight the two speeds
let solo_speed = pack_speed * 0.77;
let average_speed = (pack_speed * (1.0 - drop_probability)) + 
                   (solo_speed * drop_probability);

// 4. Calculate time
let time_hours = distance_km / average_speed;
```

### Drop Probability Factors

1. **Elevation Impact**:
   - <100m: Low probability (0.1-0.2)
   - 100-300m: Medium (0.3-0.5)
   - >300m: High (0.6-0.8)

2. **Weight Penalty**:
   - 86kg vs typical 70-75kg = disadvantage on climbs
   - Additional 0.1-0.2 probability per 10kg over average

3. **Category Factor**:
   - Lower categories = higher drop probability
   - D: +0.1, C: +0.05, B: 0, A: -0.05

## Pack Dynamics Insights

### Draft Benefit (Empirical)
- Zwift: ~33% power savings (vs 25% real world)
- Bigger groups = more consistent draft
- Position in pack matters less than being in pack

### Binary State Discovery
Analysis of 151 races revealed:
- Either with pack OR dropped - no gradual separation
- Transition happens quickly (usually on climbs)
- Once dropped, very hard to rejoin

### High Variance Explanation
Bell Lap example: 32-86 minutes for same route
- Best case: Stay with pack entire race (32 min)
- Worst case: Dropped early, solo most of race (86 min)
- 82.6% of variance explained by drop dynamics

## Route Difficulty Multipliers

Based on elevation per kilometer:

```rust
fn get_difficulty_multiplier(elevation_per_km: f64) -> f64 {
    match elevation_per_km {
        x if x < 5.0 => 1.0,   // Flat
        x if x < 10.0 => 1.05, // Rolling
        x if x < 15.0 => 1.1,  // Hilly
        x if x < 20.0 => 1.15, // Mountainous
        _ => 1.2,              // Epic climbing
    }
}
```

## Fallback Estimations

When route data unavailable:

### Category-Based
```rust
// Base speeds by category (km/h)
let base_speed = match category {
    "A" => 35.0,
    "B" => 32.5,
    "C" => 30.0,
    "D" => 27.5,
    _ => 25.0,
};
```

### Distance Parsing
1. Check event description for patterns:
   - "1 lap" → use route distance
   - "2 laps" → route distance × 2
   - "20km" → direct distance

2. Multi-stage events:
   - Parse each stage distance
   - Sum for total

## Calibration

### Regression Testing
- 151 actual race results from Jack
- Mean Absolute Error: 23.6%
- Target: <20% error

### Key Findings
1. Pure physics models fail (127% error)
2. Draft benefit crucial for accuracy
3. High variance is inherent to racing
4. Category speeds relatively consistent

## Special Cases

### Time Trials
- No draft benefit
- Use solo speed throughout
- Lower variance in predictions

### Multi-Category Events
- Use specific category's speed
- Account for different start times
- May have category-specific distances

### Lead-in Distance
- Additional distance before official route
- Varies by event type (0.2-5.7 km)
- Must be added to route distance

## Future Improvements

1. **Machine Learning**: Train on larger dataset
2. **Weather Integration**: Headwind/tailwind effects
3. **Power-Based**: Use FTP for personalized estimates
4. **Historical Performance**: Weight recent results
5. **Route-Specific Patterns**: Some routes favor breakaways