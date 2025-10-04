# Custom Chat Modes for Agentic Workflow

**Last Updated:** 2025-10-04

## 🎯 Overview

Custom chat modes configured in `.github/copilot-chat-modes.json` to simplify the 3-actor agentic workflow. Instead of remembering @ commands, just select the appropriate mode from the chat interface.

---

## 🎭 Available Modes

### ☀️ Morning Sync
**Usage:** Start of workday (15 min)
**Purpose:** Sync issues, review overnight work, plan day

**What it does:**
1. Syncs GitHub issues → `issue_tracking_status` memory
2. Reviews merged PRs from last 24 hours
3. Triages new issues
4. Provides daily status summary

**When to use:** Every morning before starting work

---

### 🔧 Quick Fix Mode
**Usage:** Bug fixes < 30 min
**Purpose:** Fast fixes using Serena symbolic search

**What it does:**
1. Uses `mcp_serena_find_symbol` (no full file reads)
2. Edits at symbol level
3. Runs tests locally
4. Commits & pushes

**Rules:**
- ❌ No read_file (use symbolic search)
- ❌ No new docs (update memories)
- ✅ Check memories first

**When to use:** Button not working, styling issue, small bug

---

### 🔍 Explore Mode
**Usage:** Understanding code without reading full files
**Purpose:** Efficient code discovery with Serena

**What it does:**
1. Gets symbol overviews
2. Finds symbols and references
3. Searches patterns
4. Updates memories if valuable

**Efficiency:** < 500 tokens per exploration

**When to use:** "How does graph validation work?", "Where is LDPC encoding?"

---

### 🚀 GitHub Handoff
**Usage:** Creating issues for GitHub AI agents
**Purpose:** Prepare work for >1hr tasks

**What it does:**
1. Verifies scope (> 1hr)
2. Checks contracts not being modified
3. Creates issue with template
4. Assigns to Copilot
5. Updates `issue_tracking_status` memory

**When to use:** Feature implementation, new component, complex refactoring

---

### ✅ Pre-PR Validation
**Usage:** Before creating PR
**Purpose:** Validate all changes pass quality gates

**What it does:**
- **TypeScript:** typecheck, lint, test coverage ≥80%, build
- **Rust:** fmt, clippy, tests, no unwrap() in core
- **Contracts:** Ensure no contract files modified
- **Ownership:** Check only assigned files modified

**When to use:** After completing work, before `gh pr create`

---

### 🌙 Evening Review
**Usage:** End of workday (10 min)
**Purpose:** Review progress, update memories, plan tomorrow

**What it does:**
1. Checks today's PR activity
2. Updates Serena memories (issues, backlog, deployment)
3. Plans top 3 priorities for tomorrow
4. Suggests overnight agent assignments
5. Commits memory changes

**When to use:** End of day before logging off

---

### 🧠 Memory Sync
**Usage:** As needed (< 5 min)
**Purpose:** Sync Serena memories with current state

**What it does:**
1. Updates `issue_tracking_status` from GitHub
2. Updates `repository_configuration` if CI changed
3. Updates `deployment_status` if deployed
4. Updates `current_tasks_backlog` for WIP
5. Commits memory changes

**When to use:** After major changes, before/after GitHub agent work

---

### 🔒 Contract Enforcement
**Usage:** Before committing
**Purpose:** Verify no contract violations

**What it does:**
1. Checks for contract file modifications (forbidden)
2. Searches for `.unwrap()` / `.expect()` in Rust core (forbidden)
3. Validates file ownership (only modify assigned files)
4. Checks imports from `contracts/`

**When to use:** Before every commit, especially for agent work

---

## 🎯 Mode Selection Guide

| Your Goal | Mode to Use | Duration |
|-----------|-------------|----------|
| Start workday | ☀️ Morning Sync | 15 min |
| Small bug fix | 🔧 Quick Fix | 3-30 min |
| Understand code | 🔍 Explore | 5-15 min |
| Create feature issue | 🚀 GitHub Handoff | 5 min |
| Ready to PR | ✅ Pre-PR Validation | 2 min |
| End workday | 🌙 Evening Review | 10 min |
| Keep memories current | 🧠 Memory Sync | 5 min |
| Check rules | 🔒 Contract Enforcement | 1 min |

---

## 🔄 Typical Daily Flow

```
08:00 - ☀️ Morning Sync
        Review what agents did overnight
        Plan today's work
        
09:00 - 🔍 Explore Mode (if needed)
        Understand unfamiliar code
        
09:30 - 🔧 Quick Fix Mode
        Handle 2-3 small bugs
        
10:30 - 🚀 GitHub Handoff
        Create issues for bigger features
        Assign to GitHub agents
        
11:00 - 🔧 Quick Fix Mode
        More quick fixes
        
12:00 - Lunch / Let agents work
        
14:00 - Review agent PRs (manual)
        
15:00 - 🔧 Quick Fix / 🔍 Explore
        Continue local work
        
17:00 - ✅ Pre-PR Validation
        Validate your changes
        
17:30 - 🌙 Evening Review
        Update memories
        Plan tomorrow
        Assign overnight agent work
```

---

## 💡 Pro Tips

### Mode Switching
- Don't stay in one mode all day
- Each mode optimized for specific task duration
- Switch modes = switch mental context

### Morning Sync is Critical
- Sets up your whole day
- Keeps memories current
- Prevents redundant GitHub API calls

### Quick Fix vs GitHub Handoff Decision
```
< 30 min → 🔧 Quick Fix Mode
> 1 hour → 🚀 GitHub Handoff
```

### Always Validate Before PR
- Use ✅ Pre-PR Validation mode
- Catches issues before CI
- Saves agent review cycles

### Evening Review Compounds
- Daily memory updates prevent drift
- Planning tomorrow saves morning time
- Overnight agent work = morning wins

---

## 🎨 Mode Customization

To modify modes, edit `.github/copilot-chat-modes.json`:

```json
{
  "id": "your-mode",
  "label": "🎯 Your Mode",
  "description": "Brief description",
  "instructions": "Detailed instructions...",
  "tools": ["serena", "github", "terminal", "workspace"]
}
```

**Available tools:**
- `serena` - Serena MCP server
- `github` - GitHub operations
- `terminal` - Shell commands
- `workspace` - File operations

---

## 🚨 Common Mistakes

### ❌ Wrong Mode Selection
```
❌ Using 🔧 Quick Fix for 2hr feature
   → Should use 🚀 GitHub Handoff

❌ Using 🔍 Explore without checking memories first
   → Check memory, might already be documented

❌ Skipping ☀️ Morning Sync
   → Wastes time on duplicate GitHub API calls
```

### ❌ Not Following Mode Instructions
```
❌ Quick Fix mode reading entire files
   → Should use symbolic search only

❌ Explore mode reading full files
   → Should use overviews + symbols

❌ Pre-PR mode skipping contract check
   → CI will fail, waste time
```

---

## 📊 Mode Effectiveness Metrics

Track these to optimize workflow:

**☀️ Morning Sync:**
- Target: < 15 min
- Success: Daily status clear, priorities set

**🔧 Quick Fix:**
- Target: 3-5 min per fix
- Success: ≥ 3 fixes per session

**🔍 Explore:**
- Target: < 500 tokens
- Success: Understanding gained without full file reads

**🚀 GitHub Handoff:**
- Target: < 5 min per issue
- Success: Agent can start work immediately

**✅ Pre-PR Validation:**
- Target: < 2 min
- Success: All checks pass, no CI failures

**🌙 Evening Review:**
- Target: < 10 min
- Success: Memories updated, tomorrow planned

---

## 🔗 Integration with Serena

All modes use Serena MCP for:
- Memory operations (read/write/update)
- Symbolic code navigation
- Pattern searching
- Efficient file operations

**Token efficiency:**
- Traditional workflow: 5000-10000 tokens
- With modes + Serena: 350-500 tokens
- **~95% reduction**

---

## 🎓 Learning Curve

### Week 1: Use Morning/Evening Modes
- Get into daily rhythm
- Understand memory updates
- Practice mode switching

### Week 2: Add Quick Fix & Explore
- Replace manual file reading
- Use symbolic search
- Understand handoff protocol

### Week 3: Full Workflow
- All 8 modes in rotation
- Efficient task batching
- Autonomous agent coordination

---

## 📝 Quick Reference

### Activate a Mode
1. Open GitHub Copilot Chat
2. Click mode selector (top of chat)
3. Choose mode from list
4. Start chatting

### Mode Shortcuts (once in mode)
- Just describe what you need
- Mode handles the protocol
- No need to remember commands

### Exit Mode
- Select different mode
- Or close/reopen chat

---

**Remember:** Modes are workflows, not just prompts. Each mode follows a proven protocol optimized for specific task types. Trust the mode, follow the instructions!