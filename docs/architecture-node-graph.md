# Chimera Node Graph Architecture

**Vision:** Web-based GNU Radio Companion in Rust/WASM with React frontend  
**Last Updated:** 2025-10-04  
**Status:** Architectural Design

---

## 🎯 Project Vision

**Chimera is a visual dataflow DSP prototyping environment** that runs entirely in the browser:

- **Visual node graph editor** (like GNU Radio Companion, LabVIEW, Max/MSP)
- **Processing blocks** with typed inputs/outputs
- **Real-time execution** with WASM performance
- **Live analysis** with interactive visualizations
- **Shareable presets** as graph configurations

**Current QPSK/AWGN demo = First preset graph, not the application**

---

## 🏗️ Core Architecture

### Node Graph Model

```
┌─────────────┐      ┌──────────────┐      ┌─────────────┐
│ BitGenerator├─────►│  LDPC        ├─────►│  QPSK       │
│   (Source)  │      │  Encoder     │      │  Modulator  │
└─────────────┘      └──────────────┘      └──────┬──────┘
                                                   │
                                                   ▼
┌─────────────┐      ┌──────────────┐      ┌─────────────┐
│Constellation│◄─────┤  QPSK        │◄─────┤  AWGN       │
│  Diagram    │      │  Demodulator │      │  Channel    │
│  (Sink)     │      └──────────────┘      └─────────────┘
└─────────────┘
```

### Key Concepts

#### 1. **Nodes (Processing Blocks)**
Every node is a self-contained processing unit:

```typescript
interface Node {
  id: string;
  type: string;  // 'encoder', 'modulator', 'constellation', etc.
  position: { x: number; y: number };
  parameters: Record<string, any>;
  
  // Node definition
  definition: NodeDefinition;
}

interface NodeDefinition {
  id: string;
  name: string;
  category: 'source' | 'processing' | 'sink' | 'analysis';
  
  // I/O ports
  inputs: PortDefinition[];
  outputs: PortDefinition[];
  
  // Configuration
  parameters: ParameterDefinition[];
  
  // Execution
  execute: (inputs: any[], params: any) => Promise<any[]>;
}
```

#### 2. **Ports (Typed I/O)**
Ports have types to ensure valid connections:

```typescript
interface PortDefinition {
  id: string;
  name: string;
  type: DataType;
  optional?: boolean;
}

enum DataType {
  BitStream = 'bitstream',      // Raw bits
  IQData = 'iq_data',            // Complex I/Q samples
  AudioBuffer = 'audio',         // Float audio samples
  Spectrum = 'spectrum',         // FFT output
  Metrics = 'metrics',           // Scalar measurements
  Message = 'message',           // Control messages
}
```

#### 3. **Edges (Data Flow)**
Edges connect output ports to input ports:

```typescript
interface Edge {
  id: string;
  source: { nodeId: string; portId: string };
  target: { nodeId: string; portId: string };
  
  // Validation
  dataType: DataType;
}
```

#### 4. **Graph (Complete Flowgraph)**
The graph contains all nodes and edges:

```typescript
interface Graph {
  id: string;
  name: string;
  description?: string;
  
  nodes: Node[];
  edges: Edge[];
  
  // Metadata
  metadata: {
    author?: string;
    created: Date;
    modified: Date;
    version: string;
  };
}
```

---

## 📦 Node Categories

### Source Nodes (No Inputs)

#### BitGenerator
```typescript
{
  outputs: [{ id: 'bits', name: 'Bits', type: DataType.BitStream }],
  parameters: [
    { name: 'length', type: 'number', default: 1024 },
    { name: 'pattern', type: 'enum', values: ['random', 'alternating', 'prbs'] }
  ]
}
```

#### AudioFileLoader
```typescript
{
  outputs: [{ id: 'audio', name: 'Audio', type: DataType.AudioBuffer }],
  parameters: [
    { name: 'file', type: 'file', accept: 'audio/*' },
    { name: 'loop', type: 'boolean', default: false }
  ]
}
```

#### NoiseGenerator
```typescript
{
  outputs: [{ id: 'noise', name: 'Noise', type: DataType.IQData }],
  parameters: [
    { name: 'power', type: 'number', default: 0, unit: 'dBm' },
    { name: 'length', type: 'number', default: 1024 }
  ]
}
```

---

### Processing Nodes

#### LDPC Encoder
```typescript
{
  inputs: [{ id: 'bits', name: 'Bits In', type: DataType.BitStream }],
  outputs: [{ id: 'encoded', name: 'Encoded Bits', type: DataType.BitStream }],
  parameters: [
    { name: 'code_rate', type: 'number', default: 0.5 },
    { name: 'block_length', type: 'number', default: 1024 }
  ]
}
```

#### QPSK Modulator
```typescript
{
  inputs: [{ id: 'bits', name: 'Bits', type: DataType.BitStream }],
  outputs: [{ id: 'iq', name: 'I/Q Samples', type: DataType.IQData }],
  parameters: [
    { name: 'symbol_rate', type: 'number', default: 1000, unit: 'Hz' },
    { name: 'pulse_shaping', type: 'enum', values: ['none', 'rrc', 'rc'] }
  ]
}
```

#### AWGN Channel
```typescript
{
  inputs: [{ id: 'signal', name: 'Signal In', type: DataType.IQData }],
  outputs: [{ id: 'signal', name: 'Signal Out', type: DataType.IQData }],
  parameters: [
    { name: 'snr_db', type: 'number', default: 15, min: -20, max: 40 },
    { name: 'link_loss_db', type: 'number', default: 0, min: 0, max: 20 }
  ]
}
```

#### QPSK Demodulator
```typescript
{
  inputs: [{ id: 'iq', name: 'I/Q Samples', type: DataType.IQData }],
  outputs: [
    { id: 'soft_bits', name: 'Soft Decisions', type: DataType.BitStream },
    { id: 'symbols', name: 'Symbols', type: DataType.IQData }  // For analysis
  ],
  parameters: [
    { name: 'algorithm', type: 'enum', values: ['hard', 'soft'] }
  ]
}
```

#### LDPC Decoder
```typescript
{
  inputs: [{ id: 'soft_bits', name: 'Soft Bits', type: DataType.BitStream }],
  outputs: [{ id: 'decoded', name: 'Decoded Bits', type: DataType.BitStream }],
  parameters: [
    { name: 'max_iterations', type: 'number', default: 50 }
  ]
}
```

---

### Analysis/Sink Nodes

#### Constellation Diagram
```typescript
{
  inputs: [{ id: 'symbols', name: 'Symbols', type: DataType.IQData }],
  outputs: [],  // Sink node
  parameters: [
    { name: 'max_points', type: 'number', default: 1000 },
    { name: 'color', type: 'color', default: '#ff6b6b' }
  ],
  visualization: ConstellationChart  // React component
}
```

#### FFT Analyzer
```typescript
{
  inputs: [{ id: 'signal', name: 'Signal', type: DataType.IQData }],
  outputs: [{ id: 'spectrum', name: 'Spectrum', type: DataType.Spectrum }],
  parameters: [
    { name: 'fft_size', type: 'number', default: 1024, values: [256, 512, 1024, 2048] },
    { name: 'window', type: 'enum', values: ['rectangular', 'hamming', 'hann', 'blackman'] }
  ],
  visualization: PSDChart
}
```

#### BER Calculator
```typescript
{
  inputs: [
    { id: 'original', name: 'Original Bits', type: DataType.BitStream },
    { id: 'received', name: 'Received Bits', type: DataType.BitStream }
  ],
  outputs: [{ id: 'metrics', name: 'Metrics', type: DataType.Metrics }],
  parameters: [],
  visualization: MetricsPanel
}
```

#### Audio Player
```typescript
{
  inputs: [{ id: 'audio', name: 'Audio', type: DataType.AudioBuffer }],
  outputs: [],
  parameters: [
    { name: 'volume', type: 'number', default: 1.0, min: 0, max: 1 },
    { name: 'autoplay', type: 'boolean', default: false }
  ],
  visualization: AudioControls
}
```

#### File Exporter
```typescript
{
  inputs: [{ id: 'data', name: 'Data', type: DataType.Any }],
  outputs: [],
  parameters: [
    { name: 'format', type: 'enum', values: ['csv', 'json', 'matlab'] },
    { name: 'filename', type: 'string', default: 'export' }
  ]
}
```

---

### Utility Nodes

#### Delay
```typescript
{
  inputs: [{ id: 'in', name: 'In', type: DataType.Any }],
  outputs: [{ id: 'out', name: 'Out', type: DataType.Any }],
  parameters: [
    { name: 'samples', type: 'number', default: 0 }
  ]
}
```

#### Multiply (Signal Mixer)
```typescript
{
  inputs: [
    { id: 'a', name: 'A', type: DataType.IQData },
    { id: 'b', name: 'B', type: DataType.IQData }
  ],
  outputs: [{ id: 'out', name: 'Out', type: DataType.IQData }]
}
```

#### Add (Signal Combiner)
```typescript
{
  inputs: [
    { id: 'a', name: 'A', type: DataType.IQData },
    { id: 'b', name: 'B', type: DataType.IQData }
  ],
  outputs: [{ id: 'out', name: 'Out', type: DataType.IQData }]
}
```

---

## 🎨 User Interface Design

### Layout

```
┌────────────────────────────────────────────────────────────────┐
│  Chimera - DSP Prototyping Environment           [Run] [Stop]  │
├──────────┬──────────────────────────────────────┬──────────────┤
│          │                                      │              │
│  Node    │         Graph Canvas                 │   Inspector  │
│  Palette │                                      │              │
│          │   ┌──────┐       ┌──────┐           │   Selected:  │
│ 📡Source │   │ Bit  ├──────►│LDPC  │           │   QPSK Mod   │
│  • Bit   │   │ Gen  │       │Enc   │           │              │
│    Gen   │   └──────┘       └───┬──┘           │   Symbol     │
│  • Audio │                      │              │   Rate: 1000 │
│    File  │                      ▼              │              │
│  • Noise │                  ┌──────┐           │   Pulse:     │
│          │                  │QPSK  │           │   [RRC   ▼]  │
│ ⚙️ Proc  │                  │Mod   │           │              │
│  • LDPC  │                  └───┬──┘           │   [Apply]    │
│    Enc   │                      │              │              │
│  • Mods  │      ┌───────────────┘              │              │
│  • Chan  │      │                              │              │
│  • Demods│      ▼                              │              │
│  • LDPC  │  ┌──────┐       ┌──────┐           │              │
│    Dec   │  │AWGN  ├──────►│QPSK  │           │              │
│          │  │Chan  │       │Demod │           │              │
│ 📊Sink   │  └──────┘       └───┬──┘           │              │
│  • Const │                     │              │              │
│    Diag  │      ┌──────────────┴───┐          │              │
│  • FFT   │      │                  │          │              │
│  • BER   │      ▼                  ▼          │              │
│  • Audio │  ┌──────┐           ┌──────┐       │              │
│  • Export│  │Const │           │ BER  │       │              │
│          │  │Diag  │           │Calc  │       │              │
├──────────┴──┴──────┴───────────┴──────┴───────┴──────────────┤
│  Results Panel                                                │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐          │
│  │ Constellation│ │     FFT      │ │   Metrics    │          │
│  │ [Chart]      │ │  [Chart]     │ │  BER: 0.001  │          │
│  └──────────────┘ └──────────────┘ └──────────────┘          │
└────────────────────────────────────────────────────────────────┘
```

### Components

#### 1. **Node Palette** (Left Sidebar)
- Categorized list of available nodes
- Drag-and-drop to canvas
- Search/filter functionality
- Custom node plugins

#### 2. **Graph Canvas** (Center)
- Infinite canvas with pan/zoom
- Drag nodes to position
- Connect ports by dragging
- Select/delete/duplicate nodes
- Grid snapping (optional)
- Minimap for navigation

#### 3. **Node Inspector** (Right Sidebar)
- Shows selected node's parameters
- Dynamic UI based on parameter types
- Real-time validation
- "Apply" or live update

#### 4. **Results Panel** (Bottom)
- Shows outputs from analysis/sink nodes
- Tabbed interface for multiple visualizations
- Expandable/collapsible
- Export buttons

#### 5. **Toolbar** (Top)
- File menu (New, Open, Save, Export)
- Edit menu (Undo, Redo, Copy, Paste)
- View menu (Zoom, Grid, Minimap)
- Run controls (Run, Stop, Step)
- Preset selector

---

## 🔄 Execution Model

### Graph Execution Flow

```typescript
class GraphExecutor {
  async execute(graph: Graph): Promise<ExecutionResult> {
    // 1. Validate graph
    const validation = this.validate(graph);
    if (!validation.valid) throw new Error(validation.errors);
    
    // 2. Topological sort
    const sortedNodes = this.topologicalSort(graph);
    
    // 3. Initialize execution context
    const context = new ExecutionContext();
    
    // 4. Execute nodes in order
    for (const node of sortedNodes) {
      // Get inputs from connected edges
      const inputs = this.getNodeInputs(node, context);
      
      // Execute node
      const outputs = await node.definition.execute(inputs, node.parameters);
      
      // Store outputs in context
      context.setNodeOutputs(node.id, outputs);
      
      // Update UI if analysis/sink node
      if (node.definition.category === 'sink' || node.definition.category === 'analysis') {
        this.updateVisualization(node, outputs);
      }
      
      // Yield for streaming execution
      await this.yield();
    }
    
    return context.getResults();
  }
  
  private topologicalSort(graph: Graph): Node[] {
    // Kahn's algorithm or DFS
    const sorted: Node[] = [];
    const visited = new Set<string>();
    const visiting = new Set<string>();
    
    const visit = (nodeId: string) => {
      if (visited.has(nodeId)) return;
      if (visiting.has(nodeId)) throw new Error('Cycle detected');
      
      visiting.add(nodeId);
      
      // Visit dependencies (nodes that feed into this one)
      const incomingEdges = graph.edges.filter(e => e.target.nodeId === nodeId);
      for (const edge of incomingEdges) {
        visit(edge.source.nodeId);
      }
      
      visiting.delete(nodeId);
      visited.add(nodeId);
      sorted.push(graph.nodes.find(n => n.id === nodeId)!);
    };
    
    // Start from source nodes
    const sourceNodes = graph.nodes.filter(n => 
      n.definition.inputs.length === 0
    );
    
    for (const node of sourceNodes) {
      visit(node.id);
    }
    
    return sorted;
  }
}
```

### Streaming Execution

```typescript
// Execute in Web Worker with progress updates
class StreamingExecutor extends GraphExecutor {
  async *executeStreaming(graph: Graph): AsyncGenerator<ExecutionProgress> {
    const sortedNodes = this.topologicalSort(graph);
    const context = new ExecutionContext();
    
    for (let i = 0; i < sortedNodes.length; i++) {
      const node = sortedNodes[i];
      
      // Execute node
      const inputs = this.getNodeInputs(node, context);
      const outputs = await node.definition.execute(inputs, node.parameters);
      context.setNodeOutputs(node.id, outputs);
      
      // Yield progress
      yield {
        nodeId: node.id,
        nodeName: node.definition.name,
        progress: (i + 1) / sortedNodes.length,
        outputs: outputs
      };
    }
  }
}
```

---

## 💾 Presets as Graphs

### Preset Structure

A preset is simply a saved graph configuration:

```json
{
  "id": "aid-3-qpsk",
  "name": "AID-3 QPSK/AWGN",
  "description": "Demonstrates QPSK modulation with LDPC coding over AWGN channel",
  "version": "1.0.0",
  "author": "Chimera Team",
  
  "graph": {
    "nodes": [
      {
        "id": "bitgen-1",
        "type": "bit_generator",
        "position": { "x": 100, "y": 200 },
        "parameters": {
          "length": 1024,
          "pattern": "random"
        }
      },
      {
        "id": "ldpc-enc-1",
        "type": "ldpc_encoder",
        "position": { "x": 300, "y": 200 },
        "parameters": {
          "code_rate": 0.5,
          "block_length": 1024
        }
      },
      {
        "id": "qpsk-mod-1",
        "type": "qpsk_modulator",
        "position": { "x": 500, "y": 200 },
        "parameters": {
          "symbol_rate": 1000,
          "pulse_shaping": "rrc"
        }
      },
      {
        "id": "awgn-1",
        "type": "awgn_channel",
        "position": { "x": 700, "y": 200 },
        "parameters": {
          "snr_db": 15,
          "link_loss_db": 0
        }
      },
      {
        "id": "qpsk-demod-1",
        "type": "qpsk_demodulator",
        "position": { "x": 900, "y": 200 },
        "parameters": {
          "algorithm": "soft"
        }
      },
      {
        "id": "ldpc-dec-1",
        "type": "ldpc_decoder",
        "position": { "x": 1100, "y": 200 },
        "parameters": {
          "max_iterations": 50
        }
      },
      {
        "id": "const-1",
        "type": "constellation_diagram",
        "position": { "x": 900, "y": 400 },
        "parameters": {
          "max_points": 1000,
          "color": "#ff6b6b"
        }
      },
      {
        "id": "ber-1",
        "type": "ber_calculator",
        "position": { "x": 1300, "y": 200 },
        "parameters": {}
      }
    ],
    "edges": [
      {
        "id": "e1",
        "source": { "nodeId": "bitgen-1", "portId": "bits" },
        "target": { "nodeId": "ldpc-enc-1", "portId": "bits" }
      },
      {
        "id": "e2",
        "source": { "nodeId": "ldpc-enc-1", "portId": "encoded" },
        "target": { "nodeId": "qpsk-mod-1", "portId": "bits" }
      },
      {
        "id": "e3",
        "source": { "nodeId": "qpsk-mod-1", "portId": "iq" },
        "target": { "nodeId": "awgn-1", "portId": "signal" }
      },
      {
        "id": "e4",
        "source": { "nodeId": "awgn-1", "portId": "signal" },
        "target": { "nodeId": "qpsk-demod-1", "portId": "iq" }
      },
      {
        "id": "e5",
        "source": { "nodeId": "qpsk-demod-1", "portId": "soft_bits" },
        "target": { "nodeId": "ldpc-dec-1", "portId": "soft_bits" }
      },
      {
        "id": "e6",
        "source": { "nodeId": "qpsk-demod-1", "portId": "symbols" },
        "target": { "nodeId": "const-1", "portId": "symbols" }
      },
      {
        "id": "e7",
        "source": { "nodeId": "bitgen-1", "portId": "bits" },
        "target": { "nodeId": "ber-1", "portId": "original" }
      },
      {
        "id": "e8",
        "source": { "nodeId": "ldpc-dec-1", "portId": "decoded" },
        "target": { "nodeId": "ber-1", "portId": "received" }
      }
    ]
  }
}
```

### Preset Management

```typescript
class PresetManager {
  async loadPreset(presetId: string): Promise<Graph> {
    const preset = await this.storage.load(presetId);
    return preset.graph;
  }
  
  async savePreset(name: string, graph: Graph): Promise<string> {
    const preset: Preset = {
      id: generateId(),
      name,
      description: '',
      version: '1.0.0',
      author: 'User',
      graph
    };
    
    await this.storage.save(preset);
    return preset.id;
  }
  
  async clonePreset(presetId: string, newName: string): Promise<Preset> {
    const original = await this.storage.load(presetId);
    const cloned = {
      ...original,
      id: generateId(),
      name: newName,
      description: `Cloned from ${original.name}`
    };
    
    await this.storage.save(cloned);
    return cloned;
  }
}
```

---

## 🛠️ Implementation Stack

### Frontend (React)

#### Graph Editor Library
Use **React Flow** (or similar) for the node graph editor:

```bash
npm install reactflow
```

```typescript
import ReactFlow, { Node, Edge, Connection } from 'reactflow';
import 'reactflow/dist/style.css';

const GraphEditor = () => {
  const [nodes, setNodes] = useState<Node[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);
  
  const onConnect = (connection: Connection) => {
    setEdges(edges => [...edges, connection]);
  };
  
  return (
    <ReactFlow
      nodes={nodes}
      edges={edges}
      onConnect={onConnect}
      nodeTypes={customNodeTypes}
    />
  );
};
```

#### Custom Node Components

```typescript
// Custom node rendering
const QPSKModulatorNode = ({ data, selected }: NodeProps) => {
  return (
    <div className={`node ${selected ? 'selected' : ''}`}>
      <div className="node-header">
        <span className="node-icon">📡</span>
        <span className="node-title">QPSK Modulator</span>
      </div>
      
      <Handle type="target" position="left" id="bits" />
      
      <div className="node-body">
        <div className="parameter">
          Symbol Rate: {data.parameters.symbol_rate} Hz
        </div>
      </div>
      
      <Handle type="source" position="right" id="iq" />
    </div>
  );
};
```

### Backend (Rust/WASM)

#### Node Registry

```rust
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NodeDefinition {
    pub id: String,
    pub name: String,
    pub category: NodeCategory,
    pub inputs: Vec<PortDefinition>,
    pub outputs: Vec<PortDefinition>,
    pub parameters: Vec<ParameterDefinition>,
}

#[wasm_bindgen]
pub struct NodeRegistry {
    nodes: HashMap<String, Box<dyn Node>>,
}

#[wasm_bindgen]
impl NodeRegistry {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut registry = Self {
            nodes: HashMap::new(),
        };
        
        // Register built-in nodes
        registry.register(Box::new(BitGeneratorNode::new()));
        registry.register(Box::new(LDPCEncoderNode::new()));
        registry.register(Box::new(QPSKModulatorNode::new()));
        registry.register(Box::new(AWGNChannelNode::new()));
        // ... etc
        
        registry
    }
    
    pub fn register(&mut self, node: Box<dyn Node>) {
        self.nodes.insert(node.id().to_string(), node);
    }
    
    pub fn get_definition(&self, node_id: &str) -> JsValue {
        if let Some(node) = self.nodes.get(node_id) {
            serde_wasm_bindgen::to_value(&node.definition()).unwrap()
        } else {
            JsValue::NULL
        }
    }
    
    pub fn list_nodes(&self) -> JsValue {
        let definitions: Vec<_> = self.nodes.values()
            .map(|n| n.definition())
            .collect();
        serde_wasm_bindgen::to_value(&definitions).unwrap()
    }
}
```

#### Node Trait

```rust
pub trait Node: Send + Sync {
    fn id(&self) -> &str;
    fn definition(&self) -> NodeDefinition;
    fn execute(&self, inputs: Vec<DataBuffer>, params: JsValue) -> Result<Vec<DataBuffer>, JsValue>;
}

// Example: QPSK Modulator
pub struct QPSKModulatorNode;

impl Node for QPSKModulatorNode {
    fn id(&self) -> &str {
        "qpsk_modulator"
    }
    
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            id: "qpsk_modulator".to_string(),
            name: "QPSK Modulator".to_string(),
            category: NodeCategory::Processing,
            inputs: vec![
                PortDefinition {
                    id: "bits".to_string(),
                    name: "Bits".to_string(),
                    data_type: DataType::BitStream,
                    optional: false,
                }
            ],
            outputs: vec![
                PortDefinition {
                    id: "iq".to_string(),
                    name: "I/Q Samples".to_string(),
                    data_type: DataType::IQData,
                    optional: false,
                }
            ],
            parameters: vec![
                ParameterDefinition {
                    name: "symbol_rate".to_string(),
                    param_type: ParameterType::Number,
                    default: serde_json::json!(1000),
                    min: Some(100.0),
                    max: Some(100000.0),
                    unit: Some("Hz".to_string()),
                }
            ],
        }
    }
    
    fn execute(&self, inputs: Vec<DataBuffer>, params: JsValue) -> Result<Vec<DataBuffer>, JsValue> {
        // Extract bits from input
        let bits = &inputs[0].as_bitstream()?;
        
        // Parse parameters
        let config: ModulatorConfig = serde_wasm_bindgen::from_value(params)?;
        
        // Perform QPSK modulation
        let iq_data = qpsk_modulate(bits, &config);
        
        // Return output
        Ok(vec![DataBuffer::IQData(iq_data)])
    }
}
```

#### Graph Executor

```rust
#[wasm_bindgen]
pub struct GraphExecutor {
    registry: NodeRegistry,
}

#[wasm_bindgen]
impl GraphExecutor {
    #[wasm_bindgen(constructor)]
    pub fn new(registry: NodeRegistry) -> Self {
        Self { registry }
    }
    
    pub async fn execute(&self, graph: JsValue) -> Result<JsValue, JsValue> {
        let graph: Graph = serde_wasm_bindgen::from_value(graph)?;
        
        // Validate graph
        self.validate(&graph)?;
        
        // Topological sort
        let sorted_nodes = self.topological_sort(&graph)?;
        
        // Execute
        let mut context = ExecutionContext::new();
        
        for node in sorted_nodes {
            let node_def = self.registry.get_node(&node.node_type)?;
            let inputs = self.get_node_inputs(&node, &graph, &context)?;
            let outputs = node_def.execute(inputs, node.parameters)?;
            context.set_outputs(&node.id, outputs);
        }
        
        Ok(serde_wasm_bindgen::to_value(&context.get_results())?)
    }
}
```

---

## 📊 Data Flow & Types

### Data Buffers

```rust
#[derive(Serialize, Deserialize, Clone)]
pub enum DataBuffer {
    BitStream(Vec<u8>),
    IQData { i: Vec<f32>, q: Vec<f32> },
    AudioBuffer(Vec<f32>),
    Spectrum { frequencies: Vec<f32>, magnitudes: Vec<f32> },
    Metrics(HashMap<String, f64>),
    Message(String),
}

impl DataBuffer {
    pub fn data_type(&self) -> DataType {
        match self {
            Self::BitStream(_) => DataType::BitStream,
            Self::IQData { .. } => DataType::IQData,
            Self::AudioBuffer(_) => DataType::AudioBuffer,
            Self::Spectrum { .. } => DataType::Spectrum,
            Self::Metrics(_) => DataType::Metrics,
            Self::Message(_) => DataType::Message,
        }
    }
}
```

### Type Checking

```typescript
class TypeChecker {
  canConnect(sourcePort: PortDefinition, targetPort: PortDefinition): boolean {
    // Exact type match
    if (sourcePort.type === targetPort.type) return true;
    
    // Allow "Any" type to connect anywhere
    if (sourcePort.type === DataType.Any || targetPort.type === DataType.Any) {
      return true;
    }
    
    // Check type compatibility (e.g., AudioBuffer can convert to IQData)
    return this.areTypesCompatible(sourcePort.type, targetPort.type);
  }
  
  private areTypesCompatible(source: DataType, target: DataType): boolean {
    const compatibilityMatrix: Record<DataType, DataType[]> = {
      [DataType.AudioBuffer]: [DataType.IQData],  // Audio can be treated as IQ
      [DataType.IQData]: [DataType.AudioBuffer],  // IQ can be treated as audio
      // ... other conversions
    };
    
    return compatibilityMatrix[source]?.includes(target) ?? false;
  }
}
```

---

## 🎯 Updated PR Plan

### Modified PRs

#### **PR #45 - Project Setup** (No change)
Infrastructure setup remains the same.

#### **PR #46 - UI Component Library** (Enhanced)
**Additional:**
- Node component (with ports, parameters)
- Port component (input/output handles)
- Edge component (connection lines)
- Canvas component (pan/zoom/grid)

#### **PR #47 - WASM Service Layer** (MAJOR CHANGE)
**Replace with Node Graph Engine:**
- Node trait and registry
- Graph executor
- Data buffer types
- Type checking
- Built-in nodes (encoder, modulator, channel, etc.)

#### **PR #48 - Chart Components** (Change to Analysis Nodes)
**Replace with Analysis/Sink Nodes:**
- Constellation diagram node
- FFT analyzer node
- BER calculator node
- Audio player node
- Export node

#### **PR #49 - Control Components** (Change to Graph Editor)
**Replace with Graph Editor Components:**
- Node palette
- Graph canvas (React Flow integration)
- Node inspector
- Parameter editors

#### **PR #51 - State Management** (Enhanced)
**Additional:**
- Graph state (nodes, edges)
- Execution state
- Selected node state
- Preset management

#### **PR #52 - Main App Integration** (MAJOR CHANGE)
**Replace with Graph Editor App:**
- Full graph editor UI
- Preset loading
- Graph execution
- Results visualization

### New PRs

#### **NEW: PR #64 - Node Graph Core**
**Phase:** 1 - Foundation  
**Effort:** 5-6 days  
**Priority:** Critical

**Deliverables:**
- Node trait and registry (Rust)
- Graph data structures
- Type system
- Graph validator
- Graph executor
- Topological sort

#### **NEW: PR #65 - Built-in Processing Nodes**
**Phase:** 2 - Core  
**Effort:** 6-7 days  
**Priority:** Critical

**Deliverables:**
- Source nodes (BitGenerator, AudioLoader, NoiseGen)
- Processing nodes (LDPC Encoder/Decoder, QPSK Mod/Demod, AWGN Channel)
- Utility nodes (Delay, Multiply, Add)

#### **NEW: PR #66 - Graph Editor UI**
**Phase:** 3 - Integration  
**Effort:** 6-7 days  
**Priority:** Critical

**Deliverables:**
- React Flow integration
- Node palette
- Custom node components
- Connection handling
- Node inspector

---

## 🚀 Migration Strategy

### Phase 1: Prove the Concept
1. Build node graph engine (Rust)
2. Implement 5-10 basic nodes
3. Create simple graph editor UI
4. Demonstrate QPSK/AWGN as a graph

### Phase 2: Feature Parity
5. Implement all nodes for current features
6. Build full graph editor UI
7. Preset system as saved graphs
8. Analysis nodes with visualizations

### Phase 3: Platform Features
9. Custom node plugin system
10. Community preset sharing
11. Advanced analysis nodes
12. Performance optimization

---

## 🎓 Success Criteria

### Core Functionality
- ✅ Can build QPSK/AWGN demo as a graph
- ✅ Visual graph editor works smoothly
- ✅ Execution produces correct results
- ✅ Analysis nodes show live data

### Extensibility
- ✅ Can add new node in <4 hours
- ✅ Community can create custom nodes
- ✅ Presets are shareable JSON files

### Performance
- ✅ Graph execution <100ms for typical graphs
- ✅ UI responsive (60fps) during editing
- ✅ Can handle 50+ nodes in a graph

---

**This transforms Chimera from a QPSK demo into a visual DSP prototyping platform!** 🚀
