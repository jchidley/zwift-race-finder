#!/bin/bash
# pc - Project Context Manager (Git-Inspired Version)
# Manages project contexts with git-like workflows

set -euo pipefail

# Configuration
CONTEXTS_DIR="${HOME}/.project-contexts"
ACTIVE_LINK="${CONTEXTS_DIR}/.active"
STASH_DIR="${CONTEXTS_DIR}/.stash"
HISTORY_FILE="${CONTEXTS_DIR}/.history"
BACKUP_DIR="${CONTEXTS_DIR}/.backups"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Ensure directories exist
mkdir -p "$CONTEXTS_DIR" "$STASH_DIR" "$BACKUP_DIR"
touch "$HISTORY_FILE"

# Utility functions
current_context() {
    if [[ -L "$ACTIVE_LINK" ]]; then
        basename "$(readlink "$ACTIVE_LINK")"
    else
        echo "none"
    fi
}

current_git_info() {
    if git rev-parse --git-dir > /dev/null 2>&1; then
        local branch=$(git branch --show-current 2>/dev/null || echo "detached")
        local status="clean"
        if [[ -n $(git status --porcelain 2>/dev/null) ]]; then
            status="dirty"
        fi
        echo "$branch ($status)"
    else
        echo "not a git repo"
    fi
}

record_history() {
    echo "$(date -u +%Y-%m-%dT%H:%M:%SZ) | $1" >> "$HISTORY_FILE"
}

backup_context() {
    local context="$1"
    local backup_name="${context}_$(date +%Y%m%d_%H%M%S)"
    local context_dir="$CONTEXTS_DIR/$context"
    
    if [[ -d "$context_dir" ]]; then
        cp -r "$context_dir" "$BACKUP_DIR/$backup_name"
        echo "Backed up to: $BACKUP_DIR/$backup_name"
    fi
}

# Command: status
cmd_status() {
    local current=$(current_context)
    local git_info=$(current_git_info)
    
    echo -e "${BLUE}Project Context Status${NC}"
    echo -e "━━━━━━━━━━━━━━━━━━━━━"
    echo -e "Context: ${GREEN}$current${NC}"
    echo -e "Git: $git_info"
    
    if [[ "$current" != "none" ]]; then
        local context_dir="$CONTEXTS_DIR/$current"
        echo -e "\nContext Files:"
        for file in HANDOFF.md PROJECT_WISDOM.md TODO.md; do
            if [[ -f "$context_dir/$file" ]]; then
                local mod_time=$(stat -c %y "$context_dir/$file" 2>/dev/null | cut -d' ' -f1 || date -r "$context_dir/$file" +%Y-%m-%d)
                echo -e "  • $file (modified: $mod_time)"
            fi
        done
        
        # Show recent history
        echo -e "\nRecent Activity:"
        tail -n 3 "$HISTORY_FILE" 2>/dev/null | while read -r line; do
            echo -e "  $line"
        done
    fi
}

# Command: switch
cmd_switch() {
    local target="$1"
    local current=$(current_context)
    
    # Check for uncommitted git changes
    if git rev-parse --git-dir > /dev/null 2>&1; then
        if [[ -n $(git status --porcelain 2>/dev/null) ]]; then
            echo -e "${YELLOW}Warning: You have uncommitted git changes${NC}"
            read -p "Stash git changes? (y/N) " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                git stash push -m "pc: switching from $current to $target"
            fi
        fi
    fi
    
    # Save current context state if exists
    if [[ "$current" != "none" && -d "$CONTEXTS_DIR/$current" ]]; then
        # Copy current project files to context
        for file in HANDOFF.md PROJECT_WISDOM*.md TODO.md *_LOG*.md; do
            if [[ -f "$file" ]]; then
                cp "$file" "$CONTEXTS_DIR/$current/" 2>/dev/null || true
            fi
        done
        record_history "saved context: $current"
    fi
    
    # Create target context if new
    local target_dir="$CONTEXTS_DIR/$target"
    if [[ ! -d "$target_dir" ]]; then
        echo -e "${GREEN}Creating new context: $target${NC}"
        mkdir -p "$target_dir"
        record_history "created context: $target"
    fi
    
    # Backup current project files before switching
    for file in HANDOFF.md PROJECT_WISDOM*.md TODO.md *_LOG*.md; do
        if [[ -f "$file" ]]; then
            backup_name="${file}.before-switch-$(date +%Y%m%d_%H%M%S)"
            cp "$file" "$backup_name" 2>/dev/null || true
        fi
    done
    
    # Restore target context files
    for file in "$target_dir"/*.md; do
        if [[ -f "$file" ]]; then
            cp "$file" . 2>/dev/null || true
        fi
    done
    
    # Update active link
    ln -sfn "$target" "$ACTIVE_LINK"
    record_history "switched to: $target (from: $current)"
    
    echo -e "${GREEN}Switched to context: $target${NC}"
    cmd_status
}

# Command: stash
cmd_stash() {
    local current=$(current_context)
    if [[ "$current" == "none" ]]; then
        echo -e "${RED}No active context to stash${NC}"
        exit 1
    fi
    
    local stash_name="${current}_$(date +%Y%m%d_%H%M%S)"
    local stash_path="$STASH_DIR/$stash_name"
    
    mkdir -p "$stash_path"
    
    # Stash context files
    for file in HANDOFF.md PROJECT_WISDOM*.md TODO.md *_LOG*.md; do
        if [[ -f "$file" ]]; then
            cp "$file" "$stash_path/" 2>/dev/null || true
        fi
    done
    
    # Save git info
    echo "git_branch=$(git branch --show-current 2>/dev/null || echo 'none')" > "$stash_path/.git_info"
    
    record_history "stashed: $current as $stash_name"
    echo -e "${GREEN}Stashed current context as: $stash_name${NC}"
}

# Command: stash pop
cmd_stash_pop() {
    if [[ ! -d "$STASH_DIR" ]] || [[ -z "$(ls -A "$STASH_DIR" 2>/dev/null)" ]]; then
        echo -e "${RED}No stashes found${NC}"
        exit 1
    fi
    
    # Get most recent stash
    local latest_stash=$(ls -t "$STASH_DIR" | head -n1)
    local stash_path="$STASH_DIR/$latest_stash"
    
    echo -e "${YELLOW}Popping stash: $latest_stash${NC}"
    
    # Restore files
    for file in "$stash_path"/*.md; do
        if [[ -f "$file" ]]; then
            cp "$file" . 2>/dev/null || true
        fi
    done
    
    # Clean up
    rm -rf "$stash_path"
    record_history "popped stash: $latest_stash"
    
    echo -e "${GREEN}Stash popped successfully${NC}"
}

# Command: log
cmd_log() {
    echo -e "${BLUE}Context History${NC}"
    echo -e "━━━━━━━━━━━━━━━"
    
    if [[ -f "$HISTORY_FILE" ]]; then
        tail -n 20 "$HISTORY_FILE" | tac
    else
        echo "No history found"
    fi
}

# Command: list
cmd_list() {
    local current=$(current_context)
    
    echo -e "${BLUE}Available Contexts${NC}"
    echo -e "━━━━━━━━━━━━━━━━"
    
    for context_dir in "$CONTEXTS_DIR"/*/; do
        if [[ -d "$context_dir" ]]; then
            local context=$(basename "$context_dir")
            if [[ "$context" == "$current" ]]; then
                echo -e "* ${GREEN}$context${NC} (active)"
            else
                echo -e "  $context"
            fi
        fi
    done
    
    # Show stashes if any
    if [[ -d "$STASH_DIR" ]] && [[ -n "$(ls -A "$STASH_DIR" 2>/dev/null)" ]]; then
        echo -e "\n${BLUE}Stashes${NC}"
        echo -e "━━━━━━━━"
        ls -t "$STASH_DIR" | head -n 5
    fi
}

# Command: diff
cmd_diff() {
    local target="${1:-}"
    local current=$(current_context)
    
    if [[ -z "$target" ]]; then
        echo -e "${RED}Usage: pc diff <context>${NC}"
        exit 1
    fi
    
    if [[ ! -d "$CONTEXTS_DIR/$target" ]]; then
        echo -e "${RED}Context not found: $target${NC}"
        exit 1
    fi
    
    echo -e "${BLUE}Diff: $current vs $target${NC}"
    echo -e "━━━━━━━━━━━━━━━━━━━━━━━"
    
    for file in HANDOFF.md PROJECT_WISDOM.md TODO.md; do
        if [[ -f "$file" ]] || [[ -f "$CONTEXTS_DIR/$target/$file" ]]; then
            echo -e "\n${YELLOW}=== $file ===${NC}"
            diff -u "$CONTEXTS_DIR/$target/$file" "$file" 2>/dev/null || echo "(differences found)"
        fi
    done
}

# Main command dispatcher
case "${1:-status}" in
    status|s)
        cmd_status
        ;;
    switch|sw)
        if [[ -z "${2:-}" ]]; then
            echo -e "${RED}Usage: pc switch <context>${NC}"
            exit 1
        fi
        cmd_switch "$2"
        ;;
    stash)
        if [[ "${2:-}" == "pop" ]]; then
            cmd_stash_pop
        else
            cmd_stash
        fi
        ;;
    log|l)
        cmd_log
        ;;
    list|ls)
        cmd_list
        ;;
    diff|d)
        cmd_diff "${2:-}"
        ;;
    help|h)
        echo "pc - Project Context Manager (Git-Inspired)"
        echo
        echo "Commands:"
        echo "  status, s        Show current context status"
        echo "  switch, sw       Switch to a different context"  
        echo "  stash           Save current context temporarily"
        echo "  stash pop       Restore most recent stash"
        echo "  list, ls        List all contexts and stashes"
        echo "  log, l          Show context history"
        echo "  diff, d         Compare contexts"
        echo "  help, h         Show this help"
        ;;
    *)
        echo -e "${RED}Unknown command: $1${NC}"
        echo "Use 'pc help' for usage"
        exit 1
        ;;
esac