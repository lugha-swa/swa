//! IR lowerer — converts a parsed Swa AST (flat-array form) into the Swa IR
//! defined in [`crate::ir`].
//!
//! ## AST flat-array format
//!
//! The parser emits several parallel arrays indexed by node id:
//!
//! | Array          | Element type | Meaning                          |
//! |----------------|-------------|----------------------------------|
//! | `ast_aina`     | `u32`       | Node kind (one of the `AST_*` constants) |
//! | `ast_kushoto`  | `i32`       | Left / first child index (-1 = none)     |
//! | `ast_kulia`    | `i32`       | Right child index (-1 = none)            |
//! | `ast_tiga`     | `i32`       | Else-branch / for-step / body (-1 = none)|
//! | `ast_nne`      | `i32`       | Sibling chain / chain continuation (-1 = none) |
//! | `ast_thamani`  | `i32`       | Encoded integer literal or type-name pool offset |
//! | `ast_jina_off` | `i32`       | Offset into `ast_pool` for identifier names |
//! | `ast_pool`     | `u8`        | String pool (null-terminated names, length-prefixed literals) |
//!
//! ## Lowering strategy
//!
//! The root node (last allocated, index `ast_idadi - 1`) is always
//! `AST_PROGRAMU` (1).  Its `ast_kushoto` points to the first child; children
//! are chained via `ast_nne`.  Each child is either a function (`AST_KAZI`, 2)
//! or a global variable (`AST_TANGAZO_ULIMWENGU`, 35).
//!
//! Functions are lowered into [`Function`] values containing basic blocks.
//! Statements produce control-flow (branches, loops, switches); expressions
//! produce [`ValueId`]s.  Short-circuit `&&` / `||` create fresh blocks.

use super::{BlockId, Const, Function, Instruction, IrBlock, IrGlobal, IrReturnClass, Module, Terminator, ValueId};
use super::types::IrType;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// AST node-type constants
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

/// Sentinel used in `ast_kushoto`, `ast_kulia`, `ast_tiga`, and `ast_nne` to
/// indicate "no child / no sibling".
const NO_NODE: i32 = -1;

// ---------------------------------------------------------------------------
// AllocInfo — tracks a named variable's stack slot
// ---------------------------------------------------------------------------

/// Describes a named variable that has been allocated on the stack.
#[derive(Debug, Clone)]
struct AllocInfo {
    /// The `ValueId` returned by the `Alloca` instruction.
    ptr: ValueId,
    /// The type of the stored value (pointee type for the alloca).
    ty: IrType,
}

// ---------------------------------------------------------------------------
// LoopInfo — saved context for `vunja` / `endelea`
// ---------------------------------------------------------------------------

/// Records the header and exit blocks of the innermost loop so that `break`
/// and `continue` can target them.
#[derive(Debug, Clone, Copy)]
struct LoopInfo {
    /// The block that tests the loop condition (where `continue` jumps).
    header: BlockId,
    /// The block immediately following the loop (where `break` jumps).
    exit: BlockId,
}

// ---------------------------------------------------------------------------
// Lowerer
// ---------------------------------------------------------------------------

/// Stateful AST → IR lowering context.
///
/// The lowerer walks the AST depth-first, accumulating functions, globals,
/// strings, and type definitions into a [`Module`].
struct Lowerer<'a> {
    // -- AST arrays (borrowed) ------------------------------------------------
    ast_aina: &'a [u32],
    ast_kushoto: &'a [i32],
    ast_kulia: &'a [i32],
    ast_tiga: &'a [i32],
    ast_nne: &'a [i32],
    ast_thamani: &'a [i32],
    ast_jina_off: &'a [i32],
    ast_pool: &'a [u8],

    // -- Accumulated module pieces --------------------------------------------
    functions: Vec<Function>,
    globals: Vec<IrGlobal>,
    types: Vec<(String, IrType)>,
    /// Collected string literals: (symbol name, raw bytes without terminator).
    /// Each unique string gets a synthetic global name like `@str.0`, `@str.1`, ...
    strings: Vec<(String, Vec<u8>)>,

    // -- Current function under construction ----------------------------------
    /// The function being lowered right now.
    func: Function,

    // -- Scope chain ----------------------------------------------------------
    /// Each push is a new lexical scope (function body, block, etc.).
    /// When looking up a name we walk from the innermost scope outward.
    scopes: Vec<HashMap<String, AllocInfo>>,

    // -- Loop context stack ---------------------------------------------------
    /// The innermost loop is at the end; `break` / `continue` target it.
    loops: Vec<LoopInfo>,

    // -- Counters -------------------------------------------------------------
    /// Global instruction counter: monotonically increases across all blocks.
    inst_counter: usize,
    /// values.len() captured at function start — must not change during body lowering.
    values_initial_len: usize,
    /// Monotonically increasing block-id counter used for fresh labels.
    block_counter: usize,

    /// Types for global variables (for lower_identifier).
    global_types: std::collections::HashMap<String, IrType>,
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Lower a flat-array Swa AST into an IR [`Module`].
///
/// # Parameters
///
/// * `ast_aina`      — node-type array
/// * `ast_kushoto`   — left / first-child array
/// * `ast_kulia`     — right-child array
/// * `ast_tiga`      — else-branch / for-step / body array
/// * `ast_nne`       — sibling-chain array
/// * `ast_thamani`   — encoded integer value or type-name pool offset
/// * `ast_jina_off`  — offset into `ast_pool` for identifier names
/// * `ast_pool`      — string pool bytes
/// * `ast_idadi`     — total number of allocated AST nodes
///
/// # Panics
///
/// Panics if `ast_idadi == 0` (empty AST).
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
        global_types: std::collections::HashMap::new(),
    };

    // Root is the last node allocated; it must be AST_PROGRAMU.
    let root = (ast_idadi - 1) as i32;
    let root_kind = lr.node_aina(root);
    assert_eq!(
        root_kind, AST_PROGRAMU,
        "lower: root node is not PROGRAMU (got {})",
        root_kind
    );

    // Pre-pass: register all struct types first, so function parameter types
    // can resolve struct references via self.types.
    let mut child = lr.ast_kushoto[root as usize];
    while child != NO_NODE {
        if lr.node_aina(child) == AST_MUUNDO {
            lr.lower_muundo(child);
        }
        child = lr.ast_nne[child as usize];
    }

    // Main pass: lower functions and globals (structs already done).
    let mut child = lr.ast_kushoto[root as usize];
    while child != NO_NODE {
        let kind = lr.node_aina(child);
        match kind {
            AST_KAZI => lr.lower_function(child),
            AST_TANGAZO_ULIMWENGU => lr.lower_global(child),
            AST_MUUNDO => {} // already done in pre-pass
            other => {
                let _ = other;
            }
        }
        child = lr.ast_nne[child as usize];
    }

    // Build module.
    let strings: Vec<IrGlobal> = lr
        .strings
        .iter()
        .enumerate()
        .map(|(i, (label, bytes))| {
            let name = format!("@str.{}", i);
            // Append null terminator if not already present.
            let mut data = bytes.clone();
            if data.last() != Some(&0) {
                data.push(0);
            }
            IrGlobal {
                name,
                bytes: data,
                is_const: true,
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
// Lowerer helpers — pool access
// ============================================================================

impl<'a> Lowerer<'a> {
    /// Read the node kind for `node_idx`, returning 0 for `NO_NODE`.
    #[inline]
    fn node_aina(&self, idx: i32) -> u32 {
        if idx == NO_NODE || idx < 0 {
            return 0;
        }
        self.ast_aina[idx as usize]
    }

    /// Read a null-terminated UTF-8 name from the string pool at `offset`.
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

    /// Read a length-prefixed byte sequence from the string pool at `offset`.
    ///
    /// Format: 4-byte LE `u32` length followed by that many raw bytes (no
    /// terminator).  Falls back to null-terminated if the length looks
    /// unreasonable.
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
            // Fallback: treat as null-terminated.
            let mut end = off;
            while end < self.ast_pool.len() && self.ast_pool[end] != 0 {
                end += 1;
            }
            self.ast_pool[off..end].to_vec()
        }
    }

    /// Read a type name from the pool at the offset stored in `ast_thamani[idx]`.
    fn read_type_from_thamani(&self, idx: i32) -> IrType {
        // ast_thamani stores an encoded type integer, not a pool offset.
        // Encoding: ((familia & 255) << 8) | (upana & 255), with bit 0 = pointer flag.
        if idx == NO_NODE || idx < 0 {
            return IrType::Void;
        }
        let enc_raw = self.ast_thamani[idx as usize];
        // Negative values: user struct name stored as pool offset.
        // -(offset) = struct by value; -(offset | 1) = struct pointer.
        if enc_raw < 0 {
            let neg = (-enc_raw) as u32;
            let mshale = neg & 1;
            let off = (neg >> 1) as usize;
            let name = self.read_pool_name(off as i32);
            // Try to find the struct in the registered types, otherwise create placeholder.
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
        let familia = (enc >> 11) & 255;
        let upana_idx = (enc >> 3) & 7;
        let mshale = enc & 7;
        let upana = match upana_idx { 0=>0, 1=>1, 2=>8, 3=>16, 4=>32, 5=>64, 6=>128, _=>32 };
        let base = match familia {
            1 => match upana { 8 => IrType::I8, 16 => IrType::I16, 32 => IrType::I32, 64 => IrType::I64, 128 => IrType::I128, _ => IrType::I32 },
            2 => match upana { 8 => IrType::U8, 16 => IrType::U16, 32 => IrType::U32, 64 => IrType::U64, 128 => IrType::U128, _ => IrType::U32 },
            3 => match upana { 16 => IrType::F16, 32 => IrType::F32, 64 => IrType::F64, 80 => IrType::F64, 128 => IrType::F64, _ => IrType::F64 },
            4 => match upana { 1 => IrType::B1, 8 => IrType::B8, 16 => IrType::B16, 32 => IrType::B32, 64 => IrType::B64, _ => IrType::B1 },
            5 => match upana { 0 => IrType::Void, _ => IrType::Void },
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

    /// Read a type from a node's `ast_thamani` field (the node IS the type
    /// specifier, not a parent referencing it).
    fn read_type_from_node(&self, type_node: i32) -> IrType {
        if type_node == NO_NODE || type_node < 0 {
            return IrType::Void;
        }
        let kind = self.node_aina(type_node);
        match kind {
            // Pointer type: *T
            28 /* NYOTA */ => {
                let inner = self.ast_kushoto[type_node as usize];
                let inner_ty = self.read_type_from_node(inner);
                IrType::Ptr(Box::new(inner_ty))
            }
            // Named type reference via thamani
            _ => {
                let name_off = self.ast_thamani[type_node as usize];
                let name = self.read_pool_name(name_off);
                if name.is_empty() {
                    IrType::Void
                } else {
                    IrType::from_swa_type(&name).unwrap_or_else(|| {
                        // Look up the struct definition in already-registered types.
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
// Lowerer helpers — block / value management
// ============================================================================

impl<'a> Lowerer<'a> {
    /// Allocate a fresh block label and return the next `BlockId`.
    fn fresh_block_id(&mut self) -> BlockId {
        let id = BlockId(self.block_counter);
        self.block_counter += 1;
        id
    }

    /// Create a new block with the given label prefix (e.g. `"entry"`,
    /// `"then"`, `"loop_header"`) appended with the block counter for
    /// uniqueness, push it into the current function, and return its
    /// `BlockId`.
    fn new_block(&mut self, label_prefix: &str) -> BlockId {
        let label = format!("{}.{}", label_prefix, self.block_counter);
        self.block_counter += 1;
        // Use RetVoid as default — caller must overwrite with set_terminator.
        // Br to BlockId(0) creates false predecessors for the entry block.
        let block = IrBlock::new(label, Terminator::RetVoid);
        let id = self.func.push_block(block);
        id
    }

    /// Append an instruction to block `block_id` and return a fresh `ValueId`.
    /// ValueIds for instructions start at params.len() + values.len() (i.e. after
    /// Emit an instruction and return its ValueId.
    /// ValueId scheme matches codegen: params(N) + values(M) + instruction_position.
    /// instruction_position starts at 0 for the first instruction in each block.
    fn emit(&mut self, block_id: BlockId, inst: Instruction) -> ValueId {
        let block = &mut self.func.blocks[block_id.0];
        block.push(inst);
        let base = self.func.params.len() + self.values_initial_len;
        let vid = ValueId(base + self.inst_counter);
        self.inst_counter += 1;
        vid
    }

    /// Look up a previously-interned constant and return its `ValueId`.
    /// Panics if the constant was not pre-interned — all constants must be
    /// interned via `collect_constants` or the pre-intern list before lowering.
    fn const_val(&mut self, c: Const) -> ValueId {
        let idx = self.func.values.iter().position(|v| *v == c)
            .unwrap_or_else(|| panic!("const_val: {:?} not pre-interned", c));
        ValueId(self.func.params.len() + idx)
    }

    /// Set the terminator of the given block.
    fn set_terminator(&mut self, block_id: BlockId, term: Terminator) {
        self.func.blocks[block_id.0].terminator = term;
    }

    /// Look up a name in the scope chain (innermost first).  Returns `None` if
    /// the name is not found.
    fn lookup(&self, name: &str) -> Option<&AllocInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info);
            }
        }
        None
    }

    /// Push a new, empty scope onto the scope chain.
    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Pop the innermost scope.
    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Register a variable in the innermost scope.
    fn define_var(&mut self, name: String, ptr: ValueId, ty: IrType) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, AllocInfo { ptr, ty });
        }
    }
}

// ============================================================================
// Top-level lowering: functions and globals
// ============================================================================

impl<'a> Lowerer<'a> {
    /// Lower a function definition (`AST_KAZI`, node 2).
    ///
    /// Layout:
    /// * `ast_jina_off[node]`  → function name
    /// * `ast_thamani[node]`   → return-type pool offset
    /// * `ast_kulia[node]`     → first parameter node (chained via `ast_nne`)
    /// * `ast_tiga[node]`      → function body (block or expression)
    fn lower_function(&mut self, func_node: i32) {
        // The function's name is stored on the name_node (ast_kushoto),
        // not on the AST_KAZI node itself.
        let name_node = self.ast_kushoto[func_node as usize];
        let name = if name_node != NO_NODE {
            self.read_pool_name(self.ast_jina_off[name_node as usize])
        } else {
            String::new()
        };
        let ret_ty = self.read_type_from_thamani(func_node);

        // -- Collect parameters -------------------------------------------------
        let mut params: Vec<(String, IrType)> = Vec::new();
        let mut param_node = self.ast_kulia[func_node as usize];
        while param_node != NO_NODE {
            let pname = self.read_pool_name(self.ast_jina_off[param_node as usize]);
            // For parameter nodes, ast_thamani may be the type node index or a
            // pool offset.  Try reading via thamani first; if that yields Void
            // and there is a kushoto type child, read from there.
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

        // -- Build function -----------------------------------------------------
        self.func = Function::new(name.clone(), ret_ty.clone(), params.clone());

        // Record the return class.
        let rc = crate::abi::classify_return(&ret_ty);
        self.func.return_class = rc;
        self.func.source_return_ty = ret_ty.clone();

        // If sret, adjust the return type and add the hidden pointer parameter.
        let sret_ptr_vid = if rc == IrReturnClass::HiddenPtr && ret_ty != IrType::Void {
            let sret_ty = IrType::Ptr(Box::new(ret_ty.clone()));
            self.func.params.insert(0, ("_sret".to_string(), sret_ty.clone()));
            self.func.return_ty = IrType::Void;
            self.func.sret_value_id = Some(ValueId(0));
            true
        } else {
            false
        };

        // Pre-intern constants AFTER sret param is added, so params.len() is final.
        self.collect_constants(self.ast_tiga[func_node as usize]);

        // Pre-intern commonly-used constants so const_val() never adds new
        // values during lowering (which would shift ValueIds out of sync
        // with the backend's N+M+I scheme).
        self.func.intern_const(Const::Zero);
        self.func.intern_const(Const::Int(0));
        self.func.intern_const(Const::Int(1));
        self.func.intern_const(Const::Int(-1));
        self.func.intern_const(Const::Bool(false));
        self.func.intern_const(Const::Bool(true));
        self.func.intern_const(Const::NullPtr);

        self.values_initial_len = self.func.values.len();
        self.inst_counter = 0;

        // -- Create entry block -------------------------------------------------
        self.scopes.clear();
        self.loops.clear();
        self.push_scope();

        let entry_id = self.new_block("entry");
        self.func.entry = entry_id;

        // -- Lower parameters into stack slots ----------------------------------
        // Parameter ValueIds are 0..N-1 in the same order as self.func.params.
        // If sret is active, the hidden pointer is ValueId(0) and user-visible
        // params start at ValueId(1).  The loop index `i` already accounts for
        // this — we skip i=0 when sret is active.

        // Clone params to avoid borrow conflict with self.emit below.
        let params: Vec<_> = self.func.params.iter().cloned().collect();
        for (i, (pname, pty)) in params.iter().enumerate() {
            // Skip the sret pointer — it is lowered separately.
            if sret_ptr_vid && i == 0 {
                continue;
            }
            let alloc = self.emit(entry_id, Instruction::Alloca(pty.clone()));
            // Parameter value: ValueId(i). (If sret, param i=1 gets ValueId(1),
            // which correctly skips ValueId(0).)
            let param_vid = ValueId(i);
            self.emit(entry_id, Instruction::Store(param_vid, alloc));
            self.define_var(pname.clone(), alloc, pty.clone());
        }

        // -- Lower body ---------------------------------------------------------
        let body_node = self.ast_tiga[func_node as usize];
        let body_block_id = self.lower_block(body_node);

        // Link entry → body.
        self.set_terminator(entry_id, Terminator::Br(body_block_id));

        // -- Finalise -----------------------------------------------------------
        self.pop_scope();
        self.functions.push(std::mem::replace(
            &mut self.func,
            Function::new("", IrType::Void, vec![]),
        ));
    }

    /// Lower a global variable (`AST_TANGAZO_ULIMWENGU`, node 35).
    ///
    /// Parser layout:
    /// * `ast_kushoto[node]` → name identifier node (jina_off set there)
    /// * `ast_thamani[node]` → encoded return type (integer)
    /// * `ast_kulia[node]`   → initialiser expression (if present)
    fn lower_global(&mut self, glob_node: i32) {
        // Name: parser stores via name_node in kushoto; fallback to direct jina_off.
        let name_node = self.ast_kushoto[glob_node as usize];
        let name = if name_node != NO_NODE {
            self.read_pool_name(self.ast_jina_off[name_node as usize])
        } else {
            self.read_pool_name(self.ast_jina_off[glob_node as usize])
        };
        let ty = self.read_type_from_thamani(glob_node);
        if !name.is_empty() {
            self.global_types.insert(name.clone(), ty.clone());
        }

        // Initialiser: parser stores in kulia; fallback to tiga (test format).
        let init_node = if self.ast_kulia[glob_node as usize] != NO_NODE {
            self.ast_kulia[glob_node as usize]
        } else {
            self.ast_tiga[glob_node as usize]
        };
        let _init_node = init_node;

        // Evaluate initialiser if present.  Since we can't run code at compile
        // time, we only support constant initialisers.  For now just create
        // zero-initialised globals.
        let size = ty.width_bytes();
        let bytes = vec![0u8; size];

        self.globals.push(IrGlobal {
            name,
            bytes,
            is_const: false,
        });
    }

    /// Register a struct definition in the module's type table.
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
// Statement lowering
// ============================================================================

impl<'a> Lowerer<'a> {
    /// Walk the AST and pre-intern all integer constants so that
    /// `func.values.len()` is stable before instruction ValueId assignment.
    fn collect_constants(&mut self, node: i32) {
        if node == NO_NODE || node < 0 { return; }
        let idx = node as usize;
        if idx >= self.ast_aina.len() { return; }
        let kind = self.ast_aina[idx];
        if kind == AST_NAMBARI {
            let val = self.ast_thamani[idx] as i128;
            self.func.intern_const(Const::Int(val));
            // Also visit nne: call argument chains use nne, so a literal
            // may be followed by another argument (e.g. fread(..., 1, 262144)).
            self.collect_constants(self.ast_nne[idx]);
            return;
        }
        self.collect_constants(self.ast_kushoto[idx]);
        self.collect_constants(self.ast_kulia[idx]);
        self.collect_constants(self.ast_tiga[idx]);
        self.collect_constants(self.ast_nne[idx]);
    }

    /// Lower a statement node (or an expression used as a statement).
    ///
    /// Returns the `BlockId` of the *continuation* block — the block to which
    /// control flows after this statement completes normally.
    fn lower_stmt(&mut self, node: i32) -> BlockId {
        if node == NO_NODE || node < 0 {
            // Empty statement → create a trivial pass-through block.
            let blk = self.new_block("empty");
            self.set_terminator(blk, Terminator::Br(blk));
            return blk;
        }

        let kind = self.node_aina(node);
        match kind {
            // ---- compound / expression-as-stmt --------------------------------
            AST_ASIMILIA => self.lower_assign(node),
            AST_KAMA => self.lower_if(node),
            AST_WAKATI => self.lower_while(node),
            AST_RUDISHA => self.lower_return(node),
            AST_TANGAZO => self.lower_local_decl(node),
            AST_CHAGUA => self.lower_switch(node),
            AST_VUNJA => self.lower_break(node),
            AST_ENDELEA => self.lower_continue(node),
            AST_TENGA => self.lower_heap_alloc_stmt(node),
            AST_ACHILIA => self.lower_heap_free_stmt(node),
            AST_WITO => {
                // Expression statement (call result discarded).
                let blk = self.new_block("call_stmt");
                let (_val, end_blk) = self.lower_expr_into(node, blk);
                // Set a placeholder terminator so lower_block can chain it.
                self.set_terminator(end_blk, Terminator::Br(end_blk));
                end_blk
            }
            // ---- expression as statement --------------------------------------
            _ => {
                // Any expression node used as statement: evaluate and discard.
                let blk = self.new_block("expr_stmt");
                let (_val, end_blk) = self.lower_expr_into(node, blk);
                // Set a placeholder terminator so lower_block can chain it
                // (short-circuit operators set their own terminators; don't overwrite).
                self.patch_br_if_needed(end_blk, end_blk);
                end_blk
            }
        }
    }

    /// Lower a block (a chain of statements linked via `ast_nne`).
    ///
    /// Returns the `BlockId` of the entry block for this sequence.
    fn lower_block(&mut self, first_stmt: i32) -> BlockId {
        if first_stmt == NO_NODE || first_stmt < 0 {
            let blk = self.new_block("empty_body");
            // Use a self-loop placeholder; the caller or finaliser will fix it.
            self.set_terminator(blk, Terminator::Br(blk));
            return blk;
        }

        // Walk the statement chain, linking each statement to the next.
        let mut current = first_stmt;
        let entry_id = self.new_block("body");
        let mut prev_block = entry_id;
        let mut is_first = true;

        while current != NO_NODE && current >= 0 {
            let next_stmt = self.ast_nne[current as usize];
            let stmt_blk = self.lower_stmt(current);

            if is_first {
                // The first statement's entry IS our block's entry.
                // Patch the empty entry to branch to the first statement.
                self.set_terminator(prev_block, Terminator::Br(stmt_blk));
                prev_block = stmt_blk;
                is_first = false;
            } else {
                // Chain prev block → stmt_blk when prev has a fall-through
                // (RetVoid default or self-loop placeholder Br(prev)).
                let prev_term = &self.func.blocks[prev_block.0].terminator;
                let needs_chain = matches!(prev_term, Terminator::RetVoid)
                    || matches!(prev_term, Terminator::Br(b) if *b == prev_block);
                if needs_chain {
                    self.set_terminator(prev_block, Terminator::Br(stmt_blk));
                }
                prev_block = stmt_blk;
            }

            current = next_stmt;
        }

        // If the last statement didn't set a real terminator, add a self-loop
        // placeholder.  The caller (or a later pass) is responsible for
        // replacing this with Ret/RetVoid appropriate for the function.
        let last_block = prev_block;
        let needs_term = match &self.func.blocks[last_block.0].terminator {
            Terminator::Br(b) if *b == last_block => true, // placeholder
            Terminator::RetVoid => true, // default from new_block
            _ => false,
        };
        if needs_term {
            self.set_terminator(last_block, Terminator::Br(last_block));
        }

        entry_id
    }

    /// Lower a statement or block — if `node` looks like a block chain (first
    /// child points to a statement and has `ast_nne` siblings), lower as block;
    /// otherwise lower as single statement.
    fn lower_block_or_stmt(&mut self, node: i32) -> BlockId {
        if node == NO_NODE || node < 0 {
            let blk = self.new_block("empty_body");
            self.set_terminator(blk, Terminator::RetVoid);
            return blk;
        }

        let kind = self.node_aina(node);
        // If the node is a compound statement container, walk its children.
        // The body is typically represented as a chain via ast_nne.
        match kind {
            // For a raw block (brace-enclosed), ast_kushoto points to first stmt,
            // and they are chained via ast_nne.
            _ => {
                // Check if this node has children via ast_kushoto.
                let first = self.ast_kushoto[node as usize];
                if first != NO_NODE {
                    self.lower_block(first)
                } else {
                    // Single expression/statement as body.
                    self.lower_stmt(node)
                }
            }
        }
    }
}

// ============================================================================
// Statement lowering — individual statement kinds
// ============================================================================

impl<'a> Lowerer<'a> {
    /// Lower `ASIMILIA` (assignment): `target = value`.
    ///
    /// Layout:
    /// * `ast_kushoto[node]` → lvalue
    /// * `ast_kulia[node]`   → rvalue expression
    fn lower_assign(&mut self, node: i32) -> BlockId {
        let lhs_node = self.ast_kushoto[node as usize];
        let rhs_node = self.ast_kulia[node as usize];

        let blk = self.new_block("assign");
        let (rhs_val, end_blk) = self.lower_expr_into(rhs_node, blk);

        // Lower the lvalue to get a pointer to store into.
        let ptr = self.lower_lvalue(lhs_node, end_blk);
        self.emit(end_blk, Instruction::Store(rhs_val, ptr));
        self.set_terminator(end_blk, Terminator::Br(end_blk)); // fall-through placeholder, caller chains
        end_blk
    }

    /// Lower `KAMA` (if): `kama (cond) then_block [tiga else_block]`.
    ///
    /// Layout:
    /// * `ast_kushoto[node]` → condition expression
    /// * `ast_kulia[node]`   → then branch
    /// * `ast_tiga[node]`    → else branch (optional, -1 if absent)
    fn lower_if(&mut self, node: i32) -> BlockId {
        let cond_node = self.ast_kushoto[node as usize];
        let then_node = self.ast_kulia[node as usize];
        let else_node = self.ast_tiga[node as usize];

        let cond_blk = self.new_block("if.cond");
        let (cond_val, cond_end) = self.lower_expr_into(cond_node, cond_blk);

        let then_blk = self.lower_block(then_node);
        let merge_blk = self.new_block("if.merge");

        // Branch from condition.
        if else_node != NO_NODE && else_node >= 0 {
            let else_blk = self.lower_block(else_node);
            self.set_terminator(
                cond_end,
                Terminator::BrCond(cond_val, then_blk, else_blk),
            );
            // Link then → merge and else → merge.
            // Only link if they don't already have a non-placeholder terminator.
            self.patch_br_if_needed(then_blk, merge_blk);
            self.patch_br_if_needed(else_blk, merge_blk);
        } else {
            self.set_terminator(
                cond_end,
                Terminator::BrCond(cond_val, then_blk, merge_blk),
            );
            self.patch_br_if_needed(then_blk, merge_blk);
        }

        merge_blk
    }

    /// Lower `WAKATI` (while loop): `wakati (cond) { body }`.
    ///
    /// Layout:
    /// * `ast_kushoto[node]` → condition expression
    /// * `ast_tiga[node]`    → loop body
    fn lower_while(&mut self, node: i32) -> BlockId {
        let cond_node = self.ast_kushoto[node as usize];
        let body_node = self.ast_tiga[node as usize];

        let header_blk = self.new_block("while.header");
        let body_blk = self.new_block("while.body");
        let exit_blk = self.new_block("while.exit");

        // Push loop context so that `break` → exit, `continue` → header.
        self.loops.push(LoopInfo {
            header: header_blk,
            exit: exit_blk,
        });

        // Header: evaluate condition, branch to body or exit.
        let (cond_val, cond_end) = self.lower_expr_into(cond_node, header_blk);
        self.set_terminator(
            cond_end,
            Terminator::BrCond(cond_val, body_blk, exit_blk),
        );

        // Lower the body statements — returns the entry block of the body chain.
        let body_entry = self.lower_block(body_node);
        // Wire the while.body block to the lowered body's entry.
        self.set_terminator(body_blk, Terminator::Br(body_entry));

        // Find the last block in the body chain (the one that falls through)
        // and wire it back to the loop header.
        let mut last = body_entry;
        loop {
            let term = &self.func.blocks[last.0].terminator;
            match term {
                Terminator::Br(target) if *target != last => {
                    // Follow the chain forward.
                    last = *target;
                }
                Terminator::Br(_) => {
                    // Self-loop placeholder — this is the last block.
                    break;
                }
                _ => {
                    // Real terminator (Ret, BrCond, Switch) — stop here.
                    break;
                }
            }
        }
        self.ensure_br(last, header_blk);

        self.loops.pop();
        exit_blk
    }

    /// Lower `KIPINDI` (for loop): `kipindi (init; cond; step) { body }`.
    ///
    /// Layout:
    /// * `ast_kushoto[node]` → initialiser
    /// * `ast_kulia[node]`   → condition
    /// * `ast_tiga[node]`    → step expression
    /// * `ast_nne[node]`     → loop body
    fn lower_for(&mut self, node: i32) -> BlockId {
        let init_node = self.ast_kushoto[node as usize];
        let cond_node = self.ast_kulia[node as usize];
        let step_node = self.ast_tiga[node as usize];
        let body_node = self.ast_nne[node as usize];

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

        self.set_terminator(init_blk, Terminator::Br(header_blk));

        // Push loop context.
        self.loops.push(LoopInfo {
            header: header_blk,
            exit: exit_blk,
        });

        // Header: evaluate condition.
        let (cond_val, cond_end) = if cond_node != NO_NODE && cond_node >= 0 {
            self.lower_expr_into(cond_node, header_blk)
        } else {
            // No condition → unconditional loop.
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

        // Body.
        let body_end = self.lower_block(body_node);
        self.set_terminator(body_end, Terminator::Br(step_blk));

        // Step.
        let step_end = if step_node != NO_NODE && step_node >= 0 {
            self.lower_stmt(step_node)
        } else {
            step_blk
        };
        self.set_terminator(step_end, Terminator::Br(header_blk));

        self.loops.pop();
        exit_blk
    }

    /// Lower `RUDISHA` (return): `rudisha [expr]`.
    ///
    /// Layout:
    /// * `ast_kushoto[node]` → return value expression (optional, -1 for void return)
    fn lower_return(&mut self, node: i32) -> BlockId {
        let val_node = self.ast_kushoto[node as usize];
        let blk = self.new_block("ret");

        if val_node != NO_NODE && val_node >= 0 {
            let (val, end_blk) = self.lower_expr_into(val_node, blk);
            // If sret, store to the sret pointer then RetVoid.
            if let Some(sret_vid) = self.func.sret_value_id {
                self.emit(end_blk, Instruction::Store(val, sret_vid));
                self.set_terminator(end_blk, Terminator::RetVoid);
            } else {
                self.set_terminator(end_blk, Terminator::Ret(val));
            }
            end_blk
        } else {
            // No explicit return value.
            if self.func.sret_value_id.is_some() {
                // sret with no explicit value — return void.
                self.set_terminator(blk, Terminator::RetVoid);
            } else if self.func.source_return_ty == IrType::Void {
                // Void function — return void.
                self.set_terminator(blk, Terminator::RetVoid);
            } else {
                // Non-void function with no explicit return: return zero.
                let zero = self.const_val(Const::Int(0));
                self.set_terminator(blk, Terminator::Ret(zero));
            }
            blk
        }
    }

    /// Lower `TANGAZO` (local variable declaration): `let name [: type] [= init]`.
    ///
    /// Parser format:
    /// * `ast_kushoto[node]` → variable name (identifier node)
    /// * `ast_thamani[node]` → encoded type integer (familia, upana, mshale)
    /// * `ast_kulia[node]`   → optional initialiser expression
    ///
    /// Test/legacy format (when ast_thamani[node]==0):
    /// * `ast_kulia[node]`   → type node
    /// * `ast_tiga[node]`    → optional initialiser expression
    fn lower_local_decl(&mut self, node: i32) -> BlockId {
        let name_node = self.ast_kushoto[node as usize];
        let var_name = self.read_pool_name(self.ast_jina_off[name_node as usize]);

        // Detect format: parser puts encoded type in thamani, tests put type node in kulia.
        let (var_ty, init_node) = if self.ast_thamani[node as usize] != 0 {
            // Parser format: type encoded in thamani, init in kulia.
            let ty = self.read_type_from_thamani(node);
            let init = self.ast_kulia[node as usize];
            (ty, init)
        } else {
            // Test/legacy format: type node in kulia, init in tiga.
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

        // Allocate stack slot.
        let alloc = self.emit(blk, Instruction::Alloca(var_ty.clone()));

        // Evaluate initialiser and store.
        if init_node != NO_NODE && init_node >= 0 {
            let (init_val, end_blk) = self.lower_expr_into(init_node, blk);
            self.emit(end_blk, Instruction::Store(init_val, alloc));
            self.define_var(var_name, alloc, var_ty);
            self.set_terminator(end_blk, Terminator::Br(end_blk));
            end_blk
        } else {
            self.define_var(var_name, alloc, var_ty.clone());
            // Zero-initialise.
            let zero = self.const_val(Const::Zero);
            self.emit(blk, Instruction::Store(zero, alloc));
            self.set_terminator(blk, Terminator::Br(blk));
            blk
        }
    }

    /// Lower `CHAGUA` (switch): `chagua (scrutinee) { visa: [arms] tivyo: default }`.
    ///
    /// Layout:
    /// * `ast_kushoto[node]` → scrutinee expression
    /// * `ast_kulia[node]`   → first case arm (chained via ast_nne)
    ///   Each case arm: ast_kushoto = case label, ast_tiga = case body.
    /// * `ast_tiga[node]`    → default body
    fn lower_switch(&mut self, node: i32) -> BlockId {
        let scrut_node = self.ast_kushoto[node as usize];
        let first_case = self.ast_kulia[node as usize];
        let default_node = self.ast_tiga[node as usize];

        let scrut_blk = self.new_block("switch.scrut");
        let (scrut_val, scrut_end) = self.lower_expr_into(scrut_node, scrut_blk);

        let merge_blk = self.new_block("switch.merge");

        // Lower default arm.
        let default_blk = if default_node != NO_NODE && default_node >= 0 {
            self.lower_block(default_node)
        } else {
            merge_blk
        };

        // Lower case arms.
        let mut arms: Vec<(ValueId, BlockId)> = Vec::new();
        let mut case_node = first_case;
        while case_node != NO_NODE && case_node >= 0 {
            let label_node = self.ast_kushoto[case_node as usize];
            let body_node = self.ast_tiga[case_node as usize];

            let case_blk = self.new_block("switch.case");
            let (label_val, case_label_end) = self.lower_expr_into(label_node, case_blk);
            // Terminate the label block by jumping to the case body.
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

    /// Lower `VUNJA` (break): jump to the innermost loop's exit block.
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

    /// Lower `ENDELEA` (continue): jump to the innermost loop's header block.
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

    /// Lower `TENGA` (heap allocate): `tenga <type>` or `tenga <size_expr>`.
    ///
    /// Layout:
    /// * `ast_kushoto[node]` → size expression or type node
    fn lower_heap_alloc_stmt(&mut self, node: i32) -> BlockId {
        let arg_node = self.ast_kushoto[node as usize];
        let blk = self.new_block("heap_alloc");

        let (size_val, end_blk) = self.lower_expr_into(arg_node, blk);
        self.emit(end_blk, Instruction::HeapAlloc(size_val));
        // The pointer result is discarded in statement context.
        self.set_terminator(end_blk, Terminator::Br(end_blk));
        end_blk
    }

    /// Lower `ACHILIA` (heap free): `achilia <ptr_expr>`.
    ///
    /// Layout:
    /// * `ast_kushoto[node]` → pointer expression
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
// Expression lowering
// ============================================================================

impl<'a> Lowerer<'a> {
    /// Lower an expression node.
    ///
    /// Returns `(ValueId, BlockId)` where `ValueId` holds the expression result
    /// and `BlockId` is the block that *ends* with that value being available
    /// (i.e. the block containing the producing instruction).  For short-circuit
    /// operators the returned block is the merge block.
    fn lower_expr(&mut self, node: i32, current_block: BlockId) -> (ValueId, BlockId) {
        self.lower_expr_into(node, current_block)
    }

    /// Lower an expression into the given block (or chain of blocks for
    /// short-circuit operators).  Returns `(value, end_block)`.
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

            // -- arithmetic ----------------------------------------------------
            AST_JUMLISHA => {
                // AST_JUMLISHA (6) with kushoto=NO_NODE is unary plus (no-op).
                // AST_JUMLISHA (6) with kushoto=left is binary add.
                if self.ast_kushoto[node as usize] == NO_NODE {
                    // Unary plus — just evaluate the right operand.
                    self.lower_expr_into(self.ast_kulia[node as usize], current_block)
                } else {
                    self.lower_binary_op(node, current_block, |l, r| Instruction::Add(l, r))
                }
            }
            AST_TOFAUTI => {
                // AST_TOFAUTI (7) with kushoto=NO_NODE is unary minus.
                // AST_TOFAUTI (7) with kushoto=left is binary subtract.
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

            // -- bitwise -------------------------------------------------------
            AST_HAMISHA_KUSHOTO => self.lower_binary_op(node, current_block, |l, r| Instruction::Shl(l, r)),
            AST_HAMISHA_KULIA => self.lower_binary_op(node, current_block, |l, r| Instruction::ShrS(l, r)),
            AST_BIT_NA => self.lower_binary_op(node, current_block, |l, r| Instruction::And(l, r)),
            AST_BIT_AU => self.lower_binary_op(node, current_block, |l, r| Instruction::Or(l, r)),
            AST_TERNARY => self.lower_ternary(node, current_block),
            AST_NA => self.lower_short_circuit_and(node, current_block),
            AST_AU => self.lower_short_circuit_or(node, current_block),

            // -- comparisons ---------------------------------------------------
            AST_SAWA => self.lower_binary_op(node, current_block, |l, r| Instruction::Eq(l, r)),
            AST_TOFAUTI_SI => self.lower_binary_op(node, current_block, |l, r| Instruction::Ne(l, r)),
            AST_CHINI => self.lower_binary_op(node, current_block, |l, r| Instruction::LtS(l, r)),
            AST_JUU => self.lower_binary_op(node, current_block, |l, r| Instruction::GtS(l, r)),
            AST_CHINI_SAWA => self.lower_binary_op(node, current_block, |l, r| Instruction::LeS(l, r)),
            AST_JUU_SAWA => self.lower_binary_op(node, current_block, |l, r| Instruction::GeS(l, r)),

            // -- unary ---------------------------------------------------------
            AST_SI => self.lower_logical_not(node, current_block),
            AST_TAJA => self.lower_deref_load(node, current_block),
            AST_KUMBUKA => self.lower_address_of(node, current_block),

            // -- member / element access ---------------------------------------
            AST_SEHEMU_DOT => self.lower_field_access(node, current_block),
            AST_SEHEMU_MSHALE => self.lower_ptr_field_access(node, current_block),
            AST_SAFU => self.lower_array_index(node, current_block),

            // -- heap alloc as expression --------------------------------------
            AST_TENGA => {
                let arg_node = self.ast_kushoto[node as usize];
                let (size_val, end_blk) = self.lower_expr_into(arg_node, current_block);
                let ptr = self.emit(end_blk, Instruction::HeapAlloc(size_val));
                (ptr, end_blk)
            }

            // -- assignment as expression (returns the assigned value) ---------
            AST_ASIMILIA => {
                let lhs_node = self.ast_kushoto[node as usize];
                let rhs_node = self.ast_kulia[node as usize];
                let (rhs_val, end_blk) = self.lower_expr_into(rhs_node, current_block);
                let ptr = self.lower_lvalue(lhs_node, end_blk);
                self.emit(end_blk, Instruction::Store(rhs_val, ptr));
                (rhs_val, end_blk)
            }

            _ => {
                // Unknown expression: return zero.
                let v = self.const_val(Const::Zero);
                (v, current_block)
            }
        }
    }

    // -- literal helpers -------------------------------------------------------

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
        let bytes = self.read_pool_bytes(offset);
        // Generate a label and record the string (the global is emitted later).
        let label = format!("@str.{}", self.strings.len());
        self.strings.push((label.clone(), bytes));
        // Emit a StringAddr instruction that references the global label.
        let ptr = self.emit(blk, Instruction::StringAddr(label));
        (ptr, blk)
    }

    // -- identifier ------------------------------------------------------------

    fn lower_identifier(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let name = self.read_pool_name(self.ast_jina_off[node as usize]);
        if let Some(info) = self.lookup(&name) {
            let alloca_ptr = info.ptr;
            // For struct types, just return the alloca pointer (opaque pointers).
            if matches!(&info.ty, IrType::Struct { .. }) {
                return (alloca_ptr, blk);
            }
            let loaded_ty = info.ty.clone();
            let val = self.emit(blk, Instruction::Load(loaded_ty, alloca_ptr));
            (val, blk)
        } else if let Some(gty) = self.global_types.get(&name).cloned() {
            let addr = self.emit(blk, Instruction::GlobalAddr(name.clone()));
            // For array types (I8 = byte array like N8 chanzo_buf[524288]),
            // return the pointer directly (array-to-pointer decay).
            // For scalar types (I32 = N32 chanzo_urefu), load the value.
            if gty == IrType::I8 || gty == IrType::U8 {
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

    // -- call ------------------------------------------------------------------

    /// Scan the AST for a function definition by name to check struct return.
    fn find_function_returns_struct(&self, name: &str) -> bool {
        matches!(self.find_function_return_type(name), Some(IrType::Struct { .. }))
    }

    /// Scan the AST for a function's return type by name.
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
        // Parser stores args on callee_node's kulia: ast_kulia[callee_node] = first_arg.
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

        // Handle builtins that look like function calls.
        if callee_name == "ukubwa" {
            let (_, end_blk) = if first_arg != NO_NODE && first_arg >= 0 {
                self.lower_expr_into(first_arg, blk)
            } else {
                (self.const_val(Const::Int(0)), blk)
            };
            let size = self.const_val(Const::Int(4)); // default N32 size
            return (size, end_blk);
        }

        // Evaluate arguments.  Parser chains args via ast_nne to avoid
        // conflicting with each arg node's own ast_kulia children.
        let mut arg_vals: Vec<ValueId> = Vec::new();
        let mut current_block = blk;
        let mut arg_node = first_arg;
        while arg_node != NO_NODE && arg_node >= 0 {
            let (arg_val, end_blk) = self.lower_expr_into(arg_node, current_block);
            arg_vals.push(arg_val);
            current_block = end_blk;
            arg_node = self.ast_nne[arg_node as usize];
        }

        // Check if the called function returns a struct (needs sret pointer).
        // First check already-lowered functions, then scan AST for forward refs.
        let needs_sret = self.functions.iter().any(|f| f.name == callee_name && matches!(f.source_return_ty, IrType::Struct { .. }))
            || self.find_function_returns_struct(&callee_name);
        let (call_val, final_block) = if needs_sret {
            // Determine the actual struct type for the sret alloca.
            let struct_ty = self.functions.iter()
                .find(|f| f.name == callee_name)
                .map(|f| f.source_return_ty.clone())
                .or_else(|| {
                    // Forward ref: scan AST for the return type.
                    self.find_function_return_type(&callee_name)
                })
                .unwrap_or(IrType::I32);
            // Alloca space for the struct result and pass as first arg (sret).
            let sret_alloca = self.emit(current_block, Instruction::Alloca(struct_ty.clone()));
            let mut sret_args = vec![sret_alloca];
            sret_args.extend(arg_vals);
            let cv = self.emit(current_block, Instruction::Call(callee_name.clone(), sret_args));
            // Load the struct from the alloca to get the value.
            let loaded = self.emit(current_block, Instruction::Load(struct_ty, sret_alloca));
            (loaded, current_block)
        } else {
            let cv = self.emit(current_block, Instruction::Call(callee_name.clone(), arg_vals));
            (cv, current_block)
        };
        (call_val, final_block)
    }

    // -- binary operations -----------------------------------------------------

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

    // -- unary negation --------------------------------------------------------

    fn lower_unary_neg(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let operand_node = self.ast_kushoto[node as usize];
        let (operand, end_blk) = self.lower_expr_into(operand_node, blk);
        let zero = self.const_val(Const::Int(0));
        let result = self.emit(end_blk, Instruction::Sub(zero, operand));
        (result, end_blk)
    }

    // -- logical not -----------------------------------------------------------

    fn lower_logical_not(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let operand_node = self.ast_kushoto[node as usize];
        let (operand, end_blk) = self.lower_expr_into(operand_node, blk);
        // NOT: compare operand == 0.
        let zero = self.const_val(Const::Int(0));
        let result = self.emit(end_blk, Instruction::Eq(operand, zero));
        (result, end_blk)
    }

    // -- short-circuit && (NA) -------------------------------------------------

    /// NA (logical AND) is short-circuit: evaluate left; if false, result is
    /// false; otherwise evaluate right.
    ///
    /// Lower ternary `cond ? true_val : false_val`.
    /// Uses the IR `Select` instruction — all three operands evaluated in the
    /// same block (no short-circuit).
    fn lower_ternary(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let cond_node = self.ast_kushoto[node as usize];
        let true_node = self.ast_kulia[node as usize];
        let false_node = self.ast_tiga[node as usize];

        let (cond_val, blk1) = self.lower_expr_into(cond_node, blk);
        // Convert condition to i1 (LLVM select requires i1 condition).
        let zero = self.const_val(Const::Int(0));
        let cond_bool = self.emit(blk1, Instruction::Ne(cond_val, zero));
        let (true_val, blk2) = self.lower_expr_into(true_node, blk1);
        let (false_val, blk3) = self.lower_expr_into(false_node, blk2);
        let result = self.emit(blk3, Instruction::Select(cond_bool, true_val, false_val));
        (result, blk3)
    }

    /// We lower short-circuit operators using an alloca for the result (phi
    /// replacement), storing the outcome from each predecessor and loading at
    /// the merge block.
    fn lower_short_circuit_and(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let lhs_node = self.ast_kushoto[node as usize];
        let rhs_node = self.ast_kulia[node as usize];

        // Lower both operands in the same block (no short-circuit for now —
        // Select evaluates both arms).  This avoids the alloca-in-cross-block
        // ValueId issue.  Proper short-circuit with phi nodes can be added
        // once the IR supports Phi instructions.
        let (lhs_val, blk1) = self.lower_expr_into(lhs_node, blk);
        let (rhs_val, blk2) = self.lower_expr_into(rhs_node, blk1);

        // Convert both to boolean: lhs != 0, rhs != 0, then AND them.
        let zero = self.const_val(Const::Int(0));
        let lhs_bool = self.emit(blk2, Instruction::Ne(lhs_val, zero));
        let rhs_bool = self.emit(blk2, Instruction::Ne(rhs_val, zero));
        let result = self.emit(blk2, Instruction::And(lhs_bool, rhs_bool));
        (result, blk2)
    }

    /// AU (logical OR) is short-circuit: evaluate left; if true, result is
    /// true; otherwise evaluate right.
    fn lower_short_circuit_or(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let lhs_node = self.ast_kushoto[node as usize];
        let rhs_node = self.ast_kulia[node as usize];

        let (lhs_val, blk1) = self.lower_expr_into(lhs_node, blk);
        let (rhs_val, blk2) = self.lower_expr_into(rhs_node, blk1);

        let zero = self.const_val(Const::Int(0));
        let lhs_bool = self.emit(blk2, Instruction::Ne(lhs_val, zero));
        let rhs_bool = self.emit(blk2, Instruction::Ne(rhs_val, zero));
        let result = self.emit(blk2, Instruction::Or(lhs_bool, rhs_bool));
        (result, blk2)
    }

    // -- pointer / address operations ------------------------------------------

    /// Lower `*expr` (pointer dereference / load).
    fn lower_deref_load(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let operand_node = self.ast_kushoto[node as usize];
        let (ptr_val, end_blk) = self.lower_expr_into(operand_node, blk);
        // We don't know the pointee type statically here, so use I8 and let
        // the backend fix it up.  A real semantic-analysis pass would provide
        // the correct type.
        let val = self.emit(end_blk, Instruction::Load(IrType::I8, ptr_val));
        (val, end_blk)
    }

    /// Lower `&expr` (address-of).
    fn lower_address_of(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let operand_node = self.ast_kushoto[node as usize];
        // The operand must be an lvalue — lower it to a pointer.
        let ptr = self.lower_lvalue(operand_node, blk);
        (ptr, blk)
    }

    // -- lvalue lowering -------------------------------------------------------

    /// Lower a node as an *lvalue*, returning a pointer (`ValueId`) that can be
    /// stored to or loaded from.
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
                    // Undefined — return null pointer.
                    self.const_val(Const::NullPtr)
                }
            }
            AST_TAJA => {
                // *ptr as lvalue: the pointer IS the address.
                let operand_node = self.ast_kushoto[node as usize];
                let (ptr_val, _end_blk) = self.lower_expr_into(operand_node, blk);
                ptr_val
            }
            AST_SEHEMU_DOT => {
                // struct.field: compute address of the field.
                // Field name is stored on the dot-access node itself via hifadhi_jina.
                let struct_node = self.ast_kushoto[node as usize];
                let field_name = self.read_pool_name(self.ast_jina_off[node as usize]);

                let base_ptr = self.lower_lvalue(struct_node, blk);

                // We need to know the field index.  In the absence of full type
                // info, use a simple hash-based index (placeholder).  Real
                // semantic analysis would provide the actual index.
                let field_idx = self.guess_field_index(&field_name);

                // Try to get struct type from the base pointer's type in scope.
                let struct_ty = if self.ast_aina[struct_node as usize] == AST_KITAMBULISHO {
                    let sname = self.read_pool_name(self.ast_jina_off[struct_node as usize]);
                    self.lookup(&sname).and_then(|info| {
                        match &info.ty {
                            IrType::Ptr(pointee) => Some((**pointee).clone()),
                            IrType::Struct { .. } => Some(info.ty.clone()),
                            _ => None,
                        }
                    })
                } else { None };
                self.emit(blk, Instruction::FieldAddr(base_ptr, field_idx, struct_ty))
            }
            AST_SEHEMU_MSHALE => {
                // ptr->field: load ptr, then compute field address.
                // Field name is stored on the arrow node itself via hifadhi_jina.
                let ptr_node = self.ast_kushoto[node as usize];
                let field_name = self.read_pool_name(self.ast_jina_off[node as usize]);

                let (struct_ptr, end_blk) = self.lower_expr_into(ptr_node, blk);
                let field_idx = self.guess_field_index(&field_name);

                self.emit(end_blk, Instruction::FieldAddr(struct_ptr, field_idx, None))
            }
            AST_SAFU => {
                // array[index] — compute element address via GEP.
                let array_node = self.ast_kushoto[node as usize];
                let index_node = self.ast_kulia[node as usize];

                let ary_ptr = self.lower_lvalue(array_node, blk);
                let (idx_val, end_blk) = self.lower_expr_into(index_node, blk);

                self.emit(end_blk, Instruction::Gep(ary_ptr, vec![idx_val]))
            }
            _ => {
                // Not an lvalue — evaluate as rvalue and return a dummy pointer.
                let (_val, _end_blk) = self.lower_expr_into(node, blk);
                self.const_val(Const::NullPtr)
            }
        }
    }

    // -- field access as rvalue ------------------------------------------------

    /// Lower `struct.field` (dot access) as an rvalue.
    /// Resolve the type of an expression node by walking the AST.
    fn resolve_expr_type(&self, node: i32) -> Option<IrType> {
        if node < 0 { return None; }
        match self.ast_aina[node as usize] {
            AST_KITAMBULISHO => {
                let name = self.read_pool_name(self.ast_jina_off[node as usize]);
                self.lookup(&name).map(|info| info.ty.clone())
                    .or_else(|| self.global_types.get(&name).cloned())
            }
            AST_SEHEMU_DOT => {
                // p.x → resolve p's type, find field x.
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
                // p->x → resolve p's type (pointer), get pointee struct, find field x.
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
            _ => None,
        }
    }

    fn lower_field_access(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let struct_node = self.ast_kushoto[node as usize];
        // Field name is stored on the dot-access node itself via hifadhi_jina.
        let field_name = self.read_pool_name(self.ast_jina_off[node as usize]);

        // Resolve the type of the left-hand-side expression, then find the field.
        let lhs_ty = self.resolve_expr_type(struct_node);
        let field_ty = match &lhs_ty {
            Some(IrType::Struct { fields, .. }) => {
                fields.iter().find(|(n, _)| n == &field_name).map(|(_, t)| t.clone())
                    .unwrap_or(IrType::I32)
            }
            _ => IrType::I32,
        };

        // First, get the address of the struct (as lvalue).
        let base_ptr = self.lower_lvalue(struct_node, blk);
        let field_idx = self.guess_field_index(&field_name);

        // Compute address of field, then load with correct type.
        let field_ptr = self.emit(blk, Instruction::FieldAddr(base_ptr, field_idx, None));
        let val = self.emit(blk, Instruction::Load(field_ty, field_ptr));
        (val, blk)
    }

    /// Lower `ptr->field` (arrow access) as an rvalue.
    fn lower_ptr_field_access(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let ptr_node = self.ast_kushoto[node as usize];
        // Field name is stored on the arrow-access node itself via hifadhi_jina.
        let field_name = self.read_pool_name(self.ast_jina_off[node as usize]);

        let (struct_ptr, end_blk) = self.lower_expr_into(ptr_node, blk);
        let field_idx = self.guess_field_index(&field_name);

        // Try to determine the struct type from the pointer's pointee type.
        // Look up the ptr_node name in scope to find its declared type.
        let struct_ty_opt = if ptr_node >= 0 && self.ast_aina[ptr_node as usize] == AST_KITAMBULISHO {
            let ptr_name = self.read_pool_name(self.ast_jina_off[ptr_node as usize]);
            self.lookup(&ptr_name).and_then(|info| {
                match &info.ty {
                    IrType::Ptr(pointee) => Some((**pointee).clone()),
                    _ => None,
                }
            })
        } else {
            None
        };

        // Determine field type from the resolved struct pointee.
        let field_ty = struct_ty_opt.as_ref().and_then(|sty| {
            if let IrType::Struct { fields, .. } = sty {
                fields.iter().find(|(n, _)| n == &field_name).map(|(_, t)| t.clone())
            } else { None }
        }).unwrap_or(IrType::I32);

        let field_ptr = self.emit(end_blk, Instruction::FieldAddr(struct_ptr, field_idx, struct_ty_opt));
        let val = self.emit(end_blk, Instruction::Load(field_ty, field_ptr));
        (val, end_blk)
    }

    /// Lower `array[index]` as an rvalue.
    fn lower_array_index(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let array_node = self.ast_kushoto[node as usize];
        let index_node = self.ast_kulia[node as usize];

        let ary_ptr = self.lower_lvalue(array_node, blk);
        let (idx_val, end_blk) = self.lower_expr_into(index_node, blk);

        // Determine element type from the array's declared type.
        let arr_ty = self.resolve_expr_type(array_node);
        let elem_ty = arr_ty.and_then(|ty| {
            match &ty {
                // Pointer: decay to pointee type (e.g. N8** → N8* → N8)
                IrType::Ptr(pointee) => Some((**pointee).clone()),
                _ => None,
            }
        }).unwrap_or(IrType::I32);

        // GEP to element, then load.
        let elem_ptr = self.emit(end_blk, Instruction::Gep(ary_ptr, vec![idx_val]));
        let val = self.emit(end_blk, Instruction::Load(elem_ty, elem_ptr));
        (val, end_blk)
    }

    // -- cast ------------------------------------------------------------------

    fn lower_cast(&mut self, node: i32, blk: BlockId) -> (ValueId, BlockId) {
        let expr_node = self.ast_kushoto[node as usize];
        let type_node = self.ast_kulia[node as usize];

        let target_ty = self.read_type_from_node(type_node);
        let (src_val, end_blk) = self.lower_expr_into(expr_node, blk);

        // Pick appropriate conversion based on target type.
        let result = if target_ty.is_float() {
            self.emit(end_blk, Instruction::SiToFp(src_val, target_ty))
        } else if target_ty == IrType::B1 {
            // To bool: compare != 0.
            let zero = self.const_val(Const::Int(0));
            self.emit(end_blk, Instruction::Ne(src_val, zero))
        } else if target_ty.is_integer_like() {
            // For same-width or narrower: Trunc.  For wider: Sext.
            // Without knowing the source width, default to Sext (safe).
            self.emit(end_blk, Instruction::Sext(src_val, target_ty))
        } else {
            self.emit(end_blk, Instruction::Bitcast(src_val, target_ty))
        };

        (result, end_blk)
    }
}

// ============================================================================
// Helpers
// ============================================================================

impl<'a> Lowerer<'a> {
    /// If `block`'s current terminator is a self-looping placeholder
    /// (`Br(block)`), replace it with `Br(target)`.
    fn patch_br_if_needed(&mut self, block: BlockId, target: BlockId) {
        let current_term = &self.func.blocks[block.0].terminator;
        let needs_patch = matches!(current_term, Terminator::Br(b) if *b == block);
        if needs_patch {
            self.set_terminator(block, Terminator::Br(target));
        }
    }

    /// Ensure the given block has an unconditional branch to `target`.  If the
    /// block already has a non-placeholder terminator this is a no-op.
    fn ensure_br(&mut self, block: BlockId, target: BlockId) {
        let current_term = &self.func.blocks[block.0].terminator;
        match current_term {
            Terminator::Br(b) if *b == block || *b == target => {
                // Placeholder or already correct — overwrite.
                self.set_terminator(block, Terminator::Br(target));
            }
            Terminator::Br(_) => {
                // Already branches somewhere else — leave alone.
            }
            _ => {
                // Has a real terminator (Ret, BrCond, Switch) — leave alone.
            }
        }
    }

    /// Guess a field index from a field name using a trivial hash.
    /// In a production compiler this would be replaced by proper type
    /// resolution during semantic analysis.
    fn guess_field_index(&self, _name: &str) -> usize {
        // Placeholder: LLVM's GEP just needs a consistent index — the backend
        // computes the actual byte offset from the struct layout.
        0
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // AST builder helpers
    // -----------------------------------------------------------------------

    /// Tiny builder for constructing flat-array ASTs in tests.
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

        /// Allocate a new node, return its index.
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

        /// Append a null-terminated name to the pool, return its offset.
        fn pool_name(&mut self, name: &str) -> i32 {
            let off = self.pool.len() as i32;
            self.pool.extend_from_slice(name.as_bytes());
            self.pool.push(0);
            off
        }

        /// Append length-prefixed bytes to the pool, return offset.
        fn pool_bytes(&mut self, data: &[u8]) -> i32 {
            let off = self.pool.len() as i32;
            let len = data.len() as u32;
            self.pool.extend_from_slice(&len.to_le_bytes());
            self.pool.extend_from_slice(data);
            off
        }

        /// Build a minimal PROGRAMU root wrapping one child and return the
        /// arrays plus `ast_idadi`.
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
    // Tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_empty_program() {
        // A program with no functions or globals — just the root.
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
        // kazi kuu() { } → function with no body
        let mut b = AstBuilder::new();
        let jina_kuu = b.pool_name("kuu");
        // Encoded type for W0 (Void): familia=5, upana=0, mshale=0 → (5<<8)|0 = 1280
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
        // Should have at least an entry block.
        assert!(f.block_count() >= 1);
    }

    #[test]
    fn test_function_with_params() {
        // kazi jumlisha(a: N32, b: N32): N32 { ... }
        let mut b = AstBuilder::new();
        let jina_jumlisha = b.pool_name("jumlisha");
        // Encoded type for N32: familia=1, upana=32, mshale=0 → (1<<8)|32 = 288
        let n32_enc: i32 = 2080; // (1<<11)|(4<<3)|0

        // Parameter nodes: each has jina_off for name, thamani for type encoding.
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
        // Chain params: a → b via kulia (matching parser convention).
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

        // Integer literal "3": AST_NAMBARI, thamani = 3
        let lit = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 3, 0);

        // Return statement: AST_RUDISHA, kushoto = lit
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
        // Should have blocks: entry, body, ret
        assert!(f.block_count() >= 3, "expected at least 3 blocks, got {}", f.block_count());
        // Verify a Ret terminator exists somewhere.
        let has_ret = f.blocks.iter().any(|blk| matches!(blk.terminator, Terminator::Ret(_)));
        assert!(has_ret, "function should contain a Ret terminator");
        // Verify the integer constant 3 is interned.
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

        // Identifier "x"
        let id_x_off = b.pool_name("x");

        // Literals
        let lit5 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 5, 0);
        let lit10 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 10, 0);

        // Type node for N32
        let type_n32 = b.node(0, NO_NODE, NO_NODE, NO_NODE, NO_NODE, n32_off, 0);

        // Name node for x
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

        // Right-hand identifier "x" for assignment
        let id_x_rhs = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, id_x_off);

        // ASIMILIA: x = 10
        let assign = b.node(
            AST_ASIMILIA,
            id_x_rhs,   // kushoto = lvalue
            lit10,      // kulia = rvalue
            NO_NODE, NO_NODE, 0, 0,
        );

        // Return: rudisha x
        let id_x_ret = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, id_x_off);
        let ret_stmt = b.node(AST_RUDISHA, id_x_ret, NO_NODE, NO_NODE, NO_NODE, 0, 0);

        // Chain: decl → assign → ret
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

        // Check that Alloca and Store instructions exist.
        let has_alloca = f.blocks.iter().any(|blk| {
            blk.instructions.iter().any(|inst| matches!(inst, Instruction::Alloca(_)))
        });
        assert!(has_alloca, "function should have Alloca instructions");

        let has_store = f.blocks.iter().any(|blk| {
            blk.instructions.iter().any(|inst| matches!(inst, Instruction::Store(_, _)))
        });
        assert!(has_store, "function should have Store instructions");
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

        // Param x
        let jina_x = b.pool_name("x");
        let param_x = b.node(0, NO_NODE, NO_NODE, NO_NODE, NO_NODE, n32_off, jina_x);

        // Literals
        let lit1 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 1, 0);
        let lit0 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, 0);

        // Identifier x
        let id_x_off = b.pool_name("x");
        let id_x = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, id_x_off);

        // then: rudisha 1
        let ret1 = b.node(AST_RUDISHA, lit1, NO_NODE, NO_NODE, NO_NODE, 0, 0);
        // else: rudisha 0
        let ret0 = b.node(AST_RUDISHA, lit0, NO_NODE, NO_NODE, NO_NODE, 0, 0);

        // kama (x) ... tivyo ...
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

        // Verify a BrCond terminator exists.
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

        // Should have BrCond for the while condition.
        let has_brcond = f.blocks.iter().any(|blk| {
            matches!(blk.terminator, Terminator::BrCond(_, _, _))
        });
        assert!(has_brcond, "while loop should produce a BrCond terminator");

        // Should have break block (Br to exit).
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

        // Should contain an Add instruction somewhere.
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

        // Callee identifier "chapisha"
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

        // AND lowers to Ne + And instructions.
        let has_and = f.blocks.iter().any(|blk| {
            blk.instructions.iter().any(|inst| matches!(inst, Instruction::And(_, _)))
        });
        assert!(has_and, "AND should produce And instruction");

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

        // OR lowers to Ne + Or instructions.
        let has_or = f.blocks.iter().any(|blk| {
            blk.instructions.iter().any(|inst| matches!(inst, Instruction::Or(_, _)))
        });
        assert!(has_or, "OR should produce Or instruction");
    }

    #[test]
    fn test_global_variable() {
        // Global: N32 KIKOMO = 0;
        let mut b = AstBuilder::new();
        let jina_kikomo = b.pool_name("KIKOMO");
        // Encoded type for N32: familia=1, upana=32, mshale=0 → (1<<8)|32 = 288
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

        // Should have at least one global (the user global; string globals may
        // also be present).
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

        // String literal "habari"
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

        // Should contain a StringAddr instruction.
        let has_string_addr = f.blocks.iter().any(|blk| {
            blk.instructions.iter().any(|inst| matches!(inst, Instruction::StringAddr(_)))
        });
        assert!(has_string_addr, "string literal should produce a StringAddr instruction");

        // Module should have a string global.
        let has_str_global = module.globals.iter().any(|g| g.is_const && g.bytes.starts_with(b"habari"));
        assert!(has_str_global, "module should contain a string global for 'habari'");
    }

    #[test]
    fn test_multiple_functions() {
        // Two functions: a() and b()
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
        // Chain a → b as siblings.
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
        // Nukta with 2 float fields → Direct return (not sret).
        let mut b = AstBuilder::new();
        let name_off = b.pool_name("pataNukta");
        // Named struct type — not a primitive, so from_swa_type returns None,
        // and we fall back to IrType::Struct with empty fields.
        // A struct with 0 fields → Direct.
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
        // Empty struct (0 fields) → Direct.
        assert_eq!(f.return_class, IrReturnClass::Direct);
        assert!(f.sret_value_id.is_none());
    }

    #[test]
    fn test_node_aina_no_node() {
        // Unit test for the node_aina helper with NO_NODE sentinel.
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
            global_types: std::collections::HashMap::new(),
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
            global_types: std::collections::HashMap::new(),
        };
        assert_eq!(lr.read_pool_name(0), "hello");
        assert_eq!(lr.read_pool_name(6), "world");
        assert_eq!(lr.read_pool_name(-1), "");
    }

    #[test]
    fn test_read_pool_bytes_length_prefixed() {
        // 4-byte LE length = 5, then 5 bytes "hello"
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
            global_types: std::collections::HashMap::new(),
        };
        let bytes = lr.read_pool_bytes(0);
        assert_eq!(bytes, b"hello");
    }

    #[test]
    fn test_read_pool_bytes_fallback_null_terminated() {
        // No length prefix (just null-terminated).
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
            global_types: std::collections::HashMap::new(),
        };
        // The pool has no length prefix, so the 4 bytes [104, 97, 98, 97] (= "haba")
        // would be interpreted as a length.  That length is huge, so it falls
        // back to null-terminated and reads from offset 0.
        let bytes = lr.read_pool_bytes(0);
        // Falls back to null-terminated: reads from offset 0 to null at index 6.
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
        // Encoded types
        let n32_enc: i32 = (1 << 11) | (4 << 3) | 0;   // N32
        let n64_enc: i32 = (1 << 11) | (5 << 3) | 0;   // N64
        let w0_enc: i32 = (5 << 11) | (0 << 3) | 0;     // W0 (not used here)

        // Names
        let jina_jaribio = b.pool_name("jaribio");
        let jina_n = b.pool_name("n");
        let jina_i = b.pool_name("i");
        let lit0 = b.pool_name("0");   // not a real lit, just for pool
        let lit1 = b.pool_name("1");

        // -- Param n: N64 --
        let p_n = b.node(0, NO_NODE, NO_NODE, NO_NODE, NO_NODE, n64_enc, jina_n);

        // -- Body: N64 i = 0; wakati ...; rudisha 0; --
        // Identifier i
        let id_i_off = jina_i;
        let name_i = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, id_i_off);

        // Literals
        let lit_0 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, 0);
        let lit_1 = b.node(AST_NAMBARI, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 1, 0);

        // Type node N64 for declaration
        let ty_n64 = b.node(0, NO_NODE, NO_NODE, NO_NODE, NO_NODE, n64_enc, 0);

        // Decl: N64 i = 0
        let decl = b.node(AST_TANGAZO, name_i, ty_n64, lit_0, NO_NODE, 0, 0);

        // -- wakati body --
        // Condition: i < n
        let id_i_cond = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, id_i_off);
        let id_n_cond = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, jina_n);
        let cond = b.node(AST_CHINI, id_i_cond, id_n_cond, NO_NODE, NO_NODE, 0, 0);

        // -- if body --
        // Condition: i == 0
        let id_i_eq = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, id_i_off);
        let if_cond = b.node(AST_SAWA, id_i_eq, lit_0, NO_NODE, NO_NODE, 0, 0);
        // then: rudisha 1
        let ret1 = b.node(AST_RUDISHA, lit_1, NO_NODE, NO_NODE, NO_NODE, 0, 0);
        // if stmt
        let if_stmt = b.node(AST_KAMA, if_cond, ret1, NO_NODE, NO_NODE, 0, 0);

        // i = i + 1 (ASIMILIA)
        let id_i_assign = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, id_i_off);
        let id_i_rhs = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, id_i_off);
        let add_expr = b.node(AST_JUMLISHA, id_i_rhs, lit_1, NO_NODE, NO_NODE, 0, 0);
        let assign = b.node(AST_ASIMILIA, id_i_assign, add_expr, NO_NODE, NO_NODE, 0, 0);

        // Chain if → assign inside while body
        b.nne[if_stmt as usize] = assign;

        // while (cond) { body }
        let while_node = b.node(AST_WAKATI, cond, NO_NODE, if_stmt, NO_NODE, 0, 0);

        // rudisha 0
        let ret0 = b.node(AST_RUDISHA, lit_0, NO_NODE, NO_NODE, NO_NODE, 0, 0);

        // Chain decl → while → ret0
        b.nne[decl as usize] = while_node;
        b.nne[while_node as usize] = ret0;

        // Function
        let name_f = b.node(AST_KITAMBULISHO, NO_NODE, NO_NODE, NO_NODE, NO_NODE, 0, jina_jaribio);
        let func = b.node(AST_KAZI, name_f, p_n, decl, NO_NODE, n32_enc, 0);

        let (aina, kushoto, kulia, tiga, nne, thamani, jina_off, pool, idadi) = b.finish(func);
        let module = lower(&aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, idadi);

        assert_eq!(module.functions.len(), 1);
        let f = &module.functions[0];
        assert_eq!(f.name, "jaribio");
        assert_eq!(f.return_ty, IrType::I32);

        // Verify there's no RetVoid in a non-void function.
        let retvoid_blocks: Vec<_> = f.blocks.iter()
            .filter(|blk| matches!(blk.terminator, Terminator::RetVoid))
            .map(|blk| blk.label.as_str())
            .collect();
        assert!(retvoid_blocks.is_empty(),
            "non-void function should not have RetVoid blocks, found: {:?}", retvoid_blocks);

        // Verify it has Ret terminators with values.
        let has_ret_val = f.blocks.iter().any(|blk| {
            matches!(blk.terminator, Terminator::Ret(_))
        });
        assert!(has_ret_val, "function should have at least one Ret(value)");
    }
}
