//! Swa parser — mirrors msambazaji.swa.
//!
//! Parses tokens from the lexer into a flat-array AST consumable by
//! `ir::lower::lower()`.

use crate::lexer::token::{Token, TokenKind};

const AST_PROGRAMU: u32 = 1;
const AST_KAZI: u32 = 2;
const AST_RUDISHA: u32 = 3;
const AST_NAMBARI: u32 = 4;
const AST_KITAMBULISHO: u32 = 5;
const AST_JUMLISHA: u32 = 6;
const AST_TOFAUTI: u32 = 7;
const AST_WITO: u32 = 8;
const AST_KAMA: u32 = 9;
const AST_WAKATI: u32 = 10;
const AST_TANGAZO: u32 = 11;
const AST_MUUNDO: u32 = 12;
const AST_SEHEMU: u32 = 13;
const AST_CHAGUA: u32 = 14;
const AST_HALI: u32 = 15;
const AST_VUNJA: u32 = 16;
const AST_ENDELEA: u32 = 17;
const AST_TENGA: u32 = 18;
const AST_ACHILIA: u32 = 19;
const AST_SAWA: u32 = 20;
const AST_TOFAUTI_SI: u32 = 21;
const AST_CHINI: u32 = 22;
const AST_JUU: u32 = 23;
const AST_CHINI_SAWA: u32 = 24;
const AST_JUU_SAWA: u32 = 25;
const AST_TAJA: u32 = 29;
const AST_KUMBUKA: u32 = 30;
const AST_NA: u32 = 26;
const AST_AU: u32 = 27;
const AST_SI: u32 = 28;
const AST_ZIDISHA: u32 = 31;
const AST_GAWANYA: u32 = 32;
const AST_SEHEMU_DOT: u32 = 33;
const AST_SEHEMU_MSHALE: u32 = 34;
const AST_TANGAZO_ULIMWENGU: u32 = 35;
const AST_HAMISHA_KUSHOTO: u32 = 36;
const AST_HAMISHA_KULIA: u32 = 39;
const AST_BIT_AU: u32 = 41;
const AST_BIT_NA: u32 = 42;
const AST_TERNARY: u32 = 43;
const AST_ASIMILIA: u32 = 37;
const AST_SAFU: u32 = 38;
const AST_MFUATANO: u32 = 40;
const NO_NODE: i32 = -1;

// ---------------------------------------------------------------------------
// AST builder — accumulates flat arrays
// ---------------------------------------------------------------------------
struct AstBuilder {
    aina: Vec<u32>,
    thamani: Vec<i32>,
    kushoto: Vec<i32>,
    kulia: Vec<i32>,
    tiga: Vec<i32>,
    nne: Vec<i32>,
    jina_off: Vec<i32>,
    pool: Vec<u8>,
}

impl AstBuilder {
    fn new() -> Self {
        Self {
            aina: Vec::new(), thamani: Vec::new(),
            kushoto: Vec::new(), kulia: Vec::new(),
            tiga: Vec::new(), nne: Vec::new(),
            jina_off: Vec::new(), pool: vec![0],
        }
    }

    fn node_mpya(&mut self, aina: u32, thamani: i32, kushoto: i32, kulia: i32) -> i32 {
        let idx = self.aina.len() as i32;
        self.aina.push(aina); self.thamani.push(thamani);
        self.kushoto.push(kushoto); self.kulia.push(kulia);
        self.tiga.push(NO_NODE); self.nne.push(NO_NODE);
        self.jina_off.push(0);
        idx
    }

    fn hifadhi_jina(&mut self, node: i32, name: &str) {
        let off = self.pool.len();
        self.jina_off[node as usize] = off as i32;
        self.pool.extend_from_slice(name.as_bytes());
        self.pool.push(0);
    }
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------
struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
    ast: AstBuilder,
    kosa: bool,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0, ast: AstBuilder::new(), kosa: false }
    }

    fn sasa(&self) -> &Token {
        if self.pos < self.tokens.len() { &self.tokens[self.pos] }
        else { &self.tokens[self.tokens.len().saturating_sub(1)] }
    }

    fn sogeza(&mut self) { if self.pos < self.tokens.len() { self.pos += 1; } }

    fn tokeni_ni(&self, s: &str) -> bool {
        let t = self.sasa();
        match &t.kind {
            TokenKind::NenoMuhimu(k) => k == s,
            TokenKind::Kitambulisho(k) => k == s,
            TokenKind::Opereta(o) => o == s,
            _ => t.lexeme == s,
        }
    }

    fn ni_aina(&self) -> bool {
        match &self.sasa().kind {
            TokenKind::NenoMuhimu(s) => s.as_bytes().first().map_or(false, |c| c.is_ascii_uppercase()),
            TokenKind::Kitambulisho(s) => s.as_bytes().first().map_or(false, |c| c.is_ascii_uppercase()),
            _ => false,
        }
    }

    fn changanua_aina(&mut self) -> i32 {
        // Returns encoded type as i32 (high bits: family, low bits: width + mshale flag)
        if !self.ni_aina() { return 0; }
        let txt = self.sasa().lexeme.clone();
        let n = txt.len();
        let (familia, upana): (u32, u32) = if n >= 2 && txt[1..].chars().all(|c| c.is_ascii_digit()) {
            let c0 = txt.as_bytes()[0];
            let fam = match c0 { b'N' => 1, b'A'|b'S' => 2, b'D' => 3, b'B' => 4, b'W' => 5, b'H' => 2, b'K' => 4, _ => 6 };
            let w = txt[1..].parse().unwrap_or(32);
            (fam, w)
        } else {
            // User-defined struct type — store name in pool and return negative offset.
            let name_off = self.ast.pool.len() as i32;
            self.ast.pool.extend_from_slice(txt.as_bytes());
            self.ast.pool.push(0);
            self.sogeza();
            // Skip pointer handling for user types (they'll have * after the name).
            let mut mshale: u32 = 0;
            while self.tokeni_ni("*") || self.tokeni_ni("**") || self.tokeni_ni("***") {
                mshale += self.sasa().lexeme.len() as u32;
                self.sogeza();
            }
            return if mshale != 0 { -(name_off | 1) } else { -name_off };
        };
        self.sogeza();
        let mut mshale: u32 = 0;
        // Handle pointer chains: *, **, ***, etc. The lexer may tokenize
        // "**" as a single operator token.
        while self.tokeni_ni("*") || self.tokeni_ni("**") || self.tokeni_ni("***") {
            mshale += self.sasa().lexeme.len() as u32;
            self.sogeza();
        }
        (((familia & 255) << 8) | (upana & 255) | (mshale & 1)) as i32
    }

    // -- expression parser (precedence climbing) -----------------------------

    fn changanua_primary(&mut self) -> i32 {
        match &self.sasa().kind {
            TokenKind::Nambari => {
                let v: i32 = self.sasa().lexeme.parse().unwrap_or(0);
                self.sogeza();
                self.ast.node_mpya(AST_NAMBARI, v, NO_NODE, NO_NODE)
            }
            TokenKind::Mfuato(_) => {
                let s = self.sasa().lexeme.clone();
                let n = self.ast.node_mpya(AST_MFUATANO, 0, NO_NODE, NO_NODE);
                self.ast.hifadhi_jina(n, &s);
                self.sogeza();
                n
            }
            TokenKind::Kitambulisho(_) | TokenKind::NenoMuhimu(_) => {
                let name = self.sasa().lexeme.clone();
                self.sogeza();
                if self.tokeni_ni("(") {
                    self.sogeza();
                    let call = self.ast.node_mpya(AST_WITO, 0, NO_NODE, NO_NODE);
                    let mut first: i32 = NO_NODE; let mut prev: i32 = NO_NODE;
                    if !self.tokeni_ni(")") { loop {
                        let a = self.changanua_usemi();
                        if prev == NO_NODE { first = a; } else { self.ast.nne[prev as usize] = a; }
                        prev = a;
                        if self.tokeni_ni(",") { self.sogeza(); continue; } else { break; }
                    }}
                    if self.tokeni_ni(")") { self.sogeza(); }
                    let name_n = self.ast.node_mpya(AST_KITAMBULISHO, 0, NO_NODE, NO_NODE);
                    self.ast.hifadhi_jina(name_n, &name);
                    self.ast.kushoto[call as usize] = name_n;
                    self.ast.kulia[name_n as usize] = first;
                    return call;
                }
                let n = self.ast.node_mpya(AST_KITAMBULISHO, 0, NO_NODE, NO_NODE);
                self.ast.hifadhi_jina(n, &name);
                n
            }
            TokenKind::MabanoKushoto => { self.sogeza(); let e = self.changanua_usemi(); if self.tokeni_ni(")") { self.sogeza(); } e }
            _ => NO_NODE,
        }
    }

    fn changanua_postfix(&mut self) -> i32 {
        let mut node = self.changanua_primary();
        if node == NO_NODE { return NO_NODE; }
        loop {
            if self.tokeni_ni("[") { self.sogeza(); let i = self.changanua_usemi(); if self.tokeni_ni("]") { self.sogeza(); } node = self.ast.node_mpya(AST_SAFU, 0, node, i); continue; }
            if self.tokeni_ni(".") { self.sogeza(); if matches!(self.sasa().kind, TokenKind::Kitambulisho(_) | TokenKind::NenoMuhimu(_)) { let fname = self.sasa().lexeme.clone(); self.sogeza(); let n = self.ast.node_mpya(AST_SEHEMU_DOT, 0, node, NO_NODE); self.ast.hifadhi_jina(n, &fname); node = n; continue; } break; }
            if self.tokeni_ni("->") || (self.tokeni_ni("-") && self.pos+1 < self.tokens.len() && self.tokens[self.pos+1].lexeme == ">") {
                if self.tokeni_ni("-") { self.sogeza(); self.sogeza(); } else { self.sogeza(); }
                if matches!(self.sasa().kind, TokenKind::Kitambulisho(_) | TokenKind::NenoMuhimu(_)) { let fname = self.sasa().lexeme.clone(); self.sogeza(); let n = self.ast.node_mpya(AST_SEHEMU_MSHALE, 0, node, NO_NODE); self.ast.hifadhi_jina(n, &fname); node = n; continue; } break;
            }
            break;
        }
        node
    }

    fn changanua_unary(&mut self) -> i32 {
        if self.tokeni_ni("-") && !matches!(self.sasa().kind, TokenKind::Nambari) { self.sogeza(); let o = self.changanua_unary(); return self.ast.node_mpya(AST_TOFAUTI, 0, 0, o); }
        if self.tokeni_ni("!") { self.sogeza(); let o = self.changanua_unary(); return self.ast.node_mpya(AST_SI, 0, o, NO_NODE); }
        if self.tokeni_ni("&") { self.sogeza(); let o = self.changanua_unary(); return self.ast.node_mpya(AST_KUMBUKA, 0, o, NO_NODE); }
        if self.tokeni_ni("*") { self.sogeza(); let o = self.changanua_unary(); return self.ast.node_mpya(AST_TAJA, 0, o, NO_NODE); }
        self.changanua_postfix()
    }

    fn binop(&mut self, next: fn(&mut Self) -> i32, ops: &[(&str, u32)]) -> i32 {
        let mut left = next(self);
        loop {
            let mut matched: Option<u32> = None;
            for (s, ast) in ops { if self.tokeni_ni(s) { matched = Some(*ast); break; } }
            if let Some(op) = matched { self.sogeza(); let r = next(self); left = self.ast.node_mpya(op, 0, left, r); }
            else { break; }
        }
        left
    }

    fn changanua_zidisha(&mut self) -> i32 { self.binop(Self::changanua_unary, &[("*", AST_ZIDISHA), ("/", AST_GAWANYA), ("%", AST_GAWANYA)]) }
    fn changanua_jumlisha(&mut self) -> i32 { self.binop(Self::changanua_zidisha, &[("+", AST_JUMLISHA), ("-", AST_TOFAUTI)]) }
    fn changanua_hamisha(&mut self) -> i32 { self.binop(Self::changanua_jumlisha, &[("<<", AST_HAMISHA_KUSHOTO), (">>", AST_HAMISHA_KULIA)]) }
    fn changanua_linganisha(&mut self) -> i32 { self.binop(Self::changanua_hamisha, &[("<", AST_CHINI), (">", AST_JUU), ("<=", AST_CHINI_SAWA), (">=", AST_JUU_SAWA)]) }
    fn changanua_sawa(&mut self) -> i32 { self.binop(Self::changanua_linganisha, &[("==", AST_SAWA), ("!=", AST_TOFAUTI_SI)]) }
    fn changanua_bit_na(&mut self) -> i32 { self.binop(Self::changanua_sawa, &[("&", AST_BIT_NA)]) }
    fn changanua_bit_au(&mut self) -> i32 { self.binop(Self::changanua_bit_na, &[("|", AST_BIT_AU)]) }
    fn changanua_na(&mut self) -> i32 { self.binop(Self::changanua_bit_au, &[("&&", AST_NA)]) }
    fn changanua_au(&mut self) -> i32 { self.binop(Self::changanua_na, &[("||", AST_AU)]) }
    fn changanua_ternary(&mut self) -> i32 {
        let cond = self.changanua_au();
        if self.tokeni_ni("?") {
            self.sogeza();
            let true_val = self.changanua_ternary();
            if self.tokeni_ni(":") { self.sogeza(); }
            let false_val = self.changanua_ternary();
            let n = self.ast.node_mpya(AST_TERNARY, 0, cond, true_val);
            self.ast.tiga[n as usize] = false_val;
            return n;
        }
        cond
    }
    fn changanua_asimilia(&mut self) -> i32 { self.binop(Self::changanua_ternary, &[("=", AST_ASIMILIA), ("+=", AST_ASIMILIA), ("-=", AST_ASIMILIA)]) }
    fn changanua_usemi(&mut self) -> i32 { self.changanua_asimilia() }

    // -- statement parser ----------------------------------------------------

    fn changanua_taarifa(&mut self) -> i32 {
        if matches!(self.sasa().kind, TokenKind::Mwisho) { return NO_NODE; }
        if self.tokeni_ni("}") { return NO_NODE; }

        // Bare block: { stmt; stmt; ... }
        if self.tokeni_ni("{") {
            self.sogeza();
            let mut first: i32 = NO_NODE; let mut prev: i32 = NO_NODE;
            while !self.tokeni_ni("}") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
                let s = self.changanua_taarifa(); if s == NO_NODE { break; }
                if prev == NO_NODE { first = s; } else { self.ast.nne[prev as usize] = s; } prev = s;
            }
            if self.tokeni_ni("}") { self.sogeza(); }
            // Return the first statement; the chain encodes the block.
            return first;
        }

        if self.ni_aina() {
            let va = self.changanua_aina();
            if matches!(self.sasa().kind, TokenKind::Kitambulisho(_)) {
                let name = self.sasa().lexeme.clone(); self.sogeza();
                let mut init: i32 = NO_NODE;
                if self.tokeni_ni("=") { self.sogeza(); init = self.changanua_usemi(); }
                if self.tokeni_ni(";") { self.sogeza(); }
                let name_n = self.ast.node_mpya(AST_KITAMBULISHO, 0, NO_NODE, NO_NODE);
                self.ast.hifadhi_jina(name_n, &name);
                self.ast.thamani[name_n as usize] = va;
                return self.ast.node_mpya(AST_TANGAZO, va, name_n, init);
            }
        }

        if self.tokeni_ni("rudisha") { self.sogeza(); let e = if self.tokeni_ni(";") { NO_NODE } else { self.changanua_usemi() }; if self.tokeni_ni(";") { self.sogeza(); } return self.ast.node_mpya(AST_RUDISHA, 0, e, NO_NODE); }
        if self.tokeni_ni("vunja") { self.sogeza(); if self.tokeni_ni(";") { self.sogeza(); } return self.ast.node_mpya(AST_VUNJA, 0, NO_NODE, NO_NODE); }
        if self.tokeni_ni("endelea") { self.sogeza(); if self.tokeni_ni(";") { self.sogeza(); } return self.ast.node_mpya(AST_ENDELEA, 0, NO_NODE, NO_NODE); }

        if self.tokeni_ni("kama") {
            self.sogeza(); if self.tokeni_ni("(") { self.sogeza(); }
            let cond = self.changanua_usemi();
            if self.tokeni_ni(")") { self.sogeza(); } if self.tokeni_ni("{") { self.sogeza(); }
            let mut body: i32 = NO_NODE; let mut prev: i32 = NO_NODE;
            while !self.tokeni_ni("}") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
                let s = self.changanua_taarifa(); if s == NO_NODE { break; }
                if prev == NO_NODE { body = s; } else { self.ast.nne[prev as usize] = s; } prev = s;
            }
            if self.tokeni_ni("}") { self.sogeza(); }
            let mut else_b: i32 = NO_NODE;
            if self.tokeni_ni("sivyo") { self.sogeza();
                if self.tokeni_ni("kama") { else_b = self.changanua_taarifa(); }
                else {
                    if self.tokeni_ni("{") { self.sogeza(); }
                    let mut pe: i32 = NO_NODE;
                    while !self.tokeni_ni("}") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
                        let s = self.changanua_taarifa(); if s == NO_NODE { break; }
                        if pe == NO_NODE { else_b = s; } else { self.ast.nne[pe as usize] = s; } pe = s;
                    }
                    if self.tokeni_ni("}") { self.sogeza(); }
                }
            }
            let n = self.ast.node_mpya(AST_KAMA, 0, cond, body);
            self.ast.tiga[n as usize] = else_b;
            return n;
        }

        if self.tokeni_ni("wakati") {
            self.sogeza(); if self.tokeni_ni("(") { self.sogeza(); }
            let cond = self.changanua_usemi();
            if self.tokeni_ni(")") { self.sogeza(); } if self.tokeni_ni("{") { self.sogeza(); }
            let mut body: i32 = NO_NODE; let mut prev: i32 = NO_NODE;
            while !self.tokeni_ni("}") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
                let s = self.changanua_taarifa(); if s == NO_NODE { break; }
                if prev == NO_NODE { body = s; } else { self.ast.nne[prev as usize] = s; } prev = s;
            }
            if self.tokeni_ni("}") { self.sogeza(); }
            return self.ast.node_mpya(AST_WAKATI, 0, cond, body);
        }

        // fallback: expression statement
        let e = self.changanua_usemi();
        if self.tokeni_ni(";") { self.sogeza(); }
        e
    }

    // -- function parser (WITH FIX) ------------------------------------------

    fn changanua_kazi(&mut self) -> i32 {
        if !self.ni_aina() { return NO_NODE; }
        let ret_a = self.changanua_aina();
        if !matches!(self.sasa().kind, TokenKind::Kitambulisho(_) | TokenKind::NenoMuhimu(_)) { self.kosa = true; return NO_NODE; }
        let name = self.sasa().lexeme.clone(); self.sogeza();

        // === THE FIX: check for ( vs = / ; ===
        let name_n = self.ast.node_mpya(AST_KITAMBULISHO, 0, NO_NODE, NO_NODE);
        self.ast.hifadhi_jina(name_n, &name);

        if self.tokeni_ni("(") {
            self.sogeza();
        } else {
            // Global variable: Type name = expr; or Type name;
            // May also have array size: Type name[size];
            if self.tokeni_ni("[") {
                self.sogeza(); // skip [
                self.changanua_usemi(); // skip size expression
                if self.tokeni_ni("]") { self.sogeza(); }
            }
            let mut init: i32 = NO_NODE;
            if self.tokeni_ni("=") { self.sogeza(); init = self.changanua_usemi(); }
            if self.tokeni_ni(";") { self.sogeza(); }
            return self.ast.node_mpya(AST_TANGAZO_ULIMWENGU, ret_a, name_n, init);
        }

        // Parse parameters
        let mut first_p: i32 = NO_NODE; let mut prev_p: i32 = NO_NODE;
        while !self.tokeni_ni(")") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
            if self.ni_aina() {
                let pa = self.changanua_aina();
                if matches!(self.sasa().kind, TokenKind::Kitambulisho(_) | TokenKind::NenoMuhimu(_)) {
                    let pn = self.sasa().lexeme.clone(); self.sogeza();
                    let pnode = self.ast.node_mpya(AST_KITAMBULISHO, 0, NO_NODE, NO_NODE);
                    self.ast.hifadhi_jina(pnode, &pn);
                    self.ast.thamani[pnode as usize] = pa;
                    if prev_p == NO_NODE { first_p = pnode; } else { self.ast.kulia[prev_p as usize] = pnode; }
                    prev_p = pnode;
                    if self.tokeni_ni(",") { self.sogeza(); continue; }
                }
            }
            break;
        }
        if self.tokeni_ni(")") { self.sogeza(); }
        if self.tokeni_ni("{") { self.sogeza(); }

        // Parse body
        let mut body: i32 = NO_NODE; let mut prev_s: i32 = NO_NODE;
        while !self.tokeni_ni("}") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
            let s = self.changanua_taarifa(); if s == NO_NODE { break; }
            if prev_s == NO_NODE { body = s; } else { self.ast.nne[prev_s as usize] = s; } prev_s = s;
        }
        if self.tokeni_ni("}") { self.sogeza(); }

        let func = self.ast.node_mpya(AST_KAZI, ret_a, name_n, first_p);
        self.ast.tiga[func as usize] = body;
        if first_p != NO_NODE { self.ast.kulia[name_n as usize] = first_p; }
        func
    }

    // -- struct parser -------------------------------------------------------

    fn changanua_muundo(&mut self) -> i32 {
        self.sogeza();
        if !matches!(self.sasa().kind, TokenKind::Kitambulisho(_) | TokenKind::NenoMuhimu(_)) { self.kosa = true; return NO_NODE; }
        let sname = self.sasa().lexeme.clone(); self.sogeza();
        if !self.tokeni_ni("{") { self.kosa = true; return NO_NODE; } self.sogeza();

        let sn = self.ast.node_mpya(AST_MUUNDO, 0, NO_NODE, NO_NODE);
        let nn = self.ast.node_mpya(AST_KITAMBULISHO, 0, NO_NODE, NO_NODE);
        self.ast.hifadhi_jina(nn, &sname);
        self.ast.kushoto[sn as usize] = nn;

        let mut first_f: i32 = NO_NODE; let mut prev_f: i32 = NO_NODE;
        while !self.tokeni_ni("}") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
            if self.ni_aina() {
                let fa = self.changanua_aina();
                if matches!(self.sasa().kind, TokenKind::Kitambulisho(_) | TokenKind::NenoMuhimu(_)) {
                    let fname = self.sasa().lexeme.clone(); self.sogeza();
                    let fn_n = self.ast.node_mpya(AST_KITAMBULISHO, 0, NO_NODE, NO_NODE);
                    self.ast.hifadhi_jina(fn_n, &fname);
                    self.ast.thamani[fn_n as usize] = fa;
                    if self.tokeni_ni(";") { self.sogeza(); }
                    let f = self.ast.node_mpya(AST_SEHEMU, 0, fn_n, NO_NODE);
                    if prev_f == NO_NODE { first_f = f; } else { self.ast.kulia[prev_f as usize] = f; }
                    prev_f = f; continue;
                }
            }
            break;
        }
        if self.tokeni_ni("}") { self.sogeza(); }
        if self.tokeni_ni(";") { self.sogeza(); }
        self.ast.kulia[sn as usize] = first_f;
        sn
    }

    // -- top-level dispatch --------------------------------------------------

    fn changanua(&mut self) -> i32 {
        let mut first: i32 = NO_NODE; let mut prev: i32 = NO_NODE;
        while !matches!(self.sasa().kind, TokenKind::Mwisho) {
            let mut node: i32 = NO_NODE;
            // Module directives: skip husisha / kitengo lines.
            // Format: husisha C::stdio  or  husisha "path.swa"
            // These are newline-terminated; consume the keyword + argument.
            if self.tokeni_ni("husisha") || self.tokeni_ni("kitengo") {
                self.sogeza(); // skip keyword
                // Consume argument: either C::stdio (ident :: ident) or "string"
                if matches!(self.sasa().kind, TokenKind::Kitambulisho(_) | TokenKind::NenoMuhimu(_)) {
                    self.sogeza();
                    // Possibly :: and another identifier
                    if self.tokeni_ni("::") { self.sogeza(); }
                    else if self.tokeni_ni(":") { self.sogeza(); if self.tokeni_ni(":") { self.sogeza(); } }
                    if matches!(self.sasa().kind, TokenKind::Kitambulisho(_) | TokenKind::NenoMuhimu(_)) {
                        self.sogeza();
                    }
                } else if matches!(self.sasa().kind, TokenKind::Mfuato(_)) {
                    self.sogeza();
                }
                // Skip trailing semicolon if present.
                if self.tokeni_ni(";") { self.sogeza(); }
                continue;
            }
            if self.tokeni_ni("muundo") { node = self.changanua_muundo(); }
            if node == NO_NODE { node = self.changanua_kazi(); }
            if node == NO_NODE { self.kosa = true; break; }
            if prev == NO_NODE { first = node; } else { self.ast.nne[prev as usize] = node; }
            prev = node;
        }
        self.ast.node_mpya(AST_PROGRAMU, 0, first, NO_NODE)
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Parse tokens into flat AST arrays consumable by `ir::lower::lower()`.
pub fn parse_full(tokens: &[Token]) -> Result<(Vec<u32>, Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>, Vec<u8>, usize), String> {
    let mut p = Parser::new(tokens);
    p.changanua();
    if p.kosa {
        let t = p.sasa();
        let msg = format!("parse error at line {} col {} near '{}'", t.span.start.line, t.span.start.column, t.lexeme);
        return Err(msg);
    }
    let count = p.ast.aina.len();
    Ok((p.ast.aina, p.ast.thamani, p.ast.kushoto, p.ast.kulia, p.ast.tiga, p.ast.nne, p.ast.jina_off, p.ast.pool, count))
}
