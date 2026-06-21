//! Mti wa Sintaksia (AST) kwa lugha ya Swa.
//!
//! ## Muundo wa nodi
//!
//! Kila nodi ina:
//! - `aina` — aina ya nodi (angalia [`AinaYaNodi`])
//! - `kushoto` — mtoto wa kushoto (jina la kazi, sharti, n.k.)
//! - `kulia` — mtoto wa kulia (vigezo, tawi la kweli, n.k.)
//! - `tatu` — mtoto wa tatu (mwili wa kazi, tawi la uongo, n.k.)
//! - `nne` — mtoto wa nne / ndugu (msururu wa taarifa, n.k.)
//! - `thamani` — thamani ya nodi (nambari iliyosimbwa au aina iliyosimbwa)
//! - `jina_off` — kianzio cha jina kwenye dimbwi la majina
//!
//! ## Dimbwi la majina
//!
//! Majina yote (vitambulisho, aina, mifuatano) huhifadhiwa kwenye dimbwi la
//! baiti lililoshinikizwa.  Kila jina huhifadhiwa kama mfuato wa baiti
//! ukimalizika kwa `\0`.

use crate::diagnostics::SourceSpan;

// ============================================================================
// Aina za nodi
// ============================================================================

/// Aina za nodi za mti wa sintaksia wa Swa.
///
/// Thamani za nambari zinalingana na zile zinazotumiwa na mchanganuzi
/// (`src/parser/mod.rs`) na kiteremshi (`src/ir/lower.rs`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum AinaYaNodi {
    /// Kitenzi cha programu — nodi ya mzizi.
    Programu = 1,
    /// Ufafanuzi wa kazi: `W0 jina() { ... }`
    Kazi = 2,
    /// Taarifa ya kurudisha: `rudisha usemi`
    Rudisha = 3,
    /// Nambari halisi: `42`, `0xFF`
    Nambari = 4,
    /// Kitambulisho: jina la kigezo, kazi, au aina
    Kitambulisho = 5,
    /// Opereta ya kujumlisha: `+`
    Jumlisha = 6,
    /// Opereta ya kutofautisha: `-`
    Tofauti = 7,
    /// Mwito wa kazi: `jina(hoja)`
    Wito = 8,
    /// Taarifa ya masharti: `kama (sharti) { ... } sivyo { ... }`
    Kama = 9,
    /// Mzunguko wa wakati: `wakati (sharti) { ... }`
    Wakati = 10,
    /// Tangazo la kigezo: `N32 x = 5`
    Tangazo = 11,
    /// Ufafanuzi wa muundo: `muundo Jina { ... }`
    Muundo = 12,
    /// Sehemu ya muundo: `N32 x`
    Sehemu = 13,
    /// Taarifa ya chagua: `chagua (x) { hali 1: ... kawaida: ... }`
    Chagua = 14,
    /// Kifungu cha hali ndani ya chagua
    Hali = 15,
    /// Taarifa ya kuvunja: `vunja`
    Vunja = 16,
    /// Taarifa ya kuendelea: `endelea`
    Endelea = 17,
    /// Usemi wa kutenga kumbukumbu: `tenga(ukubwa)`
    Tenga = 18,
    /// Taarifa ya kuachilia kumbukumbu: `achilia(kiashiria)`
    Achilia = 19,
    /// Opereta ya usawa: `==`
    Sawa = 20,
    /// Opereta ya tofauti: `!=`
    TofautiSi = 21,
    /// Opereta ya chini: `<`
    Chini = 22,
    /// Opereta ya juu: `>`
    Juu = 23,
    /// Opereta ya chini-sawa: `<=`
    ChiniSawa = 24,
    /// Opereta ya juu-sawa: `>=`
    JuuSawa = 25,
    /// Opereta ya mantiki NA: `&&`
    Na = 26,
    /// Opereta ya mantiki AU: `||`
    Au = 27,
    /// Opereta ya kukanusha: `!`
    Si = 28,
    /// Opereta ya kutaja anwani: `&`
    Taja = 29,
    /// Opereta ya kumbuka (nyoosha): `*`
    Kumbuka = 30,
    /// Opereta ya kuzidisha: `*`
    Zidisha = 31,
    /// Opereta ya kugawanya: `/`
    Gawanya = 32,
    /// Ufikiaji wa sehemu kwa nukta: `x.sehemu`
    SehemuDot = 33,
    /// Ufikiaji wa sehemu kwa mshale: `x->sehemu`
    SehemuMshale = 34,
    /// Tangazo la kigezo cha ulimwengu: `N32 JINA = 0`
    TangazoUlimwengu = 35,
    /// Taarifa ya ugawaji: `x = 5`
    Asimilia = 37,
    /// Usemi wa safu: `[a, b, c]`
    Safu = 38,
    /// Mfuato halisi: `"habari"`
    Mfuato = 40,
}

impl AinaYaNodi {
    /// Badilisha kutoka nambari ghafi hadi `AinaYaNodi`.
    /// Hakiacha kufanya kazi kwa thamani isiyojulikana.
    pub fn kutoka_nambari(n: u32) -> Self {
        match n {
            1 => Self::Programu,
            2 => Self::Kazi,
            3 => Self::Rudisha,
            4 => Self::Nambari,
            5 => Self::Kitambulisho,
            6 => Self::Jumlisha,
            7 => Self::Tofauti,
            8 => Self::Wito,
            9 => Self::Kama,
            10 => Self::Wakati,
            11 => Self::Tangazo,
            12 => Self::Muundo,
            13 => Self::Sehemu,
            14 => Self::Chagua,
            15 => Self::Hali,
            16 => Self::Vunja,
            17 => Self::Endelea,
            18 => Self::Tenga,
            19 => Self::Achilia,
            20 => Self::Sawa,
            21 => Self::TofautiSi,
            22 => Self::Chini,
            23 => Self::Juu,
            24 => Self::ChiniSawa,
            25 => Self::JuuSawa,
            26 => Self::Na,
            27 => Self::Au,
            28 => Self::Si,
            29 => Self::Taja,
            30 => Self::Kumbuka,
            31 => Self::Zidisha,
            32 => Self::Gawanya,
            33 => Self::SehemuDot,
            34 => Self::SehemuMshale,
            35 => Self::TangazoUlimwengu,
            37 => Self::Asimilia,
            38 => Self::Safu,
            40 => Self::Mfuato,
            nyingine => panic!("aina ya nodi isiyojulikana: {}", nyingine),
        }
    }

    /// Rudisha jina la aina ya nodi kwa Kiswahili.
    pub fn jina(&self) -> &'static str {
        match self {
            Self::Programu => "PROGRAMU",
            Self::Kazi => "KAZI",
            Self::Rudisha => "RUDISHA",
            Self::Nambari => "NAMBARI",
            Self::Kitambulisho => "KITAMBULISHO",
            Self::Jumlisha => "JUMLISHA",
            Self::Tofauti => "TOFAUTI",
            Self::Wito => "WITO",
            Self::Kama => "KAMA",
            Self::Wakati => "WAKATI",
            Self::Tangazo => "TANGAZO",
            Self::Muundo => "MUUNDO",
            Self::Sehemu => "SEHEMU",
            Self::Chagua => "CHAGUA",
            Self::Hali => "HALI",
            Self::Vunja => "VUNJA",
            Self::Endelea => "ENDELEA",
            Self::Tenga => "TENGA",
            Self::Achilia => "ACHILIA",
            Self::Sawa => "SAWA",
            Self::TofautiSi => "TOFAUTI_SI",
            Self::Chini => "CHINI",
            Self::Juu => "JUU",
            Self::ChiniSawa => "CHINI_SAWA",
            Self::JuuSawa => "JUU_SAWA",
            Self::Na => "NA",
            Self::Au => "AU",
            Self::Si => "SI",
            Self::Taja => "TAJA",
            Self::Kumbuka => "KUMBUKA",
            Self::Zidisha => "ZIDISHA",
            Self::Gawanya => "GAWANYA",
            Self::SehemuDot => "SEHEMU_DOT",
            Self::SehemuMshale => "SEHEMU_MSHALE",
            Self::TangazoUlimwengu => "TANGAZO_ULIMWENGU",
            Self::Asimilia => "ASIMILIA",
            Self::Safu => "SAFU",
            Self::Mfuato => "MFUATANO",
        }
    }
}

// ============================================================================
// Dimbwi la majina
// ============================================================================

/// Dimbwi la majina lenye mifuatano iliyoshinikizwa.
///
/// Kila jina huhifadhiwa kama baiti za UTF-8 ukimalizika kwa `\0`.
/// Fahirisi ya kianzio cha jina ndiyo inayorejelewa na nodi.
#[derive(Debug, Clone, Default)]
pub struct DimbwiLaMajina {
    baiti: Vec<u8>,
}

impl DimbwiLaMajina {
    /// Unda dimbwi jipya tupu.
    pub fn mpya() -> Self {
        Self { baiti: Vec::new() }
    }

    /// Hifadhi jina kwenye dimbwi na urudishe kianzio chake.
    pub fn hifadhi(&mut self, jina: &str) -> i32 {
        let off = self.baiti.len() as i32;
        self.baiti.extend_from_slice(jina.as_bytes());
        self.baiti.push(0);
        off
    }

    /// Hifadhi baiti ghafi (zisizo na kimalizio) na urudishe kianzio.
    pub fn hifadhi_baiti(&mut self, data: &[u8]) -> i32 {
        let off = self.baiti.len() as i32;
        self.baiti.extend_from_slice(data);
        off
    }

    /// Soma jina kutoka kwenye dimbwi kwa kutumia kianzio.
    /// Hurejesha mfuato tupu kama kianzio ni hasi au nje ya mipaka.
    pub fn soma(&self, off: i32) -> String {
        if off < 0 || off as usize >= self.baiti.len() {
            return String::new();
        }
        let start = off as usize;
        let mut end = start;
        while end < self.baiti.len() && self.baiti[end] != 0 {
            end += 1;
        }
        String::from_utf8_lossy(&self.baiti[start..end]).into_owned()
    }

    /// Soma baiti ghafi kwa urefu maalumu.
    pub fn soma_baiti(&self, off: i32, urefu: usize) -> &[u8] {
        if off < 0 || off as usize >= self.baiti.len() {
            return &[];
        }
        let start = off as usize;
        let end = (start + urefu).min(self.baiti.len());
        &self.baiti[start..end]
    }

    /// Rudisha urefu wa dimbwi kwa baiti.
    pub fn urefu(&self) -> usize {
        self.baiti.len()
    }

    /// Rudisha kielekezi cha baiti za ndani.
    pub fn baiti_za_ndani(&self) -> &[u8] {
        &self.baiti
    }
}

// ============================================================================
// Nodi ya AST
// ============================================================================

/// Nodi moja katika mti wa sintaksia.
///
/// Inatumia uwakilishi wa safu bapa (parallel arrays) kwa utendaji bora
/// na urahisi wa kushiriki kumbukumbu.
#[derive(Debug, Clone)]
pub struct NodiYaAst {
    /// Aina ya nodi
    pub aina: AinaYaNodi,
    /// Fahirisi ya nodi-mtoto wa kushoto
    pub kushoto: i32,
    /// Fahirisi ya nodi-mtoto wa kulia
    pub kulia: i32,
    /// Fahirisi ya nodi-mtoto wa tatu
    pub tatu: i32,
    /// Fahirisi ya nodi-mtoto wa nne / ndugu
    pub nne: i32,
    /// Thamani iliyosimbwa (nambari, aina, au kianzio cha dimbwi)
    pub thamani: i32,
    /// Kianzio cha jina kwenye dimbwi (kama ipo)
    pub jina_off: i32,
    /// Nafasi ya nodi katika msimbo wa chanzo
    pub eneo: SourceSpan,
}

impl NodiYaAst {
    /// Unda nodi mpya yenye thamani chaguo-msingi.
    pub fn mpya(aina: AinaYaNodi) -> Self {
        Self {
            aina,
            kushoto: -1,
            kulia: -1,
            tatu: -1,
            nne: -1,
            thamani: 0,
            jina_off: 0,
            eneo: SourceSpan::point(0, 0),
        }
    }

    /// Je, nodi haina mtoto?
    pub fn haina_mtoto(&self) -> bool {
        self.kushoto == -1 && self.kulia == -1 && self.tatu == -1 && self.nne == -1
    }

    /// Idadi ya watoto wasio hasi.
    pub fn idadi_ya_watoto(&self) -> u32 {
        let mut n = 0u32;
        if self.kushoto >= 0 { n += 1; }
        if self.kulia >= 0 { n += 1; }
        if self.tatu >= 0 { n += 1; }
        if self.nne >= 0 { n += 1; }
        n
    }
}

// ============================================================================
// Kijenzi cha AST (flat arrays)
// ============================================================================

/// Kijenzi cha mti wa sintaksia kinachotumia safu bapa.
///
/// Huu ndio umbile halisi linalotumiwa na mchanganuzi na kiteremshi.
/// Nodi huhifadhiwa kwenye safu sambamba kwa utendaji bora wa kumbukumbu.
#[derive(Debug, Clone, Default)]
pub struct KijenziChaAst {
    pub aina: Vec<u32>,
    pub kushoto: Vec<i32>,
    pub kulia: Vec<i32>,
    pub tatu: Vec<i32>,
    pub nne: Vec<i32>,
    pub thamani: Vec<i32>,
    pub jina_off: Vec<i32>,
    pub dimbwi: DimbwiLaMajina,
}

impl KijenziChaAst {
    /// Unda kijenzi kipya tupu.
    pub fn mpya() -> Self {
        Self::default()
    }

    /// Ongeza nodi mpya na urudishe fahirisi yake.
    ///
    /// Fahirisi inayorejeshwa hutumika kurejelea nodi hii kutoka kwa nodi
    /// nyingine (kama mtoto au ndugu).
    pub fn ongeza_nodi(&mut self, aina: AinaYaNodi) -> i32 {
        let idx = self.aina.len() as i32;
        self.aina.push(aina as u32);
        self.kushoto.push(-1);
        self.kulia.push(-1);
        self.tatu.push(-1);
        self.nne.push(-1);
        self.thamani.push(0);
        self.jina_off.push(0);
        idx
    }

    /// Ongeza nodi yenye thamani iliyosimbwa (kwa aina za nambari).
    pub fn ongeza_nodi_ya_aina(
        &mut self,
        aina: AinaYaNodi,
        familia: u32,
        upana: u32,
        mshale: u32,
    ) -> i32 {
        let encoded = (((familia & 255) << 8) | (upana & 255) | (mshale & 1)) as i32;
        let idx = self.ongeza_nodi(aina);
        self.thamani[idx as usize] = encoded;
        idx
    }

    /// Ongeza nodi yenye jina kwenye dimbwi.
    pub fn ongeza_nodi_ya_jina(&mut self, aina: AinaYaNodi, jina: &str) -> i32 {
        let jina_off = self.dimbwi.hifadhi(jina);
        let idx = self.ongeza_nodi(aina);
        self.jina_off[idx as usize] = jina_off;
        idx
    }

    /// Weka mtoto wa kushoto wa nodi.
    pub fn weka_kushoto(&mut self, mzazi: i32, mtoto: i32) {
        self.kushoto[mzazi as usize] = mtoto;
    }

    /// Weka mtoto wa kulia wa nodi.
    pub fn weka_kulia(&mut self, mzazi: i32, mtoto: i32) {
        self.kulia[mzazi as usize] = mtoto;
    }

    /// Weka mtoto wa tatu wa nodi.
    pub fn weka_tatu(&mut self, mzazi: i32, mtoto: i32) {
        self.tatu[mzazi as usize] = mtoto;
    }

    /// Weka ndugu (mtoto wa nne) wa nodi.
    pub fn weka_nne(&mut self, mzazi: i32, ndugu: i32) {
        self.nne[mzazi as usize] = ndugu;
    }

    /// Rudisha aina ya nodi.
    pub fn aina_ya_nodi(&self, idx: i32) -> AinaYaNodi {
        if idx < 0 || idx as usize >= self.aina.len() {
            // Kurudisha Kitambulisho kwa nodi batili — mpango wa usalama.
            return AinaYaNodi::Kitambulisho;
        }
        AinaYaNodi::kutoka_nambari(self.aina[idx as usize])
    }

    /// Je, nodi ni batili?
    pub fn ni_batili(&self, idx: i32) -> bool {
        idx < 0 || idx as usize >= self.aina.len()
    }

    /// Rudisha idadi ya nodi.
    pub fn idadi(&self) -> usize {
        self.aina.len()
    }

    /// Badilisha kuwa safu bapa kwa matumizi ya kiteremshi.
    pub fn kuwa_safu_bapa(self) -> (Vec<u32>, Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>, Vec<u8>, usize) {
        let idadi = self.aina.len();
        (
            self.aina,
            self.kushoto,
            self.kulia,
            self.tatu,
            self.nne,
            self.thamani,
            self.jina_off,
            self.dimbwi.baiti_za_ndani().to_vec(),
            idadi,
        )
    }
}

// ============================================================================
// Majaribio
// ============================================================================

#[cfg(test)]
mod majaribio {
    use super::*;

    #[test]
    fn jaribio_dimbwi_tupu() {
        let d = DimbwiLaMajina::mpya();
        assert_eq!(d.urefu(), 0);
        assert_eq!(d.soma(0), "");
    }

    #[test]
    fn jaribio_hifadhi_na_soma() {
        let mut d = DimbwiLaMajina::mpya();
        let off1 = d.hifadhi("x");
        let off2 = d.hifadhi("N32");
        assert_eq!(d.soma(off1), "x");
        assert_eq!(d.soma(off2), "N32");
    }

    #[test]
    fn jaribio_soma_kianzio_batili() {
        let d = DimbwiLaMajina::mpya();
        assert_eq!(d.soma(-1), "");
        assert_eq!(d.soma(1000), "");
    }

    #[test]
    fn jaribio_aina_ya_nodi_kutoka_nambari() {
        assert_eq!(AinaYaNodi::kutoka_nambari(1), AinaYaNodi::Programu);
        assert_eq!(AinaYaNodi::kutoka_nambari(2), AinaYaNodi::Kazi);
        assert_eq!(AinaYaNodi::kutoka_nambari(40), AinaYaNodi::Mfuato);
    }

    #[test]
    #[should_panic]
    fn jaribio_aina_ya_nodi_batili() {
        AinaYaNodi::kutoka_nambari(99);
    }

    #[test]
    fn jaribio_aina_jina_kwa_kiswahili() {
        assert_eq!(AinaYaNodi::Kazi.jina(), "KAZI");
        assert_eq!(AinaYaNodi::Wakati.jina(), "WAKATI");
        assert_eq!(AinaYaNodi::Kama.jina(), "KAMA");
    }

    #[test]
    fn jaribio_kijenzi_tupu() {
        let k = KijenziChaAst::mpya();
        assert_eq!(k.idadi(), 0);
    }

    #[test]
    fn jaribio_ongeza_nodi() {
        let mut k = KijenziChaAst::mpya();
        let idx = k.ongeza_nodi(AinaYaNodi::Nambari);
        assert_eq!(idx, 0);
        assert_eq!(k.idadi(), 1);
        assert_eq!(k.aina_ya_nodi(idx), AinaYaNodi::Nambari);
    }

    #[test]
    fn jaribio_nodi_mtoto() {
        let mut k = KijenziChaAst::mpya();
        let mzazi = k.ongeza_nodi(AinaYaNodi::Kazi);
        let mtoto = k.ongeza_nodi_ya_jina(AinaYaNodi::Kitambulisho, "kuu");
        k.weka_kushoto(mzazi, mtoto);
        assert_eq!(k.kushoto[mzazi as usize], mtoto);
        assert_eq!(k.dimbwi.soma(k.jina_off[mtoto as usize]), "kuu");
    }

    #[test]
    fn jaribio_nodi_haina_mtoto() {
        let n = NodiYaAst::mpya(AinaYaNodi::Nambari);
        assert!(n.haina_mtoto());
        assert_eq!(n.idadi_ya_watoto(), 0);
    }

    #[test]
    fn jaribio_nodi_ina_watoto() {
        let mut n = NodiYaAst::mpya(AinaYaNodi::Kazi);
        n.kushoto = 0;
        n.kulia = 1;
        assert!(!n.haina_mtoto());
        assert_eq!(n.idadi_ya_watoto(), 2);
    }

    #[test]
    fn jaribio_kuwa_safu_bapa() {
        let mut k = KijenziChaAst::mpya();
        k.ongeza_nodi(AinaYaNodi::Programu);
        let (aina, kushoto, kulia, tatu, nne, thamani, jina_off, dimbwi, idadi) = k.kuwa_safu_bapa();
        assert_eq!(idadi, 1);
        assert_eq!(aina[0], AinaYaNodi::Programu as u32);
        assert_eq!(kushoto.len(), 1);
        assert_eq!(kulia.len(), 1);
        assert_eq!(tatu.len(), 1);
        assert_eq!(nne.len(), 1);
        assert_eq!(thamani.len(), 1);
        assert_eq!(jina_off.len(), 1);
        assert!(dimbwi.is_empty());
    }
}
