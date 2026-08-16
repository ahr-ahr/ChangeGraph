//! Telemetry ingestion domain modules.
//!
//! This module exposes the normalized trace and span models used by the
//! ChangeGraph core ingestion layer.

pub mod span;
pub mod trace;

pub use span::{Span, SpanId, SpanKind};
pub use trace::{Trace, TraceId};
