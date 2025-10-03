# MCP Server Configuration for Enhanced Agent Capabilities

## Overview

Model Context Protocol (MCP) servers provide additional capabilities to AI agents beyond the default toolset. For the Chimera project, we'll configure MCP servers to enhance agent productivity.

---

## MCP Servers for Chimera

### 1. GitHub MCP Server (Integrated)

**Purpose:** Direct GitHub API access for issue management, PR creation, code review

**Already Integrated:** GitHub Copilot has built-in GitHub MCP access

**Capabilities:**
- ✅ Create/update issues
- ✅ Create/update pull requests
- ✅ Add comments and reviews
- ✅ Manage labels and milestones
- ✅ Query repository metadata

**Configuration:** No additional setup needed (uses repository permissions)

---

### 2. Context7 MCP Server (Integrated)

**Purpose:** Access up-to-date library documentation

**Already Integrated:** Available in GitHub Copilot workspace

**Useful for Chimera:**
```typescript
// Agent can query library docs during implementation
// Example: When implementing React Flow integration
@context7 react-flow-renderer
// Returns latest React Flow documentation

@context7 rust wasm-bindgen
// Returns wasm-bindgen documentation
```

**Usage in Agent Instructions:**
```markdown
When implementing React Flow features, use:
@context7 /vercel/react-flow

When implementing WASM bindings, use:
@context7 rustwasm/wasm-bindgen
```

---

### 3. GitKraken MCP Server (Recommended)

**Purpose:** Advanced git operations, workspace management

**Installation:**
```bash
# Add to .github/copilot-mcp.json
{
  "mcpServers": {
    "gitkraken": {
      "command": "npx",
      "args": ["-y", "@gitkraken/mcp-server"]
    }
  }
}
```

**Capabilities:**
- Git operations (branch, commit, merge)
- Workspace management
- Pull request operations
- Issue tracking integration

**Use Cases for Chimera:**
- Automated branch creation for parallel work
- Git stash management during context switches
- Workspace navigation across mono-repo

---

### 4. Filesystem MCP Server (Recommended)

**Purpose:** Enhanced file operations beyond basic read/write

**Installation:**
```bash
# Add to .github/copilot-mcp.json
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem"],
      "env": {
        "ALLOWED_DIRECTORIES": "chimera-web/src,chimera-core/src,contracts"
      }
    }
  }
}
```

**Capabilities:**
- Directory traversal with permissions
- File search and grep
- Batch file operations
- Template expansion

**Use Cases for Chimera:**
- Batch component generation
- Search across codebase for patterns
- Template-based node creation

---

### 5. Memory MCP Server (Highly Recommended)

**Purpose:** Persistent memory across agent sessions

**Installation:**
```bash
# Add to .github/copilot-mcp.json
{
  "mcpServers": {
    "memory": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-memory"]
    }
  }
}
```

**Capabilities:**
- Store context between sessions
- Remember architecture decisions
- Track completed work
- Store reusable patterns

**Use Cases for Chimera:**
```typescript
// Agent stores patterns for reuse
memory.store("button-component-pattern", {
  structure: "variants, sizes, states",
  testPattern: "render, interaction, accessibility",
  coverage: "80%"
});

// Later, another agent retrieves
const pattern = memory.get("button-component-pattern");
// Applies same pattern to Select component
```

---

### 6. Playwright MCP Server (Recommended for E2E)

**Purpose:** Enhanced browser automation for testing

**Installation:**
```bash
# Add to .github/copilot-mcp.json
{
  "mcpServers": {
    "playwright": {
      "command": "npx",
      "args": ["-y", "@playwright/mcp-server"]
    }
  }
}
```

**Capabilities:**
- Visual regression testing
- Accessibility testing
- Performance profiling
- Screenshot comparison

**Use Cases for Chimera:**
```typescript
// Agent generates visual regression tests
await playwright.screenshot({
  selector: '.constellation-diagram',
  path: 'baselines/constellation.png'
});

await playwright.compareScreenshots({
  actual: 'current.png',
  expected: 'baselines/constellation.png',
  threshold: 0.1
});
```

---

### 7. Code Analysis MCP Server (Custom)

**Purpose:** Static analysis specific to Chimera codebase

**Implementation:**
```typescript
// .github/mcp-servers/code-analysis.ts
import { Server } from '@modelcontextprotocol/sdk/server/index.js';

const server = new Server({
  name: 'chimera-code-analysis',
  version: '1.0.0'
}, {
  capabilities: {
    tools: {}
  }
});

server.setRequestHandler('tools/call', async (request) => {
  if (request.params.name === 'analyze-node') {
    // Custom analysis for DSP nodes
    return analyzeNodeImplementation(request.params.arguments);
  }
  
  if (request.params.name === 'verify-contract') {
    // Verify implementation matches contract
    return verifyContractCompliance(request.params.arguments);
  }
  
  if (request.params.name === 'check-parallel-safety') {
    // Check if changes are safe for parallel work
    return checkParallelSafety(request.params.arguments);
  }
});

server.start();
```

**Configuration:**
```json
{
  "mcpServers": {
    "chimera-analysis": {
      "command": "node",
      "args": [".github/mcp-servers/code-analysis.js"]
    }
  }
}
```

---

## Complete MCP Configuration

**File: `.github/copilot-mcp.json`**

```json
{
  "mcpServers": {
    "github": {
      "type": "builtin",
      "enabled": true
    },
    
    "context7": {
      "type": "builtin",
      "enabled": true
    },
    
    "gitkraken": {
      "command": "npx",
      "args": ["-y", "@gitkraken/mcp-server"],
      "enabled": true
    },
    
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem"],
      "env": {
        "ALLOWED_DIRECTORIES": "chimera-web/src,chimera-core/src,contracts,docs,.github"
      },
      "enabled": true
    },
    
    "memory": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-memory"],
      "env": {
        "MEMORY_DIR": ".github/agent-memory"
      },
      "enabled": true
    },
    
    "playwright": {
      "command": "npx",
      "args": ["-y", "@playwright/mcp-server"],
      "enabled": true
    },
    
    "chimera-analysis": {
      "command": "node",
      "args": [".github/mcp-servers/code-analysis.js"],
      "enabled": true
    }
  },
  
  "mcpSettings": {
    "timeout": 30000,
    "retries": 3,
    "logging": {
      "level": "info",
      "destination": ".github/mcp-logs"
    }
  }
}
```

---

## Agent Memory Structure

**File: `.github/agent-memory/README.md`**

```markdown
# Agent Memory Storage

This directory stores persistent memory for agents across sessions.

## Structure

```
agent-memory/
├── patterns/
│   ├── component-patterns.json
│   ├── node-patterns.json
│   └── test-patterns.json
├── decisions/
│   ├── architecture-decisions.json
│   └── api-decisions.json
├── progress/
│   ├── wave-2-status.json
│   ├── wave-3-status.json
│   └── completed-tasks.json
└── knowledge/
    ├── codebase-map.json
    └── dependency-graph.json
```

## Usage

Agents can store and retrieve information:

```typescript
// Store a pattern
await memory.store('patterns/button-component', {
  structure: { variants: 3, sizes: 3, states: 4 },
  testCoverage: 85,
  accessibility: 'WCAG 2.1 AA',
  lessons: ['Use CSS variables', 'Test keyboard nav', 'ARIA labels']
});

// Retrieve for reuse
const pattern = await memory.get('patterns/button-component');
// Apply pattern to new component
```

## Retention

- **Patterns:** Permanent (until explicitly deleted)
- **Decisions:** Permanent
- **Progress:** Cleared after wave completion
- **Knowledge:** Updated continuously
```

---

## Custom MCP Server: Contract Validator

**File: `.github/mcp-servers/contract-validator.ts`**

```typescript
import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { readFileSync } from 'fs';
import * as ts from 'typescript';

const server = new Server({
  name: 'chimera-contract-validator',
  version: '1.0.0'
}, {
  capabilities: {
    tools: {
      'verify-contract-compliance': {
        description: 'Verify implementation matches contract interface',
        inputSchema: {
          type: 'object',
          properties: {
            contractPath: { type: 'string' },
            implementationPath: { type: 'string' }
          },
          required: ['contractPath', 'implementationPath']
        }
      },
      
      'check-parallel-safety': {
        description: 'Check if changes are safe for parallel work',
        inputSchema: {
          type: 'object',
          properties: {
            changedFiles: { type: 'array', items: { type: 'string' } }
          },
          required: ['changedFiles']
        }
      }
    }
  }
});

server.setRequestHandler('tools/call', async (request) => {
  const { name, arguments: args } = request.params;
  
  if (name === 'verify-contract-compliance') {
    // Load contract
    const contractSource = readFileSync(args.contractPath, 'utf-8');
    const implSource = readFileSync(args.implementationPath, 'utf-8');
    
    // Parse TypeScript
    const contractProgram = ts.createProgram([args.contractPath], {});
    const implProgram = ts.createProgram([args.implementationPath], {});
    
    // Check if implementation matches contract
    const violations = checkContractCompliance(contractProgram, implProgram);
    
    return {
      content: [{
        type: 'text',
        text: JSON.stringify({
          compliant: violations.length === 0,
          violations
        })
      }]
    };
  }
  
  if (name === 'check-parallel-safety') {
    const fileOwnershipMap = loadFileOwnershipMap();
    const conflicts = checkFileConflicts(args.changedFiles, fileOwnershipMap);
    
    return {
      content: [{
        type: 'text',
        text: JSON.stringify({
          safe: conflicts.length === 0,
          conflicts
        })
      }]
    };
  }
});

server.connect();
```

---

## MCP Server for Wave Management

**File: `.github/mcp-servers/wave-manager.ts`**

```typescript
import { Server } from '@modelcontextprotocol/sdk/server/index.js';

const server = new Server({
  name: 'chimera-wave-manager',
  version: '1.0.0'
}, {
  capabilities: {
    tools: {
      'check-wave-status': {
        description: 'Check if a wave is unlocked and ready for work'
      },
      'report-task-complete': {
        description: 'Report task completion and update wave progress'
      },
      'check-dependencies': {
        description: 'Check if all dependencies are met for a task'
      }
    }
  }
});

server.setRequestHandler('tools/call', async (request) => {
  const { name, arguments: args } = request.params;
  
  if (name === 'check-wave-status') {
    const waveStatus = await loadWaveStatus(args.wave);
    
    return {
      content: [{
        type: 'text',
        text: JSON.stringify({
          wave: args.wave,
          unlocked: waveStatus.unlocked,
          blockedBy: waveStatus.blockedBy,
          progress: waveStatus.progress
        })
      }]
    };
  }
  
  if (name === 'report-task-complete') {
    await updateWaveProgress(args.wave, args.taskId, 'complete');
    const nextUnlocked = await checkWaveGates(args.wave);
    
    return {
      content: [{
        type: 'text',
        text: JSON.stringify({
          taskCompleted: args.taskId,
          waveProgress: await getWaveProgress(args.wave),
          nextWaveUnlocked: nextUnlocked
        })
      }]
    };
  }
  
  if (name === 'check-dependencies') {
    const deps = await checkTaskDependencies(args.taskId);
    
    return {
      content: [{
        type: 'text',
        text: JSON.stringify({
          allMet: deps.every(d => d.met),
          dependencies: deps
        })
      }]
    };
  }
});

server.connect();
```

---

## Testing MCP Server Configuration

```bash
#!/bin/bash
# test-mcp-servers.sh

echo "🧪 Testing MCP server configuration..."

# 1. Check if MCP config exists
if [ ! -f ".github/copilot-mcp.json" ]; then
  echo "❌ MCP config not found"
  exit 1
fi

# 2. Validate JSON
jq empty .github/copilot-mcp.json
if [ $? -eq 0 ]; then
  echo "✅ MCP config is valid JSON"
else
  echo "❌ MCP config has invalid JSON"
  exit 1
fi

# 3. Test each server
for server in $(jq -r '.mcpServers | keys[]' .github/copilot-mcp.json); do
  echo "Testing $server..."
  
  # Check if server is enabled
  enabled=$(jq -r ".mcpServers.$server.enabled // true" .github/copilot-mcp.json)
  
  if [ "$enabled" = "true" ]; then
    echo "  ✅ $server is enabled"
  else
    echo "  ⏭️  $server is disabled"
  fi
done

echo "✅ All MCP servers configured correctly"
```

---

## Summary

### Integrated MCP Servers
1. ✅ **GitHub** - Built-in, always available
2. ✅ **Context7** - Built-in, always available

### Recommended Additional Servers
3. 🔧 **GitKraken** - Git operations and workspace management
4. 🔧 **Filesystem** - Enhanced file operations
5. 🔧 **Memory** - Persistent agent memory
6. 🔧 **Playwright** - E2E testing enhancement

### Custom Chimera Servers
7. 🛠️ **Contract Validator** - Verify contract compliance
8. 🛠️ **Wave Manager** - Track parallel work progress
9. 🛠️ **Code Analysis** - Chimera-specific analysis

### Next Steps
1. Create `.github/copilot-mcp.json`
2. Implement custom MCP servers
3. Test configuration
4. Update agent instructions to use MCP capabilities
5. Monitor MCP logs for issues

**Result:** Enhanced agent capabilities with contract validation, wave management, and persistent memory!
