# ZwiftPower Export Steps

## Quick Commands (run from anywhere)

1. **Copy JavaScript to clipboard** (requires xclip):
   ```bash
   cat ~/tools/rust/zwift-race-finder/extract_zwiftpower.js | xclip -selection clipboard
   ```

2. **After running JavaScript in browser, copy the downloaded file**:
   ```bash
   ~/tools/rust/zwift-race-finder/copy_results.sh
   ```

3. **Import the results to database**:
   ```bash
   ~/tools/rust/zwift-race-finder/export_zwiftpower_logged_in.sh import
   ```

## Full Process

1. Log into ZwiftPower: https://zwiftpower.com/profile.php?z=YOUR_PROFILE_ID
2. Open Developer Tools (F12) → Console tab
3. Copy and paste the JavaScript from `~/tools/rust/zwift-race-finder/extract_zwiftpower.js`
4. File downloads to `/mnt/c/Users/YOUR_USERNAME/Downloads/zwiftpower_results.json`
5. Run the copy script: `~/tools/rust/zwift-race-finder/copy_results.sh`
6. Import to database: `~/tools/rust/zwift-race-finder/export_zwiftpower_logged_in.sh import`

## What This Does

- Extracts all your race results from ZwiftPower
- Saves them to the SQLite database
- Allows the race finder to use your actual race times for accurate predictions
- Enables regression testing based on your real performance data