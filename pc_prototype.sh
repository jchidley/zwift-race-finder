#!/bin/bash
# Project Context Manager - Prototype
# Install: cp pc_prototype.sh ~/.local/bin/pc && chmod +x ~/.local/bin/pc

CONTEXTS_DIR="$HOME/.project-contexts"
ACTIVE_LINK="$CONTEXTS_DIR/active"
REGISTRY="$CONTEXTS_DIR/registry.json"

# Ensure contexts directory exists
mkdir -p "$CONTEXTS_DIR"

# Helper to get current project name
current_project() {
    if [[ -L "$ACTIVE_LINK" ]]; then
        basename "$(readlink "$ACTIVE_LINK")"
    else
        echo "none"
    fi
}

# Helper to update registry
update_registry() {
    local project="$1"
    local timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    
    # Create registry if it doesn't exist
    [[ -f "$REGISTRY" ]] || echo "{}" > "$REGISTRY"
    
    # Update last_active time (would use jq in production)
    # For now, just touch a marker file
    touch "$CONTEXTS_DIR/$project/.last_active"
}

case "${1:-status}" in
    switch|s)
        if [[ -z "$2" ]]; then
            echo "Usage: pc switch <project-name>"
            exit 1
        fi
        
        PROJECT="$2"
        PROJECT_DIR="$CONTEXTS_DIR/$PROJECT"
        
        # Create project directory if new
        if [[ ! -d "$PROJECT_DIR" ]]; then
            echo "Creating new project context: $PROJECT"
            mkdir -p "$PROJECT_DIR"
            echo "# Project: $PROJECT" > "$PROJECT_DIR/HANDOFF.md"
            echo "Created: $(date)" >> "$PROJECT_DIR/HANDOFF.md"
        fi
        
        # Update active symlink
        ln -sfn "$PROJECT" "$ACTIVE_LINK"
        update_registry "$PROJECT"
        
        echo "📁 Switched to: $PROJECT"
        echo "   Context dir: $PROJECT_DIR"
        ;;
        
    new|n)
        if [[ -z "$2" ]]; then
            echo "Usage: pc new <project-name>"
            exit 1
        fi
        
        PROJECT="$2"
        PROJECT_DIR="$CONTEXTS_DIR/$PROJECT"
        
        if [[ -d "$PROJECT_DIR" ]]; then
            echo "Project already exists: $PROJECT"
            echo "Use 'pc switch $PROJECT' to activate it"
            exit 1
        fi
        
        mkdir -p "$PROJECT_DIR"
        echo "# Project: $PROJECT" > "$PROJECT_DIR/HANDOFF.md"
        echo "Created: $(date)" >> "$PROJECT_DIR/HANDOFF.md"
        ln -sfn "$PROJECT" "$ACTIVE_LINK"
        
        echo "✨ Created and switched to: $PROJECT"
        ;;
        
    list|ls|l)
        CURRENT=$(current_project)
        echo "Project Contexts:"
        echo "================="
        
        for dir in "$CONTEXTS_DIR"/*/; do
            [[ -d "$dir" ]] || continue
            [[ "$dir" == *"/active/" ]] && continue
            
            project=$(basename "$dir")
            if [[ "$project" == "$CURRENT" ]]; then
                echo "▸ $project (active)"
            else
                echo "  $project"
            fi
        done
        ;;
        
    status|"")
        CURRENT=$(current_project)
        if [[ "$CURRENT" == "none" ]]; then
            echo "No active project context"
            echo "Use 'pc switch <project>' to activate one"
        else
            echo "Active project: $CURRENT"
            echo "Context dir: $CONTEXTS_DIR/$CURRENT"
            
            # Show recent files
            if [[ -d "$CONTEXTS_DIR/$CURRENT" ]]; then
                echo ""
                echo "Context files:"
                ls -la "$CONTEXTS_DIR/$CURRENT" | grep -E "(HANDOFF|WISDOM|LOG|todo)" | awk '{print "  " $9}'
            fi
        fi
        ;;
        
    dir|d)
        # Output the active context directory (useful for scripts)
        if [[ -L "$ACTIVE_LINK" ]]; then
            echo "$CONTEXTS_DIR/$(basename "$(readlink "$ACTIVE_LINK")")"
        else
            echo ""
            exit 1
        fi
        ;;
        
    help|h)
        cat << EOF
Project Context Manager

Usage: pc [command] [args]

Commands:
  pc                    Show current project status
  pc switch <name>      Switch to project (creates if new)
  pc new <name>         Create new project context
  pc list               List all project contexts
  pc dir                Print active context directory
  pc help               Show this help message

Shortcuts:
  pc s <name>          Same as 'pc switch'
  pc ls                Same as 'pc list'

Examples:
  pc switch zwift      Switch to zwift project
  pc new experiment    Create new experimental context
  pc                   Show what project is active

Context Directory: $CONTEXTS_DIR
EOF
        ;;
        
    *)
        echo "Unknown command: $1"
        echo "Use 'pc help' for usage"
        exit 1
        ;;
esac