//! Graph edge domain model.
//!
//! This module defines directed relationships between graph nodes and keeps
//! telemetry observations as evidence for those relationships. The domain
//! model remains independent from the Protobuf representation.

use std::collections::BTreeMap;

use super::node::{AttributeValue, NodeAttributes, NodeId};

/// Type of directed relationship between two graph nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationshipType {
    Calls,
    Reads,
    Writes,
    Publishes,
    Consumes,
    DependsOn,
}

impl RelationshipType {
    /// Returns the canonical protocol name for this relationship type.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Calls => "calls",
            Self::Reads => "reads",
            Self::Writes => "writes",
            Self::Publishes => "publishes",
            Self::Consumes => "consumes",
            Self::DependsOn => "depends_on",
        }
    }
}

/// One telemetry observation that provides evidence for a logical edge.
#[derive(Debug, Clone, PartialEq)]
pub struct Observation {
    /// OpenTelemetry trace identifier, when available.
    pub trace_id: Option<String>,

    /// OpenTelemetry span identifier, when available.
    pub span_id: Option<String>,

    /// Unix timestamp in nanoseconds, when available.
    ///
    /// The transport layer can map this to `google.protobuf.Timestamp`.
    pub timestamp_unix_nanos: Option<i64>,

    /// Service that produced the observation, when available.
    pub source_service: Option<String>,

    /// Runtime or telemetry attributes associated with the observation.
    pub attributes: NodeAttributes,
}

impl Observation {
    /// Creates an empty observation.
    pub fn new() -> Self {
        Self {
            trace_id: None,
            span_id: None,
            timestamp_unix_nanos: None,
            source_service: None,
            attributes: BTreeMap::new(),
        }
    }

    /// Sets the trace identifier.
    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }

    /// Sets the span identifier.
    pub fn with_span_id(mut self, span_id: impl Into<String>) -> Self {
        self.span_id = Some(span_id.into());
        self
    }

    /// Sets the observation timestamp.
    pub fn with_timestamp_unix_nanos(mut self, timestamp: i64) -> Self {
        self.timestamp_unix_nanos = Some(timestamp);
        self
    }

    /// Sets the source service.
    pub fn with_source_service(mut self, source_service: impl Into<String>) -> Self {
        self.source_service = Some(source_service.into());
        self
    }

    /// Adds or replaces an observation attribute.
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: AttributeValue,
    ) -> Self {
        self.attributes.insert(key.into(), value);
        self
    }
}

impl Default for Observation {
    fn default() -> Self {
        Self::new()
    }
}

/// A directed logical relationship between two graph nodes.
#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    /// Stable identifier of the source node.
    pub source_node_id: NodeId,

    /// Stable identifier of the target node.
    pub target_node_id: NodeId,

    /// Type of relationship between source and target.
    pub relationship_type: RelationshipType,

    /// Additional relationship attributes.
    pub attributes: NodeAttributes,

    /// Telemetry observations that provide evidence for this relationship.
    pub observations: Vec<Observation>,
}

impl Edge {
    /// Creates an edge with empty attributes and observations.
    pub fn new(
        source_node_id: impl Into<NodeId>,
        target_node_id: impl Into<NodeId>,
        relationship_type: RelationshipType,
    ) -> Self {
        Self {
            source_node_id: source_node_id.into(),
            target_node_id: target_node_id.into(),
            relationship_type,
            attributes: NodeAttributes::new(),
            observations: Vec::new(),
        }
    }

    /// Adds or replaces an edge attribute.
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: AttributeValue,
    ) -> Self {
        self.attributes.insert(key.into(), value);
        self
    }

    /// Adds telemetry evidence to the edge.
    pub fn with_observation(mut self, observation: Observation) -> Self {
        self.observations.push(observation);
        self
    }
}
