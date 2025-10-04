# MCP + Hybrid Agentic Workflow Strategy

## 🎭 Three-Actor System

### 1. **Human (You)** - Strategic Director
**Tools:** GitHub web UI, VSCode, terminal
**Responsibilities:**
- Architectural decisions
- Issue prioritization & triage
- PR review & approval
- Merge conflict resolution
- Strategic planning

**When to Act:**
- Creating/closing issues
- Reviewing completed PRs
- Making architecture decisions
- Handling conflicts between agents
- End-of-day planning sessions

---

### 2. **GitHub AI Agents (Copilot)** - Feature Implementors
**Environment:** GitHub cloud workflows
**Responsibilities:**
- Implement assigned issues
- Create PRs with full implementation
- Follow contract-first development
- Maintain ≥80% test coverage
- Auto-merge when CI passes

**Assignment Pattern:**
```bash
# Assign issue to agent
gh issue edit <number> --add-assignee "Copilot"

# Agent creates PR automatically
# PR follows template with acceptance criteria
# Auto-merges if all checks pass
```

**Constraints:**
- Only modify assigned files (contract-enforced)
- No panic/unwrap in Rust core
- Must pass all CI checks
- Test coverage gates

---

### 3. **VSCode Copilot + Serena MCP** - Local Developer & Memory System
**Environment:** Local VSCode with MCP server
**Responsibilities:**
- Real-time coding assistance
- Issue synchronization with Serena memory
- Quick fixes & exploratory work
- Local testing before pushing
- Memory management for project state

**Tools Available:**
- Serena symbolic code search
- Memory read/write for project state
- Local terminal access
- File editing & refactoring
- Test execution

**When to Use:**
- Exploring codebase
- Quick bug fixes (< 30 min)
- Local testing & validation
- Updating documentation
- Managing Serena memories
- Triaging new issues

---

## 🔄 Workflow Choreography

### Daily Workflow

#### Morning Session (15 min)
```bash
# 1. Sync issues with Serena (VSCode + Copilot)
gh issue list --state open --json number,title,labels,assignees

# 2. Review overnight PR completions (Human)
gh pr list --state merged --search "merged:>=$(date -v-1d +%Y-%m-%d)"

# 3. Triage new issues (Human + VSCode Copilot)
gh issue list --state open --label "needs-triage"

# 4. Update Serena memory (VSCode Copilot)
# Uses mcp_serena_write_memory to sync issue status
```

#### Active Development (throughout day)

**Pattern A: Feature Work (GitHub Agent)**
```bash
# Human creates/assigns issue
gh issue create --title "[Phase 2] Feature X" --body "..."
gh issue edit <number> --add-assignee "Copilot"

# Agent automatically:
# - Creates branch
# - Implements feature
# - Writes tests
# - Creates PR
# - Auto-merges if passes

# Human only intervenes if CI fails or conflicts arise
```

**Pattern B: Quick Fix (VSCode Copilot + Serena)**
```bash
# In VSCode chat:
# "Fix the button styling in Button.tsx"

# Copilot:
# 1. Uses mcp_serena_find_symbol to locate code
# 2. Makes targeted edit with mcp_serena_replace_symbol_body
# 3. Validates with local tests
# 4. Commits & pushes

# No issue creation needed for minor fixes
```

**Pattern C: Exploratory Work (VSCode Copilot + Serena)**
```bash
# In VSCode chat:
# "How is the node graph validation implemented?"

# Copilot:
# 1. mcp_serena_find_symbol for entry points
# 2. mcp_serena_find_referencing_symbols for call graph
# 3. Provides summary without reading full files
# 4. Optionally updates memory for future reference
```

#### Evening Review (10 min)
```bash
# 1. Check day's progress (Human)
gh pr list --state all --search "created:>=$(date +%Y-%m-%d)"

# 2. Update Serena issue memory (VSCode Copilot)
# Updates issue_tracking_status with latest state

# 3. Plan tomorrow's priorities (Human)
# Create/assign new issues for next day
```

---

## 🎯 Decision Matrix: Who Does What?

| Task Type | Duration | Complexity | Owner | Tools |
|-----------|----------|------------|-------|-------|
| Feature implementation | >1 hour | Medium-High | GitHub Agent | Issue → PR workflow |
| Bug fix (known location) | <30 min | Low | VSCode Copilot | Serena edit tools |
| Architecture change | Any | High | Human + VSCode | Discussion + planning |
| Exploratory analysis | <1 hour | Medium | VSCode Copilot | Serena search tools |
| Test writing | <1 hour | Medium | Either | Depends on context |
| Documentation | <30 min | Low | VSCode Copilot | Local editing |
| PR review | <15 min | N/A | Human | GitHub web UI |
| Issue triage | <5 min | Low | Human or VSCode | GitHub CLI or UI |
| Memory sync | <5 min | Low | VSCode Copilot | Serena memory tools |

---

## 🧠 Serena Memory Management

### Memory Files Purpose

1. **`project_overview`** - High-level architecture, tech stack, goals
2. **`issue_tracking_status`** - Current GitHub issue state (sync daily)
3. **`code_style_conventions`** - Patterns, anti-patterns, style rules
4. **`hybrid_workflow_strategy`** - This file (workflow choreography)
5. **`suggested_commands`** - Frequently used commands
6. **`task_completion_checklist`** - Pre-commit/PR validation steps

### Update Frequency
- **Daily:** issue_tracking_status
- **Weekly:** project_overview (if architecture changes)
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
- Onboarding guides
- Protocol specifications
- Architecture decisions

---

## 🚀 Optimization Strategies

### Parallel Development
```
Phase 1 ─────┐
             ├─→ Integration Point
Phase 2 ─────┤
             ├─→ Merge
Phase 3 ─────┘
```

**Enable via:**
- Contract-first development (locked interfaces)
- File ownership (no conflicts)
- Independent test suites

### Context Switching Minimization
- **Morning:** Planning & triage (Human)
- **Midday:** Deep work (Agents run, Human codes locally)
- **Evening:** Review & planning (Human)

### Serena Efficiency
```typescript
// ❌ DON'T: Read entire file
read_file('Button.tsx')

// ✅ DO: Use symbolic search
mcp_serena_find_symbol('Button', 'Button.tsx', { include_body: true, depth: 1 })

// ❌ DON'T: Create new doc files
create_file('workflow-update.md', ...)

// ✅ DO: Update existing memories
mcp_serena_write_memory('hybrid_workflow_strategy', updated_content)
```

---

## 📊 Success Metrics

### Velocity Indicators
- **Issues closed per day:** Target 3-5
- **PR merge time:** Target <2 hours (auto-merge)
- **Human review time:** Target <15 min per PR
- **Memory sync frequency:** Daily minimum

### Quality Gates
- **Test coverage:** ≥80% enforced
- **CI pass rate:** >95% target
- **Contract violations:** 0 (enforced)
- **Merge conflicts:** <1 per week

### Context Efficiency
- **Serena memory reads:** Prefer over full file reads
- **Symbolic searches:** Use before reading code bodies
- **Memory updates:** Batch when possible
- **Documentation generation:** Avoid unless truly new material

---

## 🎓 Learning & Adaptation

### Weekly Retrospective
1. Review velocity metrics
2. Identify bottlenecks
3. Update workflow strategy if needed
4. Refine memory structure

### Continuous Improvement
- Track which tasks are fastest with each actor
- Adjust decision matrix based on outcomes
- Evolve Serena memory schema as project grows
- Document new patterns in code_style_conventions

---

## 🔧 Practical Commands

### Issue Management
```bash
# Create and assign in one go
gh issue create --title "..." --assignee "Copilot" --label "phase-2-core,high"

# Bulk status check
gh issue list --json number,title,state,assignees --jq '.[] | select(.state=="OPEN")'

# Check agent progress
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

# 3. Test locally
npm test -- --coverage

# 4. Commit & push
git commit -am "fix: description"
git push origin fix/quick-issue

# 5. Create PR (if needed)
gh pr create --fill
```

---

## 💡 Pro Tips

1. **Use VSCode for exploration, GitHub agents for implementation**
2. **Update Serena memories, don't create new docs**
3. **Let agents handle >1hr tasks, you handle <30min fixes**
4. **Review PRs as they complete, don't batch**
5. **Trust the automation, intervene only on failures**
6. **Keep Serena memory lean - delete obsolete entries**
7. **Use symbolic search before reading full files**
8. **Batch similar tasks (all triage, all reviews, etc.)**

---

## 🎯 TL;DR

**You (Human):** Architect, reviewer, strategist
**GitHub Agents:** Feature factory with auto-merge
**VSCode + Serena:** Real-time assistant with memory

**Key Rule:** Memories over docs, symbolic search over file reads, agents over manual coding.