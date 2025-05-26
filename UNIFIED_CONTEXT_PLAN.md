# Unified Project Context Management Plan

## Vision: Local-First Project Management

A tool that combines:
- **Git's** version control wisdom (branches, stashing, history)
- **GitHub's** project management insights (issues, projects, discussions)
- **LLM's** context needs (compact, relevant, hierarchical)

## Core Principle: Separation of Concerns

```
Code State (Git)
  ├── What files exist and their content
  └── Managed by: git

Work State (Context)
  ├── What needs to be done (TODOs)
  ├── Current understanding (HANDOFF)
  ├── Learned wisdom (PROJECT_WISDOM)
  └── Managed by: pc (project context)

Meta State (GitHub/Projects)
  ├── How work is organized
  ├── External references
  └── Managed by: pc + GitHub integration
```

## Implementation Phases

### Phase 1: Core Context Manager ✅
- [x] Basic switch/save/restore
- [x] Git-aware (check dirty state)
- [x] Non-destructive operations
- [x] Stash functionality

### Phase 2: Work Item Management (Current)
- [ ] Persistent todos across contexts
- [ ] Todo labels and milestones  
- [ ] Cross-context todo search
- [ ] Todo dependencies

### Phase 3: GitHub Integration
- [ ] Link todos to GitHub issues
- [ ] Create issues from todos
- [ ] Sync status bidirectionally
- [ ] PR-based context creation

### Phase 4: Knowledge Management
- [ ] Wiki for shared knowledge
- [ ] Discussions for decisions
- [ ] Templates for contexts
- [ ] Search across everything

## Immediate Next Steps

### 1. Install Enhanced PC Command
```bash
cp pc_enhanced.sh ~/.local/bin/pc
chmod +x ~/.local/bin/pc
```

### 2. Restructure for Persistent Todos
Instead of TODO.md per context, use:
```
~/.project-contexts/
├── .todos.db              # SQLite for all todos
├── zwift-race-finder/
│   ├── HANDOFF.md
│   └── .context.yml      # Links to todos, issues
```

### 3. Create Todo Management Commands
```bash
pc todo add "Fix prediction accuracy" --context=zwift
pc todo list                    # Current context
pc todo list --all             # All contexts
pc todo move 123 log-management # Move between contexts
```

### 4. Add GitHub Awareness
```bash
pc init --github              # Connect to repo
pc todo add --issue          # Create GitHub issue too
pc sync                      # Sync with GitHub
```

## Usage Patterns

### Starting Work
```bash
cd ~/project
pc switch my-feature         # or create new
pc sync                     # Pull latest from GitHub
pc todo list               # See what needs doing
```

### During Work
```bash
pc todo add "Discovered bug in parser"
pc todo start 123          # Mark as in-progress
# ... work work work ...
pc todo done 123          # Mark complete
```

### Switching Contexts
```bash
pc stash                   # Save current state
pc switch other-project    # Change context
# ... work on other thing ...
pc switch my-feature       # Return
pc stash pop              # Restore if needed
```

### Collaboration
```bash
pc todo add --issue "Breaking change in API"
# Creates GitHub issue #456

pc pr-ready               # Check if ready for PR
# - All todos complete? 
# - Tests passing?
# - Context documented?

pc handoff generate       # Create handoff for teammate
```

## Benefits

1. **Never Lose Context**: Everything is saved automatically
2. **GitHub Integration**: Real issues, not just local notes
3. **Offline-First**: Full functionality without internet
4. **LLM-Optimized**: Hierarchical, compact context loading
5. **Git-Integrated**: Respects version control boundaries

## Technical Architecture

### Storage
- **SQLite** for todos, history, metadata (like git's .git/objects)
- **Markdown files** for human-readable content
- **JSON/YAML** for configuration and state

### Commands (Git-Inspired)
- `pc switch` (like `git checkout`)
- `pc stash` (like `git stash`)  
- `pc log` (like `git log`)
- `pc status` (like `git status`)
- `pc diff` (like `git diff`)

### GitHub Integration
- Use `gh` CLI for API calls
- Cache data locally for offline
- Sync on explicit command or hooks

### Safety
- Automatic backups before changes
- History of all operations
- Recovery commands
- Dry-run options

## End Goal

A developer can:
1. Switch between multiple projects instantly
2. Never lose track of what they were doing
3. Integrate seamlessly with GitHub
4. Work effectively offline
5. Share context with teammates
6. Let LLMs load exactly the right context

All while feeling as natural as using git.