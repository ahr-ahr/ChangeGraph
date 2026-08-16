//! Mapping between ChangeGraph domain models and generated Protobuf types.

use std::collections::BTreeMap;

use prost_types::{Struct, Timestamp, Value};

use crate::graph::{
    AttributeValue, Edge, Graph, Node, NodeId, NodeType, Observation, RelationshipType,
};

use super::generated::{
    Edge as ProtoEdge,
    Graph as ProtoGraph,
    Node as ProtoNode,
    NodeType as ProtoNodeType,
    Observation as ProtoObservation,
    RelationshipType as ProtoRelationshipType,
};

/// Maps ChangeGraph domain models into generated Protobuf representations.
#[derive(Debug, Default, Clone, Copy)]
pub struct GraphMapper;

impl GraphMapper {
    /// Creates a new graph mapper.
    pub const fn new() -> Self {
        Self
    }

    /// Maps a domain graph into a Protobuf graph.
    pub fn map_graph(&self, graph: &Graph) -> ProtoGraph {
        ProtoGraph {
            nodes: graph.nodes().map(Self::map_node).collect(),
            edges: graph.edges().map(Self::map_edge).collect(),
            metadata: None,
        }
    }

    /// Maps a domain node into a Protobuf node.
    pub fn map_node(node: &Node) -> ProtoNode {
        ProtoNode {
            id: node.id.clone(),
            r#type: node_type_to_proto(node.node_type),
            name: node.name.clone(),
            attributes: map_struct(&node.attributes),
            metadata: map_struct(&node.metadata),
        }
    }

    /// Maps a domain edge into a Protobuf edge.
    pub fn map_edge(edge: &Edge) -> ProtoEdge {
        ProtoEdge {
            source_node_id: edge.source_node_id.clone(),
            target_node_id: edge.target_node_id.clone(),
            relationship_type: relationship_type_to_proto(edge.relationship_type),
            attributes: map_struct(&edge.attributes),
            observations: edge
                .observations
                .iter()
                .map(Self::map_observation)
                .collect(),
        }
    }

    /// Maps a domain observation into a Protobuf observation.
    pub fn map_observation(observation: &Observation) -> ProtoObservation {
        ProtoObservation {
            trace_id: observation.trace_id.clone().unwrap_or_default(),
            span_id: observation.span_id.clone().unwrap_or_default(),
            timestamp: observation
                .timestamp_unix_nanos
                .map(timestamp_from_unix_nanos),
            source_service: observation.source_service.clone().unwrap_or_default(),
            attributes: map_struct(&observation.attributes),
        }
    }

    /// Maps a stable node identifier without changing its value.
    pub fn map_node_id(node_id: &NodeId) -> String {
        node_id.clone()
    }
}

fn node_type_to_proto(node_type: NodeType) -> i32 {
    match node_type {
        NodeType::Service => ProtoNodeType::Service as i32,
        NodeType::Endpoint => ProtoNodeType::Endpoint as i32,
        NodeType::Database => ProtoNodeType::Database as i32,
        NodeType::DatabaseTable => ProtoNodeType::DatabaseTable as i32,
        NodeType::Queue => ProtoNodeType::Queue as i32,
        NodeType::Topic => ProtoNodeType::Topic as i32,
        NodeType::Event => ProtoNodeType::Event as i32,
        NodeType::ExternalService => ProtoNodeType::ExternalService as i32,
        NodeType::Resource => ProtoNodeType::Resource as i32,
    }
}

fn relationship_type_to_proto(relationship_type: RelationshipType) -> i32 {
    match relationship_type {
        RelationshipType::Calls => ProtoRelationshipType::Calls as i32,
        RelationshipType::Reads => ProtoRelationshipType::Reads as i32,
        RelationshipType::Writes => ProtoRelationshipType::Writes as i32,
        RelationshipType::Publishes => ProtoRelationshipType::Publishes as i32,
        RelationshipType::Consumes => ProtoRelationshipType::Consumes as i32,
        RelationshipType::DependsOn => ProtoRelationshipType::DependsOn as i32,
    }
}

fn map_struct(attributes: &BTreeMap<String, AttributeValue>) -> Option<Struct> {
    if attributes.is_empty() {
        return None;
    }

    Some(Struct {
        fields: attributes
            .iter()
            .map(|(key, value)| (key.clone(), map_value(value)))
            .collect(),
    })
}

fn map_value(value: &AttributeValue) -> Value {
    match value {
        AttributeValue::Null => Value {
            kind: Some(prost_types::value::Kind::NullValue(0)),
        },

        AttributeValue::Bool(value) => Value {
            kind: Some(prost_types::value::Kind::BoolValue(*value)),
        },

        AttributeValue::Number(value) => Value {
            kind: Some(prost_types::value::Kind::NumberValue(*value)),
        },

        AttributeValue::String(value) => Value {
            kind: Some(prost_types::value::Kind::StringValue(value.clone())),
        },

        AttributeValue::List(values) => Value {
            kind: Some(prost_types::value::Kind::ListValue(
                prost_types::ListValue {
                    values: values.iter().map(map_value).collect(),
                },
            )),
        },

        AttributeValue::Object(values) => Value {
            kind: Some(prost_types::value::Kind::StructValue(Struct {
                fields: values
                    .iter()
                    .map(|(key, value)| (key.clone(), map_value(value)))
                    .collect(),
            })),
        },
    }
}

fn timestamp_from_unix_nanos(unix_nanos: i64) -> Timestamp {
    let seconds = unix_nanos.div_euclid(1_000_000_000);
    let nanos = unix_nanos.rem_euclid(1_000_000_000) as i32;

    Timestamp { seconds, nanos }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_node_type_to_proto() {
        assert_eq!(
            node_type_to_proto(NodeType::Service),
            ProtoNodeType::Service as i32
        );
        assert_eq!(
            node_type_to_proto(NodeType::Endpoint),
            ProtoNodeType::Endpoint as i32
        );
        assert_eq!(
            node_type_to_proto(NodeType::Database),
            ProtoNodeType::Database as i32
        );
        assert_eq!(
            node_type_to_proto(NodeType::DatabaseTable),
            ProtoNodeType::DatabaseTable as i32
        );
        assert_eq!(
            node_type_to_proto(NodeType::Queue),
            ProtoNodeType::Queue as i32
        );
        assert_eq!(
            node_type_to_proto(NodeType::Topic),
            ProtoNodeType::Topic as i32
        );
        assert_eq!(
            node_type_to_proto(NodeType::Event),
            ProtoNodeType::Event as i32
        );
        assert_eq!(
            node_type_to_proto(NodeType::ExternalService),
            ProtoNodeType::ExternalService as i32
        );
        assert_eq!(
            node_type_to_proto(NodeType::Resource),
            ProtoNodeType::Resource as i32
        );
    }

    #[test]
    fn maps_relationship_type_to_proto() {
        assert_eq!(
            relationship_type_to_proto(RelationshipType::Calls),
            ProtoRelationshipType::Calls as i32
        );
        assert_eq!(
            relationship_type_to_proto(RelationshipType::Reads),
            ProtoRelationshipType::Reads as i32
        );
        assert_eq!(
            relationship_type_to_proto(RelationshipType::Writes),
            ProtoRelationshipType::Writes as i32
        );
        assert_eq!(
            relationship_type_to_proto(RelationshipType::Publishes),
            ProtoRelationshipType::Publishes as i32
        );
        assert_eq!(
            relationship_type_to_proto(RelationshipType::Consumes),
            ProtoRelationshipType::Consumes as i32
        );
        assert_eq!(
            relationship_type_to_proto(RelationshipType::DependsOn),
            ProtoRelationshipType::DependsOn as i32
        );
    }

    #[test]
    fn maps_empty_attributes_to_none() {
        let attributes = BTreeMap::new();

        assert_eq!(map_struct(&attributes), None);
    }

    #[test]
    fn maps_scalar_attributes_to_protobuf_values() {
        let mut attributes = BTreeMap::new();

        attributes.insert("null".to_owned(), AttributeValue::Null);
        attributes.insert("bool".to_owned(), AttributeValue::Bool(true));
        attributes.insert("number".to_owned(), AttributeValue::Number(42.5));
        attributes.insert(
            "string".to_owned(),
            AttributeValue::String("changegraph".to_owned()),
        );

        let structure = map_struct(&attributes).expect("struct should exist");

        assert_eq!(
            structure
                .fields
                .get("null")
                .and_then(|value| value.kind.as_ref()),
            Some(&prost_types::value::Kind::NullValue(0))
        );

        assert_eq!(
            structure
                .fields
                .get("bool")
                .and_then(|value| value.kind.as_ref()),
            Some(&prost_types::value::Kind::BoolValue(true))
        );

        assert_eq!(
            structure
                .fields
                .get("number")
                .and_then(|value| value.kind.as_ref()),
            Some(&prost_types::value::Kind::NumberValue(42.5))
        );

        assert_eq!(
            structure
                .fields
                .get("string")
                .and_then(|value| value.kind.as_ref()),
            Some(&prost_types::value::Kind::StringValue(
                "changegraph".to_owned()
            ))
        );
    }

    #[test]
    fn maps_list_attribute_to_protobuf_list() {
        let values = vec![
            AttributeValue::String("one".to_owned()),
            AttributeValue::Number(2.0),
            AttributeValue::Bool(true),
        ];

        let value = map_value(&AttributeValue::List(values));

        match value.kind {
            Some(prost_types::value::Kind::ListValue(list)) => {
                assert_eq!(list.values.len(), 3);
            }
            _ => panic!("expected protobuf list value"),
        }
    }

    #[test]
    fn maps_object_attribute_to_protobuf_struct() {
        let mut object = BTreeMap::new();

        object.insert(
            "service".to_owned(),
            AttributeValue::String("order-service".to_owned()),
        );

        object.insert(
            "version".to_owned(),
            AttributeValue::Number(1.0),
        );

        let value = map_value(&AttributeValue::Object(object));

        match value.kind {
            Some(prost_types::value::Kind::StructValue(structure)) => {
                assert_eq!(structure.fields.len(), 2);
                assert!(structure.fields.contains_key("service"));
                assert!(structure.fields.contains_key("version"));
            }
            _ => panic!("expected protobuf struct value"),
        }
    }

    #[test]
    fn maps_positive_unix_nanoseconds_to_timestamp() {
        let timestamp = timestamp_from_unix_nanos(1_500_000_000);

        assert_eq!(timestamp.seconds, 1);
        assert_eq!(timestamp.nanos, 500_000_000);
    }

    #[test]
    fn maps_negative_unix_nanoseconds_to_timestamp() {
        let timestamp = timestamp_from_unix_nanos(-500_000_000);

        assert_eq!(timestamp.seconds, -1);
        assert_eq!(timestamp.nanos, 500_000_000);
    }

    #[test]
    fn maps_zero_unix_nanoseconds_to_timestamp() {
        let timestamp = timestamp_from_unix_nanos(0);

        assert_eq!(timestamp.seconds, 0);
        assert_eq!(timestamp.nanos, 0);
    }

    #[test]
    fn preserves_node_id() {
        let node_id = "service:order".to_owned();

        assert_eq!(GraphMapper::map_node_id(&node_id), node_id);
    }
}