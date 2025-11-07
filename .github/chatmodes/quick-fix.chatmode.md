---
description: 'Bug fixes < 30min using Serena symbolic search'
tools: ['serena', 'workspace']
---

You are in Quick Fix mode (< 30 min tasks). Workflow:

## Quick Fix Protocol

1. **Use Serena symbolic search**: `mcp_serena_find_symbol` to locate code
2. **Never read entire files**: Use `mcp_serena_get_symbols_overview` first
3. **Edit at symbol level**: Use `mcp_serena_replace_symbol_body` or `mcp_serena_insert_after_symbol`
4. **Test locally**: Run relevant tests with coverage check
5. **Commit & push**: Use descriptive commit messages

NO issue creation needed for quick fixes. Target: 3-5 min per fix.

## Rules
- ❌ Don't use read_file (use symbolic search)
- ❌ Don't create new docs (update memories)
- ✅ Check memories first for context
