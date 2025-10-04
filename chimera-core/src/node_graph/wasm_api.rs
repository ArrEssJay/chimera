//! WASM bindings for node graph system

use wasm_bindgen::prelude::*;

use super::{
    GraphExecutorImpl, NodeRegistryImpl,
};

/// WASM-exposed node registry
#[wasm_bindgen]
pub struct WasmNodeRegistry {
    inner: NodeRegistryImpl,
}

impl Default for WasmNodeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl WasmNodeRegistry {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: NodeRegistryImpl::new(),
        }
    }
    
    /// Get all available node types as JSON
    #[wasm_bindgen(js_name = availableNodes)]
    pub fn available_nodes(&self) -> Result<JsValue, JsValue> {
        let nodes = self.inner.available_nodes();
        serde_wasm_bindgen::to_value(&nodes)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))
    }
    
    /// Get definition for a specific node type
    #[wasm_bindgen(js_name = getDefinition)]
    pub fn get_definition(&self, node_type: &str) -> Result<JsValue, JsValue> {
        match self.inner.get_definition(node_type) {
            Some(def) => serde_wasm_bindgen::to_value(&def)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e))),
            None => Err(JsValue::from_str(&format!("Node type '{}' not found", node_type))),
        }
    }
}

/// WASM-exposed graph executor
#[wasm_bindgen]
pub struct WasmGraphExecutor {
    inner: GraphExecutorImpl,
}

#[wasm_bindgen]
impl WasmGraphExecutor {
    #[wasm_bindgen(constructor)]
    pub fn new(registry: &WasmNodeRegistry) -> Self {
        Self {
            inner: GraphExecutorImpl::new(registry.inner.clone()),
        }
    }
    
    /// Validate a graph
    #[wasm_bindgen(js_name = validateGraph)]
    pub fn validate_graph(&self, graph_json: JsValue) -> Result<JsValue, JsValue> {
        let graph: crate::node_graph::Graph = serde_wasm_bindgen::from_value(graph_json)
            .map_err(|e| JsValue::from_str(&format!("Deserialization error: {:?}", e)))?;
        
        let result = self.inner.validate(&graph);
        serde_wasm_bindgen::to_value(&result)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))
    }
    
    /// Execute a graph
    #[wasm_bindgen(js_name = executeGraph)]
    pub fn execute_graph(&self, graph_json: JsValue) -> Result<JsValue, JsValue> {
        let graph: crate::node_graph::Graph = serde_wasm_bindgen::from_value(graph_json)
            .map_err(|e| JsValue::from_str(&format!("Deserialization error: {:?}", e)))?;
        
        let result = self.inner.execute(&graph)
            .map_err(|e| JsValue::from_str(&e))?;
        
        serde_wasm_bindgen::to_value(&result)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))
    }
}

// Re-export types for TypeScript bindings
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export interface NodeDefinition {
    id: string;
    name: string;
    category: 'Source' | 'Processing' | 'Analysis' | 'Sink';
    description: string;
    inputs: PortDefinition[];
    outputs: PortDefinition[];
    parameters: ParameterDefinition[];
}

export interface PortDefinition {
    id: string;
    name: string;
    data_type: 'BitStream' | 'IQData' | 'AudioSamples' | 'Metadata';
    description?: string;
}

export interface ParameterDefinition {
    id: string;
    name: string;
    param_type: ParameterType;
    default_value: ParameterValue;
    description?: string;
}

export type ParameterType = 
    | { Number: { min?: number; max?: number } }
    | 'String'
    | 'Boolean'
    | { Enum: { options: string[] } };

export type ParameterValue = 
    | { Number: number }
    | { String: string }
    | { Boolean: boolean };

export interface Graph {
    nodes: NodeInstance[];
    edges: Edge[];
}

export interface NodeInstance {
    id: string;
    node_type: string;
    position: Position;
    parameters: Record<string, ParameterValue>;
}

export interface Position {
    x: number;
    y: number;
}

export interface Edge {
    id: string;
    from_node: string;
    from_port: number;
    to_node: string;
    to_port: number;
}

export interface ValidationResult {
    valid: boolean;
    errors: ValidationError[];
}

export interface ValidationError {
    error_type: 'Cycle' | 'TypeMismatch' | 'Disconnected' | 'InvalidParam';
    node_id?: string;
    message: string;
}

export interface GraphResult {
    success: boolean;
    outputs: OutputData[];
    error?: string;
    execution_time_ms: number;
}

export interface OutputData {
    node_id: string;
    port_id: string;
    data: DataBuffer;
}

export type DataBuffer = 
    | { BitStream: boolean[] }
    | { IQData: IQSample[] }
    | { AudioSamples: number[] }
    | { Metadata: Record<string, string> };

export interface IQSample {
    i: number;
    q: number;
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_arch = "wasm32")]
    fn test_wasm_registry_creation() {
        let registry = WasmNodeRegistry::new();
        let result = registry.available_nodes();
        assert!(result.is_ok());
    }
}
