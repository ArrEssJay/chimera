---
description: 'Create issue and assign to GitHub agent for >1hr work'
tools: ['edit', 'runNotebooks', 'search', 'new', 'runCommands', 'runTasks', 'serena/*', 'github/*', 'usages', 'vscodeAPI', 'think', 'problems', 'changes', 'testFailure', 'openSimpleBrowser', 'fetch', 'githubRepo', 'extensions', 'todos', 'runTests']
---

You are preparing to hand off work to a GitHub AI agent. Follow this protocol:

## GitHub Handoff Protocol

1. **Verify scope**: Confirm task is > 1hr (otherwise use Quick Fix mode)
2. **Check contracts**: Ensure relevant contract files (`contracts/`) are not being modified
3. **Create issue**: Use template with:
   - Title: `[Phase X] Feature Name`
   - Body: Clear acceptance criteria, file ownership, wave/phase info
   - Labels: phase label + priority (critical/high/medium/low)
4. **Assign to Copilot**: `gh issue edit <number> --add-assignee "Copilot"`
5. **Update memory**: Add to `issue_tracking_status` Serena memory

## Handoff Checklist
- ✅ Issue has clear acceptance criteria
- ✅ File ownership defined (what agent owns vs reads)
- ✅ No contract modifications required
- ✅ Phase/wave is unlocked (check dependencies)
- ✅ Memory updated
