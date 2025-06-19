# Project: Zwift Race Finder
Updated: 2025-06-19 23:00:00

## Current State
Status: Comprehensive racing tactics documentation created based on academic research
Target: 20.4% mean absolute error on race time predictions
Latest: Created 11 racing guides documenting real-world vs Zwift physics and tactics

## Essential Context
- 378 routes in database from third-party sources (ZwiftHacks, WhatsOnZwift)
- Category speeds (30.9 km/h for Cat D) derived from 151 real races, include draft benefits
- Dual-speed model exists in code but unused - simple empirical model achieves 20.4% MAE
- CdA formula (A = 0.0276·h^0.725·m^0.425) is community reverse-engineered, not official
- Documentation now cleanly organized: for-racers/, for-developers/, reference/, project-history/
- Racing tactics documented: Binary draft model, attack mid-climb not base, 3-5 rider breakaways optimal

## Next Step
Consider testing racing tactics with vpower/gymnasticon for controlled power profiles

## Active Todo List
All documentation-related todos completed:
- [✓] Analyze documentation for different audiences
- [✓] Design organization serving both racing optimization AND development
- [✓] Create navigation strategy
- [✓] Identify consolidation candidates
- [✓] Create and execute migration plan
- [✓] Write navigation READMEs
- [✓] Create user-focused racing guides

Future development todos:
- [ ] Test gymnasticon bot for race duration algorithm validation
- [ ] Implement UDP packet monitoring for real-time OCR validation
- [ ] Polish OCR calibration tools for community contributions
- [ ] Review and clean up pending git changes

## If Blocked
No current blockers - zwift-offline integration working, 378 routes provide good coverage

## Failed Approaches
- WAD file extraction: wad_unpack.exe no longer available from referenced repos
- Merging events.txt/start_lines.txt: Files lack distance/elevation data
- Alternative tools (zwf): Only work with decompressed WAD files

## Related Documents
- docs/ROUTE_DATA_EXTRACTION.md - Technical findings on route data
- docs/ZWIFT_OFFLINE_INTEGRATION.md - How to use the integration
- REQUIREMENTS.md - Project requirements
- TODO.md - Active tasks  
- CLAUDE.md - Project instructions