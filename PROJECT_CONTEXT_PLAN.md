# Project Context Management Plan

## Problem Statement
Current slash commands assume single project context, leading to:
- HANDOFF.md overwrites when switching projects
- Mixed insights in PROJECT_WISDOM.md
- Lost state when working on side projects
- No way to resume previous contexts

## Solution: Project Context Manager

### Architecture
```
~/.project-contexts/
├── active -> zwift-race-finder     # Symlink to current
├── zwift-race-finder/
│   ├── HANDOFF.md
│   ├── PROJECT_WISDOM.md (hierarchical)
│   ├── *_LOG.md files
│   └── todo.md
├── log-management/
│   ├── HANDOFF.md
│   └── PROJECT_WISDOM.md
└── registry.json
```

### Core Commands
```bash
pc [status]                  # Show current project
pc switch <project>          # Switch context (fuzzy match)
pc new <project>             # Create new context
pc list                      # List all projects
pc stash                     # Quick tangent (saves current)
pc pop                       # Return from tangent
```

### Slash Command Integration
All commands automatically use active project context:
- `/checkpoint` → Updates `{active}/HANDOFF.md`
- `/log` → Appends to `{active}/*_LOG.md`
- `/plan` → Creates `{active}/todo.md`

### Migration Strategy
1. Create ~/.project-contexts/
2. Move current project files to named directory
3. Create 'active' symlink
4. Update slash commands to read active context
5. Test with multiple projects

### Implementation Phases

#### Phase 1: Basic Infrastructure
- Create `pc` command script
- Implement switch/list/new commands
- Test with manual file management

#### Phase 2: Slash Command Updates
- Modify commands to use ~/.project-contexts/active/
- Add context awareness to file operations
- Ensure backward compatibility

#### Phase 3: Enhanced Features
- Project registry with metadata
- Auto-detection for git repos
- Stash/pop for quick tangents
- Project tags and search

### Benefits
- **Isolation**: Each project's state is preserved
- **Flexibility**: Easy context switching
- **Discovery**: See all projects with `pc list`
- **Natural**: Works with existing workflow
- **Extensible**: Can add features without breaking core

### Example Workflow
```bash
# Working on Zwift Race Finder
pc switch zwift
/checkpoint              # Updates zwift context

# Quick side project
pc new log-management
/plan                    # Creates plan in new context
# ... work on side project ...

# Back to main project  
pc switch zwift          # All state preserved
/checkpoint              # Continues where left off

# Exploration tangent
pc stash                 # Save current state
# ... try something risky ...
pc pop                   # Restore if needed
```