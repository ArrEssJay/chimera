//! Integration tests for node graph system

use chimera_core::node_graph::{
    DataBuffer, DataType, Edge, Graph, GraphExecutorImpl, IQSample, Node, NodeCategory,
    NodeDefinition, NodeFactory, NodeInstance, NodeRegistryImpl, ParameterValue, PortDefinition,
    Position,
};
use std::collections::HashMap;
use wasm_bindgen::JsValue;

// Test node: BitStream source
struct BitStreamSource {
    id: String,
}

impl Node for BitStreamSource {
    fn id(&self) -> &str {
        &self.id
    }

    fn definition(&self) -> NodeDefinition {
        NodeDefinition::new(
            "bit_source",
            "Bit Stream Source",
            NodeCategory::Source,
            "Generates a bit stream",
        )
        .with_outputs(vec![PortDefinition::new(
            "bits",
            "Bits",
            DataType::BitStream,
        )])
    }

    fn execute(
        &self,
        _inputs: Vec<DataBuffer>,
        params: JsValue,
    ) -> Result<Vec<DataBuffer>, JsValue> {
        // Parse count parameter
        let count: usize = if let Ok(map) =
            serde_wasm_bindgen::from_value::<HashMap<String, ParameterValue>>(params)
        {
            if let Some(ParameterValue::Number(n)) = map.get("count") {
                *n as usize
            } else {
                10
            }
        } else {
            10
        };

        // Generate alternating bits
        let bits: Vec<bool> = (0..count).map(|i| i % 2 == 0).collect();
        Ok(vec![DataBuffer::BitStream(bits)])
    }
}

struct BitStreamSourceFactory;

impl NodeFactory for BitStreamSourceFactory {
    fn create(&self, id: String) -> Box<dyn Node> {
        Box::new(BitStreamSource { id })
    }

    fn definition(&self) -> NodeDefinition {
        NodeDefinition::new(
            "bit_source",
            "Bit Stream Source",
            NodeCategory::Source,
            "Generates a bit stream",
        )
        .with_outputs(vec![PortDefinition::new(
            "bits",
            "Bits",
            DataType::BitStream,
        )])
    }
}

// Test node: Bit to IQ converter
struct BitToIQConverter {
    id: String,
}

impl Node for BitToIQConverter {
    fn id(&self) -> &str {
        &self.id
    }

    fn definition(&self) -> NodeDefinition {
        NodeDefinition::new(
            "bit_to_iq",
            "Bit to IQ Converter",
            NodeCategory::Processing,
            "Converts bits to IQ samples",
        )
        .with_inputs(vec![PortDefinition::new(
            "bits",
            "Bits",
            DataType::BitStream,
        )])
        .with_outputs(vec![PortDefinition::new("iq", "IQ Data", DataType::IQData)])
    }

    fn execute(
        &self,
        inputs: Vec<DataBuffer>,
        _params: JsValue,
    ) -> Result<Vec<DataBuffer>, JsValue> {
        if inputs.is_empty() {
            return Err(JsValue::from_str("No input provided"));
        }

        match &inputs[0] {
            DataBuffer::BitStream(bits) => {
                let iq_samples: Vec<IQSample> = bits
                    .iter()
                    .map(|&bit| {
                        if bit {
                            IQSample::new(1.0, 0.0)
                        } else {
                            IQSample::new(-1.0, 0.0)
                        }
                    })
                    .collect();
                Ok(vec![DataBuffer::IQData(iq_samples)])
            }
            _ => Err(JsValue::from_str("Expected BitStream input")),
        }
    }
}

struct BitToIQConverterFactory;

impl NodeFactory for BitToIQConverterFactory {
    fn create(&self, id: String) -> Box<dyn Node> {
        Box::new(BitToIQConverter { id })
    }

    fn definition(&self) -> NodeDefinition {
        NodeDefinition::new(
            "bit_to_iq",
            "Bit to IQ Converter",
            NodeCategory::Processing,
            "Converts bits to IQ samples",
        )
        .with_inputs(vec![PortDefinition::new(
            "bits",
            "Bits",
            DataType::BitStream,
        )])
        .with_outputs(vec![PortDefinition::new("iq", "IQ Data", DataType::IQData)])
    }
}

#[test]
fn test_simple_two_node_graph() {
    // Create registry and register nodes
    let registry = NodeRegistryImpl::new();
    registry.register("bit_source".to_string(), Box::new(BitStreamSourceFactory));
    registry.register("bit_to_iq".to_string(), Box::new(BitToIQConverterFactory));

    // Create executor
    let executor = GraphExecutorImpl::new(registry);

    // Build graph
    let mut graph = Graph::new();

    let mut node1 = NodeInstance::new("source", "bit_source", Position::new(0.0, 0.0));
    node1
        .parameters
        .insert("count".to_string(), ParameterValue::Number(5.0));
    graph.add_node(node1);

    graph.add_node(NodeInstance::new(
        "converter",
        "bit_to_iq",
        Position::new(100.0, 0.0),
    ));

    graph.add_edge(Edge::new("e1", "source", 0, "converter", 0));

    // Validate graph
    let validation = executor.validate(&graph);
    assert!(validation.valid, "Graph should be valid");

    // Topological sort should work
    // Note: We can't fully execute due to WASM limitations in tests,
    // but we can validate the structure
}

#[test]
fn test_three_node_chain() {
    // Create a simple three-node chain
    let mut graph = Graph::new();

    graph.add_node(NodeInstance::new(
        "n1",
        "bit_source",
        Position::new(0.0, 0.0),
    ));
    graph.add_node(NodeInstance::new(
        "n2",
        "bit_to_iq",
        Position::new(100.0, 0.0),
    ));
    graph.add_node(NodeInstance::new(
        "n3",
        "bit_to_iq",
        Position::new(200.0, 0.0),
    ));

    graph.add_edge(Edge::new("e1", "n1", 0, "n2", 0));
    graph.add_edge(Edge::new("e2", "n2", 0, "n3", 0));

    // Check graph structure
    assert_eq!(graph.nodes.len(), 3);
    assert_eq!(graph.edges.len(), 2);
}

#[test]
fn test_graph_with_disconnected_nodes() {
    // Create graph with disconnected nodes
    let mut graph = Graph::new();

    graph.add_node(NodeInstance::new(
        "n1",
        "bit_source",
        Position::new(0.0, 0.0),
    ));
    graph.add_node(NodeInstance::new(
        "n2",
        "bit_to_iq",
        Position::new(100.0, 0.0),
    ));
    // No edge connecting them

    // This should be valid from a structure perspective,
    // but validation will catch missing inputs
    assert_eq!(graph.nodes.len(), 2);
    assert_eq!(graph.edges.len(), 0);
}

#[test]
fn test_node_parameters() {
    let mut node = NodeInstance::new("test", "bit_source", Position::new(0.0, 0.0));

    node.parameters
        .insert("count".to_string(), ParameterValue::Number(100.0));
    node.parameters
        .insert("enabled".to_string(), ParameterValue::Boolean(true));
    node.parameters.insert(
        "name".to_string(),
        ParameterValue::String("test".to_string()),
    );

    assert_eq!(node.parameters.len(), 3);
}

#[test]
fn test_iq_sample_operations() {
    let sample = IQSample::new(3.0, 4.0);

    // Test magnitude
    assert_eq!(sample.magnitude(), 5.0);

    // Test phase
    let phase = sample.phase();
    assert!((phase - 0.9272952).abs() < 0.001); // atan2(4, 3)
}

#[test]
fn test_data_buffer_types() {
    let bit_buffer = DataBuffer::BitStream(vec![true, false, true]);
    assert_eq!(bit_buffer.data_type(), DataType::BitStream);
    assert_eq!(bit_buffer.len(), 3);

    let iq_buffer = DataBuffer::IQData(vec![IQSample::new(1.0, 0.0), IQSample::new(0.0, 1.0)]);
    assert_eq!(iq_buffer.data_type(), DataType::IQData);
    assert_eq!(iq_buffer.len(), 2);

    let audio_buffer = DataBuffer::AudioSamples(vec![0.1, 0.2, 0.3]);
    assert_eq!(audio_buffer.data_type(), DataType::AudioSamples);
    assert_eq!(audio_buffer.len(), 3);

    let mut metadata = HashMap::new();
    metadata.insert("key".to_string(), "value".to_string());
    let meta_buffer = DataBuffer::Metadata(metadata);
    assert_eq!(meta_buffer.data_type(), DataType::Metadata);
    assert_eq!(meta_buffer.len(), 1);
}

#[test]
fn test_node_definition_builder() {
    let def = NodeDefinition::new(
        "test_node",
        "Test Node",
        NodeCategory::Processing,
        "A test node",
    )
    .with_inputs(vec![PortDefinition::new(
        "in",
        "Input",
        DataType::BitStream,
    )])
    .with_outputs(vec![PortDefinition::new("out", "Output", DataType::IQData)]);

    assert_eq!(def.id, "test_node");
    assert_eq!(def.name, "Test Node");
    assert_eq!(def.inputs.len(), 1);
    assert_eq!(def.outputs.len(), 1);
}

#[test]
fn test_graph_node_lookup() {
    let mut graph = Graph::new();

    graph.add_node(NodeInstance::new("n1", "test", Position::new(0.0, 0.0)));
    graph.add_node(NodeInstance::new("n2", "test", Position::new(100.0, 0.0)));

    assert!(graph.get_node("n1").is_some());
    assert!(graph.get_node("n2").is_some());
    assert!(graph.get_node("n3").is_none());
}
