# Custom Chat Modes for Agentic Workflow

> **🎯 USER-FIRST IMPERATIVE**
> 
> **These chat modes exist to deliver value to users faster.**
> 
> Each mode is optimized to minimize friction and maximize velocity, because:
> - Faster development = users get features sooner
> - Efficient workflow = more time building for users
> - Clear protocols = consistent quality = reliable user experience
> 
> **Every mode should accelerate the path from idea to user benefit.**

**Last Updated:** 2025-10-04

---

## 🎯 Overview

Custom chat modes configured in `.github/chatmodes/` to simplify the 3-actor agentic workflow. Instead of remembering commands, just select the appropriate mode from the chat interface.

**Purpose:** Streamline workflow → Ship user value faster

---

## 🎭 Available Modes

### ☀️ Morning Sync
**Usage:** Start of workday (15 min)
**Purpose:** Sync issues, review overnight work, plan day
**User Impact:** Identify highest-value tasks, maximize daily user benefit

**What it does:**
1. Syncs GitHub issues → `issue_tracking_status` memory
2. Reviews merged PRs from last 24 hours (what users got)
3. Triages new issues (user pain points)
4. Provides daily status summary

**When to use:** Every morning before starting work

---

### 🔧 Quick Fix Mode
**Usage:** Bug fixes < 30 min
**Purpose:** Fast fixes using Serena symbolic search
**User Impact:** Rapid bug resolution = less user frustration

**What it does:**
1. Uses `mcp_serena_find_symbol` (no full file reads = faster)
2. Edits at symbol level
3. Runs tests locally (protect users from regressions)
4. Commits & pushes

**Rules:**
- ❌ No read_file (use symbolic search for speed)
- ❌ No new docs (update memories for efficiency)
- ✅ Check memories first

**When to use:** Button not working, styling issue, small bug (user is waiting!)

---

### 🔍 Explore Mode
**Usage:** Understanding code without reading full files
**Purpose:** Efficient code discovery with Serena
**User Impact:** Understand faster = build features faster = users benefit sooner

**What it does:**
1. Gets symbol overviews
2. Finds symbols and references
3. Searches patterns
4. Updates memories if valuable

**Efficiency:** < 500 tokens per exploration (speed matters)

**When to use:** "How does graph validation work?", "Where is LDPC encoding?"

---

### 🚀 GitHub Handoff
**Usage:** Creating issues for GitHub AI agents
**Purpose:** Prepare work for >1hr tasks
**User Impact:** Parallel development = users get multiple features simultaneously

**What it does:**
1. Verifies scope (> 1hr)
2. Checks contracts not being modified (stability for users)
3. Creates issue with template (clear user benefit statement)
4. Assigns to Copilot
5. Updates `issue_tracking_status` memory

**When to use:** Feature implementation, new component, complex refactoring

**Remember:** Issue title should describe user benefit, not implementation

---

### ✅ Pre-PR Validation
**Usage:** Before creating PR
**Purpose:** Validate all changes pass quality gates
**User Impact:** Catch bugs before users do

**What it does:**
- **TypeScript:** typecheck, lint, test coverage ≥80%, build
- **Rust:** fmt, clippy, tests, no unwrap() in core (no crashes for users)
- **Contracts:** Ensure no contract files modified (stability for users)
- **Ownership:** Check only assigned files modified

**When to use:** After completing work, before `gh pr create`

**Remember:** Every check protects users from our mistakes

---

### 🌙 Evening Review
**Usage:** End of workday (10 min)
**Purpose:** Review progress, update memories, plan tomorrow
**User Impact:** Plan highest-value work for tomorrow

**What it does:**
1. Checks today's PR activity (what users got today)
2. Updates Serena memories (issues, backlog, deployment)
3. Plans top 3 priorities for tomorrow (focus on user value)
4. Suggests overnight agent assignments
5. Commits memory changes

**When to use:** End of day before logging off

**Ask yourself:** What user value did we deliver today? What's next?

---

### 🧠 Memory Sync
**Usage:** As needed (< 5 min)
**Purpose:** Sync Serena memories with current state
**User Impact:** Efficient workflow = faster feature delivery

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
**User Impact:** Prevent breaking changes = stable user experience

**What it does:**
1. Checks for contract file modifications (forbidden - breaks user stability)
2. Searches for `.unwrap()` / `.expect()` in Rust core (forbidden - crashes users)
3. Validates file ownership (only modify assigned files)
4. Checks imports from `contracts/`

**When to use:** Before every commit, especially for agent work

**Remember:** Contract violations = instability = poor user experience

---

## 🎯 Mode Selection Guide

| Your Goal | Mode to Use | Duration | **User Benefit** |
|-----------|-------------|----------|------------------|
| Start workday | ☀️ Morning Sync | 15 min | Identify high-value tasks |
| Small bug fix | 🔧 Quick Fix | 3-30 min | Fast bug resolution |
| Understand code | 🔍 Explore | 5-15 min | Build features faster |
| Create feature issue | 🚀 GitHub Handoff | 5 min | Parallel feature delivery |
| Ready to PR | ✅ Pre-PR Validation | 2 min | Catch bugs before users |
| End workday | 🌙 Evening Review | 10 min | Plan tomorrow's value |
| Keep memories current | 🧠 Memory Sync | 5 min | Maintain efficiency |
| Check rules | 🔒 Contract Enforcement | 1 min | Protect user stability |

---

## 🔄 Typical Daily Flow

```
08:00 - ☀️ Morning Sync
        What user value shipped overnight?
        What's highest priority for users today?
        
09:00 - 🔍 Explore Mode (if needed)
        Understand unfamiliar code
        
09:30 - 🔧 Quick Fix Mode
        Handle 2-3 user-reported bugs
        
10:30 - 🚀 GitHub Handoff
        Create issues for bigger features
        Assign to GitHub agents (parallel user value!)
        
11:00 - 🔧 Quick Fix Mode
        More quick fixes (every bug fix helps users)
        
12:00 - Lunch / Let agents work
        
14:00 - Review agent PRs (manual)
        Ask: Does this improve user experience?
        
15:00 - 🔧 Quick Fix / 🔍 Explore
        Continue local work
        
17:00 - ✅ Pre-PR Validation
        Validate your changes (protect users)
        
17:30 - 🌙 Evening Review
        What user value did we deliver today?
        Plan tomorrow's user benefits
        Assign overnight agent work
```

---

## 💡 Pro Tips

### Mode Switching
- Don't stay in one mode all day
- Each mode optimized for specific task duration
- Switch modes = switch mental context

### Morning Sync is Critical
- Sets up your whole day around user value
- Keeps memories current (efficiency)
- Prevents redundant GitHub API calls

### Quick Fix vs GitHub Handoff Decision
```
< 30 min → 🔧 Quick Fix Mode (fast user benefit)
> 1 hour → 🚀 GitHub Handoff (parallel development)
```

### Always Validate Before PR
- Use ✅ Pre-PR Validation mode
- Catches issues before CI (faster delivery)
- Saves agent review cycles
- **Protects users from bugs**

### Evening Review Compounds
- Daily memory updates prevent drift
- Planning tomorrow saves morning time
- Overnight agent work = morning wins for users

---

## 🚨 Common Mistakes

### ❌ Wrong Mode Selection
```
❌ Using 🔧 Quick Fix for 2hr feature
   → Should use 🚀 GitHub Handoff
   → User waits longer for feature

❌ Using 🔍 Explore without checking memories first
   → Inefficient = slower delivery

❌ Skipping ☀️ Morning Sync
   → Wastes time = delayed user value
```

### ❌ Not Following Mode Instructions
```
❌ Quick Fix mode reading entire files
   → Slow = delayed bug fix = user frustration

❌ Pre-PR mode skipping contract check
   → CI fails = wasted time = delayed user value
```

---

## 📊 Mode Effectiveness Metrics

Track these to optimize workflow → maximize user value delivery:

**☀️ Morning Sync:**
- Target: < 15 min
- Success: Daily priorities clear, focus on user value

**🔧 Quick Fix:**
- Target: 3-5 min per fix
- Success: ≥ 3 fixes per session = less user pain

**🔍 Explore:**
- Target: < 500 tokens
- Success: Understanding gained efficiently

**🚀 GitHub Handoff:**
- Target: < 5 min per issue
- Success: Agent can start work immediately = faster user value

**✅ Pre-PR Validation:**
- Target: < 2 min
- Success: All checks pass, no CI failures = faster merge = faster user value

**🌙 Evening Review:**
- Target: < 10 min
- Success: Memories updated, tomorrow planned around user value

---

## 🔗 Integration with Serena

All modes use Serena MCP for:
- Memory operations (read/write/update)
- Symbolic code navigation (faster = more user value)
- Pattern searching
- Efficient file operations

**Token efficiency:**
- Traditional workflow: 5000-10000 tokens
- With modes + Serena: 350-500 tokens
- **~95% reduction = faster development = more user value**

---

## 🎓 Learning Curve

### Week 1: Use Morning/Evening Modes
- Get into daily rhythm focused on user value
- Understand memory updates
- Practice mode switching

### Week 2: Add Quick Fix & Explore
- Replace manual file reading (speed up)
- Use symbolic search
- Understand handoff protocol (parallel development)

### Week 3: Full Workflow
- All 8 modes in rotation
- Efficient task batching
- Autonomous agent coordination
- **Maximum velocity = maximum user value**

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

## Remember

> **Modes exist to serve users faster.**
> 
> Each mode eliminates friction, accelerates development, and protects quality.
> 
> The faster we ship quality features, the more users benefit.
> 
> **Use the modes. Ship value. Make users happy.**
