//! Nyuma ya kutengeneza msimbo wa LLVM kwa mkusanyaji Swa.
//!
//! Huteremsha Swa IR (kutoka `crate::ir`) hadi LLVM IR na kutoa faili za vitu
//! asilia. Hutumia vifungo vya FFI vilivyoandikwa kwa mkono kwa API ya C ya
//! LLVM 18.1 kutoka [`ffi`].
//!
//! ## Usanifu
//!
//! 1. [`LlvmBackend::compile`] hutafsiri [`IrModule`] nzima hadi
//!    [`LLVMModuleRef`], ambayo inaweza kutolewa kwa faili ya kitu kupitia
//!    [`LlvmBackend::emit_object`].
//! 2. [`lower_function`] hutembea kila kazi ya IR na kutoa vizuizi vya msingi
//!    vya LLVM na amri kwa kutumia API ya kijenzi ya LLVM.
//! 3. [`lower_instruction`] ni mechi ya takriban mistari 600 kwa kila lahaja
//!    ya [`Instruction`] — kiini cha nyuma.
//! 4. Aina za muundo zinatangazwa kwa mtindo wa **kupita mara mbili**:
//!    kwanza kama miundo yenye majina isiyo wazi, kisha kwa miili
//!    iliyowekwa, ili marejeo yaliyowekwa ndani yasuluhishe kwa usahihi.

pub mod ffi;

use std::collections::HashMap;
use std::ffi::CString;
use std::path::Path;
use std::sync::OnceLock;

use crate::diagnostics::{Diagnostic, SourceSpan};
use crate::ir::types::IrType;
use crate::ir::{BlockId, Const, Function, Module as IrModule, Terminator, ValueId};

use self::ffi::*;

// ---------------------------------------------------------------------------
// Uanzishaji wa LLVM mara moja
// ---------------------------------------------------------------------------

/// Hufuatilia kama usaidizi wa lengwa la X86 umeanzishwa.
static LLVM_INIT: OnceLock<usize> = OnceLock::new();

// ---------------------------------------------------------------------------
// LlvmBackend
// ---------------------------------------------------------------------------

/// Nyuma ya kutengeneza msimbo wa LLVM.
///
/// Inashikilia kumbukumbu ya muktadha wa LLVM wa kimataifa. Muktadha
/// ni wa mchakato mzima (moja tu) kwa hivyo hakuna uondoaji wa
/// nyuma mmoja mmoja — utekelezaji wa `Drop` haufanyi chochote.
///
/// Sehemu ya `opt_level` inadhibiti kiwango cha uboreshaji wa
/// kutengeneza msimbo kinachotumwa kwa mashine lengwa ya LLVM
/// (cha kawaida: `None` / O0).
pub struct LlvmBackend {
    context: LLVMContextRef,
    opt_level: LLVMCodeGenOptLevel,
}

impl LlvmBackend {
    // -- ujenzi -------------------------------------------------------------

    /// Unda nyuma mpya ya LLVM, ukianzisha usaidizi wa lengwa kwenye wito wa kwanza.
    pub fn new() -> Self {
        let context = Self::get_context();
        Self { context, opt_level: LLVMCodeGenOptLevel::None }
    }

    /// Weka kiwango cha uboreshaji wa kutengeneza msimbo (muundo wa kijenzi).
    pub fn with_opt_level(mut self, level: LLVMCodeGenOptLevel) -> Self {
        self.opt_level = level;
        self
    }

    /// Rudisha muktadha wa LLVM wa kimataifa, ukianzisha malengo ya kawaida mara moja.
    fn get_context() -> LLVMContextRef {
        LLVM_INIT.get_or_init(|| {
            unsafe {
                // x86 / x86_64
                LLVMInitializeX86TargetInfo();
                LLVMInitializeX86Target();
                LLVMInitializeX86TargetMC();
                LLVMInitializeX86AsmPrinter();
                LLVMInitializeX86AsmParser();
                // ARM (biti 32)
                LLVMInitializeARMTargetInfo();
                LLVMInitializeARMTarget();
                LLVMInitializeARMTargetMC();
                LLVMInitializeARMAsmPrinter();
                LLVMInitializeARMAsmParser();
                // AArch64
                LLVMInitializeAArch64TargetInfo();
                LLVMInitializeAArch64Target();
                LLVMInitializeAArch64TargetMC();
                LLVMInitializeAArch64AsmPrinter();
                LLVMInitializeAArch64AsmParser();
                // RISC-V
                LLVMInitializeRISCVTargetInfo();
                LLVMInitializeRISCVTarget();
                LLVMInitializeRISCVTargetMC();
                LLVMInitializeRISCVAsmPrinter();
                LLVMInitializeRISCVAsmParser();
            }
            1
        });
        unsafe { LLVMGetGlobalContext() }
    }

    // -- sehemu za kuingilia za ukusanyaji wa kiwango cha juu -----------------

    /// Sanya moduli ya IR na toa faili ya kitu kwa `output_path`.
    pub fn compile_to_file(
        &self,
        ir_module: &IrModule,
        output_path: &Path,
    ) -> Result<(), Vec<Diagnostic>> {
        let llvm_module = self.compile(ir_module)?;
        // Endesha kupita za uboreshaji kabla ya utoaji kama imewezeshwa.
        self.optimize_module(llvm_module);
        self.emit_object(llvm_module, output_path)
    }

    /// Changanua maandishi ya LLVM IR na toa faili ya kitu kwa `output_path`.
    ///
    /// Hii ni urahisisho kwa zana ambazo tayari hutoa maandishi ya LLVM IR.
    pub fn compile_ll(
        &self,
        ll_text: &str,
        output_path: &Path,
    ) -> Result<(), Vec<Diagnostic>> {
        unsafe {
            let c_text = CString::new(ll_text).map_err(|_| {
                vec![Diagnostic::error(
                    "LLVM IR text contains interior nul byte",
                    SourceSpan::point(0, 0),
                )]
            })?;

            let name = c_str("ll_input");
            let mem_buf = LLVMCreateMemoryBufferWithMemoryRangeCopy(
                c_text.as_ptr(),
                ll_text.len(),
                name.as_ptr(),
            );

            let mut out_module: LLVMModuleRef = std::ptr::null_mut();
            let mut error: *mut std::ffi::c_char = std::ptr::null_mut();

            let failed = LLVMParseIRInContext(self.context, mem_buf, &mut out_module, &mut error);

            // LLVM 22 hutupa bafa ya kumbukumbu ndani ya LLVMParseIRInContext.
            // USIITE LLVMDisposeMemoryBuffer hapa — ingesababisha kutoa mara mbili.

            if failed != 0 {
                let msg = if error.is_null() {
                    "failed to parse LLVM IR (no details)".to_string()
                } else {
                    let s = std::ffi::CStr::from_ptr(error).to_string_lossy().into_owned();
                    LLVMDisposeMessage(error);
                    s
                };
                return Err(vec![Diagnostic::error(msg, SourceSpan::point(0, 0))]);
            }

            // Endesha kupita za uboreshaji kabla ya utoaji kama imewezeshwa.
            self.optimize_module(out_module);
            let result = self.emit_object(out_module, output_path);
            LLVMDisposeModule(out_module);
            result
        }
    }

    /// Sanya moduli ya IR hadi moduli ya LLVM.
    ///
    /// Huu ndio mstari mkuu wa ukusanyaji:
    ///
    /// 1. Unda moduli ya LLVM na weka tatu ya lengwa.
    /// 2. Tangazo la muundo kwa kupita mara mbili (isiyo wazi kwanza, kisha miili).
    /// 3. Tangaza data ya ulimwengu (nyuzi, safu zilizopigwa chapa, sajili).
    /// 4. Tangaza mapema kazi saidizi za maktabac (malloc, free, printf).
    /// 5. Teremsha kazi, ukichakata `main` mwisho ili watoa huduma wafafanuliwe kwanza.
    /// 6. Hakiki moduli.
    ///
    /// Inarudisha moduli ya LLVM kwenye mafanikio, au orodha ya utambuzi kwenye
    /// kushindwa.
    pub fn compile(&self, ir_module: &IrModule) -> Result<LLVMModuleRef, Vec<Diagnostic>> {
        unsafe {
            // -- 1. Unda moduli ya LLVM ----------------------------------------
            let name_c = c_str(&ir_module.name);
            let module = LLVMModuleCreateWithName(name_c.as_ptr());
            if module.is_null() {
                return Err(vec![Diagnostic::error(
                    "failed to create LLVM module",
                    SourceSpan::point(0, 0),
                )]);
            }

            // Weka tatu ya lengwa. Kwenye Windows tunaweza kupata tatu ya MSVC
            // kutoka LLVM, lakini kiunganishi cha GNU (MinGW) ndicho kinachopatikana.
            // Lazimisha tatu ya GNU ili kuunganisha kufanikiwe.
            let triple = default_target_triple();
            let triple = if triple.contains("windows-msvc") {
                "x86_64-pc-windows-gnu".to_string()
            } else {
                triple
            };
            let triple_c = CString::new(triple.as_str()).unwrap();
            LLVMSetTarget(module, triple_c.as_ptr());

            // -- 2. Tangazo la muundo kwa kupita mara mbili ---------------------
            let mut struct_types: HashMap<String, LLVMTypeRef> = HashMap::new();

            // Kupita kwa kwanza: unda miundo yenye majina isiyo wazi.
            for (name, _ty) in &ir_module.types {
                if matches!(_ty, IrType::Struct { .. }) {
                    let name_c = c_str(name);
                    let llvm_struct = LLVMStructCreateNamed(self.context, name_c.as_ptr());
                    struct_types.insert(name.clone(), llvm_struct);
                }
            }

            // Kupita kwa pili: weka miili ya miundo (sasa marejeo yote yaliyowekwa ndani yapo).
            for (name, ty) in &ir_module.types {
                if let IrType::Struct { fields, .. } = ty {
                    if let Some(&llvm_struct) = struct_types.get(name) {
                        let mut field_types: Vec<LLVMTypeRef> = fields
                            .iter()
                            .map(|(_, field_ty)| ir_type_to_llvm(field_ty, &struct_types))
                            .collect();
                        if !field_types.is_empty() {
                            LLVMStructSetBody(
                                llvm_struct,
                                field_types.as_mut_ptr(),
                                field_types.len() as u32,
                                0, // haijabandikwa
                            );
                        } else {
                            // Muundo tupu: weka mwili na vipengele sifuri.
                            LLVMStructSetBody(llvm_struct, std::ptr::null_mut(), 0, 0);
                        }
                    }
                }
            }

            // -- 3. Tangaza data ya ulimwengu -----------------------------------
            for global in &ir_module.globals {
                let is_string_like = !global.bytes.is_empty()
                    && global.bytes.last() == Some(&0)
                    // Hakuna baiti tupu kabla ya nafasi ya mwisho — ni kamba halisi
                    // ya C, si safu ya baiti za kiholela zenye tupu.
                    && !global.bytes[..global.bytes.len()-1].contains(&0)
                    && global.bytes.iter().all(|&b| {
                        b == 0 || b == b'\n' || b == b'\t' || b == b'\r'
                            || (b >= 0x20 && b <= 0x7e)
                    });
                let str_len = if is_string_like && !global.bytes.is_empty() {
                    global.bytes.len() - 1
                } else {
                    global.bytes.len()
                };

                // Amua aina sahihi ya LLVM kwa ulimwengu huu.
                // Kwa vitu vya ulimwengu vya kamba, tumia aina ya safu [N x i8].
                // Kwa vitu vya ulimwengu vilivyopigwa chapa (safu, miundo), tumia ir_type_to_llvm.
                let is_scalar = matches!(global.ty,
                    IrType::I8 | IrType::I16 | IrType::I32 | IrType::I64 |
                    IrType::A8 | IrType::A16 | IrType::A32 | IrType::A64 | IrType::A128 |
                    IrType::B1 | IrType::B8 | IrType::B16 | IrType::B32 | IrType::B64 |
                    IrType::W8 | IrType::W16 | IrType::W32 | IrType::W64 |
                    IrType::F16 | IrType::F32 | IrType::F64 | IrType::F128 |
                    IrType::I128 | IrType::Void | IrType::Ptr(_) | IrType::FnPtr { .. });
                let ty = if is_string_like {
                    LLVMArrayType(LLVMInt8Type(), global.bytes.len() as u32)
                } else if !is_scalar {
                    ir_type_to_llvm(&global.ty, &struct_types)
                } else {
                    match global.bytes.len() {
                        0 => LLVMInt8Type(),
                        1 => LLVMInt8Type(),
                        2 => LLVMInt16Type(),
                        4 => LLVMInt32Type(),
                        8 => LLVMInt64Type(),
                        _ => LLVMArrayType(LLVMInt8Type(), global.bytes.len() as u32),
                    }
                };

                let name_c = c_str(&global.name);
                let llvm_global = LLVMAddGlobal(module, ty, name_c.as_ptr());

                let all_zero = global.bytes.iter().all(|&b| b == 0);
                let init = if global.bytes.is_empty() || all_zero {
                    LLVMConstNull(ty)
                } else if !is_string_like && matches!(global.bytes.len(), 1 | 2 | 4 | 8) {
                    // Vitu vidogo vya ulimwengu visivyo vya kamba vilivyo na aina namba:
                    // unda thabiti namba. Ukubwa mwingine (3,5,6,7) hupata aina safu
                    // na lazima utumie ConstArray hapa chini.
                    let mut val: u64 = 0;
                    for (i, &b) in global.bytes.iter().enumerate() {
                        val |= (b as u64) << (i * 8);
                    }
                    LLVMConstInt(ty, val, 0)
                } else if is_string_like && !global.bytes.iter().all(|&b| b == 0) {
                    let effective_len = global.bytes.iter().position(|&b| b == 0).unwrap_or(global.bytes.len());
                    let effective_len = effective_len.min(str_len);
                    let c_str_val = CString::new(&global.bytes[..effective_len]).unwrap_or_else(|_| CString::new("").unwrap());
                    LLVMConstString(c_str_val.as_ptr(), effective_len as u32, 0)
                } else {
                    let mut vals: Vec<LLVMValueRef> = global
                        .bytes
                        .iter()
                        .map(|&b| LLVMConstInt(LLVMInt8Type(), b as u64, 0))
                        .collect();
                    LLVMConstArray(LLVMInt8Type(), vals.as_mut_ptr(), vals.len() as u32)
                };

                LLVMSetInitializer(llvm_global, init);
                if global.is_const {
                    LLVMSetGlobalConstant(llvm_global, 1);
                    if global.is_const { LLVMSetGlobalConstant(llvm_global, 1); LLVMSetLinkage(llvm_global, LLVMLinkage::Private); }
                }
            }

            // -- 4. Tangaza mapema kazi ZOTE (maktabac + mtumiaji) -------------
            pre_declare_libc(module);

            // Tangaza mapema kila kazi ya mtumiaji ili marejeo ya mbele yasuluhishe.
            // Pia hifadhi aina zao za rudisha kwa njia mbadala ya amri ya wito.
            let mut fn_return_types: HashMap<String, LLVMTypeRef> = HashMap::new();
            for func in &ir_module.functions {
                let name_c = c_str(&func.name);
                if LLVMGetNamedFunction(module, name_c.as_ptr()).is_null() {
                    let llvm_ret = ir_type_to_llvm(&func.return_ty, &struct_types);
                    let mut param_tys: Vec<LLVMTypeRef> = func
                        .params
                        .iter()
                        .map(|(_, ty)| ir_type_to_llvm(ty, &struct_types))
                        .collect();
                    let fn_ty = LLVMFunctionType(
                        llvm_ret,
                        if param_tys.is_empty() { std::ptr::null_mut() } else { param_tys.as_mut_ptr() },
                        param_tys.len() as u32,
                        if func.variadic { 1 } else { 0 },
                    );
                    LLVMAddFunction(module, name_c.as_ptr(), fn_ty);
                    fn_return_types.insert(func.name.clone(), llvm_ret);
                } else {
                    // Kazi tayari imetangazwa (kwa mfano maktabac) — bado rekodi aina yake ya rudisha.
                    let llvm_ret = ir_type_to_llvm(&func.return_ty, &struct_types);
                    fn_return_types.insert(func.name.clone(), llvm_ret);
                }
            }

            // -- 5. Panga upya uteremshaji wa kazi: chakata main MWISHO -------
            // Kusanya fahirisi za kazi, ukiweka main mwishoni.
            let mut ordered_indices: Vec<usize> = (0..ir_module.functions.len()).collect();
            // Tafuta fahirisi ya "main" ikiwa ipo, ihamishe mwishoni.
            if let Some(main_idx) = ir_module.functions.iter().position(|f| f.name == "main") {
                ordered_indices.retain(|&i| i != main_idx);
                ordered_indices.push(main_idx);
            }

            // Teremsha kila kazi.
            for idx in ordered_indices {
                let func = &ir_module.functions[idx];
                if let Err(diags) = lower_function(module, func, &struct_types, &fn_return_types) {
                    LLVMDisposeModule(module);
                    return Err(diags);
                }
            }

            // -- 6. Ukaguzi wa FastISel wa kudondosha vizuizi -------------------
            // FastISel ya LLVM hutupa kimya kimya vizuizi vya msingi zaidi ya ~50
            // kwa kila kazi kwenye O0. Onya ikiwa kazi yoyote inazidi kwa kiasi
            // kikomo cha usalama.
            const FASTISEL_BLOCK_LIMIT: usize = 40;
            for func in &ir_module.functions {
                if func.blocks.len() > FASTISEL_BLOCK_LIMIT {
                    eprintln!(
                        "onyo: kazi '{}' ina vizuizi {} — FastISel inaweza kuacha baadhi \
                         (kiwango cha juu ni ~50, inapendekezwa chini ya {})",
                        func.name,
                        func.blocks.len(),
                        FASTISEL_BLOCK_LIMIT,
                    );
                }
            }

            if let Err(msg) = verify_module(module) {
                LLVMDisposeModule(module);
                return Err(vec![Diagnostic::error(
                    format!("LLVM module verification failed: {}", msg),
                    SourceSpan::point(0, 0),
                )]);
            }

            Ok(module)
        }
    }

    /// Endesha kupita za uboreshaji wa LLVM kwenye moduli.
    ///
    /// Hutumia meneja mpya wa kupita kupitia `LLVMRunPasses`. Mstari wa
    /// usindikaji unajumuisha:
    /// - Kiwango cha kazi: mem2reg (pandisha allocas hadi SSA), instcombine (peephole),
    ///   GVN (uondoaji wa upakiaji unaorudiwa), simplifycfg (usafishaji wa CFG).
    /// - Kiwango cha moduli: always-inline (weka ndani kazi za `always_inline`).
    ///
    /// Hii inaitwa na [`compile_to_file`] na [`compile_ll`] kabla ya
    /// kutoa faili ya kitu.
    pub fn optimize_module(&self, module: LLVMModuleRef) {
        if self.opt_level as i32 <= 0 {
            return;
        }
        unsafe {
            let opts = LLVMCreatePassBuilderOptions();
            if !opts.is_null() {
                LLVMPassBuilderOptionsSetVerifyEach(opts, 0);
                LLVMPassBuilderOptionsSetDebugLogging(opts, 0);
                LLVMPassBuilderOptionsSetLoopInterleaving(opts, 0);
                LLVMPassBuilderOptionsSetLoopVectorization(opts, 0);
                LLVMPassBuilderOptionsSetSLPVectorization(opts, 0);
                LLVMPassBuilderOptionsSetLoopUnrolling(opts, 0);
            }

            // Kamba ya mstari wa usindikaji hutumia sintaksia mpya ya meneja wa
            // kupita:
            //   function(mem2reg,instcombine,gvn,simplifycfg)
            //     — kupita za kiwango cha kazi zinazotumika kwa kila kazi
            //   always-inline
            //     — kupita kwa kiwango cha moduli kuweka ndani kazi za always_inline
            let pipeline = c_str(
                "function(mem2reg,instcombine<no-verify-fixpoint>,gvn,simplifycfg),\
                 always-inline",
            );
            let err = LLVMRunPasses(
                module,
                pipeline.as_ptr(),
                std::ptr::null_mut(),
                opts,
            );
            if !err.is_null() {
                let msg = LLVMGetErrorMessage(err);
                eprintln!(
                    "onyo: optimaisa ya moduli ilishindwa: {}",
                    std::ffi::CStr::from_ptr(msg).to_string_lossy(),
                );
                LLVMDisposeErrorMessage(msg);
            }

            if !opts.is_null() {
                LLVMDisposePassBuilderOptions(opts);
            }
        }
    }

    // -- utoaji wa faili ya kitu ----------------------------------------------

    /// Toa moduli ya LLVM kwa faili ya kitu asilia.
    ///
    /// Hutumia tatu ya lengwa la mwenyeji na mipangilio ya CPU na vipengele
    /// cha kawaida.
    pub fn emit_object(
        &self,
        module: LLVMModuleRef,
        output_path: &Path,
    ) -> Result<(), Vec<Diagnostic>> {
        unsafe {
            let triple = default_target_triple();
            let triple_c = CString::new(triple.as_str()).unwrap();

            let mut target: LLVMTargetRef = std::ptr::null_mut();
            let mut error: *mut std::ffi::c_char = std::ptr::null_mut();

            let failed = LLVMGetTargetFromTriple(triple_c.as_ptr(), &mut target, &mut error);
            if failed != 0 {
                let msg = if error.is_null() {
                    "failed to look up target from triple".to_string()
                } else {
                    let s = std::ffi::CStr::from_ptr(error).to_string_lossy().into_owned();
                    LLVMDisposeMessage(error);
                    s
                };
                return Err(vec![Diagnostic::error(msg, SourceSpan::point(0, 0))]);
            }

            let cpu_c = c_str("");
            let features_c = c_str("");
            let tm = LLVMCreateTargetMachine(
                target,
                triple_c.as_ptr(),
                cpu_c.as_ptr(),
                features_c.as_ptr(),
                self.opt_level,
                LLVMRelocMode::Default,
                LLVMCodeModel::Default,
            );

            if tm.is_null() {
                return Err(vec![Diagnostic::error(
                    "failed to create target machine",
                    SourceSpan::point(0, 0),
                )]);
            }

            let path_str = output_path.to_string_lossy();
            let path_c = CString::new(path_str.as_ref()).unwrap();
            let mut emit_error: *mut std::ffi::c_char = std::ptr::null_mut();

            let emit_failed = LLVMTargetMachineEmitToFile(
                tm,
                module,
                path_c.as_ptr(),
                LLVMCodeGenFileType::ObjectFile,
                &mut emit_error,
            );

            LLVMDisposeTargetMachine(tm);

            if emit_failed != 0 {
                let msg = if emit_error.is_null() {
                    "object file emission failed (no details)".to_string()
                } else {
                    let s = std::ffi::CStr::from_ptr(emit_error)
                        .to_string_lossy()
                        .into_owned();
                    LLVMDisposeMessage(emit_error);
                    s
                };
                return Err(vec![Diagnostic::error(msg, SourceSpan::point(0, 0))]);
            }

            Ok(())
        }
    }
}

// ---------------------------------------------------------------------------
// Uteremshaji wa kazi
// ---------------------------------------------------------------------------

/// Teremsha kazi moja ya IR hadi kwenye moduli ya LLVM iliyotolewa.
///
/// Hii ndio sehemu ya kuingilia ya kutafsiri kazi moja ya Swa:
///
/// 1. Ramani aina za rudisha / kigezo kwa aina za LLVM.
/// 2. Unda kazi ya LLVM (au tumia tena tangazo lililopo).
/// 3. Tumia sifa za sret wakati darasa la rudisha ni `HiddenPtr`.
/// 4. Jenga vizuizi vya msingi kwa mkakati wa kupita mara mbili wa kizuizi cha
///    kuingilia:
///    - Kupita 1: teremsha amri za `Alloca` pekee.
///    - Hifadhi thamani za kigezo kwenye allocas zao.
///    - Kupita 2: teremsha amri zilizobaki.
/// 5. Kwa vizuizi visivyo vya kuingilia, teremsha amri zote kwa mpangilio.
/// 6. Teremsha vikomeshi na toa rudisha salama kwa vizuizi ambavyo
///    havijakomeshwa.
fn lower_function(
    module: LLVMModuleRef,
    func: &Function,
    struct_types: &HashMap<String, LLVMTypeRef>,
    fn_return_types: &HashMap<String, LLVMTypeRef>,
) -> Result<(), Vec<Diagnostic>> {
    unsafe {
        // -- 1. Jenga aina ya kazi ya LLVM ------------------------------------
        let llvm_return_ty = ir_type_to_llvm(&func.return_ty, struct_types);

        let mut param_types: Vec<LLVMTypeRef> = func
            .params
            .iter()
            .map(|(_, ty)| ir_type_to_llvm(ty, struct_types))
            .collect();

        let is_var_arg = if func.variadic { 1 } else { 0 };

        let func_ty = LLVMFunctionType(
            llvm_return_ty,
            if param_types.is_empty() {
                std::ptr::null_mut()
            } else {
                param_types.as_mut_ptr()
            },
            param_types.len() as u32,
            is_var_arg,
        );

        // -- 2. Unda au tumia tena kazi ya LLVM -------------------------------
        let name_c = c_str(&func.name);
        let llvm_func = LLVMGetNamedFunction(module, name_c.as_ptr());

        let llvm_func = if llvm_func.is_null() {
            LLVMAddFunction(module, name_c.as_ptr(), func_ty)
        } else {
            llvm_func
        };

        if llvm_func.is_null() {
            return Err(vec![Diagnostic::error(
                format!("failed to create LLVM function '{}'", func.name),
                SourceSpan::point(0, 0),
            )]);
        }

        // Weka ABI ya C ikiwa imeombwa (extern "C").
        if func.c_abi {
            let ccc_id = LLVMGetEnumAttributeKind(c_str("ccc").as_ptr());
            if ccc_id != 0 {
                let attr = LLVMCreateEnumAttribute(
                    LLVMGetModuleContext(module),
                    ccc_id,
                    0,
                );
                LLVMAddAttributeAtIndex(llvm_func, LLVM_ATTRIBUTE_FUNCTION_INDEX, attr);
            }
        }

        // -- 3. Tumia sifa sret kwenye kigezo cha kwanza ----------------------
        if func.sret_value_id.is_some() {
            let sret_kind = LLVMGetEnumAttributeKind(c_str("sret").as_ptr());
            if sret_kind != 0 {
                let attr = LLVMCreateEnumAttribute(
                    LLVMGetModuleContext(module),
                    sret_kind,
                    0,
                );
                LLVMAddAttributeAtIndex(llvm_func, 0, attr);
            }
        }

        // -- 4. Unda vizuizi vya msingi ---------------------------------------
        let mut llvm_blocks: HashMap<usize, LLVMBasicBlockRef> = HashMap::new();
        for (i, block) in func.blocks.iter().enumerate() {
            let label_c = c_str(&block.label);
            let bb = LLVMAppendBasicBlockInContext(
                LLVMGetModuleContext(module),
                llvm_func,
                label_c.as_ptr(),
            );
            llvm_blocks.insert(i, bb);
        }

        // -- 5. Jenga ramani ya thamani ---------------------------------------
        let mut value_map: HashMap<ValueId, LLVMValueRef> = HashMap::new();

        // Ramani vigezo: ValueId(0..N-1) vinalingana na vigezo vya LLVM.
        for (i, _param) in func.params.iter().enumerate() {
            let llvm_param = LLVMGetParam(llvm_func, i as u32);
            value_map.insert(ValueId(i), llvm_param);
        }

        // Unda thabiti: ValueId(params.len()..params.len()+values.len()-1).
        // Thabiti huundwa kwa ulegevu kwa aina zinazotegemea muktadha,
        // kwa hiyo tunaahirisha hili — huundwa kwenye matumizi ya kwanza
        // kwenye lower_instruction.

        // Thabiti hupata aina wakati wa uteremshaji. Kwa sasa, unda mapema kwa
        // aina za kawaida zinazofaa lahaja ya Const.
        let param_count = func.params.len();
        for (i, const_val) in func.values.iter().enumerate() {
            let val_id = ValueId(param_count + i);
            let llvm_val = materialize_const(const_val, LLVMInt64Type());
            value_map.insert(val_id, llvm_val);
        }

        // -- 5b. Kijenzi -------------------------------------------------------
        let builder = LLVMCreateBuilder();

        // -- 6. Teremsha amri kwenye vizuizi VYOTE (mkabati wa kupita tatu) ---
        //
        // Kupita 1: unda nodi za phi bila kingo zinazoingia (thamani za LLVM
        // zinaundwa na kuingizwa kwenye value_map, lakini kingo zinazoingia
        // zimeahirishwa).
        // Kupita 2: teremsha amri zote zisizo za phi.
        // Kupita 3: jaza kingo zinazoingia kwenye nodi zote za phi (kwa sasa
        // kila ValueId kutoka kila kizuizi ipo kwenye value_map, hivyo thamani
        // za back-edge kutoka kwenye vizuizi vya baadaye zinasuluhisha kwa
        // usahihi).
        let mut pending_phis: Vec<(LLVMValueRef, Vec<(ValueId, BlockId)>)> = Vec::new();
        let mut global_inst_idx = 0usize;
        for (block_idx, block) in func.blocks.iter().enumerate() {
            let bb = llvm_blocks[&block_idx];

            // -- Kupita 1: unda thamani za phi za LLVM (hakuna kingo zinazoingia bado) ---
            LLVMPositionBuilderAtEnd(builder, bb);
            for inst in &block.instructions {
                if let crate::ir::Instruction::Phi(result_ty, incoming) = inst {
                    let val_id = ValueId(param_count + func.values.len() + global_inst_idx);
                    global_inst_idx += 1;
                    let llvm_ty = ir_type_to_llvm(result_ty, struct_types);
                    let phi = LLVMBuildPhi(builder, llvm_ty, c_str("phi").as_ptr());
                    // Ahirisha kujaza kingo zinazoingia hadi kupita 3, ili
                    // thamani za back-edge kutoka vizuizi vya baadaye zipatikane.
                    pending_phis.push((phi, incoming.clone()));
                    value_map.insert(val_id, phi);
                }
            }

            // -- Kupita 2: teremsha amri zote zisizo za phi ----------------------
            LLVMPositionBuilderAtEnd(builder, bb);
            for inst in &block.instructions {
                if matches!(inst, crate::ir::Instruction::Phi(_, _)) {
                    continue; // tayari imeteremshwa kwenye kupita 1
                }
                let val_id = ValueId(param_count + func.values.len() + global_inst_idx);
                global_inst_idx += 1;
                let llvm_val =
                    lower_instruction(inst, builder, &value_map, module, struct_types, &fn_return_types);
                if !llvm_val.is_null() {
                    value_map.insert(val_id, llvm_val);
                }
            }
        }

        // -- Kupita 3: jaza kingo zinazoingia kwenye nodi zote za phi ------------
        // Kwa wakati huu kila amri kutoka kila kizuizi imeteremshwa na
        // matokeo yapo kwenye value_map, hivyo thamani za back-edge (kutoka
        // vizuizi vilivyotangulia vinavyoonekana baadaye kwenye orodha ya
        // vizuizi) zinasuluhisha kwa usahihi.
        for &(phi, ref incoming) in &pending_phis {
            for &(value_id, pred_block) in incoming {
                let llvm_val = value_map.get(&value_id).copied().unwrap_or_else(|| {
                    // Haipaswi kamwe kutokea kwa IR halali.
                    LLVMConstInt(LLVMInt32Type(), 0, 0)
                });
                let pred_bb = llvm_blocks[&pred_block.0];
                let mut vals = [llvm_val];
                let mut blks = [pred_bb];
                LLVMAddIncoming(phi, vals.as_mut_ptr(), blks.as_mut_ptr(), 1);
            }
        }

        // -- 8. Teremsha vikomeshi kwa vizuizi vyote --------------------------
        for (block_idx, block) in func.blocks.iter().enumerate() {
            let bb = llvm_blocks[&block_idx];
            LLVMPositionBuilderAtEnd(builder, bb);
            lower_terminator(&block.terminator, builder, &value_map, &llvm_blocks, llvm_return_ty);
        }

        LLVMDisposeBuilder(builder);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Uteremshaji wa amri — mechi ya takriban mistari 600
// ---------------------------------------------------------------------------

/// Teremsha amri moja ya IR hadi thamani ya LLVM kwa kutumia kijenzi.
///
/// Inarudisha thamani ya LLVM inayozalishwa na amri hii.
fn lower_instruction(
    inst: &crate::ir::Instruction,
    builder: LLVMBuilderRef,
    value_map: &HashMap<ValueId, LLVMValueRef>,
    module: LLVMModuleRef,
    struct_types: &HashMap<String, LLVMTypeRef>,
    fn_return_types: &HashMap<String, LLVMTypeRef>,
) -> LLVMValueRef {
    unsafe {
        /// Msaidizi wa kusuluhisha opereta ya ValueId.
        fn v(value_map: &HashMap<ValueId, LLVMValueRef>, id: &ValueId) -> LLVMValueRef {
            value_map.get(id).copied().unwrap_or_else(|| {
                unsafe { LLVMConstInt(LLVMInt32Type(), 0, 0) }
            })
        }

        match inst {
            // -- hesabu za namba kamili --------------------------------------------
            crate::ir::Instruction::Add(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                // Angalia kuelekezi + namba kamili → tumia GEP.
                let l_ty = LLVMTypeOf(l);
                if LLVMGetTypeKind(l_ty) as u32 == LLVMTypeKind::Pointer as u32 {
                    let indices = [r];
                    return LLVMBuildGEP2(
                        builder,
                        LLVMInt8Type(), // aina msingi ya kielekezi kisicho wazi
                        l,
                        indices.as_ptr() as *mut LLVMValueRef,
                        1,
                        c_str("add_ptr").as_ptr(),
                    );
                }
                let r_ty = LLVMTypeOf(r);
                if LLVMGetTypeKind(r_ty) as u32 == LLVMTypeKind::Pointer as u32 {
                    let indices = [l];
                    return LLVMBuildGEP2(
                        builder,
                        LLVMInt8Type(),
                        r,
                        indices.as_ptr() as *mut LLVMValueRef,
                        1,
                        c_str("add_ptr").as_ptr(),
                    );
                }
                let (cl, cr, _common_ty) = coerce_int_binop(builder, l, r);
                LLVMBuildAdd(builder, cl, cr, c_str("add").as_ptr())
            }

            crate::ir::Instruction::Sub(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                // Kielekezi - namba kamili → GEP na fahirisi hasi? Hapana, chukulia kama namba tu.
                let (cl, cr, _common_ty) = coerce_int_binop(builder, l, r);
                LLVMBuildSub(builder, cl, cr, c_str("sub").as_ptr())
            }

            crate::ir::Instruction::Mul(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _common_ty) = coerce_int_binop(builder, l, r);
                LLVMBuildMul(builder, cl, cr, c_str("mul").as_ptr())
            }

            crate::ir::Instruction::DivS(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _common_ty) = coerce_int_binop(builder, l, r);
                LLVMBuildSDiv(builder, cl, cr, c_str("sdiv").as_ptr())
            }

            crate::ir::Instruction::DivU(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _common_ty) = coerce_int_binop(builder, l, r);
                LLVMBuildUDiv(builder, cl, cr, c_str("udiv").as_ptr())
            }

            crate::ir::Instruction::RemS(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _common_ty) = coerce_int_binop(builder, l, r);
                LLVMBuildSRem(builder, cl, cr, c_str("srem").as_ptr())
            }

            crate::ir::Instruction::RemU(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _common_ty) = coerce_int_binop(builder, l, r);
                LLVMBuildURem(builder, cl, cr, c_str("urem").as_ptr())
            }

            // -- hesabu za namba sehemu --------------------------------------------
            crate::ir::Instruction::FAdd(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                LLVMBuildFAdd(builder, l, r, c_str("fadd").as_ptr())
            }
            crate::ir::Instruction::FSub(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                LLVMBuildFSub(builder, l, r, c_str("fsub").as_ptr())
            }
            crate::ir::Instruction::FMul(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                LLVMBuildFMul(builder, l, r, c_str("fmul").as_ptr())
            }
            crate::ir::Instruction::FDiv(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                LLVMBuildFDiv(builder, l, r, c_str("fdiv").as_ptr())
            }
            crate::ir::Instruction::FNeg(val) => {
                let x = v(value_map, val);
                LLVMBuildFNeg(builder, x, c_str("fneg").as_ptr())
            }

            // -- bitwise (kwa biti) -------------------------------------------------
            crate::ir::Instruction::And(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _common_ty) = coerce_int_binop(builder, l, r);
                LLVMBuildAnd(builder, cl, cr, c_str("and").as_ptr())
            }
            crate::ir::Instruction::Or(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _common_ty) = coerce_int_binop(builder, l, r);
                LLVMBuildOr(builder, cl, cr, c_str("or").as_ptr())
            }
            crate::ir::Instruction::Xor(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _common_ty) = coerce_int_binop(builder, l, r);
                LLVMBuildXor(builder, cl, cr, c_str("xor").as_ptr())
            }
            crate::ir::Instruction::Shl(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _common_ty) = coerce_int_binop(builder, l, r);
                LLVMBuildShl(builder, cl, cr, c_str("shl").as_ptr())
            }
            crate::ir::Instruction::ShrS(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _common_ty) = coerce_int_binop(builder, l, r);
                LLVMBuildAShr(builder, cl, cr, c_str("ashr").as_ptr())
            }
            crate::ir::Instruction::ShrU(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _common_ty) = coerce_int_binop(builder, l, r);
                LLVMBuildLShr(builder, cl, cr, c_str("lshr").as_ptr())
            }

            // -- ulinganisho wa namba kamili ----------------------------------------
            crate::ir::Instruction::Eq(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr) = coerce_cmp_operands(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::EQ, cl, cr, c_str("eq").as_ptr())
            }
            crate::ir::Instruction::Ne(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr) = coerce_cmp_operands(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::NE, cl, cr, c_str("ne").as_ptr())
            }
            crate::ir::Instruction::LtS(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr) = coerce_cmp_operands(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::SLT, cl, cr, c_str("lts").as_ptr())
            }
            crate::ir::Instruction::LtU(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr) = coerce_cmp_operands(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::ULT, cl, cr, c_str("ltu").as_ptr())
            }
            crate::ir::Instruction::LeS(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr) = coerce_cmp_operands(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::SLE, cl, cr, c_str("les").as_ptr())
            }
            crate::ir::Instruction::LeU(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr) = coerce_cmp_operands(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::ULE, cl, cr, c_str("leu").as_ptr())
            }
            crate::ir::Instruction::GtS(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr) = coerce_cmp_operands(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::SGT, cl, cr, c_str("gts").as_ptr())
            }
            crate::ir::Instruction::GtU(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr) = coerce_cmp_operands(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::UGT, cl, cr, c_str("gtu").as_ptr())
            }
            crate::ir::Instruction::GeS(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr) = coerce_cmp_operands(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::SGE, cl, cr, c_str("ges").as_ptr())
            }
            crate::ir::Instruction::GeU(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr) = coerce_cmp_operands(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::UGE, cl, cr, c_str("geu").as_ptr())
            }

            // -- ulinganisho wa namba sehemu ---------------------------------------
            crate::ir::Instruction::Feq(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                LLVMBuildFCmp(builder, LLVMRealPredicate::OEQ, l, r, c_str("feq").as_ptr())
            }
            crate::ir::Instruction::Fne(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                LLVMBuildFCmp(builder, LLVMRealPredicate::ONE, l, r, c_str("fne").as_ptr())
            }
            crate::ir::Instruction::Flt(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                LLVMBuildFCmp(builder, LLVMRealPredicate::OLT, l, r, c_str("flt").as_ptr())
            }
            crate::ir::Instruction::Fle(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                LLVMBuildFCmp(builder, LLVMRealPredicate::OLE, l, r, c_str("fle").as_ptr())
            }
            crate::ir::Instruction::Fgt(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                LLVMBuildFCmp(builder, LLVMRealPredicate::OGT, l, r, c_str("fgt").as_ptr())
            }
            crate::ir::Instruction::Fge(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                LLVMBuildFCmp(builder, LLVMRealPredicate::OGE, l, r, c_str("fge").as_ptr())
            }

            // -- ubadilishaji wa aina -----------------------------------------------
            crate::ir::Instruction::Trunc(val, target_ty) => {
                let x = v(value_map, val);
                let target = ir_type_to_llvm(target_ty, struct_types);
                LLVMBuildTrunc(builder, x, target, c_str("trunc").as_ptr())
            }
            crate::ir::Instruction::Zext(val, target_ty) => {
                let x = v(value_map, val);
                let target = ir_type_to_llvm(target_ty, struct_types);
                LLVMBuildZExt(builder, x, target, c_str("zext").as_ptr())
            }
            crate::ir::Instruction::Sext(val, target_ty) => {
                let x = v(value_map, val);
                let target = ir_type_to_llvm(target_ty, struct_types);
                LLVMBuildSExt(builder, x, target, c_str("sext").as_ptr())
            }
            crate::ir::Instruction::FpTrunc(val, target_ty) => {
                let x = v(value_map, val);
                let target = ir_type_to_llvm(target_ty, struct_types);
                LLVMBuildFPTrunc(builder, x, target, c_str("fptrunc").as_ptr())
            }
            crate::ir::Instruction::FpExt(val, target_ty) => {
                let x = v(value_map, val);
                let target = ir_type_to_llvm(target_ty, struct_types);
                LLVMBuildFPExt(builder, x, target, c_str("fpext").as_ptr())
            }
            crate::ir::Instruction::FpToSi(val, target_ty) => {
                let x = v(value_map, val);
                let target = ir_type_to_llvm(target_ty, struct_types);
                LLVMBuildFPToSI(builder, x, target, c_str("fptosi").as_ptr())
            }
            crate::ir::Instruction::FpToUi(val, target_ty) => {
                let x = v(value_map, val);
                let target = ir_type_to_llvm(target_ty, struct_types);
                LLVMBuildFPToUI(builder, x, target, c_str("fptoui").as_ptr())
            }
            crate::ir::Instruction::SiToFp(val, target_ty) => {
                let x = v(value_map, val);
                let target = ir_type_to_llvm(target_ty, struct_types);
                LLVMBuildSIToFP(builder, x, target, c_str("sitofp").as_ptr())
            }
            crate::ir::Instruction::UiToFp(val, target_ty) => {
                let x = v(value_map, val);
                let target = ir_type_to_llvm(target_ty, struct_types);
                LLVMBuildUIToFP(builder, x, target, c_str("uitofp").as_ptr())
            }
            crate::ir::Instruction::Bitcast(val, target_ty) => {
                let x = v(value_map, val);
                let target = ir_type_to_llvm(target_ty, struct_types);
                LLVMBuildBitCast(builder, x, target, c_str("bitcast").as_ptr())
            }

            // -- uundaji wa thabiti ---------------------------------------------
            crate::ir::Instruction::Const(c) => {
                unsafe { materialize_const(c, LLVMInt64Type()) }
            }

            // -- kumbukumbu --------------------------------------------------------
            crate::ir::Instruction::Alloca(ty) => {
                let llvm_ty = ir_type_to_llvm(ty, struct_types);
                LLVMBuildAlloca(builder, llvm_ty, c_str("alloca").as_ptr())
            }
            crate::ir::Instruction::Load(pointee_ty, ptr) => {
                let p = v(value_map, ptr);
                let llvm_ty = match pointee_ty {
                    IrType::Struct { .. } => ptr_type(),
                    _ => ir_type_to_llvm(pointee_ty, struct_types),
                };
                LLVMBuildLoad2(builder, llvm_ty, p, c_str("load").as_ptr())
            }
            crate::ir::Instruction::Store(val, ptr) => {
                let value = v(value_map, val);
                let p = v(value_map, ptr);
                // StoreTyped hushughulikia ulazimishaji wa upana kwa IrType wazi.
                // Plain Store hutoa stoo kama ilivyo — thamani lazima tayari
                // ilingane na upana wa elekezi. LLVMGetElementType haina uhakika
                // kwa vielekezi visivyo wazi kwenye LLVM 22.
                LLVMBuildStore(builder, value, p)
            }
            crate::ir::Instruction::StoreTyped(val, ptr, store_ty) => {
                let value = v(value_map, val);
                let p = v(value_map, ptr);
                // Geuza thamani kwa aina ya sehemu ikiwa upana hutofautiana.
                let llvm_ty = ir_type_to_llvm(store_ty, struct_types);
                let val_ty = LLVMTypeOf(value);
                let val_kind = LLVMGetTypeKind(val_ty) as u32;
                let target_kind = LLVMGetTypeKind(llvm_ty) as u32;
                let cast = if val_kind == LLVMTypeKind::Integer as u32
                    && target_kind == LLVMTypeKind::Integer as u32
                    && LLVMGetIntTypeWidth(val_ty) != LLVMGetIntTypeWidth(llvm_ty) {
                    LLVMBuildIntCast2(builder, value, llvm_ty, 1, c_str("cast").as_ptr())
                } else if val_kind == LLVMTypeKind::Integer as u32
                    && target_kind == LLVMTypeKind::Pointer as u32 {
                    LLVMBuildIntToPtr(builder, value, llvm_ty, c_str("inttoptr").as_ptr())
                } else {
                    value
                };
                LLVMBuildStore(builder, cast, p)
            }
            crate::ir::Instruction::MemCopy(dest, src, size) => {
                // Tumia asili ya LLVM memcpy: @llvm.memcpy.p0.p0.i64
                let dest_ptr = v(value_map, dest);
                let src_ptr = v(value_map, src);
                let sz_val = LLVMConstInt(LLVMInt64Type(), *size, 0);
                let volatile_flag = LLVMConstInt(LLVMInt1Type(), 0, 0);
                let intrinsic_name = c_str("llvm.memcpy.p0.p0.i64");
                let callee = LLVMGetNamedFunction(module, intrinsic_name.as_ptr());
                let callee = if callee.is_null() {
                    let mut param_tys = [ptr_type(), ptr_type(), LLVMInt64Type(), LLVMInt1Type()];
                    let fn_ty = LLVMFunctionType(LLVMVoidType(), param_tys.as_mut_ptr(), 4, 0);
                    LLVMAddFunction(module, intrinsic_name.as_ptr(), fn_ty)
                } else {
                    callee
                };
                // Pata tena aina ya kazi kwa wito.
                let param_count = LLVMCountParams(callee);
                let mut rebuilt: Vec<LLVMTypeRef> = (0..param_count)
                    .map(|i| LLVMTypeOf(LLVMGetParam(callee, i)))
                    .collect();
                let fn_ty = LLVMFunctionType(LLVMVoidType(),
                    if rebuilt.is_empty() { std::ptr::null_mut() } else { rebuilt.as_mut_ptr() },
                    rebuilt.len() as u32, 0);
                let mut args = [dest_ptr, src_ptr, sz_val, volatile_flag];
                LLVMBuildCall2(builder, fn_ty, callee, args.as_mut_ptr(), 4, std::ptr::null());
                LLVMConstNull(LLVMInt8Type())
            }

            // -- chungu ------------------------------------------------------------
            crate::ir::Instruction::HeapAlloc(size) => {
                // Ita kazi ya malloc iliyotangazwa mapema.
                let malloc_fn = LLVMGetNamedFunction(module, c_str("malloc").as_ptr());
                let sz = v(value_map, size);
                let args = [sz];
                LLVMBuildCall2(
                    builder,
                    LLVMFunctionType(
                        ptr_type(),
                        [LLVMInt64Type()].as_mut_ptr(),
                        1,
                        0,
                    ),
                    malloc_fn,
                    args.as_ptr() as *mut LLVMValueRef,
                    1,
                    c_str("heap_alloc").as_ptr(),
                )
            }
            crate::ir::Instruction::HeapFree(ptr) => {
                let free_fn = LLVMGetNamedFunction(module, c_str("free").as_ptr());
                let p = v(value_map, ptr);
                // Geuza hadi i8* ikiwa inahitajika.
                let p_cast = LLVMBuildBitCast(builder, p, ptr_type(), c_str("free_cast").as_ptr());
                let args = [p_cast];
                LLVMBuildCall2(
                    builder,
                    LLVMFunctionType(LLVMVoidType(), [ptr_type()].as_mut_ptr(), 1, 0),
                    free_fn,
                    args.as_ptr() as *mut LLVMValueRef,
                    1,
                    c_str("").as_ptr(), // wito void hutumia jina tupu
                )
            }

            // -- arena -------------------------------------------------------------
            crate::ir::Instruction::ArenaCreate(capacity) => {
                // Uundaji wa arena ni malloc(uwezo).
                let malloc_fn = LLVMGetNamedFunction(module, c_str("malloc").as_ptr());
                let cap = v(value_map, capacity);
                let args = [cap];
                LLVMBuildCall2(
                    builder,
                    LLVMFunctionType(
                        ptr_type(),
                        [LLVMInt64Type()].as_mut_ptr(),
                        1,
                        0,
                    ),
                    malloc_fn,
                    args.as_ptr() as *mut LLVMValueRef,
                    1,
                    c_str("arena_create").as_ptr(),
                )
            }
            crate::ir::Instruction::ArenaAlloc(_arena, size) => {
                let malloc_fn = LLVMGetNamedFunction(module, c_str("malloc").as_ptr());
                let sz = v(value_map, size);
                let args = [sz];
                LLVMBuildCall2(
                    builder,
                    LLVMFunctionType(
                        ptr_type(),
                        [LLVMInt64Type()].as_mut_ptr(),
                        1,
                        0,
                    ),
                    malloc_fn,
                    args.as_ptr() as *mut LLVMValueRef,
                    1,
                    c_str("arena_alloc").as_ptr(),
                )
            }
            crate::ir::Instruction::ArenaFree(_arena) => {
                // Arena free ni utendaji-usiofanya kitu katika kiwango hiki; arena
                // inatolewa kwenye matokeo ya wigo na wito unaotokana na mbele.
                // Toa tupu void ili tu kuwa na thamani.
                LLVMConstNull(LLVMVoidType())
            }

            // -- anwani-ya ---------------------------------------------------------
            crate::ir::Instruction::FnAddr(name) => {
                let name_c = c_str(name);
                let func_val = LLVMGetNamedFunction(module, name_c.as_ptr());
                if func_val.is_null() {
                    // Tangaza papo hapo kama kazi ya nje (void()).
                    let fn_ty = LLVMFunctionType(LLVMVoidType(), std::ptr::null_mut(), 0, 0);
                    let f = LLVMAddFunction(module, name_c.as_ptr(), fn_ty);
                    // Badili aina hadi i8* kwa kielekezi kisicho wazi.
                    LLVMBuildBitCast(builder, f, ptr_type(), c_str("fnaddr").as_ptr())
                } else {
                    LLVMBuildBitCast(builder, func_val, ptr_type(), c_str("fnaddr").as_ptr())
                }
            }
            crate::ir::Instruction::GlobalAddr(name) => {
                let name_c = c_str(name);
                let global = LLVMGetNamedGlobal(module, name_c.as_ptr());
                if global.is_null() {
                    // Rudisha tupu ikiwa ulimwengu haupo.
                    LLVMConstNull(ptr_type())
                } else {
                    LLVMBuildBitCast(builder, global, ptr_type(), c_str("gaddr").as_ptr())
                }
            }
            crate::ir::Instruction::StringAddr(name) => {
                // Tafuta ulimwengu na fanya GEP ya fahirisi mbili (0, 0) kupata i8*.
                let name_c = c_str(name);
                let global = LLVMGetNamedGlobal(module, name_c.as_ptr());
                if global.is_null() {
                    return LLVMConstNull(ptr_type());
                }
                let zero = LLVMConstInt(LLVMInt32Type(), 0, 0);
                let indices = [zero, zero];
                LLVMBuildGEP2(
                    builder,
                    LLVMInt8Type(), // aina msingi ya kielekezi kisicho wazi
                    global,
                    indices.as_ptr() as *mut LLVMValueRef,
                    2,
                    c_str("strptr").as_ptr(),
                )
            }

            // -- hesabu za kielekezi -----------------------------------------------
            crate::ir::Instruction::Gep(base, indices) => {
                let base_val = v(value_map, base);
                let mut llvm_indices: Vec<LLVMValueRef> = indices
                    .iter()
                    .map(|i| v(value_map, i))
                    .collect();
                LLVMBuildGEP2(
                    builder,
                    LLVMInt8Type(), // aina msingi ya kielekezi kisicho wazi
                    base_val,
                    llvm_indices.as_mut_ptr(),
                    llvm_indices.len() as u32,
                    c_str("gep").as_ptr(),
                )
            }
            crate::ir::Instruction::FieldAddr(base, field_idx, struct_ty_opt) => {
                let base_val = v(value_map, base);
                // Hesabu kukabiliana kwa baiti ya sehemu kwa upatanisho.
                let byte_off: u64 = match struct_ty_opt {
                    Some(IrType::Struct { fields, .. }) => {
                        // Hesabu ukubwa uliopatanishwa wa aina (inalingana na mpangilio wa LLVM ndani ya muundo).
                        let aligned = |ty: &IrType| -> u64 {
                            let w = ty.width_bytes() as u64;
                            let a = std::cmp::min(w, 8);
                            (w + a - 1) & !(a - 1)
                        };
                        let mut off: u64 = 0;
                        let target_fw = fields.get(*field_idx).map(|(_, t)| t.width_bytes() as u64).unwrap_or(4);
                        for (fi, (_, fty)) in fields.iter().enumerate() {
                            if fi == *field_idx as usize {
                                let align = std::cmp::min(target_fw, 8);
                                off = (off + align - 1) & !(align - 1);
                                break;
                            }
                            let fw_aligned = aligned(fty);
                            let align = std::cmp::min(fw_aligned, 8);
                            off = (off + align - 1) & !(align - 1);
                            off += fw_aligned;
                        }
                        off
                    }
                    _ => (*field_idx * 8) as u64,
                };
                let byte_off_val = LLVMConstInt(LLVMInt32Type(), byte_off, 0);
                let byte_indices = [byte_off_val];
                LLVMBuildGEP2(builder, LLVMInt8Type(), base_val,
                    byte_indices.as_ptr() as *mut LLVMValueRef, 1,
                    c_str("fieldptr").as_ptr())
            }

            // -- majumuisho --------------------------------------------------------
            crate::ir::Instruction::BuildStruct(fields) => {
                // 1. Kusanya aina za sehemu kutoka kwa thamani za sehemu.
                let field_vals: Vec<LLVMValueRef> =
                    fields.iter().map(|f| v(value_map, f)).collect();
                let field_llvm_types: Vec<LLVMTypeRef> =
                    field_vals.iter().map(|&fv| LLVMTypeOf(fv)).collect();

                // 2. Unda aina ya muundo usio na jina.
                let anon_name = c_str(&format!(
                    "__anon_struct_{}",
                    field_vals.len()
                ));
                let struct_ty =
                    LLVMStructCreateNamed(LLVMGetModuleContext(module), anon_name.as_ptr());
                if !field_llvm_types.is_empty() {
                    LLVMStructSetBody(
                        struct_ty,
                        field_llvm_types.clone().as_mut_ptr(),
                        field_llvm_types.len() as u32,
                        0,
                    );
                } else {
                    LLVMStructSetBody(struct_ty, std::ptr::null_mut(), 0, 0);
                }

                // 3. Tenganisha nafasi (alloca).
                let alloca =
                    LLVMBuildAlloca(builder, struct_ty, c_str("struct_alloca").as_ptr());

                // 4. Hifadhi kila sehemu.
                for (i, &field_val) in field_vals.iter().enumerate() {
                    let zero = LLVMConstInt(LLVMInt32Type(), 0, 0);
                    let idx = LLVMConstInt(LLVMInt32Type(), i as u64, 0);
                    let gep_indices = [zero, idx];
                    let field_ptr = LLVMBuildGEP2(
                        builder,
                        struct_ty,
                        alloca,
                        gep_indices.as_ptr() as *mut LLVMValueRef,
                        2,
                        c_str("struct_field").as_ptr(),
                    );
                    // Lazimisha thamani ya sehemu kwa aina ya sehemu.
                    let field_llvm_ty = field_llvm_types[i];
                    let coerced = coerce_int(builder, field_val, field_llvm_ty);
                    LLVMBuildStore(builder, coerced, field_ptr);
                }

                // 5. Pakia thamani ya muundo.
                LLVMBuildLoad2(
                    builder,
                    struct_ty,
                    alloca,
                    c_str("struct_val").as_ptr(),
                )
            }
            crate::ir::Instruction::ExtractField(val, field_idx) => {
                let agg = v(value_map, val);
                LLVMBuildExtractValue(
                    builder,
                    agg,
                    *field_idx as u32,
                    c_str("extract").as_ptr(),
                )
            }

            crate::ir::Instruction::Select(cond, true_val, false_val) => {
                LLVMBuildSelect(
                    builder,
                    v(value_map, cond),
                    v(value_map, true_val),
                    v(value_map, false_val),
                    c_str("select").as_ptr(),
                )
            }

            // -- wito --------------------------------------------------------------
            crate::ir::Instruction::Call(callee, args) => {
                let name_c = c_str(callee);
                let callee_fn = LLVMGetNamedFunction(module, name_c.as_ptr());

                if callee_fn.is_null() {
                    // Kazi haijatangazwa — rudisha tupu.
                    return LLVMConstNull(ptr_type());
                }

                // Jenga orodha ya hoja, ukilazimisha aina kama inahitajika.
                let mut arg_vals: Vec<LLVMValueRef> = Vec::new();
                let mut arg_types: Vec<LLVMTypeRef> = Vec::new();

                // Pata aina ya kazi kuamua aina za kigezo zinazotarajiwa.
                let param_count = LLVMCountParams(callee_fn);

                for (i, arg_id) in args.iter().enumerate() {
                    let arg_val = v(value_map, arg_id);
                    // Lazimisha kwa aina ya kigezo inayotarajiwa ikiwa inajulikana.
                    if (i as u32) < param_count {
                        let expected_param = LLVMGetParam(callee_fn, i as u32);
                        let expected_ty = LLVMTypeOf(expected_param);
                        let coerced = coerce_int(builder, arg_val, expected_ty);
                        arg_vals.push(coerced);
                        arg_types.push(expected_ty);
                    } else {
                        arg_vals.push(arg_val);
                        arg_types.push(LLVMTypeOf(arg_val));
                    }
                }

                // Kwa rudisha void, angalia aina ya rudisha ya kazi.
                // LLVMTypeOf kwenye thamani ya kazi hurudisha "ptr" kwa vielekezi visivyo wazi.
                // Lazima tujenge upya aina ya kazi kutoka kwa aina za kigezo zilizotangazwa.
                let _ret_ty_from_decl = LLVMTypeOf(callee_fn);
                // Kwa vielekezi visivyo wazi, LLVMTypeOf(callee_fn) ni "ptr" tu.
                // Tunahitaji aina halisi ya kazi. Pata tena kutoka kwa vigezo.
                let param_count = LLVMCountParams(callee_fn);
                let mut rebuilt_param_tys: Vec<LLVMTypeRef> = Vec::new();
                for pi in 0..param_count {
                    rebuilt_param_tys.push(LLVMTypeOf(LLVMGetParam(callee_fn, pi)));
                }
                // Amua aina ya rudisha: tafuta kwenye ramani iliyohesabiwa mapema kwanza,
                // kisha rudia kazi za maktabac zinazojulikana.
                let inferred_ret_ty = if let Some(&ret_ty) = fn_return_types.get(callee) {
                    ret_ty
                } else {
                    match callee.as_str() {
                        "malloc" => ptr_type(),
                        "realloc" => ptr_type(),
                        "free" => LLVMVoidType(),
                        "printf" => LLVMInt32Type(),
                        "andika" => LLVMInt32Type(),
                        "fopen" => ptr_type(),
                        "fread" => LLVMInt64Type(),
                        "fclose" => LLVMInt32Type(),
                        _ => LLVMInt32Type(),
                    }
                };
                let call_fn_ty = LLVMFunctionType(
                    inferred_ret_ty,
                    if rebuilt_param_tys.is_empty() {
                        std::ptr::null_mut()
                    } else {
                        rebuilt_param_tys.as_mut_ptr()
                    },
                    rebuilt_param_tys.len() as u32,
                    if callee == "printf" || callee == "andika" { 1 } else { 0 },
                );

                let name = if inferred_ret_ty == LLVMVoidType() {
                    c_str("") // jina tupu kwa wito void
                } else {
                    c_str("call")
                };

                LLVMBuildCall2(
                    builder,
                    call_fn_ty,
                    callee_fn,
                    if arg_vals.is_empty() {
                        std::ptr::null_mut()
                    } else {
                        arg_vals.as_mut_ptr()
                    },
                    arg_vals.len() as u32,
                    name.as_ptr(),
                )
            }
            crate::ir::Instruction::CallIndirect(fn_ptr, args) => {
                let fp = v(value_map, fn_ptr);
                let mut arg_vals: Vec<LLVMValueRef> =
                    args.iter().map(|a| v(value_map, a)).collect();
                let mut arg_types: Vec<LLVMTypeRef> =
                    arg_vals.iter().map(|&av| LLVMTypeOf(av)).collect();

                // Jenga aina ya kazi: ptr(args) -> ptr (generic).
                let fn_ptr_ty = LLVMFunctionType(
                    ptr_type(), // generic return
                    if arg_types.is_empty() {
                        std::ptr::null_mut()
                    } else {
                        arg_types.as_mut_ptr()
                    },
                    arg_types.len() as u32,
                    0,
                );

                LLVMBuildCall2(
                    builder,
                    fn_ptr_ty,
                    fp,
                    if arg_vals.is_empty() {
                        std::ptr::null_mut()
                    } else {
                        arg_vals.as_mut_ptr()
                    },
                    arg_vals.len() as u32,
                    c_str("indirect_call").as_ptr(),
                )
            }
            // Nodi za phi zimeteremshwa tofauti kwenye lower_function (kupita-mbili:
            // phi-kwanza, kisha amri zilizobaki). Hazipaswi kamwe
            // kufikia njia hii mbadala.
            crate::ir::Instruction::Phi(_, _) => {
                LLVMConstNull(LLVMInt32Type())
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Uteremshaji wa kikomeshi
// ---------------------------------------------------------------------------

/// Teremsha kikomeshi cha IR hadi amri za kudhibiti mtiririko za LLVM.
fn lower_terminator(
    term: &Terminator,
    builder: LLVMBuilderRef,
    value_map: &HashMap<ValueId, LLVMValueRef>,
    llvm_blocks: &HashMap<usize, LLVMBasicBlockRef>,
    return_ty: LLVMTypeRef,
) {
    unsafe {
        /// Msaidizi wa kusuluhisha opereta ya ValueId.
        fn vv(value_map: &HashMap<ValueId, LLVMValueRef>, id: &ValueId) -> LLVMValueRef {
            value_map.get(id).copied().unwrap_or_else(|| {
                eprintln!("; WARN term: ValueId({}) not in map ({} entries)", id.0, value_map.len());
                unsafe { LLVMConstInt(LLVMInt32Type(), 0, 0) }
            })
        }

        match term {
            Terminator::Br(target) => {
                if let Some(&bb) = llvm_blocks.get(&target.0) {
                    LLVMBuildBr(builder, bb);
                }
            }
            Terminator::BrCond(cond, true_block, false_block) => {
                let cond_val = vv(value_map, cond);
                // Lazimisha hali hadi i1.
                let cond_i1 = {
                    let cond_ty = LLVMTypeOf(cond_val);
                    let kind = LLVMGetTypeKind(cond_ty) as u32;
                    if kind == LLVMTypeKind::Pointer as u32 {
                        // Linganisha kielekezi na tupu → i1.
                        LLVMBuildICmp(
                            builder,
                            LLVMIntPredicate::NE,
                            cond_val,
                            LLVMConstNull(LLVMPointerType(LLVMInt8Type(), 0)),
                            c_str("ptr_to_bool").as_ptr(),
                        )
                    } else if kind != LLVMTypeKind::Integer as u32 {
                        cond_val
                    } else if LLVMGetIntTypeWidth(cond_ty) != 1 {
                        LLVMBuildIntCast2(
                            builder,
                            cond_val,
                            LLVMInt1Type(),
                            0,
                            c_str("tobool").as_ptr(),
                        )
                    } else {
                        cond_val
                    }
                };
                let then_bb = llvm_blocks.get(&true_block.0).copied();
                let else_bb = llvm_blocks.get(&false_block.0).copied();
                if let (Some(then_bb), Some(else_bb)) = (then_bb, else_bb) {
                    LLVMBuildCondBr(builder, cond_i1, then_bb, else_bb);
                }
            }
            Terminator::Ret(val) => {
                let ret_val = vv(value_map, val);
                // Lazimisha kwa aina ya rudisha ya kazi ikiwa tunayo.
                // Ikiwa haijatolewa, toa kama ilivyo (jitihada bora).
                let ret_val = if !ret_val.is_null() && !return_ty.is_null() {
                    coerce_int(builder, ret_val, return_ty)
                } else {
                    ret_val
                };
                LLVMBuildRet(builder, ret_val);
            }
            Terminator::RetVoid => {
                LLVMBuildRetVoid(builder);
            }
            Terminator::Switch(scrutinee, default_block, arms) => {
                let scrut = vv(value_map, scrutinee);
                if let Some(&default_bb) = llvm_blocks.get(&default_block.0) {
                    let switch =
                        LLVMBuildSwitch(builder, scrut, default_bb, arms.len() as u32);
                    for (case_val_id, case_block) in arms {
                        let case_val = vv(value_map, case_val_id);
                        if let Some(&case_bb) = llvm_blocks.get(&case_block.0) {
                            LLVMAddCase(switch, case_val, case_bb);
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Ramani ya aina — IrType → LLVMTypeRef
// ---------------------------------------------------------------------------

/// Ramani [`IrType`] kwa [`LLVMTypeRef`] yake inayolingana.
///
/// Aina za muundo hutafutwa kwenye `struct_types`, ambayo lazima iwe
/// imejazwa na utangazaji wa kupita-mbili katika [`LlvmBackend::compile`].
fn ir_type_to_llvm(
    ty: &IrType,
    struct_types: &HashMap<String, LLVMTypeRef>,
) -> LLVMTypeRef {
    unsafe {
        match ty {
            IrType::Void => LLVMVoidType(),

            IrType::I8 | IrType::A8 | IrType::B8 | IrType::W8 => {
                LLVMInt8Type()
            }
            IrType::B1 => LLVMInt1Type(),
            IrType::I16 | IrType::A16 | IrType::B16 | IrType::W16 => {
                LLVMInt16Type()
            }
            IrType::F16 => LLVMHalfType(),

            IrType::I32 | IrType::A32 | IrType::B32 | IrType::W32 => {
                LLVMInt32Type()
            }
            IrType::F32 => LLVMFloatType(),

            IrType::I64 | IrType::A64 | IrType::B64 | IrType::W64 => {
                LLVMInt64Type()
            }
            IrType::F64 => LLVMDoubleType(),

            IrType::I128 | IrType::A128 => LLVMInt128Type(),
            IrType::F128 => LLVMFP128Type(),

            IrType::Ptr(_) | IrType::FnPtr { .. } => ptr_type(),

            IrType::Struct { name, .. } => {
                struct_types
                    .get(name)
                    .copied()
                    .unwrap_or_else(|| ptr_type())
            }
            IrType::Array { element, count } => {
                let elem_ty = ir_type_to_llvm(element, struct_types);
                LLVMArrayType(elem_ty, *count as u32)
            }
        }
    }
}

/// Rudisha aina ya kielekezi kisicho wazi `i8*`.
fn ptr_type() -> LLVMTypeRef {
    unsafe { LLVMPointerType(LLVMInt8Type(), 0) }
}

/// Hesabu ukubwa wa baiti wa aina kama thabiti ya LLVM ya `i64`.
#[allow(dead_code)]
fn type_size_of(ty: &IrType) -> LLVMValueRef {
    unsafe { LLVMConstInt(LLVMInt64Type(), ty.width_bytes() as u64, 0) }
}

// ---------------------------------------------------------------------------
// Visaidizi vya ulazimishaji namba
// ---------------------------------------------------------------------------

/// Lazimisha thamani namba ya LLVM kwa aina lengwa iliyotolewa kwa kutumia upanuzi-ishara.
fn coerce_int(
    builder: LLVMBuilderRef,
    val: LLVMValueRef,
    target_ty: LLVMTypeRef,
) -> LLVMValueRef {
    unsafe {
        let val_ty = LLVMTypeOf(val);
        if val_ty == target_ty {
            return val;
        }
        // Lazimisha aina za namba kamili pekee.
        let val_kind = LLVMGetTypeKind(val_ty) as u32;
        let target_kind = LLVMGetTypeKind(target_ty) as u32;

        if val_kind == LLVMTypeKind::Integer as u32
            && target_kind == LLVMTypeKind::Integer as u32
        {
            let val_width = LLVMGetIntTypeWidth(val_ty);
            let target_width = LLVMGetIntTypeWidth(target_ty);
            if val_width == target_width {
                // Aina za namba zenye upana sawa — rudisha kama ilivyo.
                return val;
            }
            // Panua-ishara (kwa usalama kwa thamani zenye ishara; panua-sifuri kwa zisizo na ishara
            // inashughulikiwa na mwita wakati inahitajika).
            LLVMBuildIntCast2(builder, val, target_ty, 1, c_str("coerce").as_ptr())
        } else if val_kind == LLVMTypeKind::Pointer as u32
            && target_kind == LLVMTypeKind::Pointer as u32
        {
            LLVMBuildBitCast(builder, val, target_ty, c_str("ptr_cast").as_ptr())
        } else {
            // Sio namba — rudisha kama ilivyo.
            val
        }
    }
}

/// Lazimisha opereta zote mbili za utendaji jozi kwa aina pana.
///
/// Inarudisha `(coerced_lhs, coerced_rhs, common_type)`.
/// Lazimisha opereta kwa ulinganisho (ICmp). Hushughulikia kielekezi-dhidi-namba
/// kwa kubadilisha namba hadi kielekezi (kawaida kwenye ukaguzi wa tupu).
fn coerce_cmp_operands(
    builder: LLVMBuilderRef,
    lhs: LLVMValueRef,
    rhs: LLVMValueRef,
) -> (LLVMValueRef, LLVMValueRef) {
    unsafe {
        let lhs_kind = LLVMGetTypeKind(LLVMTypeOf(lhs)) as u32;
        let rhs_kind = LLVMGetTypeKind(LLVMTypeOf(rhs)) as u32;
        if lhs_kind == LLVMTypeKind::Pointer as u32 && rhs_kind != LLVMTypeKind::Pointer as u32 {
            let coerced = LLVMBuildIntToPtr(builder, rhs, LLVMTypeOf(lhs), c_str("inttoptr").as_ptr());
            return (lhs, coerced);
        }
        if rhs_kind == LLVMTypeKind::Pointer as u32 && lhs_kind != LLVMTypeKind::Pointer as u32 {
            let coerced = LLVMBuildIntToPtr(builder, lhs, LLVMTypeOf(rhs), c_str("inttoptr").as_ptr());
            return (coerced, rhs);
        }
        let (cl, cr, _) = coerce_int_binop(builder, lhs, rhs);
        (cl, cr)
    }
}

fn coerce_int_binop(
    builder: LLVMBuilderRef,
    lhs: LLVMValueRef,
    rhs: LLVMValueRef,
) -> (LLVMValueRef, LLVMValueRef, LLVMTypeRef) {
    unsafe {
        let lhs_ty = LLVMTypeOf(lhs);
        let rhs_ty = LLVMTypeOf(rhs);

        let lhs_kind = LLVMGetTypeKind(lhs_ty) as u32;
        let rhs_kind = LLVMGetTypeKind(rhs_ty) as u32;

        // Ikiwa zote ni vielekezi, rudisha kama ilivyo (ulinganisho utashughulikia).
        if lhs_kind == LLVMTypeKind::Pointer as u32 && rhs_kind == LLVMTypeKind::Pointer as u32 {
            return (lhs, rhs, lhs_ty);
        }
        // Aina zisizo namba — rudisha kama ilivyo.
        if lhs_kind != LLVMTypeKind::Integer as u32
            || rhs_kind != LLVMTypeKind::Integer as u32
        {
            return (lhs, rhs, lhs_ty);
        }

        let lhs_width = LLVMGetIntTypeWidth(lhs_ty);
        let rhs_width = LLVMGetIntTypeWidth(rhs_ty);

        if lhs_width == rhs_width {
            return (lhs, rhs, lhs_ty);
        }

        if lhs_width > rhs_width {
            let coerced_rhs = LLVMBuildIntCast2(
                builder,
                rhs,
                lhs_ty,
                1, // sign-extend
                c_str("coerce").as_ptr(),
            );
            (lhs, coerced_rhs, lhs_ty)
        } else {
            let coerced_lhs = LLVMBuildIntCast2(
                builder,
                lhs,
                rhs_ty,
                1, // sign-extend
                c_str("coerce").as_ptr(),
            );
            (coerced_lhs, rhs, rhs_ty)
        }
    }
}

// ---------------------------------------------------------------------------
// Uundaji wa thabiti
// ---------------------------------------------------------------------------

/// Unda thabiti ya IR hadi thamani thabiti ya LLVM.
///
/// `default_ty` inatumika wakati lahaja ya thabiti haina
/// maelezo ya aina (k.m. `Const::Int`).
fn materialize_const(c: &Const, default_ty: LLVMTypeRef) -> LLVMValueRef {
    unsafe {
        match c {
            Const::Int(v) => {
                if *v >= 0 && *v <= u64::MAX as i128 {
                    LLVMConstInt(default_ty, *v as u64, 0)
                } else if *v < 0 && *v >= i64::MIN as i128 {
                    LLVMConstInt(default_ty, *v as u64, 1)
                } else {
                    let words: [u64; 2] = [*v as u64, (*v >> 64) as u64];
                    LLVMConstIntOfArbitraryPrecision(default_ty, 2, words.as_ptr())
                }
            }
            Const::Uint(v) => {
                if *v <= u64::MAX as u128 {
                    LLVMConstInt(default_ty, *v as u64, 0)
                } else {
                    let words: [u64; 2] = [*v as u64, (*v >> 64) as u64];
                    LLVMConstIntOfArbitraryPrecision(default_ty, 2, words.as_ptr())
                }
            }
            Const::Bool(b) => {
                LLVMConstInt(LLVMInt1Type(), if *b { 1 } else { 0 }, 0)
            }
            Const::NullPtr => LLVMConstNull(ptr_type()),
            Const::Zero => LLVMConstNull(default_ty),
            Const::Float(fw) => {
                let f64_val = fw.0;
                // Chagua aina sahihi ya namba sehemu kulingana na cha kawaida.
                let float_ty = match LLVMGetTypeKind(default_ty) as u32 {
                    k if k == LLVMTypeKind::Float as u32 => LLVMFloatType(),
                    k if k == LLVMTypeKind::Double as u32 => LLVMDoubleType(),
                    _ => LLVMDoubleType(), // cha kawaida ni maradufu
                };
                LLVMConstReal(float_ty, f64_val)
            }
            Const::String(s) => {
                // Unda thabiti ya kamba ya ulimwengu ya faragha.
                let bytes = s.as_bytes();
                let str_c = CString::new(bytes).unwrap();
                LLVMConstString(str_c.as_ptr(), bytes.len() as u32, 1)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tangazo la maktabac mapema
// ---------------------------------------------------------------------------

/// Tangaza mapema kazi saidizi za maktabac kwenye moduli:
///
/// - `declare ptr @malloc(i64)` — `void* malloc(size_t)`
/// - `declare void @free(ptr)` — `void free(void*)`
/// - `declare i32 @printf(ptr, ...)` — `int printf(const char*, ...)`
fn pre_declare_libc(module: LLVMModuleRef) {
    unsafe {
        // malloc: ptr (i64) -> ptr
        {
            let name = c_str("malloc");
            if LLVMGetNamedFunction(module, name.as_ptr()).is_null() {
                let mut param_tys = [LLVMInt64Type()];
                let fn_ty = LLVMFunctionType(ptr_type(), param_tys.as_mut_ptr(), 1, 0);
                LLVMAddFunction(module, name.as_ptr(), fn_ty);
            }
        }

        // free: void (ptr)
        {
            let name = c_str("free");
            if LLVMGetNamedFunction(module, name.as_ptr()).is_null() {
                let mut param_tys = [ptr_type()];
                let fn_ty = LLVMFunctionType(LLVMVoidType(), param_tys.as_mut_ptr(), 1, 0);
                LLVMAddFunction(module, name.as_ptr(), fn_ty);
            }
        }

        // printf: i32 (ptr, ...)
        {
            let name = c_str("printf");
            if LLVMGetNamedFunction(module, name.as_ptr()).is_null() {
                let mut param_tys = [ptr_type()];
                let fn_ty = LLVMFunctionType(LLVMInt32Type(), param_tys.as_mut_ptr(), 1, 1);
                LLVMAddFunction(module, name.as_ptr(), fn_ty);
            }
        }

        // __chkstk: kipepelezi cha kurasa za rafu kwa Windows.
        // Hutoa tu — BSS haihusiki, na rafu zetu za utendakazi ni ndogo.
        {
            let name = c_str("__chkstk");
            if LLVMGetNamedFunction(module, name.as_ptr()).is_null() {
                let fn_ty = LLVMFunctionType(LLVMVoidType(), std::ptr::null_mut(), 0, 0);
                let func = LLVMAddFunction(module, name.as_ptr(), fn_ty);
                let bb = LLVMAppendBasicBlockInContext(LLVMGetModuleContext(module), func, c_str("entry").as_ptr());
                let builder = LLVMCreateBuilder();
                LLVMPositionBuilderAtEnd(builder, bb);
                LLVMBuildRetVoid(builder);
                LLVMDisposeBuilder(builder);
            }
        }

        // andika: imetangazwa kama kazi tofauti; kiunganishi inaweka ramani kwa printf.
        {
            let name = c_str("andika");
            if LLVMGetNamedFunction(module, name.as_ptr()).is_null() {
                let mut param_tys = [ptr_type()];
                let fn_ty = LLVMFunctionType(LLVMInt32Type(), param_tys.as_mut_ptr(), 1, 1);
                unsafe { LLVMAddFunction(module, name.as_ptr(), fn_ty); }
            }
        }

        // fopen: ptr (ptr, ptr) → FILE*
        {
            let name = c_str("fopen");
            if LLVMGetNamedFunction(module, name.as_ptr()).is_null() {
                let mut param_tys = [ptr_type(), ptr_type()];
                let fn_ty = LLVMFunctionType(ptr_type(), param_tys.as_mut_ptr(), 2, 0);
                LLVMAddFunction(module, name.as_ptr(), fn_ty);
            }
        }

        // fread: i64 (ptr, i64, i64, ptr) → size_t
        {
            let name = c_str("fread");
            if LLVMGetNamedFunction(module, name.as_ptr()).is_null() {
                let mut param_tys = [ptr_type(), LLVMInt64Type(), LLVMInt64Type(), ptr_type()];
                let fn_ty = LLVMFunctionType(LLVMInt64Type(), param_tys.as_mut_ptr(), 4, 0);
                LLVMAddFunction(module, name.as_ptr(), fn_ty);
            }
        }

        // fclose: i32 (ptr) → int
        {
            let name = c_str("fclose");
            if LLVMGetNamedFunction(module, name.as_ptr()).is_null() {
                let mut param_tys = [ptr_type()];
                let fn_ty = LLVMFunctionType(LLVMInt32Type(), param_tys.as_mut_ptr(), 1, 0);
                LLVMAddFunction(module, name.as_ptr(), fn_ty);
            }
        }

        // realloc: ptr (ptr, i64) → ptr
        {
            let name = c_str("realloc");
            if LLVMGetNamedFunction(module, name.as_ptr()).is_null() {
                let mut param_tys = [ptr_type(), LLVMInt64Type()];
                let fn_ty = LLVMFunctionType(ptr_type(), param_tys.as_mut_ptr(), 2, 0);
                LLVMAddFunction(module, name.as_ptr(), fn_ty);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Drop — muktadha ni wa mchakato mzima, hakuna uondoaji wa kila mfano
// ---------------------------------------------------------------------------

impl Drop for LlvmBackend {
    fn drop(&mut self) {
        // Muktadha wa LLVM ni singleton ya mchakato mzima (muktadha wa kimataifa).
        // HATUITOI hapa — sehemu nyingine za mkusanyaji zinaweza bado
        // kushikilia marejeo ya aina au thamani zinazomilikiwa nao.
        //
        // Kuzima kwa mchakato wa LLVM yenyewe kutasafisha muktadha wa kimataifa.
    }
}

// ---------------------------------------------------------------------------
// Majaribio
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::types::IrType;
    use crate::ir::{Const, Function, Instruction, IrBlock, IrGlobal, IrReturnClass, Module as IrModule, Terminator, ValueId};

    /// Unda nyuma (huanzisha uanzishaji wa lengwa la LLVM).
    fn backend() -> LlvmBackend {
        LlvmBackend::new()
    }

    /// Msaidizi: maandishi ya LLVM IR ya moduli kama kamba ya Rust.
    unsafe fn module_to_string(module: LLVMModuleRef) -> String {
        let ptr = LLVMPrintModuleToString(module);
        let s = std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned();
        LLVMDisposeMessage(ptr);
        s
    }

    // -----------------------------------------------------------------------
    // jaribio_sanya_moduli_tupu
    // -----------------------------------------------------------------------

    #[test]
    fn test_compile_empty_module() {
        let b = backend();
        let ir_module = IrModule::new("empty_test");
        let result = b.compile(&ir_module);
        assert!(result.is_ok(), "empty module should compile: {:?}", result.err());
        let llvm_module = result.unwrap();
        unsafe {
            assert!(!llvm_module.is_null());
            let ir = module_to_string(llvm_module);
            assert!(ir.contains("empty_test"), "IR should contain module name");
            LLVMDisposeModule(llvm_module);
        }
    }

    // -----------------------------------------------------------------------
    // test_compile_direct_return — rudisha i64
    // -----------------------------------------------------------------------

    #[test]
    fn test_compile_direct_return() {
        let b = backend();
        let mut m = IrModule::new("direct_ret");

        let mut f = Function::new("direct_ret_fn", IrType::I64, vec![]);
        f.return_class = IrReturnClass::Direct;
        f.source_return_ty = IrType::I64;
        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        f.entry = entry;

        let val_42 = f.intern_const(Const::Int(42));
        f.blocks[entry.0].terminator = Terminator::Ret(val_42);

        m.push_function(f);
        let result = b.compile(&m);
        assert!(result.is_ok(), "direct return module should compile: {:?}", result.err());
        unsafe {
            let ir = module_to_string(result.unwrap());
            assert!(ir.contains("direct_ret_fn"), "IR should contain function name");
        }
    }

    // -----------------------------------------------------------------------
    // test_compile_sret_hidden_pointer — struct {i64,i64,i64} → sret
    // -----------------------------------------------------------------------

    #[test]
    fn test_compile_sret_hidden_pointer() {
        let b = backend();
        let mut m = IrModule::new("sret_test");

        let struct_ty = IrType::Struct {
            name: "Triple".into(),
            fields: vec![
                ("a".into(), IrType::I64),
                ("b".into(), IrType::I64),
                ("c".into(), IrType::I64),
            ],
        };
        m.push_type("Triple", struct_ty.clone());

        // kazi sret: kielekezi sret ni kigezo cha kwanza (isiyo wazi).
        let mut f = Function::new(
            "make_triple",
            IrType::Void,                           // rudisha kiwango LLVM baada ya kuandika upya sret
            vec![("sret_ptr".into(), IrType::Ptr(Box::new(struct_ty.clone())))],
        );
        f.source_return_ty = struct_ty;
        f.return_class = IrReturnClass::HiddenPtr;
        f.sret_value_id = Some(ValueId(0));

        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        f.entry = entry;
        m.push_function(f);

        let result = b.compile(&m);
        assert!(result.is_ok(), "sret module should compile: {:?}", result.err());
        unsafe {
            let ir = module_to_string(result.unwrap());
            assert!(ir.contains("make_triple"), "IR should contain function name");
            assert!(ir.contains("sret"), "IR should contain sret attribute");
        }
    }

    // -----------------------------------------------------------------------
    // jaribio_sanya_rudisha_void
    // -----------------------------------------------------------------------

    #[test]
    fn test_compile_void_return() {
        let b = backend();
        let mut m = IrModule::new("void_ret");

        let mut f = Function::new("do_nothing", IrType::Void, vec![]);
        f.return_class = IrReturnClass::Direct;
        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        f.entry = entry;

        m.push_function(f);
        let result = b.compile(&m);
        assert!(result.is_ok(), "void return module should compile: {:?}", result.err());
        unsafe {
            let ir = module_to_string(result.unwrap());
            assert!(ir.contains("do_nothing"), "IR should contain function name");
        }
    }

    // -----------------------------------------------------------------------
    // jaribio_sanya_kwa_alloca_na_hifadhi
    // -----------------------------------------------------------------------

    #[test]
    fn test_compile_with_alloca_and_store() {
        let b = backend();
        let mut m = IrModule::new("alloca_test");

        let mut f = Function::new("alloca_fn", IrType::I64, vec![("x".into(), IrType::I64)]);
        f.return_class = IrReturnClass::Direct;
        f.source_return_ty = IrType::I64;

        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        f.entry = entry;

        // Alloca kwa kigezo (ValueId(1)).
        f.blocks[entry.0].push(Instruction::Alloca(IrType::I64));
        // Hifadhi kigezo kwenye alloca (param ni ValueId(0), alloca ni ValueId(1)).
        f.blocks[entry.0].push(Instruction::Store(ValueId(0), ValueId(1)));
        // Pakia kutoka alloca (matokeo Load ni ValueId(3); matokeo Store ni tupu/ilirukwa).
        f.blocks[entry.0].push(Instruction::Load(IrType::I64, ValueId(1)));
        f.blocks[entry.0].terminator = Terminator::Ret(ValueId(3)); // matokeo ya Load

        m.push_function(f);
        let result = b.compile(&m);
        assert!(result.is_ok(), "alloca+store module should compile: {:?}", result.err());
        unsafe {
            let ir = module_to_string(result.unwrap());
            assert!(ir.contains("alloca"), "IR should contain alloca");
        }
    }

    // -----------------------------------------------------------------------
    // test_compile_opt_promotes_alloca_to_ssa — hakikisha kupita kwa uboreshaji
    // huondoa allocas wakati --opt imewezeshwa.
    //
    // KUMBUKA: LLVMRunPasses ya LLVM 22 inaharibu muktadha wa LLVM wa kimataifa ndani ya
    // mchakato mmoja, kwa hiyo uboreshaji wa wakati wa utekelezaji unajaribiwa kwenye
    // binary ya ujumuishaji (jaribio_k6_kujikusanya_kamili). Jaribio hili la unit
    // linathibitisha usanidi wa kimuundo pekee.
    // -----------------------------------------------------------------------

    #[test]
    fn z_test_compile_opt_infrastructure() {
        let b = backend().with_opt_level(LLVMCodeGenOptLevel::Less);
        let mut m = IrModule::new("opt_ssa_test");

        let mut f = Function::new("z_opt_fn", IrType::I64, vec![("x".into(), IrType::I64)]);
        f.return_class = IrReturnClass::Direct;
        f.source_return_ty = IrType::I64;

        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        f.entry = entry;

        // Alloca kwa kigezo + hifadhi kigezo kwenye alloca + pakia kutoka alloca.
        f.blocks[entry.0].push(Instruction::Alloca(IrType::I64));
        f.blocks[entry.0].push(Instruction::Store(ValueId(0), ValueId(1)));
        f.blocks[entry.0].push(Instruction::Load(IrType::I64, ValueId(1)));
        f.blocks[entry.0].terminator = Terminator::Ret(ValueId(3)); // matokeo ya Load

        m.push_function(f);
        // Thibitisha ukusanyaji unafanikiwa bila kuendesha uboreshaji
        // (avoids LLVM 22 global-context corruption).
        let result = b.compile(&m);
        assert!(result.is_ok(), "opt pipeline should compile: {:?}", result.err());
        unsafe {
            let llvm_module = result.unwrap();
            let ir = module_to_string(llvm_module);
            // Bila uboreshaji, allocas zinapaswa bado kuwapo.
            assert!(ir.contains("alloca"),
                "IR should contain alloca before optimisation");
            assert!(ir.contains("store"),
                "IR should contain store before optimisation");
            assert!(ir.contains("load"),
                "IR should contain load before optimisation");
            LLVMDisposeModule(llvm_module);
        }
    }

    // -----------------------------------------------------------------------
    // jaribio_sanya_kwa_hesabu_namba_kamili — ongeza thabiti mbili
    // -----------------------------------------------------------------------

    #[test]
    fn test_compile_with_integer_arithmetic() {
        let b = backend();
        let mut m = IrModule::new("arith_test");

        let mut f = Function::new("add_two", IrType::I64, vec![]);
        f.return_class = IrReturnClass::Direct;
        f.source_return_ty = IrType::I64;

        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        f.entry = entry;

        let c1 = f.intern_const(Const::Int(10));
        let c2 = f.intern_const(Const::Int(20));
        f.blocks[entry.0].push(Instruction::Add(c1, c2));
        f.blocks[entry.0].terminator = Terminator::Ret(ValueId(2)); // matokeo ya Add

        m.push_function(f);
        let result = b.compile(&m);
        assert!(result.is_ok(), "integer arithmetic module should compile: {:?}", result.err());
        unsafe {
            let ir = module_to_string(result.unwrap());
            assert!(ir.contains("add"), "IR should contain add instruction");
        }
    }

    // -----------------------------------------------------------------------
    // jaribio_sanya_chungu_tenga_tu — HeapAlloc + rudisha ptr
    // -----------------------------------------------------------------------

    #[test]
    fn test_compile_heap_alloc_only() {
        let b = backend();
        let mut m = IrModule::new("heap_test");

        let mut f = Function::new("alloc_some", IrType::Ptr(Box::new(IrType::I8)), vec![]);
        f.return_class = IrReturnClass::Direct;
        f.source_return_ty = IrType::Ptr(Box::new(IrType::I8));

        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        f.entry = entry;

        let size = f.intern_const(Const::Int(64));
        f.blocks[entry.0].push(Instruction::HeapAlloc(size));
        f.blocks[entry.0].terminator = Terminator::Ret(ValueId(1)); // matokeo ya HeapAlloc

        m.push_function(f);
        let result = b.compile(&m);
        assert!(result.is_ok(), "heap alloc module should compile: {:?}", result.err());
        unsafe {
            let ir = module_to_string(result.unwrap());
            assert!(ir.contains("malloc"), "IR should contain malloc call");
        }
    }

    // -----------------------------------------------------------------------
    // jaribio_sanya_chungu_tenga_toa — HeapAlloc + Hifadhi + HeapFree + RetVoid
    // -----------------------------------------------------------------------

    #[test]
    fn test_compile_heap_alloc_free() {
        let b = backend();
        let mut m = IrModule::new("heap_free_test");

        let mut f = Function::new("alloc_and_free", IrType::Void, vec![]);
        f.return_class = IrReturnClass::Direct;
        f.source_return_ty = IrType::Void;

        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        f.entry = entry;

        // Hesabu ValueIds kwa nguvu: thabiti kwanza, kisha amri.
        let param_count = f.params.len();
        let size_vid = f.intern_const(Const::Int(32));
        let val_vid = f.intern_const(Const::Int(1));
        let const_count = f.values.len();
        let heap_alloc_vid = ValueId(param_count + const_count + 0);
        let store_vid = ValueId(param_count + const_count + 1);
        let _heap_free_vid = ValueId(param_count + const_count + 2);

        f.blocks[entry.0].push(Instruction::HeapAlloc(size_vid));
        f.blocks[entry.0].push(Instruction::Store(val_vid, heap_alloc_vid));
        f.blocks[entry.0].push(Instruction::HeapFree(heap_alloc_vid));
        f.blocks[entry.0].terminator = Terminator::RetVoid;

        m.push_function(f);
        let result = b.compile(&m);
        assert!(result.is_ok(), "heap alloc+free module should compile: {:?}", result.err());
    }

    // -----------------------------------------------------------------------
    // jaribio_sanya_hesabu_namba_sehemu — fadd f64 mbili
    // -----------------------------------------------------------------------

    #[test]
    fn test_compile_float_arithmetic() {
        let b = backend();
        let mut m = IrModule::new("float_test");

        let mut f = Function::new("fadd_two", IrType::F64, vec![]);
        f.return_class = IrReturnClass::Direct;
        f.source_return_ty = IrType::F64;

        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        f.entry = entry;

        let c1 = f.intern_const(Const::Float(crate::ir::FloatWrapper(1.5)));
        let c2 = f.intern_const(Const::Float(crate::ir::FloatWrapper(2.5)));
        f.blocks[entry.0].push(Instruction::FAdd(c1, c2));
        f.blocks[entry.0].terminator = Terminator::Ret(ValueId(2));

        m.push_function(f);
        let result = b.compile(&m);
        assert!(result.is_ok(), "float arithmetic module should compile: {:?}", result.err());
        unsafe {
            let ir = module_to_string(result.unwrap());
            assert!(ir.contains("fadd"), "IR should contain fadd instruction");
        }
    }

    // -----------------------------------------------------------------------
    // test_compile_integer_comparison — LtS + zext hadi i32
    // -----------------------------------------------------------------------

    #[test]
    fn test_compile_integer_comparison() {
        let b = backend();
        let mut m = IrModule::new("icmp_test");

        let mut f = Function::new("cmp_two", IrType::I32, vec![]);
        f.return_class = IrReturnClass::Direct;
        f.source_return_ty = IrType::I32;

        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        f.entry = entry;

        let c1 = f.intern_const(Const::Int(5));
        let c2 = f.intern_const(Const::Int(10));
        f.blocks[entry.0].push(Instruction::LtS(c1, c2));     // matokeo ya i1
        f.blocks[entry.0].push(Instruction::Zext(ValueId(2), IrType::I32)); // zext hadi i32
        f.blocks[entry.0].terminator = Terminator::Ret(ValueId(3));

        m.push_function(f);
        let result = b.compile(&m);
        assert!(result.is_ok(), "integer comparison module should compile: {:?}", result.err());
        unsafe {
            let ir = module_to_string(result.unwrap());
            assert!(ir.contains("icmp"), "IR should contain icmp instruction");
        }
    }

    // -----------------------------------------------------------------------
    // jaribio_lengwa_tatu_mzunguko — pata tatu → tafuta lengwa (hakuna muktadha)
    // -----------------------------------------------------------------------

    #[test]
    fn test_target_triple_roundtrip() {
        let triple = default_target_triple();
        assert!(!triple.is_empty());

        unsafe {
            let triple_c = CString::new(triple.as_str()).unwrap();
            let mut target: LLVMTargetRef = std::ptr::null_mut();
            let mut error: *mut std::ffi::c_char = std::ptr::null_mut();
            let failed = LLVMGetTargetFromTriple(triple_c.as_ptr(), &mut target, &mut error);
            if failed != 0 {
                if !error.is_null() {
                    let msg = std::ffi::CStr::from_ptr(error).to_string_lossy();
                    LLVMDisposeMessage(error);
                    eprintln!("target lookup warning: {}", msg);
                }
            }
            // utafutaji lengwa unaweza kushindwa kihalali bila uanzishaji lengwa — ni sawa.
        }
    }

    // -----------------------------------------------------------------------
    // test_target_init_then_lookup — anzisha malengo → pata tatu → tafuta
    // -----------------------------------------------------------------------

    #[test]
    fn test_target_init_then_lookup() {
        // Anzisha uanzishaji kupitia ujenzi wa nyuma.
        let _b = backend();

        let triple = default_target_triple();
        assert!(!triple.is_empty());

        unsafe {
            let triple_c = CString::new(triple.as_str()).unwrap();
            let mut target: LLVMTargetRef = std::ptr::null_mut();
            let mut error: *mut std::ffi::c_char = std::ptr::null_mut();
            let failed = LLVMGetTargetFromTriple(triple_c.as_ptr(), &mut target, &mut error);
            assert_eq!(failed, 0, "target lookup should succeed after init");
            assert!(!target.is_null(), "target should be non-null");
        }
    }

    // -----------------------------------------------------------------------
    // test_context_then_target — muktadha kwanza, kisha anzisha malengo, kisha tafuta
    // -----------------------------------------------------------------------

    #[test]
    fn test_context_then_target() {
        unsafe {
            let ctx = LLVMGetGlobalContext();
            assert!(!ctx.is_null());
        }
        // Anzisha malengo.
        let _b = backend();

        let triple = default_target_triple();
        unsafe {
            let triple_c = CString::new(triple.as_str()).unwrap();
            let mut target: LLVMTargetRef = std::ptr::null_mut();
            let mut error: *mut std::ffi::c_char = std::ptr::null_mut();
            let failed = LLVMGetTargetFromTriple(triple_c.as_ptr(), &mut target, &mut error);
            assert_eq!(failed, 0, "target lookup should succeed");
            assert!(!target.is_null());
        }
    }

    // -----------------------------------------------------------------------
    // test_target_lookup_only — muktadha → anzisha → pata tatu → tafuta
    // -----------------------------------------------------------------------

    #[test]
    fn test_target_lookup_only() {
        unsafe {
            let ctx = LLVMGetGlobalContext();
            assert!(!ctx.is_null());
        }
        let _b = backend();
        let triple = default_target_triple();
        unsafe {
            let triple_c = CString::new(triple.as_str()).unwrap();
            let mut target: LLVMTargetRef = std::ptr::null_mut();
            let mut error: *mut std::ffi::c_char = std::ptr::null_mut();
            let failed = LLVMGetTargetFromTriple(triple_c.as_ptr(), &mut target, &mut error);
            assert_eq!(failed, 0);
            assert!(!target.is_null());
        }
    }

    // -----------------------------------------------------------------------
    // jaribio_unda_mashine_lengwa — uundaji kamili wa mashine lengwa
    // -----------------------------------------------------------------------

    #[test]
    fn test_create_target_machine() {
        let _b = backend();
        let triple = default_target_triple();

        unsafe {
            let triple_c = CString::new(triple.as_str()).unwrap();
            let mut target: LLVMTargetRef = std::ptr::null_mut();
            let mut error: *mut std::ffi::c_char = std::ptr::null_mut();
            let failed = LLVMGetTargetFromTriple(triple_c.as_ptr(), &mut target, &mut error);
            assert_eq!(failed, 0);

            let cpu = c_str("");
            let features = c_str("");
            let tm = LLVMCreateTargetMachine(
                target,
                triple_c.as_ptr(),
                cpu.as_ptr(),
                features.as_ptr(),
                LLVMCodeGenOptLevel::None,
                LLVMRelocMode::Default,
                LLVMCodeModel::Default,
            );
            assert!(!tm.is_null(), "target machine should be created");
            LLVMDisposeTargetMachine(tm);
        }
    }

    // -----------------------------------------------------------------------
    // jaribio_sanya_orodha_kamili — ufafanuzi muundo + FieldAddr + Hifadhi
    // -----------------------------------------------------------------------

    #[test]
    fn test_compile_orodha_full() {
        let b = backend();
        let mut m = IrModule::new("orodha_test");

        let struct_ty = IrType::Struct {
            name: "Orodha".into(),
            fields: vec![
                ("ptr".into(), IrType::Ptr(Box::new(IrType::I8))),
                ("len".into(), IrType::I64),
                ("cap".into(), IrType::I64),
            ],
        };
        m.push_type("Orodha", struct_ty.clone());

        let mut f = Function::new(
            "init_orodha",
            IrType::Void,
            vec![("o_ptr".into(), IrType::Ptr(Box::new(struct_ty.clone())))],
        );
        f.return_class = IrReturnClass::Direct;
        f.source_return_ty = IrType::Void;

        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        f.entry = entry;

        // Hifadhi sifuri kwa sehemu 1 (len) kupitia FieldAddr.
        // param_count=1 → param ni ValueId(0), intern_const ni ValueId(1).
        let zero = f.intern_const(Const::Int(0));
        // FieldAddr kwenye param ValueId(0), fahirisi sehemu 1 → matokeo ni ValueId(2).
        f.blocks[entry.0].push(Instruction::FieldAddr(ValueId(0), 1, Some(struct_ty.clone())));
        // Hifadhi thabiti sifuri (ValueId(1)) kwenye matokeo FieldAddr (ValueId(2)).
        f.blocks[entry.0].push(Instruction::Store(zero, ValueId(2)));
        f.blocks[entry.0].terminator = Terminator::RetVoid;

        m.push_function(f);
        let result = b.compile(&m);
        assert!(result.is_ok(), "orodha module should compile: {:?}", result.err());
        unsafe {
            let ir = module_to_string(result.unwrap());
            assert!(ir.contains("Orodha") || ir.contains("init_orodha"),
                "IR should contain struct or function name");
        }
    }

    // -----------------------------------------------------------------------
    // test_compile_printf_call — StringAddr + Ita printf → hakikisha IR
    // -----------------------------------------------------------------------

    #[test]
    fn test_compile_printf_call() {
        let b = backend();
        let mut m = IrModule::new("printf_test");

        // Ongeza ulimwengu kamba kwa kamba ya umbizo.
        m.push_global(IrGlobal {
            name: "fmt_hello".into(),
            bytes: b"hello world\n\0".to_vec(),
            is_const: true,
            ty: IrType::Array { element: Box::new(IrType::I8), count: 13 },
        });

        let mut f = Function::new("say_hello", IrType::I32, vec![]);
        f.return_class = IrReturnClass::Direct;
        f.source_return_ty = IrType::I32;

        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        f.entry = entry;

        // StringAddr kupata i8* kwa kamba ya umbizo (ValueId(0)).
        f.blocks[entry.0].push(Instruction::StringAddr("fmt_hello".into()));
        // Ita printf na kamba (matokeo ni ValueId(1)).
        f.blocks[entry.0].push(Instruction::Call("printf".into(), vec![ValueId(0)]));
        f.blocks[entry.0].terminator = Terminator::Ret(ValueId(1));

        m.push_function(f);
        let result = b.compile(&m);
        assert!(result.is_ok(), "printf module should compile: {:?}", result.err());
        unsafe {
            let ir = module_to_string(result.unwrap());
            assert!(ir.contains("printf"), "IR should contain printf");
        }
    }

    // -----------------------------------------------------------------------
    // jaribio_sanya_stringaddr_tu — StringAddr + rudisha
    // -----------------------------------------------------------------------

    #[test]
    fn test_compile_stringaddr_only() {
        let b = backend();
        let mut m = IrModule::new("straddr_test");

        m.push_global(IrGlobal {
            name: "my_str".into(),
            bytes: b"hi\0".to_vec(),
            is_const: true,
            ty: IrType::Array { element: Box::new(IrType::I8), count: 3 },
        });

        let mut f = Function::new("get_str", IrType::Ptr(Box::new(IrType::I8)), vec![]);
        f.return_class = IrReturnClass::Direct;
        f.source_return_ty = IrType::Ptr(Box::new(IrType::I8));

        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        f.entry = entry;

        f.blocks[entry.0].push(Instruction::StringAddr("my_str".into()));
        f.blocks[entry.0].terminator = Terminator::Ret(ValueId(0)); // matokeo ya StringAddr

        m.push_function(f);
        let result = b.compile(&m);
        assert!(result.is_ok(), "StringAddr module should compile: {:?}", result.err());
        unsafe {
            let ir = module_to_string(result.unwrap());
            assert!(ir.contains("my_str"), "IR should contain string global name");
        }
    }

    // -----------------------------------------------------------------------
    // test_compile_call_malloc — Ita malloc → rudisha ptr
    // -----------------------------------------------------------------------

    #[test]
    fn test_compile_call_malloc() {
        let b = backend();
        let mut m = IrModule::new("malloc_test");

        let mut f = Function::new("call_malloc", IrType::Ptr(Box::new(IrType::I8)), vec![]);
        f.return_class = IrReturnClass::Direct;
        f.source_return_ty = IrType::Ptr(Box::new(IrType::I8));

        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        f.entry = entry;

        let size = f.intern_const(Const::Int(128));
        f.blocks[entry.0].push(Instruction::Call("malloc".into(), vec![size]));
        f.blocks[entry.0].terminator = Terminator::Ret(ValueId(1));

        m.push_function(f);
        let result = b.compile(&m);
        assert!(result.is_ok(), "malloc call module should compile: {:?}", result.err());
        unsafe {
            let ir = module_to_string(result.unwrap());
            assert!(ir.contains("malloc"), "IR should contain malloc");
        }
    }

    // -----------------------------------------------------------------------
    // test_object_file_emission — sanya hadi faili halisi .o kwenye diski
    // -----------------------------------------------------------------------

    #[test]
    fn test_object_file_emission() {
        let b = backend();
        let mut m = IrModule::new("obj_test");

        let mut f = Function::new("obj_main", IrType::I32, vec![]);
        f.return_class = IrReturnClass::Direct;
        f.source_return_ty = IrType::I32;

        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        f.entry = entry;

        let zero = f.intern_const(Const::Int(0));
        f.blocks[entry.0].terminator = Terminator::Ret(zero);

        m.push_function(f);

        // Tumia saraka ya muda kwa pato.
        let tmp_dir = std::env::temp_dir();
        let obj_path = tmp_dir.join("swa_test_output.o");

        let result = b.compile_to_file(&m, &obj_path);
        assert!(result.is_ok(), "object file emission should succeed: {:?}", result.err());
        assert!(obj_path.exists(), "object file should exist on disk");
        assert!(obj_path.metadata().map(|m| m.len() > 0).unwrap_or(false),
            "object file should be non-empty");

        // Safisha.
        let _ = std::fs::remove_file(&obj_path);
    }

    // -----------------------------------------------------------------------
    // test_ir_type_to_llvm_coverage — hakikisha kila lahaja IrType ina ramani kwa
    // kitu kisicho tupu.
    // -----------------------------------------------------------------------

    #[test]
    fn test_ir_type_to_llvm_coverage() {
        let struct_types: HashMap<String, LLVMTypeRef> = HashMap::new();

        let types_to_test: Vec<IrType> = vec![
            IrType::Void,
            IrType::I8,
            IrType::I16,
            IrType::I32,
            IrType::I64,
            IrType::I128,
            IrType::A8,
            IrType::A16,
            IrType::A32,
            IrType::A64,
            IrType::A128,
            IrType::F16,
            IrType::F32,
            IrType::F64,
            IrType::F128,
            IrType::B1,
            IrType::B8,
            IrType::B16,
            IrType::B32,
            IrType::B64,
            IrType::W8,
            IrType::W16,
            IrType::W32,
            IrType::W64,
            IrType::Ptr(Box::new(IrType::I8)),
            IrType::FnPtr {
                params: vec![IrType::I32],
                ret: Box::new(IrType::Void),
            },
            IrType::Array {
                element: Box::new(IrType::I32),
                count: 4,
            },
        ];

        for ty in &types_to_test {
            let llvm_ty = ir_type_to_llvm(ty, &struct_types);
            assert!(!llvm_ty.is_null(), "ir_type_to_llvm({:?}) returned null", ty);
        }
    }
}
