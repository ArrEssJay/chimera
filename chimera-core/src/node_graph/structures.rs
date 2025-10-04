//! Graph data structures (nodes, edges, graph)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::JsValue;

use super::data_buffer::{DataBuffer, DataType};

/// Node category for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

impl PortDefinition {
    pub fn new(id: impl Into<String>, name: impl Into<String>, data_type: DataType) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            data_type,
            description: None,
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
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

/// Parameter definition for node configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDefinition {
    pub id: String,
    pub name: String,
    pub param_type: ParameterType,
    pub default_value: ParameterValue,
    pub description: Option<String>,
}

impl ParameterDefinition {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        param_type: ParameterType,
        default_value: ParameterValue,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            param_type,
            default_value,
            description: None,
        }
    }
}

/// Node metadata and interface definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDefinition {
    pub id: String,
    pub name: String,
    pub category: NodeCategory,
    pub description: String,
    pub inputs: Vec<PortDefinition>,
    pub outputs: Vec<PortDefinition>,
    pub parameters: Vec<ParameterDefinition>,
}

impl NodeDefinition {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        category: NodeCategory,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            category,
            description: description.into(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            parameters: Vec::new(),
        }
    }

    pub fn with_inputs(mut self, inputs: Vec<PortDefinition>) -> Self {
        self.inputs = inputs;
        self
    }

    pub fn with_outputs(mut self, outputs: Vec<PortDefinition>) -> Self {
        self.outputs = outputs;
        self
    }

    pub fn with_parameters(mut self, parameters: Vec<ParameterDefinition>) -> Self {
        self.parameters = parameters;
        self
    }
}

/// Core trait that all DSP processing nodes must implement
pub trait Node: Send + Sync {
    /// Returns the unique identifier for this node instance
    fn id(&self) -> &str;

    /// Returns the node's metadata and interface definition
    fn definition(&self) -> NodeDefinition;

    /// Execute the node's processing logic
    fn execute(&self, inputs: Vec<DataBuffer>, params: JsValue)
        -> Result<Vec<DataBuffer>, JsValue>;
}

/// A node instance in the graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInstance {
    pub id: String,
    pub node_type: String,
    pub position: Position,
    pub parameters: HashMap<String, ParameterValue>,
}

impl NodeInstance {
    pub fn new(id: impl Into<String>, node_type: impl Into<String>, position: Position) -> Self {
        Self {
            id: id.into(),
            node_type: node_type.into(),
            position,
            parameters: HashMap::new(),
        }
    }

    pub fn with_parameter(mut self, key: impl Into<String>, value: ParameterValue) -> Self {
        self.parameters.insert(key.into(), value);
        self
    }
}

/// Position in 2D space
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Connection between two nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: String,
    pub from_node: String,
    pub from_port: usize,
    pub to_node: String,
    pub to_port: usize,
}

impl Edge {
    pub fn new(
        id: impl Into<String>,
        from_node: impl Into<String>,
        from_port: usize,
        to_node: impl Into<String>,
        to_port: usize,
    ) -> Self {
        Self {
            id: id.into(),
            from_node: from_node.into(),
            from_port,
            to_node: to_node.into(),
            to_port,
        }
    }
}

/// Graph structure containing nodes and connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: Vec<NodeInstance>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: NodeInstance) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn get_node(&self, id: &str) -> Option<&NodeInstance> {
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn get_node_mut(&mut self, id: &str) -> Option<&mut NodeInstance> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let pos = Position::new(100.0, 200.0);
        assert_eq!(pos.x, 100.0);
        assert_eq!(pos.y, 200.0);
    }

    #[test]
    fn test_port_definition() {
        let port = PortDefinition::new("input0", "Input", DataType::BitStream)
            .with_description("Test input port");
        assert_eq!(port.id, "input0");
        assert_eq!(port.name, "Input");
        assert_eq!(port.data_type, DataType::BitStream);
        assert_eq!(port.description, Some("Test input port".to_string()));
    }

    #[test]
    fn test_node_definition() {
        let def = NodeDefinition::new(
            "test_node",
            "Test Node",
            NodeCategory::Processing,
            "A test node",
        )
        .with_inputs(vec![PortDefinition::new(
            "in0",
            "Input",
            DataType::BitStream,
        )])
        .with_outputs(vec![PortDefinition::new(
            "out0",
            "Output",
            DataType::IQData,
        )]);

        assert_eq!(def.id, "test_node");
        assert_eq!(def.inputs.len(), 1);
        assert_eq!(def.outputs.len(), 1);
    }

    #[test]
    fn test_node_instance() {
        let node = NodeInstance::new("node1", "bit_generator", Position::new(0.0, 0.0))
            .with_parameter("count", ParameterValue::Number(100.0));

        assert_eq!(node.id, "node1");
        assert_eq!(node.node_type, "bit_generator");
        assert_eq!(node.parameters.len(), 1);
    }

    #[test]
    fn test_edge() {
        let edge = Edge::new("edge1", "node1", 0, "node2", 0);
        assert_eq!(edge.id, "edge1");
        assert_eq!(edge.from_node, "node1");
        assert_eq!(edge.from_port, 0);
        assert_eq!(edge.to_node, "node2");
        assert_eq!(edge.to_port, 0);
    }

    #[test]
    fn test_graph() {
        let mut graph = Graph::new();

        let node1 = NodeInstance::new("n1", "test", Position::new(0.0, 0.0));
        let node2 = NodeInstance::new("n2", "test", Position::new(100.0, 0.0));

        graph.add_node(node1);
        graph.add_node(node2);

        assert_eq!(graph.nodes.len(), 2);
        assert!(graph.get_node("n1").is_some());
        assert!(graph.get_node("n3").is_none());
    }

    #[test]
    fn test_graph_edges() {
        let mut graph = Graph::new();

        graph.add_edge(Edge::new("e1", "n1", 0, "n2", 0));
        graph.add_edge(Edge::new("e2", "n2", 0, "n3", 0));

        assert_eq!(graph.edges.len(), 2);
    }

    #[test]
    fn test_parameter_types() {
        let num_param = ParameterDefinition::new(
            "count",
            "Count",
            ParameterType::Number {
                min: Some(0.0),
                max: Some(100.0),
            },
            ParameterValue::Number(50.0),
        );
        assert_eq!(num_param.id, "count");

        let bool_param = ParameterDefinition::new(
            "enabled",
            "Enabled",
            ParameterType::Boolean,
            ParameterValue::Boolean(true),
        );
        assert!(matches!(
            bool_param.default_value,
            ParameterValue::Boolean(true)
        ));
    }
}
