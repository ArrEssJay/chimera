# MCP + Hybrid Agentic Workflow Strategy

> **🎯 USER-FIRST IMPERATIVE**
> 
> **This workflow exists to deliver value to USERS faster.**
> 
> Every efficiency gain, every automation, every process improvement has ONE purpose:
> **Help users learn signal processing and build communication systems better/faster.**
> 
> If a workflow step doesn't accelerate user value delivery, question it.
> 
> **Velocity without user impact is waste.**

---

## 🎭 Three-Actor System

### 1. **Human (You)** - Strategic Director
**Tools:** GitHub web UI, VSCode, terminal
**Responsibilities:**
- Architectural decisions that affect **user experience**
- Issue prioritization (what helps users most?)
- PR review & approval
- Merge conflict resolution
- Strategic planning (roadmap with user benefits)

**When to Act:**
- Creating/closing issues (user value focus)
- Reviewing completed PRs (does it help users?)
- Making architecture decisions (UX implications)
- Handling conflicts between agents
- End-of-day planning (tomorrow's user value)

**Remember:** You are the voice of the user in technical decisions.

---

### 2. **GitHub AI Agents (Copilot)** - Feature Implementors
**Environment:** GitHub cloud workflows
**Responsibilities:**
- Implement features **that users will experience**
- Create PRs with full implementation
- Follow contract-first development
- Maintain ≥80% test coverage (**so users don't hit bugs**)
- Auto-merge when CI passes (**so users get features faster**)

**Assignment Pattern:**
```bash
# Assign issue to agent
gh issue edit <number> --add-assignee "Copilot"

# Agent creates PR automatically
# PR follows template with acceptance criteria
# Auto-merges if all checks pass → USERS GET FEATURE
```

**Constraints:**
- Only modify assigned files (contract-enforced)
- No panic/unwrap in Rust core (**protects user experience**)
- Must pass all CI checks (**quality for users**)
- Test coverage gates (**reliability for users**)

---

### 3. **VSCode Copilot + Serena MCP** - Local Developer & Memory System
**Environment:** Local VSCode with MCP server
**Responsibilities:**
- Real-time coding assistance
- Issue synchronization with Serena memory
- Quick fixes & exploratory work
- Local testing before pushing (**catch bugs before users see them**)
- Memory management for project state

**Tools Available:**
- Serena symbolic code search
- Memory read/write for project state
- Local terminal access
- File editing & refactoring
- Test execution

**When to Use:**
- Exploring codebase (understand before building for users)
- Quick bug fixes < 30 min (**fix user-impacting issues fast**)
- Local testing & validation
- Updating documentation (**help users understand**)
- Managing Serena memories
- Triaging new issues (user pain points)

---

## 🔄 Workflow Choreography

### Daily Workflow

#### Morning Session (15 min)
```bash
# 1. Sync issues with Serena (VSCode + Copilot)
gh issue list --state open --json number,title,labels,assignees

# 2. Review overnight PR completions (Human)
# Question: What user value did these deliver?
gh pr list --state merged --search "merged:>=$(date -v-1d +%Y-%m-%d)"

# 3. Triage new issues (Human + VSCode Copilot)
# Priority: User pain points > nice-to-haves
gh issue list --state open --label "needs-triage"

# 4. Update Serena memory (VSCode Copilot)
# Uses mcp_serena_write_memory to sync issue status
```

#### Active Development (throughout day)

**Pattern A: Feature Work (GitHub Agent)**
```bash
# Human creates/assigns issue
# Title should describe USER BENEFIT
gh issue create --title "[Phase 2] Users can drag-drop nodes" --body "..."
gh issue edit <number> --add-assignee "Copilot"

# Agent automatically:
# - Creates branch
# - Implements feature FOR USERS
# - Writes tests TO PROTECT USER EXPERIENCE
# - Creates PR
# - Auto-merges if passes → USERS GET FEATURE FAST

# Human only intervenes if CI fails or conflicts arise
```

**Pattern B: Quick Fix (VSCode Copilot + Serena)**
```bash
# In VSCode chat:
# "Fix the button styling in Button.tsx"
# (User-reported: button hard to see)

# Copilot:
# 1. Uses mcp_serena_find_symbol to locate code
# 2. Makes targeted edit with mcp_serena_replace_symbol_body
# 3. Validates with local tests
# 4. Commits & pushes → USERS GET FIX FAST

# No issue creation needed for minor fixes
```

**Pattern C: Exploratory Work (VSCode Copilot + Serena)**
```bash
# In VSCode chat:
# "How is the node graph validation implemented?"
# (Context: Need to understand before adding user feature)

# Copilot:
# 1. mcp_serena_find_symbol for entry points
# 2. mcp_serena_find_referencing_symbols for call graph
# 3. Provides summary without reading full files
# 4. Optionally updates memory for future reference
```

#### Evening Review (10 min)
```bash
# 1. Check day's progress (Human)
# Question: What did users gain today?
gh pr list --state all --search "created:>=$(date +%Y-%m-%d)"

# 2. Update Serena issue memory (VSCode Copilot)
# Updates issue_tracking_status with latest state

# 3. Plan tomorrow's priorities (Human)
# Priority: Maximum user value per hour of work
# Create/assign new issues for next day
```

---

## 🎯 Decision Matrix: Who Does What?

| Task Type | Duration | Complexity | Owner | Tools | **User Impact** |
|-----------|----------|------------|-------|-------|-----------------|
| Feature implementation | >1 hour | Medium-High | GitHub Agent | Issue → PR workflow | **Direct user value** |
| Bug fix (known location) | <30 min | Low | VSCode Copilot | Serena edit tools | **Fix user pain fast** |
| Architecture change | Any | High | Human + VSCode | Discussion + planning | **Major UX decision** |
| Exploratory analysis | <1 hour | Medium | VSCode Copilot | Serena search tools | **Inform user features** |
| Test writing | <1 hour | Medium | Either | Depends on context | **Protect user experience** |
| Documentation | <30 min | Low | VSCode Copilot | Local editing | **Help users understand** |
| PR review | <15 min | N/A | Human | GitHub web UI | **Quality gate for users** |
| Issue triage | <5 min | Low | Human or VSCode | GitHub CLI or UI | **Prioritize user needs** |
| Memory sync | <5 min | Low | VSCode Copilot | Serena memory tools | **Efficiency → faster delivery** |

---

## 🧠 Serena Memory Management

### Memory Files Purpose

1. **`project_overview`** - What we're building FOR USERS + tech stack
2. **`issue_tracking_status`** - Current work (sync daily) - user value being delivered
3. **`code_style_conventions`** - How we write maintainable code for users
4. **`hybrid_workflow_strategy`** - This file (workflow for efficiency → user value)
5. **`suggested_commands`** - Frequently used commands
6. **`task_completion_checklist`** - Pre-commit validation (protect users from bugs)

### Update Frequency
- **Daily:** issue_tracking_status (what user value is flowing?)
- **Weekly:** project_overview (if architecture changes affect users)
- **As-needed:** code_style_conventions (when new patterns emerge)
- **Rarely:** hybrid_workflow_strategy (workflow refinement)

### Memory vs Documentation
**Use Serena Memory For:**
- Current state (issues, priorities)
- Project-specific patterns
- Frequently accessed info
- Quick reference data

**Use Docs Files For:**
- Permanent reference material
- User-facing guides (**help users learn!**)
- Protocol specifications
- Architecture decisions

---

## 🚀 Optimization Strategies

### Parallel Development
```
Phase 1 ─────┐
             ├─→ Integration Point → USERS GET ALL 3 FEATURES TOGETHER
Phase 2 ─────┤
             ├─→ Merge
Phase 3 ─────┘
```

**Enable via:**
- Contract-first development (locked interfaces)
- File ownership (no conflicts = faster delivery)
- Independent test suites (confidence = faster merge = faster user value)

### Context Switching Minimization
- **Morning:** Planning & triage (identify user priorities)
- **Midday:** Deep work (agents run, human codes locally)
- **Evening:** Review & planning (what did users gain? what's next?)

### Serena Efficiency
```typescript
// ❌ DON'T: Read entire file (slow = delayed user value)
read_file('Button.tsx')

// ✅ DO: Use symbolic search (fast = quick delivery)
mcp_serena_find_symbol('Button', 'Button.tsx', { include_body: true, depth: 1 })

// ❌ DON'T: Create new doc files (clutter doesn't help users)
create_file('workflow-update.md', ...)

// ✅ DO: Update existing memories (organized = efficient = faster user value)
mcp_serena_write_memory('hybrid_workflow_strategy', updated_content)
```

**Why efficiency matters:** Every hour saved = another feature users get faster.

---

## 📊 Success Metrics

### Velocity Indicators (Means to User Value)
- **Issues closed per day:** Target 3-5 (more features for users)
- **PR merge time:** Target <2 hours (fast delivery to users)
- **Human review time:** Target <15 min per PR (bottleneck minimization)
- **Memory sync frequency:** Daily minimum (stay organized = stay fast)

### Quality Gates (Protect User Experience)
- **Test coverage:** ≥80% enforced (users don't hit bugs)
- **CI pass rate:** >95% target (quality = user trust)
- **Contract violations:** 0 (enforced) (stability = user confidence)
- **Merge conflicts:** <1 per week (friction = delayed user value)

### Context Efficiency (Speed = More User Value)
- **Serena memory reads:** Prefer over full file reads
- **Symbolic searches:** Use before reading code bodies
- **Memory updates:** Batch when possible
- **Documentation generation:** Avoid unless truly helpful to users

### **MOST IMPORTANT: User Value Delivered**
- **Features shipped:** Count weekly
- **Bugs fixed:** Count daily (user pain points resolved)
- **User experience improvements:** Qualitative assessment
- **Learning effectiveness:** Does the tool teach well?

---

## 🎓 Learning & Adaptation

### Weekly Retrospective
1. Review velocity metrics
2. **Ask: What user value did we deliver this week?**
3. Identify bottlenecks (what slowed user value delivery?)
4. Update workflow strategy if needed
5. Refine memory structure

### Continuous Improvement
- Track which tasks are fastest with each actor
- Adjust decision matrix based on outcomes
- Evolve Serena memory schema as project grows
- Document new patterns in code_style_conventions
- **Always ask: Is this change helping users more?**

---

## 🔧 Practical Commands

### Issue Management
```bash
# Create and assign in one go
# Title describes USER BENEFIT
gh issue create --title "Users can hear modulated audio" --assignee "Copilot" --label "phase-2-core,high"

# Bulk status check
gh issue list --json number,title,state,assignees --jq '.[] | select(.state=="OPEN")'

# Check agent progress (user value in pipeline)
gh pr list --assignee "Copilot" --state open
```

### Memory Sync (in VSCode)
```typescript
// Daily sync command to VSCode Copilot chat:
"Sync GitHub issues with Serena memory issue_tracking_status"

// Result: Automated update without file creation
```

### Local Development Flow
```bash
# 1. Create branch
git checkout -b fix/quick-issue

# 2. Make changes with Serena assistance (in VSCode)
# Use Copilot chat with Serena tools

# 3. Test locally (protect users from bugs)
npm test -- --coverage

# 4. Commit & push
git commit -am "fix: description of user benefit"
git push origin fix/quick-issue

# 5. Create PR (if needed)
gh pr create --fill
```

---

## 💡 Pro Tips

1. **Use VSCode for exploration, GitHub agents for implementation**
2. **Update Serena memories, don't create new docs**
3. **Let agents handle >1hr tasks, you handle <30min fixes** (user value velocity)
4. **Review PRs as they complete, don't batch** (faster user value delivery)
5. **Trust the automation, intervene only on failures**
6. **Keep Serena memory lean - delete obsolete entries** (efficiency)
7. **Use symbolic search before reading full files** (speed)
8. **Batch similar tasks (all triage, all reviews, etc.)**
9. **Every feature should answer: "How does this help users?"**
10. **Bug fixes are user pain point removals - prioritize them**

---

## 🎯 TL;DR

**You (Human):** Architect, reviewer, strategist, **voice of the user**
**GitHub Agents:** Feature factory with auto-merge **delivering user value**
**VSCode + Serena:** Real-time assistant with memory **for efficient user value delivery**

**Key Rule:** Memories over docs, symbolic search over file reads, agents over manual coding.

**GOLDEN RULE:** Every action should accelerate user value delivery. If it doesn't, question it.

---

## 🚀 User Value Delivery Chain

```
Human Strategic Decision
         ↓
   Issue Created (User Benefit Defined)
         ↓
   Agent Assigned (Automatic)
         ↓
   Implementation (Parallel)
         ↓
   Tests (Protect User Experience)
         ↓
   CI Validation (Quality Gate)
         ↓
   Auto-Merge (Fast Delivery)
         ↓
   🎉 USERS GET FEATURE 🎉
```

**Optimize every step for speed and quality → More user value, faster.**
