//! Mchanganuzi wa Swa — unaoakisi msambazaji.swa.
//!
//! Huchambua tokeni kutoka kwa msomaji hadi AST ya safu bapa inayotumiwa na
//! `ir::lower::lower()`.

use crate::diagnostics::{DiagnosticBag, SourceSpan};
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
const AST_HALI: u32 = 47;
const AST_HALISI_D: u32 = 49;  // desimali halisi (D64)
const AST_KIPINDI: u32 = 15;
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
const AST_MODULO: u32 = 48;
const AST_SAFU: u32 = 38;
const AST_MFUATANO: u32 = 40;
const AST_KWELI: u32 = 44;
const AST_UONGO: u32 = 45;
const AST_TUPU: u32 = 46;
const NO_NODE: i32 = -1;

/// Matokeo ya `ruka_hadi_kifikisha` — kizuizi kilichopatikana.
#[derive(Debug, PartialEq, Eq)]
enum Kifikisha { NusuKoloni, MabanoYaWima, Mwisho }

// ---------------------------------------------------------------------------
// Kijenzi cha AST — hukusanya safu bapa
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
// Mchanganuzi
// ---------------------------------------------------------------------------
struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
    ast: AstBuilder,
    diagnostics: &'a mut DiagnosticBag,
    /// Tokeni sintetiki ya Mwisho inayotumika wakati tokeni halisi zimeisha.
    mwisho_token: Token,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token], diagnostics: &'a mut DiagnosticBag) -> Self {
        let mwisho_token = Token {
            kind: TokenKind::Mwisho,
            lexeme: String::new(),
            span: SourceSpan::point(0, 0),
        };
        Self { tokens, pos: 0, ast: AstBuilder::new(), diagnostics, mwisho_token }
    }

    fn sasa(&self) -> &Token {
        if self.pos < self.tokens.len() { &self.tokens[self.pos] }
        else { &self.mwisho_token }
    }

    /// Rekodi kosa kwenye nafasi ya tokeni ya sasa.
    fn kosa(&mut self, ujumbe: &str) {
        let span = self.sasa().span;
        self.diagnostics.error(ujumbe, span);
    }

    /// Je, kuna makosa yoyote yaliyorekodiwa?
    #[allow(dead_code)]
    fn ina_makosa(&self) -> bool {
        self.diagnostics.has_errors()
    }

    fn sogeza(&mut self) { if self.pos < self.tokens.len() { self.pos += 1; } }

    /// Ruka tokeni hadi kifikisha cha taarifa (`;`, `}`, au mwisho wa faili).
    ///
    /// Hairuki kifikisha chenyewe — mpigaji anaamua nini cha kufanya nacho.
    ///
    /// Hutambua vifikisha tu wakati viko kwenye ngazi ya taarifa
    /// (si ndani ya vielezi), kwa kuangalia kama herufi ya kwanza ya tokeni
    /// iko nje ya masafa ya herufi [A-Za-z].
    fn ruka_hadi_kifikisha(&mut self) -> Kifikisha {
        while !matches!(self.sasa().kind, TokenKind::Mwisho) {
            let ch = self.sasa().lexeme.as_bytes().first().copied().unwrap_or(0);
            // Angalia vifikisha tu wakati tokeni haianzi na herufi
            // (hivyo hatutachanganya `;` au `}` ndani ya vielezi / mifuatano)
            if ch < 65 || ch > 122 {
                if self.tokeni_ni(";") { return Kifikisha::NusuKoloni; }
                if self.tokeni_ni("}") { return Kifikisha::MabanoYaWima; }
            }
            // Kesi maalum: `;` yenye urefu wa 1
            if self.sasa().lexeme.len() == 1 && self.tokeni_ni(";") {
                return Kifikisha::NusuKoloni;
            }
            self.sogeza();
        }
        Kifikisha::Mwisho
    }

    /// Saidizi ya urejeshaji: panua tokeni ya sasa ikiwa inalingana,
    /// vinginevyo rekodi kosa na urejeshe.
    fn tarajia(&mut self, kifikisha: &str, ujumbe: &str) -> bool {
        if self.tokeni_ni(kifikisha) {
            self.sogeza();
            true
        } else {
            self.kosa(ujumbe);
            false
        }
    }

    /// Saidizi ya urejeshaji kwa miili ya bloku inaposhindikana kuchanganua
    /// taarifa. Hurejesha `true` ikiwa inapaswa kuendelea kitanzi,
    /// `false` ikiwa inapaswa kuvunja (EOF).
    fn recover_ya_mwili(&mut self) -> bool {
        match self.ruka_hadi_kifikisha() {
            Kifikisha::NusuKoloni => { self.sogeza(); true }
            Kifikisha::MabanoYaWima => { /* acha } kwa hali ya kitanzi */ true }
            Kifikisha::Mwisho => false,
        }
    }

    /// Changanua mnyororo wa matawi yanayofuata taarifa ya kama:
    /// `sivyo { ... }`, `sivyo kama (...) { ... }`, au
    /// `kamasivyo (...) { ... }` — pamoja na minyororo mingi
    /// (`kamasivyo` baada ya `kamasivyo`). Inarudisha nodi ya tawi
    /// (AST_KAMA kwa kamasivyo, au nodi ya kwanza ya msururu kwa
    /// sivyo), au NO_NODE ikiwa hakuna tawi linalofuata.
    fn changanua_mnyororo_wa_sivyo(&mut self) -> i32 {
        if self.tokeni_ni("sivyo") {
            self.sogeza();
            if self.tokeni_ni("kama") {
                return self.changanua_taarifa();
            }
            // sivyo { ... }
            if self.tokeni_ni("{") { self.sogeza(); }
            let mut first: i32 = NO_NODE; let mut prev: i32 = NO_NODE;
            while !self.tokeni_ni("}") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
                let s = self.changanua_taarifa(); if s == NO_NODE { if !self.recover_ya_mwili() { break; } continue; }
                if prev == NO_NODE { first = s; } else { self.ast.nne[prev as usize] = s; } prev = s;
                while self.ast.nne[prev as usize] != NO_NODE && self.ast.nne[prev as usize] >= 0 { prev = self.ast.nne[prev as usize]; }
            }
            if self.tokeni_ni("}") { self.sogeza(); }
            return first;
        }
        if self.tokeni_ni("kamasivyo") {
            self.sogeza();
            if self.tokeni_ni("(") { self.sogeza(); }
            let cond = self.changanua_usemi();
            self.tarajia(")", "')' inatarajiwa baada ya sharti la kamasivyo");
            if self.tokeni_ni("{") { self.sogeza(); }
            let mut first: i32 = NO_NODE; let mut prev: i32 = NO_NODE;
            while !self.tokeni_ni("}") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
                let s = self.changanua_taarifa(); if s == NO_NODE { if !self.recover_ya_mwili() { break; } continue; }
                if prev == NO_NODE { first = s; } else { self.ast.nne[prev as usize] = s; } prev = s;
                while self.ast.nne[prev as usize] != NO_NODE && self.ast.nne[prev as usize] >= 0 { prev = self.ast.nne[prev as usize]; }
            }
            if self.tokeni_ni("}") { self.sogeza(); }
            let n = self.ast.node_mpya(AST_KAMA, 0, cond, first);
            self.ast.tiga[n as usize] = self.changanua_mnyororo_wa_sivyo();
            return n;
        }
        NO_NODE
    }

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
            // "tupu" ni neno muhimu la aina ya bila-thamani (sawa na W0)
            // lakini huanza na herufi ndogo — kikubali kwa uwazi.
            TokenKind::NenoMuhimu(s) => s == "tupu"
                || s.as_bytes().first().map_or(false, |c| c.is_ascii_uppercase()),
            TokenKind::Kitambulisho(s) => s.as_bytes().first().map_or(false, |c| c.is_ascii_uppercase()),
            _ => false,
        }
    }

    fn changanua_aina(&mut self) -> i32 {
        // Hurejesha aina iliyosimbwa kama i32 (biti za juu: familia, biti za chini: upana + bendera ya mshale)
        if !self.ni_aina() { return 0; }
        let txt = self.sasa().lexeme.clone();
        let n = txt.len();
        let (familia, upana): (u32, u32) = if txt == "tupu" {
            (5, 0) // tupu == W0 (bila thamani)
        } else if n >= 2 && txt[1..].chars().all(|c| c.is_ascii_digit()) {
            let c0 = txt.as_bytes()[0];
            let fam = match c0 { b'N' => 1, b'A' => 2, b'D' => 3, b'B' => 4, b'W' => 5, _ => 6 };
            let w = txt[1..].parse().unwrap_or(32);
            (fam, w)
        } else {
            // Aina ya muundo iliyofafanuliwa na mtumiaji — hifadhi jina kwenye dimbwi na urudishe kianzio hasi.
            let name_off = self.ast.pool.len() as i32;
            self.ast.pool.extend_from_slice(txt.as_bytes());
            self.ast.pool.push(0);
            self.sogeza();
            // Ruka ushughulikiaji wa vielekezi kwa aina za mtumiaji (zitakuwa na * baada ya jina).
            let mut mshale: u32 = 0;
            while self.tokeni_ni("*") || self.tokeni_ni("**") || self.tokeni_ni("***") {
                mshale += self.sasa().lexeme.len() as u32;
                self.sogeza();
            }
            return -(name_off * 2 + mshale as i32);
        };
        self.sogeza();
        let mut mshale: u32 = 0;
        // Shughulikia mifululizo ya vielekezi: *, **, ***, n.k. Msomaji anaweza kutokeni
        // "**" kama tokeni moja ya opereta.
        while self.tokeni_ni("*") || self.tokeni_ni("**") || self.tokeni_ni("***") {
            mshale += self.sasa().lexeme.len() as u32;
            self.sogeza();
        }
        // Simba: familia << 11 | upana_idx << 3 | mshale (0-7)
        fn upana_idx(w: u32) -> u32 { match w { 0=>0, 1=>1, 8=>2, 16=>3, 32=>4, 64=>5, 128=>6, _=>4 } }
        (((familia & 255) << 11) | (upana_idx(upana) << 3) | (mshale & 7)) as i32
    }

    // -- mchanganuzi wa usemi (kupanda kwa utangulizi) -----------------------

    fn changanua_primary(&mut self) -> i32 {
        match &self.sasa().kind {
            TokenKind::Nambari => {
                // Namba kamili kama mifumo ya biti ya N32 (mbegu hufanya
                // hivyo) — parse ya i32 moja kwa moja inazunguka kwa
                // thamani zaidi ya 2^31-1. Desimali (zenye '.') zinakuwa
                // AST_HALISI_D — suala #135: zamani ziligeuka 0 kimya.
                let safi: String = self.sasa().lexeme.chars().filter(|c| *c != '_').collect();
                if safi.contains('.') {
                    let n = self.ast.node_mpya(AST_HALISI_D, 0, NO_NODE, NO_NODE);
                    self.ast.hifadhi_jina(n, &safi);
                    self.sogeza();
                    return n;
                }
                let v: i32 = safi.parse::<u32>().map(|u| u as i32).unwrap_or(0);
                self.sogeza();
                self.ast.node_mpya(AST_NAMBARI, v, NO_NODE, NO_NODE)
            }
            TokenKind::Mfuato(inner) => {
                // inner ni maudhui yasiyotolewa misimbo bila kunukuu kuzunguka
                let s = inner.clone();
                let n = self.ast.node_mpya(AST_MFUATANO, 0, NO_NODE, NO_NODE);
                self.ast.hifadhi_jina(n, &s);
                self.sogeza();
                n
            }
            TokenKind::NenoMuhimu(k) if k == "kweli" => {
                self.sogeza();
                self.ast.node_mpya(AST_KWELI, 1, NO_NODE, NO_NODE)
            }
            TokenKind::NenoMuhimu(k) if k == "uongo" => {
                self.sogeza();
                self.ast.node_mpya(AST_UONGO, 0, NO_NODE, NO_NODE)
            }
            TokenKind::NenoMuhimu(k) if k == "tupu" => {
                self.sogeza();
                self.ast.node_mpya(AST_TUPU, 0, NO_NODE, NO_NODE)
            }
            TokenKind::Kitambulisho(_) | TokenKind::NenoMuhimu(_) => {
                let name = self.sasa().lexeme.clone();
                self.sogeza();
                if name == "tenga" && self.tokeni_ni("(") {
                    self.sogeza();
                    let arg = self.changanua_usemi();
                    self.tarajia(")", "')' inatarajiwa baada ya hoja ya tenga");
                    return self.ast.node_mpya(AST_TENGA, 0, arg, NO_NODE);
                }
                if name == "achilia" && self.tokeni_ni("(") {
                    self.sogeza();
                    let arg = self.changanua_usemi();
                    self.tarajia(")", "')' inatarajiwa baada ya hoja ya achilia");
                    return self.ast.node_mpya(AST_ACHILIA, 0, arg, NO_NODE);
                }
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
                    self.tarajia(")", "')' inatarajiwa baada ya hoja za wito");
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
            TokenKind::MabanoKushoto => { self.sogeza(); let e = self.changanua_usemi(); self.tarajia(")", "')' inatarajiwa kufunga usemi wa mabano"); e }
            _ => NO_NODE,
        }
    }

    fn changanua_postfix(&mut self) -> i32 {
        let mut node = self.changanua_primary();
        if node == NO_NODE { return NO_NODE; }
        loop {
            if self.tokeni_ni("[") { self.sogeza(); let i = self.changanua_usemi(); self.tarajia("]", "']' inatarajiwa kufunga faharasa ya safu"); node = self.ast.node_mpya(AST_SAFU, 0, node, i); continue; }
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
        if self.tokeni_ni("-") && !matches!(self.sasa().kind, TokenKind::Nambari) { self.sogeza(); let o = self.changanua_unary(); return self.ast.node_mpya(AST_TOFAUTI, 0, NO_NODE, o); }
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

    fn changanua_zidisha(&mut self) -> i32 { self.binop(Self::changanua_unary, &[("*", AST_ZIDISHA), ("/", AST_GAWANYA), ("%", AST_MODULO)]) }
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

    // -- mchanganuzi wa taarifa ----------------------------------------------

    fn changanua_taarifa(&mut self) -> i32 {
        if matches!(self.sasa().kind, TokenKind::Mwisho) { return NO_NODE; }
        if self.tokeni_ni("}") { return NO_NODE; }

        // Kizuizi tupu: { taarifa; taarifa; ... }
        if self.tokeni_ni("{") {
            self.sogeza();
            let mut first: i32 = NO_NODE; let mut prev: i32 = NO_NODE;
            while !self.tokeni_ni("}") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
                let s = self.changanua_taarifa(); if s == NO_NODE { if !self.recover_ya_mwili() { break; } continue; }
                if prev == NO_NODE { first = s; } else { self.ast.nne[prev as usize] = s; } prev = s;
                while self.ast.nne[prev as usize] != NO_NODE && self.ast.nne[prev as usize] >= 0 { prev = self.ast.nne[prev as usize]; }
            }
            if self.tokeni_ni("}") { self.sogeza(); }
            // Rudisha taarifa ya kwanza; msururu husimba kizuizi.
            return first;
        }

        if self.ni_aina() {
            let va = self.changanua_aina();
            if matches!(self.sasa().kind, TokenKind::Kitambulisho(_) | TokenKind::NenoMuhimu(_)) {
                let name = self.sasa().lexeme.clone(); self.sogeza();
                let mut init: i32 = NO_NODE;
                let mut saizi_ya_safu: i32 = NO_NODE;
                if self.tokeni_ni("[") {
                    self.sogeza();
                    saizi_ya_safu = self.changanua_usemi();
                    self.tarajia("]", "']' inatarajiwa baada ya ukubwa wa safu");
                }
                if self.tokeni_ni("=") { self.sogeza(); init = self.changanua_usemi(); }
                if self.tokeni_ni(";") { self.sogeza(); }
                let name_n = self.ast.node_mpya(AST_KITAMBULISHO, 0, NO_NODE, NO_NODE);
                self.ast.hifadhi_jina(name_n, &name);
                self.ast.thamani[name_n as usize] = va;
                let node = self.ast.node_mpya(AST_TANGAZO, va, name_n, init);
                if saizi_ya_safu != NO_NODE {
                    self.ast.tiga[node as usize] = saizi_ya_safu;
                }
                return node;
            }
        }

        if self.tokeni_ni("rudisha") { self.sogeza(); let e = if self.tokeni_ni(";") { NO_NODE } else { self.changanua_usemi() }; if self.tokeni_ni(";") { self.sogeza(); } return self.ast.node_mpya(AST_RUDISHA, 0, e, NO_NODE); }
        if self.tokeni_ni("vunja") { self.sogeza(); if self.tokeni_ni(";") { self.sogeza(); } return self.ast.node_mpya(AST_VUNJA, 0, NO_NODE, NO_NODE); }
        if self.tokeni_ni("endelea") { self.sogeza(); if self.tokeni_ni(";") { self.sogeza(); } return self.ast.node_mpya(AST_ENDELEA, 0, NO_NODE, NO_NODE); }

        if self.tokeni_ni("kama") {
            self.sogeza(); if self.tokeni_ni("(") { self.sogeza(); }
            let cond = self.changanua_usemi();
            self.tarajia(")", "')' inatarajiwa baada ya sharti la kama"); if self.tokeni_ni("{") { self.sogeza(); }
            let mut body: i32 = NO_NODE; let mut prev: i32 = NO_NODE;
            while !self.tokeni_ni("}") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
                let s = self.changanua_taarifa(); if s == NO_NODE { if !self.recover_ya_mwili() { break; } continue; }
                if prev == NO_NODE { body = s; } else { self.ast.nne[prev as usize] = s; } prev = s;
                while self.ast.nne[prev as usize] != NO_NODE && self.ast.nne[prev as usize] >= 0 { prev = self.ast.nne[prev as usize]; }
            }
            if self.tokeni_ni("}") { self.sogeza(); }
            let n = self.ast.node_mpya(AST_KAMA, 0, cond, body);
            // Mnyororo wa matawi yanayofuata (sivyo, sivyo kama,
            // kamasivyo) unachanganuliwa kwa kujirudia — suala #134:
            // "kamasivyo" halikuwa neno muhimu kwenye msomaji wa Rust
            // na lilichanganuliwa kama WITO WA KAZI (undefined
            // reference wakati wa kuunganisha).
            self.ast.tiga[n as usize] = self.changanua_mnyororo_wa_sivyo();
            return n;
        }

        if self.tokeni_ni("wakati") {
            self.sogeza(); if self.tokeni_ni("(") { self.sogeza(); }
            let cond = self.changanua_usemi();
            self.tarajia(")", "')' inatarajiwa baada ya sharti la wakati"); if self.tokeni_ni("{") { self.sogeza(); }
            let mut body: i32 = NO_NODE; let mut prev: i32 = NO_NODE;
            while !self.tokeni_ni("}") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
                let s = self.changanua_taarifa(); if s == NO_NODE { if !self.recover_ya_mwili() { break; } continue; }
                if prev == NO_NODE { body = s; } else { self.ast.nne[prev as usize] = s; } prev = s;
                while self.ast.nne[prev as usize] != NO_NODE && self.ast.nne[prev as usize] >= 0 { prev = self.ast.nne[prev as usize]; }
            }
            if self.tokeni_ni("}") { self.sogeza(); }
            return self.ast.node_mpya(AST_WAKATI, 0, cond, body);
        }

        if self.tokeni_ni("kwa") {
            self.sogeza(); if self.tokeni_ni("(") { self.sogeza(); }
            let init = if self.tokeni_ni(";") { NO_NODE } else { let e = self.changanua_usemi(); if self.tokeni_ni(";") { self.sogeza(); } e };
            let cond = if self.tokeni_ni(";") { NO_NODE } else { let e = self.changanua_usemi(); if self.tokeni_ni(";") { self.sogeza(); } e };
            let step = if self.tokeni_ni(")") { NO_NODE } else { let e = self.changanua_usemi(); self.tarajia(")", "')' inatarajiwa baada ya hatua ya kwa"); e };
            if self.tokeni_ni("{") { self.sogeza(); }
            let mut body: i32 = NO_NODE; let mut prev: i32 = NO_NODE;
            while !self.tokeni_ni("}") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
                let s = self.changanua_taarifa(); if s == NO_NODE { if !self.recover_ya_mwili() { break; } continue; }
                if prev == NO_NODE { body = s; } else { self.ast.nne[prev as usize] = s; } prev = s;
                while self.ast.nne[prev as usize] != NO_NODE && self.ast.nne[prev as usize] >= 0 { prev = self.ast.nne[prev as usize]; }
            }
            if self.tokeni_ni("}") { self.sogeza(); }
            let n = self.ast.node_mpya(AST_KIPINDI, cond, init, body);
            self.ast.tiga[n as usize] = step;
            return n;
        }

        if self.tokeni_ni("chagua") {
            self.sogeza();
            if self.tokeni_ni("(") { self.sogeza(); }
            let tested = self.changanua_usemi();
            self.tarajia(")", "')' inatarajiwa baada ya usemi wa chagua");
            if self.tokeni_ni("{") { self.sogeza(); }

            let mut first_case: i32 = NO_NODE;
            let mut prev_case: i32 = NO_NODE;
            let mut default_body: i32 = NO_NODE;

            while !self.tokeni_ni("}") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
                if self.tokeni_ni("hali") {
                    self.sogeza();
                    let label = self.changanua_usemi();
                    if self.tokeni_ni(":") { self.sogeza(); }

                    // Changanua mwili wa hali hii
                    let mut case_body: i32 = NO_NODE;
                    let mut cp: i32 = NO_NODE;
                    while !self.tokeni_ni("}") && !self.tokeni_ni("hali") && !self.tokeni_ni("sivyo") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
                        let s = self.changanua_taarifa();
                        if s == NO_NODE { if !self.recover_ya_mwili() { break; } continue; }
                        if cp == NO_NODE { case_body = s; } else { self.ast.nne[cp as usize] = s; }
                        cp = s;
                        while self.ast.nne[cp as usize] != NO_NODE && self.ast.nne[cp as usize] >= 0 { cp = self.ast.nne[cp as usize]; }
                    }

                    let case_node = self.ast.node_mpya(AST_HALI, 0, label, NO_NODE);
                    self.ast.tiga[case_node as usize] = case_body;
                    if prev_case == NO_NODE { first_case = case_node; }
                    else { self.ast.nne[prev_case as usize] = case_node; }
                    prev_case = case_node;
                    continue;
                }
                if self.tokeni_ni("sivyo") {
                    self.sogeza();
                    if self.tokeni_ni(":") { self.sogeza(); }

                    // Changanua mwili wa chaguo-msingi
                    let mut dp: i32 = NO_NODE;
                    while !self.tokeni_ni("}") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
                        let s = self.changanua_taarifa();
                        if s == NO_NODE { if !self.recover_ya_mwili() { break; } continue; }
                        if dp == NO_NODE { default_body = s; } else { self.ast.nne[dp as usize] = s; }
                        dp = s;
                        while self.ast.nne[dp as usize] != NO_NODE && self.ast.nne[dp as usize] >= 0 { dp = self.ast.nne[dp as usize]; }
                    }
                    break;
                }
                break;
            }
            if self.tokeni_ni("}") { self.sogeza(); }

            let n = self.ast.node_mpya(AST_CHAGUA, 0, tested, first_case);
            self.ast.tiga[n as usize] = default_body;
            return n;
        }

        // mrejesho: taarifa ya usemi
        let e = self.changanua_usemi();
        if self.tokeni_ni(";") { self.sogeza(); }
        else if e == NO_NODE && !matches!(self.sasa().kind, TokenKind::Mwisho) {
            // Hakuna taarifa iliyotambulika — sogeza mbele ili kuzuia kitanzi kisicho na mwisho
            self.kosa("taarifa isiyotambulika");
            self.sogeza();
        }
        e
    }

    // -- mchanganuzi wa kazi (PAMOJA NA MAREKEBISHO) -------------------------

    fn changanua_kazi(&mut self) -> i32 {
        if !self.ni_aina() { return NO_NODE; }
        let ret_a = self.changanua_aina();
        if !matches!(self.sasa().kind, TokenKind::Kitambulisho(_) | TokenKind::NenoMuhimu(_)) { self.kosa("jina la kazi linatarajiwa"); return NO_NODE; }
        let name = self.sasa().lexeme.clone(); self.sogeza();

        // === MAREKEBISHO: angalia ( dhidi ya = / ; ===
        let name_n = self.ast.node_mpya(AST_KITAMBULISHO, 0, NO_NODE, NO_NODE);
        self.ast.hifadhi_jina(name_n, &name);

        if self.tokeni_ni("(") {
            self.sogeza();
        } else {
            // Kigezo cha ulimwengu: Aina jina = usemi; au Aina jina;
            // Inaweza pia kuwa na ukubwa wa safu: Aina jina[ukubwa];
            let mut saizi_ya_safu: i32 = NO_NODE;
            if self.tokeni_ni("[") {
                self.sogeza(); // ruka [
                saizi_ya_safu = self.changanua_usemi(); // nasa usemi wa ukubwa
                self.tarajia("]", "']' inatarajiwa baada ya ukubwa wa safu ya ulimwengu");
            }
            let mut init: i32 = NO_NODE;
            if self.tokeni_ni("=") { self.sogeza(); init = self.changanua_usemi(); }
            if self.tokeni_ni(";") { self.sogeza(); }
            let node = self.ast.node_mpya(AST_TANGAZO_ULIMWENGU, ret_a, name_n, init);
            if saizi_ya_safu != NO_NODE {
                self.ast.tiga[node as usize] = saizi_ya_safu;
            }
            return node;
        }

        // Changanua vigezo
        let mut first_p: i32 = NO_NODE; let mut prev_p: i32 = NO_NODE;
        while !self.tokeni_ni(")") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
            if self.ni_aina() {
                let mut pa = self.changanua_aina();
                if matches!(self.sasa().kind, TokenKind::Kitambulisho(_) | TokenKind::NenoMuhimu(_)) {
                    let pn = self.sasa().lexeme.clone(); self.sogeza();
                    // Kigezo cha safu: N32 a[4] — hubadilika kuwa kielekezi
                    // (semantiki ya C): N32 a[4] == N32* a.
                    if self.tokeni_ni("[") {
                        self.sogeza();
                        let _saizi = self.changanua_usemi(); // ukubwa hauhifadhiwi kwa kigezo
                        if !self.tarajia("]", "']' inatarajiwa baada ya ukubwa wa safu ya kigezo") {
                            return NO_NODE;
                        }
                        pa = (pa & !7) | ((pa & 7) + 1); // ongeza ngazi moja ya kielekezi
                    }
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
        // Kukosa ')' hapa kuna maana tangazo la kigezo limevunjika —
        // usijenge nodi ya kazi yenye muundo taka (ilikuwa chanzo cha
        // OOM kwenye suala #136: nodi ya uwongo ilirudishwa na mzunguko
        // wa kiwango cha juu ulizunguka bila mwisho kwa '}' isiyotumiwa).
        if !self.tarajia(")", "')' inatarajiwa baada ya vigezo vya kazi") { return NO_NODE; }

        // Tangazo la mbele (forward declaration): hakuna mwili wa { }
        if !self.tokeni_ni("{") {
            if self.tokeni_ni(";") { self.sogeza(); }
            let func = self.ast.node_mpya(AST_KAZI, ret_a, name_n, first_p);
            self.ast.tiga[func as usize] = NO_NODE;
            if first_p != NO_NODE { self.ast.kulia[name_n as usize] = first_p; }
            return func;
        }
        self.sogeza(); // ruka '{'

        // Changanua mwili
        let mut body: i32 = NO_NODE; let mut prev_s: i32 = NO_NODE;
        while !self.tokeni_ni("}") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
            let s = self.changanua_taarifa(); if s == NO_NODE { break; }
            if prev_s == NO_NODE { body = s; } else { self.ast.nne[prev_s as usize] = s; } prev_s = s;
            while self.ast.nne[prev_s as usize] != NO_NODE && self.ast.nne[prev_s as usize] >= 0 { prev_s = self.ast.nne[prev_s as usize]; }
        }
        if self.tokeni_ni("}") { self.sogeza(); }

        let func = self.ast.node_mpya(AST_KAZI, ret_a, name_n, first_p);
        self.ast.tiga[func as usize] = body;
        if first_p != NO_NODE { self.ast.kulia[name_n as usize] = first_p; }
        func
    }

    // -- mchanganuzi wa muundo -----------------------------------------------

    fn changanua_muundo(&mut self) -> i32 {
        self.sogeza();
        if !matches!(self.sasa().kind, TokenKind::Kitambulisho(_) | TokenKind::NenoMuhimu(_)) { self.kosa("jina la muundo linatarajiwa"); return NO_NODE; }
        let sname = self.sasa().lexeme.clone(); self.sogeza();
        if !self.tokeni_ni("{") { self.kosa("{ inatarajiwa baada ya jina la muundo"); return NO_NODE; } self.sogeza();

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

    // -- utumaji ngazi-ya-juu ------------------------------------------------

    fn changanua(&mut self) -> i32 {
        let mut first: i32 = NO_NODE; let mut prev: i32 = NO_NODE;
        while !matches!(self.sasa().kind, TokenKind::Mwisho) {
            let mut node: i32 = NO_NODE;
            // Maelekezo ya moduli: husisha C::stdio  /  husisha { njia }  /  husisha { njia } kutoka { dir }
            if self.tokeni_ni("husisha") {
                self.sogeza(); // ruka 'husisha'

                // Sintaksia: husisha { njia }
                if self.tokeni_ni("{") {
                    self.sogeza(); // ruka '{'
                    while !self.tokeni_ni("}") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
                        self.sogeza(); // tumia tokeni za njia ndani ya mabano
                    }
                    if self.tokeni_ni("}") { self.sogeza(); } // ruka '}'

                    // Hiari: kutoka { saraka }
                    if self.tokeni_ni("kutoka") {
                        self.sogeza(); // ruka 'kutoka'
                        if self.tokeni_ni("{") {
                            self.sogeza(); // ruka '{'
                            while !self.tokeni_ni("}") && !matches!(self.sasa().kind, TokenKind::Mwisho) {
                                self.sogeza();
                            }
                            if self.tokeni_ni("}") { self.sogeza(); } // ruka '}'
                        }
                    }
                }
                // husisha C::stdio
                else if matches!(self.sasa().kind, TokenKind::Kitambulisho(_) | TokenKind::NenoMuhimu(_)) {
                    self.sogeza();
                    if self.tokeni_ni("::") { self.sogeza(); }
                    else if self.tokeni_ni(":") { self.sogeza(); if self.tokeni_ni(":") { self.sogeza(); } }
                    if matches!(self.sasa().kind, TokenKind::Kitambulisho(_) | TokenKind::NenoMuhimu(_)) {
                        self.sogeza();
                    }
                }
                if self.tokeni_ni(";") { self.sogeza(); }
                continue;
            }
            if self.tokeni_ni("muundo") { node = self.changanua_muundo(); }
            if node == NO_NODE { node = self.changanua_kazi(); }
            if node == NO_NODE {
                // Mabano ya wima yasiyotarajiwa katika kiwango cha juu:
                // recover_ya_mwili HUYACHI kwa makusudi (hali ya kitanzi),
                // lakini katika kiwango cha juu hakuna kitanzi
                // kitakachoyatumia — bila kinga hii mzunguko unazunguka
                // milele ukilimbikiza makosa (OOM, suala #136).
                if self.tokeni_ni("}") {
                    self.kosa("mabano ya wima yasiyotarajiwa katika kiwango cha juu");
                    self.sogeza();
                    continue;
                }
                self.kosa("kipengele cha ngazi ya juu hakutambulika");
                if !self.recover_ya_mwili() { break; }
                continue;
            }
            if prev == NO_NODE { first = node; } else { self.ast.nne[prev as usize] = node; }
            prev = node;
            while self.ast.nne[prev as usize] != NO_NODE && self.ast.nne[prev as usize] >= 0 { prev = self.ast.nne[prev as usize]; }
        }
        self.ast.node_mpya(AST_PROGRAMU, 0, first, NO_NODE)
    }
}

// ---------------------------------------------------------------------------
// API ya umma
// ---------------------------------------------------------------------------

/// Chambua tokeni hadi safu bapa za AST zinazotumiwa na `ir::lower::lower()`.
///
/// Hurejesha `Vec<Diagnostic>` yenye sehemu sahihi za chanzo ikitokea makosa ya uchanganuzi.
pub fn parse_full(tokens: &[Token]) -> Result<(Vec<u32>, Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>, Vec<u8>, usize), Vec<crate::diagnostics::Diagnostic>> {
    let mut diagnostics = DiagnosticBag::new();
    let (count, aina, thamani, kushoto, kulia, tiga, nne, jina_off, pool) = {
        let mut p = Parser::new(tokens, &mut diagnostics);
        p.changanua();
        let count = p.ast.aina.len();
        (count, p.ast.aina, p.ast.thamani, p.ast.kushoto, p.ast.kulia, p.ast.tiga, p.ast.nne, p.ast.jina_off, p.ast.pool)
    };
    // Sasa diagnostics haijakopwa tena — tunaweza kuiangalia
    if diagnostics.has_errors() {
        return Err(diagnostics.all().to_vec());
    }
    Ok((aina, thamani, kushoto, kulia, tiga, nne, jina_off, pool, count))
}
