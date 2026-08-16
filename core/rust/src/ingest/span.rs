//! OpenTelemetry span ingestion domain model.
//!
//! This module represents a normalized span at the ChangeGraph core boundary.
//! Transport-specific telemetry such as OTLP is mapped into this model before
//! ingestion and graph relationship derivation.

use std::collections::BTreeMap;

use super::trace::TraceId;

/// Stable span identifier.
pub type SpanId = String;

/// Kind of operation represented by a span.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpanKind {
    Internal,
    Server,
    Client,
    Producer,
    Consumer,
}

impl SpanKind {
    /// Returns the canonical OpenTelemetry-compatible kind name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Internal => "internal",
            Self::Server => "server",
            Self::Client => "client",
            Self::Producer => "producer",
            Self::Consumer => "consumer",
        }
    }
}

/// Normalized OpenTelemetry span.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    /// OpenTelemetry trace identifier.
    pub trace_id: TraceId,

    /// OpenTelemetry span identifier.
    pub span_id: SpanId,

    /// Parent span identifier, when available.
    pub parent_span_id: Option<SpanId>,

    /// Operation name.
    pub name: String,

    /// Kind of span operation.
    pub kind: SpanKind,

    /// Start timestamp as Unix time in nanoseconds.
    pub start_time_unix_nanos: i64,

    /// End timestamp as Unix time in nanoseconds.
    pub end_time_unix_nanos: i64,

    /// Runtime and telemetry attributes.
    pub attributes: BTreeMap<String, String>,
}

impl Span {
    /// Creates a normalized span.
    pub fn new(
        trace_id: impl Into<TraceId>,
        span_id: impl Into<SpanId>,
        name: impl Into<String>,
        kind: SpanKind,
        start_time_unix_nanos: i64,
        end_time_unix_nanos: i64,
    ) -> Self {
        Self {
            trace_id: trace_id.into(),
            span_id: span_id.into(),
            parent_span_id: None,
            name: name.into(),
            kind,
            start_time_unix_nanos,
            end_time_unix_nanos,
            attributes: BTreeMap::new(),
        }
    }

    /// Sets the parent span identifier.
    pub fn with_parent_span_id(mut self, parent_span_id: impl Into<SpanId>) -> Self {
        self.parent_span_id = Some(parent_span_id.into());
        self
    }

    /// Adds or replaces a string attribute.
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Returns the span duration in nanoseconds.
    pub fn duration_nanos(&self) -> i64 {
        self.end_time_unix_nanos
            .saturating_sub(self.start_time_unix_nanos)
    }
}
