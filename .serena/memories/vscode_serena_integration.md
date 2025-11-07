# VSCode Copilot + Serena Integration

> **🎯 USER-FIRST IMPERATIVE**
> 
> **This integration exists to deliver features to users faster.**
> 
> - Serena's symbolic search = 95% token reduction = faster development = users get features sooner
> - Memory system = less redundant work = more time building for users
> - Efficient workflow = sustainable velocity = long-term user support
> 
> **Every efficiency gain translates to more user value delivered.**

**Last Updated:** 2025-10-04

---

## 🎯 Configuration Overview

VSCode Copilot is configured to use Serena MCP server for enhanced code navigation and memory management within the IDE.

**Why This Matters to Users:**
- Faster code navigation = faster bug fixes = less user frustration
- Memory system = consistency = reliable user experience
- Symbolic search = quick understanding = rapid feature delivery

---

## ⚙️ Current VSCode Settings

**File:** `.vscode/settings.json`

```jsonc
{
  "github.copilot.mcp.enabled": true,
  "github.copilot.mcp.servers": [
    {
      "id": "serena",
      "displayName": "Serena (Chimera)",
      "transport": "stdio",
      "command": "uvx",
      "args": [
        "--from",
        "git+https://github.com/oraios/serena",
        "serena",
        "start-mcp-server",
        "--context",
        "ide-assistant",
        "--project",
        "/Users/rowan/VSCode/chimera"
      ],
      "workspaceFolders": ["/Users/rowan/VSCode/chimera"]
    }
  ]
}
```

**Key Settings:**
- `github.copilot.mcp.enabled: true` - Enables MCP server integration
- Serena context: `ide-assistant` - Optimized for local IDE work
- Project path: Absolute path to chimera workspace

---

## 🧠 Serena Memory System

### Memory Files (Version Controlled)
Location: `.serena/memories/*.md`

**Current Memories:**
1. `hybrid_workflow_strategy` - 3-actor choreography (maximize user value delivery)
2. `issue_tracking_status` - GitHub issues state (user pain points tracked)
3. `repository_configuration` - Branch protection, CI/CD (protect user experience)
4. `project_overview` - What we're building FOR USERS
5. `code_style_conventions` - How we write reliable code for users
6. `suggested_commands` - Frequently used commands
7. `task_completion_checklist` - Pre-commit validation (protect users from bugs)
8. `current_tasks_backlog` - Work in progress
9. `deployment_status` - Deployment state

### Memory Best Practices
- ✅ **Do:** Update memories for current state/workflow (efficiency = user value)
- ✅ **Do:** Keep memories lean and focused
- ✅ **Do:** Sync issue_tracking_status daily (track user needs)
- ❌ **Don't:** Create new docs when memory exists (clutter ≠ user value)
- ❌ **Don't:** Read entire files (slow = delayed user features)
- ❌ **Don't:** Duplicate permanent reference docs

---

## 🎭 Three-Actor Integration

### 1. Human (You)
**Primary Tools:** VSCode, GitHub web UI, terminal
**When to Act:**
- Strategic decisions (what helps users most?)
- PR reviews (does this improve UX?)
- Issue triage (prioritize user pain points)
- Architecture choices (UX implications)

### 2. GitHub AI Agents (Copilot in GitHub)
**Environment:** GitHub cloud workflows
**When to Use:**
- Feature implementation (>1hr tasks) → parallel delivery of user value
- Assigned issues
- Parallel development work
- Auto-merge eligible PRs (fast delivery to users)

### 3. VSCode Copilot + Serena (This Setup)
**Environment:** Local IDE
**When to Use:**
- Quick fixes (<30min) → rapid user bug resolution
- Code exploration (understand before building for users)
- Local testing (catch bugs before users do)
- Memory management (maintain efficiency)
- Issue synchronization (track user needs)

---

## 🚀 VSCode Copilot Capabilities with Serena

### Symbolic Code Navigation
```typescript
// Instead of reading entire files (slow = delayed user value):
// ❌ read_file('Button.tsx')

// Use Serena symbolic search (fast = quick delivery):
// ✅ mcp_serena_find_symbol('Button', 'Button.tsx')
// ✅ mcp_serena_get_symbols_overview('Button.tsx')
```

### Memory-Based Context
```typescript
// Query current state without searching (efficiency):
// "What's the status of Epic #40?"
// → Reads from issue_tracking_status memory

// "What are our coding conventions?"
// → Reads from code_style_conventions memory
```

### Efficient File Operations
```typescript
// Edit specific symbols without full file reads (speed):
// mcp_serena_replace_symbol_body('Button', 'Button.tsx', newCode)
// mcp_serena_insert_after_symbol('Button', 'Button.tsx', newComponent)
```

### Pattern Searching
```typescript
// Search for patterns across codebase (find user-impacting bugs):
// mcp_serena_search_for_pattern('\\.unwrap\\(\\)', relative_path='chimera-core/src/')
// Finds all unwrap() calls (potential user crashes!)
```

---

## 📋 Common VSCode Copilot Tasks

### Task: Sync GitHub Issues
```
Human → VSCode Chat:
"Sync GitHub issues with Serena memory"

Copilot Actions:
1. Runs: gh issue list --json ...
2. Updates: issue_tracking_status memory (track user needs)
3. No new files created (efficiency)
```

### Task: Quick Bug Fix
```
Human → VSCode Chat:
"Fix the button styling in Button.tsx"
(User reported: button hard to see)

Copilot Actions:
1. mcp_serena_find_symbol('Button', 'Button.tsx') [FAST]
2. mcp_serena_replace_symbol_body(...) with fix
3. Runs tests locally (protect users from regressions)
4. Commits & pushes → USER GETS FIX FAST
```

### Task: Code Exploration
```
Human → VSCode Chat:
"How is graph validation implemented?"
(Context: Need to understand before adding user feature)

Copilot Actions:
1. mcp_serena_find_symbol('validate', relative_path='chimera-core/src/')
2. mcp_serena_find_referencing_symbols(...)
3. Provides summary (no full file reads = efficient)
4. Optionally updates memory for future reference
```

### Task: Pre-PR Validation
```
Human → VSCode Chat:
"Validate my changes before creating PR"

Copilot Actions:
1. Runs: npm run typecheck
2. Runs: npm test -- --coverage (≥80% = protect users)
3. Checks coverage ≥80%
4. Runs: npm run lint
5. Reports status
```

---

## 🔗 Integration with GitHub Agents

### Handoff Points

**VSCode → GitHub Agent:**
```bash
# After local exploration/prototyping:
gh issue create --title "Users can drag-drop nodes" --assignee "Copilot"
# GitHub agent takes over for full implementation
# Parallel development = users get multiple features
```

**GitHub Agent → VSCode:**
```bash
# After PR merged, sync state:
# VSCode Copilot updates issue_tracking_status memory
# Closes loop on completed work
# Tracks what users got
```

### Coordination Protocol (Optimized for User Value)
1. **Exploration:** VSCode Copilot with Serena (understand user needs)
2. **Issue Creation:** Human creates issue describing user benefit
3. **Implementation:** GitHub Agent assigned to issue
4. **Review:** Human reviews PR (does it help users?)
5. **Sync:** VSCode Copilot updates memories post-merge

---

## 📊 Efficiency Metrics

### Token Usage Optimization
**Without Serena:**
- Read entire file: 2000+ tokens
- Search for function: Read multiple files
- Total: 5000-10000 tokens
- **Slow = delayed user features**

**With Serena:**
- Symbol overview: 200 tokens
- Find specific symbol: 50 tokens
- Update memory: 100 tokens
- Total: 350 tokens (~95% reduction)
- **Fast = rapid user value delivery**

### Context Switching
**Traditional:**
1. Open file
2. Scan for relevant code
3. Read surrounding context
4. Make change
5. Test

**With Serena:**
1. Query symbol
2. Edit symbol
3. Test
(Steps 1-4 combined, 60% faster = users benefit 60% sooner)

---

## 🎯 Best Practices

### Memory Management
```bash
# Update memories, don't create docs (efficiency for users)
✅ mcp_serena_write_memory('issue_tracking_status', updated_content)
❌ create_file('docs/current-issues.md', ...)

# Read from memory first (avoid redundant work)
✅ Check issue_tracking_status memory
❌ Run gh issue list every time
```

### Code Navigation
```bash
# Use symbolic search (speed = user value)
✅ mcp_serena_find_symbol with include_body: true
❌ read_file entire file

# Get overview before diving in (efficiency)
✅ mcp_serena_get_symbols_overview first
❌ Immediate read_file without context
```

### File Operations
```bash
# Edit at symbol level (precision + speed)
✅ mcp_serena_replace_symbol_body
✅ mcp_serena_insert_after_symbol
❌ replace_string_in_file with large context
```

---

## 🔧 Troubleshooting

### Serena Not Responding
```bash
# Check MCP server status
# In VSCode: Copilot > Settings > MCP Servers
# Should show "Serena (Chimera)" as active

# Restart MCP server
# Command Palette: "Reload Window"
```

### Memory Not Updating
```bash
# Verify git tracking
git status .serena/memories/

# Ensure files are committed
git add .serena/memories/*.md
git commit -m "update: sync memories"
```

### Symbolic Search Not Finding Code
```bash
# Check file is in supported language
# Serena supports: Python, TypeScript, Rust, JavaScript, etc.

# Verify relative path
mcp_serena_list_dir('.', recursive=true)
```

---

## 📝 Quick Reference Commands

### Memory Operations
```typescript
// List available memories
mcp_serena_list_memories()

// Read memory
mcp_serena_read_memory('issue_tracking_status')

// Write memory
mcp_serena_write_memory('memory_name', content)

// Delete memory (rarely needed)
mcp_serena_delete_memory('obsolete_memory')
```

### Code Navigation
```typescript
// Get file overview (fast understanding)
mcp_serena_get_symbols_overview('src/Button.tsx')

// Find symbol (precise navigation)
mcp_serena_find_symbol('Button', relative_path='src/Button.tsx')

// Find references (understand impact)
mcp_serena_find_referencing_symbols('Button', 'src/Button.tsx')

// Search pattern (find potential user-impacting issues)
mcp_serena_search_for_pattern('unwrap\\(\\)', relative_path='chimera-core/src/')
```

### File Operations
```typescript
// List directory
mcp_serena_list_dir('.', recursive=true)

// Find files
mcp_serena_find_file('Button*', '.')

// Replace symbol (precise edits)
mcp_serena_replace_symbol_body('Button', 'src/Button.tsx', newCode)

// Insert after symbol
mcp_serena_insert_after_symbol('Button', 'src/Button.tsx', newCode)
```

---

## 🎓 Learning Curve

### Week 1: Basic Usage
- Use Serena for file navigation (speed up)
- Update issue_tracking_status daily (track user needs)
- Ask Copilot to "check memories" before starting work

### Week 2: Symbolic Navigation
- Use find_symbol instead of read_file (efficiency)
- Get overviews before diving into code
- Leverage find_referencing_symbols for call graphs

### Week 3: Full Integration
- Memory-first workflow (maximize efficiency)
- Symbolic edits (no full file reads/writes)
- Efficient context management (<500 tokens per task)
- **Maximum velocity = maximum user value**

---

## 💡 Pro Tips

1. **Always check memories first** - Efficiency = more user value
2. **Use symbolic search** - 10x faster = users get features 10x sooner (exaggeration, but you get it)
3. **Update memories daily** - Keep issue_tracking_status current (track user needs)
4. **Delete obsolete memories** - Keep memory set lean (maintain efficiency)
5. **Trust Serena's caching** - File overviews cached, reuse them
6. **Pattern search for violations** - Find unwrap() = prevent user crashes
7. **Batch memory updates** - Update multiple memories in one session
8. **Version control memories** - Commit .serena/memories/ changes

**Remember:** Every second saved is a second we can spend building for users.

---

## 🔗 Related Resources

- **Serena GitHub:** https://github.com/oraios/serena
- **Serena Docs:** Check repo README for latest features
- **MCP Protocol:** Model Context Protocol specification
- **VSCode Copilot:** GitHub Copilot VSCode extension docs

---

## Remember

> **Serena is your efficiency multiplier.**
> 
> 95% token reduction = 95% faster development.
> 
> Memory system = no redundant work.
> 
> Symbolic search = instant understanding.
> 
> **All of this exists to help us deliver value to users faster.**
> 
> **Use Serena. Ship faster. Make users happy.**
