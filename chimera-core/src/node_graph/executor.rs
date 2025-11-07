//! Graph executor with topological sort and sequential execution

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

use super::data_buffer::DataBuffer;
use super::registry::NodeRegistryImpl;
use super::structures::{Graph, NodeInstance};
use super::validator::{GraphValidator, ValidationResult};

/// Output data from a node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputData {
    pub node_id: String,
    pub port_id: String,
    pub data: DataBuffer,
}

/// Graph execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphResult {
    pub success: bool,
    pub outputs: Vec<OutputData>,
    pub error: Option<String>,
    pub execution_time_ms: u64,
}

impl GraphResult {
    pub fn success(outputs: Vec<OutputData>, execution_time_ms: u64) -> Self {
        Self {
            success: true,
            outputs,
            error: None,
            execution_time_ms,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            outputs: Vec::new(),
            error: Some(message),
            execution_time_ms: 0,
        }
    }
}

/// Execution context for storing intermediate results
struct ExecutionContext {
    outputs: HashMap<String, Vec<DataBuffer>>,
}

impl ExecutionContext {
    fn new() -> Self {
        Self {
            outputs: HashMap::new(),
        }
    }

    fn set_outputs(&mut self, node_id: String, outputs: Vec<DataBuffer>) {
        self.outputs.insert(node_id, outputs);
    }

    fn get_output(&self, node_id: &str, port: usize) -> Option<&DataBuffer> {
        self.outputs.get(node_id)?.get(port)
    }
}

/// Graph executor
pub struct GraphExecutorImpl {
    registry: NodeRegistryImpl,
}

impl GraphExecutorImpl {
    pub fn new(registry: NodeRegistryImpl) -> Self {
        Self { registry }
    }

    /// Validate graph structure before execution
    pub fn validate(&self, graph: &Graph) -> ValidationResult {
        // Build node definitions map
        let mut node_defs = HashMap::new();
        for node in &graph.nodes {
            if let Some(def) = self.registry.get_definition(&node.node_type) {
                node_defs.insert(node.node_type.clone(), def);
            }
        }

        let validator = GraphValidator::new(node_defs);
        validator.validate(graph)
    }

    /// Execute the entire graph
    pub fn execute(&self, graph: &Graph) -> Result<GraphResult, String> {
        let start_time = Instant::now();

        // Validate graph
        let validation = self.validate(graph);
        if !validation.valid {
            let error_msg = validation
                .errors
                .iter()
                .map(|e| e.message.clone())
                .collect::<Vec<_>>()
                .join("; ");
            return Ok(GraphResult::error(format!(
                "Validation failed: {}",
                error_msg
            )));
        }

        // Topological sort
        let sorted_nodes = match self.topological_sort(graph) {
            Ok(nodes) => nodes,
            Err(e) => return Ok(GraphResult::error(e)),
        };

        // Execute nodes in order
        let mut context = ExecutionContext::new();

        for node_instance in sorted_nodes {
            // Get node inputs from connected edges
            let inputs = match self.get_node_inputs(&node_instance, graph, &context) {
                Ok(inputs) => inputs,
                Err(e) => return Ok(GraphResult::error(e)),
            };

            // Create node instance
            let node = match self
                .registry
                .create_node(&node_instance.node_type, node_instance.id.clone())
            {
                Ok(n) => n,
                Err(e) => return Ok(GraphResult::error(format!("Failed to create node: {}", e))),
            };

            // Convert parameters to JsValue
            let params = match serde_wasm_bindgen::to_value(&node_instance.parameters) {
                Ok(v) => v,
                Err(e) => {
                    return Ok(GraphResult::error(format!(
                        "Failed to serialize parameters: {:?}",
                        e
                    )))
                }
            };

            // Execute node
            let outputs = match node.execute(inputs, params) {
                Ok(o) => o,
                Err(e) => {
                    let error_msg = if let Some(s) = e.as_string() {
                        s
                    } else {
                        format!("Node execution failed: {:?}", e)
                    };
                    return Ok(GraphResult::error(format!(
                        "Node '{}': {}",
                        node_instance.id, error_msg
                    )));
                }
            };

            // Store outputs
            context.set_outputs(node_instance.id.clone(), outputs);
        }

        // Collect all outputs
        let mut all_outputs = Vec::new();
        for node in &graph.nodes {
            if let Some(outputs) = context.outputs.get(&node.id) {
                for (port_idx, data) in outputs.iter().enumerate() {
                    all_outputs.push(OutputData {
                        node_id: node.id.clone(),
                        port_id: port_idx.to_string(),
                        data: data.clone(),
                    });
                }
            }
        }

        let execution_time = start_time.elapsed().as_millis() as u64;
        Ok(GraphResult::success(all_outputs, execution_time))
    }

    /// Get inputs for a node from connected edges
    fn get_node_inputs(
        &self,
        node: &NodeInstance,
        graph: &Graph,
        context: &ExecutionContext,
    ) -> Result<Vec<DataBuffer>, String> {
        // Get node definition to know how many inputs to expect
        let def = self
            .registry
            .get_definition(&node.node_type)
            .ok_or_else(|| format!("Node type '{}' not found", node.node_type))?;

        let mut inputs = vec![None; def.inputs.len()];

        // Find all edges connected to this node's inputs
        for edge in &graph.edges {
            if edge.to_node == node.id {
                if edge.to_port >= inputs.len() {
                    return Err(format!(
                        "Invalid input port {} for node '{}'",
                        edge.to_port, node.id
                    ));
                }

                let data = context
                    .get_output(&edge.from_node, edge.from_port)
                    .ok_or_else(|| {
                        format!(
                            "Output not found: node '{}' port {}",
                            edge.from_node, edge.from_port
                        )
                    })?;

                inputs[edge.to_port] = Some(data.clone());
            }
        }

        // Convert to Vec<DataBuffer>, checking that all required inputs are present
        let mut result = Vec::new();
        for (idx, input) in inputs.into_iter().enumerate() {
            result.push(
                input.ok_or_else(|| {
                    format!("Missing input at port {} for node '{}'", idx, node.id)
                })?,
            );
        }

        Ok(result)
    }

    /// Perform topological sort using Kahn's algorithm
    fn topological_sort(&self, graph: &Graph) -> Result<Vec<NodeInstance>, String> {
        // Build adjacency list and in-degree map
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();
        let mut node_map: HashMap<String, NodeInstance> = HashMap::new();

        // Initialize
        for node in &graph.nodes {
            in_degree.insert(node.id.clone(), 0);
            adj_list.insert(node.id.clone(), Vec::new());
            node_map.insert(node.id.clone(), node.clone());
        }

        // Build graph
        for edge in &graph.edges {
            adj_list
                .entry(edge.from_node.clone())
                .or_default()
                .push(edge.to_node.clone());

            *in_degree.entry(edge.to_node.clone()).or_insert(0) += 1;
        }

        // Find all nodes with in-degree 0 (source nodes)
        let mut queue: VecDeque<String> = VecDeque::new();
        for (node_id, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node_id.clone());
            }
        }

        // Process nodes
        let mut sorted = Vec::new();
        let mut processed = HashSet::new();

        while let Some(node_id) = queue.pop_front() {
            if processed.contains(&node_id) {
                continue;
            }

            processed.insert(node_id.clone());

            if let Some(node) = node_map.get(&node_id) {
                sorted.push(node.clone());
            }

            // Decrease in-degree for neighbors
            if let Some(neighbors) = adj_list.get(&node_id) {
                for neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }

        // Check if all nodes were processed (no cycles)
        if sorted.len() != graph.nodes.len() {
            return Err("Graph contains a cycle or unreachable nodes".to_string());
        }

        Ok(sorted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_graph::{DataType, Edge, NodeCategory, NodeFactory, PortDefinition, Position};
    use wasm_bindgen::JsValue;

    // Mock passthrough node for testing
    struct PassthroughNode {
        id: String,
    }

    impl crate::node_graph::structures::Node for PassthroughNode {
        fn id(&self) -> &str {
            &self.id
        }

        fn definition(&self) -> crate::node_graph::structures::NodeDefinition {
            crate::node_graph::structures::NodeDefinition::new(
                "passthrough",
                "Passthrough",
                NodeCategory::Processing,
                "Passes data through",
            )
            .with_inputs(vec![PortDefinition::new(
                "in",
                "Input",
                DataType::BitStream,
            )])
            .with_outputs(vec![PortDefinition::new(
                "out",
                "Output",
                DataType::BitStream,
            )])
        }

        fn execute(
            &self,
            inputs: Vec<DataBuffer>,
            _params: JsValue,
        ) -> Result<Vec<DataBuffer>, JsValue> {
            Ok(inputs)
        }
    }

    struct PassthroughFactory;

    impl NodeFactory for PassthroughFactory {
        fn create(&self, id: String) -> Box<dyn crate::node_graph::structures::Node> {
            Box::new(PassthroughNode { id })
        }

        fn definition(&self) -> crate::node_graph::structures::NodeDefinition {
            crate::node_graph::structures::NodeDefinition::new(
                "passthrough",
                "Passthrough",
                NodeCategory::Processing,
                "Passes data through",
            )
            .with_inputs(vec![PortDefinition::new(
                "in",
                "Input",
                DataType::BitStream,
            )])
            .with_outputs(vec![PortDefinition::new(
                "out",
                "Output",
                DataType::BitStream,
            )])
        }
    }

    // Source node (no inputs)
    struct SourceNode {
        id: String,
    }

    impl crate::node_graph::structures::Node for SourceNode {
        fn id(&self) -> &str {
            &self.id
        }

        fn definition(&self) -> crate::node_graph::structures::NodeDefinition {
            crate::node_graph::structures::NodeDefinition::new(
                "source",
                "Source",
                NodeCategory::Source,
                "Generates data",
            )
            .with_outputs(vec![PortDefinition::new(
                "out",
                "Output",
                DataType::BitStream,
            )])
        }

        fn execute(
            &self,
            _inputs: Vec<DataBuffer>,
            _params: JsValue,
        ) -> Result<Vec<DataBuffer>, JsValue> {
            Ok(vec![DataBuffer::BitStream(vec![true, false, true])])
        }
    }

    struct SourceFactory;

    impl NodeFactory for SourceFactory {
        fn create(&self, id: String) -> Box<dyn crate::node_graph::structures::Node> {
            Box::new(SourceNode { id })
        }

        fn definition(&self) -> crate::node_graph::structures::NodeDefinition {
            crate::node_graph::structures::NodeDefinition::new(
                "source",
                "Source",
                NodeCategory::Source,
                "Generates data",
            )
            .with_outputs(vec![PortDefinition::new(
                "out",
                "Output",
                DataType::BitStream,
            )])
        }
    }

    #[test]
    fn test_topological_sort_simple_chain() {
        let registry = NodeRegistryImpl::new();
        registry.register("passthrough".to_string(), Box::new(PassthroughFactory));

        let executor = GraphExecutorImpl::new(registry);

        let mut graph = Graph::new();
        graph.add_node(NodeInstance::new(
            "n1",
            "passthrough",
            Position::new(0.0, 0.0),
        ));
        graph.add_node(NodeInstance::new(
            "n2",
            "passthrough",
            Position::new(100.0, 0.0),
        ));
        graph.add_edge(Edge::new("e1", "n1", 0, "n2", 0));

        let result = executor.topological_sort(&graph);
        assert!(result.is_ok());

        let sorted = result.unwrap();
        assert_eq!(sorted.len(), 2);
        assert_eq!(sorted[0].id, "n1");
        assert_eq!(sorted[1].id, "n2");
    }

    #[test]
    fn test_topological_sort_multiple_paths() {
        let registry = NodeRegistryImpl::new();
        registry.register("passthrough".to_string(), Box::new(PassthroughFactory));

        let executor = GraphExecutorImpl::new(registry);

        let mut graph = Graph::new();
        graph.add_node(NodeInstance::new(
            "n1",
            "passthrough",
            Position::new(0.0, 0.0),
        ));
        graph.add_node(NodeInstance::new(
            "n2",
            "passthrough",
            Position::new(100.0, 0.0),
        ));
        graph.add_node(NodeInstance::new(
            "n3",
            "passthrough",
            Position::new(100.0, 100.0),
        ));
        graph.add_node(NodeInstance::new(
            "n4",
            "passthrough",
            Position::new(200.0, 0.0),
        ));

        // n1 -> n2 -> n4
        // n1 -> n3 -> n4
        graph.add_edge(Edge::new("e1", "n1", 0, "n2", 0));
        graph.add_edge(Edge::new("e2", "n1", 0, "n3", 0));
        graph.add_edge(Edge::new("e3", "n2", 0, "n4", 0));

        let result = executor.topological_sort(&graph);
        assert!(result.is_ok());

        let sorted = result.unwrap();
        assert_eq!(sorted.len(), 4);
        assert_eq!(sorted[0].id, "n1"); // Must be first
        assert_eq!(sorted[3].id, "n4"); // Must be last
    }

    #[test]
    #[cfg(target_arch = "wasm32")]
    fn test_execute_simple_graph() {
        let registry = NodeRegistryImpl::new();
        registry.register("source".to_string(), Box::new(SourceFactory));
        registry.register("passthrough".to_string(), Box::new(PassthroughFactory));

        let executor = GraphExecutorImpl::new(registry);

        let mut graph = Graph::new();
        graph.add_node(NodeInstance::new("n1", "source", Position::new(0.0, 0.0)));
        graph.add_node(NodeInstance::new(
            "n2",
            "passthrough",
            Position::new(100.0, 0.0),
        ));
        graph.add_edge(Edge::new("e1", "n1", 0, "n2", 0));

        let result = executor.execute(&graph);
        assert!(result.is_ok());

        let graph_result = result.unwrap();
        if !graph_result.success {
            eprintln!("Error: {:?}", graph_result.error);
        }
        assert!(graph_result.success);
        assert!(graph_result.error.is_none());
        assert!(!graph_result.outputs.is_empty());
    }

    #[test]
    fn test_execute_empty_graph() {
        let registry = NodeRegistryImpl::new();
        let executor = GraphExecutorImpl::new(registry);

        let graph = Graph::new();

        let result = executor.execute(&graph);
        assert!(result.is_ok());

        let graph_result = result.unwrap();
        assert!(graph_result.success);
        assert_eq!(graph_result.outputs.len(), 0);
    }

    #[test]
    fn test_validate_simple_graph() {
        let registry = NodeRegistryImpl::new();
        registry.register("source".to_string(), Box::new(SourceFactory));
        registry.register("passthrough".to_string(), Box::new(PassthroughFactory));

        let executor = GraphExecutorImpl::new(registry);

        let mut graph = Graph::new();
        graph.add_node(NodeInstance::new("n1", "source", Position::new(0.0, 0.0)));
        graph.add_node(NodeInstance::new(
            "n2",
            "passthrough",
            Position::new(100.0, 0.0),
        ));
        graph.add_edge(Edge::new("e1", "n1", 0, "n2", 0));

        let result = executor.validate(&graph);
        assert!(result.valid);
        assert_eq!(result.errors.len(), 0);
    }
}
