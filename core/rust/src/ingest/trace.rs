//! OpenTelemetry trace ingestion domain model.
//!
//! This module represents a normalized trace at the ChangeGraph core boundary.
//! Transport-specific representations such as OTLP are mapped into this model
//! before graph relationships are derived.

/// Stable trace identifier.
pub type TraceId = String;

/// A normalized OpenTelemetry trace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trace {
    /// OpenTelemetry trace identifier.
    pub trace_id: TraceId,

    /// Spans belonging to this trace.
    ///
    /// Span details are introduced by the ingestion span model and are kept
    /// out of this initial trace foundation.
    pub span_ids: Vec<String>,
}

impl Trace {
    /// Creates an empty trace with the given identifier.
    pub fn new(trace_id: impl Into<TraceId>) -> Self {
        Self {
            trace_id: trace_id.into(),
            span_ids: Vec::new(),
        }
    }

    /// Adds a span identifier to the trace.
    pub fn add_span(&mut self, span_id: impl Into<String>) {
        self.span_ids.push(span_id.into());
    }

    /// Returns the number of spans associated with the trace.
    pub fn span_count(&self) -> usize {
        self.span_ids.len()
    }
}
