//! Impact analysis over the ChangeGraph dependency graph.
//!
//! This module turns graph traversal results into a structured impact report.
//! It deliberately does not assign risk scores yet. Its responsibility in
//! this stage is to identify the affected nodes and preserve traversal
//! direction.

use crate::graph::{Graph, NodeId, NodeType};

use super::traversal::{downstream, upstream, TraversalError};

/// Direction used when calculating graph impact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImpactDirection {
    Downstream,
    Upstream,
}

/// One node identified as affected by a graph change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImpactedNode {
    /// Stable identifier of the affected node.
    pub node_id: NodeId,

    /// Category of the affected node.
    pub node_type: NodeType,
}

/// Result of an impact analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImpactResult {
    /// Node from which the analysis started.
    pub source_node_id: NodeId,

    /// Direction used for traversal.
    pub direction: ImpactDirection,

    /// Nodes reachable from the source node in the selected direction.
    pub impacted_nodes: Vec<ImpactedNode>,
}

/// Errors that can occur during impact analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImpactError {
    /// The source node does not exist in the graph.
    SourceNodeNotFound(NodeId),
}

impl From<TraversalError> for ImpactError {
    fn from(error: TraversalError) -> Self {
        match error {
            TraversalError::StartNodeNotFound(node_id) => {
                Self::SourceNodeNotFound(node_id)
            }
        }
    }
}

/// Analyzes graph impact from a source node.
#[derive(Debug, Default, Clone, Copy)]
pub struct ImpactAnalyzer;

impl ImpactAnalyzer {
    /// Creates a new impact analyzer.
    pub const fn new() -> Self {
        Self
    }

    /// Calculates downstream impact.
    pub fn analyze_downstream(
        &self,
        graph: &Graph,
        source_node_id: &NodeId,
    ) -> Result<ImpactResult, ImpactError> {
        self.analyze(graph, source_node_id, ImpactDirection::Downstream)
    }

    /// Calculates upstream impact.
    pub fn analyze_upstream(
        &self,
        graph: &Graph,
        source_node_id: &NodeId,
    ) -> Result<ImpactResult, ImpactError> {
        self.analyze(graph, source_node_id, ImpactDirection::Upstream)
    }

    fn analyze(
        &self,
        graph: &Graph,
        source_node_id: &NodeId,
        direction: ImpactDirection,
    ) -> Result<ImpactResult, ImpactError> {
        let node_ids = match direction {
            ImpactDirection::Downstream => downstream(graph, source_node_id)?,
            ImpactDirection::Upstream => upstream(graph, source_node_id)?,
        };

        let impacted_nodes = node_ids
            .into_iter()
            .filter_map(|node_id| {
                graph.node(&node_id).map(|node| ImpactedNode {
                    node_id,
                    node_type: node.node_type,
                })
            })
            .collect();

        Ok(ImpactResult {
            source_node_id: source_node_id.clone(),
            direction,
            impacted_nodes,
        })
    }
}
