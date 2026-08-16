//! Graph traversal primitives for impact analysis.
//!
//! This module provides dependency traversal over the ChangeGraph graph.
//! Traversal is intentionally separate from impact scoring or risk analysis:
//! it answers which nodes are reachable from a starting node.

use std::collections::{BTreeSet, VecDeque};

use crate::graph::{Graph, NodeId};

/// Traversal errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraversalError {
    /// The requested starting node does not exist in the graph.
    StartNodeNotFound(NodeId),
}

/// Traverses the graph in the outgoing dependency direction.
///
/// Starting from `start_node_id`, every reachable target node is returned.
/// The starting node itself is not included in the result.
pub fn downstream(
    graph: &Graph,
    start_node_id: &NodeId,
) -> Result<Vec<NodeId>, TraversalError> {
    if !graph.contains_node(start_node_id) {
        return Err(TraversalError::StartNodeNotFound(start_node_id.clone()));
    }

    let mut visited = BTreeSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();

    visited.insert(start_node_id.clone());
    queue.push_back(start_node_id.clone());

    while let Some(current_node_id) = queue.pop_front() {
        for edge in graph.outgoing_edges(&current_node_id) {
            let target_node_id = &edge.target_node_id;

            if visited.insert(target_node_id.clone()) {
                result.push(target_node_id.clone());
                queue.push_back(target_node_id.clone());
            }
        }
    }

    Ok(result)
}

/// Traverses the graph in the incoming dependency direction.
///
/// Starting from `start_node_id`, every reachable source node is returned.
/// The starting node itself is not included in the result.
pub fn upstream(
    graph: &Graph,
    start_node_id: &NodeId,
) -> Result<Vec<NodeId>, TraversalError> {
    if !graph.contains_node(start_node_id) {
        return Err(TraversalError::StartNodeNotFound(start_node_id.clone()));
    }

    let mut visited = BTreeSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();

    visited.insert(start_node_id.clone());
    queue.push_back(start_node_id.clone());

    while let Some(current_node_id) = queue.pop_front() {
        for edge in graph.incoming_edges(&current_node_id) {
            let source_node_id = &edge.source_node_id;

            if visited.insert(source_node_id.clone()) {
                result.push(source_node_id.clone());
                queue.push_back(source_node_id.clone());
            }
        }
    }

    Ok(result)
}
