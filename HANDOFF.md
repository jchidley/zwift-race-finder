# Project: Zwift Race Finder
Updated: 2025-06-19 21:55:00

## Current State
Status: Documentation reorganization complete, accuracy research done
Target: 20.4% mean absolute error on race time predictions
Latest: Discovered draft modeling already implicit in empirical category speeds

## Essential Context
- 378 routes in database from third-party sources (ZwiftHacks, WhatsOnZwift)
- Category speeds (30.9 km/h for Cat D) derived from 151 real races, include draft benefits
- Dual-speed model exists in code but unused - simple empirical model achieves 20.4% MAE
- CdA formula (A = 0.0276·h^0.725·m^0.425) is community reverse-engineered, not official
- Racing guides created emphasizing power as only controllable variable during races

## Next Step
Execute documentation migration plan to reorganize docs/ structure

## Active Todo List
[✓] Analyze documentation for different audiences (users, developers, maintainers)
[✓] Design organization that serves both racing optimization AND ongoing development
[✓] Create navigation strategy to help people find what they need
[✓] Identify documents that need consolidation vs those that should remain separate
[✓] Create detailed migration plan showing which files go where
[✓] Write navigation READMEs once migration plan is approved
[✓] Create user-focused racing guides once structure is approved

Previous todos (for reference):
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