//! Node Graph System for Visual Dataflow Programming
//!
//! This module provides the core node graph engine for building visual DSP pipelines.
//! It includes:
//! - Data buffer types for passing data between nodes
//! - Graph data structures (nodes, edges, connections)
//! - Node registry for dynamic node creation
//! - Graph validation (cycle detection, type checking)
//! - Graph executor (topological sort, sequential execution)

pub mod data_buffer;
pub mod executor;
pub mod registry;
pub mod structures;
pub mod validator;
pub mod wasm_api;

// Re-export key types for convenience
pub use data_buffer::*;
pub use executor::*;
pub use registry::*;
pub use structures::*;
pub use validator::*;
pub use wasm_api::*;
