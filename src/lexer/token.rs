//! Aina za tokeni kwa msomaji wa Swa.

use crate::diagnostics::SourceSpan;

/// Aina ya tokeni.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    /// Neno muhimu: `kama`, `N32`, `rudisha`, n.k.
    NenoMuhimu(String),
    /// Kitambulisho: `jumlisha`, `x`, `matokeo`, n.k.
    Kitambulisho(String),
    /// Nambari halisi (namba kamili au sehemu-desimali): `42`, `0xFF`, `3.14`.
    Nambari,
    /// Herufi halisi: `'A'`, `'\n'`. Lugha ni thamani ya N8 iliyotatuliwa.
    Herufi,
    /// Mfuato halisi: `"Habari"`. Lahaja hubeba maudhui yasiyotolewa misimbo.
    Mfuato(String),
    /// Opereta au uakifishaji: `+`, `==`, `<<=`.
    Opereta(String),
    /// Mabano ya kushoto `(`.
    MabanoKushoto,
    /// Mabano ya kulia `)`.
    MabanoKulia,
    /// Mabano ya ganda ya kushoto `{`.
    MabanoGandaKushoto,
    /// Mabano ya ganda ya kulia `}`.
    MabanoGandaKulia,
    /// Mabano ya mraba ya kushoto `[`.
    MabanoMrabaKushoto,
    /// Mabano ya mraba ya kulia `]`.
    MabanoMrabaKulia,
    /// Nukta mkato `;`.
    NuktaMkato,
    /// Koma `,`.
    Koma,
    /// Nukta `.`.
    Nukta,
    /// Nukta tatu `...`.
    NuktaTatu,
    /// Nukta mbili `:`.
    NuktaMbili,
    /// Alama ya `@` (kwa sifa ya `@kipekee`).
    Kipekee,
    /// Alama ya swali `?` (haitumiki kwa sasa, imehifadhiwa).
    AlamaSwali,
    /// Mstari wa maelekezo ya kichakachu: `#ingiza`, `#fafanua`, n.k.
    Kiunzi(String),
    /// Mwisho wa faili.
    Mwisho,
}

impl TokenKind {
    /// Bainisha aina ya tokeni kutoka kwa mfano wa kitambulisho.
    /// Hurejesha `NenoMuhimu` ikiwa ni neno muhimu la Swa, `Kitambulisho` vinginevyo.
    pub fn from_identifier(s: &str) -> Self {
        match s {
            // Type families
            "N8" | "N16" | "N32" | "N64" | "N128" => TokenKind::NenoMuhimu(s.to_string()),
            "A8" | "A16" | "A32" | "A64" | "A128" => TokenKind::NenoMuhimu(s.to_string()),
            "D16" | "D32" | "D64" | "D80" | "D128" => TokenKind::NenoMuhimu(s.to_string()),
            "B1" | "B8" | "B16" | "B32" | "B64" => TokenKind::NenoMuhimu(s.to_string()),
            "W0" | "W8" | "W16" | "W32" | "W64" => TokenKind::NenoMuhimu(s.to_string()),

            // Control flow
            "kama" | "sivyo" | "chagua" | "hali" => {
                TokenKind::NenoMuhimu(s.to_string())
            }
            "vunja" | "endelea" | "rudisha" | "nenda" => TokenKind::NenoMuhimu(s.to_string()),

            // Loops
            "kwa" | "wakati" | "fanya" => TokenKind::NenoMuhimu(s.to_string()),

            // Composite types
            "muundo" | "muungano" => TokenKind::NenoMuhimu(s.to_string()),

            // Memory
            "ukubwa" | "tenga" | "achilia" | "badili" | "nakili" => {
                TokenKind::NenoMuhimu(s.to_string())
            }

            // Module system
            "husisha" | "kutoka" => TokenKind::NenoMuhimu(s.to_string()),

            // Kitu kingine chochote ni kitambulisho.
            _ => TokenKind::Kitambulisho(s.to_string()),
        }
    }

    /// Hurejesha kweli ikiwa hii ni neno muhimu la aina.
    pub fn is_type_keyword(&self) -> bool {
        matches!(self, TokenKind::NenoMuhimu(s) if Self::is_type_name(s))
    }

    /// Angalia ikiwa mfuato unataja aina ya Swa.
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

    /// Hurejesha kweli ikiwa hii ni neno muhimu la kibainishi.
    pub fn is_qualifier(&self) -> bool {
        matches!(
            self,
            TokenKind::NenoMuhimu(s) if matches!(s.as_str(), "thabiti" | "tete" | "fiche")
        )
    }

    /// Hurejesha mfuato wa opereta ikiwa hii ni tokeni ya opereta.
    pub fn as_operator(&self) -> Option<&str> {
        match self {
            TokenKind::Opereta(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Hurejesha mfuato wa neno muhimu ikiwa hii ni tokeni ya neno muhimu.
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

/// Tokeni moja inayozalishwa na msomaji.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// Aina ya tokeni hii.
    pub kind: TokenKind,
    /// Msimbo asilia wa chanzo.
    pub lexeme: String,
    /// Sehemu ya chanzo.
    pub span: SourceSpan,
}

impl Token {
    /// Unda tokeni mpya.
    pub fn new(kind: TokenKind, lexeme: String, span: SourceSpan) -> Self {
        Self {
            kind,
            lexeme,
            span,
        }
    }

    /// Hurejesha kweli ikiwa tokeni hii inalingana na neno muhimu lililotolewa.
    pub fn is_keyword(&self, kw: &str) -> bool {
        matches!(&self.kind, TokenKind::NenoMuhimu(s) if s == kw)
    }

    /// Hurejesha kweli ikiwa tokeni hii ni kitambulisho (si neno muhimu).
    pub fn is_identifier(&self) -> bool {
        matches!(&self.kind, TokenKind::Kitambulisho(_))
    }

    /// Hurejesha jina la kitambulisho ikiwa hiki ni kitambulisho.
    pub fn as_identifier(&self) -> Option<&str> {
        match &self.kind {
            TokenKind::Kitambulisho(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Hurejesha kweli ikiwa tokeni hii ni neno muhimu la aina.
    pub fn is_type(&self) -> bool {
        self.kind.is_type_keyword()
    }
}
