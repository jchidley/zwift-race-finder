# Zwift Race Finder - TODO

## 🚀 High Priority (Do First)

### 1. Fix Unknown Routes Tracking
- [ ] Update import script to properly track all 109 unique events
- [ ] Show all unmapped events in unknown_routes table
- [ ] Add query to group by event name pattern (e.g., all "3R Racing" variants)

### 2. Map Top 10 Most Common Routes
- [ ] Research route data for:
  - [ ] 3R Racing (13 races) - likely Volcano Flat
  - [ ] EVO CC Race Series (8 races)
  - [ ] Team DRAFT Monday Race (6 races)
  - [ ] KISS Racing (4 races)
  - [ ] 3R Volcano Flat Race - 3 Laps (3 races) - definitely Volcano Flat
- [ ] Create route_mappings.sql with proper route data
- [ ] Update import script to use mappings

### 3. Write Basic Regression Test
- [ ] Create tests/regression.rs
- [ ] Load actual race times from database
- [ ] Compare with predicted times using current algorithm
- [ ] Output accuracy metrics (even with placeholder data)

## 📊 Medium Priority

### 4. Improve Route Research Tools
- [ ] Create route_research.sh script that:
  - [ ] Shows all variants of an event name
  - [ ] Calculates average distance/time for consistency
  - [ ] Suggests likely route matches
- [ ] Add WebFetch integration for ZwiftHacks API

### 5. Enhance Import Process
- [ ] Add event_name → route_id mapping table
- [ ] Support partial matching (e.g., "Volcano" → Volcano routes)
- [ ] Add confidence scoring for mappings
- [ ] Create update mechanism for existing data

### 6. Regression Test Improvements
- [ ] Group results by category (A/B/C/D/E)
- [ ] Account for draft benefit in races vs TTs
- [ ] Add visualization of prediction errors
- [ ] Identify systematic biases

## 🔍 Low Priority

### 7. Data Quality
- [ ] Identify and handle DNF/DQ results
- [ ] Detect obvious data errors (0 minute races, etc.)
- [ ] Add data validation to import process

### 8. Documentation
- [ ] Document route mapping process
- [ ] Create CONTRIBUTING.md for route additions
- [ ] Add examples of good route research

### 9. Long-term Features
- [ ] Auto-fetch from ZwiftPower API (if available)
- [ ] Machine learning for route detection
- [ ] Community database of route mappings
- [ ] Integration with Zwift Companion API

## 📝 Quick Wins (Can do anytime)

- [ ] Add --list-routes command to show all known routes
- [ ] Add --stats command to show database statistics  
- [ ] Color code output for better readability
- [ ] Add progress bar for import process

## 🐛 Known Issues

- [ ] Unknown routes only showing 1 entry instead of all unique events
- [ ] Warning about unused functions in Rust code
- [ ] Need proper error handling for malformed Zwift scores

## 💡 Ideas for Later

- Consider using event_id from ZwiftPower for better matching
- Add support for workout/group ride filtering
- Create web interface for route mapping collaboration
- Add export functionality for training analysis