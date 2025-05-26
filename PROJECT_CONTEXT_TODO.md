# Project Context Management - TODO

## High Priority - Core Functionality

- [ ] Create ~/.project-contexts/ directory structure
- [ ] Write `pc` command script with basic operations:
  - [ ] `pc` - show current project status
  - [ ] `pc switch <name>` - switch active context
  - [ ] `pc new <name>` - create new project context
  - [ ] `pc list` - list all project contexts
- [ ] Test manual context switching between projects
- [ ] Document installation steps for `pc` command

## Medium Priority - Slash Command Integration

- [ ] Update `/checkpoint` to use active context directory
- [ ] Update `/log` to use active context directory  
- [ ] Update `/plan` to use active context directory
- [ ] Update other commands that create/modify files
- [ ] Add error handling for missing context directory
- [ ] Test all slash commands with multiple contexts

## Low Priority - Enhanced Features

- [ ] Implement `pc stash` and `pc pop` for quick tangents
- [ ] Add fuzzy matching for project names
- [ ] Create registry.json for project metadata
- [ ] Add auto-detection for git repositories
- [ ] Implement project tagging system
- [ ] Add `pc info <project>` to show project details
- [ ] Create `pc archive <project>` for old projects
- [ ] Add context switching confirmation prompts

## Future Enhancements

- [ ] Integration with shell prompt (show active project)
- [ ] VS Code extension for project context awareness
- [ ] Automatic context inference from pwd
- [ ] Project templates for common setups
- [ ] Context sharing between machines
- [ ] Time tracking per project context