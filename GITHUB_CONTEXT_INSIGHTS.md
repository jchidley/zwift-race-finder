# GitHub-Inspired Project Context Management

## Key Insight: Separation of Layers

GitHub brilliantly separates:
1. **Code Layer** (git) - The actual files and history
2. **Work Layer** (issues/PRs) - What needs to be done
3. **Meta Layer** (projects/milestones) - How work is organized
4. **Knowledge Layer** (wiki/discussions) - Why decisions were made

Our context system has been conflating these layers!

## GitHub Patterns to Adopt

### 1. Persistent Work Items (Issues)
GitHub issues persist across branches. Similarly:
- TODOs should persist across context switches
- Link todos to specific contexts but allow cross-context view
- "Close" todos rather than delete them

### 2. Context Linking (like PR ↔ Issue)
```markdown
# In HANDOFF.md
Related Issues: #23, #45
Addresses: zwift-race-finder#12
Blocks: log-management context

# In TODO.md  
- [ ] Fix race predictions accuracy (#23)
- [ ] Implement stash command (blocks: pc-enhanced)
```

### 3. Project Views (like GitHub Projects)
Different views of the same data:
- **By Context**: Current view - files per project
- **By Status**: All active todos across contexts
- **By Timeline**: What was worked on when
- **By Dependency**: What blocks what

### 4. Milestone Thinking
GitHub uses milestones to group issues. We need:
```bash
pc milestone "MVP Release"        # Create milestone
pc todo add --milestone="MVP"     # Tag todos
pc milestone status "MVP"         # Show progress
```

### 5. Labels for Cross-Cutting Concerns
```bash
pc label "performance"            # Create label
pc todo add --label="bug,ux"     # Multi-label todos
pc list --label="security"        # Find across contexts
```

## Revolutionary Insight: Contexts ≠ Projects

We've been thinking "context = project" but GitHub shows us:
- **Repository** = Codebase boundary
- **Project** = Work organization (can span repos!)
- **Context** = Active working state

So we need:
```
Codebase (git repo)
  └── Multiple Contexts (working states)
       └── Linked to Projects (work organization)
            └── Containing Issues/Todos (work items)
```

## Implementation: GitHub-Aware Features

### 1. Issue Integration
```bash
pc todo add "Fix prediction accuracy" --github-issue
# Creates GitHub issue AND local todo
# Links them bidirectionally

pc sync github
# Updates todo status from GitHub
# Updates GitHub from local changes
```

### 2. PR-Aware Context Switching
```bash
pc switch --from-pr 123
# Creates context from PR description
# Imports PR todos/checklist
# Links back to PR for updates
```

### 3. Project Board Visualization
```bash
pc board
# Shows kanban-style view:
# TODO | IN PROGRESS | DONE
#  #1  |     #2      | #3
#  #4  |             | #5
```

### 4. Cross-Repository Contexts
Like GitHub Projects that span repos:
```bash
pc new "full-stack-feature" --repos="frontend,backend,docs"
# Context that tracks work across multiple repos
# Switches git branches in all repos together
```

## Advanced GitHub Concepts

### 1. Templates (like Issue Templates)
```bash
pc new --template=feature
# Populates with standard structure:
# - HANDOFF.md with feature template
# - TODO.md with standard checklist
# - Links to relevant docs
```

### 2. Automation (like GitHub Actions)
```bash
# .pc/hooks/post-switch
#!/bin/bash
# Run after context switch
if [[ -f "requirements.txt" ]]; then
    uv sync
fi
```

### 3. Discussions → Context Discussions
Long-form thinking that persists:
```bash
pc discuss "Architecture Decision: Pack Physics Model"
# Opens editor for long-form write-up
# Saved to context's DISCUSSIONS/ folder
# Searchable across all contexts
```

### 4. Wiki → Shared Knowledge Base
```bash
pc wiki "Zwift API Gotchas"
# Global knowledge, not context-specific
# Available from any context
# Version controlled separately
```

## The Hybrid Model

Combining Git + GitHub insights:

### Local-First (Git-style)
- Fast context switching
- Works offline
- Private by default
- Full control

### Connected Features (GitHub-style)
- Issue sync when online
- Shared project boards
- Team collaboration
- Backup to cloud

### Best of Both Worlds
```bash
pc status
# Local: zwift-race-finder context (modified)
# GitHub: Connected to repo, 3 linked issues
# Todos: 5 local, 2 synced with GitHub
# Offline: 2 hours (changes pending sync)
```

## Practical Architecture

```
~/.project-contexts/
├── .active -> zwift-race-finder
├── .wiki/                    # Shared knowledge
├── .templates/               # Context templates  
├── zwift-race-finder/
│   ├── HANDOFF.md
│   ├── TODO.md
│   ├── .github/              # GitHub integration
│   │   ├── issues.json       # Cached issue data
│   │   └── config.yml        # Repo mapping
│   └── DISCUSSIONS/          # Long-form thinking
└── log-management/
    └── ... (same structure)
```

## Immediate Wins

1. **Todo Persistence**: Todos survive context switches
2. **GitHub Issue Linking**: Reference real issues
3. **Cross-Context Search**: Find work items anywhere
4. **Templates**: Standardize context creation
5. **Offline-First**: Work anywhere, sync when connected

## Future Vision

The context manager becomes a **local-first project management layer** that:
- Integrates with GitHub when available
- Works perfectly offline
- Provides rich local workflows
- Syncs seamlessly when connected
- Respects both code boundaries and work boundaries