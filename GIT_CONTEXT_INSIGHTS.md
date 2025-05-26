# Git-Inspired Project Context Management

## Core Git Principles We Should Adopt

### 1. Non-Destructive by Default
Git never loses data unless explicitly forced. Similarly:
- Never overwrite context files without explicit action
- Always preserve history of context switches
- Automatic backups before any destructive operation

### 2. Clear State Awareness
Git always shows current branch. We need:
- Always know which project context is active
- Visual indicators in prompts/terminals
- Quick status command showing context + code state

### 3. Cheap Context Creation
Creating a git branch is instant. We should have:
- `pc new <name>` creates context instantly
- Branching from current context (inherit state)
- Templates for common project types

## Git Workflows → Context Workflows

### Git Stash → Context Stash
```bash
# Git way
git stash         # save current work
git checkout main # switch branch
git stash pop     # restore work

# Context way
pc stash          # save current context
pc switch main    # switch project
pc stash pop      # restore context
```

### Git Branches → Context Branches
```bash
# Git way
git checkout -b feature/new-thing  # create and switch

# Context way
pc new feature/new-thing --from=current  # branch context
```

### Git Log → Context History
```bash
# Git way
git reflog        # see where HEAD has been

# Context way
pc log            # see context switch history
pc log --detail   # include file changes
```

## Integration Opportunities

### 1. Git-Aware Context Switching
When switching contexts, automatically:
- Check for uncommitted changes
- Optionally stash git changes
- Record git branch/commit with context
- Restore git state when returning

### 2. Unified Status
```bash
pc status
# Output:
# Project: zwift-race-finder
# Git Branch: main (clean)
# Context Files: HANDOFF.md (modified), PROJECT_WISDOM.md
# Active Since: 2025-05-26 10:00 UTC
# Last Switch: 4 hours ago (from: log-management)
```

### 3. Context Diffing
```bash
pc diff log-management
# Shows differences in context files between current and log-management
```

## Advanced Features from Git

### 1. Worktree-Inspired Isolation
While git worktrees duplicate code, we could:
- Have "context worktrees" - multiple active contexts
- Each in separate terminal/tmux session
- Share code but isolate context

### 2. Remote Contexts (like git remote)
- Backup contexts to cloud/shared location
- Share contexts between machines
- Team contexts for collaborative projects

### 3. Context Hooks (like git hooks)
- Pre-switch: validate/save state
- Post-switch: restore environment, open files
- Context-specific setup scripts

## Implementation Phases

### Phase 1: Git-Aware Basics ✅
- Stash/switch/status commands
- Git state preservation
- Non-destructive operations

### Phase 2: Git-Inspired Features
- Context branching/merging
- History/reflog
- Diff capabilities

### Phase 3: Advanced Integration
- Editor integration (restore open files)
- Terminal integration (restore working directory)
- Environment management

## Key Insight: Separation of Concerns

Git separates:
- **Repository State** (commits, branches)
- **Working State** (uncommitted changes)
- **Configuration** (.git/config)

We should separate:
- **Code State** (handled by git)
- **Context State** (our .md files, todos)
- **Environment State** (terminals, editors, etc)

## Safety Patterns from Git

1. **Dry Run Options**
   - `pc switch --dry-run` shows what would change

2. **Force Flags for Dangerous Operations**
   - `pc delete --force` to remove context

3. **Recovery Commands**
   - `pc recover` to restore from backups
   - `pc reflog` to see all context changes

4. **Atomic Operations**
   - All-or-nothing context switches
   - No partial state corruption