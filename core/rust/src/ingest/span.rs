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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_span_with_expected_fields() {
        let span = Span::new(
            "trace-1",
            "span-1",
            "http.request",
            SpanKind::Server,
            1_000,
            3_000,
        );

        assert_eq!(span.trace_id, "trace-1");
        assert_eq!(span.span_id, "span-1");
        assert_eq!(span.parent_span_id, None);
        assert_eq!(span.name, "http.request");
        assert_eq!(span.kind, SpanKind::Server);
        assert_eq!(span.start_time_unix_nanos, 1_000);
        assert_eq!(span.end_time_unix_nanos, 3_000);
        assert!(span.attributes.is_empty());
    }

    #[test]
    fn sets_parent_span_id() {
        let span = Span::new(
            "trace-1",
            "span-2",
            "db.query",
            SpanKind::Client,
            1_000,
            2_000,
        )
        .with_parent_span_id("span-1");

        assert_eq!(span.parent_span_id.as_deref(), Some("span-1"));
    }

    #[test]
    fn adds_and_replaces_attributes() {
        let span = Span::new(
            "trace-1",
            "span-1",
            "http.request",
            SpanKind::Server,
            1_000,
            2_000,
        )
        .with_attribute("service.name", "order-service")
        .with_attribute("http.method", "GET")
        .with_attribute("http.method", "POST");

        assert_eq!(
            span.attributes.get("service.name").map(String::as_str),
            Some("order-service")
        );

        assert_eq!(
            span.attributes.get("http.method").map(String::as_str),
            Some("POST")
        );
    }

    #[test]
    fn calculates_duration_in_nanoseconds() {
        let span = Span::new(
            "trace-1",
            "span-1",
            "http.request",
            SpanKind::Server,
            1_000,
            4_500,
        );

        assert_eq!(span.duration_nanos(), 3_500);
    }

    #[test]
    fn preserves_negative_duration_when_end_precedes_start() {
        let span = Span::new(
            "trace-1",
            "span-1",
            "invalid",
            SpanKind::Internal,
            5_000,
            1_000,
        );

        assert_eq!(span.duration_nanos(), -4_000);
    }

    #[test]
    fn returns_canonical_span_kind_names() {
        assert_eq!(SpanKind::Internal.as_str(), "internal");
        assert_eq!(SpanKind::Server.as_str(), "server");
        assert_eq!(SpanKind::Client.as_str(), "client");
        assert_eq!(SpanKind::Producer.as_str(), "producer");
        assert_eq!(SpanKind::Consumer.as_str(), "consumer");
    }
}