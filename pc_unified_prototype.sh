#!/bin/bash
# pc - Unified Project Context Manager
# Combines git workflows, GitHub integration, and persistent todos

set -euo pipefail

# Configuration
PC_HOME="${HOME}/.project-contexts"
ACTIVE_LINK="${PC_HOME}/.active"
TODOS_DB="${PC_HOME}/todos.db"
WIKI_DIR="${PC_HOME}/.wiki"
TEMPLATES_DIR="${PC_HOME}/.templates"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

# Initialize system
init_pc() {
    mkdir -p "$PC_HOME" "$WIKI_DIR" "$TEMPLATES_DIR"
    
    # Initialize todos database
    if [[ ! -f "$TODOS_DB" ]]; then
        sqlite3 "$TODOS_DB" <<EOF
CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    context TEXT NOT NULL,
    title TEXT NOT NULL,
    status TEXT DEFAULT 'todo',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    github_issue INTEGER,
    labels TEXT,
    milestone TEXT,
    priority INTEGER DEFAULT 0
);

CREATE TABLE context_state (
    context TEXT PRIMARY KEY,
    github_repo TEXT,
    last_sync DATETIME,
    metadata TEXT
);

CREATE INDEX idx_todos_context ON todos(context);
CREATE INDEX idx_todos_status ON todos(status);
EOF
    fi
}

# Get current context
current_context() {
    if [[ -L "$ACTIVE_LINK" ]]; then
        basename "$(readlink "$ACTIVE_LINK")"
    else
        echo "none"
    fi
}

# Todo management functions
todo_add() {
    local context=$(current_context)
    local title="$1"
    shift
    
    local github_issue=""
    local labels=""
    local milestone=""
    
    # Parse options
    while [[ $# -gt 0 ]]; do
        case $1 in
            --issue)
                # Create GitHub issue if requested
                if command -v gh &> /dev/null; then
                    github_issue=$(gh issue create --title "$title" --body "Created from pc todo" | grep -o '[0-9]\+$')
                    echo -e "${GREEN}Created GitHub issue #$github_issue${NC}"
                fi
                ;;
            --labels=*)
                labels="${1#*=}"
                ;;
            --milestone=*)
                milestone="${1#*=}"
                ;;
        esac
        shift
    done
    
    # Insert todo
    sqlite3 "$TODOS_DB" <<EOF
INSERT INTO todos (context, title, github_issue, labels, milestone)
VALUES ('$context', '$title', ${github_issue:-NULL}, '$labels', '$milestone');
EOF
    
    local todo_id=$(sqlite3 "$TODOS_DB" "SELECT last_insert_rowid();")
    echo -e "${GREEN}Added todo #$todo_id: $title${NC}"
}

todo_list() {
    local context=$(current_context)
    local filter="context = '$context'"
    
    # Check for --all flag
    if [[ "${1:-}" == "--all" ]]; then
        filter="1=1"
        echo -e "${BLUE}All Todos Across Contexts${NC}"
    else
        echo -e "${BLUE}Todos for context: $context${NC}"
    fi
    
    echo -e "━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    sqlite3 -column -header "$TODOS_DB" <<EOF
SELECT 
    id,
    CASE 
        WHEN status = 'todo' THEN '[ ]'
        WHEN status = 'in_progress' THEN '[~]'
        WHEN status = 'done' THEN '[x]'
    END as ' ',
    substr(title, 1, 40) as title,
    CASE WHEN github_issue IS NOT NULL THEN '#' || github_issue ELSE '' END as issue,
    CASE WHEN '$filter' = '1=1' THEN context ELSE '' END as context
FROM todos
WHERE $filter AND status != 'done'
ORDER BY priority DESC, id;
EOF
    
    # Show summary
    local todo_count=$(sqlite3 "$TODOS_DB" "SELECT COUNT(*) FROM todos WHERE $filter AND status = 'todo';")
    local in_progress=$(sqlite3 "$TODOS_DB" "SELECT COUNT(*) FROM todos WHERE $filter AND status = 'in_progress';")
    local done_today=$(sqlite3 "$TODOS_DB" "SELECT COUNT(*) FROM todos WHERE $filter AND status = 'done' AND date(updated_at) = date('now');")
    
    echo
    echo -e "Summary: ${GREEN}$todo_count todo${NC}, ${YELLOW}$in_progress in progress${NC}, ${BLUE}$done_today done today${NC}"
}

todo_update() {
    local todo_id="$1"
    local new_status="$2"
    
    sqlite3 "$TODOS_DB" <<EOF
UPDATE todos 
SET status = '$new_status', updated_at = CURRENT_TIMESTAMP
WHERE id = $todo_id;
EOF
    
    echo -e "${GREEN}Updated todo #$todo_id to $new_status${NC}"
    
    # Sync with GitHub if linked
    local github_issue=$(sqlite3 "$TODOS_DB" "SELECT github_issue FROM todos WHERE id = $todo_id;")
    if [[ -n "$github_issue" ]] && command -v gh &> /dev/null; then
        if [[ "$new_status" == "done" ]]; then
            gh issue close "$github_issue" 2>/dev/null && echo -e "${GREEN}Closed GitHub issue #$github_issue${NC}"
        fi
    fi
}

# Context switching with todo awareness
switch_context() {
    local target="$1"
    local current=$(current_context)
    
    # Show todo summary before switching
    if [[ "$current" != "none" ]]; then
        local pending=$(sqlite3 "$TODOS_DB" "SELECT COUNT(*) FROM todos WHERE context = '$current' AND status = 'in_progress';")
        if [[ $pending -gt 0 ]]; then
            echo -e "${YELLOW}Warning: $pending todos in progress in current context${NC}"
        fi
    fi
    
    # Save current context files
    if [[ "$current" != "none" ]]; then
        local context_dir="$PC_HOME/$current"
        mkdir -p "$context_dir"
        
        for file in HANDOFF.md PROJECT_WISDOM*.md *_LOG*.md; do
            [[ -f "$file" ]] && cp "$file" "$context_dir/" 2>/dev/null || true
        done
    fi
    
    # Create target if new
    local target_dir="$PC_HOME/$target"
    mkdir -p "$target_dir"
    
    # Restore target context files
    for file in "$target_dir"/*.md; do
        [[ -f "$file" ]] && cp "$file" . 2>/dev/null || true
    done
    
    # Update active link
    ln -sfn "$target" "$ACTIVE_LINK"
    
    echo -e "${GREEN}Switched to context: $target${NC}"
    
    # Show todos for new context
    todo_list
}

# GitHub sync
sync_github() {
    local context=$(current_context)
    
    echo -e "${BLUE}Syncing with GitHub...${NC}"
    
    # Get repo info
    local repo=$(git remote get-url origin 2>/dev/null | sed 's/.*github.com[:\/]\(.*\)\.git/\1/')
    
    if [[ -z "$repo" ]]; then
        echo -e "${RED}No GitHub remote found${NC}"
        return 1
    fi
    
    # Sync issues to todos
    if command -v gh &> /dev/null; then
        gh issue list --json number,title,state --limit 100 | jq -r '.[] | [.number, .title, .state] | @tsv' | while IFS=$'\t' read -r number title state; do
            # Check if todo exists
            local exists=$(sqlite3 "$TODOS_DB" "SELECT COUNT(*) FROM todos WHERE github_issue = $number AND context = '$context';")
            
            if [[ $exists -eq 0 ]]; then
                # Create new todo from issue
                sqlite3 "$TODOS_DB" <<EOF
INSERT INTO todos (context, title, github_issue, status)
VALUES ('$context', '$title', $number, '$([ "$state" = "CLOSED" ] && echo "done" || echo "todo")');
EOF
                echo -e "${GREEN}Imported issue #$number: $title${NC}"
            else
                # Update existing todo
                local todo_status=$([ "$state" = "CLOSED" ] && echo "done" || echo "todo")
                sqlite3 "$TODOS_DB" "UPDATE todos SET status = '$todo_status' WHERE github_issue = $number AND context = '$context';"
            fi
        done
        
        # Update sync timestamp
        sqlite3 "$TODOS_DB" "INSERT OR REPLACE INTO context_state (context, github_repo, last_sync) VALUES ('$context', '$repo', CURRENT_TIMESTAMP);"
        
        echo -e "${GREEN}Sync complete${NC}"
    else
        echo -e "${RED}gh CLI not found - install from https://cli.github.com${NC}"
    fi
}

# Enhanced status showing todos
show_status() {
    local context=$(current_context)
    
    echo -e "${BLUE}Project Context Status${NC}"
    echo -e "━━━━━━━━━━━━━━━━━━━━━"
    echo -e "Context: ${GREEN}$context${NC}"
    
    # Git info
    if git rev-parse --git-dir > /dev/null 2>&1; then
        local branch=$(git branch --show-current 2>/dev/null || echo "detached")
        local changes=$(git status --porcelain 2>/dev/null | wc -l)
        echo -e "Git: $branch (${changes} changes)"
    fi
    
    # GitHub connection
    local github_repo=$(sqlite3 "$TODOS_DB" "SELECT github_repo FROM context_state WHERE context = '$context';" 2>/dev/null)
    if [[ -n "$github_repo" ]]; then
        echo -e "GitHub: $github_repo"
    fi
    
    # Todo summary
    echo
    todo_list
}

# Main command dispatcher
init_pc

case "${1:-status}" in
    status|s)
        show_status
        ;;
    
    switch|sw)
        switch_context "${2:?Usage: pc switch <context>}"
        ;;
    
    todo)
        case "${2:-list}" in
            add)
                todo_add "${3:?Usage: pc todo add <title>}" "${@:4}"
                ;;
            list|ls)
                todo_list "${3:-}"
                ;;
            start)
                todo_update "${3:?Usage: pc todo start <id>}" "in_progress"
                ;;
            done)
                todo_update "${3:?Usage: pc todo done <id>}" "done"
                ;;
            *)
                echo -e "${RED}Unknown todo command: $2${NC}"
                ;;
        esac
        ;;
    
    sync)
        sync_github
        ;;
    
    init)
        if [[ "${2:-}" == "--github" ]]; then
            local context=$(current_context)
            local repo=$(git remote get-url origin 2>/dev/null | sed 's/.*github.com[:\/]\(.*\)\.git/\1/')
            sqlite3 "$TODOS_DB" "INSERT OR REPLACE INTO context_state (context, github_repo) VALUES ('$context', '$repo');"
            echo -e "${GREEN}Connected context to GitHub repo: $repo${NC}"
        fi
        ;;
    
    help|h)
        cat <<EOF
${BLUE}pc - Unified Project Context Manager${NC}

${YELLOW}Core Commands:${NC}
  status, s          Show current context and todos
  switch, sw         Switch to different context
  
${YELLOW}Todo Management:${NC}  
  todo add           Add a new todo
    --issue         Also create GitHub issue
    --labels=a,b    Add labels
    --milestone=v1  Set milestone
  todo list          List todos (add --all for all contexts)
  todo start <id>    Mark todo as in progress
  todo done <id>     Mark todo as done
  
${YELLOW}GitHub Integration:${NC}
  init --github      Connect current context to GitHub repo
  sync              Sync todos with GitHub issues
  
${YELLOW}Examples:${NC}
  pc switch my-feature
  pc todo add "Fix authentication bug" --issue
  pc todo start 123
  pc sync
EOF
        ;;
    
    *)
        echo -e "${RED}Unknown command: $1${NC}"
        echo "Use 'pc help' for usage"
        ;;
esac