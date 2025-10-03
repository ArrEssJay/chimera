# Contract Definitions for Chimera Node Graph System

## ⚠️ LOCKED CONTRACTS - DO NOT MODIFY

These contract files define the stable interfaces that all agents must implement against. They are **LOCKED** and require team approval to change.

## Files

### TypeScript Contracts
- **`node-types.ts`** - All TypeScript interface definitions
  - `NodeDefinition` - Node metadata
  - `GraphAPI` - Graph manipulation interface
  - `DataType` - Supported data types
  - `ValidationResult`, `GraphResult` - Return types

### Rust Contracts
- **`node-trait.rs`** - All Rust trait and type definitions
  - `Node` trait - Core node interface
  - `NodeDefinition` struct - Node metadata (matches TS)
  - `DataBuffer` enum - Data passing between nodes
  - `Graph`, `Edge` - Graph structure
  - `GraphExecutor` trait - Execution interface

## Usage for Agents

### When implementing a new DSP node (Rust):

```rust
use chimera_core::contracts::*;

pub struct MyCustomNode {
    id: String,
}

impl Node for MyCustomNode {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn definition(&self) -> NodeDefinition {
        // Return your node's metadata
    }
    
    fn execute(
        &self,
        inputs: Vec<DataBuffer>,
        params: JsValue,
    ) -> Result<Vec<DataBuffer>, JsValue> {
        // Implement your processing logic
        // MUST NOT PANIC - return Err() instead
    }
}
```

### When implementing UI components (TypeScript):

```typescript
import type { NodeDefinition, GraphAPI } from '../contracts/node-types';

export function MyNodeComponent({ definition }: { definition: NodeDefinition }) {
  // Use the NodeDefinition interface
  // TypeScript will enforce the contract
}
```

## Contract Modification Process

If you believe a contract needs to be changed:

1. Open an issue explaining the need
2. Discuss with the team
3. Get approval from maintainer
4. Update BOTH TypeScript and Rust versions
5. Ensure they stay in sync
6. Update all implementations
7. Increment version number

## Version History

- **v1.0.0** (2025-10-04) - Initial contract definitions
  - Node trait
  - DataBuffer types
  - Graph API
  - Validation interfaces

## Validation

Contracts are validated by:
- TypeScript compiler checks
- Rust type system checks
- Integration tests that verify Rust/TS compatibility
- CI workflow that enforces contract stability

## Questions?

If anything is unclear about the contracts:
- Open an issue with the `contracts` label
- Tag the maintainer for clarification
- DO NOT make assumptions - ask first!
