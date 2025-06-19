# Migration Plan: Adopting the UOM (Units of Measurement) Crate

## Overview

This document outlines a comprehensive plan to migrate from manual unit conversions to the type-safe `uom` crate. This migration will prevent unit conversion errors and make the codebase more maintainable.

## Benefits of Migration

1. **Type Safety**: Compile-time prevention of unit mixing errors (e.g., adding meters to seconds)
2. **Automatic Conversions**: No manual conversion factors needed
3. **Self-Documenting Code**: Types clearly indicate what units are being used
4. **Zero-Cost Abstraction**: No runtime overhead compared to raw numeric types
5. **Prevention of Bugs**: Eliminates entire classes of unit conversion errors

## Migration Phases

### Phase 1: Foundation Setup
1. Add `uom` dependency to `Cargo.toml`:
   ```toml
   [dependencies]
   uom = { version = "0.36", features = ["f64", "std"] }
   ```

2. Create a new module `src/units.rs` for type aliases and common conversions:
   ```rust
   use uom::si::f64::*;
   use uom::si::length::meter;
   use uom::si::time::minute;
   use uom::si::velocity::kilometer_per_hour;
   
   pub type Distance = Length;
   pub type Duration = Time;
   pub type Speed = Velocity;
   ```

### Phase 2: Data Structure Migration

#### Update Core Structs
1. **RouteData**:
   ```rust
   pub struct RouteData {
       pub distance: Distance,        // was: distance_km: f64
       pub elevation: Distance,        // was: elevation_m: u32
       pub lead_in_distance: Distance, // was: lead_in_distance_km: f64
       // ... other fields
   }
   ```

2. **ZwiftEvent**:
   ```rust
   pub struct ZwiftEvent {
       pub distance: Option<Distance>,     // was: distance_in_meters: Option<f64>
       pub duration: Option<Duration>,     // was: duration_in_minutes: Option<u32>
       // ... other fields
   }
   ```

3. **RaceResult**:
   ```rust
   pub struct RaceResult {
       pub actual_duration: Duration,  // was: actual_minutes: u32
       // ... other fields
   }
   ```

### Phase 3: Function Signature Updates

#### Duration Estimation Functions
```rust
// Before:
pub fn estimate_duration_for_category(distance_km: f64, route_name: &str, zwift_score: u32) -> u32

// After:
pub fn estimate_duration_for_category(distance: Distance, route_name: &str, zwift_score: u32) -> Duration

// Before:
pub fn get_route_difficulty_multiplier_from_elevation(distance_km: f64, elevation_m: u32) -> f64

// After:
pub fn get_route_difficulty_multiplier_from_elevation(distance: Distance, elevation: Distance) -> f64
```

#### Speed Calculations
```rust
// Before:
pub fn get_category_speed(category: &str) -> f64  // km/h

// After:
pub fn get_category_speed(category: &str) -> Speed
```

### Phase 4: Conversion Points

#### API Data Ingestion
When receiving data from Zwift API:
```rust
// Convert API meters to Distance type
let distance = Distance::new::<meter>(api_event.distance_in_meters.unwrap_or(0.0));

// Convert API minutes to Duration type  
let duration = Duration::new::<minute>(api_event.duration_in_minutes.unwrap_or(0) as f64);
```

#### Database Storage
- Continue storing as raw numeric values in SQLite
- Convert at boundaries:
  ```rust
  // Storing
  let distance_km = distance.get::<kilometer>();
  
  // Loading
  let distance = Distance::new::<kilometer>(row.distance_km);
  ```

### Phase 5: Calculation Updates

#### Speed Calculations
```rust
// Before:
let duration_hours = distance_km / effective_speed;
let minutes = (duration_hours * 60.0) as u32;

// After:
let duration = distance / effective_speed;
let minutes = duration.get::<minute>() as u32;
```

#### Gradient Calculations
```rust
// Before:
let avg_gradient = (elevation_m as f64 / (distance_km * 1000.0)) * 100.0;

// After:
let avg_gradient = (elevation / distance).get::<ratio>() * 100.0;
```

### Phase 6: Display Formatting

Create display helpers:
```rust
impl Distance {
    fn format_km(&self) -> String {
        format!("{:.1} km", self.get::<kilometer>())
    }
    
    fn format_m(&self) -> String {
        format!("{:.0} m", self.get::<meter>())
    }
}

impl Duration {
    fn format_hhmm(&self) -> String {
        let total_minutes = self.get::<minute>() as u32;
        let hours = total_minutes / 60;
        let mins = total_minutes % 60;
        format!("{:02}:{:02}", hours, mins)
    }
}
```

## Migration Order

1. **Start with isolated modules**: 
   - `duration_estimation.rs` (core calculations)
   - `category.rs` (speed constants)

2. **Update data structures**:
   - `models.rs`
   - `database.rs` (add conversion layer)

3. **Update core logic**:
   - `main.rs` (gradually, function by function)
   - `parsing.rs`
   - `route_discovery.rs`

4. **Update peripheral code**:
   - Tests
   - Binary tools

## Testing Strategy

1. **Parallel Implementation**: Keep old functions alongside new ones during migration
2. **Property Testing**: Ensure conversions are correct
3. **Regression Testing**: Compare outputs before/after migration
4. **Boundary Testing**: Test edge cases in unit conversions

## Common Patterns

### Working with Options
```rust
// Convert Option<f64> to Option<Distance>
let distance = meters_opt.map(|m| Distance::new::<meter>(m));

// Convert Option<Distance> to Option<f64>
let km_opt = distance_opt.map(|d| d.get::<kilometer>());
```

### Arithmetic Operations
```rust
// All units must match or be compatible
let total_distance = route_distance + lead_in_distance;  // Both Distance
let speed = distance / time;                             // Returns Velocity
let time = distance / speed;                             // Returns Time
```

### Percentage Calculations
For percentages, continue using raw f64 as uom doesn't have a percentage unit:
```rust
let gradient_percent = (elevation / distance).get::<ratio>() * 100.0;
```

## Potential Challenges

1. **API Integration**: Need to convert at boundaries
2. **Database Schema**: Keep existing schema, convert in code
3. **Performance**: Minimal impact, but benchmark critical paths
4. **Learning Curve**: Team needs to understand uom patterns

## Resources

- [uom documentation](https://docs.rs/uom/latest/uom/)
- [uom examples](https://github.com/iliekturtles/uom/tree/master/examples)
- [Dimensional analysis guide](https://github.com/iliekturtles/uom/blob/master/docs/dimensional-analysis.md)

## Next Steps

1. Create a feature branch for the migration
2. Start with Phase 1 and 2 in isolation
3. Create comprehensive tests before refactoring
4. Migrate module by module
5. Run full regression suite after each phase