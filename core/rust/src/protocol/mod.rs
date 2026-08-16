//! Protocol integration for ChangeGraph.
//!
//! This module exposes generated Protobuf types and protocol mapping
//! utilities.

pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/changegraph.graph.v1.rs"));
}

pub mod mapper;

pub use generated::{Edge, Graph, Node, Observation};

pub use mapper::{
    GraphMapper,
    ProtocolEdge,
    ProtocolGraph,
    ProtocolNode,
    ProtocolObservation,
    ProtocolValue,
};