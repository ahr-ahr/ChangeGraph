//! Impact analysis modules.
//!
//! This module exposes graph traversal and impact analysis primitives.

pub mod analyzer;
pub mod traversal;

pub use analyzer::{
    ImpactAnalyzer, ImpactDirection, ImpactError, ImpactResult, ImpactedNode,
};
pub use traversal::{downstream, upstream, TraversalError};
