//! Swa ABI v1.0 — struct-return classification.
//!
//! ## Rules
//!
//! 1. Scalar types (integers, floats, pointers, words, bools) are passed
//!    directly in a register.
//! 2. Struct types are **flattened** into their leaf (non-struct) fields
//!    recursively.
//! 3. If the flattened field count is ≤ 2 the struct is returned **directly**
//!    (fields in up to two registers).
//! 4. If the flattened field count is > 2 the struct is returned via a
//!    **hidden pointer** (sret). The caller allocates space and passes a
//!    pointer as the first implicit parameter.
//!
//! ## Classification of each field
//!
//! Every leaf field is tagged as either [`AbiClass::Integer`] or
//! [`AbiClass::Float`] so that the backend can map them to the correct
//! register class (general-purpose vs. SIMD/floating-point).
//!
//! ## Examples
//!
//! | Type               | Flattened fields   | Count | Result       |
//! |--------------------|--------------------|-------|--------------|
//! | `N32`              | `[Integer]`        | 1     | Direct       |
//! | `D64`              | `[Float]`          | 1     | Direct       |
//! | `{D64, D64}`       | `[Float, Float]`   | 2     | Direct       |
//! | `{*N8, N64}`       | `[Integer,Integer]`| 2     | Direct       |
//! | `{N32, N32, N32}`  | `[Integer × 3]`    | 3     | HiddenPtr    |

use crate::ir::types::{AbiClass, IrType};
use crate::ir::IrReturnClass;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Classify the return type of a function according to the Swa ABI v1.0.
///
/// Returns [`IrReturnClass::Direct`] when the type fits in ≤ 2 registers
/// (after struct flattening), and [`IrReturnClass::HiddenPtr`] when the
/// caller must allocate space and pass an implicit sret pointer.
///
/// # Parameters
///
/// * `ty` — The function's return type from the IR.
///
/// # Panics
///
/// Panics if any leaf field cannot be classified (e.g. a `Void` leaf inside a
/// struct).  This indicates a malformed type and should be caught during
/// semantic analysis.
pub fn classify_return(ty: &IrType) -> IrReturnClass {
    let fields = flatten_struct(ty);
    if fields.len() <= 2 {
        IrReturnClass::Direct
    } else {
        IrReturnClass::HiddenPtr
    }
}

/// Flatten `ty` into a list of `AbiClass` values, one per leaf field.
///
/// Scalar types produce a single-element list.
/// Structs are recursively unwrapped — nested struct fields contribute their
/// own leaves in order.
/// `Void` contributes nothing.
/// Arrays are **not** flattened because the Swa ABI treats them as opaque;
/// they are classified as `Integer` (a pointer / aggregate reference).
pub fn flatten_struct(ty: &IrType) -> Vec<AbiClass> {
    match ty {
        IrType::Void => vec![],

        // Signed integers — classify as Integer.
        IrType::I8 | IrType::I16 | IrType::I32 | IrType::I64 | IrType::I128 => {
            vec![AbiClass::Integer]
        }

        // Unsigned integers.
        IrType::A8 | IrType::A16 | IrType::A32 | IrType::A64 | IrType::A128 => {
            vec![AbiClass::Integer]
        }

        // Booleans / opaque bits.
        IrType::B1 | IrType::B8 | IrType::B16 | IrType::B32 | IrType::B64 => {
            vec![AbiClass::Integer]
        }

        // Words.
        IrType::W8 | IrType::W16 | IrType::W32 | IrType::W64 => {
            vec![AbiClass::Integer]
        }

        // Floating-point — classify as Float.
        IrType::F16 | IrType::F32 | IrType::F64 | IrType::F128 => {
            vec![AbiClass::Float]
        }

        // Struct — recursively flatten each field.
        IrType::Struct { fields, .. } => {
            let mut out = Vec::new();
            for (_name, field_ty) in fields {
                out.extend(flatten_struct(field_ty));
            }
            out
        }

        // Opaque / aggregate types — treat as a single integer-class slot.
        IrType::Array { .. } => vec![AbiClass::Integer],
        IrType::FnPtr { .. } => vec![AbiClass::Integer],
        IrType::Ptr(_) => vec![AbiClass::Integer],
    }
}

/// Return the field-class list **and** the count, for diagnostic / debugging
/// purposes.
///
/// This is a convenience wrapper around [`flatten_struct`].
pub fn classify_fields(ty: &IrType) -> (Vec<AbiClass>, usize) {
    let fields = flatten_struct(ty);
    let count = fields.len();
    (fields, count)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- helpers ------------------------------------------------------------

    /// Shorthand for building a named struct type.
    fn struct_ty(name: &str, fields: Vec<(&str, IrType)>) -> IrType {
        IrType::Struct {
            name: name.into(),
            fields: fields.into_iter().map(|(n, t)| (n.into(), t)).collect(),
        }
    }

    /// Shorthand for a pointer-to-I8.
    fn ptr_ty() -> IrType {
        IrType::Ptr(Box::new(IrType::I8))
    }

    // -- classify_return ----------------------------------------------------

    #[test]
    fn test_void() {
        // Void has 0 fields → direct.
        assert_eq!(classify_return(&IrType::Void), IrReturnClass::Direct);
        assert!(flatten_struct(&IrType::Void).is_empty());
    }

    #[test]
    fn test_single_integer() {
        assert_eq!(classify_return(&IrType::I32), IrReturnClass::Direct);
        assert_eq!(flatten_struct(&IrType::I32), vec![AbiClass::Integer]);

        assert_eq!(classify_return(&IrType::A64), IrReturnClass::Direct);
        assert_eq!(classify_return(&IrType::B1), IrReturnClass::Direct);
        assert_eq!(classify_return(&IrType::W32), IrReturnClass::Direct);
        assert_eq!(classify_return(&ptr_ty()), IrReturnClass::Direct);
    }

    #[test]
    fn test_single_float() {
        assert_eq!(classify_return(&IrType::F64), IrReturnClass::Direct);
        assert_eq!(flatten_struct(&IrType::F64), vec![AbiClass::Float]);

        assert_eq!(classify_return(&IrType::F32), IrReturnClass::Direct);
    }

    #[test]
    fn test_nukta() {
        // Nukta = { f64, f64 } → 2 float fields → direct.
        let nukta = struct_ty("Nukta", vec![("x", IrType::F64), ("y", IrType::F64)]);
        assert_eq!(classify_return(&nukta), IrReturnClass::Direct);
        assert_eq!(
            flatten_struct(&nukta),
            vec![AbiClass::Float, AbiClass::Float]
        );
    }

    #[test]
    fn test_mstari() {
        // Mstari = { ptr, i64 } → 2 integer fields → direct.
        let mstari = struct_ty("Mstari", vec![("data", ptr_ty()), ("len", IrType::I64)]);
        assert_eq!(classify_return(&mstari), IrReturnClass::Direct);
        assert_eq!(
            flatten_struct(&mstari),
            vec![AbiClass::Integer, AbiClass::Integer]
        );
    }

    #[test]
    fn test_three_fields_hidden_ptr() {
        // { i32, i32, i32 } → 3 integer fields → hidden ptr.
        let triplet = struct_ty(
            "Triplet",
            vec![
                ("a", IrType::I32),
                ("b", IrType::I32),
                ("c", IrType::I32),
            ],
        );
        assert_eq!(classify_return(&triplet), IrReturnClass::HiddenPtr);
        assert_eq!(
            flatten_struct(&triplet),
            vec![AbiClass::Integer, AbiClass::Integer, AbiClass::Integer]
        );
    }

    #[test]
    fn test_nested_struct_flattening() {
        // Outer { Inner { f64, f64 }, i32 }
        // → flatten → [Float, Float, Integer] = 3 fields → hidden ptr.
        let inner = struct_ty("Ndani", vec![("a", IrType::F64), ("b", IrType::F64)]);
        let outer = struct_ty("Nje", vec![("inner", inner), ("tag", IrType::I32)]);
        assert_eq!(classify_return(&outer), IrReturnClass::HiddenPtr);
        assert_eq!(
            flatten_struct(&outer),
            vec![AbiClass::Float, AbiClass::Float, AbiClass::Integer]
        );
    }

    #[test]
    fn test_nested_struct_two_leaf() {
        // Outer { Inner { f64, f64 } }
        // → flatten → [Float, Float] = 2 fields → direct.
        let inner = struct_ty("Ndani", vec![("a", IrType::F64), ("b", IrType::F64)]);
        let outer = struct_ty("Nje", vec![("inner", inner)]);
        assert_eq!(classify_return(&outer), IrReturnClass::Direct);
        assert_eq!(
            flatten_struct(&outer),
            vec![AbiClass::Float, AbiClass::Float]
        );
    }

    #[test]
    fn test_deeply_nested() {
        // A { B { C { i32 } }, f64 } → [Integer, Float] = 2 → direct.
        let c = struct_ty("C", vec![("x", IrType::I32)]);
        let b = struct_ty("B", vec![("c", c)]);
        let a = struct_ty("A", vec![("b", b), ("y", IrType::F64)]);
        assert_eq!(classify_return(&a), IrReturnClass::Direct);
        assert_eq!(
            flatten_struct(&a),
            vec![AbiClass::Integer, AbiClass::Float]
        );
    }

    #[test]
    fn test_mixed_fields() {
        // { f64, i64, f64 } → 3 fields → hidden ptr.
        let mixed = struct_ty(
            "Mixed",
            vec![
                ("f1", IrType::F64),
                ("i", IrType::I64),
                ("f2", IrType::F64),
            ],
        );
        assert_eq!(classify_return(&mixed), IrReturnClass::HiddenPtr);
    }

    #[test]
    fn test_classify_fields_convenience() {
        let nukta = struct_ty("Nukta", vec![("x", IrType::F64), ("y", IrType::F64)]);
        let (classes, count) = classify_fields(&nukta);
        assert_eq!(count, 2);
        assert_eq!(classes, vec![AbiClass::Float, AbiClass::Float]);
    }

    #[test]
    fn test_fnptr_is_single_integer() {
        let fnty = IrType::FnPtr {
            params: vec![IrType::I32],
            ret: Box::new(IrType::Void),
        };
        assert_eq!(classify_return(&fnty), IrReturnClass::Direct);
        assert_eq!(flatten_struct(&fnty), vec![AbiClass::Integer]);
    }

    #[test]
    fn test_array_is_single_integer() {
        let arr = IrType::Array {
            element: Box::new(IrType::I32),
            count: 8,
        };
        assert_eq!(classify_return(&arr), IrReturnClass::Direct);
        assert_eq!(flatten_struct(&arr), vec![AbiClass::Integer]);
    }

    #[test]
    fn test_empty_struct_direct() {
        // A struct with no fields → 0 leaf fields → direct.
        let empty = struct_ty("Tupu", vec![]);
        assert_eq!(classify_return(&empty), IrReturnClass::Direct);
        assert!(flatten_struct(&empty).is_empty());
    }

    #[test]
    fn test_struct_with_nested_empty_struct() {
        // Outer { EmptyStruct, i32 } → [Integer] = 1 → direct.
        let empty = struct_ty("Tupu", vec![]);
        let outer = struct_ty("Nje", vec![("e", empty), ("x", IrType::I32)]);
        assert_eq!(classify_return(&outer), IrReturnClass::Direct);
        assert_eq!(flatten_struct(&outer), vec![AbiClass::Integer]);
    }
}
