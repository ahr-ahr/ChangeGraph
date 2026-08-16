//! Graph aggregate for ChangeGraph.
//!
//! The graph owns nodes and directed edges while keeping the domain model
//! independent from transport and storage implementations.

use std::collections::BTreeMap;

use super::edge::Edge;
use super::node::{Node, NodeId};

/// Errors that can occur while mutating a graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphError {
    /// A node with the same stable identifier already exists.
    DuplicateNode(NodeId),

    /// An edge references a source node that does not exist.
    SourceNodeNotFound(NodeId),

    /// An edge references a target node that does not exist.
    TargetNodeNotFound(NodeId),
}

/// A directed dependency graph composed of nodes and edges.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Graph {
    nodes: BTreeMap<NodeId, Node>,
    edges: Vec<Edge>,
}

impl Graph {
    /// Creates an empty graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns whether a node with the given identifier exists.
    pub fn contains_node(&self, node_id: &NodeId) -> bool {
        self.nodes.contains_key(node_id)
    }

    /// Adds a node to the graph.
    pub fn add_node(&mut self, node: Node) -> Result<(), GraphError> {
        if self.nodes.contains_key(&node.id) {
            return Err(GraphError::DuplicateNode(node.id));
        }

        self.nodes.insert(node.id.clone(), node);

        Ok(())
    }

    /// Returns a node by stable identifier.
    pub fn node(&self, node_id: &NodeId) -> Option<&Node> {
        self.nodes.get(node_id)
    }

    /// Returns an iterator over all nodes.
    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    /// Adds an edge when both endpoint nodes exist.
    pub fn add_edge(&mut self, edge: Edge) -> Result<(), GraphError> {
        if !self.nodes.contains_key(&edge.source_node_id) {
            return Err(GraphError::SourceNodeNotFound(edge.source_node_id));
        }

        if !self.nodes.contains_key(&edge.target_node_id) {
            return Err(GraphError::TargetNodeNotFound(edge.target_node_id));
        }

        self.edges.push(edge);

        Ok(())
    }

    /// Returns an iterator over all edges.
    pub fn edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.iter()
    }

    /// Returns all outgoing edges from a node.
    pub fn outgoing_edges(&self, node_id: &NodeId) -> impl Iterator<Item = &Edge> {
        self.edges
            .iter()
            .filter(move |edge| &edge.source_node_id == node_id)
    }

    /// Returns all incoming edges to a node.
    pub fn incoming_edges(&self, node_id: &NodeId) -> impl Iterator<Item = &Edge> {
        self.edges
            .iter()
            .filter(move |edge| &edge.target_node_id == node_id)
    }
}
