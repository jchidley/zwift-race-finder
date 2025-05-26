# Updated /log Command with Project Context Support

This shows how the `/log` command would be updated to use project contexts:

## Original Behavior
- Searches for *_LOG.md files in current directory
- Appends to chosen file or creates new one

## Updated Behavior
- Gets active project context from `pc dir`
- Searches for *_LOG.md files in context directory
- All logs isolated per project

## Key Changes

```diff
Review the current session and create documentation...

2. **Find or Create Documentation File**:
-   - Search for existing *_LOG.md files that match the current topic
+   - Get active context: $(pc dir)
+   - Search for existing *_LOG.md files in context directory
    - Check if main log file already has hierarchical structure
    - For structured logs: append to main *_LOG.md under "Active Session"
    - For simple logs: append directly to the file
    - For new topics: suggest a new topic-specific file
-   - Use PROJECT_LOG.md only for general/miscellaneous items
+   - Use {context}/PROJECT_LOG.md only for general items

...

6. **Present for Review**: Show the proposed documentation and target file
+   Note: Files will be created in project context directory, not pwd

7. **Session Management** (for hierarchical logs):
-   - When session gets large, archive to `sessions/TOPIC_LOG_SESSION_YYYYMMDD_NNN.md`
+   - Archive to `{context}/sessions/TOPIC_LOG_SESSION_YYYYMMDD_NNN.md`
    - Update TOPIC_LOG_RECENT.md with key points
    - Clear "Active Session" section in main log
```

## Usage Examples

```bash
# In any directory, logs go to active project context
cd /tmp
pc switch zwift              # Set context
/log                         # Creates log in ~/.project-contexts/zwift/

# Switch projects, logs are isolated  
pc switch mcp-server
/log                         # Creates log in ~/.project-contexts/mcp-server/

# Return to previous project, all logs preserved
pc switch zwift
/log                         # Appends to existing zwift logs
```

## Benefits
- **No Contamination**: Project logs never mix
- **Work Anywhere**: Don't need to cd to project directory
- **Persistent**: Logs survive even if project directory deleted
- **Discoverable**: `pc dir` shows where logs are stored