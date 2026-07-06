//! Kiteremshi cha IR — hubadilisha AST (safu-bapa) ya Swa iliyochanganuliwa kuwa IR ya Swa
//! iliyofafanuliwa katika [`crate::ir`].
//!
//! ## Muundo wa safu-bapa wa AST
//!
//! Mchanganuzi hutoa safu sambamba kadhaa zilizoorodheshwa kwa kitambulisho cha nodi:
//!
//! | Safu          | Aina ya kipengele | Maana                          |
//! |---------------|-------------------|--------------------------------|
//! | `ast_aina`    | `u32`             | Aina ya nodi (mojawapo ya vibadilishasthamani vya `AST_*`) |
//! | `ast_kushoto` | `i32`             | Faharasa ya mtoto wa kushoto/wa kwanza (-1 = hakuna)     |
//! | `ast_kulia`   | `i32`             | Faharasa ya mtoto wa kulia (-1 = hakuna)                 |
//! | `ast_tiga`    | `i32`             | Tawi la sivyo/hatua-ya-mzunguko/mwili (-1 = hakuna)      |
//! | `ast_nne`     | `i32`             | Msururu wa ndugu/msururu wa kuendelea (-1 = hakuna)      |
//! | `ast_thamani` | `i32`             | Halisi ya nambari kamili iliyosimbwa au kukabilisha dimbwi la aina-jina |
//! | `ast_jina_off`| `i32`             | Kukabilisha ndani ya `ast_pool` kwa majina ya vitambulisho |
//! | `ast_pool`    | `u8`              | Dimbwi la mifuatano (majina yenye ncha-tupu, halisi zenye urefu-kiambishi) |
//!
//! ## Mkakati wa kuteremsha
//!
//! Nodi ya mizizi (iliyotengwa mwisho, faharasa `ast_idadi - 1`) daima ni
//! `AST_PROGRAMU` (1).  `ast_kushoto` yake inaelekeza kwa mtoto wa kwanza; watoto
//! wameunganishwa kupitia `ast_nne`.  Kila mtoto ni ama kazi (`AST_KAZI`, 2)
//! au kigezo cha ulimwengu (`AST_TANGAZO_ULIMWENGU`, 35).
//!
//! Kazi zinateremshwa kuwa thamani za [`Function`] zilizo na vizuizi vya msingi.
//! Taarifa hutoa mtiririko-dhibiti (matawi, mizunguko, chagua); misemo
//! hutoa [`ValueId`].  Fupi-hali `&&` / `||` huunda vizuizi vipya.

use super::{BlockId, Const, Function, Instruction, IrBlock, IrGlobal, IrReturnClass, Module, Terminator, ValueId};
use super::types::IrType;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Vibadilishasthamani vya aina-nodi za AST
// ---------------------------------------------------------------------------

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
const AST_NA: u32 = 26;
const AST_AU: u32 = 27;
const AST_SI: u32 = 28;
const AST_TAJA: u32 = 29;
const AST_KUMBUKA: u32 = 30;
const AST_ZIDISHA: u32 = 31;
const AST_GAWANYA: u32 = 32;
const AST_SEHEMU_DOT: u32 = 33;
const AST_SEHEMU_MSHALE: u32 = 34;
const AST_TANGAZO_ULIMWENGU: u32 = 35;
const AST_HAMISHA_KUSHOTO: u32 = 36;
const AST_ASIMILIA: u32 = 37;
const AST_SAFU: u32 = 38;
const AST_HAMISHA_KULIA: u32 = 39;
const AST_MFUATANO: u32 = 40;
const AST_BIT_AU: u32 = 41;
const AST_BIT_NA: u32 = 42;
const AST_TERNARY: u32 = 43;
const AST_KWELI: u32 = 44;
const AST_UONGO: u32 = 45;
const AST_TUPU: u32 = 46;

/// Kialamisho kinachotumiwa katika `ast_kushoto`, `ast_kulia`, `ast_tiga`, na `ast_nne`
/// kuashiria "hakuna mtoto / hakuna ndugu".
const NO_NODE: i32 = -1;

// ---------------------------------------------------------------------------
// AllocInfo — hufuatilia sehemu ya mrundikano wa kigezo chenye jina
// ---------------------------------------------------------------------------

/// Inaelezea kigezo chenye jina ambacho kimetengwa kwenye mrundikano.
#[derive(Debug, Clone)]
struct AllocInfo {
    /// `ValueId` iliyorudishwa na amri ya `Alloca`.
    ptr: ValueId,
    /// Aina ya thamani iliyohifadhiwa (aina ya pointee kwa alloca).
    ty: IrType,
}

// ---------------------------------------------------------------------------
// LoopInfo — mazingira yaliyohifadhiwa kwa `vunja` / `endelea`
// ---------------------------------------------------------------------------

/// Hurekodi kichwa na vizuizi vya kutoka vya mzunguko wa ndani kabisa ili `vunja`
/// na `endelea` ziweze kuzilenga.
#[derive(Debug, Clone, Copy)]
struct LoopInfo {
    /// Kizuizi kinachojaribu sharti la mzunguko (ambapo `endelea` huruka).
    header: BlockId,
    /// Kizuizi kinachofuata mara moja baada ya mzunguko (ambapo `vunja` huruka).
    exit: BlockId,
}

// ---------------------------------------------------------------------------
// Kiteremshi
// ---------------------------------------------------------------------------

/// Mazingira ya kuteremsha AST → IR yenye hali.
///
/// Kiteremshi kinatembea AST kwa kina-kwanza, kikikusanya kazi,
/// vigezo vya ulimwengu, mifuatano, na ufafanuzi wa aina ndani ya [`Module`].
struct Lowerer<'a> {
    // -- Safu za AST (zilizokopwa) -------------------------------------------
    ast_aina: &'a [u32],
    ast_kushoto: &'a [i32],
    ast_kulia: &'a [i32],
    ast_tiga: &'a [i32],
    ast_nne: &'a [i32],
    ast_thamani: &'a [i32],
    ast_jina_off: &'a [i32],
    ast_pool: &'a [u8],

    // -- Vipande vya moduli vilivyokusanywa -----------------------------------
    functions: Vec<Function>,
    globals: Vec<IrGlobal>,
    types: Vec<(String, IrType)>,
    /// Halisi za mifuatano zilizokusanywa: (jina la ishara, baiti ghafi bila kimalizio).
    /// Kila mfuatano wa kipekee hupata jina la ulimwengu la sintetiki kama `@str.0`, `@str.1`, ...
    strings: Vec<(String, Vec<u8>)>,

    // -- Kazi inayojengwa kwa sasa --------------------------------------------
    /// Kazi inayoteremshwa sasa hivi.
    func: Function,

    // -- Msururu wa upeo ------------------------------------------------------
    /// Kila sukuma ni upeo mpya wa kileksia (mwili wa kazi, kizuizi, n.k.).
    /// Tunapotafuta jina tunatembea kutoka upeo wa ndani kabisa kwenda nje.
    scopes: Vec<HashMap<String, AllocInfo>>,

    // -- Rundo la mazingira ya mzunguko ----------------------------------------
    /// Mzunguko wa ndani kabisa uko mwishoni; `vunja` / `endelea` hulenga.
    loops: Vec<LoopInfo>,

    // -- Viheshi --------------------------------------------------------------
    /// Kiheshi cha amri cha ulimwengu: huongezeka monotoniki katika vizuizi vyote.
    inst_counter: usize,
    /// values.len() iliyonaswa mwanzoni mwa kazi — haipaswi kubadilika wakati wa kuteremsha mwili.
    values_initial_len: usize,
    /// Kiheshi cha kitambulisho-kizuizi kinachoongezeka monotoniki kinachotumiwa kwa lebo mpya.
    block_counter: usize,

    /// Sehemu za mrundikano zilizotengwa awali kwa vigezo vya ndani,
    /// zilizowekwa kwa ufunguo wa faharasa ya nodi AST_TANGAZO.
    /// Hujazwa wakati wa kupita-awali katika kazi_ya_kuteremsha ili
    /// kila alloca iwe katika kizuizi cha kuingia; lower_local_decl
    /// hatafuta ValueId iliyotengwa awali badala ya kutoa Alloca mpya.
    pre_allocated_locals: std::collections::HashMap<i32, ValueId>,

    /// Aina za vigezo vya ulimwengu (kwa lower_identifier).
    global_types: std::collections::HashMap<String, IrType>,
    /// Lengo la sret lililotengwa awali kwa wito unaorudisha muundo.
    sret_dest: Option<ValueId>,
}

// ---------------------------------------------------------------------------
// Sehemu ya kuingia ya umma
// ---------------------------------------------------------------------------

/// Teremsha AST ya Swa ya safu-bapa kuwa [`Module`] ya IR.
///
/// # Vigezo
///
/// * `ast_aina`      — safu ya aina-nodi
/// * `ast_kushoto`   — safu ya mtoto wa kushoto / wa kwanza
/// * `ast_kulia`     — safu ya mtoto wa kulia
/// * `ast_tiga`      — safu ya tawi-sivyo / hatua-ya-mzunguko / mwili
/// * `ast_nne`       — safu ya msururu wa ndugu
/// * `ast_thamani`   — thamani ya nambari kamili iliyosimbwa au kukabilisha dimbwi la aina-jina
/// * `ast_jina_off`  — kukabilisha ndani ya `ast_pool` kwa majina ya vitambulisho
/// * `ast_pool`      — baiti za dimbwi la mifuatano
/// * `ast_idadi`     — jumla ya nodi za AST zilizotengwa
///
/// # Hofu
///
/// Hofu ikiwa `ast_idadi == 0` (AST tupu).
pub fn lower(
    ast_aina: &[u32],
    ast_kushoto: &[i32],
    ast_kulia: &[i32],
    ast_tiga: &[i32],
    ast_nne: &[i32],
    ast_thamani: &[i32],
    ast_jina_off: &[i32],
    ast_pool: &[u8],
    ast_idadi: usize,
) -> Module {
    assert!(ast_idadi > 0, "lower: empty AST (ast_idadi == 0)");

    let mut lr = Lowerer {
        ast_aina,
        ast_kushoto,
        ast_kulia,
        ast_tiga,
        ast_nne,
        ast_thamani,
        ast_jina_off,
        ast_pool,
        functions: Vec::new(),
        globals: Vec::new(),
        types: Vec::new(),
        strings: Vec::new(),
        func: Function::new("", IrType::Void, vec![]),
        scopes: Vec::new(),
        loops: Vec::new(),
        inst_counter: 0,
        values_initial_len: 0,
        block_counter: 0,
        pre_allocated_locals: std::collections::HashMap::new(),
        global_types: std::collections::HashMap::new(),
        sret_dest: None,
    };

    // Nodi ya mizizi ndiyo iliyotengwa mwisho; lazima iwe AST_PROGRAMU.
    let root = (ast_idadi - 1) as i32;
    let root_kind = lr.node_aina(root);
    assert_eq!(
        root_kind, AST_PROGRAMU,
        "lower: root node is not PROGRAMU (got {})",
        root_kind
    );

    // Kupita-awali: sajili aina zote za muundo kwanza, ili aina za vigezo vya kazi
    // ziweze kutatua marejeo ya muundo kupitia self.types.
    let mut child = lr.ast_kushoto[root as usize];
    while child != NO_NODE {
        if lr.node_aina(child) == AST_MUUNDO {
            lr.lower_muundo(child);
        }
        child = lr.ast_nne[child as usize];
    }

    // Kupita-awali 2: kusanya majina ya kazi zenye miili.  Matangazo
    // ya mbele (bila mwili) yenye jina linaloonekana katika seti hili ni ya ziada
    // na lazima yarukwe wakati wa kuteremsha, vinginevyo huunda
    // vishika nafasi tupu vinavyoficha utekelezaji halisi.
    let mut has_body: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut child = lr.ast_kushoto[root as usize];
    while child != NO_NODE {
        if lr.node_aina(child) == AST_KAZI {
            let body_node = lr.ast_tiga[child as usize];
            if body_node != NO_NODE {
                let name_node = lr.ast_kushoto[child as usize];
                if name_node != NO_NODE {
                    let name = lr.read_pool_name(lr.ast_jina_off[name_node as usize]);
                    has_body.insert(name);
                }
            }
        }
        child = lr.ast_nne[child as usize];
    }

    // Kupita-kuu: teremsha kazi na vigezo vya ulimwengu (miundo tayari imefanywa).
    let mut child = lr.ast_kushoto[root as usize];
    while child != NO_NODE {
        let kind = lr.node_aina(child);
        match kind {
            AST_KAZI => lr.lower_function(child, &has_body),
            AST_TANGAZO_ULIMWENGU => lr.lower_global(child),
            AST_MUUNDO => {} // already done in pre-pass
            other => {
                let _ = other;
            }
        }
        child = lr.ast_nne[child as usize];
    }

    // Jenga moduli.
    let strings: Vec<IrGlobal> = lr
        .strings
        .iter()
        .enumerate()
        .map(|(i, (_label, bytes))| {
            let name = format!("@str.{}", i);
            // Ongeza kimalizio cha ncha-tupu ikiwa hakipo tayari.
            let mut data = bytes.clone();
            if data.last() != Some(&0) {
                data.push(0);
            }
            let data_len = data.len();
            IrGlobal {
                name,
                bytes: data,
                is_const: true,
                ty: IrType::Array { element: Box::new(IrType::I8), count: data_len as u64 },
            }
        })
        .collect();

    let mut all_globals: Vec<IrGlobal> = Vec::new();
    all_globals.extend(strings);
    all_globals.extend(lr.globals);

    Module {
        name: String::new(),
        types: lr.types,
        globals: all_globals,
        functions: lr.functions,
    }
}

// ============================================================================
// Visaidizi vya Kiteremshi — ufikiaji wa dimbwi
// ============================================================================

impl<'a> Lowerer<'a> {
    /// Soma aina ya nodi kwa `node_idx`, ikirudisha 0 kwa `NO_NODE`.
    #[inline]
    fn node_aina(&self, idx: i32) -> u32 {
        if idx == NO_NODE || idx < 0 {
            return 0;
        }
        self.ast_aina[idx as usize]
    }

    /// Soma jina la UTF-8 lenye ncha-tupu kutoka dimbwi la mifuatano kwenye `offset`.
    fn read_pool_name(&self, offset: i32) -> String {
        if offset < 0 || offset as usize >= self.ast_pool.len() {
            return String::new();
        }
        let off = offset as usize;
        let mut end = off;
        while end < self.ast_pool.len() && self.ast_pool[end] != 0 {
            end += 1;
        }
        std::str::from_utf8(&self.ast_pool[off..end])
            .unwrap_or("")
            .to_string()
    }

    /// Soma mfuatano wa baiti wenye urefu-kiambishi kutoka dimbwi la mifuatano kwenye `offset`.
    ///
    /// Muundo: urefu wa `u32` wa LE wa baiti 4 ukifuatiwa na baiti ghafi
    /// nyingi kama hiyo (hakuna kimalizio).  Inarudi kwenye ncha-tupu
    /// ikiwa urefu hauonekani kuwa sahihi.
    fn read_pool_bytes(&self, offset: i32) -> Vec<u8> {
        if offset < 0 {
            return Vec::new();
        }
        let off = offset as usize;
        if off + 4 > self.ast_pool.len() {
            return Vec::new();
        }
        let len = u32::from_le_bytes([
            self.ast_pool[off],
            self.ast_pool[off + 1],
            self.ast_pool[off + 2],
            self.ast_pool[off + 3],
        ]) as usize;
        let data_start = off + 4;
        if len > 0 && data_start + len <= self.ast_pool.len() {
            self.ast_pool[data_start..data_start + len].to_vec()
        } else {
            // Rudia la urithi: chukua kama ncha-tupu.
            let mut end = off;
            while end < self.ast_pool.len() && self.ast_pool[end] != 0 {
                end += 1;
            }
            self.ast_pool[off..end].to_vec()
        }
    }

    /// Soma jina la aina kutoka dimbwi kwenye kukabilisha kilichohifadhiwa katika `ast_thamani[idx]`.
    fn read_type_from_thamani(&self, idx: i32) -> IrType {
        // ast_thamani huhifadhi nambari kamili ya aina iliyosimbwa, si kukabilisha dimbwi.
        // Usimbaji: ((familia & 255) << 8) | (upana & 255), na biti 0 = bendera ya kielekezi.
        if idx == NO_NODE || idx < 0 {
            return IrType::Void;
        }
        let enc_raw = self.ast_thamani[idx as usize];
        // Thamani hasi: jina la muundo wa mtumiaji limehifadhiwa kama kukabilisha dimbwi.
        // -(offset) = muundo kwa thamani; -(offset | 1) = kielekezi cha muundo.
        if enc_raw < 0 {
            let neg = (-enc_raw) as u32;
            let mshale = neg & 1;
            let off = (neg >> 1) as usize;
            let name = self.read_pool_name(off as i32);
            // Jaribu kupata muundo katika aina zilizosajiliwa, vinginevyo unda kishika nafasi.
            let struct_ty = self.types.iter()
                .find(|(n, _)| n == &name)
                .map(|(_, t)| t.clone())
                .unwrap_or_else(|| IrType::Struct { name, fields: vec![] });
            if mshale != 0 { return IrType::Ptr(Box::new(struct_ty)); }
            else { return struct_ty; }
        }
        let enc = enc_raw as u32;
        if enc == 0 {
            return IrType::Void;
        }
        // Usimbaji kutoka mchanganuzi wa Rust: ((familia & 255) << 11) | (upana_idx << 3) | (mshale & 7)
        let familia = (enc >> 11) & 255;
        let upana_idx = (enc >> 3) & 7;
        let mshale = enc & 7;
        let upana = match upana_idx { 0=>0, 1=>1, 2=>8, 3=>16, 4=>32, 5=>64, 6=>128, _=>32 };
        let base = match familia {
            1 => match upana { 8 => IrType::I8, 16 => IrType::I16, 32 => IrType::I32, 64 => IrType::I64, 128 => IrType::I128, _ => IrType::I32 },
            2 => match upana { 8 => IrType::A8, 16 => IrType::A16, 32 => IrType::A32, 64 => IrType::A64, 128 => IrType::A128, _ => IrType::A32 },
            3 => match upana { 16 => IrType::F16, 32 => IrType::F32, 64 => IrType::F64, 80 => IrType::F64, 128 => IrType::F64, _ => IrType::F64 },
            4 => match upana { 1 => IrType::B1, 8 => IrType::B8, 16 => IrType::B16, 32 => IrType::B32, 64 => IrType::B64, _ => IrType::B1 },
            5 => match upana { 0 => IrType::Void, 8 => IrType::W8, 16 => IrType::W16, 32 => IrType::W32, 64 => IrType::W64, _ => IrType::Void },
            6 => {
                IrType::Struct { name: format!("struct_{}", enc), fields: vec![] }
            }
            _ => IrType::I32,
        };
        let mut ty = base;
        for _ in 0..mshale {
            ty = IrType::Ptr(Box::new(ty));
        }
        ty
    }

    /// Soma aina kutoka sehemu ya `ast_thamani` ya nodi (nodi YENYEWE NI
    /// kibainishi cha aina, si mzazi anayekirejelea).
    fn read_type_from_node(&self, type_node: i32) -> IrType {
        if type_node == NO_NODE || type_node < 0 {
            return IrType::Void;
        }
        let kind = self.node_aina(type_node);
        match kind {
            // Aina ya kielekezi: *T
            28 /* NYOTA */ => {
                let inner = self.ast_kushoto[type_node as usize];
                let inner_ty = self.read_type_from_node(inner);
                IrType::Ptr(Box::new(inner_ty))
            }
            // Rejeleo la aina yenye jina kupitia thamani
            _ => {
                let name_off = self.ast_thamani[type_node as usize];
                let name = self.read_pool_name(name_off);
                if name.is_empty() {
                    IrType::Void
                } else {
                    IrType::from_swa_type(&name).unwrap_or_else(|| {
                        // Tafuta ufafanuzi wa muundo katika aina zilizosajiliwa tayari.
                        self.types.iter()
                            .find(|(n, _)| n == &name)
                            .map(|(_, t)| t.clone())
                            .unwrap_or_else(|| IrType::Struct {
                                name,
                                fields: Vec::new(),
                            })
                    })
                }
            }
        }
    }
}

// ============================================================================
// Visaidizi vya Kiteremshi — usimamizi wa kizuizi / thamani
// ============================================================================

impl<'a> Lowerer<'a> {
    /// Unda kizuizi kipya chenye kiambishi cha lebo (mf. `"entry"`,
    /// `"then"`, `"loop_header"`) kilichounganishwa na kiheshi cha kizuizi
    /// kwa upekee, kikisukuma ndani ya kazi ya sasa, na kurudisha
    /// `BlockId` yake.
    fn new_block(&mut self, label_prefix: &str) -> BlockId {
        let label = format!("{}.{}", label_prefix, self.block_counter);
        self.block_counter += 1;
        // Tumia RetVoid kama chaguo-msingi — mpigaji lazima aandike upya kwa set_terminator.
        // Br kwa BlockId(0) huunda watangulizi wa uwongo kwa kizuizi cha kuingia.
        let block = IrBlock::new(label, Terminator::RetVoid);
        let id = self.func.push_block(block);
        id
    }

    /// Ongeza amri kwenye kizuizi `block_id` na urudisha `ValueId` mpya.
    /// ValueIds kwa amri huanza kwa params.len() + values.len() (yaani baada ya
    /// Toa amri na urudisha ValueId yake.
    /// Mpango wa ValueId unalingana na codegen: params(N) + values(M) + instruction_position.
    /// instruction_position huanza kwa 0 kwa amri ya kwanza katika kila kizuizi.
    fn emit(&mut self, block_id: BlockId, inst: Instruction) -> ValueId {
        let block = &mut self.func.blocks[block_id.0];
        let vid = ValueId(self.func.params.len() + self.func.values.len() + self.inst_counter);
        block.push(inst);
        self.inst_counter += 1;
        vid
    }

    /// Tafuta thabiti iliyofanywa ndani awali na urudisha `ValueId` yake.
    /// Hofu ikiwa thabiti haikufanywa ndani awali — thabiti zote lazima
    /// zifanywe ndani kupitia `collect_constants` au orodha ya pre-intern kabla ya kuteremsha.
    fn const_val(&mut self, c: Const) -> ValueId {
        let idx = self.func.values.iter().position(|v| *v == c)
            .unwrap_or_else(|| panic!("const_val: {:?} not pre-interned", c));
        ValueId(self.func.params.len() + idx)
    }

    /// Weka kimalizio cha kizuizi kilichotolewa.
    fn set_terminator(&mut self, block_id: BlockId, term: Terminator) {
        self.func.blocks[block_id.0].terminator = term;
    }

    /// Tafuta jina katika msururu wa upeo (ndani kabisa kwanza).  Inarudisha `None` ikiwa
    /// jina halipatikani.
    fn lookup(&self, name: &str) -> Option<&AllocInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info);
            }
        }
        None
    }

    /// Sukuma upeo mpya, tupu kwenye msururu wa upeo.
    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Ondoa upeo wa ndani kabisa.
    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Sajili kigezo katika upeo wa ndani kabisa.
    fn define_var(&mut self, name: String, ptr: ValueId, ty: IrType) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, AllocInfo { ptr, ty });
        }
    }
}

// ============================================================================
// Kuteremsha kiwango-juu: kazi na vigezo vya ulimwengu
// ============================================================================

impl<'a> Lowerer<'a> {
    /// Teremsha ufafanuzi wa kazi (`AST_KAZI`, nodi 2).
    ///
    /// Mpangilio:
    /// * `ast_jina_off[node]`  → jina la kazi
    /// * `ast_thamani[node]`   → kukabilisha dimbwi la aina-rudisha
    /// * `ast_kulia[node]`     → nodi ya kigezo cha kwanza (iliyounganishwa kupitia `ast_nne`)
    /// * `ast_tiga[node]`      → mwili wa kazi (kizuizi au usemi)
    fn lower_function(&mut self, func_node: i32, has_body: &std::collections::HashSet<String>) {
        // Ruka matangazo ya mbele ambayo jina lake lina ufafanuzi sambamba
        // wenye mwili mahali pengine katika kitengo kimoja cha ukusanyaji.  Kuyateremsha
        // kunaweza kuunda kishika nafasi tupu kinachoficha utekelezaji halisi.
        let body_node = self.ast_tiga[func_node as usize];
        if body_node == NO_NODE {
            let name_node = self.ast_kushoto[func_node as usize];
            if name_node != NO_NODE {
                let name = self.read_pool_name(self.ast_jina_off[name_node as usize]);
                if has_body.contains(&name) {
                    return; // tangazo la mbele — ufafanuzi halisi upo
                }
            }
        }

        // Jina la kazi limehifadhiwa kwenye name_node (ast_kushoto),
        // si kwenye nodi ya AST_KAZI yenyewe.
        let name_node = self.ast_kushoto[func_node as usize];
        let name = if name_node != NO_NODE {
            self.read_pool_name(self.ast_jina_off[name_node as usize])
        } else {
            String::new()
        };
        let ret_ty = self.read_type_from_thamani(func_node);

        // -- Kukusanya vigezo ---------------------------------------------------
        let mut params: Vec<(String, IrType)> = Vec::new();
        let mut param_node = self.ast_kulia[func_node as usize];
        while param_node != NO_NODE {
            let pname = self.read_pool_name(self.ast_jina_off[param_node as usize]);
            // Kwa nodi za vigezo, ast_thamani inaweza kuwa faharasa ya nodi ya aina au
            // kukabilisha dimbwi.  Jaribu kusoma kupitia thamani kwanza; ikiwa hiyo inatoa Void
            // na kuna mtoto wa aina wa kushoto, soma kutoka hapo.
            let mut pty = self.read_type_from_thamani(param_node);
            if pty == IrType::Void {
                let type_child = self.ast_kushoto[param_node as usize];
                if type_child != NO_NODE {
                    pty = self.read_type_from_node(type_child);
                }
            }
            params.push((pname, pty));
            param_node = self.ast_kulia[param_node as usize];
        }

        // -- Jenga kazi ---------------------------------------------------------
        self.func = Function::new(name.clone(), ret_ty.clone(), params.clone());

        // Rekodi darasa la rudisha.
        let rc = crate::abi::classify_return(&ret_ty);
        self.func.return_class = rc;
        self.func.source_return_ty = ret_ty.clone();

        // Ikiwa sret, rekebisha aina ya rudisha na ongeza kigezo cha kielekezi kilichofichwa.
        let sret_ptr_vid = if rc == IrReturnClass::HiddenPtr && ret_ty != IrType::Void {
            let sret_ty = IrType::Ptr(Box::new(ret_ty.clone()));
            self.func.params.insert(0, ("_sret".to_string(), sret_ty.clone()));
            self.func.return_ty = IrType::Void;
            self.func.sret_value_id = Some(ValueId(0));
            true
        } else {
            false
        };

        // Fanya thabiti ndani kabla BAADA ya kigezo cha sret kuongezwa, ili params.len() iwe ya mwisho.
        self.collect_constants(self.ast_tiga[func_node as usize]);

        // Fanya thabiti zinazotumiwa kawaida ndani ili const_val() isiongeze kamwe
        // thamani mpya wakati wa kuteremsha (ambazo zingehamaisha ValueIds
        // kutoka kwa mpango wa N+M+I wa backend).
        self.func.intern_const(Const::Zero);
        self.func.intern_const(Const::Int(0));
        self.func.intern_const(Const::Int(1));
        self.func.intern_const(Const::Int(-1));
        self.func.intern_const(Const::Bool(false));
        self.func.intern_const(Const::Bool(true));
        self.func.intern_const(Const::NullPtr);

        self.values_initial_len = self.func.values.len();
        self.inst_counter = 0;

        // -- Unda kizuizi cha kuingia -------------------------------------------
        self.scopes.clear();
        self.loops.clear();
        self.push_scope();

        let entry_id = self.new_block("entry");
        self.func.entry = entry_id;

        // -- Teremsha vigezo kwenye sehemu za mrundikano -----------------------
        // ValueIds za vigezo ni 0..N-1 kwa mpangilio sawa na self.func.params.
        // Ikiwa sret inatumika, kielekezi kilichofichwa ni ValueId(0) na vigezo
        // vinavyoonekana kwa mtumiaji huanza kwa ValueId(1).  Faharasa ya mzunguko
        // `i` tayari inashughulikia hili — tunaruka i=0 wakati sret inatumika.

        // Nakili params ili kuepuka mgongano wa kukopa na self.emit chini.
        let params: Vec<_> = self.func.params.iter().cloned().collect();
        for (i, (pname, pty)) in params.iter().enumerate() {
            // Ruka kielekezi cha sret — kinateremshwa tofauti.
            if sret_ptr_vid && i == 0 {
                continue;
            }
            let alloc = self.emit(entry_id, Instruction::Alloca(pty.clone()));
            // Thamani ya kigezo: ValueId(i). (Ikiwa sret, kigezo i=1 kinapata ValueId(1),
            // ambacho kinaruka ValueId(0) kwa usahihi.)
            let param_vid = ValueId(i);
            self.emit(entry_id, Instruction::StoreTyped(param_vid, alloc, pty.clone()));
            self.define_var(pname.clone(), alloc, pty.clone());
        }

        // -- Kupita-awali: inua allocas zote za vigezo-ndani kwenye kizuizi cha kuingia ------
        // Tembea AST ya mwili wa kazi na kusanya kila nodi ya AST_TANGAZO.  Toa
        // Alloca ndani ya kizuizi cha kuingia kwa kila moja na rekodi ramani ya
        // (nodi → ValueId) ili lower_local_decl iweze kutumia tena sehemu
        // iliyotengwa awali badala ya kuunda alloca mpya katika kizuizi cha sasa.
        let mut local_decls: Vec<(i32, IrType)> = Vec::new();
        self.collect_local_decls(body_node, &mut local_decls);
        for (node, var_ty) in &local_decls {
            // Ruka vigezo vya muundo wakati sret inatumika — lower_local_decl
            // itatumia tena kielekezi cha sret moja kwa moja badala ya sehemu ya alloca.
            let is_sret_struct = matches!(&var_ty, IrType::Struct { .. })
                && self.func.sret_value_id.is_some();
            if !is_sret_struct {
                let alloc = self.emit(entry_id, Instruction::Alloca(var_ty.clone()));
                self.pre_allocated_locals.insert(*node, alloc);
            }
        }

        // -- Teremsha mwili -----------------------------------------------------
        let body_node = self.ast_tiga[func_node as usize];
        let body_block_id = self.lower_block(body_node);

        // Unganisha kuingia → mwili.
        self.set_terminator(entry_id, Terminator::Br(body_block_id));

        // -- Maliza -------------------------------------------------------------
        // Badilisha kimalizio cha kujizungusha na RetVoid cha kishika nafasi
        // na rudisha zinazofaa aina ya rudisha ya kazi.
        let is_void = matches!(self.func.return_ty, IrType::Void);
        let needs_fixup = |blk_id: BlockId, term: &Terminator| -> bool {
            matches!(term, Terminator::Br(b) if *b == blk_id)
                || matches!(term, Terminator::RetVoid)
        };
        let block_count = self.func.blocks.len();
        for i in 0..block_count {
            let blk_id = BlockId(i);
            let term = &self.func.blocks[i].terminator;
            if needs_fixup(blk_id, term) {
                if is_void {
                    self.set_terminator(blk_id, Terminator::RetVoid);
                } else {
                    let zero = self.const_val(Const::Int(0));
                    self.set_terminator(blk_id, Terminator::Ret(zero));
                }
            }
        }

        self.pop_scope();
        self.functions.push(std::mem::replace(
            &mut self.func,
            Function::new("", IrType::Void, vec![]),
        ));
    }

    /// Teremsha kigezo cha ulimwengu (`AST_TANGAZO_ULIMWENGU`, nodi 35).
    ///
    /// Mpangilio wa mchanganuzi:
    /// * `ast_kushoto[node]` → nodi ya kitambulisho cha jina (jina_off imewekwa hapo)
    /// * `ast_thamani[node]` → aina ya rudisha iliyosimbwa (nambari kamili)
    /// * `ast_kulia[node]`   → usemi wa kianzisha (ikiwa upo)
    fn lower_global(&mut self, glob_node: i32) {
        // Jina: mchanganuzi huhifadha kupitia name_node katika kushoto; rudia kwa jina_off moja kwa moja.
        let name_node = self.ast_kushoto[glob_node as usize];
        let name = if name_node != NO_NODE {
            self.read_pool_name(self.ast_jina_off[name_node as usize])
        } else {
            self.read_pool_name(self.ast_jina_off[glob_node as usize])
        };
        let base_ty = self.read_type_from_thamani(glob_node);

        // Angalia ukubwa wa safu uliohifadhiwa katika tiga (iliyowekwa na mchanganuzi kwa Aina jina[ukubwa]).
        let saizi_node = self.ast_tiga[glob_node as usize];
        let ty = if saizi_node != NO_NODE && self.node_aina(saizi_node) == AST_NAMBARI {
            let count = self.ast_thamani[saizi_node as usize] as u32;
            IrType::Array { element: Box::new(base_ty), count: count as u64 }
        } else {
            base_ty
        };

        if !name.is_empty() {
            self.global_types.insert(name.clone(), ty.clone());
        }

        // Kianzisha: mchanganuzi huhifadhi katika kulia (si tiga — tiga ni ukubwa wa safu hapo juu).
        let init_node = self.ast_kulia[glob_node as usize];

        // Tathmini vianzisha thabiti (halisi za nambari kamili pekee kwa sasa).
        let size = ty.width_bytes();
        let bytes = if init_node != NO_NODE && init_node >= 0
            && self.node_aina(init_node) == AST_NAMBARI {
            let val = self.ast_thamani[init_node as usize] as i128;
            let mut b = vec![0u8; size];
            for i in 0..size.min(8) {
                b[i] = ((val >> (i * 8)) & 0xFF) as u8;
            }
            b
        } else {
            vec![0u8; size]
        };

        self.globals.push(IrGlobal {
            name,
            bytes,
            is_const: false,
            ty,
        });
    }

    /// Sajili ufafanuzi wa muundo katika jedwali la aina za moduli.
    fn lower_muundo(&mut self, muundo_node: i32) {
        let name_node = self.ast_kushoto[muundo_node as usize];
        if name_node == NO_NODE { return; }
        let name = self.read_pool_name(self.ast_jina_off[name_node as usize]);
        if name.is_empty() { return; }

        let mut fields: Vec<(String, IrType)> = Vec::new();
        let mut field_node = self.ast_kulia[muundo_node as usize];
        while field_node != NO_NODE {
            if self.node_aina(field_node) == AST_SEHEMU {
                let fname_node = self.ast_kushoto[field_node as usize];
                let fname = self.read_pool_name(self.ast_jina_off[fname_node as usize]);
                let fty = self.read_type_from_thamani(fname_node);
                fields.push((fname, fty));
            }
            field_node = self.ast_kulia[field_node as usize];
        }
        self.types.push((name.clone(), IrType::Struct { name, fields }));
    }
}

// ============================================================================
// Kuteremsha taarifa
// ============================================================================

impl<'a> Lowerer<'a> {
    /// Tembea AST na fanya thabiti zote za nambari kamili ndani kabla ili
    /// `func.values.len()` iwe thabiti kabla ya ugawaji wa ValueId wa amri.
    fn collect_constants(&mut self, node: i32) {
        if node == NO_NODE || node < 0 { return; }
        let idx = node as usize;
        if idx >= self.ast_aina.len() { return; }
        let kind = self.ast_aina[idx];
        match kind {
            AST_NAMBARI => {
                let val = self.ast_thamani[idx] as i128;
                self.func.intern_const(Const::Int(val));
                // Pia tembelea nne: misururu ya hoja za wito hutumia nne, kwa hivyo halisi
                // inaweza kufuatwa na hoja nyingine (mf. fread(..., 1, 262144)).
                self.collect_constants(self.ast_nne[idx]);
                return;
            }
            AST_KWELI => {
                self.func.intern_const(Const::Bool(true));
                return;
            }
            AST_UONGO => {
                self.func.intern_const(Const::Bool(false));
                return;
            }
            AST_TUPU => {
                self.func.intern_const(Const::NullPtr);
                return;
            }
            _ => {}
        }
        self.collect_constants(self.ast_kushoto[idx]);
        self.collect_constants(self.ast_kulia[idx]);
        self.collect_constants(self.ast_tiga[idx]);
        self.collect_constants(self.ast_nne[idx]);
        // AST_KIPINDI huhifadhi usemi wake wa sharti katika ast_thamani
        // (ast_nne imetengwa kwa msururu wa taarifa ndugu).
        if kind == AST_KIPINDI {
            self.collect_constants(self.ast_thamani[idx]);
        }
    }

    /// Tembea AST kwa kurudia na kusanya nodi zote za `AST_TANGAZO` (tangazo
    /// la kigezo cha ndani) pamoja na aina zao zilizotatuliwa.
    ///
    /// Hii inatumika wakati wa kupita-awali katika `kazi_ya_kuteremsha` ili
    /// kila amri ya `Alloca` ya ndani iweze kutolewa ndani ya kizuizi cha
    /// kuingia badala ya kizuizi cha sasa, kuzuia uchovu wa mrundikano wa alloca-katika-mzunguko.
    fn collect_local_decls(&self, node: i32, decls: &mut Vec<(i32, IrType)>) {
        if node == NO_NODE || node < 0 { return; }
        let idx = node as usize;
        if idx >= self.ast_aina.len() { return; }
        let kind = self.ast_aina[idx];
        if kind == AST_TANGAZO {
            // Tatua aina kwa kutumia mantiki sawa na lower_local_decl.
            let var_ty = if self.ast_thamani[idx] != 0 {
                // Muundo wa mchanganuzi: aina imesimbwa katika thamani.
                self.read_type_from_thamani(node)
            } else {
                // Muundo wa jaribio/urithi: nodi ya aina katika kulia, kianzisha katika tiga.
                let type_node = self.ast_kulia[idx];
                if type_node != NO_NODE && type_node >= 0 {
                    self.read_type_from_node(type_node)
                } else {
                    IrType::I32
                }
            };
            decls.push((node, var_ty));
            // USIRUDISHE mapema — tangazo la ndani linaweza kuwa na usemi wa kianzisha
            // ambao wenyewe una misemo iliyopachikwa.  Endelea kurudia.
        }
        self.collect_local_decls(self.ast_kushoto[idx], decls);
        self.collect_local_decls(self.ast_kulia[idx], decls);
        self.collect_local_decls(self.ast_tiga[idx], decls);
        self.collect_local_decls(self.ast_nne[idx], decls);
        // AST_KIPINDI huhifadhi usemi wake wa sharti katika ast_thamani
        // (ast_nne imetengwa kwa msururu wa taarifa ndugu).
        if kind == AST_KIPINDI {
            self.collect_local_decls(self.ast_thamani[idx], decls);
        }
    }

    /// Teremsha nodi ya taarifa (au usemi unaotumika kama taarifa).
    ///
    /// Inarudisha `BlockId` ya kizuizi cha *muendelezo* — kizuizi ambacho
    /// mtiririko-dhibiti unapita baada ya taarifa hii kukamilika kwa kawaida.
    fn lower_stmt(&mut self, node: i32) -> BlockId {
        if node == NO_NODE || node < 0 {
            // Taarifa tupu → unda kizuizi rahisi cha kupita.
            let blk = self.new_block("empty");
            self.set_terminator(blk, Terminator::Br(blk));
            return blk;
        }

        let kind = self.node_aina(node);
        match kind {
            // ---- mchanganyiko / usemi-kama-taarifa ----------------------------
            AST_ASIMILIA => self.lower_assign(node),
            AST_KAMA => self.lower_if(node),
            AST_WAKATI => self.lower_while(node),
            AST_RUDISHA => self.lower_return(node),
            AST_TANGAZO => self.lower_local_decl(node),
            AST_CHAGUA => self.lower_switch(node),
            AST_VUNJA => self.lower_break(node),
            AST_KIPINDI => self.lower_for(node),
            AST_ENDELEA => self.lower_continue(node),
            AST_TENGA => self.lower_heap_alloc_stmt(node),
            AST_ACHILIA => self.lower_heap_free_stmt(node),
            AST_WITO => {
                // Taarifa ya usemi (matokeo ya wito yametupwa).
                let blk = self.new_block("call_stmt");
                let (_val, end_blk) = self.lower_expr_into(node, blk);
                // Weka kimalizio cha kishika nafasi ili lower_block kiweze kukiunganisha.
                self.set_terminator(end_blk, Terminator::Br(end_blk));
                end_blk
            }
            // ---- usemi kama taarifa -------------------------------------------
            _ => {
                // Nodi yoyote ya usemi inayotumika kama taarifa: tathmini na tupe.
                let blk = self.new_block("expr_stmt");
                let (_val, end_blk) = self.lower_expr_into(node, blk);
                // Weka kimalizio cha kishika nafasi ili lower_block kiweze kukiunganisha
                // (waendeshaji fupi-hali huweka vimalizio vyao wenyewe; usiandike juu).
                self.patch_br_if_needed(end_blk, end_blk);
                end_blk
            }
        }
    }

    /// Teremsha kizuizi (msururu wa taarifa zilizounganishwa kupitia `ast_nne`).
    ///
    /// Inarudisha `BlockId` ya kizuizi cha kuingia kwa mfuatano huu.
    fn lower_block(&mut self, first_stmt: i32) -> BlockId {
        if first_stmt == NO_NODE || first_stmt < 0 {
            let blk = self.new_block("empty_body");
            // Tumia kishika nafasi cha kujizungusha; mpigaji au kimalizia kitarekebisha.
            self.set_terminator(blk, Terminator::Br(blk));
            return blk;
        }

        // Tembea msururu wa taarifa, ukiunganisha kila taarifa kwa inayofuata.
        let mut current = first_stmt;
        let entry_id = self.new_block("body");
        let mut prev_block = entry_id;
        let mut is_first = true;

        while current != NO_NODE && current >= 0 {
            let next_stmt = self.ast_nne[current as usize];
            let stmt_blk = self.lower_stmt(current);

            // Ikiwa stmt_blk ni kizuizi cha sharti (BrCond), kipitio halisi
            // ni kizuizi cha muunganiko.  Tembea tawi la uongo; ikiwa linaisha kwa Ret,
            // tembea tawi la kweli badala yake.  Shughulikia BrConds zilizopachikwa kwa kurudia.
            let actual_prev = match &self.func.blocks[stmt_blk.0].terminator {
                Terminator::BrCond(_, true_blk, false_blk) => {
                    let mut seen: Vec<BlockId> = Vec::new();
                    fn tembea_tawi(
                        blocks: &[IrBlock],
                        start: BlockId,
                        seen: &mut Vec<BlockId>,
                    ) -> (BlockId, bool) {
                        if seen.contains(&start) {
                            return (start, false);
                        }
                        seen.push(start);
                        match &blocks[start.0].terminator {
                            Terminator::Br(t) if *t != start => {
                                tembea_tawi(blocks, *t, seen)
                            }
                            Terminator::BrCond(_, t, f) => {
                                let (fb, f_ret) = tembea_tawi(blocks, *f, seen);
                                if f_ret {
                                    tembea_tawi(blocks, *t, seen)
                                } else {
                                    (fb, false)
                                }
                            }
                            Terminator::Ret(_) | Terminator::RetVoid => {
                                (start, true)
                            }
                            _ => (start, false),
                        }
                    }
                    let (result, is_ret) = tembea_tawi(&self.func.blocks, *false_blk, &mut seen);
                    if is_ret {
                        tembea_tawi(&self.func.blocks, *true_blk, &mut seen).0
                    } else {
                        result
                    }
                }
                _ => stmt_blk,
            };

            if is_first {
                self.set_terminator(prev_block, Terminator::Br(stmt_blk));
                prev_block = actual_prev;
                is_first = false;
            } else {
                let prev_term = &self.func.blocks[prev_block.0].terminator;
                let inahitaji_mnyororo = matches!(prev_term, Terminator::RetVoid)
                    || matches!(prev_term, Terminator::Br(b) if *b == prev_block);
                if inahitaji_mnyororo {
                    self.set_terminator(prev_block, Terminator::Br(stmt_blk));
                }
                prev_block = actual_prev;
            }

            current = next_stmt;
        }

        // Ikiwa taarifa ya mwisho haikuweka kimalizio halisi, ongeza kishika nafasi
        // cha kujizungusha.  Mpigaji (au kupita kwa baadaye) anawajibika
        // kubadilisha hili kwa Ret/RetVoid inayofaa kazi.
        let last_block = prev_block;
        let inahitaji_kimalizio = match &self.func.blocks[last_block.0].terminator {
            Terminator::Br(b) if *b == last_block => true, // kishika nafasi
            // RetVoid ni kimalizio halali (kutoka chaguo-msingi cha new_block au
            // kutoka rudisha wazi).  USIBADILISHE na kujizungusha —
            // hiyo ingezuia rudisha kurudi.
            _ => false,
        };
        if inahitaji_kimalizio {
            self.set_terminator(last_block, Terminator::Br(last_block));
        }

        entry_id
    }

}

// ============================================================================
// Kuteremsha taarifa — aina za taarifa binafsi
// ============================================================================

impl<'a> Lowerer<'a> {
    /// Teremsha `ASIMILIA` (ugawaji): `lengo = thamani`.
    ///
    /// Mpangilio:
    /// * `ast_kushoto[node]` → thamani-l (lvalue)
    /// * `ast_kulia[node]`   → usemi wa thamani-r (rvalue)
    fn lower_assign(&mut self, node: i32) -> BlockId {
        let lhs_node = self.ast_kushoto[node as usize];
        let rhs_node = self.ast_kulia[node as usize];

        let blk = self.new_block("assign");

        // Kwanza hesabu kielekezi cha upande wa kushoto (LHS).
        let ptr = self.lower_lvalue(lhs_node, blk);

        // Ikiwa LHS ni aina ya muundo, toa kama sret_dest ili wito
        // unaorudisha muundo kwenye RHS uandike moja kwa moja kwenye LHS.
        let lhs_ty = self.resolve_expr_type(lhs_node);
        let is_struct_assign = matches!(&lhs_ty, Some(IrType::Struct { .. }));
        if is_struct_assign {
            self.sret_dest = Some(ptr);
        }

        let (rhs_val, end_blk) = self.lower_expr_into(rhs_node, blk);

        // Ikiwa sret_dest ililiwa na lower_call, muundo uliandikwa moja
        // kwa moja kwenye ptr na hakuna Store inayohitajika.
        let sret_consumed = is_struct_assign && self.sret_dest.is_none();
        if !sret_consumed {
            let store_ty = lhs_ty.unwrap_or(IrType::I32);
            if is_struct_assign {
                // Ugawaji wa muundo: rhs_val ni kielekezi cha alloca cha RHS
                // (lower_identifier inarudisha info.ptr kwa aina za muundo).
                // Tumia MemCopy kunakili baiti za muundo.
                let struct_size = store_ty.width_bytes() as u64;
                if struct_size > 0 {
                    self.emit(end_blk, Instruction::MemCopy(ptr, rhs_val, struct_size));
                }
            } else {
                self.emit(end_blk, Instruction::StoreTyped(rhs_val, ptr, store_ty));
            }
        }
        self.sret_dest = None;
        self.set_terminator(end_blk, Terminator::Br(end_blk));
        end_blk
    }

    /// Teremsha `KAMA` (kama): `kama (sharti) tawi_la_kweli [tiga tawi_la_sivyo]`.
    ///
    /// Mpangilio:
    /// * `ast_kushoto[node]` → usemi wa sharti
    /// * `ast_kulia[node]`   → tawi la kweli
    /// * `ast_tiga[node]`    → tawi la sivyo (si lazima, -1 ikiwa halipo)
    fn lower_if(&mut self, node: i32) -> BlockId {
        let cond_node = self.ast_kushoto[node as usize];
        let then_node = self.ast_kulia[node as usize];
        let else_node = self.ast_tiga[node as usize];

        let cond_blk = self.new_block("if.cond");
        let (cond_val, cond_end) = self.lower_expr_into(cond_node, cond_blk);

        let then_blk = self.lower_block(then_node);
        let merge_blk = self.new_block("if.merge");

        // Tawi kutoka sharti.
        if else_node != NO_NODE && else_node >= 0 {
            let else_blk = self.lower_block(else_node);
            self.set_terminator(
                cond_end,
                Terminator::BrCond(cond_val, then_blk, else_blk),
            );
            self.patch_br_if_needed(then_blk, merge_blk);
            self.patch_br_if_needed(else_blk, merge_blk);
        } else {
            self.set_terminator(
                cond_end,
                Terminator::BrCond(cond_val, then_blk, merge_blk),
            );
            self.patch_br_if_needed(then_blk, merge_blk);
        }
        // Hakikisha merge_blk inaweza kurekebishwa na mpigaji wa lower_block.
        self.set_terminator(merge_blk, Terminator::Br(merge_blk));

        // Rudisha cond_blk ili lower_block ipeleke kizuizi cha kuingia kwenye
        // kizuizi cha sharti (si kizuizi cha muunganiko), na kufanya sharti
        // lifikike.  Mantiki ya actual_prev ya lower_block inafuata tawi la
        // uongo la BrCond kupata kizuizi cha muunganiko kwa kuunganisha
        // taarifa inayofuata.
        cond_blk
    }

    /// Teremsha `WAKATI` (mzunguko wa wakati): `wakati (sharti) { mwili }`.
    ///
    /// Mpangilio:
    /// * `ast_kushoto[node]` → usemi wa sharti
    /// * `ast_tiga[node]`    → mwili wa mzunguko
    fn lower_while(&mut self, node: i32) -> BlockId {
        let cond_node = self.ast_kushoto[node as usize];
        let body_node = self.ast_kulia[node as usize];

        let header_blk = self.new_block("while.header");
        let body_blk = self.new_block("while.body");
        let exit_blk = self.new_block("while.exit");
        // Weka kishika nafasi cha kujizungusha kwenye exit_blk ili mantiki
        // ya actual_prev ya lower_block itambue kwa usahihi kama njia ya
        // kipitio (badala ya RetVoid, ambayo ingechukuliwa kama rudisha).
        self.set_terminator(exit_blk, Terminator::Br(exit_blk));

        // Sukuma mazingira ya mzunguko ili `vunja` → exit, `endelea` → header.
        self.loops.push(LoopInfo {
            header: header_blk,
            exit: exit_blk,
        });

        // Kichwa: tathmini sharti, tawi kwa mwili au kutoka.
        let (cond_val, cond_end) = self.lower_expr_into(cond_node, header_blk);
        self.set_terminator(
            cond_end,
            Terminator::BrCond(cond_val, body_blk, exit_blk),
        );

        // Teremsha taarifa za mwili — inarudisha kizuizi cha kuingia cha msururu wa mwili.
        let body_entry = self.lower_block(body_node);
        // Chomeka kizuizi cha while.body kwenye kuingia kwa mwili ulioteremshwa.
        self.set_terminator(body_blk, Terminator::Br(body_entry));

        // Tafuta kizuizi cha mwisho katika msururu wa mwili (kile kinachopitia)
        // na kikiunganisha kurudi kwenye kichwa cha mzunguko.
        let mut last = body_entry;
        loop {
            let term = &self.func.blocks[last.0].terminator;
            match term {
                Terminator::Br(target) if *target != last => {
                    // Fuata msururu mbele, lakini simama kwenye vunja (→ exit_blk)
                    // au endelea (→ header_blk) ili kuepuka kutembea nje ya mwili.
                    if *target == exit_blk || *target == header_blk {
                        break;
                    }
                    last = *target;
                }
                Terminator::Br(_) => {
                    // Kishika nafasi cha kujizungusha — hiki ni kizuizi cha mwisho.
                    break;
                }
                Terminator::BrCond(_, _, merge) => {
                    // Njia ya kipitio ya sharti ni kizuizi cha muunganiko.
                    // Endelea kutembea kutoka hapo kupata kizuizi halisi cha mwisho.
                    if *merge == exit_blk || *merge == header_blk {
                        break;
                    }
                    last = *merge;
                }
                _ => {
                    // Kimalizio halisi (Ret, Switch) — simama hapa.
                    break;
                }
            }
        }
        self.ensure_br(last, header_blk);

        self.loops.pop();
        header_blk
    }

    /// Teremsha `KIPINDI` (mzunguko wa kwa): `kipindi (kianzisha; sharti; hatua) { mwili }`.
    ///
    /// Mpangilio:
    /// * `ast_kushoto[node]` → kianzisha
    /// * `ast_kulia[node]`   → mwili wa mzunguko (kulia huepuka mgongano wa ast_nne na msururu wa taarifa)
    /// * `ast_tiga[node]`    → usemi wa hatua
    /// * `ast_nne[node]`     → sharti (nne ni salama kwani usemi rahisi una ast_nne = NO_NODE)
    fn lower_for(&mut self, node: i32) -> BlockId {
        let init_node = self.ast_kushoto[node as usize];
        let body_node = self.ast_kulia[node as usize];
        let step_node = self.ast_tiga[node as usize];
        // sharti limehifadhiwa katika ast_thamani (si ast_nne!) kwa sababu ast_nne
        // imetengwa kwa msururu wa ndugu unaotumiwa na lower_block.
        let cond_node = self.ast_thamani[node as usize];

        let init_blk = if init_node != NO_NODE && init_node >= 0 {
            self.lower_stmt(init_node)
        } else {
            let blk = self.new_block("for.init");
            self.set_terminator(blk, Terminator::Br(blk));
            blk
        };

        let header_blk = self.new_block("for.header");
        let body_blk = self.new_block("for.body");
        let step_blk = self.new_block("for.step");
        let exit_blk = self.new_block("for.exit");
        // Sasisho sawa na lower_while: weka kishika nafasi cha kujizungusha ili
        // mantiki ya actual_prev ya lower_block ipate exit_blk kama kipitio.
        self.set_terminator(exit_blk, Terminator::Br(exit_blk));

        self.set_terminator(init_blk, Terminator::Br(header_blk));

        // Sukuma mazingira ya mzunguko.
        self.loops.push(LoopInfo {
            header: header_blk,
            exit: exit_blk,
        });

        // Kichwa: tathmini sharti.
        let (cond_val, cond_end) = if cond_node != NO_NODE && cond_node >= 0 {
            self.lower_expr_into(cond_node, header_blk)
        } else {
            // Hakuna sharti → mzunguko usio na sharti.
            let one = self.const_val(Const::Bool(true));
            (one, header_blk)
        };

        if cond_node != NO_NODE && cond_node >= 0 {
            self.set_terminator(
                cond_end,
                Terminator::BrCond(cond_val, body_blk, exit_blk),
            );
        } else {
            self.set_terminator(cond_end, Terminator::Br(body_blk));
        }

        // Mwili.
        let body_end = self.lower_block(body_node);
        self.set_terminator(body_blk, Terminator::Br(body_end));

        // Tembea msururu wa mwili kupata kizuizi cha mwisho na kukiunganisha kwa step_blk.
        let mut last = body_end;
        loop {
            let term = &self.func.blocks[last.0].terminator;
            match term {
                Terminator::Br(target) if *target != last => {
                    if *target == exit_blk || *target == header_blk {
                        break;
                    }
                    last = *target;
                }
                Terminator::Br(_) => {
                    break;
                }
                Terminator::BrCond(_, _, merge) => {
                    if *merge == exit_blk || *merge == header_blk {
                        break;
                    }
                    last = *merge;
                }
                _ => {
                    break;
                }
            }
        }
        self.ensure_br(last, step_blk);

        // Hatua.
        let step_end = if step_node != NO_NODE && step_node >= 0 {
            self.lower_stmt(step_node)
        } else {
            step_blk
        };
        // Chomeka kizuizi cha lebo ya hatua kwenye kuingia kwa hatua (kujizungusha ikiwa hakuna hatua).
        self.set_terminator(step_blk, Terminator::Br(step_end));
        // Tembea msururu wa hatua kupata kizuizi cha mwisho na kukiunganisha kwa header_blk.
        let mut last_step = step_end;
        loop {
            let term = &self.func.blocks[last_step.0].terminator;
            match term {
                Terminator::Br(target) if *target != last_step => {
                    last_step = *target;
                }
                Terminator::Br(_) => {
                    break;
                }
                Terminator::BrCond(_, _, merge) => {
                    last_step = *merge;
                }
                _ => {
                    break;
                }
            }
        }
        self.ensure_br(last_step, header_blk);

        self.loops.pop();
        header_blk
    }

    /// Teremsha `RUDISHA` (rudisha): `rudisha [usemi]`.
    ///
    /// Mpangilio:
    /// * `ast_kushoto[node]` → usemi wa thamani ya rudisha (si lazima, -1 kwa rudisha tupu)
    fn lower_return(&mut self, node: i32) -> BlockId {
        let val_node = self.ast_kushoto[node as usize];
        let blk = self.new_block("ret");

        if val_node != NO_NODE && val_node >= 0 {
            let (val, end_blk) = self.lower_expr_into(val_node, blk);
            // Ikiwa sret na thamani si tayari kielekezi cha sret,
            // nakili baiti za muundo kwa kielekezi cha sret kwa kutumia MemCopy.
            if let Some(sret_vid) = self.func.sret_value_id {
                if val != sret_vid {
                    let struct_size = self.func.source_return_ty.width_bytes() as u64;
                    if struct_size > 0 {
                        self.emit(end_blk, Instruction::MemCopy(sret_vid, val, struct_size));
                    }
                }
                self.set_terminator(end_blk, Terminator::RetVoid);
            } else {
                self.set_terminator(end_blk, Terminator::Ret(val));
            }
            end_blk
        } else {
            // Hakuna thamani wazi ya rudisha.
            if self.func.sret_value_id.is_some() {
                // sret bila thamani wazi — rudisha tupu.
                self.set_terminator(blk, Terminator::RetVoid);
            } else if self.func.source_return_ty == IrType::Void {
                // Kazi ya tupu — rudisha tupu.
                self.set_terminator(blk, Terminator::RetVoid);
            } else {
                // Kazi isiyo tupu bila rudisha wazi: rudisha sifuri.
                let zero = self.const_val(Const::Int(0));
                self.set_terminator(blk, Terminator::Ret(zero));
            }
            blk
        }
    }

    /// Teremsha `TANGAZO` (tangazo la kigezo cha ndani): `acha jina [: aina] [= kianzisha]`.
    ///
    /// Muundo wa mchanganuzi:
    /// * `ast_kushoto[node]` → jina la kigezo (nodi ya kitambulisho)
    /// * `ast_thamani[node]` → nambari kamili ya aina iliyosimbwa (familia, upana, mshale)
    /// * `ast_kulia[node]`   → usemi wa kianzisha (si lazima)
    ///
    /// Muundo wa jaribio/urithi (wakati ast_thamani[node]==0):
    /// * `ast_kulia[node]`   → nodi ya aina
    /// * `ast_tiga[node]`    → usemi wa kianzisha (si lazima)
    fn lower_local_decl(&mut self, node: i32) -> BlockId {
        let name_node = self.ast_kushoto[node as usize];
        let var_name = self.read_pool_name(self.ast_jina_off[name_node as usize]);

        // Tambua muundo: mchanganuzi anaweka aina iliyosimbwa katika thamani, majaribio yanaweka nodi ya aina katika kulia.
        let (var_ty, init_node) = if self.ast_thamani[node as usize] != 0 {
            // Muundo wa mchanganuzi: aina imesimbwa katika thamani, kianzisha katika kulia.
            let ty = self.read_type_from_thamani(node);
            let init = self.ast_kulia[node as usize];
            (ty, init)
        } else {
            // Muundo wa jaribio/urithi: nodi ya aina katika kulia, kianzisha katika tiga.
            let type_node = self.ast_kulia[node as usize];
            let ty = if type_node != NO_NODE && type_node >= 0 {
                self.read_type_from_node(type_node)
            } else {
                IrType::I32
            };
            let init = self.ast_tiga[node as usize];
            (ty, init)
        };

        let blk = self.new_block("decl");

        // Ikiwa hii ni muundo wa thamani-rudisha ya kazi ya sret, tumia
        // kielekezi cha sret moja kwa moja badala ya kutenga sehemu ya ndani.
        // Vinginevyo, tafuta alloca iliyotolewa awali ndani ya kizuizi cha
        // kuingia na kupita-awali ya lower_function — hii inazuia alloca-katika-mzunguko
        // (kila marudio ya mzunguko vinginevyo yangeunda alloca mpya,
        // ikimaliza mrundikano).
        let alloc = if matches!(&var_ty, IrType::Struct { .. })
            && self.func.sret_value_id.is_some()
        {
            self.func.sret_value_id.unwrap()
        } else {
            self.pre_allocated_locals[&node]
        };

        // Tathmini kianzisha na hifadhi.
        if init_node != NO_NODE && init_node >= 0 {
            // Kwa vigezo vya muundo, toa alloca kama sret_dest ili wito
            // uandike moja kwa moja kwenye lengwa (hakuna Load+Store inayohitajika).
            if matches!(&var_ty, IrType::Struct { .. }) {
                self.sret_dest = Some(alloc);
            }
            let (init_val, end_blk) = self.lower_expr_into(init_node, blk);
            // Ikiwa sret_dest ilitumika, init_val NI alloc na tunaruka store.
            if init_val != alloc {
                // Kwa muundo, init_val ni kielekezi kwenye muundo chanzo;
                // tumia MemCopy kunakili baiti badala ya StoreTyped
                // ambayo ingehifadhi thamani ya kielekezi yenyewe.
                if matches!(&var_ty, IrType::Struct { .. }) {
                    let struct_size = var_ty.width_bytes() as u64;
                    if struct_size > 0 {
                        self.emit(end_blk, Instruction::MemCopy(alloc, init_val, struct_size));
                    }
                } else {
                    self.emit(end_blk, Instruction::StoreTyped(init_val, alloc, var_ty.clone()));
                }
            }
            self.define_var(var_name, alloc, var_ty);
            self.set_terminator(end_blk, Terminator::Br(end_blk));
            end_blk
        } else {
            self.define_var(var_name, alloc, var_ty.clone());
            // Anzisha vigezo vya ndani kwa sifuri, lakini ruka kielekezi
            // cha sret chenyewe — sehemu ya sret inaweza tayari kuwa na
            // data kutoka kwa mpigaji (mf. kisaidizi kinapoandika matokeo
            // sehemu kabla ya kumwita kisaidizi kingine).  Kuzifuta sifuri
            // hapa kungeandika juu ya matokeo hayo na kusababisha thamani
            // za rudisha zilizoharibika kwa O1.
            let is_sret_slot = self.func.sret_value_id == Some(alloc);
            if !is_sret_slot {
                let zero = self.const_val(Const::Zero);
                self.emit(blk, Instruction::StoreTyped(zero, alloc, var_ty.clone()));
            }
            self.set_terminator(blk, Terminator::Br(blk));
            blk
        }
    }

    /// Teremsha `CHAGUA` (chagua): `chagua (skrutinia) { hali: [mikono] la_sivyo: chaguo-msingi }`.
    ///
    /// Mpangilio:
    /// * `ast_kushoto[node]` → usemi wa skrutinia
    /// * `ast_kulia[node]`   → mkono wa kwanza wa hali (uliounganishwa kupitia ast_nne)
    ///   Kila mkono wa hali: ast_kushoto = lebo ya hali, ast_tiga = mwili wa hali.
    /// * `ast_tiga[node]`    → mwili wa chaguo-msingi
    fn lower_switch(&mut self, node: i32) -> BlockId {
        let scrut_node = self.ast_kushoto[node as usize];
        let first_case = self.ast_kulia[node as usize];
        let default_node = self.ast_tiga[node as usize];

        let scrut_blk = self.new_block("switch.scrut");
        let (scrut_val, scrut_end) = self.lower_expr_into(scrut_node, scrut_blk);

        let merge_blk = self.new_block("switch.merge");

        // Teremsha mkono wa chaguo-msingi.
        let default_blk = if default_node != NO_NODE && default_node >= 0 {
            self.lower_block(default_node)
        } else {
            merge_blk
        };

        // Teremsha mikono ya hali.
        let mut arms: Vec<(ValueId, BlockId)> = Vec::new();
        let mut case_node = first_case;
        while case_node != NO_NODE && case_node >= 0 {
            let label_node = self.ast_kushoto[case_node as usize];
            let body_node = self.ast_tiga[case_node as usize];

            let case_blk = self.new_block("switch.case");
            let (label_val, case_label_end) = self.lower_expr_into(label_node, case_blk);
            // Maliza kizuizi cha lebo kwa kuruka kwenye mwili wa hali.
            let body_blk = self.lower_block(body_node);
            self.set_terminator(case_label_end, Terminator::Br(body_blk));
            self.patch_br_if_needed(body_blk, merge_blk);

            arms.push((label_val, body_blk));
            case_node = self.ast_nne[case_node as usize];
        }

        self.set_terminator(
            scrut_end,
            Terminator::Switch(scrut_val, default_blk, arms),
        );

        self.patch_br_if_needed(default_blk, merge_blk);
        merge_blk
    }

    /// Teremsha `VUNJA` (vunja): ruka kwenye kizuizi cha kutoka cha mzunguko wa ndani kabisa.
    fn lower_break(&mut self, _node: i32) -> BlockId {
        let blk = self.new_block("break");
        let exit = self
            .loops
            .last()
            .expect("vunja outside of loop")
            .exit;
        self.set_terminator(blk, Terminator::Br(exit));
        blk
    }

    /// Teremsha `ENDELEA` (endelea): ruka kwenye kizuizi cha kichwa cha mzunguko wa ndani kabisa.
    fn lower_continue(&mut self, _node: i32) -> BlockId {
        let blk = self.new_block("continue");
        let header = self
            .loops
            .last()
            .expect("endelea outside of loop")
            .header;
        self.set_terminator(blk, Terminator::Br(header));
        blk
    }

    /// Teremsha `TENGA` (tenga kwenye chungu): `tenga <aina>` au `tenga <usemi_ukubwa>`.
    ///
    /// Mpangilio:
    /// * `ast_kushoto[node]` → usemi wa ukubwa au nodi ya aina
    fn lower_heap_alloc_stmt(&mut self, node: i32) -> BlockId {
        let arg_node = self.ast_kushoto[node as usize];
        let blk = self.new_block("heap_alloc");

        let (size_val, end_blk) = self.lower_expr_into(arg_node, blk);
        self.emit(end_blk, Instruction::HeapAlloc(size_val));
        // Matokeo ya kielekezi yametupwa katika mazingira ya taarifa.
        self.set_terminator(end_blk, Terminator::Br(end_blk));
        end_blk
    }

    /// Teremsha `ACHILIA` (achilia kwenye chungu): `achilia <usemi_kielekezi>`.
    ///
    /// Mpangilio:
    /// * `ast_kushoto[node]` → usemi wa kielekezi
    fn lower_heap_free_stmt(&mut self, node: i32) -> BlockId {
        let arg_node = self.ast_kushoto[node as usize];
        let blk = self.new_block("heap_free");

        let (ptr_val, end_blk) = self.lower_expr_into(arg_node, blk);
        self.emit(end_blk, Instruction::HeapFree(ptr_val));
        self.set_terminator(end_blk, Terminator::Br(end_blk));
        end_blk
    }
}

// ============================================================================
// Kuteremsha misemo
// ============================================================================

impl<'a> Lowerer<'a> {
    /// Panda faharasa ya safu kwa upana wa kipengele ili uorodheshaji wa
    /// kukabilisha-baiti wa GEP utoe anwani sahihi ya kipengele.
    ///
    /// Hutumia nyongeza iliyorudiwa kuepuka kufanya thabiti mpya ndani
    /// (ambayo ingehamaisha `values_initial_len` na kuvunja ramani ya ValueId).
    fn scale_index(&mut self, elem_ty: &IrType, raw_idx: ValueId, blk: BlockId) -> ValueId {
        match elem_ty.width_bytes() {
            1 => raw_idx,
            2 => self.emit(blk, Instruction::Add(raw_idx, raw_idx)),
            4 => {
                let x2 = self.emit(blk, Instruction::Add(raw_idx, raw_idx));
                self.emit(blk, Instruction::Add(x2, x2))
            }
            8 => {
                let x2 = self.emit(blk, Instruction::Add(raw_idx, raw_idx));
                let x4 = self.emit(blk, Instruction::Add(x2, x2));
                self.emit(blk, Instruction::Add(x4, x4))
            }
            _ => raw_idx,
        }
    }

    /// Teremsha usemi ndani ya kizuizi kilichotolewa (au msururu wa vizuizi kwa
    /// waendeshaji fupi-hali).  Inarudisha `(value, end_block)`.
    fn lower_expr_into(&mut self, node: i32, current_block: BlockId) -> (ValueId, BlockId) {
        if node == NO_NODE || node < 0 {
            let v = self.const_val(Const::Zero);
            return (v, current_block);
        }

        let kind = self.node_aina(node);
        match kind {
            AST_NAMBARI => self.lower_int_literal(node, current_block),
            AST_MFUATANO => self.lower_string_literal(node, current_block),
            AST_KITAMBULISHO => self.lower_identifier(node, current_block),
            AST_WITO => self.lower_call(node, current_block),

            // -- halisi za boolean / null ---------------------------------------
            AST_KWELI => {
                let v = self.func.intern_const(Const::Bool(true));
                (v, current_block)
            }
            AST_UONGO => {
                let v = self.func.intern_const(Const::Bool(false));
                (v, current_block)
            }
            AST_TUPU => {
                let v = self.func.intern_const(Const::NullPtr);
                (v, current_block)
            }

            // -- hesabu --------------------------------------------------------
            AST_JUMLISHA => {
                // AST_JUMLISHA (6) ikiwa na kushoto=NO_NODE ni jumlisha la kwanza (bila kazi).
                // AST_JUMLISHA (6) ikiwa na kushoto=left ni jumlisha la binary.
                if self.ast_kushoto[node as usize] == NO_NODE {
                    // Jumlisha la kwanza — tathmini tu operanda ya kulia.
                    self.lower_expr_into(self.ast_kulia[node as usize], current_block)
                } else {
                    self.lower_binary_op(node, current_block, |l, r| Instruction::Add(l, r))
                }
            }
            AST_TOFAUTI => {
                // AST_TOFAUTI (7) ikiwa na kushoto=NO_NODE ni tofauti la kwanza.
                // AST_TOFAUTI (7) ikiwa na kushoto=left ni toa la binary.
                if self.ast_kushoto[node as usize] == NO_NODE {
                    let operand_node = self.ast_kulia[node as usize];
                    let (operand, end_blk) = self.lower_expr_into(operand_node, current_block);
                    let zero = self.const_val(Const::Int(0));
                    let result = self.emit(end_blk, Instruction::Sub(zero, operand));
                    (result, end_blk)
                } else {
                    self.lower_binary_op(node, current_block, |l, r| Instruction::Sub(l, r))
                }
            }
            AST_ZIDISHA => self.lower_binary_op(node, current_block, |l, r| Instruction::Mul(l, r)),
            AST_GAWANYA => self.lower_binary_op(node, current_block, |l, r| Instruction::DivS(l, r)),

            // -- shughuli za biti ----------------------------------------------
            AST_HAMISHA_KUSHOTO => self.lower_binary_op(node, current_block, |l, r| Instruction::Shl(l, r)),
            AST_HAMISHA_KULIA => self.lower_binary_op(node, current_block, |l, r| Instruction::ShrS(l, r)),
            AST_BIT_NA => self.lower_binary_op(node, current_block, |l, r| Instruction::And(l, r)),
            AST_BIT_AU => self.lower_binary_op(node, current_block, |l, r| Instruction::Or(l, r)),
            AST_TERNARY => self.lower_ternary(node, current_block),
            AST_NA => self.lower_short_circuit_and(node, current_block),
            AST_AU => self.lower_short_circuit_or(node, current_block),

            // -- ulinganisho ----------------------------------------------------
            AST_SAWA => self.lower_binary_op(node, current_block, |l, r| Instruction::Eq(l, r)),
            AST_TOFAUTI_SI => self.lower_binary_op(node, current_block, |l, r| Instruction::Ne(l, r)),
            AST_CHINI => self.lower_binary_op(node, current_block, |l, r| Instruction::LtS(l, r)),
            AST_JUU => self.lower_binary_op(node, current_block, |l, r| Instruction::GtS(l, r)),
            AST_CHINI_SAWA => self.lower_binary_op(node, current_block, |l, r| Instruction::LeS(l, r)),
            AST_JUU_SAWA => self.lower_binary_op(node, current_block, |l, r| Instruction::GeS(l, r)),

            // -- kwanza --------------------------------------------------------
            AST_SI => self.lower_logical_not(node, current_block),
            AST_TAJA => {
                // AST_TAJA inatumika kwa *ptr (kuelekeza) na arr[idx]
                // (usajili wa safu).  Angalia nodi ya faharasa kutofautisha.
                if self.ast_kulia[node as usize] != NO_NODE {
                    self.lower_array_index(node, current_block)
                } else {
                    self.lower_deref_load(node, current_block)
                }
            }
            AST_KUMBUKA => self.lower_address_of(node, current_block),

            // -- ufikiaji wa mwanachama / kipengele ----------------------------
            AST_SEHEMU_DOT => self.lower_field_access(node, current_block),
            AST_SEHEMU_MSHALE => self.lower_ptr_field_access(node, current_block),
            AST_SAFU => self.lower_array_index(node, current_block),

            // -- tenga kwenye chungu kama usemi --------------------------------
            AST_TENGA => {
                let arg_node = self.ast_kushoto[node as usize];
                let (size_val, end_blk) = self.lower_expr_into(arg_node, current_block);
                let ptr = self.emit(end_blk, Instruction::HeapAlloc(size_val));
                (ptr, end_blk)
            }

            // -- ugawaji kama usemi (inarudisha thamani iliyogawiwa) -----------
            AST_ASIMILIA => {
                let lhs_node = self.ast_kushoto[node as usize];
                let rhs_node = self.ast_kulia[node as usize];
                let (rhs_val, end_blk) = self.lower_expr_into(rhs_node, current_block);
                let ptr = self.lower_lvalue(lhs_node, end_blk);
                self.emit(end_blk, Instruction::Store(rhs_val, ptr));
                (rhs_val, end_blk)
            }

            _ => {
                // Usemi usiojulikana: rudisha sifuri.
                let v = self.const_val(Const::Zero);
                (v, current_block)
            }
        }
    }

    // -- visaidizi vya halisi --------------------------------------------------

    fn lower_int_literal(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let val = self.ast_thamani[node as usize] as i128;
        let c = Const::Int(val);
        let prev_len = self.func.values.len();
        let vid = self.func.intern_const(c);
        if self.func.values.len() > prev_len {
            self.values_initial_len = self.func.values.len();
        }
        (vid, blk)
    }

    fn lower_string_literal(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let offset = self.ast_jina_off[node as usize];
        let raw_bytes = self.read_pool_bytes(offset);
        // Chambua mifuatano ya kutoroka: \n \t \r \\ \" \0
        let mut bytes: Vec<u8> = Vec::with_capacity(raw_bytes.len());
        let mut i = 0;
        while i < raw_bytes.len() {
            if raw_bytes[i] == b'\\' && i + 1 < raw_bytes.len() {
                i += 1;
                match raw_bytes[i] {
                    b'n' => bytes.push(b'\n'),
                    b't' => bytes.push(b'\t'),
                    b'r' => bytes.push(b'\r'),
                    b'\\' => bytes.push(b'\\'),
                    b'"' => bytes.push(b'"'),
                    b'0' => bytes.push(b'\0'),
                    other => { bytes.push(b'\\'); bytes.push(other); }
                }
            } else {
                bytes.push(raw_bytes[i]);
            }
            i += 1;
        }
        // Tengeneza lebo na rekodi mfuatano (ya ulimwengu inatolewa baadaye).
        let label = format!("@str.{}", self.strings.len());
        self.strings.push((label.clone(), bytes));
        // Toa amri ya StringAddr inayorejelea lebo ya ulimwengu.
        let ptr = self.emit(blk, Instruction::StringAddr(label));
        (ptr, blk)
    }

    // -- kitambulisho ----------------------------------------------------------

    fn lower_identifier(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let name = self.read_pool_name(self.ast_jina_off[node as usize]);
        if let Some(info) = self.lookup(&name) {
            let alloca_ptr = info.ptr;
            // Kwa aina za muundo, rudisha tu kielekezi cha alloca (vielekezi visivyo wazi).
            if matches!(&info.ty, IrType::Struct { .. }) {
                return (alloca_ptr, blk);
            }
            let loaded_ty = info.ty.clone();
            let val = self.emit(blk, Instruction::Load(loaded_ty, alloca_ptr));
            (val, blk)
        } else if let Some(gty) = self.global_types.get(&name).cloned() {
            let addr = self.emit(blk, Instruction::GlobalAddr(name.clone()));
            // Kwa aina za safu, rudisha kielekezi moja kwa moja (kuoza-safu-kwa-kielekezi).
            // Kwa aina za scalar (I32 = N32 chanzo_urefu), pakia thamani.
            if matches!(&gty, IrType::Array { .. }) {
                (addr, blk)
            } else {
                let val = self.emit(blk, Instruction::Load(gty, addr));
                (val, blk)
            }
        } else {
            let v = self.const_val(Const::Zero);
            (v, blk)
        }
    }

    // -- wito ------------------------------------------------------------------

    /// Kagua AST kwa ufafanuzi wa kazi kwa jina kuangalia rudisha la muundo.
    fn find_function_returns_struct(&self, name: &str) -> bool {
        matches!(self.find_function_return_type(name), Some(IrType::Struct { .. }))
    }

    /// Kagua AST kwa aina ya rudisha ya kazi kwa jina.
    fn find_function_return_type(&self, name: &str) -> Option<IrType> {
        let root = (self.ast_aina.len() - 1) as i32;
        let mut child = self.ast_kushoto[root as usize];
        while child != NO_NODE {
            if self.node_aina(child) == AST_KAZI {
                let name_node = self.ast_kushoto[child as usize];
                if name_node != NO_NODE {
                    let fname = self.read_pool_name(self.ast_jina_off[name_node as usize]);
                    if fname == name {
                        let ret_ty = self.read_type_from_thamani(child);
                        return Some(ret_ty);
                    }
                }
            }
            child = self.ast_nne[child as usize];
        }
        None
    }

    fn lower_call(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let callee_node = self.ast_kushoto[node as usize];
        // Mchanganuzi huhifadhi hoja kwenye kulia ya callee_node: ast_kulia[callee_node] = first_arg.
        let first_arg = if callee_node != NO_NODE && callee_node >= 0 {
            self.ast_kulia[callee_node as usize]
        } else {
            NO_NODE
        };

        let callee_name = if callee_node != NO_NODE && callee_node >= 0 {
            self.read_pool_name(self.ast_jina_off[callee_node as usize])
        } else {
            String::new()
        };

        // Shughulikia vitendajivilivyojengwa ndani vinavyoonekana kama wito wa kazi.
        if callee_name == "ukubwa" {
            let (_, end_blk) = if first_arg != NO_NODE && first_arg >= 0 {
                self.lower_expr_into(first_arg, blk)
            } else {
                (self.func.intern_const(Const::Int(0)), blk)
            };
            let ty = if first_arg != NO_NODE && first_arg >= 0 {
                let kind = self.node_aina(first_arg);
                if kind == AST_KITAMBULISHO {
                    let name = self.read_pool_name(self.ast_jina_off[first_arg as usize]);
                    IrType::from_swa_type(&name).unwrap_or(IrType::I32)
                } else {
                    self.read_type_from_node(first_arg)
                }
            } else {
                IrType::I32
            };
            let width = ty.width_bytes() as i128;
            let size = self.emit(end_blk, Instruction::Const(Const::Int(width)));
            return (size, end_blk);
        }

        if callee_name == "tenga" {
            let (size_val, end_blk) = if first_arg != NO_NODE && first_arg >= 0 {
                self.lower_expr_into(first_arg, blk)
            } else {
                (self.emit(blk, Instruction::Const(Const::Int(0))), blk)
            };
            let ptr = self.emit(end_blk, Instruction::HeapAlloc(size_val));
            return (ptr, end_blk);
        }

        if callee_name == "achilia" {
            let (ptr_val, end_blk) = if first_arg != NO_NODE && first_arg >= 0 {
                self.lower_expr_into(first_arg, blk)
            } else {
                (self.emit(blk, Instruction::Const(Const::NullPtr)), blk)
            };
            self.emit(end_blk, Instruction::HeapFree(ptr_val));
            let zero = self.emit(end_blk, Instruction::Const(Const::Int(0)));
            return (zero, end_blk);
        }

        if callee_name == "badili" {
            let (ptr_val, mid_blk) = if first_arg != NO_NODE && first_arg >= 0 {
                self.lower_expr_into(first_arg, blk)
            } else {
                (self.emit(blk, Instruction::Const(Const::NullPtr)), blk)
            };
            let size_node = self.ast_nne[first_arg as usize];
            let (size_val, end_blk) = if size_node != NO_NODE && size_node >= 0 {
                self.lower_expr_into(size_node, mid_blk)
            } else {
                (self.emit(mid_blk, Instruction::Const(Const::Int(0))), mid_blk)
            };
            let new_ptr = self.emit(end_blk, Instruction::Call("realloc".into(), vec![ptr_val, size_val]));
            return (new_ptr, end_blk);
        }

        // Tathmini hoja.  Mchanganuzi huunganisha hoja kupitia ast_nne ili kuepuka
        // mgongano na watoto wa ast_kulia wa kila nodi ya hoja.
        let mut arg_vals: Vec<ValueId> = Vec::new();
        let mut current_block = blk;
        let mut arg_node = first_arg;
        while arg_node != NO_NODE && arg_node >= 0 {
            let (arg_val, end_blk) = self.lower_expr_into(arg_node, current_block);
            arg_vals.push(arg_val);
            current_block = end_blk;
            arg_node = self.ast_nne[arg_node as usize];
        }

        // Angalia ikiwa kazi iliyoitwa inarudisha muundo (inahitaji kielekezi cha sret).
        // Kwanza angalia kazi zilizoteremshwa tayari, kisha kagua AST kwa rejeleo la mbele.
        let needs_sret = self.functions.iter().any(|f| f.name == callee_name && matches!(f.source_return_ty, IrType::Struct { .. }))
            || self.find_function_returns_struct(&callee_name);
        let (call_val, final_block) = if needs_sret {
            // Tambua aina halisi ya muundo kwa alloca ya sret.
            let struct_ty = self.functions.iter()
                .find(|f| f.name == callee_name)
                .map(|f| f.source_return_ty.clone())
                .or_else(|| {
                    // Rejeleo la mbele: kagua AST kwa aina ya rudisha.
                    self.find_function_return_type(&callee_name)
                })
                .unwrap_or(IrType::I32);
            // Tumia lengo la sret lililotengwa awali ikiwa mpigaji alitoa
            // (mf., kwa `Msambazaji p = call()` ambapo p_alloca tayari imetengwa).
            let sret_alloca = if let Some(dest) = self.sret_dest.take() {
                dest
            } else {
                // Toa ndani ya kizuizi cha sasa.  Hatuwezi kutoa ndani ya kizuizi
                // cha kuingia hapa kwa sababu inst_counter ingegawa ValueId
                // isiyolingana na mpangilio wa urudiaji-kizuizi wa backend (backend
                // inagawa ValueIds za chini kwa amri za kizuizi cha kuingia).
                // Hii ni njia adimu — inawaka tu wakati wito wa rudisha-muundo
                // hauna sret_dest iliyopo awali, ambayo haitokei
                // ndani ya mizunguko katika msimbo wa kujitegemea.
                self.emit(current_block, Instruction::Alloca(struct_ty.clone()))
            };
            let mut sret_args = vec![sret_alloca];
            sret_args.extend(arg_vals);
            let _cv = self.emit(current_block, Instruction::Call(callee_name.clone(), sret_args));
            // Rudisha kielekezi cha alloca cha sret kama matokeo ya wito.  Mpigaji
            // (mf. lower_local_decl) anajua kama alitoa alloca.
            (sret_alloca, current_block)
        } else {
            let cv = self.emit(current_block, Instruction::Call(callee_name.clone(), arg_vals));
            (cv, current_block)
        };
        (call_val, final_block)
    }

    // -- shughuli za binary ----------------------------------------------------

    fn lower_binary_op<F>(
        &mut self,
        node: i32,
        blk: BlockId,
        make_inst: F,
    ) -> (ValueId, BlockId)
    where
        F: FnOnce(ValueId, ValueId) -> Instruction,
    {
        let lhs_node = self.ast_kushoto[node as usize];
        let rhs_node = self.ast_kulia[node as usize];

        let (lhs_val, mid_blk) = self.lower_expr_into(lhs_node, blk);
        let (rhs_val, end_blk) = self.lower_expr_into(rhs_node, mid_blk);
        let result = self.emit(end_blk, make_inst(lhs_val, rhs_val));
        (result, end_blk)
    }

    // -- si la mantiki ---------------------------------------------------------

    fn lower_logical_not(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let operand_node = self.ast_kushoto[node as usize];
        let (operand, end_blk) = self.lower_expr_into(operand_node, blk);
        // SI: linganisha operanda == 0.
        let zero = self.const_val(Const::Int(0));
        let result = self.emit(end_blk, Instruction::Eq(operand, zero));
        (result, end_blk)
    }

    // -- fupi-hali && (NA) -----------------------------------------------------

    /// NA (NA ya mantiki) ni fupi-hali: tathmini kushoto; ikiwa si kweli, matokeo ni
    /// si kweli; vinginevyo tathmini kulia.
    ///
    /// Teremsha ternary `sharti ? thamani_kweli : thamani_si_kweli`.
    /// Hutumia amri ya `Select` ya IR — operanda zote tatu zinatathminiwa katika
    /// kizuizi kimoja (hakuna fupi-hali).
    fn lower_ternary(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let cond_node = self.ast_kushoto[node as usize];
        let true_node = self.ast_kulia[node as usize];
        let false_node = self.ast_tiga[node as usize];

        let (cond_val, blk1) = self.lower_expr_into(cond_node, blk);
        // Badilisha sharti kuwa i1 (select ya LLVM inahitaji sharti la i1).
        let zero = self.const_val(Const::Int(0));
        let cond_bool = self.emit(blk1, Instruction::Ne(cond_val, zero));
        let (true_val, blk2) = self.lower_expr_into(true_node, blk1);
        let (false_val, blk3) = self.lower_expr_into(false_node, blk2);
        let result = self.emit(blk3, Instruction::Select(cond_bool, true_val, false_val));
        (result, blk3)
    }

    /// Inarudisha `true` ikiwa nodi ya AST yenye aina `aina` daima hutoa
    /// thamani ya boolean (ulinganisho, opereta wa mantiki, au halisi ya boolean).
    /// Thamani hizo hazihitaji `Ne(…, 0)` la ziada kuzibadilisha kuwa i1.
    fn ast_aina_ni_boolean(aina: u32) -> bool {
        matches!(aina,
            AST_SAWA          // ==
            | AST_TOFAUTI_SI  // !=
            | AST_CHINI       // <
            | AST_JUU         // >
            | AST_CHINI_SAWA  // <=
            | AST_JUU_SAWA    // >=
            | AST_NA          // &&
            | AST_AU          // ||
            | AST_SI          // !
            | AST_KWELI       // true
            | AST_UONGO       // false
        )
    }

    /// Teremsha `NA` (NA ya mantiki) kwa tathmini sahihi ya fupi-hali
    /// ikitumia nodi ya Phi kuunganisha matokeo mawili yanayowezekana.
    ///
    /// ```text
    ///   entry:
    ///     lhs_val = eval(lhs)
    ///     lhs_bool = lhs_val != 0    (imerukwa wakati lhs tayari ni boolean)
    ///     br lhs_bool ? rhs_blk : merge_blk
    ///
    ///   rhs_blk:
    ///     rhs_val = eval(rhs)
    ///     rhs_bool = rhs_val != 0    (imerukwa wakati rhs tayari ni boolean)
    ///     br merge_blk
    ///
    ///   merge_blk:
    ///     result = phi(B1, [(false, entry), (rhs_bool, rhs_blk)])
    /// ```
    fn lower_short_circuit_and(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let lhs_node = self.ast_kushoto[node as usize];
        let rhs_node = self.ast_kulia[node as usize];

        // Tathmini upande wa kushoto.
        let (lhs_val, lhs_end) = self.lower_expr_into(lhs_node, blk);

        // Badilisha lhs kuwa boolean inapohitajika tu (ulinganisho na shughuli za mantiki
        // tayari hutoa thamani ya boolean).
        let lhs_bool = if Self::ast_aina_ni_boolean(self.node_aina(lhs_node)) {
            lhs_val
        } else {
            let zero = self.const_val(Const::Int(0));
            self.emit(lhs_end, Instruction::Ne(lhs_val, zero))
        };

        // Unda vizuizi vya tathmini ya rhs na muunganiko.
        let rhs_blk = self.new_block("sc_and_rhs");
        let merge_blk = self.new_block("sc_and_merge");

        // Tawi: ikiwa lhs ni kweli → tathmini rhs; sivyo → fupi-hali kwa muunganiko.
        self.set_terminator(lhs_end,
            Terminator::BrCond(lhs_bool, rhs_blk, merge_blk));

        // Njia ya tathmini ya RHS.  Badilisha kuwa boolean inapohitajika tu.
        let (rhs_val, rhs_end) = self.lower_expr_into(rhs_node, rhs_blk);
        let rhs_bool = if Self::ast_aina_ni_boolean(self.node_aina(rhs_node)) {
            rhs_val
        } else {
            let zero = self.const_val(Const::Int(0));
            self.emit(rhs_end, Instruction::Ne(rhs_val, zero))
        };
        self.set_terminator(rhs_end, Terminator::Br(merge_blk));

        // Muunganiko: nodi ya Phi inachagua kati ya si kweli fupi-hali na matokeo ya rhs.
        let false_val = self.const_val(Const::Bool(false));
        let result = self.emit(merge_blk, Instruction::Phi(IrType::B1, vec![
            (false_val, lhs_end),
            (rhs_bool, rhs_end),
        ]));
        // Kishika nafasi — mpigaji ataandika juu kwa kimalizio kinachofaa.
        self.set_terminator(merge_blk, Terminator::Br(merge_blk));
        (result, merge_blk)
    }

    /// Teremsha `AU` (AU ya mantiki) kwa tathmini sahihi ya fupi-hali
    /// ikitumia nodi ya Phi kuunganisha matokeo mawili yanayowezekana.
    ///
    /// ```text
    ///   entry:
    ///     lhs_val = eval(lhs)
    ///     lhs_bool = lhs_val != 0    (imerukwa wakati lhs tayari ni boolean)
    ///     br lhs_bool ? merge_blk : rhs_blk
    ///
    ///   rhs_blk:
    ///     rhs_val = eval(rhs)
    ///     rhs_bool = rhs_val != 0    (imerukwa wakati rhs tayari ni boolean)
    ///     br merge_blk
    ///
    ///   merge_blk:
    ///     result = phi(B1, [(true, entry), (rhs_bool, rhs_blk)])
    /// ```
    fn lower_short_circuit_or(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let lhs_node = self.ast_kushoto[node as usize];
        let rhs_node = self.ast_kulia[node as usize];

        // Tathmini upande wa kushoto.
        let (lhs_val, lhs_end) = self.lower_expr_into(lhs_node, blk);

        // Badilisha lhs kuwa boolean inapohitajika tu.
        let lhs_bool = if Self::ast_aina_ni_boolean(self.node_aina(lhs_node)) {
            lhs_val
        } else {
            let zero = self.const_val(Const::Int(0));
            self.emit(lhs_end, Instruction::Ne(lhs_val, zero))
        };

        // Unda vizuizi vya tathmini ya rhs na muunganiko.
        let rhs_blk = self.new_block("sc_or_rhs");
        let merge_blk = self.new_block("sc_or_merge");

        // Tawi: ikiwa lhs ni kweli → fupi-hali kwa muunganiko; sivyo → tathmini rhs.
        self.set_terminator(lhs_end,
            Terminator::BrCond(lhs_bool, merge_blk, rhs_blk));

        // Njia ya tathmini ya RHS.  Badilisha kuwa boolean inapohitajika tu.
        let (rhs_val, rhs_end) = self.lower_expr_into(rhs_node, rhs_blk);
        let rhs_bool = if Self::ast_aina_ni_boolean(self.node_aina(rhs_node)) {
            rhs_val
        } else {
            let zero = self.const_val(Const::Int(0));
            self.emit(rhs_end, Instruction::Ne(rhs_val, zero))
        };
        self.set_terminator(rhs_end, Terminator::Br(merge_blk));

        // Muunganiko: nodi ya Phi inachagua kati ya kweli fupi-hali na matokeo ya rhs.
        let true_val = self.const_val(Const::Bool(true));
        let result = self.emit(merge_blk, Instruction::Phi(IrType::B1, vec![
            (true_val, lhs_end),
            (rhs_bool, rhs_end),
        ]));
        // Kishika nafasi — mpigaji ataandika juu kwa kimalizio kinachofaa.
        self.set_terminator(merge_blk, Terminator::Br(merge_blk));
        (result, merge_blk)
    }

    // -- shughuli za kielekezi / anwani ----------------------------------------

    /// Teremsha `*usemi` (kuelekeza / kupakia kielekezi).
    ///
    /// Kwa kuelekeza kwa kawaida (`*ptr`), hutatua aina ya pointee kutoka
    /// aina iliyotangazwa ya operanda inapowezekana.  Kwa usajili wa safu
    /// (`arr[idx]`), mpigaji (`lower_expr_into`) anatuma kwa
    /// [`lower_array_index`] badala yake, inayoshughulikia GEP + upakiaji wenye aina.
    fn lower_deref_load(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let operand_node = self.ast_kushoto[node as usize];
        let (ptr_val, end_blk) = self.lower_expr_into(operand_node, blk);
        // Jaribu kutatua aina ya pointee kutoka aina iliyotangazwa ya operanda.
        let pointee_ty = self.resolve_expr_type(operand_node)
            .and_then(|ty| match &ty {
                IrType::Ptr(inner) => Some((**inner).clone()),
                _ => None,
            })
            .unwrap_or(IrType::I8);
        let val = self.emit(end_blk, Instruction::Load(pointee_ty, ptr_val));
        (val, end_blk)
    }

    /// Teremsha `&usemi` (anwani-ya).
    fn lower_address_of(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let operand_node = self.ast_kushoto[node as usize];
        // Operanda lazima iwe thamani-l — iteremshe kuwa kielekezi.
        let ptr = self.lower_lvalue(operand_node, blk);
        (ptr, blk)
    }

    // -- kuteremsha thamani-l --------------------------------------------------

    /// Teremsha nodi kama *thamani-l*, ikirudisha kielekezi (`ValueId`) ambacho
    /// kinaweza kuhifadhiwa au kupakiwa kutoka.
    fn lower_lvalue(&mut self, node: i32, blk: BlockId) -> ValueId {
        if node == NO_NODE || node < 0 {
            return self.const_val(Const::NullPtr);
        }

        let kind = self.node_aina(node);
        match kind {
            AST_KITAMBULISHO => {
                let name = self.read_pool_name(self.ast_jina_off[node as usize]);
                if let Some(info) = self.lookup(&name) {
                    info.ptr
                } else if self.global_types.contains_key(&name) {
                    self.emit(blk, Instruction::GlobalAddr(name.clone()))
                } else {
                    // Haijafafanuliwa — rudisha kielekezi batili.
                    self.const_val(Const::NullPtr)
                }
            }
            AST_TAJA => {
                // *ptr au safu[faharasa] kama thamani-l.
                let base_node = self.ast_kushoto[node as usize];
                let index_node = self.ast_kulia[node as usize];
                let (base_ptr, end_blk) = self.lower_expr_into(base_node, blk);
                if index_node != NO_NODE && index_node >= 0 {
                    let (raw_idx, end_blk2) = self.lower_expr_into(index_node, end_blk);
                    // Tatua aina ya kipengele kwa ukokotoaji sahihi wa faharasa.
                    let elem_ty = self.resolve_expr_type(base_node)
                        .and_then(|ty| match &ty {
                            IrType::Ptr(pointee) => Some((**pointee).clone()),
                            IrType::Array { element, .. } => Some((**element).clone()),
                            other => Some(other.clone()),
                        })
                        .unwrap_or(IrType::I32);
                    let idx_val = self.scale_index(&elem_ty, raw_idx, end_blk2);
                    let gep = self.emit(end_blk2, Instruction::Gep(base_ptr, vec![idx_val]));
                    gep
                } else {
                    base_ptr
                }
            }
            AST_SEHEMU_DOT => {
                // muundo.sehemu: hesabu anwani ya sehemu.
                // Jina la sehemu limehifadhiwa kwenye nodi ya ufikiaji-doti yenyewe kupitia hifadhi_jina.
                let struct_node = self.ast_kushoto[node as usize];
                let field_name = self.read_pool_name(self.ast_jina_off[node as usize]);

                let base_ptr = self.lower_lvalue(struct_node, blk);

                // Tatua aina ya muundo kutoka usemi, kisha tafuta
                // faharasa ya sehemu ndani ya muundo huo mahususi.
                let struct_ty = self.resolve_expr_type(struct_node)
                    .and_then(|ty| match &ty {
                        IrType::Ptr(pointee) => Some((**pointee).clone()),
                        IrType::Struct { .. } => Some(ty.clone()),
                        _ => None,
                    });
                let field_idx = struct_ty.as_ref()
                    .and_then(|sty| Self::find_field_index(sty, &field_name))
                    .unwrap_or_else(|| self.guess_field_index(&field_name));

                self.emit(blk, Instruction::FieldAddr(base_ptr, field_idx, struct_ty))
            }
            AST_SEHEMU_MSHALE => {
                // ptr->sehemu: pakia ptr, kisha hesabu anwani ya sehemu.
                // Jina la sehemu limehifadhiwa kwenye nodi ya mshale yenyewe kupitia hifadhi_jina.
                let ptr_node = self.ast_kushoto[node as usize];
                let field_name = self.read_pool_name(self.ast_jina_off[node as usize]);

                let (struct_ptr, end_blk) = self.lower_expr_into(ptr_node, blk);

                // Tatua aina ya muundo kutoka pointee ya kielekezi, kisha
                // tafuta faharasa ya sehemu ndani ya muundo huo mahususi.
                let struct_ty = self.resolve_expr_type(ptr_node).and_then(|ty| {
                    match &ty {
                        IrType::Ptr(pointee) => Some((**pointee).clone()),
                        IrType::Struct { .. } => Some(ty.clone()),
                        _ => None,
                    }
                });
                let field_idx = struct_ty.as_ref()
                    .and_then(|sty| Self::find_field_index(sty, &field_name))
                    .unwrap_or_else(|| self.guess_field_index(&field_name));

                self.emit(end_blk, Instruction::FieldAddr(struct_ptr, field_idx, struct_ty))
            }
            AST_SAFU => {
                // safu[faharasa] — hesabu anwani ya kipengele kupitia GEP.
                let array_node = self.ast_kushoto[node as usize];
                let index_node = self.ast_kulia[node as usize];

                let raw_ptr = self.lower_lvalue(array_node, blk);
                // Ikiwa safu kwa kweli ni kigezo cha kielekezi (N8**), lower_lvalue
                // inarudisha alloca. Pakia thamani ya kielekezi kwa GEP sahihi.
                let arr_ty = self.resolve_expr_type(array_node);
                let is_ptr = matches!(&arr_ty, Some(IrType::Ptr(_)));
                let ary_ptr = if is_ptr {
                    let loaded_ty = arr_ty.clone().unwrap();
                    self.emit(blk, Instruction::Load(loaded_ty, raw_ptr))
                } else {
                    raw_ptr
                };
                let (raw_idx, end_blk) = self.lower_expr_into(index_node, blk);

                // Tambua aina ya kipengele kwa ukubwa wa faharasa.
                let elem_ty = arr_ty.and_then(|ty| {
                    match &ty {
                        IrType::Ptr(pointee) => Some((**pointee).clone()),
                        IrType::Array { element, .. } => Some((**element).clone()),
                        other => Some(other.clone()),
                    }
                }).unwrap_or(IrType::I32);

                // Panda faharasa kwa upana wa kipengele — GEP hutumia kukabilisha kwa baiti.
                let idx_val = self.scale_index(&elem_ty, raw_idx, end_blk);

                self.emit(end_blk, Instruction::Gep(ary_ptr, vec![idx_val]))
            }
            _ => {
                // Si thamani-l — tathmini kama thamani-r na rudisha kielekezi bandia.
                let (_val, _end_blk) = self.lower_expr_into(node, blk);
                self.const_val(Const::NullPtr)
            }
        }
    }

    // -- ufikiaji wa sehemu kama thamani-r -------------------------------------

    /// Teremsha `muundo.sehemu` (ufikiaji wa doti) kama thamani-r.
    /// Tatua aina ya nodi ya usemi kwa kutembea AST.
    fn resolve_expr_type(&self, node: i32) -> Option<IrType> {
        if node < 0 { return None; }
        match self.ast_aina[node as usize] {
            AST_KITAMBULISHO => {
                let name = self.read_pool_name(self.ast_jina_off[node as usize]);
                self.lookup(&name).map(|info| info.ty.clone())
                    .or_else(|| self.global_types.get(&name).cloned())
            }
            AST_SEHEMU_DOT => {
                // p.x → tatua aina ya p, tafuta sehemu x.
                let lhs = self.ast_kushoto[node as usize];
                let field = self.read_pool_name(self.ast_jina_off[node as usize]);
                self.resolve_expr_type(lhs).and_then(|ty| {
                    let st = match &ty {
                        IrType::Struct { .. } => ty.clone(),
                        IrType::Ptr(i) if matches!(**i, IrType::Struct { .. }) => (**i).clone(),
                        _ => return None,
                    };
                    if let IrType::Struct { fields, .. } = st {
                        fields.iter().find(|(n, _)| n == &field).map(|(_, t)| t.clone())
                    } else { None }
                })
            }
            AST_SEHEMU_MSHALE => {
                // p->x → tatua aina ya p (kielekezi), pata muundo wa pointee, tafuta sehemu x.
                let lhs = self.ast_kushoto[node as usize];
                let field = self.read_pool_name(self.ast_jina_off[node as usize]);
                self.resolve_expr_type(lhs).and_then(|ty| {
                    let st = match &ty {
                        IrType::Ptr(inner) => (**inner).clone(),
                        _ => return None,
                    };
                    if let IrType::Struct { fields, .. } = st {
                        fields.iter().find(|(n, _)| n == &field).map(|(_, t)| t.clone())
                    } else { None }
                })
            }
            AST_SAFU => {
                let array_node = self.ast_kushoto[node as usize];
                self.resolve_expr_type(array_node).and_then(|ty| match &ty {
                    IrType::Array { element, .. } => Some((**element).clone()),
                    IrType::Ptr(pointee) => Some((**pointee).clone()),
                    _ => None,
                })
            }
            _ => None,
        }
    }

    fn lower_field_access(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let struct_node = self.ast_kushoto[node as usize];
        // Jina la sehemu limehifadhiwa kwenye nodi ya ufikiaji-doti yenyewe kupitia hifadhi_jina.
        let field_name = self.read_pool_name(self.ast_jina_off[node as usize]);

        // Tatua aina ya usemi wa upande wa kushoto mara moja.
        let lhs_ty = self.resolve_expr_type(struct_node);

        // Tambua aina ya muundo na aina ya sehemu kutoka lhs_ty.
        let (struct_ty, field_ty, field_idx) = match &lhs_ty {
            Some(IrType::Struct { fields, .. }) => {
                let idx = Self::find_field_index(lhs_ty.as_ref().unwrap(), &field_name)
                    .unwrap_or_else(|| self.guess_field_index(&field_name));
                let fty = fields.iter()
                    .find(|(n, _)| n == &field_name)
                    .map(|(_, t)| t.clone())
                    .unwrap_or(IrType::I32);
                // Kwa ufikiaji wa muundo moja kwa moja (si kupitia kielekezi), struct_ty ni aina ya muundo yenyewe.
                let sty = lhs_ty.clone();
                (sty, fty, idx)
            }
            Some(IrType::Ptr(pointee)) if matches!(**pointee, IrType::Struct { .. }) => {
                let st = (**pointee).clone();
                let idx = Self::find_field_index(&st, &field_name)
                    .unwrap_or_else(|| self.guess_field_index(&field_name));
                let fty = if let IrType::Struct { fields, .. } = &st {
                    fields.iter()
                        .find(|(n, _)| n == &field_name)
                        .map(|(_, t)| t.clone())
                        .unwrap_or(IrType::I32)
                } else { IrType::I32 };
                // Aina ya muundo kwa FieldAddr ni muundo wa pointee.
                let sty = Some(st);
                (sty, fty, idx)
            }
            _ => {
                let idx = self.guess_field_index(&field_name);
                (None, IrType::I32, idx)
            }
        };

        // Pata anwani ya muundo (kama thamani-l).
        let base_ptr = self.lower_lvalue(struct_node, blk);

        // Hesabu anwani ya sehemu, kisha pakia kwa aina sahihi.
        let field_ptr = self.emit(blk, Instruction::FieldAddr(base_ptr, field_idx, struct_ty));
        let val = self.emit(blk, Instruction::Load(field_ty, field_ptr));
        (val, blk)
    }

    /// Teremsha `ptr->sehemu` (ufikiaji wa mshale) kama thamani-r.
    fn lower_ptr_field_access(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let ptr_node = self.ast_kushoto[node as usize];
        // Jina la sehemu limehifadhiwa kwenye nodi ya ufikiaji-mshale yenyewe kupitia hifadhi_jina.
        let field_name = self.read_pool_name(self.ast_jina_off[node as usize]);

        let (struct_ptr, end_blk) = self.lower_expr_into(ptr_node, blk);

        // Tatua aina ya muundo wa pointee kwa kutumia resolve_expr_type (inashughulikia
        // vitambulisho, ufikiaji wa sehemu, na misemo mingine).
        let struct_ty_opt = self.resolve_expr_type(ptr_node).and_then(|ty| {
            match &ty {
                IrType::Ptr(pointee) => Some((**pointee).clone()),
                _ => None,
            }
        });
        let field_idx = struct_ty_opt.as_ref()
            .and_then(|sty| Self::find_field_index(sty, &field_name))
            .unwrap_or_else(|| self.guess_field_index(&field_name));

        // Tambua aina ya sehemu kutoka muundo wa pointee uliotatuliwa.
        // Ikiwa hatuwezi kuipata kupitia msururu wa upeo, jaribu jedwali la aina za moduli.
        let field_ty = struct_ty_opt.as_ref().and_then(|sty| {
            if let IrType::Struct { fields, .. } = sty {
                fields.iter().find(|(n, _)| n == &field_name).map(|(_, t)| t.clone())
            } else { None }
        }).or_else(|| {
            // Rudia: jaribu jedwali la aina za kiwango-moduli kwa jina la muundo
            struct_ty_opt.as_ref().and_then(|sty| {
                if let IrType::Struct { name, .. } = sty {
                    self.types.iter().find(|(n, _)| n == name).and_then(|(_, t)| {
                        if let IrType::Struct { fields, .. } = t {
                            fields.iter().find(|(n, _)| n == &field_name).map(|(_, ft)| ft.clone())
                        } else { None }
                    })
                } else { None }
            })
        }).unwrap_or(IrType::I32);

        let field_ptr = self.emit(end_blk, Instruction::FieldAddr(struct_ptr, field_idx, struct_ty_opt));
        let val = self.emit(end_blk, Instruction::Load(field_ty, field_ptr));
        (val, end_blk)
    }

    /// Teremsha `safu[faharasa]` kama thamani-r.
    fn lower_array_index(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let array_node = self.ast_kushoto[node as usize];
        let index_node = self.ast_kulia[node as usize];

        let raw_ptr = self.lower_lvalue(array_node, blk);
        // Ikiwa hiki ni kigezo cha kielekezi (N8**), lower_lvalue inarudisha alloca.
        // Pakia thamani halisi ya kielekezi kwa GEP sahihi.
        let arr_ty = self.resolve_expr_type(array_node);
        let is_ptr = matches!(&arr_ty, Some(IrType::Ptr(_)));
        let ary_ptr = if is_ptr {
            let loaded_ty = arr_ty.clone().unwrap();
            self.emit(blk, Instruction::Load(loaded_ty, raw_ptr))
        } else {
            raw_ptr
        };
        let (raw_idx, end_blk) = self.lower_expr_into(index_node, blk);

        // Tambua aina ya kipengele kutoka aina iliyotangazwa ya safu.
        let elem_ty = arr_ty.and_then(|ty| {
            match &ty {
                IrType::Ptr(pointee) => Some((**pointee).clone()),
                IrType::Array { element, .. } => Some((**element).clone()),
                other => Some(other.clone()),
            }
        }).unwrap_or(IrType::I32);

        // Panda faharasa kwa upana wa kipengele — GEP hutumia kukabilisha kwa baiti.
        let idx_val = self.scale_index(&elem_ty, raw_idx, end_blk);

        // GEP kwa kipengele, kisha pakia.
        let elem_ptr = self.emit(end_blk, Instruction::Gep(ary_ptr, vec![idx_val]));
        let val = self.emit(end_blk, Instruction::Load(elem_ty, elem_ptr));
        (val, end_blk)
    }

}

// ============================================================================
// Visaidizi
// ============================================================================

impl<'a> Lowerer<'a> {
    /// Ikiwa kimalizio cha sasa cha `block` ni kishika nafasi cha kujizungusha
    /// (`Br(block)`), badilisha na `Br(target)`.
    fn patch_br_if_needed(&mut self, block: BlockId, target: BlockId) {
        // Tembea msururu kupitia matawi yasiyo na sharti na kupitia
        // njia ya muunganiko (kipitio) ya matawi yenye sharti, ukirekebisha
        // kila kishika nafasi cha kujizungusha kwa `target`.  Hii inahakikisha
        // taarifa za kama/wakati zilizopachikwa ndani ya kizuizi cha kweli
        // zote hatimaye zinafikia muendelezo sahihi.
        let mut visited: Vec<BlockId> = Vec::new();
        let mut work = vec![block];
        while let Some(blk) = work.pop() {
            if visited.contains(&blk) { continue; }
            visited.push(blk);
            let term = &self.func.blocks[blk.0].terminator;
            match term {
                Terminator::Br(b) if *b == blk => {
                    // Kishika nafasi cha kujizungusha — rekebisha kwa target.
                    self.set_terminator(blk, Terminator::Br(target));
                }
                Terminator::Br(next) => {
                    // Fuata msururu usio na sharti, lakini simama kwenye vizuizi
                    // ambavyo lebo yake inaonyesha mtiririko-dhibiti wa mzunguko
                    // (endelea au vunja).  Kufuata hivi kungetembea
                    // ndani ya miili ya mizunguko iliyofunga na kuharibu
                    // vizuizi vyao vya kutoka.
                    let src_label = &self.func.blocks[blk.0].label;
                    if !src_label.starts_with("continue.")
                        && !src_label.starts_with("break.")
                    {
                        work.push(*next);
                    }
                }
                Terminator::BrCond(_, true_target, false_target) => {
                    // Fuata matawi YOTE MAMAWILI — tawi la kweli linaweza
                    // kupeleka kwenye vizuizi vinavyohitaji kurekebishwa (mf.
                    // kama iliyopachikwa yenye sivyo ambapo tawi la uongo
                    // linaisha kwa Ret).
                    work.push(*true_target);
                    work.push(*false_target);
                }
                _ => {
                    // Kimalizio halisi (Ret, Switch) — simama.
                }
            }
        }
    }

    /// Hakikisha kizuizi kilichotolewa kina tawi lisilo na sharti kwa `target`.  Ikiwa
    /// kizuizi tayari kina kimalizio kisicho kishika nafasi hii ni no-op.
    fn ensure_br(&mut self, block: BlockId, target: BlockId) {
        let current_term = &self.func.blocks[block.0].terminator;
        match current_term {
            Terminator::Br(b) if *b == block || *b == target => {
                // Kishika nafasi au tayari sahihi — andika juu.
                self.set_terminator(block, Terminator::Br(target));
            }
            Terminator::Br(_) => {
                // Tayari inatawi mahali pengine — acha peke yake.
            }
            _ => {
                // Ina kimalizio halisi (Ret, BrCond, Switch) — acha peke yake.
            }
        }
    }

    /// Tafuta faharasa ya sehemu yenye jina ndani ya aina mahususi ya muundo.
    ///
    /// Inarudisha `None` wakati sehemu haipatikani au aina si muundo.
    fn find_field_index(struct_ty: &IrType, field_name: &str) -> Option<usize> {
        if let IrType::Struct { fields, .. } = struct_ty {
            fields.iter().position(|(n, _)| n == field_name)
        } else {
            None
        }
    }

    /// Rudia la urithi: tafuta aina zote za muundo zilizosajiliwa kwa jina la sehemu.
    /// Inatumika tu wakati aina mahususi ya muundo haiwezi kutatuliwa.
    fn guess_field_index(&self, name: &str) -> usize {
        for (_, ty) in &self.types {
            if let IrType::Struct { fields, .. } = ty {
                if let Some(idx) = fields.iter().position(|(n, _)| n == name) {
                    return idx;
                }
            }
        }
        0
    }

}

// ============================================================================
// Majaribio
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Visaidizi vya mjenzi wa AST
    // -----------------------------------------------------------------------

    /// Mjenzi mdogo wa kuunda AST za safu-bapa katika majaribio.
    struct AstBuilder {
        aina: Vec<u32>,
        kushoto: Vec<i32>,
        kulia: Vec<i32>,
        tiga: Vec<i32>,
        nne: Vec<i32>,
        thamani: Vec<i32>,
        jina_off: Vec<i32>,
        pool: Vec<u8>,
    }

    impl AstBuilder {
        fn new() -> Self {
            Self {
                aina: Vec::new(),
                kushoto: Vec::new(),
                kulia: Vec::new(),
                tiga: Vec::new(),
                nne: Vec::new(),
                thamani: Vec::new(),
                jina_off: Vec::new(),
                pool: Vec::new(),
            }
        }

        /// Tenge nodi mpya, rudisha faharasa yake.
        fn node(
            &mut self,
            kind: u32,
            kushoto: i32,
            kulia: i32,
            tiga: i32,
            nne: i32,
            thamani: i32,
            jina_off: i32,
        ) -> i32 {
            let idx = self.aina.len() as i32;
            self.aina.push(kind);
            self.kushoto.push(kushoto);
            self.kulia.push(kulia);
            self.tiga.push(tiga);
            self.nne.push(nne);
            self.thamani.push(thamani);
            self.jina_off.push(jina_off);
            idx
        }

        /// Ongeza jina lenye ncha-tupu kwenye dimbwi, rudisha kukabilisha kwake.
        fn pool_name(&mut self, name: &str) -> i32 {
            let off = self.pool.len() as i32;
            self.pool.extend_from_slice(name.as_bytes());
            self.pool.push(0);
            off
        }

        /// Ongeza baiti zenye urefu-kiambishi kwenye dimbwi, rudisha kukabilisha.
        fn pool_bytes(&mut self, data: &[u8]) -> i32 {
            let off = self.pool.len() as i32;
            let len = data.len() as u32;
            self.pool.extend_from_slice(&len.to_le_bytes());
            self.pool.extend_from_slice(data);
            off
        }

        /// Jenga mzizi mdogo wa PROGRAMU unaofunga mtoto mmoja na urudishe
        /// safu pamoja na `ast_idadi`.
        fn finish(&mut self, root_child: i32) -> (Vec<u32>, Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>, Vec<u8>, usize) {
            let root = self.node(
                AST_PROGRAMU,     // kind
                root_child,       // kushoto = first child
                NO_NODE,          // kulia
                NO_NODE,          // tiga
                NO_NODE,          // nne
                0,                // thamani
                0,                // jina_off
            );
            let _ = root; // root is the last node
            let idadi = self.aina.len();
            (
                std::mem::take(&mut self.aina),
                std::mem::take(&mut self.kushoto),
                std::mem::take(&mut self.kulia),
                std::mem::take(&mut self.tiga),
                std::mem::take(&mut self.nne),
                std::mem::take(&mut self.thamani),
                std::mem::take(&mut self.jina_off),
                std::mem::take(&mut self.pool),
                idadi,
            )
        }
    }

    // -----------------------------------------------------------------------
    // Majaribio
    // -----------------------------------------------------------------------

    #[test]
    fn test_empty_program() {
        // Programu isiyo na kazi au vigezo vya ulimwengu — mzizi tu.
        let mut b = AstBuilder::new();
        let root_child = NO_NODE;
        let (aina, kushoto, kulia, tiga, nne, thamani, jina_off, pool, idadi) =
            b.finish(root_child);

        let module = lower(&aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, idadi);
        assert!(module.functions.is_empty());
        assert_eq!(module.globals.len(), 0);
    }

    #[test]
    fn test_simple_function_no_body() {
        // kazi kuu() { } → kazi isiyo na mwili
        let mut b = AstBuilder::new();
        let jina_kuu = b.pool_name("kuu");
        // Aina iliyosimbwa kwa W0 (Tupu): familia=5, upana=0, mshale=0 → (5<<8)|0 = 1280
        let w0_enc: i32 = 10240; // (5<<11)|(0<<3)|0

        let name_node = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, jina_kuu);
        let func_node = b.node(
            AST_KAZI,
            name_node, // kushoto = name node
            NO_NODE,   // kulia = no params
            NO_NODE,   // tiga = no body
            NO_NODE,   // nne
            w0_enc,    // thamani = encoded W0 type
            0,         // jina_off unused on func
        );

        let (aina, kushoto, kulia, tiga, nne, thamani, jina_off, pool, idadi) =
            b.finish(func_node);

        let module = lower(&aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, idadi);
        assert_eq!(module.functions.len(), 1);
        let f = &module.functions[0];
        assert_eq!(f.name, "kuu");
        assert_eq!(f.return_ty, IrType::Void);
        assert_eq!(f.params.len(), 0);
        // Inapaswa kuwa na angalau kizuizi kimoja cha kuingia.
        assert!(f.block_count() >= 1);
    }

    #[test]
    fn test_function_with_params() {
        // kazi jumlisha(a: N32, b: N32): N32 { ... }
        let mut b = AstBuilder::new();
        let jina_jumlisha = b.pool_name("jumlisha");
        // Aina iliyosimbwa kwa N32: familia=1, upana=32, mshale=0 → (1<<8)|32 = 288
        let n32_enc: i32 = 2080; // (1<<11)|(4<<3)|0

        // Nodi za vigezo: kila moja ina jina_off kwa jina, thamani kwa usimbaji wa aina.
        let jina_a = b.pool_name("a");
        let param_a = b.node(
            0, // dummy kind for param
            NO_NODE, NO_NODE, NO_NODE, NO_NODE,
            n32_enc,                    // thamani = encoded N32 type
            jina_a,                     // jina_off = "a"
        );
        let jina_b = b.pool_name("b");
        let param_b = b.node(
            0, NO_NODE, NO_NODE, NO_NODE, NO_NODE,
            n32_enc,
            jina_b,
        );
        // Unganisha vigezo: a → b kupitia kulia (kulingana na muundo wa mchanganuzi).
        b.kulia[param_a as usize] = param_b;

        let name_node = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, jina_jumlisha);
        let func_node = b.node(
            AST_KAZI,
            name_node,   // kushoto = name node
            param_a,     // kulia = first param
            NO_NODE,     // tiga = no body
            NO_NODE,     // nne
            n32_enc,     // thamani = encoded N32 type
            0,           // jina_off unused on func
        );

        let (aina, kushoto, kulia, tiga, nne, thamani, jina_off, pool, idadi) =
            b.finish(func_node);

        let module = lower(&aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, idadi);
        assert_eq!(module.functions.len(), 1);
        let f = &module.functions[0];
        assert_eq!(f.name, "jumlisha");
        assert_eq!(f.return_ty, IrType::I32);
        assert_eq!(f.params.len(), 2);
        assert_eq!(f.params[0].0, "a");
        assert_eq!(f.params[1].0, "b");
    }

    #[test]
    fn test_function_with_return_expr() {
        // kazi tatu(): N32 { rudisha 3 }
        let mut b = AstBuilder::new();
        let jina_tatu = b.pool_name("tatu");
        let ret_off = b.pool_name("N32");

        // Halisi ya nambari "3": AST_NAMBARI, thamani = 3
        let lit = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 3, 0);

        // Taarifa ya rudisha: AST_RUDISHA, kushoto = lit
        let ret_stmt = b.node(AST_RUDISHA, lit, NO_NODE, NO_NODE, NO_NODE, 0, 0);

        let name_node = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, jina_tatu);
        let func_node = b.node(
            AST_KAZI,
            name_node,  // kushoto = name node
            NO_NODE,    // kulia = no params
            ret_stmt,   // tiga = body (return stmt)
            NO_NODE,    // nne
            ret_off,    // thamani = "N32"
            0,          // jina_off unused on func
        );

        let (aina, kushoto, kulia, tiga, nne, thamani, jina_off, pool, idadi) =
            b.finish(func_node);

        let module = lower(&aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, idadi);
        assert_eq!(module.functions.len(), 1);
        let f = &module.functions[0];
        assert_eq!(f.name, "tatu");
        // Inapaswa kuwa na vizuizi: kuingia, mwili, rudisha
        assert!(f.block_count() >= 3, "expected at least 3 blocks, got {}", f.block_count());
        // Thibitisha kimalizio cha Ret kipo mahali fulani.
        let has_ret = f.blocks.iter().any(|blk| matches!(blk.terminator, Terminator::Ret(_)));
        assert!(has_ret, "function should contain a Ret terminator");
        // Thibitisha thabiti ya nambari 3 imefanywa ndani.
        let has_int3 = f.values.iter().any(|c| *c == Const::Int(3));
        assert!(has_int3, "function should contain Const::Int(3)");
    }

    #[test]
    fn test_local_variable_decl_and_assign() {
        // kazi hesabu(): N32 {
        //   N32 x = 5;
        //   x = 10;
        //   rudisha x;
        // }
        let mut b = AstBuilder::new();
        let jina_hesabu = b.pool_name("hesabu");
        let ret_off = b.pool_name("N32");
        let n32_off = b.pool_name("N32");

        // Kitambulisho "x"
        let id_x_off = b.pool_name("x");

        // Halisi
        let lit5 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 5, 0);
        let lit10 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 10, 0);

        // Nodi ya aina kwa N32
        let type_n32 = b.node(0, NO_NODE, NO_NODE, NO_NODE, NO_NODE, n32_off, 0);

        // Nodi ya jina kwa x
        let name_x = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, id_x_off);

        // TANGAZO: N32 x = 5
        let decl = b.node(
            AST_TANGAZO,
            name_x,     // kushoto = variable name
            type_n32,   // kulia = type
            lit5,       // tiga = init
            NO_NODE,    // nne
            0, 0,
        );

        // Kitambulisho cha kulia "x" kwa ugawaji
        let id_x_rhs = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, id_x_off);

        // ASIMILIA: x = 10
        let assign = b.node(
            AST_ASIMILIA,
            id_x_rhs,   // kushoto = lvalue
            lit10,      // kulia = rvalue
            NO_NODE, NO_NODE, 0, 0,
        );

        // Rudisha: rudisha x
        let id_x_ret = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, id_x_off);
        let ret_stmt = b.node(AST_RUDISHA, id_x_ret, NO_NODE, NO_NODE, NO_NODE, 0, 0);

        // Msururu: tangazo → ugawaji → rudisha
        b.nne[decl as usize] = assign;
        b.nne[assign as usize] = ret_stmt;

        let name_node = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, jina_hesabu);
        let func_node = b.node(
            AST_KAZI,
            name_node,  // kushoto = name node
            NO_NODE,    // kulia = no params
            decl,       // tiga = first stmt in body
            NO_NODE,    // nne
            ret_off,    // thamani = "N32"
            0,          // jina_off unused on func
        );

        let (aina, kushoto, kulia, tiga, nne, thamani, jina_off, pool, idadi) =
            b.finish(func_node);

        let module = lower(&aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, idadi);
        assert_eq!(module.functions.len(), 1);
        let f = &module.functions[0];
        assert_eq!(f.name, "hesabu");

        // Angalia kwamba amri za Alloca na Store zipo.
        let has_alloca = f.blocks.iter().any(|blk| {
            blk.instructions.iter().any(|inst| matches!(inst, Instruction::Alloca(_)))
        });
        assert!(has_alloca, "function should have Alloca instructions");

        let has_store = f.blocks.iter().any(|blk| {
            blk.instructions.iter().any(|inst| matches!(inst, Instruction::Store(_, _) | Instruction::StoreTyped(_, _, _)))
        });
        assert!(has_store, "function should have Store or StoreTyped instructions");
    }

    #[test]
    fn test_if_statement() {
        // kazi kadirifu(x: N32): N32 {
        //   kama (x) rudisha 1;
        //   rudisha 0;
        // }
        let mut b = AstBuilder::new();
        let jina_kadirifu = b.pool_name("kadirifu");
        let ret_off = b.pool_name("N32");
        let n32_off = b.pool_name("N32");

        // Kigezo x
        let jina_x = b.pool_name("x");
        let param_x = b.node(0, NO_NODE, NO_NODE, NO_NODE, NO_NODE, n32_off, jina_x);

        // Halisi
        let lit1 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 1, 0);
        let lit0 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, 0);

        // Kitambulisho x
        let id_x_off = b.pool_name("x");
        let id_x = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, id_x_off);

        // ndipo: rudisha 1
        let ret1 = b.node(AST_RUDISHA, lit1, NO_NODE, NO_NODE, NO_NODE, 0, 0);
        // la_sivyo: rudisha 0
        let ret0 = b.node(AST_RUDISHA, lit0, NO_NODE, NO_NODE, NO_NODE, 0, 0);

        // kama (x) ... la_sivyo ...
        let if_stmt = b.node(
            AST_KAMA,
            id_x,       // kushoto = condition
            ret1,       // kulia = then branch
            ret0,       // tiga = else branch
            NO_NODE, 0, 0,
        );

        let name_node = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, jina_kadirifu);
        let func_node = b.node(
            AST_KAZI,
            name_node,  // kushoto = name node
            param_x,    // kulia = first param
            if_stmt,    // tiga = body
            NO_NODE,    // nne
            ret_off,    // thamani = "N32"
            0,          // jina_off unused on func
        );

        let (aina, kushoto, kulia, tiga, nne, thamani, jina_off, pool, idadi) =
            b.finish(func_node);

        let module = lower(&aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, idadi);
        assert_eq!(module.functions.len(), 1);
        let f = &module.functions[0];
        assert_eq!(f.name, "kadirifu");

        // Thibitisha kimalizio cha BrCond kipo.
        let has_brcond = f.blocks.iter().any(|blk| {
            matches!(blk.terminator, Terminator::BrCond(_, _, _))
        });
        assert!(has_brcond, "if statement should produce a BrCond terminator");
    }

    #[test]
    fn test_while_loop() {
        // kazi chemsha(): W0 {
        //   wakati (1) { vunja; }
        // }
        let mut b = AstBuilder::new();
        let name_off = b.pool_name("chemsha");
        let ret_off = b.pool_name("W0");

        let lit1 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 1, 0);
        let brk = b.node(AST_VUNJA, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, 0);

        let while_stmt = b.node(
            AST_WAKATI,
            lit1,       // kushoto = condition
            NO_NODE,    // kulia
            brk,        // tiga = body
            NO_NODE, 0, 0,
        );

        let func_node = b.node(
            AST_KAZI,
            NO_NODE, NO_NODE,
            while_stmt, // tiga = body
            NO_NODE, ret_off, name_off,
        );

        let (aina, kushoto, kulia, tiga, nne, thamani, jina_off, pool, idadi) =
            b.finish(func_node);

        let module = lower(&aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, idadi);
        assert_eq!(module.functions.len(), 1);
        let f = &module.functions[0];

        // Inapaswa kuwa na BrCond kwa sharti la wakati.
        let has_brcond = f.blocks.iter().any(|blk| {
            matches!(blk.terminator, Terminator::BrCond(_, _, _))
        });
        assert!(has_brcond, "while loop should produce a BrCond terminator");

        // Inapaswa kuwa na kizuizi cha vunja (Br kwa exit).
        let has_br = f.blocks.iter().any(|blk| {
            matches!(blk.terminator, Terminator::Br(_))
        });
        assert!(has_br, "while/break should produce Br terminators");
    }

    #[test]
    fn test_arithmetic_expression() {
        // kazi ongeza(): N32 { rudisha 2 + 3; }
        let mut b = AstBuilder::new();
        let name_off = b.pool_name("ongeza");
        let ret_off = b.pool_name("N32");

        let lit2 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 2, 0);
        let lit3 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 3, 0);

        let add = b.node(AST_JUMLISHA, lit2, lit3, NO_NODE, NO_NODE, 0, 0);
        let ret_stmt = b.node(AST_RUDISHA, add, NO_NODE, NO_NODE, NO_NODE, 0, 0);

        let func_node = b.node(
            AST_KAZI,
            NO_NODE, NO_NODE,
            ret_stmt,   // tiga = body
            NO_NODE, ret_off, name_off,
        );

        let (aina, kushoto, kulia, tiga, nne, thamani, jina_off, pool, idadi) =
            b.finish(func_node);

        let module = lower(&aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, idadi);
        assert_eq!(module.functions.len(), 1);
        let f = &module.functions[0];

        // Inapaswa kuwa na amri ya Add mahali fulani.
        let has_add = f.blocks.iter().any(|blk| {
            blk.instructions.iter().any(|inst| matches!(inst, Instruction::Add(_, _)))
        });
        assert!(has_add, "2 + 3 should produce an Add instruction");
    }

    #[test]
    fn test_comparison_expression() {
        // kazi linganisha(): B1 { rudisha 5 == 3; }
        let mut b = AstBuilder::new();
        let name_off = b.pool_name("linganisha");
        let ret_off = b.pool_name("B1");

        let lit5 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 5, 0);
        let lit3 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 3, 0);

        let eq = b.node(AST_SAWA, lit5, lit3, NO_NODE, NO_NODE, 0, 0);
        let ret_stmt = b.node(AST_RUDISHA, eq, NO_NODE, NO_NODE, NO_NODE, 0, 0);

        let func_node = b.node(
            AST_KAZI,
            NO_NODE, NO_NODE,
            ret_stmt, NO_NODE, ret_off, name_off,
        );

        let (aina, kushoto, kulia, tiga, nne, thamani, jina_off, pool, idadi) =
            b.finish(func_node);

        let module = lower(&aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, idadi);
        assert_eq!(module.functions.len(), 1);
        let f = &module.functions[0];

        let has_eq = f.blocks.iter().any(|blk| {
            blk.instructions.iter().any(|inst| matches!(inst, Instruction::Eq(_, _)))
        });
        assert!(has_eq, "5 == 3 should produce an Eq instruction");
    }

    #[test]
    fn test_call_expression() {
        // kazi wita(): W0 { chapisha(); }
        let mut b = AstBuilder::new();
        let name_off = b.pool_name("wita");
        let ret_off = b.pool_name("W0");

        // Kitambulisho cha mpigiwa "chapisha"
        let jina_chapisha = b.pool_name("chapisha");
        let callee = b.node(
            AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0,
            jina_chapisha,
        );

        let call = b.node(AST_WITO, callee, NO_NODE, NO_NODE, NO_NODE, 0, 0);

        let func_node = b.node(
            AST_KAZI,
            NO_NODE, NO_NODE,
            call,       // tiga = body (the call stmt)
            NO_NODE, ret_off, name_off,
        );

        let (aina, kushoto, kulia, tiga, nne, thamani, jina_off, pool, idadi) =
            b.finish(func_node);

        let module = lower(&aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, idadi);
        assert_eq!(module.functions.len(), 1);
        let f = &module.functions[0];

        let has_call = f.blocks.iter().any(|blk| {
            blk.instructions.iter().any(|inst| matches!(inst, Instruction::Call(_, _)))
        });
        assert!(has_call, "call should produce a Call instruction");
    }

    #[test]
    fn test_short_circuit_and() {
        // kazi angalia(): B1 { rudisha 1 NA 0; }
        let mut b = AstBuilder::new();
        let name_off = b.pool_name("angalia");
        let ret_off = b.pool_name("B1");

        let lit1 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 1, 0);
        let lit0 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, 0);

        let and_expr = b.node(AST_NA, lit1, lit0, NO_NODE, NO_NODE, 0, 0);
        let ret_stmt = b.node(AST_RUDISHA, and_expr, NO_NODE, NO_NODE, NO_NODE, 0, 0);

        let func_node = b.node(
            AST_KAZI,
            NO_NODE, NO_NODE,
            ret_stmt, NO_NODE, ret_off, name_off,
        );

        let (aina, kushoto, kulia, tiga, nne, thamani, jina_off, pool, idadi) =
            b.finish(func_node);

        let module = lower(&aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, idadi);
        assert_eq!(module.functions.len(), 1);
        let f = &module.functions[0];

        // NA yenye fupi-hali hutoa Phi + BrCond + Ne.
        let has_phi = f.blocks.iter().any(|blk| {
            blk.instructions.iter().any(|inst| matches!(inst, Instruction::Phi(_, _)))
        });
        assert!(has_phi, "short-circuit AND should produce Phi instruction");

        let has_brcond = f.blocks.iter().any(|blk| {
            matches!(blk.terminator, Terminator::BrCond(_, _, _))
        });
        assert!(has_brcond, "short-circuit AND should produce BrCond");

        let has_ne = f.blocks.iter().any(|blk| {
            blk.instructions.iter().any(|inst| matches!(inst, Instruction::Ne(_, _)))
        });
        assert!(has_ne, "AND should convert operands to bool with Ne");
    }

    #[test]
    fn test_short_circuit_or() {
        // kazi ama(): B1 { rudisha 0 AU 1; }
        let mut b = AstBuilder::new();
        let name_off = b.pool_name("ama");
        let ret_off = b.pool_name("B1");

        let lit0 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, 0);
        let lit1 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 1, 0);

        let or_expr = b.node(AST_AU, lit0, lit1, NO_NODE, NO_NODE, 0, 0);
        let ret_stmt = b.node(AST_RUDISHA, or_expr, NO_NODE, NO_NODE, NO_NODE, 0, 0);

        let func_node = b.node(
            AST_KAZI,
            NO_NODE, NO_NODE,
            ret_stmt, NO_NODE, ret_off, name_off,
        );

        let (aina, kushoto, kulia, tiga, nne, thamani, jina_off, pool, idadi) =
            b.finish(func_node);

        let module = lower(&aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, idadi);
        assert_eq!(module.functions.len(), 1);
        let f = &module.functions[0];

        // AU yenye fupi-hali hutoa Phi + BrCond + Ne.
        let has_phi = f.blocks.iter().any(|blk| {
            blk.instructions.iter().any(|inst| matches!(inst, Instruction::Phi(_, _)))
        });
        assert!(has_phi, "short-circuit OR should produce Phi instruction");

        let has_brcond = f.blocks.iter().any(|blk| {
            matches!(blk.terminator, Terminator::BrCond(_, _, _))
        });
        assert!(has_brcond, "short-circuit OR should produce BrCond");
    }

    #[test]
    fn test_global_variable() {
        // Ulimwengu: N32 KIKOMO = 0;
        let mut b = AstBuilder::new();
        let jina_kikomo = b.pool_name("KIKOMO");
        // Aina iliyosimbwa kwa N32: familia=1, upana=32, mshale=0 → (1<<8)|32 = 288
        let n32_enc: i32 = 2080; // (1<<11)|(4<<3)|0

        let global = b.node(
            AST_TANGAZO_ULIMWENGU,
            NO_NODE,    // kushoto
            NO_NODE,    // kulia
            NO_NODE,    // tiga = no init
            NO_NODE,    // nne
            n32_enc,    // thamani = encoded N32 type
            jina_kikomo,// jina_off = "KIKOMO"
        );

        let (aina, kushoto, kulia, tiga, nne, thamani, jina_off, pool, idadi) =
            b.finish(global);

        let module = lower(&aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, idadi);
        assert_eq!(module.functions.len(), 0);

        // Inapaswa kuwa na angalau kigezo kimoja cha ulimwengu (kigezo cha mtumiaji; vigezo vya mfuatano vinaweza pia kuwepo).
        let user_global = module.globals.iter().find(|g| g.name == "KIKOMO");
        assert!(user_global.is_some(), "module should contain global 'KIKOMO'");
        let g = user_global.unwrap();
        assert!(!g.is_const);
        assert_eq!(g.bytes.len(), 4); // N32 = 4 bytes
    }

    #[test]
    fn test_string_literal() {
        // kazi salamu(): *N8 { rudisha "habari"; }
        let mut b = AstBuilder::new();
        let name_off = b.pool_name("salamu");
        let ret_off = b.pool_name("*N8"); // returning pointer to string

        // Halisi ya mfuatano "habari"
        let str_off = b.pool_bytes(b"habari");
        let str_node = b.node(AST_MFUATANO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, str_off);

        let ret_stmt = b.node(AST_RUDISHA, str_node, NO_NODE, NO_NODE, NO_NODE, 0, 0);

        let func_node = b.node(
            AST_KAZI,
            NO_NODE, NO_NODE,
            ret_stmt, NO_NODE, ret_off, name_off,
        );

        let (aina, kushoto, kulia, tiga, nne, thamani, jina_off, pool, idadi) =
            b.finish(func_node);

        let module = lower(&aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, idadi);
        assert_eq!(module.functions.len(), 1);
        let f = &module.functions[0];

        // Inapaswa kuwa na amri ya StringAddr.
        let has_string_addr = f.blocks.iter().any(|blk| {
            blk.instructions.iter().any(|inst| matches!(inst, Instruction::StringAddr(_)))
        });
        assert!(has_string_addr, "string literal should produce a StringAddr instruction");

        // Moduli inapaswa kuwa na kigezo cha mfuatano cha ulimwengu.
        let has_str_global = module.globals.iter().any(|g| g.is_const && g.bytes.starts_with(b"habari"));
        assert!(has_str_global, "module should contain a string global for 'habari'");
    }

    #[test]
    fn test_multiple_functions() {
        // Kazi mbili: a() na b()
        let mut b = AstBuilder::new();
        let w0_off = b.pool_name("W0");

        let jina_a2 = b.pool_name("a");
        let name_a = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, jina_a2);
        let func_a = b.node(
            AST_KAZI, name_a, NO_NODE, NO_NODE, NO_NODE,
            w0_off, 0,
        );
        let jina_b2 = b.pool_name("b");
        let name_b = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, jina_b2);
        let func_b = b.node(
            AST_KAZI, name_b, NO_NODE, NO_NODE, NO_NODE,
            w0_off, 0,
        );
        // Msururu a → b kama ndugu.
        b.nne[func_a as usize] = func_b;

        let (aina, kushoto, kulia, tiga, nne, thamani, jina_off, pool, idadi) =
            b.finish(func_a);

        let module = lower(&aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, idadi);
        assert_eq!(module.functions.len(), 2);
        assert_eq!(module.functions[0].name, "a");
        assert_eq!(module.functions[1].name, "b");
    }

    #[test]
    fn test_sret_classification() {
        // kazi pataNukta(): Nukta{x: D64, y: D64} { rudisha ... }
        // Nukta yenye sehemu 2 za float → Rudisha moja kwa moja (si sret).
        let mut b = AstBuilder::new();
        let name_off = b.pool_name("pataNukta");
        // Aina ya muundo yenye jina — si primitive, hivyo from_swa_type inarudisha None,
        // na tunarudia kwa IrType::Struct yenye sehemu tupu.
        // Muundo wenye sehemu 0 → Direct.
        let ret_off = b.pool_name("Nukta");

        let func_node = b.node(
            AST_KAZI, NO_NODE, NO_NODE, NO_NODE, NO_NODE,
            ret_off, name_off,
        );

        let (aina, kushoto, kulia, tiga, nne, thamani, jina_off, pool, idadi) =
            b.finish(func_node);

        let module = lower(&aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, idadi);
        assert_eq!(module.functions.len(), 1);
        let f = &module.functions[0];
        // Muundo tupu (sehemu 0) → Direct.
        assert_eq!(f.return_class, IrReturnClass::Direct);
        assert!(f.sret_value_id.is_none());
    }

    #[test]
    fn test_node_aina_no_node() {
        // Jaribio la kitengo kwa kisaidizi cha node_aina chenye kialamisho NO_NODE.
        let _b = AstBuilder::new();
        let lr = Lowerer {
            ast_aina: &[],
            ast_kushoto: &[],
            ast_kulia: &[],
            ast_tiga: &[],
            ast_nne: &[],
            ast_thamani: &[],
            ast_jina_off: &[],
            ast_pool: &[],
            functions: Vec::new(),
            globals: Vec::new(),
            types: Vec::new(),
            strings: Vec::new(),
            func: Function::new("test", IrType::Void, vec![]),
            scopes: Vec::new(),
            loops: Vec::new(),
            inst_counter: 0,
        values_initial_len: 0,
            block_counter: 0,
            pre_allocated_locals: std::collections::HashMap::new(),
            global_types: std::collections::HashMap::new(),
            sret_dest: None,
        };
        assert_eq!(lr.node_aina(NO_NODE), 0);
        assert_eq!(lr.node_aina(-1), 0);
        assert_eq!(lr.node_aina(-5), 0);
    }

    #[test]
    fn test_read_pool_name_empty() {
        let lr = Lowerer {
            ast_aina: &[],
            ast_kushoto: &[],
            ast_kulia: &[],
            ast_tiga: &[],
            ast_nne: &[],
            ast_thamani: &[],
            ast_jina_off: &[],
            ast_pool: b"hello\0world\0",
            functions: Vec::new(),
            globals: Vec::new(),
            types: Vec::new(),
            strings: Vec::new(),
            func: Function::new("test", IrType::Void, vec![]),
            scopes: Vec::new(),
            loops: Vec::new(),
            inst_counter: 0,
        values_initial_len: 0,
            block_counter: 0,
            pre_allocated_locals: std::collections::HashMap::new(),
            global_types: std::collections::HashMap::new(),
            sret_dest: None,
        };
        assert_eq!(lr.read_pool_name(0), "hello");
        assert_eq!(lr.read_pool_name(6), "world");
        assert_eq!(lr.read_pool_name(-1), "");
    }

    #[test]
    fn test_read_pool_bytes_length_prefixed() {
        // Urefu wa LE wa baiti 4 = 5, kisha baiti 5 "hello"
        let mut data = vec![5u8, 0, 0, 0];
        data.extend_from_slice(b"hello");
        let lr = Lowerer {
            ast_aina: &[],
            ast_kushoto: &[],
            ast_kulia: &[],
            ast_tiga: &[],
            ast_nne: &[],
            ast_thamani: &[],
            ast_jina_off: &[],
            ast_pool: &data,
            functions: Vec::new(),
            globals: Vec::new(),
            types: Vec::new(),
            strings: Vec::new(),
            func: Function::new("test", IrType::Void, vec![]),
            scopes: Vec::new(),
            loops: Vec::new(),
            inst_counter: 0,
        values_initial_len: 0,
            block_counter: 0,
            pre_allocated_locals: std::collections::HashMap::new(),
            global_types: std::collections::HashMap::new(),
            sret_dest: None,
        };
        let bytes = lr.read_pool_bytes(0);
        assert_eq!(bytes, b"hello");
    }

    #[test]
    fn test_read_pool_bytes_fallback_null_terminated() {
        // Hakuna kiambishi cha urefu (ncha-tupu tu).
        let data = b"habari\0extra";
        let lr = Lowerer {
            ast_aina: &[],
            ast_kushoto: &[],
            ast_kulia: &[],
            ast_tiga: &[],
            ast_nne: &[],
            ast_thamani: &[],
            ast_jina_off: &[],
            ast_pool: data.as_slice(),
            functions: Vec::new(),
            globals: Vec::new(),
            types: Vec::new(),
            strings: Vec::new(),
            func: Function::new("test", IrType::Void, vec![]),
            scopes: Vec::new(),
            loops: Vec::new(),
            inst_counter: 0,
        values_initial_len: 0,
            block_counter: 0,
            pre_allocated_locals: std::collections::HashMap::new(),
            global_types: std::collections::HashMap::new(),
            sret_dest: None,
        };
        // Dimbwi halina kiambishi cha urefu, kwa hivyo baiti 4 [104, 97, 98, 97] (= "haba")
        // lingefasiriwa kama urefu.  Urefu huo ni mkubwa, kwa hivyo linarudia
        // kwa ncha-tupu na kusoma kutoka kukabilisha 0.
        let bytes = lr.read_pool_bytes(0);
        // Inarudia kwa ncha-tupu: inasoma kutoka kukabilisha 0 hadi ncha-tupu kwenye faharasa 6.
        assert_eq!(bytes, b"habari");
    }

    #[test]
    fn test_while_with_if_return_inside() {
        // N32 jaribio(N64 n) {
        //     N64 i = 0;
        //     wakati (i < n) {
        //         kama (i == 0) { rudisha 1; }
        //         i = i + 1;
        //     }
        //     rudisha 0;
        // }
        let mut b = AstBuilder::new();
        // Aina zilizosimbwa
        let n32_enc: i32 = (1 << 11) | (4 << 3) | 0;   // N32
        let n64_enc: i32 = (1 << 11) | (5 << 3) | 0;   // N64
        let w0_enc: i32 = (5 << 11) | (0 << 3) | 0;     // W0 (haitumiki hapa)

        // Majina
        let jina_jaribio = b.pool_name("jaribio");
        let jina_n = b.pool_name("n");
        let jina_i = b.pool_name("i");
        let lit0 = b.pool_name("0");   // si halisi halisi, kwa dimbwi tu
        let lit1 = b.pool_name("1");

        // -- Kigezo n: N64 --
        let p_n = b.node(0, NO_NODE, NO_NODE, NO_NODE, NO_NODE, n64_enc, jina_n);

        // -- Mwili: N64 i = 0; wakati ...; rudisha 0; --
        // Kitambulisho i
        let id_i_off = jina_i;
        let name_i = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, id_i_off);

        // Halisi
        let lit_0 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, 0);
        let lit_1 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 1, 0);

        // Nodi ya aina N64 kwa tangazo
        let ty_n64 = b.node(0, NO_NODE, NO_NODE, NO_NODE, NO_NODE, n64_enc, 0);

        // Tangazo: N64 i = 0
        let decl = b.node(AST_TANGAZO, name_i, ty_n64, lit_0, NO_NODE, 0, 0);

        // -- mwili wa wakati --
        // Sharti: i < n
        let id_i_cond = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, id_i_off);
        let id_n_cond = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, jina_n);
        let cond = b.node(AST_CHINI, id_i_cond, id_n_cond, NO_NODE, NO_NODE, 0, 0);

        // -- mwili wa kama --
        // Sharti: i == 0
        let id_i_eq = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, id_i_off);
        let if_cond = b.node(AST_SAWA, id_i_eq, lit_0, NO_NODE, NO_NODE, 0, 0);
        // ndipo: rudisha 1
        let ret1 = b.node(AST_RUDISHA, lit_1, NO_NODE, NO_NODE, NO_NODE, 0, 0);
        // taarifa ya kama
        let if_stmt = b.node(AST_KAMA, if_cond, ret1, NO_NODE, NO_NODE, 0, 0);

        // i = i + 1 (ASIMILIA)
        let id_i_assign = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, id_i_off);
        let id_i_rhs = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, id_i_off);
        let add_expr = b.node(AST_JUMLISHA, id_i_rhs, lit_1, NO_NODE, NO_NODE, 0, 0);
        let assign = b.node(AST_ASIMILIA, id_i_assign, add_expr, NO_NODE, NO_NODE, 0, 0);

        // Msururu kama → ugawaji ndani ya mwili wa wakati
        b.nne[if_stmt as usize] = assign;

        // wakati (sharti) { mwili }
        let while_node = b.node(AST_WAKATI, cond, NO_NODE, if_stmt, NO_NODE, 0, 0);

        // rudisha 0
        let ret0 = b.node(AST_RUDISHA, lit_0, NO_NODE, NO_NODE, NO_NODE, 0, 0);

        // Msururu tangazo → wakati → ret0
        b.nne[decl as usize] = while_node;
        b.nne[while_node as usize] = ret0;

        // Kazi
        let name_f = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, jina_jaribio);
        let func = b.node(AST_KAZI, name_f, p_n, decl, NO_NODE, n32_enc, 0);

        let (aina, kushoto, kulia, tiga, nne, thamani, jina_off, pool, idadi) = b.finish(func);
        let module = lower(&aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, idadi);

        assert_eq!(module.functions.len(), 1);
        let f = &module.functions[0];
        assert_eq!(f.name, "jaribio");
        assert_eq!(f.return_ty, IrType::I32);

        // Thibitisha hakuna RetVoid katika kazi isiyo tupu.
        let retvoid_blocks: Vec<_> = f.blocks.iter()
            .filter(|blk| matches!(blk.terminator, Terminator::RetVoid))
            .map(|blk| blk.label.as_str())
            .collect();
        assert!(retvoid_blocks.is_empty(),
            "non-void function should not have RetVoid blocks, found: {:?}", retvoid_blocks);

        // Thibitisha ina vimalizio vya Ret vyenye thamani.
        let has_ret_val = f.blocks.iter().any(|blk| {
            matches!(blk.terminator, Terminator::Ret(_))
        });
        assert!(has_ret_val, "function should have at least one Ret(value)");
    }
}
