# Log Management Improvement Plan

## Problem Statement

Current log files (e.g., ZWIFT_API_LOG_2025-05-25.md at 56KB) are becoming too large to efficiently load into LLM context. We need a system that:
- Preserves all historical data
- Minimizes context usage (target: <5KB per session)
- Works with existing slash commands (/start, /checkpoint, /log, /update-project)
- Handles dynamic log file names across different projects

## Proposed Architecture

### File Structure Pattern
```
PROJECT_LOG_SUMMARY.md      # Living executive summary (3KB max)
PROJECT_LOG_RECENT.md       # Last 3-5 sessions only (2KB max)
logs/
  └── session_YYYY-MM-DD_HHMM_topic.md  # Full session archives
```

Where PROJECT can be any prefix (ZWIFT_API, STRAVA, API, etc.)

### Key Design Decisions

1. **Session-based, not day-based**: Aligns with how commands actually work
2. **Dynamic discovery**: No hardcoded filenames in commands
3. **Append-only archives**: Never need to read large files to add content
4. **Living summaries**: Updated incrementally each session

## Implementation Plan

### Phase 1: Architecture Design
- [x] Design flexible log naming pattern
- [ ] Create session-based log specification
- [ ] Document file size limits and rotation rules

### Phase 2: Command Updates
- [ ] Update /log command:
  - Discover existing *_LOG patterns dynamically
  - Create session archives in logs/ directory
  - Update *_LOG_RECENT.md (rotate old entries)
  - Update *_LOG_SUMMARY.md for major discoveries
- [ ] Update /start command:
  - Read HANDOFF.md (primary)
  - Read *_LOG_RECENT.md if exists (supplementary)
  - Never read full logs or archives
- [ ] Update /update-project:
  - Similar to /log but more comprehensive
  - Work from session memory, not file reads

### Phase 3: Migration
- [ ] Create migration script for existing logs:
  - Split ZWIFT_API_LOG_2025-05-25.md → session files
  - Extract executive summary → ZWIFT_API_LOG_SUMMARY.md
  - Extract recent sessions → ZWIFT_API_LOG_RECENT.md
- [ ] Test with current project logs
- [ ] Document migration process

### Phase 4: Automation
- [ ] Create helper scripts:
  - log_rotate.sh: Archive old sessions
  - log_summarize.sh: Generate summaries from sessions
  - log_cleanup.sh: Remove old archives per retention policy

## File Specifications

### *_LOG_SUMMARY.md (3KB max)
```markdown
# [PROJECT] Log Summary

## Executive Summary
[200-word overview of entire project]

## Key Discoveries
1. [Most important finding]
2. [Second most important]
... (max 10)

## Current Status
- Performance: [metric]
- Last Updated: [date]
- Sessions Logged: [count]

## Quick Reference
[Essential commands/configs]
```

### *_LOG_RECENT.md (2KB max)
```markdown
# Recent Sessions

## Session: 2025-05-27 14:30 - Topic
- Key outcome 1
- Key outcome 2
- Full log: logs/session_2025-05-27_1430_topic.md

## Session: 2025-05-27 09:00 - Topic
...
[Max 5 sessions]
```

### logs/session_*.md (No limit)
Full session details, never loaded unless specifically requested

## Success Criteria

1. Context usage reduced from 56KB to <5KB per session
2. No data loss - all historical information preserved
3. Commands work seamlessly with new structure
4. Pattern works across all projects (not Zwift-specific)

## Migration Timeline

- Week 1: Implement and test command updates
- Week 2: Migrate existing logs
- Week 3: Monitor and refine based on usage

## Notes

- This plan focuses on sustainable log management
- Session-based approach aligns with actual workflow
- Dynamic discovery ensures flexibility across projects
- Incremental summaries avoid expensive "read all" operations