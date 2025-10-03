# Safe Parallelization Strategy for Epic #40

**Problem:** How to parallelize agent work without introducing dependency conflicts, merge conflicts, or integration failures?

**Solution:** Contract-first development + Dependency waves + File ownership + Integration branches

---

## 🌊 Dependency Wave Model

Execute work in **dependency waves**. Within each wave, agents work in parallel with zero conflicts. Between waves, integration gates ensure contracts are met.

```
Wave 1: Foundation (Sequential - must complete first)
  ├─ #45: React Setup ✅ COMPLETE
  └─ Contracts Definition (Human) - CRITICAL GATE
  
Wave 2: Independent Components (Parallel - no dependencies)
  ├─ #46: UI Components (5 agents in parallel)
  │   ├─ Agent A: Button.tsx
  │   ├─ Agent B: Select.tsx
  │   ├─ Agent C: Panel.tsx
  │   ├─ Agent D: Tooltip.tsx
  │   └─ Agent E: Badge.tsx
  └─ Merge to: feature/ui-components branch
  
Wave 3: Core Architecture (Sequential within, parallel groups)
  ├─ #47: Node Graph Core (4 agents, dependencies managed)
  │   ├─ Agent F: DataBuffer types (FIRST)
  │   ├─ Agent G: Graph structures (after F)
  │   ├─ Agent H: Validator (after G)
  │   └─ Agent I: Executor (after all above)
  └─ Merge to: feature/node-graph-core branch
  
Wave 4: Parallel Nodes (15 agents in parallel)
  ├─ #48: Built-in Nodes (depends on Wave 3 complete)
  │   ├─ Agent J: BitGeneratorNode.rs
  │   ├─ Agent K: NoiseGeneratorNode.rs
  │   ├─ Agent L: LdpcEncoderNode.rs
  │   └─ ... 12 more agents (one per node)
  └─ Merge to: feature/builtin-nodes branch
  
Wave 5: Graph Editor (6 agents, managed dependencies)
  ├─ #49: Graph Editor UI (depends on Wave 3 + Wave 4)
  │   ├─ Agent M: ReactFlowSetup.tsx (FIRST)
  │   ├─ Agent N: NodePalette.tsx (after M)
  │   ├─ Agent O: CustomNode.tsx (after M)
  │   ├─ Agent P: ConnectionValidation.ts (after O)
  │   ├─ Agent Q: NodeInspector.tsx (after O)
  │   └─ Agent R: GraphExecution.ts (after all above)
  └─ Merge to: feature/graph-editor branch
  
Wave 6: Integration (Sequential)
  ├─ #51: Graph State Management
  ├─ #52: Graph Editor Integration
  └─ Merge to: feature/integration branch
  
Wave 7: Final Integration to Main
  └─ Merge all feature branches → main
```

---

## 🔒 Contract-First Development Protocol

**RULE:** All public interfaces must be defined and locked BEFORE parallel agent work begins.

### Phase 1: Human Defines Contracts (1 day)

Before any agent starts coding, humans create **contract files** that define all interfaces:

```rust
// contracts/node_trait.rs - LOCKED, DO NOT MODIFY
/// Core trait that all DSP nodes must implement
pub trait Node: Send + Sync {
    /// Unique identifier for this node
    fn id(&self) -> &str;
    
    /// Node metadata (name, category, ports)
    fn definition(&self) -> NodeDefinition;
    
    /// Execute node processing
    fn execute(&self, inputs: Vec<DataBuffer>, params: JsValue) 
        -> Result<Vec<DataBuffer>, JsValue>;
}

// contracts/data_buffer.rs - LOCKED, DO NOT MODIFY
#[derive(Clone, Debug)]
pub enum DataBuffer {
    BitStream(Vec<bool>),
    IQData(Vec<Complex<f32>>),
    AudioSamples(Vec<f32>),
    Metadata(HashMap<String, String>),
}

// contracts/graph.rs - LOCKED, DO NOT MODIFY
pub struct Graph {
    pub nodes: HashMap<String, Box<dyn Node>>,
    pub edges: Vec<Edge>,
}

pub struct Edge {
    pub from_node: String,
    pub from_port: usize,
    pub to_node: String,
    pub to_port: usize,
}
```

```typescript
// contracts/node-types.ts - LOCKED, DO NOT MODIFY
export interface NodeDefinition {
  id: string;
  name: string;
  category: 'source' | 'processing' | 'analysis' | 'sink';
  inputs: PortDefinition[];
  outputs: PortDefinition[];
  parameters: ParameterDefinition[];
}

export interface PortDefinition {
  id: string;
  name: string;
  type: 'BitStream' | 'IQData' | 'AudioSamples' | 'Metadata';
}

// contracts/graph-api.ts - LOCKED, DO NOT MODIFY
export interface GraphAPI {
  createNode(type: string, position: Position): Promise<string>;
  connectNodes(fromId: string, fromPort: number, toId: string, toPort: number): Promise<void>;
  executeGraph(): Promise<GraphResult>;
  validateGraph(): Promise<ValidationResult>;
}
```

### Phase 2: Agents Implement Against Contracts

Once contracts are locked:
- ✅ Agents can work in parallel
- ✅ TypeScript/Rust type checking catches violations
- ✅ No API mismatches
- ✅ Integration is predictable

---

## 📁 File Ownership Strategy

**RULE:** Each agent owns distinct files. No two agents edit the same file simultaneously.

### Issue #46 - UI Components (Parallel Safe ✅)

```yaml
agent_a:
  owns:
    - chimera-web/src/components/Button.tsx
    - chimera-web/src/components/Button.test.tsx
    - chimera-web/src/components/Button.stories.tsx
  reads:
    - chimera-web/style.css (read-only)
  conflicts: NONE

agent_b:
  owns:
    - chimera-web/src/components/Select.tsx
    - chimera-web/src/components/Select.test.tsx
    - chimera-web/src/components/Select.stories.tsx
  reads:
    - chimera-web/style.css (read-only)
  conflicts: NONE

# ... agents C, D, E for Panel, Tooltip, Badge

integration_agent:
  owns:
    - chimera-web/src/components/index.ts  # Barrel export
  waits_for: [agent_a, agent_b, agent_c, agent_d, agent_e]
  action: "Export all components"
```

### Issue #48 - Built-in Nodes (Parallel Safe ✅)

```yaml
agent_j:
  owns:
    - chimera-core/src/nodes/bit_generator.rs
    - chimera-core/src/nodes/bit_generator_test.rs
  implements: contracts/node_trait.rs
  conflicts: NONE

agent_k:
  owns:
    - chimera-core/src/nodes/noise_generator.rs
    - chimera-core/src/nodes/noise_generator_test.rs
  implements: contracts/node_trait.rs
  conflicts: NONE

# ... agents L-X for remaining 13 nodes

registry_agent:
  owns:
    - chimera-core/src/nodes/mod.rs  # Module exports
    - chimera-core/src/registry.rs   # Node registry
  waits_for: [all_node_agents]
  action: "Register all nodes"
```

### Issue #47 - Node Graph Core (Sequential Dependencies ⚠️)

```yaml
agent_f:  # FIRST - foundational types
  owns:
    - chimera-core/src/data_buffer.rs
  implements: contracts/data_buffer.rs
  blocks: [agent_g, agent_h, agent_i]

agent_g:  # SECOND - depends on data types
  owns:
    - chimera-core/src/graph.rs
  depends_on: [agent_f]
  blocks: [agent_h, agent_i]

agent_h:  # THIRD - depends on graph structures
  owns:
    - chimera-core/src/validator.rs
  depends_on: [agent_g]
  blocks: [agent_i]

agent_i:  # FOURTH - depends on everything
  owns:
    - chimera-core/src/executor.rs
  depends_on: [agent_f, agent_g, agent_h]
```

**Strategy:** Sequential execution with hand-off gates. Agent G starts ONLY after Agent F's PR is merged.

---

## 🌳 Branch Strategy for Parallel Work

**RULE:** Use feature branches to isolate parallel work. Merge to integration branch, not main.

```
main
  │
  ├─── feature/ui-components (Wave 2)
  │     ├─── button-component (Agent A PR)
  │     ├─── select-component (Agent B PR)
  │     ├─── panel-component (Agent C PR)
  │     ├─── tooltip-component (Agent D PR)
  │     └─── badge-component (Agent E PR)
  │     └─── [All merged to feature/ui-components]
  │
  ├─── feature/node-graph-core (Wave 3)
  │     ├─── data-buffer-types (Agent F PR)
  │     ├─── graph-structures (Agent G PR - waits for F)
  │     ├─── graph-validator (Agent H PR - waits for G)
  │     └─── graph-executor (Agent I PR - waits for H)
  │     └─── [All merged sequentially to feature/node-graph-core]
  │
  ├─── feature/builtin-nodes (Wave 4)
  │     ├─── bit-generator-node (Agent J PR)
  │     ├─── noise-generator-node (Agent K PR)
  │     └─── ... (13 more parallel PRs)
  │     └─── [All merged to feature/builtin-nodes]
  │
  ├─── feature/graph-editor (Wave 5)
  │     └─── [6 agents with managed dependencies]
  │
  └─── [Feature branches merged to main in Wave 7]
```

### Merge Protocol

1. **Within Wave:**
   - Agents create PR to feature branch (e.g., `button-component` → `feature/ui-components`)
   - CI validates in isolation
   - Human reviews API contracts
   - Auto-merge if gates pass

2. **Between Waves:**
   - Feature branch merged to `main` only after ALL PRs in wave complete
   - Integration tests run on feature branch
   - Human validates wave completion

3. **Conflict Resolution:**
   - If conflict detected, halt parallel work
   - Resolve conflict in feature branch
   - Re-run all tests
   - Resume parallel work

---

## 🧪 Mock/Stub Strategy for Early Parallelization

**Problem:** Wave 5 (Graph Editor) needs Wave 3 (Node Graph Core) + Wave 4 (Nodes), but we don't want to wait.

**Solution:** Mock the dependencies with stubs that match contracts.

### Example: Mock WASM Core for UI Development

```typescript
// chimera-web/src/mocks/graph-core.mock.ts
// Allows Wave 5 to start before Wave 3 completes

import type { GraphAPI, NodeDefinition } from '../contracts/node-types';

export class MockGraphCore implements GraphAPI {
  private nodes = new Map<string, MockNode>();
  
  async createNode(type: string, position: Position): Promise<string> {
    const id = `mock-node-${Math.random()}`;
    this.nodes.set(id, { type, position });
    return id;
  }
  
  async connectNodes(fromId: string, fromPort: number, toId: string, toPort: number): Promise<void> {
    // Mock implementation - no actual graph validation
    console.log(`Mock: Connecting ${fromId}:${fromPort} -> ${toId}:${toPort}`);
  }
  
  async executeGraph(): Promise<GraphResult> {
    // Mock implementation - returns fake data
    return {
      success: true,
      outputs: [{ type: 'IQData', data: [/* mock IQ samples */] }],
    };
  }
  
  async validateGraph(): Promise<ValidationResult> {
    // Mock implementation - always valid
    return { valid: true, errors: [] };
  }
}

// In UI code, use dependency injection
const graphCore = import.meta.env.MODE === 'mock' 
  ? new MockGraphCore() 
  : await import('../wasm/graph-core');
```

**Benefits:**
- Wave 5 agents can start immediately
- UI development unblocked
- When Wave 3 completes, swap mock for real implementation
- Contract ensures compatibility

---

## ⚠️ Conflict Detection & Prevention

### Automated Conflict Detection

```yaml
# .github/workflows/conflict-detection.yml
name: Parallel Work Conflict Detection

on:
  pull_request:
    types: [opened, synchronize]

jobs:
  detect-conflicts:
    runs-on: ubuntu-latest
    steps:
      - name: Check for file ownership violations
        run: |
          # Get list of files modified in this PR
          CHANGED_FILES=$(git diff --name-only origin/main..HEAD)
          
          # Get list of files in other open PRs
          OTHER_PR_FILES=$(gh pr list --json files --jq '.[].files[].path')
          
          # Check for overlap
          CONFLICTS=$(comm -12 <(echo "$CHANGED_FILES" | sort) <(echo "$OTHER_PR_FILES" | sort))
          
          if [ ! -z "$CONFLICTS" ]; then
            echo "⚠️ WARNING: File conflicts detected with other PRs:"
            echo "$CONFLICTS"
            echo ""
            echo "These files are being modified in parallel PRs."
            echo "Coordinate with other agents or wait for their PRs to merge."
            exit 1
          fi
          
          echo "✅ No file conflicts detected"
      
      - name: Check for dependency violations
        run: |
          # Check if this PR depends on unreleased features
          # Parse contracts and check if all dependencies are met
          python scripts/check-dependencies.py
```

### Human Oversight Dashboard

```yaml
# dashboard/parallel-work-status.yml
waves:
  wave_2:
    name: "UI Components"
    status: "in_progress"
    agents:
      - name: "Agent A - Button"
        pr: "#101"
        status: "review"
        conflicts: []
      - name: "Agent B - Select"
        pr: "#102"
        status: "ci_running"
        conflicts: []
      - name: "Agent C - Panel"
        pr: "#103"
        status: "draft"
        conflicts: []
    
  wave_3:
    name: "Node Graph Core"
    status: "blocked"
    waiting_for: "wave_2"
    agents:
      - name: "Agent F - DataBuffer"
        status: "ready"
        conflicts: []
```

---

## 📊 Integration Gates Between Waves

**RULE:** A wave cannot start until the previous wave passes integration gates.

### Integration Gate Checklist

```yaml
wave_2_complete:
  criteria:
    - "✅ All 5 UI component PRs merged"
    - "✅ Integration test: render all components"
    - "✅ Storybook builds successfully"
    - "✅ No TypeScript errors"
    - "✅ Coverage ≥80%"
  action: "Unlock Wave 3"

wave_3_complete:
  criteria:
    - "✅ Node trait implemented"
    - "✅ Graph structures complete"
    - "✅ Validator passes all tests"
    - "✅ Executor passes integration tests"
    - "✅ WASM builds successfully (<5MB)"
    - "✅ TypeScript types exported"
  action: "Unlock Wave 4 + Wave 5 (with mocks)"

wave_4_complete:
  criteria:
    - "✅ All 15 nodes implemented"
    - "✅ Each node passes unit tests"
    - "✅ Node registry complete"
    - "✅ Integration test: QPSK pipeline works"
  action: "Unlock Wave 5 (remove mocks)"
```

### Automated Gate Enforcement

```yaml
# .github/workflows/wave-gate.yml
name: Wave Integration Gate

on:
  workflow_dispatch:
    inputs:
      wave:
        description: 'Wave to validate'
        required: true
        type: choice
        options:
          - wave_2
          - wave_3
          - wave_4
          - wave_5

jobs:
  validate-wave:
    runs-on: ubuntu-latest
    steps:
      - name: Run wave validation
        run: |
          case "${{ inputs.wave }}" in
            wave_2)
              npm test -- --testPathPattern=components
              npm run build-storybook
              ;;
            wave_3)
              cargo test --lib
              wasm-pack build
              npm run test:wasm-integration
              ;;
            wave_4)
              cargo test --lib nodes::
              cargo test --test qpsk_pipeline
              ;;
          esac
      
      - name: Update wave status
        run: |
          gh api repos/${{ github.repository }}/issues \
            -f title="Wave ${{ inputs.wave }} Integration Complete" \
            -f body="All gates passed. Next wave unlocked."
```

---

## 🎯 Recommended Execution Plan

### Week 1: Setup + Wave 2

**Monday:**
- Human: Define contracts (all interfaces locked)
- Human: Create feature branches
- Human: Update issue #46 with agent assignments

**Tuesday-Thursday:**
- Agent A-E: Work in parallel on UI components
- CI: Validate each PR independently
- Human: Review API contracts (5 reviews total)

**Friday:**
- Merge all 5 PRs to `feature/ui-components`
- Run integration gate
- Merge `feature/ui-components` → `main`
- **Unlock Wave 3**

### Week 2: Wave 3 (Sequential)

**Monday:**
- Agent F: DataBuffer types
- Review + merge

**Tuesday:**
- Agent G: Graph structures (uses DataBuffer)
- Review + merge

**Wednesday:**
- Agent H: Validator (uses Graph)
- Review + merge

**Thursday:**
- Agent I: Executor (uses all above)
- Review + merge

**Friday:**
- Run integration gate
- Merge `feature/node-graph-core` → `main`
- **Unlock Wave 4 + Wave 5 (with mocks)**

### Week 3: Wave 4 (Massive Parallel)

**Monday-Wednesday:**
- Agent J-X: All 15 nodes in parallel
- Each agent submits PR as soon as done
- CI validates independently

**Thursday:**
- Registry agent: Integrate all nodes
- Run integration test (QPSK pipeline)

**Friday:**
- Merge `feature/builtin-nodes` → `main`
- **Unlock Wave 5 (remove mocks)**

### Week 4-5: Wave 5 (Managed Parallel)

Graph editor features with careful dependency management.

---

## 🚨 Risk Mitigation

### Risk: Parallel PRs Create Merge Conflict

**Prevention:**
- File ownership strategy (distinct files)
- Automated conflict detection
- Feature branch isolation

**Response:**
1. Halt parallel work in that wave
2. Resolve conflict in feature branch
3. Re-run all tests
4. Resume parallel work

### Risk: Agent Violates Contract

**Prevention:**
- TypeScript/Rust type checking (automatic)
- Integration tests run on every PR
- Contract files marked read-only

**Response:**
1. CI fails the PR
2. Agent revises to match contract
3. Human reviews if multiple failures

### Risk: Wave Gate Fails (Integration Issues)

**Prevention:**
- Comprehensive integration tests
- Feature branch testing before main merge
- Human validation of critical paths

**Response:**
1. Do NOT unlock next wave
2. Identify failing integration
3. Create fix PR
4. Re-run gate validation
5. Only unlock after pass

---

## 📈 Expected Outcomes

### With This Strategy:

✅ **Zero merge conflicts** - File ownership prevents overlap  
✅ **Zero integration failures** - Contracts ensure compatibility  
✅ **Maximum parallelization** - 15 nodes simultaneously in Wave 4  
✅ **Predictable timeline** - Wave model clear progress  
✅ **Safe rollback** - Feature branches isolated  

### Timeline Comparison:

**Traditional Sequential:**
- Week 1: UI components (5 days × 5 = 25 days)
- Week 6-7: Core (4 components × 2 days = 8 days)
- Week 8-10: Nodes (15 nodes × 1 day = 15 days)
- **Total: ~48 days**

**Parallel with This Strategy:**
- Week 1: UI components (5 agents in parallel = 5 days)
- Week 2: Core (sequential but fast = 5 days)
- Week 3: Nodes (15 agents in parallel = 5 days)
- **Total: ~15 days** 🚀

**3x faster while maintaining quality and safety!**

---

## 🎬 Getting Started

1. **Human: Define contracts** (1 day)
   - `contracts/node_trait.rs`
   - `contracts/data_buffer.rs`
   - `contracts/graph.rs`
   - `contracts/node-types.ts`
   - `contracts/graph-api.ts`

2. **Human: Create feature branches**
   ```bash
   git checkout -b feature/ui-components
   git push -u origin feature/ui-components
   
   git checkout main
   git checkout -b feature/node-graph-core
   git push -u origin feature/node-graph-core
   
   # ... etc
   ```

3. **Human: Configure branch protection**
   - Require PR reviews
   - Require CI checks
   - Require branch to be up-to-date

4. **Human: Brief agents**
   - Share contract files
   - Assign file ownership
   - Explain wave model

5. **Start Wave 2!** 🚀

---

**"The secret to going fast is to never get stuck. The secret to never getting stuck is to eliminate dependencies."**
