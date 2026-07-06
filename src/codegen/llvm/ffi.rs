//! Vifungo vya FFI vya LLVM-C kwa mkusanyaji wa Swa.
//!
//! Vifungo vyembamba, vilivyoandikwa kwa mkono kwa API ya LLVM 18.1 C.
//! Tunakwepa tegemezi za crate (inkwell, llvm-sys) na tunaunganisha moja kwa
//! moja dhidi ya `LLVM-C.dll` wakati wa utekelezaji kupitia hati ya ujenzi
//! (`cargo:rustc-link-lib=LLVM-C`).
//!
//! Aina zote za vielekezi si wazi (`*mut c_void` au sawa) ili waitwao
//! wasiwahi kuhitaji ufafanuzi wa kichwa cha LLVM.
//!
//! ## LLVM 18.1.8
//!
//! Imesakinishwa kwenye `C:\LLVM18`.  Hati ya ujenzi inaiambia rustc
//! mahali `.lib` na `.dll` ziko.

use std::ffi::{c_char, c_void, CStr, CString};

// ---------------------------------------------------------------------------
// Aina za vielekezi opaque vya LLVM-C
// ---------------------------------------------------------------------------

pub type LLVMBool = i32;

pub type LLVMContextRef = *mut c_void;
pub type LLVMModuleRef = *mut c_void;
pub type LLVMTypeRef = *mut c_void;
pub type LLVMValueRef = *mut c_void;
pub type LLVMBasicBlockRef = *mut c_void;
pub type LLVMBuilderRef = *mut c_void;
pub type LLVMMemoryBufferRef = *mut c_void;
pub type LLVMTargetRef = *mut c_void;
pub type LLVMTargetMachineRef = *mut c_void;
pub type LLVMTargetDataRef = *mut c_void;
pub type LLVMPassManagerRef = *mut c_void;
pub type LLVMErrorRef = *mut c_void;
pub type LLVMPassBuilderOptionsRef = *mut c_void;
pub type LLVMAttributeRef = *mut c_void;

// ---------------------------------------------------------------------------
// Enumeri
// ---------------------------------------------------------------------------

/// Vivumishi vya ulinganishaji namba sahihi.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LLVMIntPredicate {
    EQ  = 32,
    NE  = 33,
    UGT = 34,
    UGE = 35,
    ULT = 36,
    ULE = 37,
    SGT = 38,
    SGE = 39,
    SLT = 40,
    SLE = 41,
}

/// Vivumishi vya ulinganishaji namba sehemu-desimali.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LLVMRealPredicate {
    OEQ    = 0,
    OGT    = 1,
    OGE    = 2,
    OLT    = 3,
    OLE    = 4,
    ONE    = 5,
    ORD    = 6,
    UNO    = 7,
    UEQ    = 8,
    UGT    = 9,
    UGE    = 10,
    ULT    = 11,
    ULE    = 12,
    UNE    = 13,
    /// Kweli ikiwa operanda yoyote ni NaN (tumia kwa "iliyopangwa na sawa").
    FALSE  = 14,   // daima si kweli
    TRUE   = 15,   // daima kweli
}

/// Kiwango cha uboreshaji cha kuzalisha msimbo.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LLVMCodeGenOptLevel {
    None       = 0,
    Less       = 1,
    Default    = 2,
    Aggressive = 3,
}

/// Mfano wa uhamishaji.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LLVMRelocMode {
    Default      = 0,
    Static       = 1,
    PIC          = 2,
    DynamicNoPic = 3,
}

/// Mfano wa msimbo.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LLVMCodeModel {
    Default    = 0,
    JITDefault = 1,
    Small      = 2,
    Kernel     = 3,
    Medium     = 4,
    Large      = 5,
}

/// Aina ya faili ya pato kwa `LLVMTargetMachineEmitToFile`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LLVMCodeGenFileType {
    AssemblyFile = 0,
    ObjectFile   = 1,
}

/// Aina za uunganishaji.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LLVMLinkage {
    External = 0,
    Private  = 9,
}

/// Aina za aina zinazorejeshwa na `LLVMGetTypeKind`.
///
/// Majina ya lahaja yanafuata enum ya LLVM-C haswa; `#[allow]` inatumika kwa
/// sababu hayafuati mkataba wa CamelCase wa Rust.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum LLVMTypeKind {
    Void      = 0,
    Half      = 1,
    Float     = 2,
    Double    = 3,
    X86_FP80  = 4,
    FP128     = 5,
    PPC_FP128 = 6,
    Label     = 7,
    Integer   = 8,
    Function  = 9,
    Struct    = 10,
    Array     = 11,
    Pointer   = 12,
    Vector    = 13,
    Metadata  = 14,
    X86_MMX   = 15,
    Token     = 16,
    ScalableVector = 17,
    BFloat    = 18,
    X86_AMX   = 19,
}

/// Fahirisi ya mwangalizi wa sifa kwa `LLVMAddAttributeAtIndex`.
pub const LLVM_ATTRIBUTE_FUNCTION_INDEX: u32 = u32::MAX;
pub const LLVM_ATTRIBUTE_RETURN_INDEX: u32 = 0;

// ---------------------------------------------------------------------------
// LLVM-C API — extern block
// ---------------------------------------------------------------------------

// Kuunganisha kunashughulikiwa na build.rs (linux: -lLLVM, windows: -lLLVM-C).
extern "C" {
    // -- muktadha / moduli ---------------------------------------------------

    pub fn LLVMContextCreate() -> LLVMContextRef;
    pub fn LLVMContextDispose(ctx: LLVMContextRef);
    pub fn LLVMGetGlobalContext() -> LLVMContextRef;

    pub fn LLVMModuleCreateWithName(name: *const c_char) -> LLVMModuleRef;
    pub fn LLVMModuleCreateWithNameInContext(
        name: *const c_char,
        ctx: LLVMContextRef,
    ) -> LLVMModuleRef;
    pub fn LLVMDisposeModule(module: LLVMModuleRef);
    pub fn LLVMCloneModule(module: LLVMModuleRef) -> LLVMModuleRef;
    pub fn LLVMSetTarget(module: LLVMModuleRef, triple: *const c_char);
    pub fn LLVMGetTarget(module: LLVMModuleRef) -> *const c_char;
    pub fn LLVMGetModuleContext(module: LLVMModuleRef) -> LLVMContextRef;

    // -- aina ----------------------------------------------------------------

    pub fn LLVMInt1Type() -> LLVMTypeRef;
    pub fn LLVMInt8Type() -> LLVMTypeRef;
    pub fn LLVMInt16Type() -> LLVMTypeRef;
    pub fn LLVMInt32Type() -> LLVMTypeRef;
    pub fn LLVMInt64Type() -> LLVMTypeRef;
    pub fn LLVMIntTypeInContext(ctx: LLVMContextRef, bits: u32) -> LLVMTypeRef;
    pub fn LLVMInt128Type() -> LLVMTypeRef;
    pub fn LLVMHalfType() -> LLVMTypeRef;
    pub fn LLVMFloatType() -> LLVMTypeRef;
    pub fn LLVMDoubleType() -> LLVMTypeRef;
    pub fn LLVMX86FP80Type() -> LLVMTypeRef;
    pub fn LLVMFP128Type() -> LLVMTypeRef;
    pub fn LLVMVoidType() -> LLVMTypeRef;
    pub fn LLVMPointerType(ty: LLVMTypeRef, addr_space: u32) -> LLVMTypeRef;
    pub fn LLVMArrayType(ty: LLVMTypeRef, count: u32) -> LLVMTypeRef;
    pub fn LLVMStructCreateNamed(ctx: LLVMContextRef, name: *const c_char) -> LLVMTypeRef;
    pub fn LLVMStructSetBody(
        ty: LLVMTypeRef,
        elements: *mut LLVMTypeRef,
        count: u32,
        packed: LLVMBool,
    );
    pub fn LLVMGetTypeKind(ty: LLVMTypeRef) -> LLVMTypeKind;
    pub fn LLVMGetIntTypeWidth(ty: LLVMTypeRef) -> u32;
    pub fn LLVMPrintTypeToString(ty: LLVMTypeRef) -> *const c_char;
    pub fn LLVMTypeOf(val: LLVMValueRef) -> LLVMTypeRef;
    pub fn LLVMGetElementType(ty: LLVMTypeRef) -> LLVMTypeRef;
    pub fn LLVMFunctionType(
        return_ty: LLVMTypeRef,
        params: *mut LLVMTypeRef,
        param_count: u32,
        is_var_arg: LLVMBool,
    ) -> LLVMTypeRef;

    // -- thamani / ulimwengu / kazi -------------------------------------------

    pub fn LLVMAddFunction(
        module: LLVMModuleRef,
        name: *const c_char,
        ty: LLVMTypeRef,
    ) -> LLVMValueRef;
    pub fn LLVMGetNamedFunction(
        module: LLVMModuleRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMGetParam(func: LLVMValueRef, index: u32) -> LLVMValueRef;
    pub fn LLVMCountParams(func: LLVMValueRef) -> u32;
    pub fn LLVMGetValueName2(value: LLVMValueRef, len: *mut usize) -> *const c_char;
    pub fn LLVMSetValueName2(value: LLVMValueRef, name: *const c_char, name_len: usize);

    pub fn LLVMAddGlobal(
        module: LLVMModuleRef,
        ty: LLVMTypeRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMGetNamedGlobal(
        module: LLVMModuleRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMSetInitializer(global: LLVMValueRef, constant: LLVMValueRef);
    pub fn LLVMSetGlobalConstant(global: LLVMValueRef, is_const: LLVMBool);
    pub fn LLVMIsGlobalConstant(global: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetLinkage(global: LLVMValueRef, linkage: LLVMLinkage);
    pub fn LLVMGetLinkage(global: LLVMValueRef) -> LLVMLinkage;

    // -- vibadilika ----------------------------------------------------------

    pub fn LLVMConstInt(ty: LLVMTypeRef, value: u64, sign_extend: LLVMBool) -> LLVMValueRef;
    pub fn LLVMConstIntOfArbitraryPrecision(
        IntTy: LLVMTypeRef,
        NumWords: u32,
        Words: *const u64,
    ) -> LLVMValueRef;
    pub fn LLVMConstIntOfString(ty: LLVMTypeRef, text: *const c_char, radix: u8) -> LLVMValueRef;
    pub fn LLVMConstReal(ty: LLVMTypeRef, value: f64) -> LLVMValueRef;
    pub fn LLVMConstNull(ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstString(
        str: *const c_char,
        length: u32,
        null_terminate: LLVMBool,
    ) -> LLVMValueRef;
    pub fn LLVMConstArray(
        element_ty: LLVMTypeRef,
        values: *mut LLVMValueRef,
        count: u32,
    ) -> LLVMValueRef;
    pub fn LLVMConstStructInContext(
        ctx: LLVMContextRef,
        values: *mut LLVMValueRef,
        count: u32,
        packed: LLVMBool,
    ) -> LLVMValueRef;
    pub fn LLVMConstNamedStruct(
        ty: LLVMTypeRef,
        values: *mut LLVMValueRef,
        count: u32,
    ) -> LLVMValueRef;
    pub fn LLVMConstPointerNull(ty: LLVMTypeRef) -> LLVMValueRef;

    // -- kijenzi -------------------------------------------------------------

    pub fn LLVMCreateBuilder() -> LLVMBuilderRef;
    pub fn LLVMDisposeBuilder(builder: LLVMBuilderRef);
    pub fn LLVMPositionBuilderAtEnd(builder: LLVMBuilderRef, block: LLVMBasicBlockRef);
    pub fn LLVMPositionBuilderBefore(builder: LLVMBuilderRef, inst: LLVMValueRef);
    pub fn LLVMGetInsertBlock(builder: LLVMBuilderRef) -> LLVMBasicBlockRef;
    pub fn LLVMClearInsertionPosition(builder: LLVMBuilderRef);

    // -- hesabu --------------------------------------------------------------

    pub fn LLVMBuildAdd(
        builder: LLVMBuilderRef, lhs: LLVMValueRef, rhs: LLVMValueRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildSub(
        builder: LLVMBuilderRef, lhs: LLVMValueRef, rhs: LLVMValueRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildMul(
        builder: LLVMBuilderRef, lhs: LLVMValueRef, rhs: LLVMValueRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildSDiv(
        builder: LLVMBuilderRef, lhs: LLVMValueRef, rhs: LLVMValueRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildUDiv(
        builder: LLVMBuilderRef, lhs: LLVMValueRef, rhs: LLVMValueRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildSRem(
        builder: LLVMBuilderRef, lhs: LLVMValueRef, rhs: LLVMValueRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildURem(
        builder: LLVMBuilderRef, lhs: LLVMValueRef, rhs: LLVMValueRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFAdd(
        builder: LLVMBuilderRef, lhs: LLVMValueRef, rhs: LLVMValueRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFSub(
        builder: LLVMBuilderRef, lhs: LLVMValueRef, rhs: LLVMValueRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFMul(
        builder: LLVMBuilderRef, lhs: LLVMValueRef, rhs: LLVMValueRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFDiv(
        builder: LLVMBuilderRef, lhs: LLVMValueRef, rhs: LLVMValueRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFNeg(
        builder: LLVMBuilderRef, val: LLVMValueRef, name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildAnd(
        builder: LLVMBuilderRef, lhs: LLVMValueRef, rhs: LLVMValueRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildOr(
        builder: LLVMBuilderRef, lhs: LLVMValueRef, rhs: LLVMValueRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildXor(
        builder: LLVMBuilderRef, lhs: LLVMValueRef, rhs: LLVMValueRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildShl(
        builder: LLVMBuilderRef, lhs: LLVMValueRef, rhs: LLVMValueRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildAShr(
        builder: LLVMBuilderRef, lhs: LLVMValueRef, rhs: LLVMValueRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildLShr(
        builder: LLVMBuilderRef, lhs: LLVMValueRef, rhs: LLVMValueRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildNeg(
        builder: LLVMBuilderRef, val: LLVMValueRef, name: *const c_char,
    ) -> LLVMValueRef;

    // -- ulinganishaji -------------------------------------------------------

    pub fn LLVMBuildICmp(
        builder: LLVMBuilderRef, pred: LLVMIntPredicate,
        lhs: LLVMValueRef, rhs: LLVMValueRef, name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFCmp(
        builder: LLVMBuilderRef, pred: LLVMRealPredicate,
        lhs: LLVMValueRef, rhs: LLVMValueRef, name: *const c_char,
    ) -> LLVMValueRef;

    // -- ubadilishaji --------------------------------------------------------

    pub fn LLVMBuildTrunc(
        builder: LLVMBuilderRef, val: LLVMValueRef, ty: LLVMTypeRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildZExt(
        builder: LLVMBuilderRef, val: LLVMValueRef, ty: LLVMTypeRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildSExt(
        builder: LLVMBuilderRef, val: LLVMValueRef, ty: LLVMTypeRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFPTrunc(
        builder: LLVMBuilderRef, val: LLVMValueRef, ty: LLVMTypeRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFPExt(
        builder: LLVMBuilderRef, val: LLVMValueRef, ty: LLVMTypeRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFPToSI(
        builder: LLVMBuilderRef, val: LLVMValueRef, ty: LLVMTypeRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFPToUI(
        builder: LLVMBuilderRef, val: LLVMValueRef, ty: LLVMTypeRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildSIToFP(
        builder: LLVMBuilderRef, val: LLVMValueRef, ty: LLVMTypeRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildUIToFP(
        builder: LLVMBuilderRef, val: LLVMValueRef, ty: LLVMTypeRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildBitCast(
        builder: LLVMBuilderRef, val: LLVMValueRef, ty: LLVMTypeRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildPtrToInt(
        builder: LLVMBuilderRef, val: LLVMValueRef, ty: LLVMTypeRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildIntToPtr(
        builder: LLVMBuilderRef, val: LLVMValueRef, ty: LLVMTypeRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildIntCast2(
        builder: LLVMBuilderRef, val: LLVMValueRef, ty: LLVMTypeRef,
        is_signed: LLVMBool, name: *const c_char,
    ) -> LLVMValueRef;

    // -- kumbukumbu ----------------------------------------------------------

    pub fn LLVMBuildAlloca(
        builder: LLVMBuilderRef, ty: LLVMTypeRef, name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildArrayAlloca(
        builder: LLVMBuilderRef, ty: LLVMTypeRef,
        size: LLVMValueRef, name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildLoad2(
        builder: LLVMBuilderRef, ty: LLVMTypeRef, ptr: LLVMValueRef,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildStore(
        builder: LLVMBuilderRef, val: LLVMValueRef, ptr: LLVMValueRef,
    ) -> LLVMValueRef;

    // -- mtiririko wa udhibiti -----------------------------------------------

    pub fn LLVMAppendBasicBlockInContext(
        ctx: LLVMContextRef,
        func: LLVMValueRef,
        name: *const c_char,
    ) -> LLVMBasicBlockRef;
    pub fn LLVMGetEntryBasicBlock(func: LLVMValueRef) -> LLVMBasicBlockRef;
    pub fn LLVMBuildBr(builder: LLVMBuilderRef, dest: LLVMBasicBlockRef) -> LLVMValueRef;
    pub fn LLVMBuildCondBr(
        builder: LLVMBuilderRef, cond: LLVMValueRef,
        then_block: LLVMBasicBlockRef, else_block: LLVMBasicBlockRef,
    ) -> LLVMValueRef;
    pub fn LLVMBuildRet(builder: LLVMBuilderRef, val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMBuildRetVoid(builder: LLVMBuilderRef) -> LLVMValueRef;
    pub fn LLVMBuildSwitch(
        builder: LLVMBuilderRef, val: LLVMValueRef,
        default_block: LLVMBasicBlockRef, num_cases: u32,
    ) -> LLVMValueRef;
    pub fn LLVMAddCase(
        switch: LLVMValueRef,
        on_val: LLVMValueRef,
        dest: LLVMBasicBlockRef,
    );

    // -- mwito ---------------------------------------------------------------

    pub fn LLVMBuildCall2(
        builder: LLVMBuilderRef, fn_ty: LLVMTypeRef, func: LLVMValueRef,
        args: *mut LLVMValueRef, num_args: u32, name: *const c_char,
    ) -> LLVMValueRef;

    // -- GEP / toa -----------------------------------------------------------

    pub fn LLVMBuildGEP2(
        builder: LLVMBuilderRef, ty: LLVMTypeRef, ptr: LLVMValueRef,
        indices: *mut LLVMValueRef, num_indices: u32, name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildExtractValue(
        builder: LLVMBuilderRef, agg: LLVMValueRef, index: u32,
        name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildInsertValue(
        builder: LLVMBuilderRef, agg: LLVMValueRef, val: LLVMValueRef,
        index: u32, name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildSelect(
        builder: LLVMBuilderRef, cond: LLVMValueRef, then_val: LLVMValueRef,
        else_val: LLVMValueRef, name: *const c_char,
    ) -> LLVMValueRef;

    // -- phi -----------------------------------------------------------------

    pub fn LLVMBuildPhi(
        builder: LLVMBuilderRef, ty: LLVMTypeRef, name: *const c_char,
    ) -> LLVMValueRef;
    pub fn LLVMAddIncoming(
        phi: LLVMValueRef,
        values: *mut LLVMValueRef,
        blocks: *mut LLVMBasicBlockRef,
        count: u32,
    );

    // -- I/O ya moduli -------------------------------------------------------

    pub fn LLVMPrintModuleToString(module: LLVMModuleRef) -> *const c_char;
    pub fn LLVMDisposeMessage(message: *const c_char);
    pub fn LLVMParseIRInContext(
        ctx: LLVMContextRef,
        mem_buf: LLVMMemoryBufferRef,
        out_module: *mut LLVMModuleRef,
        out_error: *mut *mut c_char,
    ) -> LLVMBool;
    pub fn LLVMCreateMemoryBufferWithMemoryRangeCopy(
        input_data: *const c_char,
        input_data_length: usize,
        buffer_name: *const c_char,
    ) -> LLVMMemoryBufferRef;
    pub fn LLVMDisposeMemoryBuffer(buf: LLVMMemoryBufferRef);

    // -- uhakiki -------------------------------------------------------------

    pub fn LLVMVerifyModule(
        module: LLVMModuleRef,
        action: u32,
        out_error: *mut *mut c_char,
    ) -> LLVMBool;

    // -- ugunduzi wa lengwa --------------------------------------------------

    pub fn LLVMGetDefaultTargetTriple() -> *const c_char;
    pub fn LLVMGetTargetFromTriple(
        triple: *const c_char,
        out_target: *mut LLVMTargetRef,
        out_error: *mut *mut c_char,
    ) -> LLVMBool;
    pub fn LLVMCreateTargetMachine(
        target: LLVMTargetRef,
        triple: *const c_char,
        cpu: *const c_char,
        features: *const c_char,
        level: LLVMCodeGenOptLevel,
        reloc: LLVMRelocMode,
        code_model: LLVMCodeModel,
    ) -> LLVMTargetMachineRef;
    pub fn LLVMDisposeTargetMachine(tm: LLVMTargetMachineRef);
    pub fn LLVMTargetMachineEmitToFile(
        tm: LLVMTargetMachineRef,
        module: LLVMModuleRef,
        filename: *const c_char,
        file_type: LLVMCodeGenFileType,
        out_error: *mut *mut c_char,
    ) -> LLVMBool;

    // -- uanzishaji wa lengwa x86 --------------------------------------------

    pub fn LLVMInitializeX86TargetInfo();
    pub fn LLVMInitializeX86Target();
    pub fn LLVMInitializeX86TargetMC();
    pub fn LLVMInitializeX86AsmPrinter();
    pub fn LLVMInitializeX86AsmParser();

    pub fn LLVMInitializeARMTargetInfo();
    pub fn LLVMInitializeARMTarget();
    pub fn LLVMInitializeARMTargetMC();
    pub fn LLVMInitializeARMAsmPrinter();
    pub fn LLVMInitializeARMAsmParser();

    pub fn LLVMInitializeAArch64TargetInfo();
    pub fn LLVMInitializeAArch64Target();
    pub fn LLVMInitializeAArch64TargetMC();
    pub fn LLVMInitializeAArch64AsmPrinter();
    pub fn LLVMInitializeAArch64AsmParser();

    pub fn LLVMInitializeRISCVTargetInfo();
    pub fn LLVMInitializeRISCVTarget();
    pub fn LLVMInitializeRISCVTargetMC();
    pub fn LLVMInitializeRISCVAsmPrinter();
    pub fn LLVMInitializeRISCVAsmParser();

    // -- sifa ----------------------------------------------------------------

    pub fn LLVMCreateEnumAttribute(
        ctx: LLVMContextRef,
        kind_id: u32,
        val: u64,
    ) -> LLVMAttributeRef;
    pub fn LLVMAddAttributeAtIndex(
        func: LLVMValueRef,
        index: u32,
        attr: LLVMAttributeRef,
    );
    pub fn LLVMGetEnumAttributeKind(name: *const c_char) -> u32;
    pub fn LLVMGetStringAttributeKind(name: *const c_char) -> u32;

    // -- DIBuilder (vibadala vya msingi vya habari za utatuzi) ----------------

    pub fn LLVMCreateDIBuilder(module: LLVMModuleRef) -> *mut c_void;
    pub fn LLVMDisposeDIBuilder(builder: *mut c_void);
    pub fn LLVMDIBuilderFinalize(builder: *mut c_void);

    // -- ushughulikiaji wa makosa --------------------------------------------

    pub fn LLVMConsumeError(err: LLVMErrorRef);
    pub fn LLVMGetErrorMessage(err: LLVMErrorRef) -> *mut c_char;
    pub fn LLVMDisposeErrorMessage(msg: *mut c_char);

    // -- kijenzi cha kupita (meneja mpy wa kupita / bomba la uboreshaji) -----

    /// Run a set of passes on a module using the new pass manager pipeline
    /// syntax (e.g. "function(mem2reg,instcombine,gvn,simplifycfg)").
    /// Returns null on success, or an LLVMErrorRef on failure.
    pub fn LLVMRunPasses(
        m: LLVMModuleRef,
        passes: *const c_char,
        tm: LLVMTargetMachineRef,
        options: LLVMPassBuilderOptionsRef,
    ) -> LLVMErrorRef;

    /// Run a set of passes on a single function using the new pass manager
    /// pipeline syntax.
    /// Returns null on success, or an LLVMErrorRef on failure.
    pub fn LLVMRunPassesOnFunction(
        f: LLVMValueRef,
        passes: *const c_char,
        tm: LLVMTargetMachineRef,
        options: LLVMPassBuilderOptionsRef,
    ) -> LLVMErrorRef;

    pub fn LLVMCreatePassBuilderOptions() -> LLVMPassBuilderOptionsRef;
    pub fn LLVMDisposePassBuilderOptions(options: LLVMPassBuilderOptionsRef);

    pub fn LLVMPassBuilderOptionsSetVerifyEach(
        options: LLVMPassBuilderOptionsRef,
        verify_each: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetDebugLogging(
        options: LLVMPassBuilderOptionsRef,
        debug_logging: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetLoopInterleaving(
        options: LLVMPassBuilderOptionsRef,
        enable: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetLoopVectorization(
        options: LLVMPassBuilderOptionsRef,
        enable: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetSLPVectorization(
        options: LLVMPassBuilderOptionsRef,
        enable: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetLoopUnrolling(
        options: LLVMPassBuilderOptionsRef,
        enable: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetForgetAllSCEVInLoopUnroll(
        options: LLVMPassBuilderOptionsRef,
        forget: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetLicmMssaOptCap(
        options: LLVMPassBuilderOptionsRef,
        cap: u32,
    );
    pub fn LLVMPassBuilderOptionsSetLicmMssaNoAccForPromotionCap(
        options: LLVMPassBuilderOptionsRef,
        cap: u32,
    );
    pub fn LLVMPassBuilderOptionsSetCallGraphProfile(
        options: LLVMPassBuilderOptionsRef,
        enable: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetMergeFunctions(
        options: LLVMPassBuilderOptionsRef,
        enable: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetInlinerThreshold(
        options: LLVMPassBuilderOptionsRef,
        threshold: i32,
    );
    pub fn LLVMPassBuilderOptionsSetAAPipeline(
        options: LLVMPassBuilderOptionsRef,
        aa_pipeline: *const c_char,
    );
}

// ---------------------------------------------------------------------------
// Visaidizi vya urahisi
// ---------------------------------------------------------------------------

/// Badilisha `&str` ya Rust hadi `CString` iliyomalizika kwa nul, ukirudisha
/// baiti zinazomilikiwa ili mpigaji aweze kutoa kielekezi cha `*const c_char`
/// halali kwa maisha ya thamani iliyorejeshwa.
///
/// # Hofia
///
/// Hofia ikiwa `s` ina baiti ya nul ya ndani.
#[inline]
pub fn c_str(s: &str) -> CString {
    CString::new(s).expect("interior nul byte in C string")
}

/// Tumia mfuato wa C unaomilikiwa na LLVM na urudishe `String` ya Rust.
///
/// Hii inafaa kwa mifuatano inayorejeshwa na kazi za LLVM zinazotumia
/// `LLVMDisposeMessage` kwa usafishaji (k.m. `LLVMPrintModuleToString`).
///
/// # Usalama
///
/// `ptr` lazima iwe mfuato wa C usio null uliomalizika kwa nul uliotengwa na LLVM.
/// Baada ya mwito huu mpigaji **hatakiwi** kutumia `ptr` tena — umiliki
/// umehamishwa hadi kwenye `String` iliyorejeshwa.
#[inline]
pub unsafe fn owned_str_from_llvm(ptr: *const c_char) -> String {
    assert!(!ptr.is_null(), "LLVM returned null string pointer");
    let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    LLVMDisposeMessage(ptr);
    s
}

/// Hakiki moduli ya LLVM na urudishe ujumbe wa utambuzi kwenye kushindwa.
///
/// `action` hudhibiti tabia ya mhakiki kwenye kushindwa:
///   - `0` — acha mchakato (muhimu kwa ujenzi wa utatuzi)
///   - `1` — chapisha kwa stderr na endelea
///   - `2` — rudisha ujumbe wa kosa (cha msingi hapa)
///
/// Hurejesha `Ok(())` ikiwa moduli inafanya uhakiki, au `Err(String)` na
/// ujumbe wa kosa wa mhakiki.
///
/// # Usalama
///
/// `module` lazima iwe `LLVMModuleRef` halali.
pub unsafe fn verify_module(module: LLVMModuleRef) -> Result<(), String> {
    let mut error: *mut c_char = std::ptr::null_mut();
    let failed = LLVMVerifyModule(module, 2, &mut error);
    if failed != 0 {
        let msg = if error.is_null() {
            "module verification failed (no details)".to_string()
        } else {
            let s = CStr::from_ptr(error).to_string_lossy().into_owned();
            LLVMDisposeMessage(error);
            s
        };
        Err(msg)
    } else {
        Ok(())
    }
}

/// Rudisha tatu ya lengwa cha msingi kwa mashine mwenyeji.
///
/// Hii inaita `LLVMGetDefaultTargetTriple` na kunakili matokeo hadi
/// `String` ya Rust inayomilikiwa, ikitupa ujumbe uliotengwa na LLVM baadaye.
pub fn default_target_triple() -> String {
    unsafe {
        let ptr = LLVMGetDefaultTargetTriple();
        assert!(!ptr.is_null(), "LLVMGetDefaultTargetTriple returned null");
        let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
        LLVMDisposeMessage(ptr);
        s
    }
}

// ---------------------------------------------------------------------------
// Majaribio
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_str_roundtrip() {
        let cs = c_str("habari");
        assert_eq!(cs.to_str().unwrap(), "habari");
    }

    #[test]
    #[should_panic(expected = "interior nul byte")]
    fn test_c_str_nul_panics() {
        let _ = c_str("hello\0world");
    }

    #[test]
    fn test_default_target_triple() {
        // Hakikisha tu haitoi hofia na inarudisha mfuato usio tupu.
        let triple = default_target_triple();
        assert!(!triple.is_empty(), "target triple should not be empty");
        eprintln!("default target triple: {}", triple);
    }

    #[test]
    fn test_verify_module_passes_on_empty_module() {
        unsafe {
            let ctx = LLVMContextCreate();
            let cs = c_str("verify_test");
            let module = LLVMModuleCreateWithNameInContext(cs.as_ptr(), ctx);
            let result = verify_module(module);
            // Moduli tupu inapaswa kufanya uhakiki kwa usafi.
            assert!(result.is_ok(), "empty module should verify: {:?}", result.err());
            LLVMDisposeModule(module);
            LLVMContextDispose(ctx);
        }
    }

    #[test]
    fn test_module_create_and_print() {
        unsafe {
            let cs = c_str("test_mod");
            let module = LLVMModuleCreateWithName(cs.as_ptr());
            assert!(!module.is_null());

            let ir_str = LLVMPrintModuleToString(module);
            assert!(!ir_str.is_null());
            let s = CStr::from_ptr(ir_str).to_string_lossy();
            assert!(s.contains("test_mod"), "IR should contain module name");

            LLVMDisposeMessage(ir_str);
            LLVMDisposeModule(module);
        }
    }

    #[test]
    fn test_int_types() {
        unsafe {
            assert!(!LLVMInt1Type().is_null());
            assert!(!LLVMInt8Type().is_null());
            assert!(!LLVMInt32Type().is_null());
            assert!(!LLVMInt64Type().is_null());
        }
    }

    #[test]
    fn test_float_types() {
        unsafe {
            assert!(!LLVMHalfType().is_null());
            assert!(!LLVMFloatType().is_null());
            assert!(!LLVMDoubleType().is_null());
        }
    }

    #[test]
    fn test_void_ptr_types() {
        unsafe {
            assert!(!LLVMVoidType().is_null());
            let i8_ptr = LLVMPointerType(LLVMInt8Type(), 0);
            assert!(!i8_ptr.is_null());
        }
    }
}
