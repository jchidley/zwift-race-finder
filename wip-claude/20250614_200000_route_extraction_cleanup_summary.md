# Route Extraction Investigation - Cleanup Summary

*Date: 2025-06-14 20:00:00*

## What We Learned

1. **All route data exists in WAD files** in the Zwift installation
2. **wad_unpack.exe is required** but no longer publicly available
3. **zwift-offline provides 55 event routes** via API (event-only routes)
4. **We already have 378 routes** from third-party sources

## Code Cleanup Performed

### Removed (Experimental/No Value)
- `src/bin/compare_routes.rs` - Investigation tool, no longer needed
- `route_comparison_report.txt` - Investigation output
- `route_comparison_summary.md` - Investigation summary
- `debug_speed_*.png` - OCR debug images
- `zwift-offline/scripts/get_all_routes.py` - Requires unavailable wad_unpack.exe
- `zwift-offline/scripts/extract_all_routes_wsl.sh` - WSL wrapper for above
- `docs/guides/ZWIFT_ROUTE_EXTRACTION.md` - Duplicate documentation

### Kept (Working Integration)
- `zwift-offline/route_export.py` - API endpoints for route export
- `scripts/import_from_zwift_offline.sh` - Import script
- `src/bin/import_zwift_offline_routes.rs` - Import binary
- `src/zwift_offline_client.rs` - Client code for API
- `docs/ZWIFT_OFFLINE_INTEGRATION.md` - How to use the integration
- `docs/ROUTE_DATA_EXTRACTION.md` - Technical findings documentation

### Documentation Updates
- Added current limitations section to ZWIFT_OFFLINE_INTEGRATION.md
- Noted that only 55 event routes are available
- Clarified that 378 routes from third-party sources provide good coverage

## Status
- Working zwift-offline integration via API boundary (license-safe)
- 55 event routes available through the integration
- 378 routes total in database from various sources
- Complete extraction blocked by unavailable tooling, not data availability