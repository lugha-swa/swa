//! Token types for the Swa lexer.

use crate::diagnostics::SourceSpan;

/// The kind of a token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    /// A keyword: `kama`, `N32`, `rudisha`, etc.
    NenoMuhimu(String),
    /// An identifier: `jumlisha`, `x`, `matokeo`, etc.
    Kitambulisho(String),
    /// A numeric literal (integer or float): `42`, `0xFF`, `3.14`.
    Nambari,
    /// A character literal: `'A'`, `'\n'`. Lexeme is the resolved N8 value.
    Herufi,
    /// A string literal: `"Habari"`. The variant carries the unescaped content.
    Mfuato(String),
    /// An operator or punctuation: `+`, `==`, `<<=`.
    Opereta(String),
    /// Left parenthesis `(`.
    MabanoKushoto,
    /// Right parenthesis `)`.
    MabanoKulia,
    /// Left brace `{`.
    MabanoGandaKushoto,
    /// Right brace `}`.
    MabanoGandaKulia,
    /// Left bracket `[`.
    MabanoMrabaKushoto,
    /// Right bracket `]`.
    MabanoMrabaKulia,
    /// Semicolon `;`.
    NuktaMkato,
    /// Comma `,`.
    Koma,
    /// Dot `.`.
    Nukta,
    /// Ellipsis `...`.
    NuktaTatu,
    /// Colon `:`.
    NuktaMbili,
    /// At sign `@` (for `@kipekee` attribute).
    Kipekee,
    /// Question mark `?` (unused for now, reserved).
    AlamaSwali,
    /// A preprocessor directive line: `#ingiza`, `#fafanua`, etc.
    Kiunzi(String),
    /// End of file.
    Mwisho,
}

impl TokenKind {
    /// Determine the token kind from an identifier-like string.
    /// Returns `NenoMuhimu` if it's a Swa keyword, `Kitambulisho` otherwise.
    pub fn from_identifier(s: &str) -> Self {
        match s {
            // Type families
            "N8" | "N16" | "N32" | "N64" | "N128" => TokenKind::NenoMuhimu(s.to_string()),
            "A8" | "A16" | "A32" | "A64" | "A128" => TokenKind::NenoMuhimu(s.to_string()),
            "D16" | "D32" | "D64" | "D80" | "D128" => TokenKind::NenoMuhimu(s.to_string()),
            "B1" | "B8" | "B16" | "B32" | "B64" => TokenKind::NenoMuhimu(s.to_string()),
            "W0" | "W8" | "W16" | "W32" | "W64" => TokenKind::NenoMuhimu(s.to_string()),

            // Qualifiers
            "thabiti" | "tete" | "fiche" => TokenKind::NenoMuhimu(s.to_string()),

            // Control flow
            "kama" | "sivyo" | "chagua" | "hali" | "kawaida" => {
                TokenKind::NenoMuhimu(s.to_string())
            }
            "vunja" | "endelea" | "rudisha" | "nenda" => TokenKind::NenoMuhimu(s.to_string()),

            // Loops
            "kwa" | "wakati" | "fanya" => TokenKind::NenoMuhimu(s.to_string()),

            // Composite types
            "muundo" | "muungano" | "orodha" | "aina" => TokenKind::NenoMuhimu(s.to_string()),

            // Memory
            "ukubwa" | "tenga" | "achilia" | "badili" | "nakili" => {
                TokenKind::NenoMuhimu(s.to_string())
            }

            // Module system
            "husisha" => TokenKind::NenoMuhimu(s.to_string()),

            // Truth values and null
            "kweli" | "uongo" | "tupu" => TokenKind::NenoMuhimu(s.to_string()),

            // Special
            "toka" => TokenKind::NenoMuhimu(s.to_string()),

            // Anything else is an identifier.
            _ => TokenKind::Kitambulisho(s.to_string()),
        }
    }

    /// Returns true if this is a type keyword.
    pub fn is_type_keyword(&self) -> bool {
        matches!(self, TokenKind::NenoMuhimu(s) if Self::is_type_name(s))
    }

    /// Check if a string names a Swa type.
    pub fn is_type_name(s: &str) -> bool {
        matches!(
            s,
            "N8" | "N16"
                | "N32"
                | "N64"
                | "N128"
                | "A8"
                | "A16"
                | "A32"
                | "A64"
                | "A128"
                | "D16"
                | "D32"
                | "D64"
                | "D80"
                | "D128"
                | "B1"
                | "B8"
                | "B16"
                | "B32"
                | "B64"
                | "W0"
                | "W8"
                | "W16"
                | "W32"
                | "W64"
        )
    }

    /// Returns true if this is a qualifier keyword.
    pub fn is_qualifier(&self) -> bool {
        matches!(
            self,
            TokenKind::NenoMuhimu(s) if matches!(s.as_str(), "thabiti" | "tete" | "fiche")
        )
    }

    /// Returns the operator string if this is an operator token.
    pub fn as_operator(&self) -> Option<&str> {
        match self {
            TokenKind::Opereta(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Returns the keyword string if this is a keyword token.
    pub fn as_keyword(&self) -> Option<&str> {
        match self {
            TokenKind::NenoMuhimu(s) => Some(s.as_str()),
            _ => None,
        }
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::NenoMuhimu(s) => write!(f, "{}", s),
            TokenKind::Kitambulisho(s) => write!(f, "{}", s),
            TokenKind::Nambari => write!(f, "nambari"),
            TokenKind::Herufi => write!(f, "herufi"),
            TokenKind::Mfuato(_) => write!(f, "mfuatano"),
            TokenKind::Opereta(s) => write!(f, "{}", s),
            TokenKind::MabanoKushoto => write!(f, "("),
            TokenKind::MabanoKulia => write!(f, ")"),
            TokenKind::MabanoGandaKushoto => write!(f, "{{"),
            TokenKind::MabanoGandaKulia => write!(f, "}}"),
            TokenKind::MabanoMrabaKushoto => write!(f, "["),
            TokenKind::MabanoMrabaKulia => write!(f, "]"),
            TokenKind::NuktaMkato => write!(f, ";"),
            TokenKind::Koma => write!(f, ","),
            TokenKind::Nukta => write!(f, "."),
            TokenKind::NuktaTatu => write!(f, "..."),
            TokenKind::NuktaMbili => write!(f, ":"),
            TokenKind::Kipekee => write!(f, "@"),
            TokenKind::AlamaSwali => write!(f, "?"),
            TokenKind::Kiunzi(s) => write!(f, "{}", s),
            TokenKind::Mwisho => write!(f, "<mwisho>"),
        }
    }
}

/// A single token produced by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The kind of this token.
    pub kind: TokenKind,
    /// The original source text.
    pub lexeme: String,
    /// Source span.
    pub span: SourceSpan,
}

impl Token {
    /// Create a new token.
    pub fn new(kind: TokenKind, lexeme: String, span: SourceSpan) -> Self {
        Self {
            kind,
            lexeme,
            span,
        }
    }

    /// Returns true if this token matches the given keyword.
    pub fn is_keyword(&self, kw: &str) -> bool {
        matches!(&self.kind, TokenKind::NenoMuhimu(s) if s == kw)
    }

    /// Returns true if this token is an identifier (not a keyword).
    pub fn is_identifier(&self) -> bool {
        matches!(&self.kind, TokenKind::Kitambulisho(_))
    }

    /// Returns the identifier name if this is an identifier.
    pub fn as_identifier(&self) -> Option<&str> {
        match &self.kind {
            TokenKind::Kitambulisho(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Returns true if this token is a type keyword.
    pub fn is_type(&self) -> bool {
        self.kind.is_type_keyword()
    }
}
