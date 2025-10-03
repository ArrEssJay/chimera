// Rust Contract Definitions for Chimera Node Graph System
// ⚠️ LOCKED CONTRACT - DO NOT MODIFY WITHOUT TEAM APPROVAL
// All agents must implement against these traits and types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// Core trait that all DSP processing nodes must implement
/// 
/// This trait is thread-safe and can be sent across threads.
/// All implementations must handle errors gracefully using Result types.
pub trait Node: Send + Sync {
    /// Returns the unique identifier for this node instance
    fn id(&self) -> &str;
    
    /// Returns the node's metadata and interface definition
    fn definition(&self) -> NodeDefinition;
    
    /// Execute the node's processing logic
    /// 
    /// # Arguments
    /// * `inputs` - Input data buffers (one per input port)
    /// * `params` - Configuration parameters as JSON
    /// 
    /// # Returns
    /// * `Ok(Vec<DataBuffer>)` - Output data buffers (one per output port)
    /// * `Err(JsValue)` - Error message for display to user
    /// 
    /// # Panics
    /// **MUST NOT PANIC** - All errors must be returned as Results
    fn execute(
        &self,
        inputs: Vec<DataBuffer>,
        params: JsValue,
    ) -> Result<Vec<DataBuffer>, JsValue>;
}

/// Node metadata and interface definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct NodeDefinition {
    /// Unique node type identifier (e.g., "bit_generator")
    pub id: String,
    
    /// Human-readable name (e.g., "Bit Generator")
    pub name: String,
    
    /// Category for palette organization
    pub category: NodeCategory,
    
    /// Description of what this node does
    pub description: String,
    
    /// Input port definitions
    pub inputs: Vec<PortDefinition>,
    
    /// Output port definitions
    pub outputs: Vec<PortDefinition>,
    
    /// Parameter definitions
    pub parameters: Vec<ParameterDefinition>,
}

/// Node category for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum NodeCategory {
    Source,
    Processing,
    Analysis,
    Sink,
}

/// Port definition for inputs/outputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortDefinition {
    pub id: String,
    pub name: String,
    pub data_type: DataType,
    pub description: Option<String>,
}

/// Supported data types for node connections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum DataType {
    BitStream,
    IQData,
    AudioSamples,
    Metadata,
}

/// Parameter definition for node configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDefinition {
    pub id: String,
    pub name: String,
    pub param_type: ParameterType,
    pub default_value: ParameterValue,
    pub description: Option<String>,
}

/// Parameter types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterType {
    Number { min: Option<f64>, max: Option<f64> },
    String,
    Boolean,
    Enum { options: Vec<String> },
}

/// Parameter values
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParameterValue {
    Number(f64),
    String(String),
    Boolean(bool),
}

/// Data buffer enum for passing data between nodes
/// 
/// This is the core data structure for node graph execution.
/// All data flowing through the graph must be one of these types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataBuffer {
    /// Sequence of bits (true = 1, false = 0)
    BitStream(Vec<bool>),
    
    /// Complex IQ samples for RF signals
    IQData(Vec<IQSample>),
    
    /// Audio samples (mono, normalized -1.0 to 1.0)
    AudioSamples(Vec<f32>),
    
    /// Metadata key-value pairs
    Metadata(HashMap<String, String>),
}

/// Complex IQ sample
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct IQSample {
    pub i: f32,
    pub q: f32,
}

impl IQSample {
    pub fn new(i: f32, q: f32) -> Self {
        Self { i, q }
    }
    
    pub fn magnitude(&self) -> f32 {
        (self.i * self.i + self.q * self.q).sqrt()
    }
    
    pub fn phase(&self) -> f32 {
        self.q.atan2(self.i)
    }
}

/// Graph structure containing nodes and connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    /// Map of node ID to node instance
    pub nodes: HashMap<String, Box<dyn Node>>,
    
    /// List of connections between nodes
    pub edges: Vec<Edge>,
}

/// Connection between two nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    /// ID of this connection
    pub id: String,
    
    /// Source node ID
    pub from_node: String,
    
    /// Source port index
    pub from_port: usize,
    
    /// Target node ID
    pub to_node: String,
    
    /// Target port index
    pub to_port: usize,
}

/// Graph validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
}

/// Validation error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub error_type: ValidationErrorType,
    pub node_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationErrorType {
    Cycle,
    TypeMismatch,
    Disconnected,
    InvalidParam,
}

/// Graph execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphResult {
    pub success: bool,
    pub outputs: Vec<OutputData>,
    pub error: Option<String>,
    pub execution_time_ms: u64,
}

/// Output data from a node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputData {
    pub node_id: String,
    pub port_id: String,
    pub data: DataBuffer,
}

/// Graph executor trait
/// 
/// Responsible for executing nodes in topological order
pub trait GraphExecutor {
    /// Execute the entire graph
    fn execute(&self, graph: &Graph) -> Result<GraphResult, String>;
    
    /// Validate graph structure before execution
    fn validate(&self, graph: &Graph) -> ValidationResult;
}

/// Node registry for dynamic node creation
pub trait NodeRegistry {
    /// Register a node type
    fn register(&mut self, node_type: String, factory: Box<dyn NodeFactory>);
    
    /// Create a node instance by type
    fn create_node(&self, node_type: &str, id: String) -> Result<Box<dyn Node>, String>;
    
    /// Get all registered node types
    fn available_nodes(&self) -> Vec<NodeDefinition>;
}

/// Factory for creating node instances
pub trait NodeFactory: Send + Sync {
    fn create(&self, id: String) -> Box<dyn Node>;
    fn definition(&self) -> NodeDefinition;
}
