# Files Review List for REQUIREMENTS.md Update

This document tracks all *.md files in the project, sorted by modification time (newest first), for systematic review to update REQUIREMENTS.md based on the most recent user needs.

## Review Status Legend
- ✅ Reviewed and incorporated into REQUIREMENTS.md
- 🔄 In progress
- ⏳ Pending review

## Files List (Newest to Oldest)

1. ✅ `/home/jack/tools/rust/zwift-race-finder/todo.md` - Main task tracking
2. ✅ `/home/jack/tools/rust/zwift-race-finder/HANDOFF.md` - Current state and next steps
3. ✅ `/home/jack/tools/rust/zwift-race-finder/REQUIREMENTS.md` - Requirements document (target)
4. ✅ `/home/jack/tools/rust/zwift-race-finder/sessions/ZWIFT_API_LOG_SESSION_20250527_007.md` - Code cleanup session
5. ✅ `/home/jack/tools/rust/zwift-race-finder/PROJECT_WISDOM.md` - Technical insights
6. ✅ `/home/jack/tools/rust/zwift-race-finder/ZWIFT_API_LOG_RECENT.md` - Recent session summaries
7. ✅ `/home/jack/tools/rust/zwift-race-finder/sessions/ZWIFT_API_LOG_SESSION_20250527_006.md` - Production deployment
8. ✅ `/home/jack/tools/rust/zwift-race-finder/DEPLOYMENT.md` - Deployment guide
9. ✅ `/home/jack/tools/rust/zwift-race-finder/FEEDBACK.md` - User feedback collection
10. ✅ `/home/jack/tools/rust/zwift-race-finder/README.md` - Project overview
11. ✅ `/home/jack/tools/rust/zwift-race-finder/sessions/ZWIFT_API_LOG_SESSION_20250527_005.md` - Multi-lap accuracy achievement
12. ✅ `/home/jack/tools/rust/zwift-race-finder/sessions/ZWIFT_API_LOG_SESSION_20250527_004.md` - Multi-lap production testing
13. ✅ `/home/jack/tools/rust/zwift-race-finder/sessions/ZWIFT_API_LOG_SESSION_20250527_003.md` - Multi-lap implementation
14. ✅ `/home/jack/tools/rust/zwift-race-finder/sessions/ZWIFT_API_LOG_SESSION_20250527_002.md` - Web research session
15. ✅ `/home/jack/tools/rust/zwift-race-finder/ROUTE_MAPPING_RESEARCH.md` - Route mapping findings
16. ✅ `/home/jack/tools/rust/zwift-race-finder/sessions/PROJECT_WISDOM_SESSION_20250527_001.md` - Manual route mappings
17. ✅ `/home/jack/tools/rust/zwift-race-finder/sessions/ZWIFT_API_LOG_SESSION_20250527_001.md` - Batch discovery
18. ✅ `/home/jack/tools/rust/zwift-race-finder/sessions/ZWIFT_API_LOG_SESSION_20250526_005.md` - World detection
19. ✅ `/home/jack/tools/rust/zwift-race-finder/sessions/ZWIFT_API_LOG_SESSION_20250526_004.md` - Route discovery enhancement
20. ✅ `/home/jack/tools/rust/zwift-race-finder/sessions/ZWIFT_API_LOG_SESSION_20250526_003.md` - Event description parsing
21. ✅ `/home/jack/tools/rust/zwift-race-finder/plan.md` - Project plan and milestones
22. ✅ `/home/jack/tools/rust/zwift-race-finder/sessions/ZWIFT_API_LOG_SESSION_20250526_002.md` - Racing Score fix
23. ✅ `/home/jack/tools/rust/zwift-race-finder/CLAUDE.md` - AI development instructions
24. ✅ `/home/jack/tools/rust/zwift-race-finder/log_with_context.md` - Logging strategy
25. ✅ `/home/jack/tools/rust/zwift-race-finder/PROJECT_CONTEXT_TODO.md` - Context management todo
26. ✅ `/home/jack/tools/rust/zwift-race-finder/sessions/PROJECT_WISDOM_SESSION_20250526_001.md` - Pack dynamics
27. ✅ `/home/jack/tools/rust/zwift-race-finder/PROJECT_WISDOM_RECENT.md` - Recent insights
28. ✅ `/home/jack/tools/rust/zwift-race-finder/PROJECT_WISDOM_SUMMARY.md` - Project wisdom summary
29. ✅ `/home/jack/tools/rust/zwift-race-finder/ZWIFT_API_LOG.md` - API log index
30. ✅ `/home/jack/tools/rust/zwift-race-finder/sessions/ZWIFT_API_LOG_SESSION_20250526_001.md` - UX improvements
31. ✅ `/home/jack/tools/rust/zwift-race-finder/sessions/ZWIFT_API_LOG_SESSION_20250525_001.md` - Drop dynamics
32. ✅ `/home/jack/tools/rust/zwift-race-finder/ZWIFT_API_LOG_SUMMARY.md` - Executive summary
33. ✅ `/home/jack/tools/rust/zwift-race-finder/docs/screenshots/README.md` - Screenshot documentation
34. ✅ `/home/jack/tools/rust/zwift-race-finder/ACCURACY_TIMELINE.md` - Accuracy history
35. ✅ `/home/jack/tools/rust/zwift-race-finder/HANDOFF_2025-05-25.md` - Previous handoff
36. ✅ `/home/jack/tools/rust/zwift-race-finder/AI_DEVELOPMENT.md` - AI development approach
37. ✅ `/home/jack/tools/rust/zwift-race-finder/PHYSICAL_STATS.md` - Physics calculations
38. ✅ `/home/jack/tools/rust/zwift-race-finder/GITHUB_PUBLISH_LOG.md` - GitHub publishing
39. ✅ `/home/jack/tools/rust/zwift-race-finder/ZWIFTPOWER_EXPORT_STEPS.md` - Data export guide
40. ✅ `/home/jack/tools/rust/zwift-race-finder/SECURITY_AUDIT.md` - Security review
41. ✅ `/home/jack/tools/rust/zwift-race-finder/BITWARDEN_SETUP.md` - Password manager setup

## Comprehensive Review Complete (All 41 Files ✅)

### Highest Priority Issues
1. **User Concern**: "I'm not convinced that the program is working as I'd like" (from HANDOFF.md)
2. **Security**: OAuth tokens stored in plain text, personal data hardcoded (from SECURITY_AUDIT.md)
3. **Configuration**: Need seamless personal config that survives sanitization
4. **Physics**: Height/weight stats collected but not fully utilized for predictions
5. **API Limits**: 200 event hard limit needs clear user communication

### Key Technical Discoveries
1. **Pack Dynamics**: Getting dropped explains 82.6% of variance (binary state: pack or solo)
2. **Event Types**: Two systems - Traditional vs Racing Score (50% of events affected)
3. **Route Discovery**: Most "unknowns" are custom event names needing manual mapping
4. **Multi-lap Fixes**: Pattern matching reduced errors from 533% to 16%
5. **AI Development**: Human expertise + AI implementation = successful product

### Security Requirements Added
- Bitwarden integration for secure token storage
- Pre-commit hooks to prevent secret commits
- Sanitization scripts for public release
- Multiple secure config options (GPG, secure dir, env vars)

### REQUIREMENTS.md Updated
- Added all security requirements from audit
- Enhanced configuration management section
- Added physics modeling details
- Documented critical discoveries
- Updated revision history with comprehensive review