//! Graph validation logic (cycle detection, type checking)

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::structures::{Graph, NodeDefinition};

/// Validation error types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationErrorType {
    Cycle,
    TypeMismatch,
    Disconnected,
    InvalidParam,
}

/// Validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub error_type: ValidationErrorType,
    pub node_id: Option<String>,
    pub message: String,
}

impl ValidationError {
    pub fn new(
        error_type: ValidationErrorType,
        node_id: Option<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            error_type,
            node_id,
            message: message.into(),
        }
    }
}

/// Graph validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
        }
    }

    pub fn invalid(errors: Vec<ValidationError>) -> Self {
        Self {
            valid: false,
            errors,
        }
    }

    pub fn add_error(&mut self, error: ValidationError) {
        self.valid = false;
        self.errors.push(error);
    }
}

/// Graph validator
pub struct GraphValidator {
    node_definitions: HashMap<String, NodeDefinition>,
}

impl GraphValidator {
    pub fn new(node_definitions: HashMap<String, NodeDefinition>) -> Self {
        Self { node_definitions }
    }

    /// Validate the entire graph
    pub fn validate(&self, graph: &Graph) -> ValidationResult {
        let mut result = ValidationResult::valid();

        // Check for cycles
        if let Err(cycle_errors) = self.check_cycles(graph) {
            for error in cycle_errors {
                result.add_error(error);
            }
        }

        // Check type compatibility
        let type_errors = self.check_type_compatibility(graph);
        for error in type_errors {
            result.add_error(error);
        }

        // Check for disconnected required inputs
        let disconnected_errors = self.check_disconnected_inputs(graph);
        for error in disconnected_errors {
            result.add_error(error);
        }

        result
    }

    /// Check for cycles using DFS
    fn check_cycles(&self, graph: &Graph) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        // Build adjacency list
        let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();
        for node in &graph.nodes {
            adj_list.insert(node.id.clone(), Vec::new());
        }

        for edge in &graph.edges {
            adj_list
                .entry(edge.from_node.clone())
                .or_default()
                .push(edge.to_node.clone());
        }

        // DFS from each node
        for node in &graph.nodes {
            if !visited.contains(&node.id)
                && Self::dfs_cycle_check(
                    &node.id,
                    &adj_list,
                    &mut visited,
                    &mut rec_stack,
                    &mut errors,
                )
            {
                // Found a cycle, continue checking other components
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn dfs_cycle_check(
        node: &str,
        adj_list: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        errors: &mut Vec<ValidationError>,
    ) -> bool {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());

        if let Some(neighbors) = adj_list.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    if Self::dfs_cycle_check(neighbor, adj_list, visited, rec_stack, errors) {
                        return true;
                    }
                } else if rec_stack.contains(neighbor) {
                    // Cycle detected
                    errors.push(ValidationError::new(
                        ValidationErrorType::Cycle,
                        Some(node.to_string()),
                        format!("Cycle detected involving node '{}'", node),
                    ));
                    return true;
                }
            }
        }

        rec_stack.remove(node);
        false
    }

    /// Check type compatibility between connected ports
    fn check_type_compatibility(&self, graph: &Graph) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for edge in &graph.edges {
            // Get source node definition
            let from_node = match graph.get_node(&edge.from_node) {
                Some(n) => n,
                None => {
                    errors.push(ValidationError::new(
                        ValidationErrorType::Disconnected,
                        Some(edge.from_node.clone()),
                        format!("Source node '{}' not found", edge.from_node),
                    ));
                    continue;
                }
            };

            let from_def = match self.node_definitions.get(&from_node.node_type) {
                Some(d) => d,
                None => continue,
            };

            // Get target node definition
            let to_node = match graph.get_node(&edge.to_node) {
                Some(n) => n,
                None => {
                    errors.push(ValidationError::new(
                        ValidationErrorType::Disconnected,
                        Some(edge.to_node.clone()),
                        format!("Target node '{}' not found", edge.to_node),
                    ));
                    continue;
                }
            };

            let to_def = match self.node_definitions.get(&to_node.node_type) {
                Some(d) => d,
                None => continue,
            };

            // Check port indices
            if edge.from_port >= from_def.outputs.len() {
                errors.push(ValidationError::new(
                    ValidationErrorType::TypeMismatch,
                    Some(edge.from_node.clone()),
                    format!(
                        "Output port {} does not exist on node '{}'",
                        edge.from_port, edge.from_node
                    ),
                ));
                continue;
            }

            if edge.to_port >= to_def.inputs.len() {
                errors.push(ValidationError::new(
                    ValidationErrorType::TypeMismatch,
                    Some(edge.to_node.clone()),
                    format!(
                        "Input port {} does not exist on node '{}'",
                        edge.to_port, edge.to_node
                    ),
                ));
                continue;
            }

            // Check type compatibility
            let from_type = &from_def.outputs[edge.from_port].data_type;
            let to_type = &to_def.inputs[edge.to_port].data_type;

            if !from_type.is_compatible_with(to_type) {
                errors.push(ValidationError::new(
                    ValidationErrorType::TypeMismatch,
                    Some(edge.from_node.clone()),
                    format!(
                        "Type mismatch: Cannot connect {:?} output to {:?} input",
                        from_type, to_type
                    ),
                ));
            }
        }

        errors
    }

    /// Check for disconnected required inputs
    fn check_disconnected_inputs(&self, graph: &Graph) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Build map of which input ports are connected
        let mut connected_inputs: HashMap<(String, usize), bool> = HashMap::new();
        for edge in &graph.edges {
            connected_inputs.insert((edge.to_node.clone(), edge.to_port), true);
        }

        // Check each node's required inputs
        for node in &graph.nodes {
            if let Some(def) = self.node_definitions.get(&node.node_type) {
                for (idx, input) in def.inputs.iter().enumerate() {
                    // For now, all inputs are required (no optional field in contract)
                    if !connected_inputs.contains_key(&(node.id.clone(), idx)) {
                        errors.push(ValidationError::new(
                            ValidationErrorType::Disconnected,
                            Some(node.id.clone()),
                            format!(
                                "Required input port '{}' is not connected on node '{}'",
                                input.name, node.id
                            ),
                        ));
                    }
                }
            }
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_graph::{DataType, Edge, NodeCategory, NodeInstance, PortDefinition, Position};

    fn create_test_node_def(id: &str, inputs: usize, outputs: usize) -> NodeDefinition {
        let mut def = NodeDefinition::new(id, id, NodeCategory::Processing, "Test node");

        let input_ports: Vec<_> = (0..inputs)
            .map(|i| {
                PortDefinition::new(
                    format!("in{}", i),
                    format!("Input {}", i),
                    DataType::BitStream,
                )
            })
            .collect();

        let output_ports: Vec<_> = (0..outputs)
            .map(|i| {
                PortDefinition::new(
                    format!("out{}", i),
                    format!("Output {}", i),
                    DataType::BitStream,
                )
            })
            .collect();

        def.inputs = input_ports;
        def.outputs = output_ports;
        def
    }

    #[test]
    fn test_validate_empty_graph() {
        let graph = Graph::new();
        let validator = GraphValidator::new(HashMap::new());
        let result = validator.validate(&graph);
        assert!(result.valid);
    }

    #[test]
    fn test_validate_simple_chain() {
        let mut graph = Graph::new();
        let mut defs = HashMap::new();

        let def = create_test_node_def("test_node", 1, 1);
        defs.insert("test_node".to_string(), def);

        graph.add_node(NodeInstance::new(
            "n1",
            "test_node",
            Position::new(0.0, 0.0),
        ));
        graph.add_node(NodeInstance::new(
            "n2",
            "test_node",
            Position::new(100.0, 0.0),
        ));
        graph.add_edge(Edge::new("e1", "n1", 0, "n2", 0));

        let validator = GraphValidator::new(defs);
        let result = validator.validate(&graph);

        // Should be valid except for n1's disconnected input
        assert!(!result.valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(
            result.errors[0].error_type,
            ValidationErrorType::Disconnected
        );
    }

    #[test]
    fn test_detect_cycle() {
        let mut graph = Graph::new();
        let mut defs = HashMap::new();

        let def = create_test_node_def("test_node", 1, 1);
        defs.insert("test_node".to_string(), def);

        graph.add_node(NodeInstance::new(
            "n1",
            "test_node",
            Position::new(0.0, 0.0),
        ));
        graph.add_node(NodeInstance::new(
            "n2",
            "test_node",
            Position::new(100.0, 0.0),
        ));

        // Create a cycle: n1 -> n2 -> n1
        graph.add_edge(Edge::new("e1", "n1", 0, "n2", 0));
        graph.add_edge(Edge::new("e2", "n2", 0, "n1", 0));

        let validator = GraphValidator::new(defs);
        let result = validator.validate(&graph);

        assert!(!result.valid);
        let has_cycle_error = result
            .errors
            .iter()
            .any(|e| e.error_type == ValidationErrorType::Cycle);
        assert!(has_cycle_error);
    }

    #[test]
    fn test_type_mismatch() {
        let mut graph = Graph::new();
        let mut defs = HashMap::new();

        let mut def1 = NodeDefinition::new("source", "Source", NodeCategory::Source, "Test");
        def1.outputs = vec![PortDefinition::new("out", "Output", DataType::BitStream)];
        defs.insert("source".to_string(), def1);

        let mut def2 = NodeDefinition::new("sink", "Sink", NodeCategory::Sink, "Test");
        def2.inputs = vec![PortDefinition::new("in", "Input", DataType::IQData)];
        defs.insert("sink".to_string(), def2);

        graph.add_node(NodeInstance::new("n1", "source", Position::new(0.0, 0.0)));
        graph.add_node(NodeInstance::new("n2", "sink", Position::new(100.0, 0.0)));
        graph.add_edge(Edge::new("e1", "n1", 0, "n2", 0));

        let validator = GraphValidator::new(defs);
        let result = validator.validate(&graph);

        assert!(!result.valid);
        let has_type_error = result
            .errors
            .iter()
            .any(|e| e.error_type == ValidationErrorType::TypeMismatch);
        assert!(has_type_error);
    }

    #[test]
    fn test_invalid_port_index() {
        let mut graph = Graph::new();
        let mut defs = HashMap::new();

        let def = create_test_node_def("test_node", 1, 1);
        defs.insert("test_node".to_string(), def);

        graph.add_node(NodeInstance::new(
            "n1",
            "test_node",
            Position::new(0.0, 0.0),
        ));
        graph.add_node(NodeInstance::new(
            "n2",
            "test_node",
            Position::new(100.0, 0.0),
        ));

        // Try to connect to non-existent port
        graph.add_edge(Edge::new("e1", "n1", 5, "n2", 0));

        let validator = GraphValidator::new(defs);
        let result = validator.validate(&graph);

        assert!(!result.valid);
        assert!(result
            .errors
            .iter()
            .any(|e| e.error_type == ValidationErrorType::TypeMismatch));
    }
}
