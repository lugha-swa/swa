//! Mchanganuzi (msomaji) kwa lugha ya programu ya Swa.
//!
//! Hubadilisha maandishi ghafi ya chanzo kuwa mkondo wa [`Token`]z.
//! Huruka nafasi na maoni (`//` na `/* */`).
//! Hufuatilia nafasi za mstari / safu kwa kuripoti makosa kupitia [`SourceSpan`].

pub mod token;
pub use token::{Token, TokenKind};

use crate::diagnostics::SourceSpan;
use crate::diagnostics::SourceLocation;

// ---------------------------------------------------------------------------
// Mchanganuzi (Lexer)
// ---------------------------------------------------------------------------

/// Mchanganuzi wa mkondo kwa msimbo chanzo wa Swa.
///
/// Mchanganuzi hushikilia kumbukumbu ya maandishi chanzo na kusonga mbele
/// herufi kwa herufi, ikitoa [`Vec<Token>`] inapohitajika.
pub struct Lexer<'a> {
    /// Maandishi kamili ya chanzo yanayochanganuliwa.
    source: &'a str,
    /// Kikifuatio cha (kukabilia_baiti, herufi) kwa chanzo kilichosalia.
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    /// Kukabilia kwa baiti mara *baada* ya herufi ya mwisho iliyorejeshwa na [`advance`].
    byte_pos: usize,
    /// Nambari ya sasa ya mstari (kuanzia 1).
    line: usize,
    /// Nambari ya sasa ya safu (kukabilia kwa baiti kutoka mwanzo wa mstari).
    column: usize,
}

// -- API ya umma ------------------------------------------------------------

impl<'a> Lexer<'a> {
    /// Unda mchanganuzi mpya kwa maandishi chanzo yaliyotolewa.
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
            byte_pos: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tumia mchanganuzi na urejeshe kila tokeni (ikiwemo [`TokenKind::Mwisho`]).
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

// -- kitanzi kikuu -------------------------------------------------------------

impl<'a> Lexer<'a> {
    /// Toa tokeni inayofuata kutoka kwenye chanzo.
    fn next_token(&mut self) -> Token {
        // Meza nafasi na maoni; hazitoi tokeni yoyote.
        self.skip_whitespace_and_comments();

        // Rekodi mahali tokeni hii inaanzia.
        let start_line = self.line;
        let start_col = self.column;
        let start_byte = self.byte_pos;

        // Mwisho wa faili?
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

        // Tuma kwa herufi ya kwanza.
        match c {
            // --- vitambulisho & maneno funguo ---------------------------------------
            'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier_or_keyword(start_line, start_col, start_byte),

            // --- vitendanishi vya nambari ----------------------------------------------
            '0'..='9' => self.lex_number(start_line, start_col, start_byte),

            // --- kitendanishi cha mfuatano ------------------------------------------------
            '"' => self.lex_string(start_line, start_col, start_byte),

            // --- kitendanishi cha herufi ---------------------------------------------
            '\'' => self.lex_char(start_line, start_col, start_byte),

            // --- elekezo la kichakato awali ----------------------------------------
            '#' => self.lex_preprocessor(start_line, start_col, start_byte),

            // --- alama ya sifa -----------------------------------------------
            '@' => {
                let _ = self.advance();
                self.make_token(TokenKind::Kipekee, start_line, start_col, start_byte)
            }

            // --- alama swali iliyohifadhiwa ----------------------------------------
            '?' => {
                let _ = self.advance();
                self.make_token(TokenKind::AlamaSwali, start_line, start_col, start_byte)
            }

            // --- waendeshaji & alama za uakifishaji ---------------------------------------
            _ => self.lex_operator(start_line, start_col, start_byte),
        }
    }
}

// -- visaidizi ----------------------------------------------------------------

impl<'a> Lexer<'a> {
    /// Chungulia herufi ya sasa bila kuitumia.
    #[inline]
    fn current(&mut self) -> Option<char> {
        self.chars.peek().map(|&(_, ch)| ch)
    }

    /// Chungulia herufi moja mbele (herufi ya pili).
    fn peek_next(&mut self) -> Option<char> {
        let mut iter = self.chars.clone();
        iter.next(); // ruka ya sasa
        iter.next().map(|(_, ch)| ch)
    }

    /// Songa herufi moja mbele, ukisasisha utunzaji wa mstari / safu / kukabilia kwa baiti.
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

    /// Songa mbele wakati `predicate` inashikilia.  Hurejesha herufi ya kwanza
    /// iliyoshindwa kwenye predicate (ikiwepo), iliyotumiwa tayari.
    fn advance_while(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some(c) = self.current() {
            if !predicate(c) {
                break;
            }
            self.advance();
        }
    }

    /// Jenga span kamili kutoka `(mstari_wa_kuanzia, safu_ya_kuanzia)` hadi herufi ya *mwisho*
    /// iliyotumiwa (yaani, inarudi nyuma moja kutoka safu ya sasa
    /// ya "uandishi unaofuata").
    fn make_span(&self, start_line: usize, start_col: usize, start_byte: usize) -> SourceSpan {
        let (end_line, end_col) = if self.byte_pos == start_byte {
            // Tokeni yenye urefu-sifuri (mf. Mwisho wa Faili).
            (start_line, start_col)
        } else if self.column == 1 && self.line > start_line {
            // Herufi ya mwisho ilikuwa mstari mpya.
            (self.line - 1, 1)
        } else {
            (self.line, self.column.saturating_sub(1))
        };
        SourceSpan::new(
            SourceLocation::new(start_line, start_col),
            SourceLocation::new(end_line, end_col),
        )
    }

    /// Jenga tokeni inayojitanua kutoka `(mstari_wa_kuanzia, safu_ya_kuanzia)` hadi herufi
    /// ya mwisho iliyotumiwa.
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

    /// Jenga span inayofunika nafasi ya sasa.
    #[allow(dead_code)]
    fn current_span(&self) -> SourceSpan {
        SourceSpan::point(self.line, self.column)
    }

    /// Rudisha kukabilia kwa baiti kwa herufi tunayochungulia ijayo (yaani,
    /// mwanzo wa tokeni inayofuata baada ya kuruka nafasi).
    #[allow(dead_code)]
    #[inline]
    fn current_byte(&mut self) -> usize {
        self.chars.peek().map(|&(off, _)| off).unwrap_or(self.source.len())
    }
}

// -- nafasi & maoni -------------------------------------------------

impl<'a> Lexer<'a> {
    /// Ruka nafasi za usawa na aina zote mbili za maoni (`//`, `/* */`).
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.current() {
                // Nafasi za usawa.
                Some(' ' | '\t' | '\r') => {
                    self.advance();
                }
                // Mstari mpya — songa mbele ili ufuatiliaji wa mstari ubaki sahihi.
                Some('\n') => {
                    self.advance();
                }
                // Maoni ya mstari: ruka hadi mstari mpya au Mwisho wa Faili.
                Some('/') if self.peek_next() == Some('/') => {
                    self.skip_line_comment();
                }
                // Maoni ya kitalu: ruka hadi `*/` au Mwisho wa Faili.
                Some('/') if self.peek_next() == Some('*') => {
                    self.skip_block_comment();
                }
                _ => break,
            }
        }
    }

    /// Tumia maoni ya mstari ya `//` (`//` ya ufunguzi tayari imeonekana).
    fn skip_line_comment(&mut self) {
        // Meza herufi zote `/`.
        self.advance(); // / ya kwanza
        self.advance(); // / ya pili
        while let Some(c) = self.current() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    /// Tumia maoni ya kitalu ya `/* ... */` (`/*` ya ufunguzi tayari imeonekana).
    fn skip_block_comment(&mut self) {
        // Meza `/*` ya ufunguzi.
        self.advance(); // /
        self.advance(); // *
        let mut depth: u32 = 1; // wezesha upachikaji
        while depth > 0 {
            match self.advance() {
                None => break, // maoni yasiyokamilishwa — mwisho wa faili
                Some('/') => {
                    if self.current() == Some('*') {
                        self.advance(); // tumia *
                        depth += 1;
                    }
                }
                Some('*') => {
                    if self.current() == Some('/') {
                        self.advance(); // tumia /
                        depth -= 1;
                    }
                }
                _ => {}
            }
        }
    }
}

// -- kitambulisho / neno funguo --------------------------------------------------

impl<'a> Lexer<'a> {
    /// Changanua kitambulisho au neno funguo linaloanza kwenye nafasi iliyotolewa.
    /// Herufi ya kwanza (herufi au kistari cha chini) **haijatumiwa** bado.
    fn lex_identifier_or_keyword(
        &mut self,
        start_line: usize,
        start_col: usize,
        start_byte: usize,
    ) -> Token {
        // Soma [a-zA-Z_][a-zA-Z0-9_]*
        self.advance_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'));

        let lexeme = &self.source[start_byte..self.byte_pos];
        let kind = TokenKind::from_identifier(lexeme);
        let span = self.make_span(start_line, start_col, start_byte);
        Token::new(kind, lexeme.to_string(), span)
    }
}

// -- vitendanishi vya nambari ------------------------------------------------------

impl<'a> Lexer<'a> {
    /// Changanua kitendanishi cha nambari (namba kamili au sehemu) kinachoanza kwenye nafasi iliyotolewa.
    fn lex_number(
        &mut self,
        start_line: usize,
        start_col: usize,
        start_byte: usize,
    ) -> Token {
        let first = self.current().unwrap();

        // Shughulikia viambishi awali vya radiksi: 0x 0X 0o 0O 0b 0B
        if first == '0' {
            self.advance(); // tumia '0'
            match self.current() {
                Some('x' | 'X') => {
                    self.advance(); // tumia x
                    self.advance_while(|c| matches!(c, '0'..='9' | 'a'..='f' | 'A'..='F' | '_'));
                    return self.make_number_token(start_line, start_col, start_byte);
                }
                Some('o' | 'O') => {
                    self.advance(); // tumia o
                    self.advance_while(|c| matches!(c, '0'..='7' | '_'));
                    return self.make_number_token(start_line, start_col, start_byte);
                }
                Some('b' | 'B') => {
                    self.advance(); // tumia b
                    self.advance_while(|c| matches!(c, '0'..='1' | '_'));
                    return self.make_number_token(start_line, start_col, start_byte);
                }
                // '0' tupu — angukia chini kwenye ushughulikiaji wa desimali/float hapa chini.
                _ => {}
            }
        } else {
            // Nambari inayoongoza isiyo-sifuri — tumia sehemu iliyosalia ya namba kamili.
            self.advance(); // tumia nambari ya kwanza
        }

        // Nambari za desimali (ikijumuisha vistari vya chini).
        self.advance_while(|c| matches!(c, '0'..='9' | '_'));

        // Sehemu ya hisia hiari: . ikifuatiwa na nambari (si `.` pekee,
        // kwa sababu `.` ni tokeni tofauti kwa ufikiaji wa uga).
        let mut saw_fraction = false;
        if self.current() == Some('.') && self.peek_next().map_or(false, |c| c.is_ascii_digit()) {
            self.advance(); // tumia .
            self.advance_while(|c| matches!(c, '0'..='9' | '_'));
            saw_fraction = true;
        }

        // Sehemu ya kipeo hiari: e au E, ikifuatiwa na + au - kwa hiari, kisha nambari.
        if matches!(self.current(), Some('e' | 'E')) {
            self.advance(); // tumia e/E
            if matches!(self.current(), Some('+' | '-')) {
                self.advance();
            }
            self.advance_while(|c| matches!(c, '0'..='9' | '_'));
        }

        // Kiambishi cha float `f` cha hiari — kinamaana tu baada ya hisia au kipeo.
        if matches!(self.current(), Some('f' | 'F')) && saw_fraction {
            self.advance();
        }

        self.make_number_token(start_line, start_col, start_byte)
    }

    /// Kisaidizi: jenga tokeni ya nambari kutoka kwenye span iliyorekodiwa.
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

// -- kitendanishi cha mfuatano --------------------------------------------------------

impl<'a> Lexer<'a> {
    /// Changanua kitendanishi cha mfuatano chenye alama za kunukuu mbili, ikijumuisha usindikaji wa mfuatano wa kutoroka.
    fn lex_string(
        &mut self,
        start_line: usize,
        start_col: usize,
        start_byte: usize,
    ) -> Token {
        self.advance(); // " ya ufunguzi

        let mut content = String::new();

        loop {
            match self.advance() {
                None => {
                    // Mfuatano usiokamilishwa — toa tuliyonayo.
                    let raw = self.source[start_byte..self.byte_pos].to_string();
                    let span = self.make_span(start_line, start_col, start_byte);
                    return Token::new(TokenKind::Mfuato(content), raw, span);
                }
                Some('"') => break, // kunukuu kwa kufunga
                Some('\\') => {
                    // Rekodi nafasi ghafi ya backslash kwa kutoroka kwa herufi nyingi.
                    match self.advance() {
                        None => break, // Mwisho wa Faili baada ya backslash
                        Some(esc) => {
                            let resolved = Self::resolve_escape(esc);
                            content.push(resolved);
                        }
                    }
                }
                Some(c) => {
                    // Mistari mipya inaruhusiwa katika mifuatano ya Swa (mifuatano ya mistari mingi).
                    content.push(c);
                }
            }
        }

        let raw = self.source[start_byte..self.byte_pos].to_string();
        let span = self.make_span(start_line, start_col, start_byte);
        Token::new(TokenKind::Mfuato(content), raw, span)
    }

    /// Suluhisha mfuatano wa kutoroka wa herufi moja kwa thamani yake iliyofasiriwa.
    fn resolve_escape(c: char) -> char {
        match c {
            'n'  => '\n',
            't'  => '\t',
            'r'  => '\r',
            '\\' => '\\',
            '\'' => '\'',
            '"'  => '"',
            '0'  => '\0',
            'a'  => '\x07', // tahadhari / kengele
            'b'  => '\x08', // backspace
            'v'  => '\x0B', // tabu wima
            'f'  => '\x0C', // malisho ya ukurasa
            // Kutoroka kwa heksa: \xNN — mpigaji lazima achungulie mbele kwa nambari mbili za heksadesimali.
            // Kwa urahisi tunashughulikia \x kama 'x' halisi ikiwa haifuatwi na
            // nambari mbili za heksadesimali; mkaguzi / sema anaweza kuboresha hili baadaye.
            'x'  => 'x',   // kutoroka kwa heksa kunashughulikiwa na lex_char; mifuatano hupita tu
            other => other, // kutoroka kusikojulikana hupita kihalisi
        }
    }
}

// -- kitendanishi cha herufi -----------------------------------------------------

impl<'a> Lexer<'a> {
    /// Changanua kitendanishi cha herufi chenye kunukuu moja: `'A'`, `'\n'`, `'\x41'`.
    fn lex_char(
        &mut self,
        start_line: usize,
        start_col: usize,
        start_byte: usize,
    ) -> Token {
        self.advance(); // ' ya ufunguzi

        // Soma herufi (inayowezekana imetoroka).
        match self.advance() {
            None => {
                // Kitendanishi cha herufi kisichokamilishwa.
                let raw = self.source[start_byte..self.byte_pos].to_string();
                let span = self.make_span(start_line, start_col, start_byte);
                return Token::new(TokenKind::Herufi, raw, span);
            }
            Some('\\') => {
                // Herufi iliyotoroka.
                match self.advance() {
                    None => {} // Mwisho wa Faili baada ya backslash — angukia chini
                    Some('x') => {
                        // \xNN kutoroka kwa heksa — tumia hadi nambari mbili za heksadesimali.
                        let _ = self.try_consume_hex_digits(2);
                    }
                    Some(_esc) => {
                        // Kutoroka kwa herufi moja tayari kumetumiwa na advance().
                    }
                }
            }
            Some('\'') => {
                // Kitendanishi tupu cha herufi '' — usitumie chochote kingine.
            }
            Some(_c) => {
                // Herufi wazi — tayari imetumiwa.
            }
        }

        // Tumia kunukuu kwa kufunga ikiwa kipo.
        if self.current() == Some('\'') {
            self.advance();
        }

        let raw = self.source[start_byte..self.byte_pos].to_string();
        let span = self.make_span(start_line, start_col, start_byte);
        Token::new(TokenKind::Herufi, raw, span)
    }

    /// Jaribu kutumia hadi `count` nambari za heksadesimali.  Hurejesha ngapi zilitumiwa.
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

// -- waendeshaji & uakifishaji -----------------------------------------------

impl<'a> Lexer<'a> {
    /// Changanua kiendeshi, kitenganishi, au alama nyingine yoyote ya uakifishaji ya ASCII.
    fn lex_operator(
        &mut self,
        start_line: usize,
        start_col: usize,
        start_byte: usize,
    ) -> Token {
        let c = self.advance().unwrap(); // mpigaji amehakikisha kuna herufi

        let kind = match c {
            // --- vitenganishi vya herufi-moja (bila kuangalia mbele) ----------------------
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

            // --- nukta — inaweza kuwa mwanzo wa nukta tatu au nukta tu -----------------
            '.' => {
                match (self.current(), self.peek_next()) {
                    (Some('.'), Some('.')) => {
                        self.advance(); // nukta ya pili
                        self.advance(); // nukta ya tatu
                        TokenKind::NuktaTatu
                    }
                    _ => TokenKind::Nukta,
                }
            }

            // --- waendeshaji wa herufi-nyingi wanaoanza na `+` -----------------------
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

            // --- `~` (kukanusha kwa biti ya pekee tu, hakuna fomu mchanganyiko) ----------------
            '~' => TokenKind::Opereta("~".to_string()),

            // --- kila kitu kingine kinachukuliwa kama kiendeshi kisichojulikana -------------
            other => TokenKind::Opereta(other.to_string()),
        };

        self.make_token(kind, start_line, start_col, start_byte)
    }

    /// Jaribu kiendeshi mchanganyiko cha herufi-mbili: `c + second` au `c + '='`.
    /// Ikiwa hakuna kinacholingana, toa kiendeshi cha herufi-moja.
    fn lex_compound_op(&mut self, first: char, second: char, eq: char) -> TokenKind {
        let next = self.current();
        if next == Some(second) {
            // mf. `++`, `&&`, `||`, `**`
            self.advance();
            let s = format!("{}{}", first, second);
            TokenKind::Opereta(s)
        } else if next == Some(eq) {
            // mf. `+=`, `&=`, `|=`
            self.advance();
            let s = format!("{}{}", first, eq);
            TokenKind::Opereta(s)
        } else {
            TokenKind::Opereta(first.to_string())
        }
    }

    /// Jaribu waendeshaji mchanganyiko ambapo herufi ya kwanza inaweza kurudiwa
    /// (`<<`, `<=`, `<<=`, `>>`, `>=`, `>>=`) au kuunda `->`.
    fn lex_compound_op3(&mut self, first: char, second: char, eq: char) -> TokenKind {
        let next = self.current();

        // `->` (mshale) — kwa `-` pekee.
        if first == '-' && next == Some('>') {
            self.advance();
            return TokenKind::Opereta("->".to_string());
        }

        if next == Some(second) {
            // mf. `<<`, `>>`
            self.advance();
            // Angalia kwa `<<=` au `>>=`.
            if self.current() == Some(eq) {
                self.advance();
                let s = format!("{}{}{}", first, second, eq);
                TokenKind::Opereta(s)
            } else {
                let s = format!("{}{}", first, second);
                TokenKind::Opereta(s)
            }
        } else if next == Some(eq) {
            // mf. `<=`, `>=`
            self.advance();
            let s = format!("{}{}", first, eq);
            TokenKind::Opereta(s)
        } else {
            TokenKind::Opereta(first.to_string())
        }
    }
}

// -- elekezo la kichakato awali ------------------------------------------------

impl<'a> Lexer<'a> {
    /// Changanua elekezo la kichakato awali: `#` likifuatiwa na sehemu iliyosalia ya mstari.
    ///
    /// Hutoa tokeni ya [`TokenKind::Kiunzi`].  Maudhui ya elekezo huanza
    /// baada ya `#` na kuendelea hadi mwisho wa mstari (`\r` ya mwisho huondolewa).
    fn lex_preprocessor(
        &mut self,
        start_line: usize,
        start_col: usize,
        start_byte: usize,
    ) -> Token {
        // Tumia `#`.
        self.advance();

        // Tumia hadi mwisho wa mstari au Mwisho wa Faili.
        while let Some(c) = self.current() {
            if c == '\n' {
                break;
            }
            self.advance();
        }

        // Jenga maudhui ya elekezo (lexemu bila `#`).
        let full_lexeme = self.source[start_byte..self.byte_pos].to_string();
        // Maudhui ni kila kitu baada ya `#`.
        let content = full_lexeme[1..].trim_end_matches('\r').to_string();

        let span = self.make_span(start_line, start_col, start_byte);
        Token::new(TokenKind::Kiunzi(content), full_lexeme, span)
    }
}

// ---------------------------------------------------------------------------
// Vipimo
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Kisaidizi: kusanya mifuatano ya onyesho ya tokeni zote zisizo za Mwisho.
    fn token_strings(source: &str) -> Vec<String> {
        let lexer = Lexer::new(source);
        lexer
            .tokenize()
            .into_iter()
            .filter(|t| t.kind != TokenKind::Mwisho)
            .map(|t| format!("{}", t.kind))
            .collect()
    }

    /// Kisaidizi: kusanya jozi (kind_debug, lexeme).
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
        // Nukta moja ni tokeni ya ufikiaji wa uga / uakifishaji.
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
        // Elekezo la kichakato awali ni tokeni moja, kisha `kama`.
        assert_eq!(kinds.len(), 2);
        assert!(kinds[0].contains("ingiza"));
        assert_eq!(kinds[1], "kama");
    }

    #[test]
    fn test_span_tracking() {
        let lexer = Lexer::new("kama rudisha");
        let tokens = lexer.tokenize();
        // kama: mstari 1 safu 1-4
        assert_eq!(tokens[0].span.start.line, 1);
        assert_eq!(tokens[0].span.start.column, 1);
        assert_eq!(tokens[0].span.end.column, 4);
        // rudisha: mstari 1 safu 6-12
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
