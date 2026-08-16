//! Mapping between ChangeGraph domain models and protocol representations.
//!
//! The generated Protobuf types are intentionally kept out of this module
//! until the protocol build pipeline is introduced. The intermediate
//! representations below mirror the stable fields defined by
//! `specs/graph/graph.proto` and provide a transport-neutral mapping boundary.

use std::collections::BTreeMap;

use crate::graph::{
    AttributeValue, Edge, Graph, Node, NodeId, NodeType, Observation, RelationshipType,
};

/// Protocol-level value representation used by the mapping boundary.
#[derive(Debug, Clone, PartialEq)]
pub enum ProtocolValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    List(Vec<ProtocolValue>),
    Object(BTreeMap<String, ProtocolValue>),
}

/// Protocol-level representation of a graph node.
#[derive(Debug, Clone, PartialEq)]
pub struct ProtocolNode {
    pub id: String,
    pub node_type: NodeType,
    pub name: String,
    pub attributes: BTreeMap<String, ProtocolValue>,
    pub metadata: BTreeMap<String, ProtocolValue>,
}

/// Protocol-level representation of an observation.
#[derive(Debug, Clone, PartialEq)]
pub struct ProtocolObservation {
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
    pub timestamp_unix_nanos: Option<i64>,
    pub source_service: Option<String>,
    pub attributes: BTreeMap<String, ProtocolValue>,
}

/// Protocol-level representation of a graph edge.
#[derive(Debug, Clone, PartialEq)]
pub struct ProtocolEdge {
    pub source_node_id: String,
    pub target_node_id: String,
    pub relationship_type: RelationshipType,
    pub attributes: BTreeMap<String, ProtocolValue>,
    pub observations: Vec<ProtocolObservation>,
}

/// Protocol-level representation of a dependency graph.
#[derive(Debug, Clone, PartialEq)]
pub struct ProtocolGraph {
    pub nodes: Vec<ProtocolNode>,
    pub edges: Vec<ProtocolEdge>,
}

/// Maps domain graph values into the protocol boundary.
#[derive(Debug, Default, Clone, Copy)]
pub struct GraphMapper;

impl GraphMapper {
    /// Creates a new graph mapper.
    pub const fn new() -> Self {
        Self
    }

    /// Maps a domain graph into its protocol representation.
    pub fn map_graph(&self, graph: &Graph) -> ProtocolGraph {
        ProtocolGraph {
            nodes: graph.nodes().map(Self::map_node).collect(),
            edges: graph.edges().map(Self::map_edge).collect(),
        }
    }

    /// Maps a domain node into its protocol representation.
    pub fn map_node(node: &Node) -> ProtocolNode {
        ProtocolNode {
            id: node.id.clone(),
            node_type: node.node_type,
            name: node.name.clone(),
            attributes: map_attributes(&node.attributes),
            metadata: map_attributes(&node.metadata),
        }
    }

    /// Maps a domain edge into its protocol representation.
    pub fn map_edge(edge: &Edge) -> ProtocolEdge {
        ProtocolEdge {
            source_node_id: edge.source_node_id.clone(),
            target_node_id: edge.target_node_id.clone(),
            relationship_type: edge.relationship_type,
            attributes: map_attributes(&edge.attributes),
            observations: edge
                .observations
                .iter()
                .map(Self::map_observation)
                .collect(),
        }
    }

    /// Maps a domain observation into its protocol representation.
    pub fn map_observation(observation: &Observation) -> ProtocolObservation {
        ProtocolObservation {
            trace_id: observation.trace_id.clone(),
            span_id: observation.span_id.clone(),
            timestamp_unix_nanos: observation.timestamp_unix_nanos,
            source_service: observation.source_service.clone(),
            attributes: map_attributes(&observation.attributes),
        }
    }

    /// Maps a stable node identifier without changing its value.
    pub fn map_node_id(node_id: &NodeId) -> String {
        node_id.clone()
    }
}

fn map_attributes(
    attributes: &BTreeMap<String, AttributeValue>,
) -> BTreeMap<String, ProtocolValue> {
    attributes
        .iter()
        .map(|(key, value)| (key.clone(), map_value(value)))
        .collect()
}

fn map_value(value: &AttributeValue) -> ProtocolValue {
    match value {
        AttributeValue::Null => ProtocolValue::Null,
        AttributeValue::Bool(value) => ProtocolValue::Bool(*value),
        AttributeValue::Number(value) => ProtocolValue::Number(*value),
        AttributeValue::String(value) => ProtocolValue::String(value.clone()),
        AttributeValue::List(values) => {
            ProtocolValue::List(values.iter().map(map_value).collect())
        }
        AttributeValue::Object(values) => ProtocolValue::Object(
            values
                .iter()
                .map(|(key, value)| (key.clone(), map_value(value)))
                .collect(),
        ),
    }
}
