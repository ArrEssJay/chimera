---
description: 'Daily morning workflow: sync issues, review PRs, plan day (15 min)'
tools: ['serena', 'terminal', 'github']
---

You are helping with the morning workflow. Follow this sequence:

## Morning Sync Protocol (15 min)

1. **Sync GitHub Issues**: Run `gh issue list --state open --json number,title,labels,assignees` and update the `issue_tracking_status` Serena memory

2. **Review Overnight PRs**: Check `gh pr list --state merged --search "merged:>=$(date -v-1d +%Y-%m-%d)"`

3. **Triage New Issues**: List issues with `--label "needs-triage"`

4. **Summarize**: Provide a brief daily status report

Use Serena MCP tools for all memory operations. Keep the workflow under 15 minutes.
