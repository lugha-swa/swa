//! Utambuzi wa mkusanyaji, maeneo ya chanzo, na vipindi.

/// Nafasi inayoanzia 1 (mstari, safu) katika maandishi ya chanzo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceLocation {
    /// Nambari ya mstari (kuanzia 1).
    pub line: usize,
    /// Nambari ya safu (kianzio cha baiti kutoka mwanzo wa mstari).
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

/// Kipindi kinachoendelea cha maandishi ya chanzo kati ya mwanzo na mwisho.
///
/// Mipaka yote miwili imojumuishwa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceSpan {
    /// Mwanzo wa kipindi (mwigizwamo).
    pub start: SourceLocation,
    /// Mwisho wa kipindi (mwigizwamo).
    pub end: SourceLocation,
}

impl SourceSpan {
    pub fn new(start: SourceLocation, end: SourceLocation) -> Self {
        Self { start, end }
    }

    /// Unda kipindi cha upana-sifuri katika eneo moja.
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

/// Ukali wa ujumbe wa utambuzi.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    /// Kosa linalozuia ukusanyaji wenye mafanikio.
    Error,
    /// Onyo lisilo mauti.
    Warning,
    /// Maelezo ya habari yaliyounganishwa na utambuzi mwingine.
    Note,
}

/// Ujumbe mmoja wa utambuzi uliofungwa kwenye kipindi cha chanzo.
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

    /// Toa utambuzi huu kwa mstari wa panda chini ya mstari wenye kosa.
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

/// Mkusanyo wa utambuzi unaokua tu.
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

    /// Ongeza utambuzi.
    pub fn push(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }

    /// Ongeza utambuzi wa kosa.
    pub fn error(&mut self, message: impl Into<String>, span: SourceSpan) {
        self.push(Diagnostic::error(message, span));
    }

    /// Ongeza utambuzi wa onyo.
    pub fn warning(&mut self, message: impl Into<String>, span: SourceSpan) {
        self.push(Diagnostic::warning(message, span));
    }

    /// Tazama utambuzi wote uliokusanywa.
    pub fn all(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Idadi ya utambuzi uliokusanywa.
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Je, mfuko ni tupu?
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Kweli wakati angalau utambuzi mmoja wa kiwango cha kosa umerekodiwa.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| matches!(d.severity, Severity::Error))
    }
}
