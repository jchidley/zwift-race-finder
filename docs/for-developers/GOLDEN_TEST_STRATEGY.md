# Golden Test Strategy

**Created**: 2025-06-08  
**Purpose**: Explain the golden test approach and improvements

## Original Issue

The initial golden baseline generated 9,414 tests by testing every combination of:
- 23 routes × 15 distances × 17 scores = 5,865 tests (3 times over)
- Plus edge cases

This approach had several problems:
1. **Database Dependency**: Tests required the production database at `~/.local/share/zwift-race-finder/races.db`
2. **No Cleanup**: No test data cleanup mechanism
3. **Environment Dependent**: Tests could fail if user's database differs
4. **Excessive Tests**: Many redundant combinations that don't add value

## Improved Approach

### 1. Focused Test Data

Instead of testing every possible combination, focus on representative cases:

**Routes** (11 instead of 23):
- Flat: Tempus Fugit, Tick Tock
- Rolling: Watopia's Waistband, Two Village Loop, Downtown Dolphin
- Hilly: Hilly Route, Castle to Castle, Epic KOM
- Mountain: Road to Sky, Ven-Top, Four Horsemen

**Distances** (11 instead of 15):
- Short races: 10, 15, 20 km
- Medium races: 25, 30, 40 km
- Long races: 50, 60, 80 km
- Edge cases: 0.1, 200 km

**Scores** (12 instead of 17):
- Category boundaries: 99/100, 199/200, 299/300, 399/400
- Category centers: 150, 250, 350, 450
- Edge cases: 0, 999

This reduces tests from 5,865 to 1,452 (75% reduction) while maintaining coverage.

### 2. No Database Dependency

The improved golden tests use the `estimate_duration_for_category` function which:
- Takes route NAME as input (not route_id)
- Uses hardcoded difficulty multipliers
- Doesn't access the database

This makes tests:
- ✅ Reproducible across environments
- ✅ Fast (no I/O)
- ✅ Reliable (no external dependencies)

### 3. Test Database Strategy

For tests that DO need database access:

```rust
use zwift_race_finder::test_db::TestDatabase;

#[test]
fn test_with_database() {
    // Creates temporary database that auto-cleans
    let test_db = TestDatabase::new().unwrap();
    
    // Seed with minimal test data
    test_db.seed_test_routes().unwrap();
    
    // Run tests...
    
    // Database automatically deleted when test_db drops
}
```

Benefits:
- Isolated from production data
- Automatic cleanup
- Consistent test data
- Can run in parallel

## Migration Strategy

### Phase 1: Use Improved Golden Tests
```bash
# Generate focused baseline (1,452 tests)
cargo test generate_golden_baseline_improved -- --ignored
```

### Phase 2: Database-Dependent Tests
For functions that need database access:
1. Create minimal test fixtures
2. Use TestDatabase for isolation
3. Test only critical paths

### Phase 3: Integration Tests
Keep existing integration tests but:
1. Mock API calls
2. Use test database
3. Verify CLI output format

## Summary

The improved approach:
- **Reduces test count by 75%** while maintaining coverage
- **Removes database dependency** for most tests
- **Provides test isolation** when database is needed
- **Ensures reproducibility** across environments
- **Simplifies maintenance** with focused test cases

This makes the golden tests more reliable and practical for ensuring behavioral preservation during the UOM migration.