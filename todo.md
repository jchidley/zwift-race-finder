# Zwift Race Finder - TODO

## 🚨 IMMEDIATE PRIORITY - Post-Cleanup Tasks

### Step 1: Test Everything After Cleanup ⚡ IN PROGRESS
```bash
# Test core functionality
cargo test
cargo run -- --help
cargo run  # Should show upcoming races

# Test imports still work
./import_zwiftpower_dev.sh --help

# Check scripts have correct permissions
ls -la *.sh | grep -v "rwx"
```

### Step 2: Commit Cleanup to GitHub
```bash
git add -A
git commit -m "refactor: major cleanup - remove dead code and rename files

- Removed 28 obsolete files (old extractors, abandoned approaches)
- Renamed files for clarity (extract_zwiftpower_v2 → zwiftpower_profile_extractor)
- Kept zwiftpower_event_extractor.js for future individual event scraping
- Updated documentation to reflect new filenames"
git push
```

## 🎯 THEN: Get Real Race Data

### Step 3: Run Strava Import 
```bash
cd ~/tools/rust/zwift-race-finder
./strava_auth.sh                    # Authenticate with Strava
./strava_fetch_activities.sh        # Download your races  
./strava_import_to_db.sh           # Import REAL times
python strava_analyze.py           # Check your actual speeds
```

### Step 4: Fix KISS Racing Distance
```sql
-- Run this after Strava import
sqlite3 ~/.local/share/zwift-race-finder/races.db \
  "UPDATE routes SET distance_km = 35.0 WHERE route_id = 2474227587;"
```

### Step 5: Run Regression Test
```bash
cargo test regression_test -- --nocapture
# Should see MASSIVE improvement from 92.8% error!
```

## 📊 Only After Real Data Is Imported

### Fix Remaining Routes (as needed)
- [ ] Check any routes with >50% error
- [ ] Focus on your most frequent races
- [ ] Use Strava distances as ground truth

### Future Projects
- [ ] Scrape individual ZwiftPower event pages for precise times
- [ ] Automate Strava sync
- [ ] Add draft vs solo toggle
- [ ] Implement device emulation testing

## ✅ Completed Today
- [x] Major cleanup: removed 28 dead files
- [x] Renamed files with sensible names
- [x] Updated CLAUDE.md and logs with new filenames
- [x] Discovered ZwiftPower event pages have actual times

## 🐛 Current Known Issues
- [x] 92.8% prediction error - caused by fake "actual" times
- [ ] Route distances wrong (KISS = 100km?)
- [ ] Multi-lap races not handled correctly

## 💡 The Big Lesson
**We were comparing estimates to estimates!** The "actual_minutes" in the database were calculated as distance ÷ 30 km/h, not real race times. Strava + individual ZwiftPower events have the real data we need.