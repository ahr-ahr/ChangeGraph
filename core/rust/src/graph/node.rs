//! Graph node domain model.
//!
//! This module defines the internal representation of a ChangeGraph node.
//! It intentionally does not depend on the generated Protobuf types so the
//! core domain remains independent from a specific transport or wire format.

use std::collections::BTreeMap;

/// Stable identity of a graph node.
pub type NodeId = String;

/// A value that can be attached to a node as an attribute or metadata.
///
/// This is intentionally small and transport-neutral. Protocol-specific
/// representations, such as `google.protobuf.Struct`, are mapped into this
/// domain representation by the protocol layer.
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    List(Vec<AttributeValue>),
    Object(BTreeMap<String, AttributeValue>),
}

/// Collection of node attributes.
pub type NodeAttributes = BTreeMap<String, AttributeValue>;

/// Category of a graph node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeType {
    Service,
    Endpoint,
    Database,
    DatabaseTable,
    Queue,
    Topic,
    Event,
    ExternalService,
    Resource,
}

impl NodeType {
    /// Returns the canonical protocol name for this node type.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Service => "service",
            Self::Endpoint => "endpoint",
            Self::Database => "database",
            Self::DatabaseTable => "database_table",
            Self::Queue => "queue",
            Self::Topic => "topic",
            Self::Event => "event",
            Self::ExternalService => "external_service",
            Self::Resource => "resource",
        }
    }
}

/// Represents one node in the ChangeGraph dependency graph.
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    /// Stable identity of the node.
    pub id: NodeId,

    /// Category of the node.
    pub node_type: NodeType,

    /// Human-readable name.
    pub name: String,

    /// Descriptive attributes associated with the node.
    pub attributes: NodeAttributes,

    /// Additional metadata associated with the node.
    pub metadata: NodeAttributes,
}

impl Node {
    /// Creates a node with empty attributes and metadata.
    pub fn new(
        id: impl Into<NodeId>,
        node_type: NodeType,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            node_type,
            name: name.into(),
            attributes: NodeAttributes::new(),
            metadata: NodeAttributes::new(),
        }
    }

    /// Adds or replaces an attribute.
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: AttributeValue,
    ) -> Self {
        self.attributes.insert(key.into(), value);
        self
    }

    /// Adds or replaces metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: AttributeValue,
    ) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }
}
