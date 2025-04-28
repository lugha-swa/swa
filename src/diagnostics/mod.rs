//! Compiler diagnostics, source locations, and spans.

/// A 1-based position (line, column) in source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceLocation {
    /// 1-based line number.
    pub line: usize,
    /// 1-based column number (byte offset from line start).
    pub column: usize,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// A contiguous span of source text between a start and end location.
///
/// Both bounds are inclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceSpan {
    /// Start of the span (inclusive).
    pub start: SourceLocation,
    /// End of the span (inclusive).
    pub end: SourceLocation,
}

impl SourceSpan {
    pub fn new(start: SourceLocation, end: SourceLocation) -> Self {
        Self { start, end }
    }

    /// Create a zero-width span at a single location.
    pub fn point(line: usize, column: usize) -> Self {
        let loc = SourceLocation::new(line, column);
        Self::new(loc, loc)
    }
}

impl std::fmt::Display for SourceSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.start.line == self.end.line && self.start.column == self.end.column {
            write!(f, "{}", self.start)
        } else if self.start.line == self.end.line {
            write!(f, "{}:{}-{}", self.start.line, self.start.column, self.end.column)
        } else {
            write!(f, "{}-{}", self.start, self.end)
        }
    }
}

/// Severity of a diagnostic message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    /// An error that prevents successful compilation.
    Error,
    /// A non-fatal warning.
    Warning,
    /// An informational note attached to another diagnostic.
    Note,
}

/// A single diagnostic message bound to a source span.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub span: SourceSpan,
}

impl Diagnostic {
    pub fn new(severity: Severity, message: impl Into<String>, span: SourceSpan) -> Self {
        Self {
            severity,
            message: message.into(),
            span,
        }
    }

    pub fn error(message: impl Into<String>, span: SourceSpan) -> Self {
        Self::new(Severity::Error, message, span)
    }

    pub fn warning(message: impl Into<String>, span: SourceSpan) -> Self {
        Self::new(Severity::Warning, message, span)
    }

    pub fn note(message: impl Into<String>, span: SourceSpan) -> Self {
        Self::new(Severity::Note, message, span)
    }

    /// Render this diagnostic with a caret underline beneath the offending line.
    pub fn render(&self, source: &str) -> String {
        let sev_str = match self.severity {
            Severity::Error => "hitilafu",
            Severity::Warning => "onyo",
            Severity::Note => "kumbuka",
        };

        let line_idx = self.span.start.line.saturating_sub(1);
        let source_line = source.lines().nth(line_idx).unwrap_or("<mwisho>");

        let col = self.span.start.column.saturating_sub(1);
        let width = if self.span.start.line == self.span.end.line {
            (self.span.end.column.saturating_sub(self.span.start.column) + 1).max(1)
        } else {
            1
        };

        let caret_pad = " ".repeat(col);
        let carets = "^".repeat(width);

        format!(
            "{}:{}: {}: {}\n  {}\n  {}{}",
            self.span.start, sev_str, sev_str, self.message,
            source_line,
            caret_pad, carets,
        )
    }
}

/// A grow-only collection of diagnostics.
#[derive(Debug, Clone, Default)]
pub struct DiagnosticBag {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticBag {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    /// Append a diagnostic.
    pub fn push(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }

    /// Append an error diagnostic.
    pub fn error(&mut self, message: impl Into<String>, span: SourceSpan) {
        self.push(Diagnostic::error(message, span));
    }

    /// Append a warning diagnostic.
    pub fn warning(&mut self, message: impl Into<String>, span: SourceSpan) {
        self.push(Diagnostic::warning(message, span));
    }

    /// View all accumulated diagnostics.
    pub fn all(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Number of diagnostics collected.
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Whether the bag is empty.
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// True when at least one error-level diagnostic has been recorded.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| matches!(d.severity, Severity::Error))
    }
}
