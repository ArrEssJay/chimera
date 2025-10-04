//! Node registry for dynamic node creation

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::structures::{Node, NodeDefinition};

/// Factory for creating node instances
pub trait NodeFactory: Send + Sync {
    fn create(&self, id: String) -> Box<dyn Node>;
    fn definition(&self) -> NodeDefinition;
}

/// Node registry for dynamic node creation
pub struct NodeRegistryImpl {
    factories: Arc<RwLock<HashMap<String, Box<dyn NodeFactory>>>>,
}

impl NodeRegistryImpl {
    pub fn new() -> Self {
        Self {
            factories: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Register a node type
    pub fn register(&self, node_type: String, factory: Box<dyn NodeFactory>) {
        let mut factories = self.factories.write()
            .expect("Failed to acquire write lock on registry");
        factories.insert(node_type, factory);
    }
    
    /// Create a node instance by type
    pub fn create_node(&self, node_type: &str, id: String) -> Result<Box<dyn Node>, String> {
        let factories = self.factories.read()
            .expect("Failed to acquire read lock on registry");
        
        match factories.get(node_type) {
            Some(factory) => Ok(factory.create(id)),
            None => Err(format!("Unknown node type: {}", node_type)),
        }
    }
    
    /// Get all registered node types
    pub fn available_nodes(&self) -> Vec<NodeDefinition> {
        let factories = self.factories.read()
            .expect("Failed to acquire read lock on registry");
        
        factories.values()
            .map(|f| f.definition())
            .collect()
    }
    
    /// Get definition for a specific node type
    pub fn get_definition(&self, node_type: &str) -> Option<NodeDefinition> {
        let factories = self.factories.read()
            .expect("Failed to acquire read lock on registry");
        
        factories.get(node_type).map(|f| f.definition())
    }
}

impl Default for NodeRegistryImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for NodeRegistryImpl {
    fn clone(&self) -> Self {
        Self {
            factories: Arc::clone(&self.factories),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_graph::{DataBuffer, NodeCategory, PortDefinition, DataType};
    use wasm_bindgen::JsValue;

    // Mock node for testing
    struct TestNode {
        id: String,
    }

    impl Node for TestNode {
        fn id(&self) -> &str {
            &self.id
        }

        fn definition(&self) -> NodeDefinition {
            NodeDefinition::new(
                "test_node",
                "Test Node",
                NodeCategory::Processing,
                "A test node"
            )
            .with_inputs(vec![PortDefinition::new("in", "Input", DataType::BitStream)])
            .with_outputs(vec![PortDefinition::new("out", "Output", DataType::BitStream)])
        }

        fn execute(
            &self,
            inputs: Vec<DataBuffer>,
            _params: JsValue,
        ) -> Result<Vec<DataBuffer>, JsValue> {
            Ok(inputs)
        }
    }

    struct TestNodeFactory;

    impl NodeFactory for TestNodeFactory {
        fn create(&self, id: String) -> Box<dyn Node> {
            Box::new(TestNode { id })
        }

        fn definition(&self) -> NodeDefinition {
            NodeDefinition::new(
                "test_node",
                "Test Node",
                NodeCategory::Processing,
                "A test node"
            )
        }
    }

    #[test]
    fn test_register_and_create() {
        let registry = NodeRegistryImpl::new();
        
        registry.register("test_node".to_string(), Box::new(TestNodeFactory));
        
        let node = registry.create_node("test_node", "n1".to_string());
        assert!(node.is_ok());
        
        let node = node.unwrap();
        assert_eq!(node.id(), "n1");
    }

    #[test]
    fn test_create_unknown_node() {
        let registry = NodeRegistryImpl::new();
        
        let result = registry.create_node("unknown", "n1".to_string());
        assert!(result.is_err());
        let err_msg = result.err().unwrap();
        assert!(err_msg.contains("Unknown node type"));
    }

    #[test]
    fn test_available_nodes() {
        let registry = NodeRegistryImpl::new();
        
        registry.register("test_node".to_string(), Box::new(TestNodeFactory));
        
        let nodes = registry.available_nodes();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].id, "test_node");
    }

    #[test]
    fn test_get_definition() {
        let registry = NodeRegistryImpl::new();
        
        registry.register("test_node".to_string(), Box::new(TestNodeFactory));
        
        let def = registry.get_definition("test_node");
        assert!(def.is_some());
        assert_eq!(def.unwrap().id, "test_node");
        
        let def = registry.get_definition("unknown");
        assert!(def.is_none());
    }

    #[test]
    fn test_registry_clone() {
        let registry1 = NodeRegistryImpl::new();
        registry1.register("test_node".to_string(), Box::new(TestNodeFactory));
        
        let registry2 = registry1.clone();
        
        // Both should have access to the same factories
        assert_eq!(registry2.available_nodes().len(), 1);
    }
}
