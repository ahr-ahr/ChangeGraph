//! Graph domain modules.
//!
//! This module exposes the core node, edge, and graph aggregate types.

pub mod edge;
pub mod graph;
pub mod node;

pub use edge::{Edge, Observation, RelationshipType};
pub use graph::{Graph, GraphError};
pub use node::{AttributeValue, Node, NodeAttributes, NodeId, NodeType};
