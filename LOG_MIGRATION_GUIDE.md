# Log Migration Guide

## Overview
This guide documents the migration from a single large log file (66KB+) to a hierarchical log management system that keeps active context under 5KB.

## Migration Completed (2025-05-26)

### What Changed
1. **Before**: Single ZWIFT_API_LOG.md file plus dated archives (66KB total)
2. **After**: Hierarchical structure with:
   - Main log file (< 1KB) with references
   - Summary file (< 3KB) with executive summary
   - Recent file (< 2KB) with latest sessions
   - Sessions directory with full archives

### Files Created
- `ZWIFT_API_LOG_SUMMARY.md` - Executive summary of entire project
- `ZWIFT_API_LOG_RECENT.md` - Latest session details
- `sessions/ZWIFT_API_LOG_SESSION_20250525_001.md` - Archived from ZWIFT_API_LOG_2025-05-25.md
- `sessions/ZWIFT_API_LOG_SESSION_20250526_001.md` - Archived from ZWIFT_API_LOG_2025-05-26.md

### Files Modified
- `ZWIFT_API_LOG.md` - Now a lightweight index file
- `/log` command - Updated to handle hierarchical structure

## How It Works

### Reading Logs
When starting a session, only these files are loaded:
1. Main log file (references and active session)
2. Summary file (if context needed)
3. Recent file (if working on related topics)

Total context: < 5KB instead of 66KB

### Writing Logs
New session content is appended to main log under "Active Session" section.

### Archiving Process
When active session exceeds 5KB:
1. Move content to `sessions/TOPIC_LOG_SESSION_YYYYMMDD_NNN.md`
2. Update RECENT file with key points
3. Clear active session in main log
4. Update SUMMARY if needed (major breakthroughs only)

## Benefits Achieved
- **Performance**: 13x reduction in context size (66KB → 5KB)
- **Efficiency**: No need to read entire file to append
- **Preservation**: All historical data retained in archives
- **Flexibility**: Session-based archives, not day-based
- **Discovery**: Pattern matching finds all *_LOG.md files

## Future Logs
Apply same pattern to any log that exceeds 10KB:
1. Create TOPIC_LOG_SUMMARY.md
2. Create TOPIC_LOG_RECENT.md
3. Move old content to sessions/
4. Update main file to reference structure