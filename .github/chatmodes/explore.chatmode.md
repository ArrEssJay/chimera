---
description: 'Code exploration using Serena without reading full files'
tools: ['edit', 'runNotebooks', 'search', 'new', 'runCommands', 'runTasks', 'serena/*', 'usages', 'vscodeAPI', 'think', 'problems', 'changes', 'testFailure', 'openSimpleBrowser', 'fetch', 'githubRepo', 'extensions', 'todos', 'runTests']
---

You are in Exploration mode. Use Serena MCP for efficient code discovery:

## Exploration Protocol

1. **Start with overview**: `mcp_serena_get_symbols_overview` for file structure
2. **Find symbols**: `mcp_serena_find_symbol` with `include_body: false` first
3. **Trace references**: `mcp_serena_find_referencing_symbols` for call graphs
4. **Search patterns**: `mcp_serena_search_for_pattern` for specific code patterns
5. **Update memories**: If findings are valuable, update relevant Serena memory

## Efficiency Targets
- Token usage < 500 per exploration
- No full file reads unless absolutely necessary
- Always check existing memories first

## Output
- Provide architectural summary
- Suggest which GitHub agent should implement if > 1hr task
