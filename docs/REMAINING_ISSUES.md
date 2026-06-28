# Remaining Issues — Swahili Self-Hosting Compiler

This document tracks known bugs, limitations, and next steps for the Swahili self-hosting compiler project. Items are listed roughly in order of severity and dependency.

---

## 1. O1 (Less) Optimization Bug — `tokeni_soma_kitambulisho` Urefu Corruption

### Status: **Broken at O1, works at O0**

### Summary

At O1, LLVM's SelectionDAG miscompiles the subtraction in the following expression from `tokeni_soma_kitambulisho` (inside `msomaji.swa`):

```
t->urefu = m->nafasi - anza;
```

The subtraction of `anza` appears to be lost entirely, so `urefu` receives the raw value of `m->nafasi` (some large integer) instead of the correct length `m->nafasi - anza`. This causes subsequent writes into `ast_pool` to land far past the array bounds, crashing the parser.

### Evidence (gdb)

At the crash point:

| Register | Value | Meaning |
|----------|-------|---------|
| `rdx`    | 36797 | Crash address offset (expected small) |
| `off`    | 3     | Correct AST node offset within the element |
| `i`      | 36794 | Wrong — should be 0 for a single-token identifier |

The value 36794 = 36797 − 3, meaning the element index `i` was computed from the corrupted `urefu` rather than the correct value of 1.

### Root Cause (Likely)

SelectionDAG (used at O1 and above) is either:
- Failing to emit the `sub` instruction for the pointer-difference expression, or
- Reordering loads/stores such that `anza` has not yet been computed when the subtraction is evaluated.

The bug is specific to the LLVM IR generated for `tokeni_soma_kitambulisho`. At O0, FastISel handles this function correctly; at O1, SelectionDAG takes over and produces wrong code.

### Why O1 Matters

FastISel silently drops basic blocks past approximately 50 per function. The self-hosting lexer and parser are already split into many small helper functions to stay under this limit (see section 4). If any remaining function crosses the threshold, O1 is the only fallback — and O1 is currently broken.

### What Needs to Be Done

1. Isolate the generated `.ll` for `tokeni_soma_kitambulisho` and verify the IR is correct (the `sub` is present before optimization passes).
2. If the IR is correct, bisect which LLVM pass corrupts the value (likely an early SelectionDAG lowering pass).
3. If the IR is wrong, fix the codegen for pointer-difference expressions in the Swahili compiler's LLVM backend.
4. Consider whether this is a known LLVM bug with a specific version — if so, upgrading or patching LLVM may fix it.

---

## 2. Array Size Limitation — BSS > ~47KB Crashes at Startup

### Status: **Unresolved — possibly Windows-specific**

### Summary

When AST pool arrays are small (512 elements, ~32 KB `ast_pool`), the parser binary works correctly at O0. Increasing the arrays (2048 elements, ~128 KB pool) causes an immediate segfault **before `main()` executes** — even with identical source code and no logic changes.

### Observed Behavior

- 512-element arrays: works at O0.
- 2048-element arrays: segfault before `main()`.
- A previously working binary with larger arrays later stopped working, suggesting an environmental rather than a code-level issue.
- The crash is in CRT startup or PE loader initialization, not in user code.

### Hypotheses

- **Windows ASLR / PE loader**: Larger BSS sections may trigger different loader behavior or relocation handling.
- **CRT zero-initialization (`__security_init_cookie` or `memset` of BSS)**: The CRT may walk BSS differently for larger sections, hitting a page boundary or guard page.
- **Stack probe / guard page**: Windows may touch BSS pages during startup and fault on a guard page adjacent to BSS.
- **Linker script or PE section layout**: The linker may place BSS in an unexpected location when it exceeds a certain size.

### What Needs to Be Done

1. **Test on Arch Linux** (or any Linux) with the same binary. If it works on Linux, the issue is Windows-specific (loader, CRT, or PE format) and not a compiler bug.
2. If it fails on Linux too, the issue is in the generated code or linker output — inspect the ELF sections.
3. If it is Windows-specific, investigate the PE header and `.bss` section placement, or consider using `calloc`/`malloc` for large arrays instead of static/global BSS allocation.

---

## 3. Self-Hosting Parser Edge Cases

### Status: **Partially working**

### What Works

```
N32 f() { rudisha 1; }
```

At O0, the parser successfully parses this input and returns `mzizi=3` (indicating a valid AST root with three nodes).

### What Doesn't Work

After parsing the above input, the parser prints:

```
unexpected token on line 1
```

This is a leftover-token issue: the parser does not consume the closing brace `}` (or some other final token), so the driver loop finds a stray token after the parse is logically complete.

### What Has Not Been Tested

- Multi-file `.swa` source (the compiler's own source files) due to the array size limit.
- Functions with parameters, struct field access, control flow (`ikiwa`, `wakati`), or nested scopes.
- Error recovery: the parser may crash or infinite-loop on malformed input.

### What Needs to Be Done

1. Fix the closing-brace consumption (or whichever token is left over).
2. After the array size issue is resolved, run the parser on real `.swa` source files and fix any parse failures.
3. Add basic error recovery so the parser can survive syntax errors without crashing.

---

## 4. Function Splitting for O0 — FastISel Block Limit

### Status: **Workaround in place, fragility remains**

### Background

At O0, LLVM's FastISel silently drops basic blocks past approximately 50 per function. This is not a configurable limit — it is a hardcoded fallback where FastISel gives up and produces no code for those blocks, resulting in wrong behavior with no warning.

### Current Mitigations

- **Lexer** (`msomaji.swa`): Long functions were manually split into helpers:
  - `ruka_nafasi_na_maelezo` — skip whitespace and comments
  - `tokeni_soma_kitambulisho` — read identifier token
  - And others as needed
- **Parser** (`mchambuzi.swa`): Split automatically via `_finish.py` into multiple functions to stay under the block limit.

### Remaining Risk

If any function — after future edits or new features — exceeds ~50 basic blocks, FastISel will silently produce wrong code at O0. There is no compile-time or run-time check for this condition.

### What Needs to Be Done

1. Add a post-codegen assertion or check that verifies no blocks were dropped by FastISel (LLVM may provide a mechanism to detect this).
2. Alternatively, move to O1 for all functions once the O1 bug (section 1) is fixed, eliminating the FastISel limit entirely.
3. If staying on O0, document the block-count constraint clearly in the contributor guide.

---

## 5. Next Steps (Priority Order)

| Priority | Task | Rationale |
|----------|------|-----------|
| **P0** | Test on Arch Linux with larger arrays | Determines whether issue 2 is a compiler bug or a Windows environmental problem. Unblocks larger-array testing of the parser. |
| **P1** | Fix the O1 `urefu` corruption bug | O1 is needed as a fallback for functions that exceed FastISel's block limit. Currently unusable. |
| **P2** | Test self-hosting with real `.swa` files | Validates the parser on the compiler's own source code. Requires larger arrays (depends on P0/P1). |
| **P3** | Resolve the leftover-token parser edge case | Minor correctness issue; does not block parsing but produces spurious diagnostics. |
| **P4** | Add FastISel block-drop detection | Prevents silent miscompilation when functions grow too large for O0. |

---
