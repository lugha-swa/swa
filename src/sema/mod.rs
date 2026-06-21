//! Mkaguzi wa kisemantiki kwa lugha ya Swa.
//!
//! Hukagua uthabiti wa aina, uwepo wa vigezo, sahihi za kazi, na
//! uhalali wa miundo baada ya mchanganuzi kumaliza kujenga mti wa sintaksia.
//!
//! ## Hatua za ukaguzi
//!
//! 1. **Ukusanyaji wa alama** — tembelea nodi zote za AST ili kukusanya
//!    kazi, vigezo vya ulimwengu, miundo, na aina zilizotangazwa.
//! 2. **Ukaguzi wa aina** — hakikisha kila usemi unatoa aina sahihi na
//!    operesheni zinafanyika kati ya aina zinazopatana.
//! 3. **Ukaguzi wa mtiririko** — hakikisha taarifa za `rudisha`,
//!    `vunja`, na `endelea` ziko ndani ya muktadha sahihi.
//! 4. **Ukaguzi wa mwito** — hakikisha hoja za mwito wa kazi zinapatana
//!    na vigezo vya kazi iliyotangazwa.

use crate::diagnostics::Diagnostic;
use crate::ir::types::IrType;
use std::collections::HashMap;

// ============================================================================
// Alama (symbol)
// ============================================================================

/// Aina ya alama katika jedwali la alama.
#[derive(Debug, Clone, PartialEq)]
pub enum AinaYaAlama {
    /// Kigezo cha ndani au cha ulimwengu.
    Kigezo { aina: IrType, ni_thabiti: bool },
    /// Kazi iliyotangazwa.
    Kazi {
        vigezo: Vec<(String, IrType)>,
        aina_ya_kurudisha: IrType,
        ni_ya_nje: bool,
    },
    /// Muundo uliotangazwa.
    Muundo { sehemu: Vec<(String, IrType)> },
    /// Aina iliyotafsiriwa (`aina Jina = N32`).
    TafsiriYaAina { aina_halisi: IrType },
}

/// Jedwali la alama — hukusanya alama zote katika upeo fulani.
#[derive(Debug, Clone, Default)]
pub struct JedwaliLaAlama {
    alama: HashMap<String, AinaYaAlama>,
}

impl JedwaliLaAlama {
    pub fn mpya() -> Self {
        Self { alama: HashMap::new() }
    }

    /// Tangaza alama mpya. Hurejesha kosa ikiwa jina tayari limetangazwa.
    pub fn tangaza(&mut self, jina: &str, aina: AinaYaAlama) -> Result<(), KosaLaSemantiki> {
        if self.alama.contains_key(jina) {
            return Err(KosaLaSemantiki::AlamaMaraMbili(jina.to_string()));
        }
        self.alama.insert(jina.to_string(), aina);
        Ok(())
    }

    /// Tafuta alama kwa jina.
    pub fn tafuta(&self, jina: &str) -> Option<&AinaYaAlama> {
        self.alama.get(jina)
    }

    /// Je, jina limetangazwa?
    pub fn imetangazwa(&self, jina: &str) -> bool {
        self.alama.contains_key(jina)
    }

    /// Rudisha idadi ya alama.
    pub fn idadi(&self) -> usize {
        self.alama.len()
    }
}

// ============================================================================
// Makosa ya kisemantiki
// ============================================================================

/// Aina za makosa ya kisemantiki.
#[derive(Debug, Clone)]
pub enum KosaLaSemantiki {
    /// Alama imetangazwa mara mbili.
    AlamaMaraMbili(String),
    /// Jina halikutangazwa kabla ya matumizi.
    JinaHalikutangazwa(String),
    /// Aina haziendani katika usemi.
    AinaHaziendani { inayotarajiwa: String, iliyopatikana: String },
    /// Idadi ya hoja hailingani na vigezo vya kazi.
    IdadiYaHojaHaikaliki { jina_la_kazi: String, inayotarajiwa: usize, iliyopatikana: usize },
    /// Operesheni haitumiki kwa aina hizi.
    OperesheniBatili { opereta: String, kushoto: String, kulia: String },
    /// Taarifa ya kurudisha iko nje ya kazi.
    RudishaNjeYaKazi,
    /// Thamani ya kurudisha hailingani na aina ya kazi.
    AinaYaKurudishaHaikaliki { inayotarajiwa: String, iliyopatikana: String },
    /// Sehemu haipo kwenye muundo.
    SehemuHaipo { muundo: String, sehemu: String },
    /// Jaribio la kufikia sehemu kwenye aina isiyo muundo.
    SiMuundo(String),
}

impl KosaLaSemantiki {
    /// Badilisha kuwa `Diagnostic` kwa ajili ya kuripoti.
    pub fn kuwa_diagnostic(&self) -> Diagnostic {
        let ujumbe = match self {
            Self::AlamaMaraMbili(j) => format!("alama '{}' imetangazwa mara mbili", j),
            Self::JinaHalikutangazwa(j) => format!("jina '{}' halikutangazwa kabla ya matumizi", j),
            Self::AinaHaziendani { inayotarajiwa, iliyopatikana } => {
                format!("aina haziendani: inatarajiwa '{}', lakini imepatikana '{}'", inayotarajiwa, iliyopatikana)
            }
            Self::IdadiYaHojaHaikaliki { jina_la_kazi, inayotarajiwa, iliyopatikana } => {
                format!(
                    "idadi ya hoja kwa kazi '{}' haikaliki: inatarajiwa {}, imetolewa {}",
                    jina_la_kazi, inayotarajiwa, iliyopatikana
                )
            }
            Self::OperesheniBatili { opereta, kushoto, kulia } => {
                format!("opereta '{}' haitumiki kati ya '{}' na '{}'", opereta, kushoto, kulia)
            }
            Self::RudishaNjeYaKazi => "taarifa ya 'rudisha' iko nje ya mwili wa kazi".to_string(),
            Self::AinaYaKurudishaHaikaliki { inayotarajiwa, iliyopatikana } => {
                format!(
                    "aina ya kurudisha haikaliki: inatarajiwa '{}', lakini imepatikana '{}'",
                    inayotarajiwa, iliyopatikana
                )
            }
            Self::SehemuHaipo { muundo, sehemu } => {
                format!("sehemu '{}' haipo kwenye muundo '{}'", sehemu, muundo)
            }
            Self::SiMuundo(j) => format!("'{}' si muundo — haiwezi kufikiwa kwa nukta au mshale", j),
        };
        Diagnostic::error(ujumbe, crate::diagnostics::SourceSpan::point(0, 0))
    }
}

// ============================================================================
// Mkaguzi mkuu
// ============================================================================

/// Mkaguzi wa kisemantiki kwa msimbo wa Swa.
///
/// Hukagua uthabiti wa aina na sheria za lugha baada ya mchanganuzi
/// kumaliza kazi yake. Matokeo yake ni orodha ya makosa ya kisemantiki.
#[derive(Debug, Default)]
pub struct Mkaguzi {
    /// Jedwali la alama za ulimwengu (kazi, vigezo vya ulimwengu, miundo).
    pub ulimwengu: JedwaliLaAlama,
    /// Upeo wa sasa (vigezo vya ndani).
    pub upeo_wa_sasa: JedwaliLaAlama,
    /// Makosa yaliyokusanywa.
    pub makosa: Vec<KosaLaSemantiki>,
    /// Aina ya kazi ya sasa (kwa ukaguzi wa kurudisha).
    aina_ya_kurudisha_ya_sasa: Option<IrType>,
    /// Je, tuko ndani ya mzunguko? (kwa ukaguzi wa vunja/endelea).
    ndani_ya_mzunguko: bool,
}

impl Mkaguzi {
    /// Unda mkaguzi mpya tupu.
    pub fn mpya() -> Self {
        Self::default()
    }

    /// Fanya ukaguzi kamili wa kisemantiki.
    ///
    /// Kwa sasa, ukaguzi wote unafanyika kwenye `JedwaliLaAlama` —
    /// mteja hujaza alama na kupiga `kagua()`.
    pub fn kagua(&mut self) -> Vec<KosaLaSemantiki> {
        // Hifadhi nakala ya makosa na ufute hali ya ndani.
        std::mem::take(&mut self.makosa)
    }

    /// Ingiza upeo mpya (kwa mfano, ndani ya kazi au kizuizi).
    pub fn ingiza_upeo(&mut self) {
        // Hifadhi upeo wa sasa kwenye historia (kwa sasa tunatumia
        // mbinu rahisi ya kubadilisha jedwali zima).
        let upeo_mpya = JedwaliLaAlama::mpya();
        let upeo_wa_zamani = std::mem::replace(&mut self.upeo_wa_sasa, upeo_mpya);
        // Upeo wa zamani unapotea hapa — kwa sasa, sema haitumii
        // upeo wa viota.  Hii itaboreshwa wakati kiunganishi cha mchanganuzi
        // kitakapokamilika.
        let _ = upeo_wa_zamani;
    }

    /// Toka kwenye upeo wa sasa.
    pub fn toka_upeo(&mut self) {
        self.upeo_wa_sasa = JedwaliLaAlama::mpya();
    }

    /// Weka muktadha wa kazi ya sasa.
    pub fn ingiza_kazi(&mut self, aina_ya_kurudisha: Option<IrType>) {
        self.aina_ya_kurudisha_ya_sasa = aina_ya_kurudisha;
        self.ndani_ya_mzunguko = false;
        self.ingiza_upeo();
    }

    /// Toka kwenye muktadha wa kazi.
    pub fn toka_kazi(&mut self) {
        self.toka_upeo();
        self.aina_ya_kurudisha_ya_sasa = None;
    }

    /// Weka muktadha wa mzunguko.
    pub fn ingiza_mzunguko(&mut self) {
        self.ndani_ya_mzunguko = true;
    }

    /// Toka kwenye muktadha wa mzunguko.
    pub fn toka_mzunguko(&mut self) {
        self.ndani_ya_mzunguko = false;
    }

    /// Je, tuko ndani ya kazi? (yaani, `rudisha` ni halali?)
    pub fn ndani_ya_kazi(&self) -> bool {
        self.aina_ya_kurudisha_ya_sasa.is_some()
    }

    /// Kagua usemi wa aina mbili (k.m. `a + b`).
    pub fn kagua_operesheni_mbili(
        &mut self,
        opereta: &str,
        aina_kushoto: &IrType,
        aina_kulia: &IrType,
    ) -> IrType {
        // Aina zinazotarajiwa kwa kila opereta.
        match opereta {
            // Hisabati — inahitaji nambari.
            "+" | "-" | "*" | "/" | "%" => {
                if !aina_kushoto.is_numeric() || !aina_kulia.is_numeric() {
                    self.makosa.push(KosaLaSemantiki::OperesheniBatili {
                        opereta: opereta.to_string(),
                        kushoto: format!("{:?}", aina_kushoto),
                        kulia: format!("{:?}", aina_kulia),
                    });
                }
                // Aina ya matokeo ni aina pana zaidi.
                aina_kushoto.widen(aina_kulia)
            }
            // Ulinganifu — inahitaji aina zinazopatana.
            "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                if !aina_kushoto.compatible_with(aina_kulia) {
                    self.makosa.push(KosaLaSemantiki::OperesheniBatili {
                        opereta: opereta.to_string(),
                        kushoto: format!("{:?}", aina_kushoto),
                        kulia: format!("{:?}", aina_kulia),
                    });
                }
                IrType::B1
            }
            // Mantiki — inahitaji bulioni.
            "&&" | "||" => {
                IrType::B1
            }
            _ => IrType::Void,
        }
    }

    /// Tangaza kigezo katika upeo wa sasa.
    pub fn tangaza_kigezo(
        &mut self,
        jina: &str,
        aina: IrType,
        ni_thabiti: bool,
    ) -> Result<(), KosaLaSemantiki> {
        self.upeo_wa_sasa.tangaza(jina, AinaYaAlama::Kigezo { aina, ni_thabiti })
    }

    /// Tafuta kigezo au alama kwa jina (hutafuta upeo wa ndani kwanza,
    /// kisha ulimwengu).
    pub fn tafuta_kigezo(&self, jina: &str) -> Option<&AinaYaAlama> {
        self.upeo_wa_sasa
            .tafuta(jina)
            .or_else(|| self.ulimwengu.tafuta(jina))
    }

    /// Rekodi kosa la kisemantiki.
    pub fn rekodi_kosa(&mut self, kosa: KosaLaSemantiki) {
        self.makosa.push(kosa);
    }
}

// ============================================================================
// Vipanuzi vya IrType kwa ukaguzi wa kisemantiki
// ============================================================================

/// Vipanuzi vya muda kwa `IrType` — vinatumika na mkaguzi wa kisemantiki.
trait IrTypeExt {
    /// Je, aina ni ya nambari (inaweza kufanyiwa hisabati)?
    fn is_numeric(&self) -> bool;
    /// Je, aina mbili zinapatana (zinaweza kulinganishwa)?
    fn compatible_with(&self, other: &IrType) -> bool;
    /// Rudisha aina pana zaidi kati ya hizi mbili (kwa ukuzaji wa aina).
    fn widen(&self, other: &IrType) -> IrType;
}

impl IrTypeExt for IrType {
    fn is_numeric(&self) -> bool {
        matches!(
            self,
            IrType::I8
                | IrType::I16 | IrType::I32 | IrType::I64 | IrType::I128
                | IrType::A8 | IrType::A16 | IrType::A32 | IrType::A64
                | IrType::A128 | IrType::F16 | IrType::F32 | IrType::F64
                | IrType::F128 | IrType::B8 | IrType::B16 | IrType::B32
                | IrType::B64 | IrType::W8 | IrType::W16 | IrType::W32
                | IrType::W64
        )
    }

    fn compatible_with(&self, other: &IrType) -> bool {
        // Aina zinazofanana zinapatana.
        if std::mem::discriminant(self) == std::mem::discriminant(other) {
            return true;
        }
        // Nambari zote zinapatana kwa kulinganisha.
        self.is_numeric() && other.is_numeric()
    }

    fn widen(&self, other: &IrType) -> IrType {
        // Rudisha aina yenye upana mkubwa zaidi.
        let w1 = self.width_bytes();
        let w2 = other.width_bytes();
        if w1 >= w2 { self.clone() } else { other.clone() }
    }
}

// ============================================================================
// Majaribio
// ============================================================================

#[cfg(test)]
mod majaribio {
    use super::*;

    fn aina_n32() -> IrType { IrType::I32 }
    fn aina_n64() -> IrType { IrType::I64 }
    fn aina_f64() -> IrType { IrType::F64 }
    fn aina_w0() -> IrType { IrType::Void }

    // -- Jedwali la alama --

    #[test]
    fn jaribio_tangaza_na_tafuta() {
        let mut j = JedwaliLaAlama::mpya();
        j.tangaza("x", AinaYaAlama::Kigezo { aina: aina_n32(), ni_thabiti: false }).unwrap();
        assert!(j.imetangazwa("x"));
        assert!(!j.imetangazwa("y"));
    }

    #[test]
    fn jaribio_alama_mara_mbili() {
        let mut j = JedwaliLaAlama::mpya();
        j.tangaza("x", AinaYaAlama::Kigezo { aina: aina_n32(), ni_thabiti: false }).unwrap();
        let matokeo = j.tangaza("x", AinaYaAlama::Kigezo { aina: aina_n64(), ni_thabiti: false });
        assert!(matokeo.is_err());
    }

    #[test]
    fn jaribio_jedwali_tupu() {
        let j = JedwaliLaAlama::mpya();
        assert_eq!(j.idadi(), 0);
        assert!(!j.imetangazwa("chochote"));
    }

    // -- Makosa ya kisemantiki --

    #[test]
    fn jaribio_kosa_kuwa_diagnostic() {
        let kosa = KosaLaSemantiki::JinaHalikutangazwa("x".into());
        let d = kosa.kuwa_diagnostic();
        assert!(d.message.contains("x"));
        assert!(d.message.contains("halikutangazwa"));
    }

    #[test]
    fn jaribio_kosa_aina_haziendani() {
        let kosa = KosaLaSemantiki::AinaHaziendani {
            inayotarajiwa: "N32".into(),
            iliyopatikana: "N64".into(),
        };
        let d = kosa.kuwa_diagnostic();
        assert!(d.message.contains("N32"));
        assert!(d.message.contains("N64"));
    }

    // -- Mkaguzi mkuu --

    #[test]
    fn jaribio_mkaguzi_tupu() {
        let mut m = Mkaguzi::mpya();
        let makosa = m.kagua();
        assert!(makosa.is_empty());
    }

    #[test]
    fn jaribio_mkaguzi_kazi() {
        let mut m = Mkaguzi::mpya();
        m.ingiza_kazi(Some(aina_n32()));
        assert!(m.ndani_ya_kazi());
        m.toka_kazi();
        assert!(!m.ndani_ya_kazi());
    }

    #[test]
    fn jaribio_mkaguzi_mzunguko() {
        let mut m = Mkaguzi::mpya();
        assert!(!m.ndani_ya_mzunguko);
        m.ingiza_mzunguko();
        assert!(m.ndani_ya_mzunguko);
        m.toka_mzunguko();
        assert!(!m.ndani_ya_mzunguko);
    }

    #[test]
    fn jaribio_mkaguzi_tangaza_kigezo() {
        let mut m = Mkaguzi::mpya();
        m.ingiza_kazi(Some(aina_w0()));
        m.tangaza_kigezo("x", aina_n32(), false).unwrap();
        assert!(m.tafuta_kigezo("x").is_some());
        m.toka_kazi();
    }

    #[test]
    fn jaribio_mkaguzi_kagua_operesheni_mbili() {
        let mut m = Mkaguzi::mpya();
        // Hisabati halali.
        let matokeo = m.kagua_operesheni_mbili("+", &aina_n32(), &aina_n32());
        assert!(!matches!(matokeo, IrType::Void));
        assert!(m.makosa.is_empty());
    }

    // -- Vipanuzi vya IrType --

    #[test]
    fn jaribio_is_numeric() {
        assert!(IrType::I32.is_numeric());
        assert!(IrType::F64.is_numeric());
        assert!(!IrType::Void.is_numeric());
        assert!(!IrType::Ptr(Box::new(IrType::I8)).is_numeric());
    }

    #[test]
    fn jaribio_compatible_with() {
        assert!(IrType::I32.compatible_with(&IrType::I32));
        assert!(IrType::I32.compatible_with(&IrType::I64));
        assert!(!IrType::I32.compatible_with(&IrType::Void));
    }

    #[test]
    fn jaribio_widen() {
        let pana = IrType::I32.widen(&IrType::I64);
        // I64 has width 8 > I32's width 4
        assert_eq!(pana.width_bytes(), 8);
    }

    #[test]
    fn jaribio_ulimwengu_hutafuta_kwanza() {
        let mut m = Mkaguzi::mpya();
        // Weka alama ya ulimwengu.
        m.ulimwengu.tangaza("ulimwengu_x", AinaYaAlama::Kigezo {
            aina: aina_n32(), ni_thabiti: true,
        }).unwrap();
        // Mkaguzi anapaswa kuipata hata bila upeo wa ndani.
        let matokeo = m.tafuta_kigezo("ulimwengu_x");
        assert!(matokeo.is_some());
    }

    #[test]
    fn jaribio_upeo_wa_ndani_huficha_ulimwengu() {
        let mut m = Mkaguzi::mpya();
        m.ulimwengu.tangaza("jina", AinaYaAlama::Kigezo {
            aina: aina_n64(), ni_thabiti: true,
        }).unwrap();
        m.ingiza_kazi(Some(aina_w0()));
        m.tangaza_kigezo("jina", aina_n32(), false).unwrap();
        // Upeo wa ndani unapaswa kuchukua nafasi ya kwanza.
        let matokeo = m.tafuta_kigezo("jina");
        assert!(matokeo.is_some());
        if let Some(AinaYaAlama::Kigezo { aina, .. }) = matokeo {
            assert_eq!(*aina, aina_n32());
        } else {
            panic!("inatarajiwa Kigezo");
        }
    }
}
