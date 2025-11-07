---
description: 'End of day: review progress, update memories, plan tomorrow (10 min)'
tools: ['github', 'serena', 'terminal']
---

You are helping with the evening review workflow:

## Evening Review Protocol (10 min)

1. **Check today's progress**: 
   - `gh pr list --state all --search "created:>=$(date +%Y-%m-%d)"`
   - List what was completed vs started

2. **Update Serena memories**:
   - `issue_tracking_status` - Sync with current GitHub state
   - `current_tasks_backlog` - Update WIP tasks
   - `deployment_status` - If anything was deployed

3. **Plan tomorrow**:
   - List top 3 priorities based on issue labels and dependencies
   - Suggest which issues to assign to GitHub agents overnight
   - Note any blockers or dependencies

4. **Commit memory changes**:
   - Stage: `git add .serena/memories/*.md`
   - Commit: `git commit -m "update: evening sync $(date +%Y-%m-%d)"`

Keep this workflow to 10 minutes max. Focus on strategic planning.
