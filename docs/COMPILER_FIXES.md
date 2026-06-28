# Compiler Fixes: Self-Hosting Bootstrap

During the effort to make the kande Rust compiler self-hosting (compile itself), six correctness bugs were identified and fixed. This document describes each bug precisely — where it lived, what went wrong, how it manifested, and how it was fixed.

---

## 1. Forward Declarations Created Empty Stubs

**File:** `src/ir/lower.rs`

**Bug.** When the IR lowerer encountered a *forward declaration* — a function signature with no body, e.g.

```c
N32 foo(Msambazaji* p);
```

it unconditionally emitted an LLVM function definition. For functions that lacked a body in the current translation unit, this produced an *empty function body* (a definition with no basic blocks). If a real definition of the same function appeared later in the TU, the empty stub *shadowed* it — the LLVM linker would see two definitions for the same symbol and choose the first (empty) one. The real implementation was effectively dead code.

**Fix.** A pre-scan of the AST was added to collect the set of function names that *do* have bodies. In `lower_function`, a check was inserted: if a function has no body **and** a function by the same name with a body exists elsewhere, the empty stub is skipped and only the real definition is emitted.

---

## 2. Store Width Mismatch — `i64` to `i32`

**File:** `src/codegen/llvm/mod.rs`

**Bug.** The `Const::Int` variant always materialized as an `i64` (8 bytes) in LLVM IR, regardless of the target type. When code assigned a `Const::Int` to an `i32`-typed stack allocation, the store instruction wrote 8 bytes into 4 bytes of alloca space. The extra bytes overflowed into adjacent stack variables, silently corrupting their values.

**Fix.** The store handler was changed to query the pointee type of the destination alloca via `LLVMGetElementType`, then insert an `LLVMBuildIntCast2` to truncate (or extend) the integer to match the destination width before emitting the store instruction.

---

## 3. Store Width Mismatch — `i32` to `i64` (Struct Fields)

**File:** `src/codegen/llvm/mod.rs`

**Bug.** When assigning an `N32` literal (i32, 4 bytes) to an `N64` struct field (i64, 8 bytes), the store handler wrote only 4 bytes. The upper 4 bytes of the destination field retained whatever garbage was already in memory. The original condition guarded the cast only for cases where the source width was *strictly greater than* the destination width (`>`), so widening extension never happened.

**Fix.** The width-comparison condition was changed from `>` to `!=`, so both truncation (4-byte source into 2-byte field) and extension (4-byte source into 8-byte field) are handled. The `StoreTyped` helper was also fixed to match.

---

## 4. FieldAddr Ignored Alignment

**File:** `src/codegen/llvm/mod.rs`

**Bug.** The `FieldAddr` handler computed byte offsets into LLVM aggregate types by summing the *raw element sizes* of preceding fields without applying alignment padding. LLVM struct layout requires each field to be aligned to its natural alignment; the compiler would place a field at a misaligned offset.

For example, given a struct `{i32, ptr, i64}`:
- `i32` at offset 0 (size 4)
- `ptr` at offset 4 (4 is aligned for an 8-byte pointer on 64-bit? No — it needs offset 8)
- `i64` at offset `4 + 8 = 12` (should be 16)

The compiler computed `4 + 8 = 12` for the `i64` field offset, but LLVM's own layout placed it at offset 16. GEPs computed with the wrong offset accessed the wrong bytes.

**Fix.** The `FieldAddr` handler now applies alignment to each field width before summing: each field's offset is rounded up to the next multiple of the field's alignment, then the field's size is added. This produces offsets that match LLVM's `getelementptr` expectations.

---

## 5. Struct `width_bytes` Missing Trailing Padding

**File:** `src/ir/types.rs`

**Bug.** The `width_bytes()` method on struct types computed the total size by summing the raw widths of each field, with no padding between or after fields. For example, the struct `Tokeni { i32, i8*, i64, i32, i32 }` has fields:

| field | size | natural alignment |
|-------|------|-------------------|
| i32   | 4    | 4                 |
| i8*   | 8    | 8                 |
| i64   | 8    | 8                 |
| i32   | 4    | 4                 |
| i32   | 4    | 4                 |

With alignment (max alignment = 8): `4 + 4(pad) + 8 + 8 + 4 + 4 + 4(pad) = 36…` actually:
- i32 at 0..4
- pad 4..8
- i8* at 8..16
- i64 at 16..24
- i32 at 24..28
- i32 at 28..32
- trailing pad to multiple of 8: 32

So the struct is 32 bytes. `width_bytes()` without padding returned 28 (4+8+8+4+4).

This caused `sret` (struct return) allocas to be undersized. When a function returned a struct by hidden pointer, the caller allocated space based on `width_bytes()` — too little — and the callee wrote past the allocation.

**Fix.** `width_bytes()` was rewritten to compute size with proper alignment: each field's offset is aligned to the field's natural alignment before placing it, and the total size is padded to a multiple of the struct's maximum field alignment. This matches LLVM's `DataLayout` sizing.

---

## 6. Global Array Types Declared as `[N×i8]` Instead of `[N×i32]`

**Files:** `src/ir/mod.rs`, `src/ir/lower.rs`, `src/codegen/llvm/mod.rs`

**Bug.** The `IrGlobal` struct had no type field — it carried only a byte length. When the LLVM backend needed to declare a global array, it guessed the element type from the byte length: if the array was more than 8 bytes, it always became `[N×i8]`, because the only information available was the total byte count and the backend assumed everything byte-addressable was byte-typed.

Consider `N32 ast_aina[2048]` — an array of 2048 four-byte integers (8192 bytes total). The backend declared it as `[2048×i8]` (only 2048 bytes). Every indexed write via `GEP i32` accessed memory at `base + index * 4`, which quickly overflowed the 2048-byte allocation and corrupted adjacent global variables.

**Fix.** An `ty: IrType` field was added to `IrGlobal`. The LLVM backend now uses `ir_type_to_llvm()` to emit the correct LLVM array type (e.g., `[2048×i32]`) for complex element types, reserving `[N×i8]` only for genuinely byte-typed data.

---

## Driver Change: Stack Size and Symbol Aliasing

**File:** `src/driver/main.rs`

Three linker-related changes were made to the compiler driver:

1. **Windows 8 MB stack reserve.** On Windows, the linker flag `-Wl,--stack,8388608` is passed. The self-hosting compiler's recursive-descent parser and deeply nested IR passes need more than the default 1–2 MB stack, particularly in debug builds.

2. **Linux unchanged.** Linux does not need special stack-size flags; the default stack limit (typically 8 MB from `ulimit -s`) is sufficient, and the `--stack` flag is not portable to GNU ld or LLD on ELF.

3. **`andika` symbol alias.** The flag `-Wl,--defsym,andika=printf` maps Swa's `andika` function to libc `printf`. This avoids requiring the Swa runtime library to be linked, since `andika` is used as the built-in print during bootstrapping but libc `printf` provides equivalent functionality.

---

## Summary Table

| # | Bug | Files | Symptom | Root Cause |
|---|-----|-------|---------|------------|
| 1 | Forward decls emit empty stubs | `src/ir/lower.rs` | Real implementations shadowed | No check for body existence |
| 2 | i64 store to i32 alloca | `src/codegen/llvm/mod.rs` | Adjacent stack variables corrupted | No width coercion on store |
| 3 | i32 store to i64 struct field | `src/codegen/llvm/mod.rs` | Garbage in upper 4 bytes | Width check used `>` not `!=` |
| 4 | FieldAddr ignores alignment | `src/codegen/llvm/mod.rs` | Wrong struct field accessed | Offsets summed without alignment |
| 5 | Struct width_bytes missing padding | `src/ir/types.rs` | sret allocas too small | No trailing alignment padding |
| 6 | Global arrays typed as [N×i8] | `src/ir/mod.rs`, `lower.rs`, `llvm/mod.rs` | Adjacent globals corrupted | IrGlobal lacked a type field |
