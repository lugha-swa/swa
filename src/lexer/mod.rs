//! Lexer (msomaji) for the Swa programming language.
//!
//! Converts raw source text into a stream of [`Token`]s.
//! Skips whitespace and comments (`//` and `/* */`).
//! Tracks line / column positions for error reporting via [`SourceSpan`].

pub mod token;
pub use token::{Token, TokenKind};

use crate::diagnostics::SourceSpan;
use crate::diagnostics::SourceLocation;

// ---------------------------------------------------------------------------
// Lexer
// ---------------------------------------------------------------------------

/// A streaming tokeniser for Swa source code.
///
/// The lexer holds a reference to the source text and advances through it
/// character by character, producing a [`Vec<Token>`] on demand.
pub struct Lexer<'a> {
    /// The full source text being tokenised.
    source: &'a str,
    /// Iterator over (byte_offset, char) for the remaining source.
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    /// Byte offset immediately *after* the last character returned by [`advance`].
    byte_pos: usize,
    /// Current 1-based line number.
    line: usize,
    /// Current 1-based column number (byte offset from start of line).
    column: usize,
}

// -- public API ------------------------------------------------------------

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given source text.
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
            byte_pos: 0,
            line: 1,
            column: 1,
        }
    }

    /// Consume the lexer and return every token (including [`TokenKind::Mwisho`]).
    pub fn tokenize(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let done = tok.kind == TokenKind::Mwisho;
            tokens.push(tok);
            if done {
                break;
            }
        }
        tokens
    }
}

// -- core loop -------------------------------------------------------------

impl<'a> Lexer<'a> {
    /// Extract the next token from the source.
    fn next_token(&mut self) -> Token {
        // Swallow whitespace and comments; they produce no token.
        self.skip_whitespace_and_comments();

        // Record where this token starts.
        let start_line = self.line;
        let start_col = self.column;
        let start_byte = self.byte_pos;

        // End of file?
        let c = match self.current() {
            Some(ch) => ch,
            None => {
                return Token::new(
                    TokenKind::Mwisho,
                    String::new(),
                    SourceSpan::point(self.line, self.column),
                );
            }
        };

        // Dispatch on the first character.
        match c {
            // --- identifiers & keywords ---------------------------------------
            'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier_or_keyword(start_line, start_col, start_byte),

            // --- numeric literals ----------------------------------------------
            '0'..='9' => self.lex_number(start_line, start_col, start_byte),

            // --- string literal ------------------------------------------------
            '"' => self.lex_string(start_line, start_col, start_byte),

            // --- character literal ---------------------------------------------
            '\'' => self.lex_char(start_line, start_col, start_byte),

            // --- preprocessor directive ----------------------------------------
            '#' => self.lex_preprocessor(start_line, start_col, start_byte),

            // --- attribute sigil -----------------------------------------------
            '@' => {
                let _ = self.advance();
                self.make_token(TokenKind::Kipekee, start_line, start_col, start_byte)
            }

            // --- reserved question mark ----------------------------------------
            '?' => {
                let _ = self.advance();
                self.make_token(TokenKind::AlamaSwali, start_line, start_col, start_byte)
            }

            // --- operators & punctuation ---------------------------------------
            _ => self.lex_operator(start_line, start_col, start_byte),
        }
    }
}

// -- helpers ----------------------------------------------------------------

impl<'a> Lexer<'a> {
    /// Peek at the current character without consuming it.
    #[inline]
    fn current(&mut self) -> Option<char> {
        self.chars.peek().map(|&(_, ch)| ch)
    }

    /// Peek one character ahead (the second character).
    fn peek_next(&mut self) -> Option<char> {
        let mut iter = self.chars.clone();
        iter.next(); // skip current
        iter.next().map(|(_, ch)| ch)
    }

    /// Advance one character, updating line / column / byte-offset bookkeeping.
    fn advance(&mut self) -> Option<char> {
        let (offset, c) = self.chars.next()?;
        self.byte_pos = offset + c.len_utf8();
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(c)
    }

    /// Advance while `predicate` holds.  Returns the first character that
    /// failed the predicate (if any), already consumed.
    fn advance_while(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some(c) = self.current() {
            if !predicate(c) {
                break;
            }
            self.advance();
        }
    }

    /// Build an inclusive span from `(start_line, start_col)` to the *last*
    /// character that was consumed (i.e. backs up one from the current
    /// "next write" column).
    fn make_span(&self, start_line: usize, start_col: usize, start_byte: usize) -> SourceSpan {
        let (end_line, end_col) = if self.byte_pos == start_byte {
            // Zero-length token (e.g. EOF).
            (start_line, start_col)
        } else if self.column == 1 && self.line > start_line {
            // Last character was a newline.
            (self.line - 1, 1)
        } else {
            (self.line, self.column.saturating_sub(1))
        };
        SourceSpan::new(
            SourceLocation::new(start_line, start_col),
            SourceLocation::new(end_line, end_col),
        )
    }

    /// Build a token spanning from `(start_line, start_col)` to the last
    /// consumed character.
    fn make_token(
        &self,
        kind: TokenKind,
        start_line: usize,
        start_col: usize,
        start_byte: usize,
    ) -> Token {
        let lexeme = self.source[start_byte..self.byte_pos].to_string();
        let span = self.make_span(start_line, start_col, start_byte);
        Token::new(kind, lexeme, span)
    }

    /// Build a span covering the current position.
    #[allow(dead_code)]
    fn current_span(&self) -> SourceSpan {
        SourceSpan::point(self.line, self.column)
    }

    /// Return the byte offset of the character we would peek next (i.e. the
    /// start of the next token after whitespace skipping).
    #[allow(dead_code)]
    #[inline]
    fn current_byte(&mut self) -> usize {
        self.chars.peek().map(|&(off, _)| off).unwrap_or(self.source.len())
    }
}

// -- whitespace & comments -------------------------------------------------

impl<'a> Lexer<'a> {
    /// Skip over horizontal whitespace and both styles of comment (`//`, `/* */`).
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.current() {
                // Horizontal whitespace.
                Some(' ' | '\t' | '\r') => {
                    self.advance();
                }
                // Newline — advance so line tracking stays accurate.
                Some('\n') => {
                    self.advance();
                }
                // Line comment: skip until newline or EOF.
                Some('/') if self.peek_next() == Some('/') => {
                    self.skip_line_comment();
                }
                // Block comment: skip until `*/` or EOF.
                Some('/') if self.peek_next() == Some('*') => {
                    self.skip_block_comment();
                }
                _ => break,
            }
        }
    }

    /// Consume a `//` line comment (the opening `//` has already been seen).
    fn skip_line_comment(&mut self) {
        // Eat both `/` characters.
        self.advance(); // first  /
        self.advance(); // second /
        while let Some(c) = self.current() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    /// Consume a `/* ... */` block comment (the opening `/*` has already been seen).
    fn skip_block_comment(&mut self) {
        // Eat opening `/*`.
        self.advance(); // /
        self.advance(); // *
        let mut depth: u32 = 1; // support nesting
        while depth > 0 {
            match self.advance() {
                None => break, // unterminated comment — end of file
                Some('/') => {
                    if self.current() == Some('*') {
                        self.advance(); // consume *
                        depth += 1;
                    }
                }
                Some('*') => {
                    if self.current() == Some('/') {
                        self.advance(); // consume /
                        depth -= 1;
                    }
                }
                _ => {}
            }
        }
    }
}

// -- identifier / keyword --------------------------------------------------

impl<'a> Lexer<'a> {
    /// Lex an identifier or keyword starting at the given position.
    /// The first character (letter or underscore) has **not** been consumed yet.
    fn lex_identifier_or_keyword(
        &mut self,
        start_line: usize,
        start_col: usize,
        start_byte: usize,
    ) -> Token {
        // Read [a-zA-Z_][a-zA-Z0-9_]*
        self.advance_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'));

        let lexeme = &self.source[start_byte..self.byte_pos];
        let kind = TokenKind::from_identifier(lexeme);
        let span = self.make_span(start_line, start_col, start_byte);
        Token::new(kind, lexeme.to_string(), span)
    }
}

// -- numeric literals ------------------------------------------------------

impl<'a> Lexer<'a> {
    /// Lex a numeric literal (integer or float) starting at the given position.
    fn lex_number(
        &mut self,
        start_line: usize,
        start_col: usize,
        start_byte: usize,
    ) -> Token {
        let first = self.current().unwrap();

        // Handle radix prefixes: 0x 0X 0o 0O 0b 0B
        if first == '0' {
            self.advance(); // consume '0'
            match self.current() {
                Some('x' | 'X') => {
                    self.advance(); // consume x
                    self.advance_while(|c| matches!(c, '0'..='9' | 'a'..='f' | 'A'..='F' | '_'));
                    return self.make_number_token(start_line, start_col, start_byte);
                }
                Some('o' | 'O') => {
                    self.advance(); // consume o
                    self.advance_while(|c| matches!(c, '0'..='7' | '_'));
                    return self.make_number_token(start_line, start_col, start_byte);
                }
                Some('b' | 'B') => {
                    self.advance(); // consume b
                    self.advance_while(|c| matches!(c, '0'..='1' | '_'));
                    return self.make_number_token(start_line, start_col, start_byte);
                }
                // Plain '0' — fall through to decimal/float handling below.
                _ => {}
            }
        } else {
            // Leading non-zero digit — consume the rest of the integer part.
            self.advance(); // consume first digit
        }

        // Decimal digits (including underscores).
        self.advance_while(|c| matches!(c, '0'..='9' | '_'));

        // Optional fractional part: . followed by a digit (not `.` alone,
        // because `.` is a separate token for field access).
        let mut saw_fraction = false;
        if self.current() == Some('.') && self.peek_next().map_or(false, |c| c.is_ascii_digit()) {
            self.advance(); // consume .
            self.advance_while(|c| matches!(c, '0'..='9' | '_'));
            saw_fraction = true;
        }

        // Optional exponent part: e or E, optionally followed by + or -, then digits.
        if matches!(self.current(), Some('e' | 'E')) {
            self.advance(); // consume e/E
            if matches!(self.current(), Some('+' | '-')) {
                self.advance();
            }
            self.advance_while(|c| matches!(c, '0'..='9' | '_'));
        }

        // Optional float suffix `f` — only meaningful after a fraction or exponent.
        if matches!(self.current(), Some('f' | 'F')) && saw_fraction {
            self.advance();
        }

        self.make_number_token(start_line, start_col, start_byte)
    }

    /// Helper: build a numeric token from the recorded span.
    fn make_number_token(
        &self,
        start_line: usize,
        start_col: usize,
        start_byte: usize,
    ) -> Token {
        let lexeme = self.source[start_byte..self.byte_pos].to_string();
        let span = self.make_span(start_line, start_col, start_byte);
        Token::new(TokenKind::Nambari, lexeme, span)
    }
}

// -- string literal --------------------------------------------------------

impl<'a> Lexer<'a> {
    /// Lex a double-quoted string literal, including escape-sequence processing.
    fn lex_string(
        &mut self,
        start_line: usize,
        start_col: usize,
        start_byte: usize,
    ) -> Token {
        self.advance(); // opening "

        let mut content = String::new();

        loop {
            match self.advance() {
                None => {
                    // Unterminated string — produce what we have.
                    let raw = self.source[start_byte..self.byte_pos].to_string();
                    let span = self.make_span(start_line, start_col, start_byte);
                    return Token::new(TokenKind::Mfuato(content), raw, span);
                }
                Some('"') => break, // closing quote
                Some('\\') => {
                    // Record the raw backslash position for multi-char escapes.
                    match self.advance() {
                        None => break, // EOF after backslash
                        Some(esc) => {
                            let resolved = Self::resolve_escape(esc);
                            content.push(resolved);
                        }
                    }
                }
                Some(c) => {
                    // Newlines are allowed in Swa strings (multi-line strings).
                    content.push(c);
                }
            }
        }

        let raw = self.source[start_byte..self.byte_pos].to_string();
        let span = self.make_span(start_line, start_col, start_byte);
        Token::new(TokenKind::Mfuato(content), raw, span)
    }

    /// Resolve a single-character escape sequence to its interpreted value.
    fn resolve_escape(c: char) -> char {
        match c {
            'n'  => '\n',
            't'  => '\t',
            'r'  => '\r',
            '\\' => '\\',
            '\'' => '\'',
            '"'  => '"',
            '0'  => '\0',
            'a'  => '\x07', // alert / bell
            'b'  => '\x08', // backspace
            'v'  => '\x0B', // vertical tab
            'f'  => '\x0C', // form feed
            // Hex escape: \xNN — the caller must peek ahead for two hex digits.
            // For simplicity we handle \x as a literal 'x' if not followed by
            // two hex digits; the parser / sema may later refine this.
            'x'  => 'x',   // hex escapes are handled by lex_char; strings pass through
            other => other, // unknown escapes pass through literally
        }
    }
}

// -- character literal -----------------------------------------------------

impl<'a> Lexer<'a> {
    /// Lex a single-quoted character literal: `'A'`, `'\n'`, `'\x41'`.
    fn lex_char(
        &mut self,
        start_line: usize,
        start_col: usize,
        start_byte: usize,
    ) -> Token {
        self.advance(); // opening '

        // Read the (possibly escaped) character.
        match self.advance() {
            None => {
                // Unterminated char literal.
                let raw = self.source[start_byte..self.byte_pos].to_string();
                let span = self.make_span(start_line, start_col, start_byte);
                return Token::new(TokenKind::Herufi, raw, span);
            }
            Some('\\') => {
                // Escaped character.
                match self.advance() {
                    None => {} // EOF after backslash — fall through
                    Some('x') => {
                        // \xNN hex escape — consume up to two hex digits.
                        let _ = self.try_consume_hex_digits(2);
                    }
                    Some(_esc) => {
                        // Single-character escape already consumed by advance().
                    }
                }
            }
            Some('\'') => {
                // Empty character literal '' — consume nothing else.
            }
            Some(_c) => {
                // Plain character — already consumed.
            }
        }

        // Consume the closing quote if present.
        if self.current() == Some('\'') {
            self.advance();
        }

        let raw = self.source[start_byte..self.byte_pos].to_string();
        let span = self.make_span(start_line, start_col, start_byte);
        Token::new(TokenKind::Herufi, raw, span)
    }

    /// Try to consume up to `count` hex digits.  Returns how many were consumed.
    fn try_consume_hex_digits(&mut self, count: usize) -> usize {
        let mut n = 0;
        while n < count {
            match self.current() {
                Some(c) if c.is_ascii_hexdigit() => {
                    self.advance();
                    n += 1;
                }
                _ => break,
            }
        }
        n
    }
}

// -- operators & punctuation -----------------------------------------------

impl<'a> Lexer<'a> {
    /// Lex an operator, delimiter, or any other ASCII punctuation.
    fn lex_operator(
        &mut self,
        start_line: usize,
        start_col: usize,
        start_byte: usize,
    ) -> Token {
        let c = self.advance().unwrap(); // caller guaranteed there is a char

        let kind = match c {
            // --- single-char delimiters (lookahead-free) ----------------------
            '(' => TokenKind::MabanoKushoto,
            ')' => TokenKind::MabanoKulia,
            '{' => TokenKind::MabanoGandaKushoto,
            '}' => TokenKind::MabanoGandaKulia,
            '[' => TokenKind::MabanoMrabaKushoto,
            ']' => TokenKind::MabanoMrabaKulia,
            ';' => TokenKind::NuktaMkato,
            ',' => TokenKind::Koma,
            ':' => TokenKind::NuktaMbili,
            '@' => TokenKind::Kipekee,
            '?' => TokenKind::AlamaSwali,

            // --- dot — may be start of ellipsis or just a dot -----------------
            '.' => {
                match (self.current(), self.peek_next()) {
                    (Some('.'), Some('.')) => {
                        self.advance(); // second dot
                        self.advance(); // third  dot
                        TokenKind::NuktaTatu
                    }
                    _ => TokenKind::Nukta,
                }
            }

            // --- multi-char operators starting with `+` -----------------------
            '+' => self.lex_compound_op(c, '+', '='),

            // --- `-` ----------------------------------------------------------
            '-' => self.lex_compound_op3(c, '>', '='),

            // --- `*` ----------------------------------------------------------
            '*' => self.lex_compound_op(c, '*', '='),

            // --- `/` ----------------------------------------------------------
            '/' => self.lex_compound_op(c, '/', '='),

            // --- `%` ----------------------------------------------------------
            '%' => self.lex_compound_op(c, '%', '='),

            // --- `=` ----------------------------------------------------------
            '=' => self.lex_compound_op(c, '=', '='),

            // --- `!` ----------------------------------------------------------
            '!' => self.lex_compound_op(c, '!', '='),

            // --- `<` ----------------------------------------------------------
            '<' => self.lex_compound_op3(c, '<', '='),

            // --- `>` ----------------------------------------------------------
            '>' => self.lex_compound_op3(c, '>', '='),

            // --- `&` ----------------------------------------------------------
            '&' => self.lex_compound_op(c, '&', '='),

            // --- `|` ----------------------------------------------------------
            '|' => self.lex_compound_op(c, '|', '='),

            // --- `^` ----------------------------------------------------------
            '^' => self.lex_compound_op(c, '^', '='),

            // --- `~` (only unary bitwise-not, no compound form) ----------------
            '~' => TokenKind::Opereta("~".to_string()),

            // --- everything else is treated as an unknown operator -------------
            other => TokenKind::Opereta(other.to_string()),
        };

        self.make_token(kind, start_line, start_col, start_byte)
    }

    /// Try a two-character compound operator: `c + second` or `c + '='`.
    /// If neither matches, produce a single-char operator.
    fn lex_compound_op(&mut self, first: char, second: char, eq: char) -> TokenKind {
        let next = self.current();
        if next == Some(second) {
            // e.g. `++`, `&&`, `||`, `**`
            self.advance();
            let s = format!("{}{}", first, second);
            TokenKind::Opereta(s)
        } else if next == Some(eq) {
            // e.g. `+=`, `&=`, `|=`
            self.advance();
            let s = format!("{}{}", first, eq);
            TokenKind::Opereta(s)
        } else {
            TokenKind::Opereta(first.to_string())
        }
    }

    /// Try compound operators where the first character can be repeated
    /// (`<<`, `<=`, `<<=`, `>>`, `>=`, `>>=`) or form `->`.
    fn lex_compound_op3(&mut self, first: char, second: char, eq: char) -> TokenKind {
        let next = self.current();

        // `->` (arrow) — only for `-`.
        if first == '-' && next == Some('>') {
            self.advance();
            return TokenKind::Opereta("->".to_string());
        }

        if next == Some(second) {
            // e.g. `<<`, `>>`
            self.advance();
            // Check for `<<=` or `>>=`.
            if self.current() == Some(eq) {
                self.advance();
                let s = format!("{}{}{}", first, second, eq);
                TokenKind::Opereta(s)
            } else {
                let s = format!("{}{}", first, second);
                TokenKind::Opereta(s)
            }
        } else if next == Some(eq) {
            // e.g. `<=`, `>=`
            self.advance();
            let s = format!("{}{}", first, eq);
            TokenKind::Opereta(s)
        } else {
            TokenKind::Opereta(first.to_string())
        }
    }
}

// -- preprocessor directive ------------------------------------------------

impl<'a> Lexer<'a> {
    /// Lex a preprocessor directive: `#` followed by the rest of the line.
    ///
    /// Produces a [`TokenKind::Kiunzi`] token.  The directive content starts
    /// after the `#` and runs to the end of the line (trailing `\r` is stripped).
    fn lex_preprocessor(
        &mut self,
        start_line: usize,
        start_col: usize,
        start_byte: usize,
    ) -> Token {
        // Consume `#`.
        self.advance();

        // Consume until end of line or EOF.
        while let Some(c) = self.current() {
            if c == '\n' {
                break;
            }
            self.advance();
        }

        // Build the directive content (the lexeme without the `#`).
        let full_lexeme = self.source[start_byte..self.byte_pos].to_string();
        // The content is everything after the `#`.
        let content = full_lexeme[1..].trim_end_matches('\r').to_string();

        let span = self.make_span(start_line, start_col, start_byte);
        Token::new(TokenKind::Kiunzi(content), full_lexeme, span)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: collect the display strings of all non-Mwisho tokens.
    fn token_strings(source: &str) -> Vec<String> {
        let lexer = Lexer::new(source);
        lexer
            .tokenize()
            .into_iter()
            .filter(|t| t.kind != TokenKind::Mwisho)
            .map(|t| format!("{}", t.kind))
            .collect()
    }

    /// Helper: collect (kind_debug, lexeme) pairs.
    fn token_debug(source: &str) -> Vec<(String, String)> {
        let lexer = Lexer::new(source);
        lexer
            .tokenize()
            .into_iter()
            .filter(|t| t.kind != TokenKind::Mwisho)
            .map(|t| (format!("{:?}", t.kind), t.lexeme.clone()))
            .collect()
    }

    #[test]
    fn test_empty_input() {
        let lexer = Lexer::new("");
        let tokens = lexer.tokenize();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Mwisho);
    }

    #[test]
    fn test_whitespace_only() {
        let lexer = Lexer::new("  \t\n\r\n  ");
        let tokens = lexer.tokenize();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Mwisho);
    }

    #[test]
    fn test_keywords() {
        let src = "kama sivyo kwa wakati rudisha kweli uongo tupu";
        let kinds: Vec<String> = token_strings(src);
        assert_eq!(
            kinds,
            vec![
                "kama", "sivyo", "kwa", "wakati", "rudisha", "kweli", "uongo", "tupu"
            ]
        );
    }

    #[test]
    fn test_type_keywords() {
        let src = "N8 N16 N32 N64 N128 A8 A16 A32 A64 fiche";
        let kinds: Vec<String> = token_strings(src);
        assert!(kinds.contains(&"N32".to_string()));
        assert!(kinds.contains(&"fiche".to_string()));
    }

    #[test]
    fn test_identifiers() {
        let src = "jumlisha x matokeo _private _123 foo_bar";
        let kinds: Vec<String> = token_strings(src);
        assert_eq!(
            kinds,
            vec!["jumlisha", "x", "matokeo", "_private", "_123", "foo_bar"]
        );
    }

    #[test]
    fn test_decimal_integers() {
        let src = "0 42 100 1_000_000";
        let kinds: Vec<String> = token_strings(src);
        assert_eq!(kinds, vec!["nambari"; 4]);
    }

    #[test]
    fn test_hex_integers() {
        let src = "0xFF 0XDEAD 0x0 0x1A2B";
        let kinds: Vec<String> = token_strings(src);
        assert_eq!(kinds, vec!["nambari"; 4]);
    }

    #[test]
    fn test_octal_integers() {
        let src = "0o77 0O10 0o0";
        let kinds: Vec<String> = token_strings(src);
        assert_eq!(kinds, vec!["nambari"; 3]);
    }

    #[test]
    fn test_binary_integers() {
        let src = "0b1010 0B0 0b1_0000";
        let kinds: Vec<String> = token_strings(src);
        assert_eq!(kinds, vec!["nambari"; 3]);
    }

    #[test]
    fn test_floats() {
        let src = "3.14 0.5 1.0e10 2.5E-3 1.5f";
        let kinds: Vec<String> = token_strings(src);
        assert_eq!(kinds, vec!["nambari"; 5]);
    }

    #[test]
    fn test_dot_vs_float_distinction() {
        // Single dot is a field-access / punctuation token.
        let src = "a.b 3.14 .";
        let kinds: Vec<String> = token_strings(src);
        // a . b 3.14 .
        assert_eq!(kinds, vec!["a", ".", "b", "nambari", "."]);
    }

    #[test]
    fn test_string_literal() {
        let src = r#""Habari"  "Jina \"Mimi\"""#;
        let kinds: Vec<String> = token_strings(src);
        assert_eq!(kinds, vec!["mfuatano", "mfuatano"]);
    }

    #[test]
    fn test_char_literal() {
        let src = r#"'A' '\n' '\x41' '\0'"#;
        let kinds: Vec<String> = token_strings(src);
        assert_eq!(kinds, vec!["herufi"; 4]);
    }

    #[test]
    fn test_operators() {
        let src = "+ - * / % = == != < > <= >= << >> && || !";
        let lexer = Lexer::new(src);
        let tokens = lexer.tokenize();
        let ops: Vec<String> = tokens
            .iter()
            .filter(|t| matches!(&t.kind, TokenKind::Opereta(_)))
            .map(|t| t.lexeme.clone())
            .collect();
        assert_eq!(
            ops,
            vec![
                "+", "-", "*", "/", "%", "=", "==", "!=", "<", ">", "<=", ">=",
                "<<", ">>", "&&", "||", "!",
            ]
        );
    }

    #[test]
    fn test_compound_assignment_operators() {
        let src = "+= -= *= /= %= &= |= ^= <<= >>=";
        let lexer = Lexer::new(src);
        let tokens = lexer.tokenize();
        let ops: Vec<String> = tokens
            .iter()
            .filter(|t| matches!(&t.kind, TokenKind::Opereta(_)))
            .map(|t| t.lexeme.clone())
            .collect();
        assert_eq!(
            ops,
            vec!["+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>="]
        );
    }

    #[test]
    fn test_delimiters() {
        let src = "( ) { } [ ] ; , : . ... @ ?";
        let kinds: Vec<String> = token_strings(src);
        assert_eq!(
            kinds,
            vec!["(", ")", "{", "}", "[", "]", ";", ",", ":", ".", "...", "@", "?"]
        );
    }

    #[test]
    fn test_line_comment() {
        let src = "kama // hiki ni maelezo\nrudisha";
        let kinds: Vec<String> = token_strings(src);
        assert_eq!(kinds, vec!["kama", "rudisha"]);
    }

    #[test]
    fn test_block_comment() {
        let src = "kama /* block */ rudisha";
        let kinds: Vec<String> = token_strings(src);
        assert_eq!(kinds, vec!["kama", "rudisha"]);
    }

    #[test]
    fn test_nested_block_comment() {
        let src = "/* outer /* inner */ still */ x";
        let kinds: Vec<String> = token_strings(src);
        assert_eq!(kinds, vec!["x"]);
    }

    #[test]
    fn test_preprocessor_directive() {
        let src = "#ingiza \"moduli.c\"\nkama";
        let kinds: Vec<String> = token_strings(src);
        // The preprocessor directive is one token, then `kama`.
        assert_eq!(kinds.len(), 2);
        assert!(kinds[0].contains("ingiza"));
        assert_eq!(kinds[1], "kama");
    }

    #[test]
    fn test_span_tracking() {
        let lexer = Lexer::new("kama rudisha");
        let tokens = lexer.tokenize();
        // kama: line 1 col 1-4
        assert_eq!(tokens[0].span.start.line, 1);
        assert_eq!(tokens[0].span.start.column, 1);
        assert_eq!(tokens[0].span.end.column, 4);
        // rudisha: line 1 col 6-12
        assert_eq!(tokens[1].span.start.column, 6);
        assert_eq!(tokens[1].span.end.column, 12);
    }

    #[test]
    fn test_multiline_span() {
        let src = "\"hello\nworld\"";
        let lexer = Lexer::new(src);
        let tokens = lexer.tokenize();
        let span = tokens[0].span;
        assert_eq!(span.start.line, 1);
        assert_eq!(span.end.line, 2);
    }

    #[test]
    fn test_complex_program() {
        let src = r#"
// Mfano: chaguo la kukokotoa
kama x > 0 {
    rudisha x * 2;
} sivyo {
    rudisha 0;
}
"#;
        let kinds: Vec<String> = token_strings(src);
        assert_eq!(
            kinds,
            vec![
                "kama", "x", ">", "nambari", "{",
                "rudisha", "x", "*", "nambari", ";",
                "}", "sivyo", "{",
                "rudisha", "nambari", ";",
                "}",
            ]
        );
    }

    #[test]
    fn test_arrow_operator() {
        let src = "muundo { aina * -> N32 }";
        let kinds: Vec<String> = token_strings(src);
        assert!(kinds.contains(&"->".to_string()));
    }

    #[test]
    fn test_attribute_sigil() {
        let src = "@kipekee fanya jambo()";
        let kinds: Vec<String> = token_strings(src);
        assert_eq!(kinds[0], "@");
        assert_eq!(kinds[1], "kipekee");
    }

    #[test]
    fn test_underscore_number_separators() {
        let src = "1_000_000 0xFF_FF 0b1010_0101";
        let kinds: Vec<String> = token_strings(src);
        assert_eq!(kinds, vec!["nambari"; 3]);
    }
}
