# Zwift-Offline Integration Guide

This document explains how to use the zwift-offline fork to import route data into zwift-race-finder while maintaining license compatibility.

## License Compatibility

- **zwift-race-finder**: MIT/Apache-2.0 (permissive)
- **zwift-offline**: AGPL v3 (copyleft)

The integration uses an **API boundary** to prevent license contamination. We never copy AGPL code into our MIT/Apache project. Instead, we make HTTP API calls to zwift-offline running as a separate service.

## Setup

### 1. Fork Setup (Already Complete)

You've already created and cloned the fork:
- Fork: https://github.com/jchidley/zwift-offline
- Local path: `zwift-offline/` (gitignored)

### 2. Enable Route Export in zwift-offline

The route export module has been added to your fork:
- `zwift_offline/route_export.py` - Export endpoints
- Automatically registered when zwift-offline starts

### 3. Setup Python Environment

```bash
cd zwift-offline
./setup_venv.sh  # One-time setup
```

### 4. Run zwift-offline

```bash
cd zwift-offline
./run_server.sh  # Uses uv to run in virtual environment
```

The server will start on:
- CDN server: `http://localhost:8080`
- API server: `https://localhost:8443` (with self-signed certificate)

No root privileges needed.

## Import Process

### Option A: Direct API Import

```bash
# Import directly from running zwift-offline server
./scripts/import_from_zwift_offline.sh --skip-ssl-verify

# This creates JSON files in data/zwift_offline_export/
```

### Option B: Import to Database

```bash
# First export from zwift-offline (if not done already)
./scripts/import_from_zwift_offline.sh --skip-ssl-verify

# Then import into SQLite database
cargo run --bin import_zwift_offline_routes -- \
    --input-dir data/zwift_offline_export \
    --database zwift_routes.db
```

### Option C: Dry Run First

```bash
# See what would be imported without making changes
cargo run --bin import_zwift_offline_routes -- \
    --input-dir data/zwift_offline_export \
    --database zwift_routes.db \
    --dry-run
```

## Available Endpoints

Your zwift-offline fork now provides:

- `/api/export/summary` - Overview of available data
- `/api/export/routes` - All routes with IDs, distances, worlds
- `/api/export/start_lines` - Start line positions and timing
- `/api/export/events` - Event definitions with route mappings

## Data Mapping

The integration maps zwift-offline data to zwift-race-finder's schema:

| zwift-offline | zwift-race-finder |
|---------------|-------------------|
| route (ID) | route_id |
| name | name |
| distance | distance_km |
| course | world_id (via mapping) |
| sport | sport (0=Cycling, 1=Running) |

## Current Limitations

The zwift-offline integration provides **55 event-only routes**. This is a subset of all routes because:
- zwift-offline filters routes with `eventOnly='1'` attribute
- Free-ride routes are not included in the export
- Complete route extraction requires WAD file decompression tools that are not currently available

For comprehensive route coverage, use the existing database of 378 routes imported from third-party sources.

## Contributing Back

If the route export functionality proves useful, consider submitting a PR to the original zwift-offline project. The export module is self-contained and doesn't modify core functionality.

## Troubleshooting

### SSL Certificate Errors

zwift-offline uses self-signed certificates. Use `--skip-ssl-verify` flag when importing.

### Missing Data Files

Ensure zwift-offline has the latest game data:
```bash
cd zwift-offline
python get_events.py
python get_start_lines.py
```

### Route ID Mismatches

zwift-offline uses game-internal route IDs which may differ from ZwiftPower. The integration preserves zwift-offline's IDs as authoritative.