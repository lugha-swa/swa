//! Uwakilishi wa kati wa Swa — amri, bloku, kazi, na
//! chombo cha moduli.
//!
//! ## Aina za kiwango cha juu
//!
//! | Aina             | Jukumu                                      |
//! |------------------|---------------------------------------------|
//! | `Module`         | Kipashio kizima cha ukusanyaji              |
//! | `Function`       | Kazi moja ya Swa (au kianzishi cha ulimwengu)|
//! | `IrBlock`        | Bloku ya msingi ndani ya kazi               |
//! | `Instruction`    | Operesheni moja ya SSA yenye viendeshwa     |
//! | `Const`          | Thamani thabiti ya wakati wa ukusanyaji     |
//! | `Terminator`     | Uhamisho wa udhibiti unaoishia bloku        |
//! | `IrBuilder`      | Mjenzi wa urahisi kwa kujaza bloku          |
//! | `IrReturnClass`  | Jinsi struct-return inavyopitishwa (direct/sret)|

pub mod types;
pub mod lower;

use std::collections::HashMap;
use types::IrType;

// ---------------------------------------------------------------------------
// Vifungashio vya Newtype
// ---------------------------------------------------------------------------

/// Kitambulishi kisicho wazi cha bloku ya msingi ndani ya kazi.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub usize);

/// Kitambulishi kisicho wazi cha thamani ya SSA (matokeo ya amri, kigezo, au
/// thabiti) ndani ya kazi.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(pub usize);

// ---------------------------------------------------------------------------
// FloatWrapper — inaturuhusu kuhifadhi `f64` ndani ya `Const` ingawa `f64` si
// `Eq` / `Hash` kwa chaguo-msingi.
// ---------------------------------------------------------------------------

/// Newtype juu ya `f64` inayotoa `Eq` na `Hash` kwa kulinganisha bitwise
/// ya uwakilishi wa msingi wa IEEE-754.
///
/// Uwakilishi mbili za NaN **zinachukuliwa kuwa sawa** (zinalingana kwenye
/// biti zake), ambalo ni tabia inayotakikana kwa utambulisho thabiti wa IR.
#[derive(Debug, Clone, Copy)]
pub struct FloatWrapper(pub f64);

impl FloatWrapper {
    pub fn to_bits(self) -> u64 {
        self.0.to_bits()
    }

    #[allow(dead_code)]
    pub fn from_bits(bits: u64) -> Self {
        Self(f64::from_bits(bits))
    }
}

impl PartialEq for FloatWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for FloatWrapper {}

impl std::hash::Hash for FloatWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl std::fmt::Display for FloatWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// Thabiti (Const)
// ---------------------------------------------------------------------------

/// Thamani thabiti ya wakati wa ukusanyaji.
///
/// Hizi huwekwa ndani ya `Function.values` na kurejelewa na
/// `ValueId` katika amri.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Const {
    /// Thabiti kamili yenye ishara (iN).  Imewekwa kama `i128` kufunika upana wote.
    Int(i128),
    /// Thabiti kamili bila ishara (uN).  Imewekwa kama `u128`.
    Uint(u128),
    /// Thabiti ya buliani.
    Bool(bool),
    /// Kielekezi batili chenye aina.
    NullPtr,
    /// Kianzishi sifuri kwa aina yoyote.
    Zero,
    /// Thabiti ya namba ya IEEE-754 yenye kuelea.
    Float(FloatWrapper),
    /// Maandishi halisi (kwa amri za `StringAddr`).
    String(String),
}

// ---------------------------------------------------------------------------
// Amri (Instruction)
// ---------------------------------------------------------------------------

/// Amri moja ya SSA ndani ya bloku ya msingi.
///
/// Kila amri hutoa thamani moja haswa (inayotambuliwa na `ValueId`).
/// Amri zinazoathiri kumbukumbu (`Store`, `HeapFree`, ...) pia hutoa
/// thamani — mara nyingi `void` — ili ziweze kutoshea muundo wa SSA kwa usawa.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    // -- hesabu za kamili -------------------------------------------------
    Add(ValueId, ValueId),
    Sub(ValueId, ValueId),
    Mul(ValueId, ValueId),
    DivS(ValueId, ValueId),
    DivU(ValueId, ValueId),
    RemS(ValueId, ValueId),
    RemU(ValueId, ValueId),

    // -- hesabu za kuelea ------------------------------------------
    FAdd(ValueId, ValueId),
    FSub(ValueId, ValueId),
    FMul(ValueId, ValueId),
    FDiv(ValueId, ValueId),
    FNeg(ValueId),

    // -- bitwise (ya biti) --------------------------------------------------
    And(ValueId, ValueId),
    Or(ValueId, ValueId),
    Xor(ValueId, ValueId),
    Shl(ValueId, ValueId),
    ShrS(ValueId, ValueId),
    ShrU(ValueId, ValueId),

    // -- ulinganisho kamili ------------------------------------------------
    Eq(ValueId, ValueId),
    Ne(ValueId, ValueId),
    LtS(ValueId, ValueId),
    LtU(ValueId, ValueId),
    LeS(ValueId, ValueId),
    LeU(ValueId, ValueId),
    GtS(ValueId, ValueId),
    GtU(ValueId, ValueId),
    GeS(ValueId, ValueId),
    GeU(ValueId, ValueId),

    // -- ulinganisho wa kuelea -----------------------------------------
    Feq(ValueId, ValueId),
    Fne(ValueId, ValueId),
    Flt(ValueId, ValueId),
    Fle(ValueId, ValueId),
    Fgt(ValueId, ValueId),
    Fge(ValueId, ValueId),

    // -- ubadilishaji wa aina ---------------------------------------------------
    Trunc(ValueId, IrType),
    Zext(ValueId, IrType),
    Sext(ValueId, IrType),
    FpTrunc(ValueId, IrType),
    FpExt(ValueId, IrType),
    FpToSi(ValueId, IrType),
    FpToUi(ValueId, IrType),
    SiToFp(ValueId, IrType),
    UiToFp(ValueId, IrType),
    Bitcast(ValueId, IrType),

    // -- kumbukumbu -------------------------------------------------------------
    Alloca(IrType),
    Const(Const),                       // weka thabiti kama thamani
    Load(IrType, ValueId),              // (pointee_type, ptr)
    Store(ValueId, ValueId),            // (value, ptr)
    StoreTyped(ValueId, ValueId, IrType), // (value, ptr, stored_type)
    MemCopy(ValueId, ValueId, u64), // (dest_ptr, src_ptr, size_bytes)

    // -- rundo ---------------------------------------------------------------
    HeapAlloc(ValueId), // ukubwa kwa ka → ptr
    HeapFree(ValueId),  // ptr

    // -- arena (ugawaji wa kimaeneo) -----------------------------------
    ArenaCreate(ValueId), // uwezo kwa ka → kishikio cha arena
    ArenaAlloc(ValueId, ValueId), // (arena, ukubwa) → ptr
    ArenaFree(ValueId),   // kishikio cha arena

    // -- anwani-ya ---------------------------------------------------------
    FnAddr(String),             // jina la kazi → kielekezi cha kazi
    GlobalAddr(String),         // jina la ulimwengu → kielekezi cha ulimwengu
    StringAddr(String),         // maandishi thabiti → kielekezi cha ka

    // -- hesabu ya kielekezi -------------------------------------------------
    Gep(ValueId, Vec<ValueId>),                  // getelementptr (msingi, fahirisi)
    FieldAddr(ValueId, usize, Option<IrType>),   // anwani ya sehemu ya struct (ptr, field_index, struct_type?)

    // -- jumlisha ----------------------------------------------------------
    BuildStruct(Vec<ValueId>),
    ExtractField(ValueId, usize),

    // -- chagua (pembe) ---------------------------------------------------
    Select(ValueId, ValueId, ValueId),  // (sharti, thamani_kweli, thamani_uwongo)

    // -- phi (muunganiko wa SSA) ----------------------------------------------------
    /// Nodi ya Phi: huunganisha thamani kutoka kwa bloku tofauti za watangulizi.
    /// `(result_type, [(value, predecessor_block), ...])`
    Phi(IrType, Vec<(ValueId, BlockId)>),

    // -- simu --------------------------------------------------------------
    Call(String, Vec<ValueId>),         // simu ya moja kwa moja
    CallIndirect(ValueId, Vec<ValueId>), // simu isiyo ya moja kwa moja
}

// ---------------------------------------------------------------------------
// Mwishishaji (Terminator)
// ---------------------------------------------------------------------------

/// Kila bloku ya msingi inaishia na mwishishaji mmoja haswa.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Terminator {
    /// Tawi lisilo na sharti hadi `BlockId`.
    Br(BlockId),
    /// Tawi lenye sharti: (sharti, bloku_kweli, bloku_uwongo).
    BrCond(ValueId, BlockId, BlockId),
    /// Rudisha thamani kutoka kwa kazi.
    Ret(ValueId),
    /// Rudisha kutoka kwa kazi ya void.
    RetVoid,
    /// Usambazaji wa njia nyingi: (scrutinee, bloku_chaucho, mikono).
    Switch(ValueId, BlockId, Vec<(ValueId, BlockId)>),
}

// ---------------------------------------------------------------------------
// Bloku ya IR (IrBlock)
// ---------------------------------------------------------------------------

/// Bloku moja ya msingi: mfuatano wa amri zisizo za mwishishaji zikifuatiwa
/// na mwishishaji mmoja haswa.
#[derive(Debug, Clone)]
pub struct IrBlock {
    /// Lebo ya bloku (inatumika katika malengo ya tawi na utatuzi).
    pub label: String,
    /// Amri zisizo za mwishishaji kwa mpangilio.
    pub instructions: Vec<Instruction>,
    /// Mwishishaji wa mwisho wa bloku.
    pub terminator: Terminator,
}

impl IrBlock {
    pub fn new(label: impl Into<String>, terminator: Terminator) -> Self {
        Self {
            label: label.into(),
            instructions: Vec::new(),
            terminator,
        }
    }

    pub fn push(&mut self, inst: Instruction) {
        self.instructions.push(inst);
    }

    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }
}

// ---------------------------------------------------------------------------
// Kigezo cha IR (IrGlobal)
// ---------------------------------------------------------------------------

/// Kigezo cha kimataifa cha kiwango cha moduli.
#[derive(Debug, Clone)]
pub struct IrGlobal {
    pub name: String,
    pub bytes: Vec<u8>,
    /// Kama kigezo hiki ni thabiti (soma-tu).
    pub is_const: bool,
    /// Aina ya Swa IR (inatumika kujenga aina sahihi ya LLVM).
    pub ty: IrType,
}

// ---------------------------------------------------------------------------
// Kazi (Function)
// ---------------------------------------------------------------------------

/// Ufafanuzi kamili wa kazi wa IR.
#[derive(Debug, Clone)]
pub struct Function {
    /// Jina la kazi (alama ya kiunganishi).
    pub name: String,
    /// Aina ya kurejesha (kiwango cha LLVM, baada ya kuandikwa upya kwa sret).
    pub return_ty: IrType,
    /// Aina asili ya kurejesha kabla ya kuandikwa upya kwa sret (kwa ABI/utatuzi).
    pub source_return_ty: IrType,
    /// Jinsi thamani ya kurejesha inavyopitishwa (Direct au HiddenPtr/sret).
    pub return_class: IrReturnClass,
    /// Vigezo rasmi: (jina, aina).
    pub params: Vec<(String, IrType)>,
    /// Bloku zote za msingi zinazomilikiwa na kazi hii.
    pub blocks: Vec<IrBlock>,
    /// Thamani thabiti zilizosawazishwa, zikiorodheshwa na `ValueId`.
    pub values: Vec<Const>,
    /// `BlockId` ya bloku ya kuingilia (ya kwanza).
    pub entry: BlockId,
    /// Kama kazi inatumia C ABI (extern "C").
    pub c_abi: bool,
    /// Kama kazi ni variadic.
    pub variadic: bool,
    /// Wakati `return_class == HiddenPtr`, `ValueId` ya kielekezi cha sret
    /// (ambacho ni kigezo cha kwanza cha fiche).
    pub sret_value_id: Option<ValueId>,
}

impl Function {
    pub fn new(name: impl Into<String>, return_ty: IrType, params: Vec<(String, IrType)>) -> Self {
        let return_class = IrReturnClass::Direct;
        Self {
            name: name.into(),
            return_ty: return_ty.clone(),
            source_return_ty: return_ty,
            return_class,
            params,
            blocks: Vec::new(),
            values: Vec::new(),
            entry: BlockId(0),
            c_abi: false,
            variadic: false,
            sret_value_id: None,
        }
    }

    /// Ongeza bloku na urudishe `BlockId` yake.
    pub fn push_block(&mut self, block: IrBlock) -> BlockId {
        let id = BlockId(self.blocks.len());
        self.blocks.push(block);
        id
    }

    /// Weka thabiti ndani na urudishe `ValueId` inayoweza kutumika kama
    /// kiendeshwa.  Uondoaji-nakala unafanywa ili kuepuka viingilio maradufu.
    ///
    /// ValueIds kwa thabiti huanza baada ya nafasi za vigezo:
    ///   ValueId(params.len() + position_in_values)
    pub fn intern_const(&mut self, c: Const) -> ValueId {
        if let Some(pos) = self.values.iter().position(|v| *v == c) {
            return ValueId(self.params.len() + pos);
        }
        let pos = self.values.len();
        let id = ValueId(self.params.len() + pos);
        self.values.push(c);
        id
    }

    /// Idadi ya bloku.
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }
}

// ---------------------------------------------------------------------------
// Moduli (Module)
// ---------------------------------------------------------------------------

/// Kipashio cha ukusanyaji cha kiwango cha juu.
#[derive(Debug, Clone)]
pub struct Module {
    /// Jina la moduli / kitengo cha kutafsiri.
    pub name: String,
    /// Ufafanuzi wa aina (jina, IrType).  Hujazwa wakati wa ushushaji.
    pub types: Vec<(String, IrType)>,
    /// Vigezo vya kiwango cha moduli.
    pub globals: Vec<IrGlobal>,
    /// Ufafanuzi wa kazi.
    pub functions: Vec<Function>,
}

impl Module {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            types: Vec::new(),
            globals: Vec::new(),
            functions: Vec::new(),
        }
    }

    pub fn push_function(&mut self, func: Function) {
        self.functions.push(func);
    }

    pub fn push_global(&mut self, global: IrGlobal) {
        self.globals.push(global);
    }

    pub fn push_type(&mut self, name: impl Into<String>, ty: IrType) {
        self.types.push((name.into(), ty));
    }
}

// ---------------------------------------------------------------------------
// Darasa la Kurejesha IR (IrReturnClass)
// ---------------------------------------------------------------------------

/// Jinsi thamani ya kurejesha ya kazi inavyopitishwa katika kiwango cha ABI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrReturnClass {
    /// Rudisha katika rejista (hadi sehemu 2).
    Direct,
    /// Rudisha kupitia kielekezi kilichofichwa (sret) — mpigaji anagawa, mpigiwa anaandika.
    HiddenPtr,
}

// ---------------------------------------------------------------------------
// IrBuilder — API ya urahisi kwa kujenga IR ndani ya kazi
// ---------------------------------------------------------------------------

/// Mjenzi wa IR wa nyongeza anayeongeza amri kwenye bloku ya *sasa* ya
/// kazi.
pub struct IrBuilder<'f> {
    func: &'f mut Function,
    /// Bloku ambayo `push` inaongeza.
    current: BlockId,
    /// Ramani kutoka (Instruction, operand-ids) → `ValueId` kwa CSE ya ndani.
    /// Funguo ni tuples ili tuweze kutumia tena misemo rahisi.
    cache: HashMap<Instruction, ValueId>,
}

impl<'f> IrBuilder<'f> {
    /// Unda mjenzi uliowekwa kwenye `entry_block` ya kazi iliyotolewa.
    /// Bloku ya kuingilia lazima iwepo tayari (kwa kawaida imeundwa na mpigaji
    /// kwa `Function::push_block`).
    pub fn new(func: &'f mut Function, entry_block: BlockId) -> Self {
        Self {
            func,
            current: entry_block,
            cache: HashMap::new(),
        }
    }

    /// Badilisha mjenzi kuongeza kwenye bloku tofauti.
    pub fn switch_to_block(&mut self, block: BlockId) {
        self.current = block;
        self.cache.clear(); // CSE ni ya ndani ya bloku
    }

    /// Rudi `BlockId` ya sasa.
    pub fn current_block(&self) -> BlockId {
        self.current
    }

    /// Pata kazi ya msingi (soma-tu).
    #[allow(dead_code)]
    pub fn func(&self) -> &Function {
        self.func
    }

    /// Pata kazi ya msingi (inayobadilika).
    #[allow(dead_code)]
    pub fn func_mut(&mut self) -> &mut Function {
        self.func
    }

    /// Weka thabiti ndani na urudishe `ValueId`.
    pub fn const_val(&mut self, c: Const) -> ValueId {
        self.func.intern_const(c)
    }

    /// Ongeza amri kwenye bloku ya sasa na urudishe `ValueId` mpya
    /// kwa matokeo yake.  Ikiwa amri hiyo hiyo tayari ipo kwenye
    /// kashe ya bloku ya sasa, `ValueId` iliyopo inarudishwa (uondoaji
    /// wa misemo ya kawaida ya ndani).
    fn emit(&mut self, inst: Instruction) -> ValueId {
        if let Some(&existing) = self.cache.get(&inst) {
            return existing;
        }

        let block = &mut self.func.blocks[self.current.0];
        block.push(inst.clone());

        // Thamani zinazozalishwa na amri hupewa nambari kwa mfuatano *baada ya*
        // thamani thabiti ili safu za ValueId zisigongane.
        let id = ValueId(self.func.values.len() + block.len() - 1);
        self.cache.insert(inst, id);
        id
    }

    // -- vifaa vya utoaji vya urahisi kwa kila op -------------------------------

    pub fn build_add(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::Add(lhs, rhs))
    }
    pub fn build_sub(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::Sub(lhs, rhs))
    }
    pub fn build_mul(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::Mul(lhs, rhs))
    }
    pub fn build_div_s(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::DivS(lhs, rhs))
    }
    pub fn build_div_u(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::DivU(lhs, rhs))
    }
    pub fn build_rem_s(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::RemS(lhs, rhs))
    }
    pub fn build_rem_u(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::RemU(lhs, rhs))
    }

    pub fn build_fadd(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::FAdd(lhs, rhs))
    }
    pub fn build_fsub(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::FSub(lhs, rhs))
    }
    pub fn build_fmul(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::FMul(lhs, rhs))
    }
    pub fn build_fdiv(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::FDiv(lhs, rhs))
    }
    pub fn build_fneg(&mut self, val: ValueId) -> ValueId {
        self.emit(Instruction::FNeg(val))
    }

    pub fn build_and(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::And(lhs, rhs))
    }
    pub fn build_or(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::Or(lhs, rhs))
    }
    pub fn build_xor(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::Xor(lhs, rhs))
    }
    pub fn build_shl(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::Shl(lhs, rhs))
    }
    pub fn build_shr_s(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::ShrS(lhs, rhs))
    }
    pub fn build_shr_u(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::ShrU(lhs, rhs))
    }

    pub fn build_eq(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::Eq(lhs, rhs))
    }
    pub fn build_ne(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::Ne(lhs, rhs))
    }
    pub fn build_lt_s(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::LtS(lhs, rhs))
    }
    pub fn build_lt_u(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::LtU(lhs, rhs))
    }
    pub fn build_le_s(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::LeS(lhs, rhs))
    }
    pub fn build_le_u(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::LeU(lhs, rhs))
    }
    pub fn build_gt_s(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::GtS(lhs, rhs))
    }
    pub fn build_gt_u(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::GtU(lhs, rhs))
    }
    pub fn build_ge_s(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::GeS(lhs, rhs))
    }
    pub fn build_ge_u(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::GeU(lhs, rhs))
    }

    pub fn build_feq(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::Feq(lhs, rhs))
    }
    pub fn build_fne(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::Fne(lhs, rhs))
    }
    pub fn build_flt(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::Flt(lhs, rhs))
    }
    pub fn build_fle(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::Fle(lhs, rhs))
    }
    pub fn build_fgt(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::Fgt(lhs, rhs))
    }
    pub fn build_fge(&mut self, lhs: ValueId, rhs: ValueId) -> ValueId {
        self.emit(Instruction::Fge(lhs, rhs))
    }

    pub fn build_trunc(&mut self, val: ValueId, target: IrType) -> ValueId {
        self.emit(Instruction::Trunc(val, target))
    }
    pub fn build_zext(&mut self, val: ValueId, target: IrType) -> ValueId {
        self.emit(Instruction::Zext(val, target))
    }
    pub fn build_sext(&mut self, val: ValueId, target: IrType) -> ValueId {
        self.emit(Instruction::Sext(val, target))
    }
    pub fn build_fptrunc(&mut self, val: ValueId, target: IrType) -> ValueId {
        self.emit(Instruction::FpTrunc(val, target))
    }
    pub fn build_fpext(&mut self, val: ValueId, target: IrType) -> ValueId {
        self.emit(Instruction::FpExt(val, target))
    }
    pub fn build_fptosi(&mut self, val: ValueId, target: IrType) -> ValueId {
        self.emit(Instruction::FpToSi(val, target))
    }
    pub fn build_fptoui(&mut self, val: ValueId, target: IrType) -> ValueId {
        self.emit(Instruction::FpToUi(val, target))
    }
    pub fn build_sitofp(&mut self, val: ValueId, target: IrType) -> ValueId {
        self.emit(Instruction::SiToFp(val, target))
    }
    pub fn build_uitofp(&mut self, val: ValueId, target: IrType) -> ValueId {
        self.emit(Instruction::UiToFp(val, target))
    }
    pub fn build_bitcast(&mut self, val: ValueId, target: IrType) -> ValueId {
        self.emit(Instruction::Bitcast(val, target))
    }

    pub fn build_alloca(&mut self, ty: IrType) -> ValueId {
        self.emit(Instruction::Alloca(ty))
    }
    pub fn build_load(&mut self, pointee_ty: IrType, ptr: ValueId) -> ValueId {
        self.emit(Instruction::Load(pointee_ty, ptr))
    }
    pub fn build_store(&mut self, val: ValueId, ptr: ValueId) -> ValueId {
        self.emit(Instruction::Store(val, ptr))
    }

    pub fn build_heap_alloc(&mut self, size: ValueId) -> ValueId {
        self.emit(Instruction::HeapAlloc(size))
    }
    pub fn build_heap_free(&mut self, ptr: ValueId) -> ValueId {
        self.emit(Instruction::HeapFree(ptr))
    }

    pub fn build_arena_create(&mut self, capacity: ValueId) -> ValueId {
        self.emit(Instruction::ArenaCreate(capacity))
    }
    pub fn build_arena_alloc(&mut self, arena: ValueId, size: ValueId) -> ValueId {
        self.emit(Instruction::ArenaAlloc(arena, size))
    }
    pub fn build_arena_free(&mut self, arena: ValueId) -> ValueId {
        self.emit(Instruction::ArenaFree(arena))
    }

    pub fn build_fn_addr(&mut self, name: impl Into<String>) -> ValueId {
        self.emit(Instruction::FnAddr(name.into()))
    }
    pub fn build_global_addr(&mut self, name: impl Into<String>) -> ValueId {
        self.emit(Instruction::GlobalAddr(name.into()))
    }
    pub fn build_string_addr(&mut self, s: impl Into<String>) -> ValueId {
        self.emit(Instruction::StringAddr(s.into()))
    }

    pub fn build_gep(&mut self, base: ValueId, indices: Vec<ValueId>) -> ValueId {
        self.emit(Instruction::Gep(base, indices))
    }
    pub fn build_field_addr(&mut self, base: ValueId, field_idx: usize, struct_ty: Option<IrType>) -> ValueId {
        self.emit(Instruction::FieldAddr(base, field_idx, struct_ty))
    }

    pub fn build_struct(&mut self, fields: Vec<ValueId>) -> ValueId {
        self.emit(Instruction::BuildStruct(fields))
    }
    pub fn build_extract_field(&mut self, val: ValueId, field_idx: usize) -> ValueId {
        self.emit(Instruction::ExtractField(val, field_idx))
    }

    /// Jenga nodi ya Phi inayounganisha thamani kutoka kwa bloku nyingi za watangulizi.
    pub fn build_phi(&mut self, result_ty: IrType, incoming: Vec<(ValueId, BlockId)>) -> ValueId {
        self.emit(Instruction::Phi(result_ty, incoming))
    }

    pub fn build_call(&mut self, callee: impl Into<String>, args: Vec<ValueId>) -> ValueId {
        self.emit(Instruction::Call(callee.into(), args))
    }
    pub fn build_call_indirect(&mut self, fn_ptr: ValueId, args: Vec<ValueId>) -> ValueId {
        self.emit(Instruction::CallIndirect(fn_ptr, args))
    }

    // -- vifaa vya mwishishaji -------------------------------------------------

    /// Weka mwishishaji wa bloku ya sasa (ukibadilisha yoyote ya awali).
    pub fn set_terminator(&mut self, term: Terminator) {
        self.func.blocks[self.current.0].terminator = term;
    }

    pub fn build_br(&mut self, target: BlockId) {
        self.set_terminator(Terminator::Br(target));
    }

    pub fn build_br_cond(&mut self, cond: ValueId, then_block: BlockId, else_block: BlockId) {
        self.set_terminator(Terminator::BrCond(cond, then_block, else_block));
    }

    pub fn build_ret(&mut self, val: ValueId) {
        self.set_terminator(Terminator::Ret(val));
    }

    pub fn build_ret_void(&mut self) {
        self.set_terminator(Terminator::RetVoid);
    }

    pub fn build_switch(
        &mut self,
        scrutinee: ValueId,
        default: BlockId,
        arms: Vec<(ValueId, BlockId)>,
    ) {
        self.set_terminator(Terminator::Switch(scrutinee, default, arms));
    }
}

// ---------------------------------------------------------------------------
// Majaribio
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Msaidizi: unda kazi ndogo inayorudisha void yenye bloku moja tupu.
    fn dummy_func() -> Function {
        let mut f = Function::new("test", IrType::Void, vec![]);
        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        // entry ni daima BlockId(0) hapa, lakini hatutegemei hilo.
        f.entry = entry;
        f.c_abi = false;
        f.variadic = false;
        f.sret_value_id = None;
        f
    }

    #[test]
    fn test_function_creation() {
        let f = Function::new(
            "fanya_kitu",
            IrType::I32,
            vec![("a".into(), IrType::I32), ("b".into(), IrType::F64)],
        );
        assert_eq!(f.name, "fanya_kitu");
        assert_eq!(f.return_ty, IrType::I32);
        assert_eq!(f.params.len(), 2);
        assert!(f.blocks.is_empty());
    }

    #[test]
    fn test_block_push() {
        let mut f = dummy_func();
        // Ongeza bloku ya pili
        let b1 = f.push_block(IrBlock::new("then", Terminator::RetVoid));
        assert_eq!(f.block_count(), 2);
        assert_eq!(b1, BlockId(1));
    }

    #[test]
    fn test_const_intern() {
        let mut f = dummy_func();
        let c1 = f.intern_const(Const::Int(42));
        let c2 = f.intern_const(Const::Int(-1));
        assert_eq!(c1, ValueId(0));
        assert_eq!(c2, ValueId(1));
        assert_eq!(f.values.len(), 2);
        assert_eq!(f.values[0], Const::Int(42));
        assert_eq!(f.values[1], Const::Int(-1));
    }

    #[test]
    fn test_builder_emit_add() {
        let mut f = dummy_func();
        let entry = f.entry;
        let mut b = IrBuilder::new(&mut f, entry);

        let one = b.const_val(Const::Int(1));
        let two = b.const_val(Const::Int(2));
        let sum = b.build_add(one, two);

        // Thamani kutoka kwa thabiti
        assert_eq!(one, ValueId(0));
        assert_eq!(two, ValueId(1));
        // sum ni amri ya kwanza
        assert_ne!(sum, one);
        assert_ne!(sum, two);

        let block = &b.func.blocks[entry.0];
        assert_eq!(block.len(), 1);
        assert_eq!(block.instructions[0], Instruction::Add(ValueId(0), ValueId(1)));
    }

    #[test]
    fn test_builder_cse() {
        let mut f = dummy_func();
        let entry = f.entry;
        let mut b = IrBuilder::new(&mut f, entry);

        let x = b.const_val(Const::Int(10));
        let y = b.const_val(Const::Int(20));

        let a = b.build_add(x, y);
        let b2 = b.build_add(x, y); // amri sawa → inapaswa kuwa CSE
        assert_eq!(a, b2, "CSE should return the same ValueId");
        assert_eq!(b.func.blocks[entry.0].len(), 1);
    }

    #[test]
    fn test_builder_terminator() {
        let mut f = dummy_func();
        let entry = f.entry;
        // Bloku ya kuingilia kwa sasa ina RetVoid.
        assert_eq!(f.blocks[entry.0].terminator, Terminator::RetVoid);

        let mut b = IrBuilder::new(&mut f, entry);
        let ret_val = b.const_val(Const::Int(0));
        b.build_ret(ret_val);

        assert_eq!(b.func.blocks[entry.0].terminator, Terminator::Ret(ValueId(0)));
    }

    #[test]
    fn test_module_construction() {
        let mut m = Module::new("moduli_ya_mtihani");
        m.push_type("Nukta", IrType::Struct {
            name: "Nukta".into(),
            fields: vec![("x".into(), IrType::F64), ("y".into(), IrType::F64)],
        });
        m.push_global(IrGlobal {
            name: "salamu".into(),
            bytes: b"habari\0".to_vec(),
            is_const: true,
            ty: IrType::Array { element: Box::new(IrType::I8), count: 7 },
        });
        m.push_function(Function::new("kuu", IrType::I32, vec![]));

        assert_eq!(m.name, "moduli_ya_mtihani");
        assert_eq!(m.types.len(), 1);
        assert_eq!(m.globals.len(), 1);
        assert_eq!(m.functions.len(), 1);
    }

    #[test]
    fn test_block_instructions_order() {
        let mut blk = IrBlock::new("l0", Terminator::RetVoid);
        blk.push(Instruction::Alloca(IrType::I32));
        blk.push(Instruction::Load(IrType::I32, ValueId(0)));

        assert_eq!(blk.len(), 2);
        assert!(!blk.is_empty());
    }

    #[test]
    fn test_float_wrapper_eq() {
        let a = FloatWrapper(3.14);
        let b = FloatWrapper(3.14);
        let c = FloatWrapper(2.71);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_float_wrapper_nan_eq() {
        let nan_a = FloatWrapper(f64::NAN);
        let nan_b = FloatWrapper(f64::NAN);
        // Kuwakilishwa kwa NaN kwa bitwise kunapaswa kuwa sawa
        assert_eq!(nan_a, nan_b);
    }
}
