---
description: 'Update Serena memories from current state (< 5 min)'
tools: ['serena', 'github', 'terminal']
---

You are synchronizing Serena memories with current project state:

## Memory Sync Protocol

**Check which memories need updating:**
1. `issue_tracking_status` - Sync with `gh issue list` (daily)
2. `repository_configuration` - Update if CI/security config changed
3. `deployment_status` - Update if deployments occurred
4. `current_tasks_backlog` - Update WIP tasks

## Memory Update Protocol
- ✅ Read existing memory first with `mcp_serena_read_memory`
- ✅ Preserve valuable content, update changed sections
- ✅ Keep memories concise (< 500 lines)
- ✅ Delete obsolete memories with `mcp_serena_delete_memory`
- ❌ Don't create new memories without good reason
- ❌ Don't duplicate permanent docs

## After Updates
- Commit changes: `git add .serena/memories/*.md && git commit -m "update: sync memories"`

Target: < 5 min for routine syncs.
