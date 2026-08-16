//! Protocol integration for ChangeGraph.
//!
//! This module exposes generated Protobuf types and protocol mapping
//! utilities.

pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/changegraph.graph.v1.rs"));
}

pub mod mapper;

pub use generated::{
    Edge,
    Graph,
    Node,
    NodeType,
    Observation,
    RelationshipType,
};

pub use mapper::GraphMapper;