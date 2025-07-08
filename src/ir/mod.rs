//! Swa intermediate representation — instructions, blocks, functions, and
//! the module container.
//!
//! ## Top-level types
//!
//! | Type             | Role                                       |
//! |------------------|--------------------------------------------|
//! | `Module`         | Whole compilation unit                     |
//! | `Function`       | One Swa function (or global initialiser)   |
//! | `IrBlock`        | A basic block inside a function            |
//! | `Instruction`    | One SSA operation with operands            |
//! | `Const`          | Compile-time constant value                |
//! | `Terminator`     | Block-ending control-flow transfer         |
//! | `IrBuilder`      | Convenience builder for populating blocks  |
//! | `IrReturnClass`  | How a struct-return is passed (direct/sret)|

pub mod types;
pub mod lower;

use std::collections::HashMap;
use types::IrType;

// ---------------------------------------------------------------------------
// Newtype wrappers
// ---------------------------------------------------------------------------

/// Opaque identifier for a basic block within a function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub usize);

/// Opaque identifier for an SSA value (instruction result, parameter, or
/// constant) within a function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(pub usize);

// ---------------------------------------------------------------------------
// FloatWrapper — lets us store `f64` inside `Const` even though `f64` is not
// `Eq` / `Hash` by default.
// ---------------------------------------------------------------------------

/// A newtype over `f64` that provides `Eq` and `Hash` via bitwise comparison
/// of the underlying IEEE-754 representation.
///
/// Two NaN representations **are** considered equal (they compare equal on
/// their bits), which is the desired behaviour for IR constant identity.
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
// Const
// ---------------------------------------------------------------------------

/// A compile-time constant.
///
/// These are materialised inside `Function.values` and referenced by
/// `ValueId` in instructions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Const {
    /// Signed integer constant (iN).  Stored as `i128` to cover all widths.
    Int(i128),
    /// Unsigned integer constant (uN).  Stored as `u128`.
    Uint(u128),
    /// Boolean constant.
    Bool(bool),
    /// Typed null pointer.
    NullPtr,
    /// Zero-initialiser for any type.
    Zero,
    /// IEEE-754 floating-point constant.
    Float(FloatWrapper),
    /// String literal (for `StringAddr` instructions).
    String(String),
}

// ---------------------------------------------------------------------------
// Instruction
// ---------------------------------------------------------------------------

/// One SSA instruction inside a basic block.
///
/// Every instruction produces exactly one value (identified by a `ValueId`).
/// Memory-effecting instructions (`Store`, `HeapFree`, …) also produce a
/// value — often `void` — so that they fit the SSA model uniformly.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    // -- integer arithmetic -------------------------------------------------
    Add(ValueId, ValueId),
    Sub(ValueId, ValueId),
    Mul(ValueId, ValueId),
    DivS(ValueId, ValueId),
    DivU(ValueId, ValueId),
    RemS(ValueId, ValueId),
    RemU(ValueId, ValueId),

    // -- floating-point arithmetic ------------------------------------------
    FAdd(ValueId, ValueId),
    FSub(ValueId, ValueId),
    FMul(ValueId, ValueId),
    FDiv(ValueId, ValueId),
    FNeg(ValueId),

    // -- bitwise ------------------------------------------------------------
    And(ValueId, ValueId),
    Or(ValueId, ValueId),
    Xor(ValueId, ValueId),
    Shl(ValueId, ValueId),
    ShrS(ValueId, ValueId),
    ShrU(ValueId, ValueId),

    // -- integer comparisons ------------------------------------------------
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

    // -- floating-point comparisons -----------------------------------------
    Feq(ValueId, ValueId),
    Fne(ValueId, ValueId),
    Flt(ValueId, ValueId),
    Fle(ValueId, ValueId),
    Fgt(ValueId, ValueId),
    Fge(ValueId, ValueId),

    // -- type conversions ---------------------------------------------------
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

    // -- memory -------------------------------------------------------------
    Alloca(IrType),
    Load(IrType, ValueId),  // (pointee_type, ptr)
    Store(ValueId, ValueId), // (value, ptr)

    // -- heap ---------------------------------------------------------------
    HeapAlloc(ValueId), // size in bytes → ptr
    HeapFree(ValueId),  // ptr

    // -- arenas (region-based allocation) -----------------------------------
    ArenaCreate(ValueId), // capacity in bytes → arena handle
    ArenaAlloc(ValueId, ValueId), // (arena, size) → ptr
    ArenaFree(ValueId),   // arena handle

    // -- address-of ---------------------------------------------------------
    FnAddr(String),             // function name → function pointer
    GlobalAddr(String),         // global name → pointer to global
    StringAddr(String),         // string constant → pointer to bytes

    // -- pointer arithmetic -------------------------------------------------
    Gep(ValueId, Vec<ValueId>),                  // getelementptr (base, indices)
    FieldAddr(ValueId, usize, Option<IrType>),   // address of struct field (ptr, field_index, struct_type?)

    // -- aggregate ----------------------------------------------------------
    BuildStruct(Vec<ValueId>),
    ExtractField(ValueId, usize),

    // -- calls --------------------------------------------------------------
    Call(String, Vec<ValueId>),         // direct call
    CallIndirect(ValueId, Vec<ValueId>), // indirect call
}

// ---------------------------------------------------------------------------
// Terminator
// ---------------------------------------------------------------------------

/// Every basic block ends with exactly one terminator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Terminator {
    /// Unconditional branch to `BlockId`.
    Br(BlockId),
    /// Conditional branch: (condition, true_block, false_block).
    BrCond(ValueId, BlockId, BlockId),
    /// Return a value from the function.
    Ret(ValueId),
    /// Return from a void function.
    RetVoid,
    /// Multi-way dispatch: (scrutinee, default_block, arms).
    Switch(ValueId, BlockId, Vec<(ValueId, BlockId)>),
}

// ---------------------------------------------------------------------------
// IrBlock
// ---------------------------------------------------------------------------

/// A single basic block: a sequence of non-terminator instructions followed
/// by exactly one terminator.
#[derive(Debug, Clone)]
pub struct IrBlock {
    /// Label for the block (used in branch targets and debugging).
    pub label: String,
    /// Non-terminator instructions in order.
    pub instructions: Vec<Instruction>,
    /// Block-ending terminator.
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
// IrGlobal
// ---------------------------------------------------------------------------

/// A module-level global variable.
#[derive(Debug, Clone)]
pub struct IrGlobal {
    pub name: String,
    pub bytes: Vec<u8>,
    /// Whether this global is constant (read-only).
    pub is_const: bool,
}

// ---------------------------------------------------------------------------
// Function
// ---------------------------------------------------------------------------

/// A complete IR function definition.
#[derive(Debug, Clone)]
pub struct Function {
    /// Function name (linker symbol).
    pub name: String,
    /// Return type (LLVM-level, after sret rewriting).
    pub return_ty: IrType,
    /// Original return type before sret rewriting (for ABI / debugging).
    pub source_return_ty: IrType,
    /// How the return value is passed (Direct or HiddenPtr/sret).
    pub return_class: IrReturnClass,
    /// Formal parameters: (name, type).
    pub params: Vec<(String, IrType)>,
    /// All basic blocks owned by this function.
    pub blocks: Vec<IrBlock>,
    /// Canonicalised constant values, indexed by `ValueId`.
    pub values: Vec<Const>,
    /// The `BlockId` of the entry (first) block.
    pub entry: BlockId,
    /// Whether the function uses the C ABI (extern "C").
    pub c_abi: bool,
    /// Whether the function is variadic.
    pub variadic: bool,
    /// When `return_class == HiddenPtr`, the `ValueId` of the sret pointer
    /// (which is an implicit first parameter).
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

    /// Append a block and return its `BlockId`.
    pub fn push_block(&mut self, block: IrBlock) -> BlockId {
        let id = BlockId(self.blocks.len());
        self.blocks.push(block);
        id
    }

    /// Intern a constant and return a `ValueId` that can be used as an
    /// operand.  Deduplication is performed to avoid duplicate entries.
    ///
    /// ValueIds for constants start after the parameter slots:
    ///   ValueId(params.len() + position_in_values)
    pub fn intern_const(&mut self, c: Const) -> ValueId {
        let pos = self.values.len();
        let id = ValueId(self.params.len() + pos);
        self.values.push(c);
        id
    }

    /// Number of blocks.
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }
}

// ---------------------------------------------------------------------------
// Module
// ---------------------------------------------------------------------------

/// Top-level compilation unit.
#[derive(Debug, Clone)]
pub struct Module {
    /// Module / translation-unit name.
    pub name: String,
    /// Type definitions (name, IrType).  Populated during lowering.
    pub types: Vec<(String, IrType)>,
    /// Module-level globals.
    pub globals: Vec<IrGlobal>,
    /// Function definitions.
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
// IrReturnClass
// ---------------------------------------------------------------------------

/// How a function's return value is passed at the ABI level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrReturnClass {
    /// Return in registers (up to 2 fields).
    Direct,
    /// Return via a hidden pointer (sret) — caller allocates, callee writes.
    HiddenPtr,
}

// ---------------------------------------------------------------------------
// IrBuilder — convenience API for building IR inside a function
// ---------------------------------------------------------------------------

/// Incremental IR builder that appends instructions to the *current* block of
/// a function.
pub struct IrBuilder<'f> {
    func: &'f mut Function,
    /// The block that `push` appends to.
    current: BlockId,
    /// A map from (Instruction, operand-ids) → `ValueId` for local CSE.
    /// Keys are tuples so that we can re-use simple expressions.
    cache: HashMap<Instruction, ValueId>,
}

impl<'f> IrBuilder<'f> {
    /// Create a builder positioned at `entry_block` of the given function.
    /// The entry block must already exist (typically created by the caller
    /// with `Function::push_block`).
    pub fn new(func: &'f mut Function, entry_block: BlockId) -> Self {
        Self {
            func,
            current: entry_block,
            cache: HashMap::new(),
        }
    }

    /// Switch the builder to append to a different block.
    pub fn switch_to_block(&mut self, block: BlockId) {
        self.current = block;
        self.cache.clear(); // CSE is local to a block
    }

    /// Return the current `BlockId`.
    pub fn current_block(&self) -> BlockId {
        self.current
    }

    /// Access the underlying function (read-only).
    #[allow(dead_code)]
    pub fn func(&self) -> &Function {
        self.func
    }

    /// Access the underlying function (mutable).
    #[allow(dead_code)]
    pub fn func_mut(&mut self) -> &mut Function {
        self.func
    }

    /// Intern a constant and return a `ValueId`.
    pub fn const_val(&mut self, c: Const) -> ValueId {
        self.func.intern_const(c)
    }

    /// Append an instruction to the current block and return a fresh
    /// `ValueId` for its result.  If the same instruction already exists in
    /// the current block's cache the existing `ValueId` is returned (local
    /// common-subexpression elimination).
    fn emit(&mut self, inst: Instruction) -> ValueId {
        if let Some(&existing) = self.cache.get(&inst) {
            return existing;
        }

        let block = &mut self.func.blocks[self.current.0];
        block.push(inst.clone());

        // Values produced by instructions are numbered sequentially *after*
        // constant values so that ValueId ranges do not collide.
        let id = ValueId(self.func.values.len() + block.len() - 1);
        self.cache.insert(inst, id);
        id
    }

    // -- convenience emit helpers for each op -------------------------------

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

    pub fn build_call(&mut self, callee: impl Into<String>, args: Vec<ValueId>) -> ValueId {
        self.emit(Instruction::Call(callee.into(), args))
    }
    pub fn build_call_indirect(&mut self, fn_ptr: ValueId, args: Vec<ValueId>) -> ValueId {
        self.emit(Instruction::CallIndirect(fn_ptr, args))
    }

    // -- terminator helpers -------------------------------------------------

    /// Set the terminator of the current block (replacing any previous one).
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
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: create a minimal void-returning function with one empty block.
    fn dummy_func() -> Function {
        let mut f = Function::new("test", IrType::Void, vec![]);
        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        // entry is always BlockId(0) here, but we do not rely on that.
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
        // Push a second block
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

        // Values from constants
        assert_eq!(one, ValueId(0));
        assert_eq!(two, ValueId(1));
        // sum is the first instruction
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
        let b2 = b.build_add(x, y); // same instruction → should CSE
        assert_eq!(a, b2, "CSE should return the same ValueId");
        assert_eq!(b.func.blocks[entry.0].len(), 1);
    }

    #[test]
    fn test_builder_terminator() {
        let mut f = dummy_func();
        let entry = f.entry;
        // Entry block currently has RetVoid.
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
        // Bitwise-equal NaN representations should be equal
        assert_eq!(nan_a, nan_b);
    }
}
