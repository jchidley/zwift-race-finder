# PROJECT_WISDOM.md - Zwift Race Finder

Learning log for project-specific insights and solutions.

*Note: Older entries archived to PROJECT_WISDOM_ARCHIVE_20250614.md*

## 2025-06-13: Gymnasticon UDP Control - Correct netcat Syntax
**Insight**: OpenBSD netcat (on Raspberry Pi) requires specific syntax for UDP: `echo '{"power":250}' | nc -u -w1 127.0.0.1 3000`. The `-w1` flag and explicit IP `127.0.0.1` (not `localhost`) are critical.
**Impact**: UDP messages won't work without proper syntax. Always use the exact format shown above.

## 2025-06-13: Gymnasticon Bluetooth/Noble - Socket Binding Issue
**Insight**: The gymnasticon bot mode had a duplicate `bind()` call in the compiled code causing "Socket already bound" errors. Check for duplicate binds when debugging UDP server issues.
**Impact**: If you see "ERR_SOCKET_ALREADY_BOUND", look for duplicate bind() calls in the code, not just port conflicts.

## 2025-06-13: Zwift Connection - Both Power AND Cadence Required
**Insight**: When connecting gymnasticon to Zwift, you MUST connect both Power Source AND Cadence sensors (same device). Connecting only Power often fails. Connection reliability varies between Bluetooth and ANT+.
**Impact**: Always connect both sensors, have both protocols available, and be prepared to try different connection orders.

## 2025-06-14: License Integration - API Boundary for AGPL/MIT Compatibility
**Insight**: Successfully integrated AGPL-licensed zwift-offline with MIT/Apache zwift-race-finder using API boundary pattern. Created fork with export endpoints, no code copying required.
**Impact**: When integrating copyleft (AGPL/GPL) code with permissive licenses, use HTTP APIs or service boundaries. Never copy code directly.

## 2025-06-14: Python Environment - Use uv for zwift-offline Setup
**Insight**: zwift-offline Python environment setup is fast and reliable with `uv`: create setup_venv.sh script, use `uv venv` and `uv pip install -r requirements.txt`.
**Impact**: Avoid pip/virtualenv issues. Always use uv for Python dependency management in this project.

## 2025-06-14: Port Privileges - zwift-offline Environment Variables
**Insight**: zwift-offline defaults to privileged ports (80/443) causing "Permission denied". Use environment variables: ZOFFLINE_CDN_PORT=8080 ZOFFLINE_API_PORT=8443.
**Impact**: Never run as root. Always set port environment variables for non-privileged operation.

## 2025-06-14: Database Path - races.db not zwift_routes.db
**Insight**: zwift-race-finder uses `~/.local/share/zwift-race-finder/races.db` (from get_database_path()), not zwift_routes.db. Always check actual implementation.
**Impact**: Database operations fail with wrong filename. Verify paths in database.rs before assuming filenames.

## 2025-06-14: Route Data Location - WAD Archives Contain Everything
**Insight**: All Zwift route data exists in WAD archives at `C:\Program Files (x86)\Zwift\assets\Worlds\world*\data_1.wad`. Complete extraction requires wad_unpack.exe (no longer publicly available from referenced repos).
**Impact**: Can't extract complete route data without the decompression tool. Third-party sources and zwift-offline API provide sufficient coverage.

## 2025-06-14: zwift-offline Route Filtering - Event-Only by Design
**Insight**: zwift-offline only exports routes with `eventOnly='1'` attribute (55 routes). Free-ride routes are intentionally filtered out in the codebase.
**Impact**: This is a design choice, not a bug. For complete route coverage, use third-party sources that include all route types.

## 2025-06-14: Zwift Event Structure - Routes + Modifiers
**Insight**: Events don't create new routes - they use existing base routes with modifiers: laps (repeat loop N times), distance (fixed total), or duration (time-based). Free-ride routes have lead-in + lap structure.
**Impact**: When analyzing events, look for the base route_id and modifier, not a unique "event route". All base data is in the route definition.

## 2025-06-14: Route ID Handling - Use IDs Not Names
**Insight**: Route IDs can be negative (signed integers). Always use route_id as the identifier, never route name, as names change frequently but IDs are permanent.
**Impact**: Database and import tools must handle signed route IDs. Name-based lookups will fail when Zwift updates route names.

## 2025-06-19: Duration Model Simplification - Draft Already in Category Speeds
**Insight**: Claude simplified the original modeling approach for unknown reasons. Original intent was to model solo riding accurately then apply drafting factors, assuming detailed route profiling beyond just elevation/distance. Current system uses category speeds (30.9 km/h for Cat D) that already include average draft benefits from 151 real races.
**Impact**: The dual-speed model in `calculate_duration_with_dual_speed` exists but isn't used. Current 20.4% accuracy achieved with simpler model because empirical speeds inherently include draft. More sophisticated modeling still possible with better route profiling.

## 2025-06-19: Power Simulation Tools - vpower and gymnasticon
**Insight**: Tools like vpower (https://github.com/oldnapalm/vpower) and gymnasticon can simulate power output for Zwift, enabling controlled testing of race duration algorithms with repeatable power profiles.
**Impact**: Can validate duration predictions by simulating consistent power outputs across different routes and conditions. Useful for understanding power/speed relationships and testing edge cases without needing real race data.

## 2025-06-22: Garmin Connect API - FIT Files Wrapped in ZIP
**Insight**: Garmin Connect API returns FIT files wrapped in ZIP containers (identified by PK\x03\x04 header). Virtual cycling activities from Zwift use "virtual_ride" or "virtual_cycling" activity type keys, not the standard cycling types.
**Impact**: When downloading FIT files, check for ZIP header and extract. Must include virtual_* activity types to capture Zwift rides.