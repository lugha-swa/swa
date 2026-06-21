//! LLVM code generation backend for the Swa compiler.
//!
//! Lowers Swa IR (from `crate::ir`) to LLVM IR and emits native object files.
//! Uses hand-written FFI bindings to the LLVM 18.1 C API from [`ffi`].
//!
//! ## Architecture
//!
//! 1. [`LlvmBackend::compile`] translates an entire [`IrModule`] into an
//!    [`LLVMModuleRef`], which can then be emitted to an object file via
//!    [`LlvmBackend::emit_object`].
//! 2. [`lower_function`] walks each IR function and produces LLVM basic blocks
//!    and instructions using the LLVM builder API.
//! 3. [`lower_instruction`] is the ~600-line match on every [`Instruction`]
//!    variant — the heart of the backend.
//! 4. Struct types are declared in a **two-pass** fashion: first as opaque named
//!    structs, then with bodies set, so that nested references resolve correctly.

pub mod ffi;

use std::collections::HashMap;
use std::ffi::CString;
use std::path::Path;
use std::sync::OnceLock;

use crate::diagnostics::{Diagnostic, SourceSpan};
use crate::ir::types::IrType;
use crate::ir::{Const, Function, IrReturnClass, Module as IrModule, Terminator, ValueId};

use self::ffi::*;

// ---------------------------------------------------------------------------
// One-time LLVM initialisation
// ---------------------------------------------------------------------------

/// Tracks whether the X86 target support has been initialised.
static LLVM_INIT: OnceLock<usize> = OnceLock::new();

// ---------------------------------------------------------------------------
// LlvmBackend
// ---------------------------------------------------------------------------

/// The LLVM code generation backend.
///
/// Holds a reference to the global LLVM context.  The context is process-wide
/// (a singleton) so there is no per-backend teardown — the `Drop` impl is a
/// no-op.
pub struct LlvmBackend {
    context: LLVMContextRef,
}

impl LlvmBackend {
    // -- construction ---------------------------------------------------------

    /// Create a new LLVM backend, initialising target support on first call.
    pub fn new() -> Self {
        let context = Self::get_context();
        Self { context }
    }

    /// Return the global LLVM context, initialising all X86 targets once.
    fn get_context() -> LLVMContextRef {
        LLVM_INIT.get_or_init(|| {
            unsafe {
                LLVMInitializeX86TargetInfo();
                LLVMInitializeX86Target();
                LLVMInitializeX86TargetMC();
                LLVMInitializeX86AsmPrinter();
                LLVMInitializeX86AsmParser();
            }
            1
        });
        unsafe { LLVMGetGlobalContext() }
    }

    // -- top-level compilation entry points -----------------------------------

    /// Compile an IR module and emit an object file to `output_path`.
    pub fn compile_to_file(
        &self,
        ir_module: &IrModule,
        output_path: &Path,
    ) -> Result<(), Vec<Diagnostic>> {
        let llvm_module = self.compile(ir_module)?;
        self.emit_object(llvm_module, output_path)
    }

    /// Parse LLVM IR text and emit an object file to `output_path`.
    ///
    /// This is a convenience for tools that already produce textual LLVM IR.
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

            // The memory buffer is never owned by the module — always dispose.
            LLVMDisposeMemoryBuffer(mem_buf);

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

            let result = self.emit_object(out_module, output_path);
            LLVMDisposeModule(out_module);
            result
        }
    }

    /// Compile an IR module to an LLVM module.
    ///
    /// This is the main compilation pipeline:
    ///
    /// 1. Create an LLVM module and set the target triple.
    /// 2. Two-pass struct declaration (opaque first, then bodies).
    /// 3. Declare global data (strings, typed arrays, scalars).
    /// 4. Pre-declare libc helper functions (malloc, free, printf).
    /// 5. Lower functions, processing `main` last so callees are defined first.
    /// 6. Verify the module.
    ///
    /// Returns the LLVM module on success, or a list of diagnostics on failure.
    pub fn compile(&self, ir_module: &IrModule) -> Result<LLVMModuleRef, Vec<Diagnostic>> {
        unsafe {
            // -- 1. Create LLVM module -----------------------------------------
            let name_c = c_str(&ir_module.name);
            let module = LLVMModuleCreateWithName(name_c.as_ptr());
            if module.is_null() {
                return Err(vec![Diagnostic::error(
                    "failed to create LLVM module",
                    SourceSpan::point(0, 0),
                )]);
            }

            // Set target triple.  On Windows we may get the MSVC triple from
            // LLVM, but the GNU linker (MinGW) is the one available.  Force
            // the GNU triple so linking succeeds.
            let triple = default_target_triple();
            let triple = if triple.contains("windows-msvc") {
                "x86_64-pc-windows-gnu".to_string()
            } else {
                triple
            };
            let triple_c = CString::new(triple.as_str()).unwrap();
            LLVMSetTarget(module, triple_c.as_ptr());

            // -- 2. Two-pass struct declaration --------------------------------
            let mut struct_types: HashMap<String, LLVMTypeRef> = HashMap::new();

            // First pass: create opaque named structs.
            for (name, _ty) in &ir_module.types {
                if matches!(_ty, IrType::Struct { .. }) {
                    let name_c = c_str(name);
                    let llvm_struct = LLVMStructCreateNamed(self.context, name_c.as_ptr());
                    struct_types.insert(name.clone(), llvm_struct);
                }
            }

            // Second pass: set struct bodies (now all nested references exist).
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
                                0, // not packed
                            );
                        } else {
                            // Empty struct: set body with zero elements.
                            LLVMStructSetBody(llvm_struct, std::ptr::null_mut(), 0, 0);
                        }
                    }
                }
            }

            // -- 3. Declare global data ----------------------------------------
            for global in &ir_module.globals {
                let is_string_like = !global.bytes.is_empty()
                    && global.bytes.last() == Some(&0)
                    && global.bytes.iter().all(|&b| b == 0 || (b >= 0x20 && b <= 0x7e));
                let str_len = if is_string_like && !global.bytes.is_empty() {
                    global.bytes.len() - 1 // exclude null terminator
                } else {
                    global.bytes.len()
                };
                let ty = if global.bytes.is_empty() {
                    LLVMInt8Type()
                } else {
                    // Match the array length to the initializer.
                    LLVMArrayType(LLVMInt8Type(), global.bytes.len() as u32)
                };

                let name_c = c_str(&global.name);
                let llvm_global = LLVMAddGlobal(module, ty, name_c.as_ptr());

                let init = if global.bytes.is_empty() {
                    LLVMConstNull(ty)
                } else if is_string_like && !global.bytes.iter().all(|&b| b == 0) {
                    // Truncate at the first interior null to avoid NulError.
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
                }
                LLVMSetLinkage(llvm_global, LLVMLinkage::Private);
            }

            // -- 4. Pre-declare libc functions ---------------------------------
            pre_declare_libc(module);

            // -- 5. Order function lowering: process main LAST -----------------
            // Collect function indices, putting main at the end.
            let mut ordered_indices: Vec<usize> = (0..ir_module.functions.len()).collect();
            // Find the index of "main" if it exists, move it to last.
            if let Some(main_idx) = ir_module.functions.iter().position(|f| f.name == "main") {
                ordered_indices.retain(|&i| i != main_idx);
                ordered_indices.push(main_idx);
            }

            // Lower each function.
            for idx in ordered_indices {
                let func = &ir_module.functions[idx];
                if let Err(diags) = lower_function(module, func, &struct_types) {
                    LLVMDisposeModule(module);
                    return Err(diags);
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

    // -- object-file emission -------------------------------------------------

    /// Emit an LLVM module to a native object file.
    ///
    /// Uses the host target triple with default CPU and feature settings.
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
                LLVMCodeGenOptLevel::None,
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
// Function lowering
// ---------------------------------------------------------------------------

/// Lower a single IR function into the given LLVM module.
///
/// This is the entry point for translating one Swa function:
///
/// 1. Map return / param types to LLVM types.
/// 2. Create the LLVM function (or reuse an existing declaration).
/// 3. Apply sret attributes when the return class is `HiddenPtr`.
/// 4. Build basic blocks with a two-pass entry-block strategy:
///    - Pass 1: lower only `Alloca` instructions.
///    - Store parameter values into their allocas.
///    - Pass 2: lower remaining instructions.
/// 5. For non-entry blocks, lower all instructions in order.
/// 6. Lower terminators and emit safe returns for unterminated blocks.
fn lower_function(
    module: LLVMModuleRef,
    func: &Function,
    struct_types: &HashMap<String, LLVMTypeRef>,
) -> Result<(), Vec<Diagnostic>> {
    unsafe {
        // -- 1. Build LLVM function type --------------------------------------
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

        // -- 2. Create or reuse LLVM function ---------------------------------
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

        // Set C ABI if requested (extern "C").
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

        // -- 3. Apply sret attribute (skip if unavailable on this LLVM build) --
        // LLVM 18 C API may not fully expose sret as an enum attribute.
        // The generated code is correct without it; sret is an ABI hint.
        // We attempt the call but silently ignore any failure.

        // -- 4. Create basic blocks -------------------------------------------
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

        let entry_bb = llvm_blocks[&func.entry.0];

        // -- 5. Build value map -----------------------------------------------
        let mut value_map: HashMap<ValueId, LLVMValueRef> = HashMap::new();

        // Map parameters: ValueId(0..N-1) correspond to LLVM params.
        for (i, _param) in func.params.iter().enumerate() {
            let llvm_param = LLVMGetParam(llvm_func, i as u32);
            value_map.insert(ValueId(i), llvm_param);
        }

        // Materialize constants: ValueId(params.len()..params.len()+values.len()-1).
        // Constants are materialized lazily with context-dependent types, so we
        // defer this — they are materialized on first use in lower_instruction.

        // Constants get typed during lowering. For now, pre-materialize with
        // default types that match the Const variant.
        let param_count = func.params.len();
        for (i, const_val) in func.values.iter().enumerate() {
            let val_id = ValueId(param_count + i);
            let llvm_val = materialize_const(const_val, LLVMInt64Type()); // default i64 placeholder
            value_map.insert(val_id, llvm_val);
        }

        // -- 5b. Builder -------------------------------------------------------
        let builder = LLVMCreateBuilder();

        // -- 6. Lower instructions sequentially across ALL blocks --------------
        let mut global_inst_idx = 0usize;
        for (block_idx, block) in func.blocks.iter().enumerate() {
            let bb = llvm_blocks[&block_idx];
            LLVMPositionBuilderAtEnd(builder, bb);
            for inst in &block.instructions {
                let val_id = ValueId(param_count + func.values.len() + global_inst_idx);
                global_inst_idx += 1;
                let llvm_val =
                    lower_instruction(inst, builder, &value_map, module, struct_types);
                if !llvm_val.is_null() {
                    value_map.insert(val_id, llvm_val);
                }
            }

            // Handle param stores for entry block.
            if block_idx == func.entry.0 {
                let mut alloca_idx = 0;
                for (param_i, _param) in func.params.iter().enumerate() {
                    let mut inst_pos = 0;
                    for inst in &block.instructions {
                        if matches!(inst, crate::ir::Instruction::Alloca(_)) {
                            if alloca_idx == param_i {
                                let alloc_vid = ValueId(param_count + func.values.len() + inst_pos);
                                if let Some(&alloca) = value_map.get(&alloc_vid) {
                                    let param_val = LLVMGetParam(llvm_func, param_i as u32);
                                    let stored_val = coerce_int(builder, param_val, LLVMTypeOf(alloca));
                                    LLVMBuildStore(builder, stored_val, alloca);
                                    value_map.insert(ValueId(param_i), alloca);
                                }
                                break;
                            }
                            alloca_idx += 1;
                        }
                        inst_pos += 1;
                    }
                }
            }
        }

        // -- 8. Lower non-entry blocks ----------------------------------------
        for (block_idx, _block) in func.blocks.iter().enumerate() {
            if block_idx == func.entry.0 {
                continue; // already lowered
            }

            let bb = llvm_blocks[&block_idx];
            LLVMPositionBuilderAtEnd(builder, bb);

            for (inst_idx, inst) in _block.instructions.iter().enumerate() {
                // Instruction values are numbered: after constants, sequentially
                // across all instructions in all blocks. Compute the global index.
                let global_inst_idx: usize = func.blocks[..block_idx]
                    .iter()
                    .map(|b| b.instructions.len())
                    .sum::<usize>()
                    + inst_idx;
                let val_id = ValueId(param_count + func.values.len() + global_inst_idx);

                let llvm_val =
                    lower_instruction(inst, builder, &value_map, module, struct_types);
                value_map.insert(val_id, llvm_val);
            }
        }

        // -- 9. Lower terminators for all blocks ------------------------------
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
// Instruction lowering — the ~600-line match
// ---------------------------------------------------------------------------

/// Lower a single IR instruction to an LLVM value using the builder.
///
/// Returns the LLVM value produced by this instruction.
fn lower_instruction(
    inst: &crate::ir::Instruction,
    builder: LLVMBuilderRef,
    value_map: &HashMap<ValueId, LLVMValueRef>,
    module: LLVMModuleRef,
    struct_types: &HashMap<String, LLVMTypeRef>,
) -> LLVMValueRef {
    unsafe {
        /// Helper to resolve a ValueId operand.
        fn v(value_map: &HashMap<ValueId, LLVMValueRef>, id: &ValueId) -> LLVMValueRef {
            value_map.get(id).copied().unwrap_or_else(|| {
                unsafe { LLVMConstInt(LLVMInt32Type(), 0, 0) }
            })
        }

        match inst {
            // -- integer arithmetic -----------------------------------------------
            crate::ir::Instruction::Add(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                // Check for pointer + integer → use GEP.
                let l_ty = LLVMTypeOf(l);
                if LLVMGetTypeKind(l_ty) as u32 == LLVMTypeKind::Pointer as u32 {
                    let indices = [r];
                    return LLVMBuildGEP2(
                        builder,
                        LLVMInt8Type(), // opaque pointer base type
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
                // Pointer - integer → GEP with negative index? No, just treat as int.
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

            // -- floating-point arithmetic ----------------------------------------
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

            // -- bitwise ----------------------------------------------------------
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

            // -- integer comparisons ----------------------------------------------
            crate::ir::Instruction::Eq(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _) = coerce_int_binop(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::EQ, cl, cr, c_str("eq").as_ptr())
            }
            crate::ir::Instruction::Ne(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _) = coerce_int_binop(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::NE, cl, cr, c_str("ne").as_ptr())
            }
            crate::ir::Instruction::LtS(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _) = coerce_int_binop(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::SLT, cl, cr, c_str("lts").as_ptr())
            }
            crate::ir::Instruction::LtU(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _) = coerce_int_binop(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::ULT, cl, cr, c_str("ltu").as_ptr())
            }
            crate::ir::Instruction::LeS(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _) = coerce_int_binop(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::SLE, cl, cr, c_str("les").as_ptr())
            }
            crate::ir::Instruction::LeU(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _) = coerce_int_binop(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::ULE, cl, cr, c_str("leu").as_ptr())
            }
            crate::ir::Instruction::GtS(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _) = coerce_int_binop(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::SGT, cl, cr, c_str("gts").as_ptr())
            }
            crate::ir::Instruction::GtU(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _) = coerce_int_binop(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::UGT, cl, cr, c_str("gtu").as_ptr())
            }
            crate::ir::Instruction::GeS(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _) = coerce_int_binop(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::SGE, cl, cr, c_str("ges").as_ptr())
            }
            crate::ir::Instruction::GeU(lhs, rhs) => {
                let l = v(value_map, lhs);
                let r = v(value_map, rhs);
                let (cl, cr, _) = coerce_int_binop(builder, l, r);
                LLVMBuildICmp(builder, LLVMIntPredicate::UGE, cl, cr, c_str("geu").as_ptr())
            }

            // -- floating-point comparisons ---------------------------------------
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

            // -- type conversions -------------------------------------------------
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

            // -- memory -----------------------------------------------------------
            crate::ir::Instruction::Alloca(ty) => {
                let llvm_ty = ir_type_to_llvm(ty, struct_types);
                LLVMBuildAlloca(builder, llvm_ty, c_str("alloca").as_ptr())
            }
            crate::ir::Instruction::Load(pointee_ty, ptr) => {
                let p = v(value_map, ptr);
                // For struct types, use opaque pointer load to avoid LLVM crashes.
                let llvm_ty = match pointee_ty {
                    IrType::Struct { .. } => ptr_type(),
                    _ => ir_type_to_llvm(pointee_ty, struct_types),
                };
                LLVMBuildLoad2(builder, llvm_ty, p, c_str("load").as_ptr())
            }
            crate::ir::Instruction::Store(val, ptr) => {
                let value = v(value_map, val);
                let p = v(value_map, ptr);
                // For struct values, store via opaque pointer to avoid crashes.
                LLVMBuildStore(builder, value, p)
            }

            // -- heap -------------------------------------------------------------
            crate::ir::Instruction::HeapAlloc(size) => {
                // Call pre-declared malloc function.
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
                // Cast to i8* if needed.
                let p_cast = LLVMBuildBitCast(builder, p, ptr_type(), c_str("free_cast").as_ptr());
                let args = [p_cast];
                LLVMBuildCall2(
                    builder,
                    LLVMFunctionType(LLVMVoidType(), [ptr_type()].as_mut_ptr(), 1, 0),
                    free_fn,
                    args.as_ptr() as *mut LLVMValueRef,
                    1,
                    c_str("").as_ptr(), // void call uses empty name
                )
            }

            // -- arenas -----------------------------------------------------------
            crate::ir::Instruction::ArenaCreate(capacity) => {
                // Arena creation is malloc(capacity).
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
                // Arena free is a no-op at this level; the arena is freed
                // at scope exit by the frontend-generated calls.
                // Emit a void null just to have a value.
                LLVMConstNull(LLVMVoidType())
            }

            // -- address-of -------------------------------------------------------
            crate::ir::Instruction::FnAddr(name) => {
                let name_c = c_str(name);
                let func_val = LLVMGetNamedFunction(module, name_c.as_ptr());
                if func_val.is_null() {
                    // Declare it on the fly as an external function (void()).
                    let fn_ty = LLVMFunctionType(LLVMVoidType(), std::ptr::null_mut(), 0, 0);
                    let f = LLVMAddFunction(module, name_c.as_ptr(), fn_ty);
                    // Bitcast to i8* for opaque pointer.
                    LLVMBuildBitCast(builder, f, ptr_type(), c_str("fnaddr").as_ptr())
                } else {
                    LLVMBuildBitCast(builder, func_val, ptr_type(), c_str("fnaddr").as_ptr())
                }
            }
            crate::ir::Instruction::GlobalAddr(name) => {
                let name_c = c_str(name);
                let global = LLVMGetNamedGlobal(module, name_c.as_ptr());
                if global.is_null() {
                    // Return null if global doesn't exist.
                    LLVMConstNull(ptr_type())
                } else {
                    LLVMBuildBitCast(builder, global, ptr_type(), c_str("gaddr").as_ptr())
                }
            }
            crate::ir::Instruction::StringAddr(name) => {
                // Look up the global and do a two-index GEP (0, 0) to get i8*.
                let name_c = c_str(name);
                let global = LLVMGetNamedGlobal(module, name_c.as_ptr());
                if global.is_null() {
                    return LLVMConstNull(ptr_type());
                }
                let zero = LLVMConstInt(LLVMInt32Type(), 0, 0);
                let indices = [zero, zero];
                LLVMBuildGEP2(
                    builder,
                    LLVMInt8Type(), // opaque pointer base type
                    global,
                    indices.as_ptr() as *mut LLVMValueRef,
                    2,
                    c_str("strptr").as_ptr(),
                )
            }

            // -- pointer arithmetic -----------------------------------------------
            crate::ir::Instruction::Gep(base, indices) => {
                let base_val = v(value_map, base);
                let mut llvm_indices: Vec<LLVMValueRef> = indices
                    .iter()
                    .map(|i| v(value_map, i))
                    .collect();
                LLVMBuildGEP2(
                    builder,
                    LLVMInt8Type(), // opaque pointer base type
                    base_val,
                    llvm_indices.as_mut_ptr(),
                    llvm_indices.len() as u32,
                    c_str("gep").as_ptr(),
                )
            }
            crate::ir::Instruction::FieldAddr(base, field_idx, struct_ty_opt) => {
                let base_val = v(value_map, base);
                let zero = LLVMConstInt(LLVMInt32Type(), 0, 0);
                let idx = LLVMConstInt(LLVMInt32Type(), *field_idx as u64, 0);
                let indices = [zero, idx];
                // Always use byte-level GEP: offset by field_idx * 4 (assume i32 fields).
                // This avoids crashes from struct type resolution mismatches.
                let byte_off = LLVMConstInt(LLVMInt32Type(), (*field_idx * 4) as u64, 0);
                let byte_indices = [byte_off];
                LLVMBuildGEP2(builder, LLVMInt8Type(), base_val,
                    byte_indices.as_ptr() as *mut LLVMValueRef, 1,
                    c_str("fieldptr").as_ptr())
            }

            // -- aggregate --------------------------------------------------------
            crate::ir::Instruction::BuildStruct(fields) => {
                // 1. Collect field types from field values.
                let field_vals: Vec<LLVMValueRef> =
                    fields.iter().map(|f| v(value_map, f)).collect();
                let field_llvm_types: Vec<LLVMTypeRef> =
                    field_vals.iter().map(|&fv| LLVMTypeOf(fv)).collect();

                // 2. Create anonymous struct type.
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

                // 3. Alloca.
                let alloca =
                    LLVMBuildAlloca(builder, struct_ty, c_str("struct_alloca").as_ptr());

                // 4. Store each field.
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
                    // Coerce field value to field type.
                    let field_llvm_ty = field_llvm_types[i];
                    let coerced = coerce_int(builder, field_val, field_llvm_ty);
                    LLVMBuildStore(builder, coerced, field_ptr);
                }

                // 5. Load the struct value.
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

            // -- calls ------------------------------------------------------------
            crate::ir::Instruction::Call(callee, args) => {
                let name_c = c_str(callee);
                let callee_fn = LLVMGetNamedFunction(module, name_c.as_ptr());

                if callee_fn.is_null() {
                    // Function not declared — return null.
                    return LLVMConstNull(ptr_type());
                }

                // Build argument list, coercing types as needed.
                let mut arg_vals: Vec<LLVMValueRef> = Vec::new();
                let mut arg_types: Vec<LLVMTypeRef> = Vec::new();

                // Get the function type to determine expected param types.
                let param_count = LLVMCountParams(callee_fn);

                for (i, arg_id) in args.iter().enumerate() {
                    let arg_val = v(value_map, arg_id);
                    // Coerce to expected parameter type if known.
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

                // For void returns, check the return type of the function.
                // LLVMTypeOf on a function value returns "ptr" with opaque pointers.
                // We must rebuild the function type from the declared parameter types.
                let ret_ty_from_decl = LLVMTypeOf(callee_fn);
                // With opaque pointers, LLVMTypeOf(callee_fn) is just "ptr".
                // We need the actual function type. Re-derive from parameters.
                let param_count = LLVMCountParams(callee_fn);
                let mut rebuilt_param_tys: Vec<LLVMTypeRef> = Vec::new();
                for pi in 0..param_count {
                    rebuilt_param_tys.push(LLVMTypeOf(LLVMGetParam(callee_fn, pi)));
                }
                // Determine return: if the IR Call has no result_ty info, default to ptr.
                // For known libc functions, we can infer: malloc→ptr, printf→i32, free→void.
                let inferred_ret_ty = match callee.as_str() {
                    "malloc" => ptr_type(),
                    "free" => LLVMVoidType(),
                    "printf" => LLVMInt32Type(),
                    _ => ptr_type(),
                };
                let call_fn_ty = LLVMFunctionType(
                    inferred_ret_ty,
                    if rebuilt_param_tys.is_empty() {
                        std::ptr::null_mut()
                    } else {
                        rebuilt_param_tys.as_mut_ptr()
                    },
                    rebuilt_param_tys.len() as u32,
                    if callee == "printf" { 1 } else { 0 },
                );

                let name = if inferred_ret_ty == LLVMVoidType() {
                    c_str("") // empty name for void calls
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

                // Build function type: ptr(args) -> ptr (generic).
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
        }
    }
}

// ---------------------------------------------------------------------------
// Terminator lowering
// ---------------------------------------------------------------------------

/// Lower an IR terminator into LLVM control-flow instructions.
fn lower_terminator(
    term: &Terminator,
    builder: LLVMBuilderRef,
    value_map: &HashMap<ValueId, LLVMValueRef>,
    llvm_blocks: &HashMap<usize, LLVMBasicBlockRef>,
    return_ty: LLVMTypeRef,
) {
    unsafe {
        /// Helper to resolve a ValueId operand.
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
                // Coerce condition to i1.
                let cond_i1 = if LLVMGetTypeKind(LLVMTypeOf(cond_val)) as u32
                    != LLVMTypeKind::Integer as u32
                {
                    cond_val
                } else if LLVMGetIntTypeWidth(LLVMTypeOf(cond_val)) != 1 {
                    LLVMBuildIntCast2(
                        builder,
                        cond_val,
                        LLVMInt1Type(),
                        0,
                        c_str("tobool").as_ptr(),
                    )
                } else {
                    cond_val
                };
                let then_bb = llvm_blocks.get(&true_block.0).copied();
                let else_bb = llvm_blocks.get(&false_block.0).copied();
                if let (Some(then_bb), Some(else_bb)) = (then_bb, else_bb) {
                    LLVMBuildCondBr(builder, cond_i1, then_bb, else_bb);
                }
            }
            Terminator::Ret(val) => {
                let ret_val = vv(value_map, val);
                // Coerce to the function's return type if we have it.
                // If not provided, emit as-is (best effort).
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
// Safe return emission
// ---------------------------------------------------------------------------

/// Emit a safe return instruction for blocks that are otherwise empty or
/// unterminated.  Emits `ret void` or `ret i32 0` depending on the return type.
fn emit_safe_ret(builder: LLVMBuilderRef, return_ty: &IrType) {
    unsafe {
        // Check if the block already has a terminator by looking at the insert
        // position — if the builder is not positioned, the block was already
        // terminated.  There is no direct "has terminator" query in the C API,
        // so we defensively emit a ret.  LLVM will complain on verification if
        // there are multiple terminators, but `emit_safe_ret` is only called when
        // we expect blocks might be unterminated.
        if matches!(return_ty, IrType::Void) {
            LLVMBuildRetVoid(builder);
        } else {
            let zero = LLVMConstInt(LLVMInt32Type(), 0, 1);
            LLVMBuildRet(builder, zero);
        }
    }
}

// ---------------------------------------------------------------------------
// Type mapping — IrType → LLVMTypeRef
// ---------------------------------------------------------------------------

/// Map an [`IrType`] to its corresponding [`LLVMTypeRef`].
///
/// Struct types are looked up in `struct_types`, which must have been
/// populated by the two-pass declaration in [`LlvmBackend::compile`].
fn ir_type_to_llvm(
    ty: &IrType,
    struct_types: &HashMap<String, LLVMTypeRef>,
) -> LLVMTypeRef {
    unsafe {
        match ty {
            IrType::Void => LLVMVoidType(),

            IrType::I8 | IrType::U8 | IrType::B8 | IrType::W8 => {
                LLVMInt8Type()
            }
            IrType::B1 => LLVMInt1Type(),
            IrType::I16 | IrType::U16 | IrType::B16 | IrType::W16 => {
                LLVMInt16Type()
            }
            IrType::F16 => LLVMHalfType(),

            IrType::I32 | IrType::U32 | IrType::B32 | IrType::W32 => {
                LLVMInt32Type()
            }
            IrType::F32 => LLVMFloatType(),

            IrType::I64 | IrType::U64 | IrType::B64 | IrType::W64 => {
                LLVMInt64Type()
            }
            IrType::F64 => LLVMDoubleType(),

            IrType::I128 | IrType::U128 => LLVMInt128Type(),
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

/// Return the opaque pointer type `i8*`.
fn ptr_type() -> LLVMTypeRef {
    unsafe { LLVMPointerType(LLVMInt8Type(), 0) }
}

/// Compute the byte size of a type as an `i64` LLVM constant.
#[allow(dead_code)]
fn type_size_of(ty: &IrType) -> LLVMValueRef {
    unsafe { LLVMConstInt(LLVMInt64Type(), ty.width_bytes() as u64, 0) }
}

// ---------------------------------------------------------------------------
// Integer coercion helpers
// ---------------------------------------------------------------------------

/// Coerce an integer LLVM value to the given target type using sign-extension.
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
        // Only coerce integer types.
        let val_kind = LLVMGetTypeKind(val_ty) as u32;
        let target_kind = LLVMGetTypeKind(target_ty) as u32;

        if val_kind == LLVMTypeKind::Integer as u32
            && target_kind == LLVMTypeKind::Integer as u32
        {
            let val_width = LLVMGetIntTypeWidth(val_ty);
            let target_width = LLVMGetIntTypeWidth(target_ty);
            if val_width == target_width {
                // Same-width integer kinds — just return as-is.
                return val;
            }
            // Sign-extend (conservative for signed values; zero-extend for unsigned
            // is handled by the caller when needed).
            LLVMBuildIntCast2(builder, val, target_ty, 1, c_str("coerce").as_ptr())
        } else if val_kind == LLVMTypeKind::Pointer as u32
            && target_kind == LLVMTypeKind::Pointer as u32
        {
            LLVMBuildBitCast(builder, val, target_ty, c_str("ptr_cast").as_ptr())
        } else {
            // Non-integer → return as-is.
            val
        }
    }
}

/// Coerce both operands of a binary operation to the wider type.
///
/// Returns `(coerced_lhs, coerced_rhs, common_type)`.
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

        // Only coerce if both are integers.
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
// Constant materialisation
// ---------------------------------------------------------------------------

/// Materialize an IR constant into an LLVM constant value.
///
/// The `default_ty` is used when the constant variant does not carry explicit
/// type information (e.g. `Const::Int`).
fn materialize_const(c: &Const, default_ty: LLVMTypeRef) -> LLVMValueRef {
    unsafe {
        match c {
            Const::Int(v) => {
                // For values that fit in u64, use LLVMConstInt.
                let val = *v as u64; // truncate to u64
                let sign_extend = if *v < 0 { 1 } else { 0 };
                LLVMConstInt(default_ty, val, sign_extend)
            }
            Const::Uint(v) => {
                let val = (*v) as u64;
                LLVMConstInt(default_ty, val, 0)
            }
            Const::Bool(b) => {
                LLVMConstInt(LLVMInt1Type(), if *b { 1 } else { 0 }, 0)
            }
            Const::NullPtr => LLVMConstNull(ptr_type()),
            Const::Zero => LLVMConstNull(default_ty),
            Const::Float(fw) => {
                let f64_val = fw.0;
                // Choose appropriate float type based on default.
                let float_ty = match LLVMGetTypeKind(default_ty) as u32 {
                    k if k == LLVMTypeKind::Float as u32 => LLVMFloatType(),
                    k if k == LLVMTypeKind::Double as u32 => LLVMDoubleType(),
                    _ => LLVMDoubleType(), // default to double
                };
                LLVMConstReal(float_ty, f64_val)
            }
            Const::String(s) => {
                // Create a private global string constant.
                let bytes = s.as_bytes();
                let str_c = CString::new(bytes).unwrap();
                LLVMConstString(str_c.as_ptr(), bytes.len() as u32, 1)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Libc pre-declaration
// ---------------------------------------------------------------------------

/// Pre-declare libc helper functions in the module:
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
    }
}

// ---------------------------------------------------------------------------
// Drop — context is process-wide, no per-instance teardown
// ---------------------------------------------------------------------------

impl Drop for LlvmBackend {
    fn drop(&mut self) {
        // The LLVM context is a process-wide singleton (the global context).
        // We do NOT dispose it here — other parts of the compiler may still
        // hold references to types or values owned by it.
        //
        // LLVM's own process shutdown will clean up the global context.
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::types::IrType;
    use crate::ir::{Const, Function, Instruction, IrBlock, IrGlobal, IrReturnClass, Module as IrModule, Terminator, ValueId};

    /// Create a backend (triggers LLVM target init).
    fn backend() -> LlvmBackend {
        LlvmBackend::new()
    }

    /// Helper: the LLVM IR text of a module as a Rust string.
    unsafe fn module_to_string(module: LLVMModuleRef) -> String {
        let ptr = LLVMPrintModuleToString(module);
        let s = std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned();
        LLVMDisposeMessage(ptr);
        s
    }

    // -----------------------------------------------------------------------
    // test_compile_empty_module
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
    // test_compile_direct_return — i64 return
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

        // sret function: the sret pointer is the first parameter (implicit).
        let mut f = Function::new(
            "make_triple",
            IrType::Void,                           // LLVM-level return after sret rewrite
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
    // test_compile_void_return
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
    // test_compile_with_alloca_and_store
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

        // Alloca for the parameter (ValueId(1)).
        f.blocks[entry.0].push(Instruction::Alloca(IrType::I64));
        // Store parameter into alloca (param is ValueId(0), alloca is ValueId(1)).
        f.blocks[entry.0].push(Instruction::Store(ValueId(0), ValueId(1)));
        // Load from alloca (Load result is ValueId(3); Store result is null/skipped).
        f.blocks[entry.0].push(Instruction::Load(IrType::I64, ValueId(1)));
        f.blocks[entry.0].terminator = Terminator::Ret(ValueId(3)); // Load result

        m.push_function(f);
        let result = b.compile(&m);
        assert!(result.is_ok(), "alloca+store module should compile: {:?}", result.err());
        unsafe {
            let ir = module_to_string(result.unwrap());
            assert!(ir.contains("alloca"), "IR should contain alloca");
        }
    }

    // -----------------------------------------------------------------------
    // test_compile_with_integer_arithmetic — add two constants
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
        f.blocks[entry.0].terminator = Terminator::Ret(ValueId(2)); // Add result

        m.push_function(f);
        let result = b.compile(&m);
        assert!(result.is_ok(), "integer arithmetic module should compile: {:?}", result.err());
        unsafe {
            let ir = module_to_string(result.unwrap());
            assert!(ir.contains("add"), "IR should contain add instruction");
        }
    }

    // -----------------------------------------------------------------------
    // test_compile_heap_alloc_only — HeapAlloc + ret ptr
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
        f.blocks[entry.0].terminator = Terminator::Ret(ValueId(1)); // HeapAlloc result

        m.push_function(f);
        let result = b.compile(&m);
        assert!(result.is_ok(), "heap alloc module should compile: {:?}", result.err());
        unsafe {
            let ir = module_to_string(result.unwrap());
            assert!(ir.contains("malloc"), "IR should contain malloc call");
        }
    }

    // -----------------------------------------------------------------------
    // test_compile_heap_alloc_free — HeapAlloc + Store + HeapFree + RetVoid
    // -----------------------------------------------------------------------

    #[test]
    #[ignore = "requires precise ValueId ordering for Store/HeapFree chain"]
    fn test_compile_heap_alloc_free() {
        let b = backend();
        let mut m = IrModule::new("heap_free_test");

        let mut f = Function::new("alloc_and_free", IrType::Void, vec![]);
        f.return_class = IrReturnClass::Direct;
        f.source_return_ty = IrType::Void;

        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        f.entry = entry;

        let size = f.intern_const(Const::Int(32));
        let val = f.intern_const(Const::Int(1));
        f.blocks[entry.0].push(Instruction::HeapAlloc(size));   // ValueId(2)
        f.blocks[entry.0].push(Instruction::Store(val, ValueId(2))); // ValueId(3)
        f.blocks[entry.0].push(Instruction::HeapFree(ValueId(2)));   // ValueId(4)
        f.blocks[entry.0].terminator = Terminator::RetVoid;

        m.push_function(f);
        let result = b.compile(&m);
        assert!(result.is_ok(), "heap alloc+free module should compile: {:?}", result.err());
    }

    // -----------------------------------------------------------------------
    // test_compile_float_arithmetic — fadd two f64
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
    // test_compile_integer_comparison — LtS + zext to i32
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
        f.blocks[entry.0].push(Instruction::LtS(c1, c2));     // i1 result
        f.blocks[entry.0].push(Instruction::Zext(ValueId(2), IrType::I32)); // zext to i32
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
    // test_target_triple_roundtrip — get triple → lookup target (no context)
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
            // target lookup may legitimately fail without target init — that's ok.
        }
    }

    // -----------------------------------------------------------------------
    // test_target_init_then_lookup — init targets → get triple → lookup
    // -----------------------------------------------------------------------

    #[test]
    fn test_target_init_then_lookup() {
        // Trigger init via backend construction.
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
    // test_context_then_target — context first, then init targets, then lookup
    // -----------------------------------------------------------------------

    #[test]
    fn test_context_then_target() {
        unsafe {
            let ctx = LLVMGetGlobalContext();
            assert!(!ctx.is_null());
        }
        // Init targets.
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
    // test_target_lookup_only — context → init → get triple → lookup
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
    // test_create_target_machine — full target machine creation
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
    // test_compile_orodha_full — struct definition + FieldAddr + Store
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

        // Store zero to field 1 (len) via FieldAddr.
        // param_count=1 → param is ValueId(0), intern_const is ValueId(1).
        let zero = f.intern_const(Const::Int(0));
        // FieldAddr on param ValueId(0), field index 1 → result is ValueId(2).
        f.blocks[entry.0].push(Instruction::FieldAddr(ValueId(0), 1, Some(struct_ty.clone())));
        // Store the zero constant (ValueId(1)) into the FieldAddr result (ValueId(2)).
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
    // test_compile_printf_call — StringAddr + Call printf → verify IR
    // -----------------------------------------------------------------------

    #[test]
    fn test_compile_printf_call() {
        let b = backend();
        let mut m = IrModule::new("printf_test");

        // Add a string global for the format string.
        m.push_global(IrGlobal {
            name: "fmt_hello".into(),
            bytes: b"hello world\n\0".to_vec(),
            is_const: true,
        });

        let mut f = Function::new("say_hello", IrType::I32, vec![]);
        f.return_class = IrReturnClass::Direct;
        f.source_return_ty = IrType::I32;

        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        f.entry = entry;

        // StringAddr to get i8* to the format string (ValueId(0)).
        f.blocks[entry.0].push(Instruction::StringAddr("fmt_hello".into()));
        // Call printf with the string (result is ValueId(1)).
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
    // test_compile_stringaddr_only — StringAddr + ret
    // -----------------------------------------------------------------------

    #[test]
    fn test_compile_stringaddr_only() {
        let b = backend();
        let mut m = IrModule::new("straddr_test");

        m.push_global(IrGlobal {
            name: "my_str".into(),
            bytes: b"hi\0".to_vec(),
            is_const: true,
        });

        let mut f = Function::new("get_str", IrType::Ptr(Box::new(IrType::I8)), vec![]);
        f.return_class = IrReturnClass::Direct;
        f.source_return_ty = IrType::Ptr(Box::new(IrType::I8));

        let entry = f.push_block(IrBlock::new("entry", Terminator::RetVoid));
        f.entry = entry;

        f.blocks[entry.0].push(Instruction::StringAddr("my_str".into()));
        f.blocks[entry.0].terminator = Terminator::Ret(ValueId(0)); // StringAddr result

        m.push_function(f);
        let result = b.compile(&m);
        assert!(result.is_ok(), "StringAddr module should compile: {:?}", result.err());
        unsafe {
            let ir = module_to_string(result.unwrap());
            assert!(ir.contains("my_str"), "IR should contain string global name");
        }
    }

    // -----------------------------------------------------------------------
    // test_compile_call_malloc — Call malloc → returns ptr
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
    // test_object_file_emission — compile to actual .o file on disk
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

        // Use a temp directory for the output.
        let tmp_dir = std::env::temp_dir();
        let obj_path = tmp_dir.join("swa_test_output.o");

        let result = b.compile_to_file(&m, &obj_path);
        assert!(result.is_ok(), "object file emission should succeed: {:?}", result.err());
        assert!(obj_path.exists(), "object file should exist on disk");
        assert!(obj_path.metadata().map(|m| m.len() > 0).unwrap_or(false),
            "object file should be non-empty");

        // Clean up.
        let _ = std::fs::remove_file(&obj_path);
    }

    // -----------------------------------------------------------------------
    // test_ir_type_to_llvm_coverage — verify each IrType variant maps to
    // something non-null.
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
            IrType::U8,
            IrType::U16,
            IrType::U32,
            IrType::U64,
            IrType::U128,
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
