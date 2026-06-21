//! Swa intermediate representation type system.
//!
//! Defines the `IrType` enum covering all primitive and compound types
//! in the Swa language, along with ABI classification and sizing helpers.
//!
//! ## Swa type-name mapping
//!
//! | Swa prefix | Meaning          | Rust prefix |
//! |------------|------------------|-------------|
//! | `N`        | Namba (signed)   | `I`         |
//! | `A`        | Asili (unsigned)  | `U`         |
//! | `D`        | Desimali (float) | `F`         |
//! | `B`        | Buli (boolean)   | `B`         |
//! | `W`        | Wosia (word)     | `W`         |
//!
//! Width suffixes are in bits: `N32` == `i32`, `D64` == `f64`, etc.

use std::fmt;

// ---------------------------------------------------------------------------
// IrType
// ---------------------------------------------------------------------------

/// Every representable type in the Swa IR.
///
/// Variants are named with LLVM-friendly prefixes so that codegen can
/// translate them mechanically (`I` → signed integer, `U` → unsigned,
/// `F` → floating-point, `B` → boolean / opaque-bit, `W` → word).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrType {
    /// The empty type (unit / `void`).
    Void,

    // -- signed integers (Namba) -------------------------------------------
    I8,
    I16,
    I32,
    I64,
    I128,

    // -- unsigned integers -------------------------------------------------
    A8,
    A16,
    A32,
    A64,
    A128,

    // -- floating-point (Desimali) -----------------------------------------
    F16,
    F32,
    F64,
    F128,

    // -- booleans & opaque bits (Buli) -------------------------------------
    B1,
    B8,
    B16,
    B32,
    B64,

    // -- word-sized opaque types (Wosia) -----------------------------------
    W8,
    W16,
    W32,
    W64,

    // -- compound types ----------------------------------------------------
    /// An opaque pointer to `element`.
    Ptr(Box<IrType>),

    /// A function pointer: `fn (params) -> ret`.
    FnPtr {
        params: Vec<IrType>,
        ret: Box<IrType>,
    },

    /// A named struct (product type).  Fields are ordered.
    Struct {
        name: String,
        fields: Vec<(String, IrType)>,
    },

    /// A fixed-size array of `element` repeated `count` times.
    Array {
        element: Box<IrType>,
        count: u64,
    },
}

// ---------------------------------------------------------------------------
// ABI classification helper
// ---------------------------------------------------------------------------

/// Narrow ABI class used during struct-return classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbiClass {
    Integer,
    Float,
}

// ---------------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------------

impl IrType {
    /// Map a Swa source-level type name to its canonical IR type.
    ///
    /// Returns `None` for unrecognised or compound names (structs, arrays,
    /// pointers, function pointers — those are constructed node-by-node during
    /// semantic analysis rather than parsed from a single keyword).
    ///
    /// # Examples
    ///
    /// ```
    /// use kande_lib::ir::types::IrType;
    ///
    /// assert_eq!(IrType::from_swa_type("N32"), Some(IrType::I32));
    /// assert_eq!(IrType::from_swa_type("D64"), Some(IrType::F64));
    /// assert_eq!(IrType::from_swa_type("W0"),  Some(IrType::Void));
    /// assert_eq!(IrType::from_swa_type("Foobar"), None);
    /// ```
    pub fn from_swa_type(name: &str) -> Option<IrType> {
        match name {
            "W0" => Some(IrType::Void),

            // Namba (signed)
            "N8" => Some(IrType::I8),
            "N16" => Some(IrType::I16),
            "N32" => Some(IrType::I32),
            "N64" => Some(IrType::I64),
            "N128" => Some(IrType::I128),

            // Unsigned (A = Asili — natural/non-negative integers)
            "A8" => Some(IrType::A8),
            "A16" => Some(IrType::A16),
            "A32" => Some(IrType::A32),
            "A64" => Some(IrType::A64),
            "A128" => Some(IrType::A128),

            // Desimali (float)
            "D16" => Some(IrType::F16),
            "D32" => Some(IrType::F32),
            "D64" => Some(IrType::F64),
            "D128" => Some(IrType::F128),

            // Buli (boolean / opaque bit)
            "B1" => Some(IrType::B1),
            "B8" => Some(IrType::B8),
            "B16" => Some(IrType::B16),
            "B32" => Some(IrType::B32),
            "B64" => Some(IrType::B64),

            // Wosia (word)
            "W8" => Some(IrType::W8),
            "W16" => Some(IrType::W16),
            "W32" => Some(IrType::W32),
            "W64" => Some(IrType::W64),

            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

impl IrType {
    /// Whether this is an integer-like type (signed, unsigned, boolean, or
    /// pointer).
    pub fn is_integer_like(&self) -> bool {
        matches!(
            self,
            IrType::I8
                | IrType::I16
                | IrType::I32
                | IrType::I64
                | IrType::I128
                | IrType::A8
                | IrType::A16
                | IrType::A32
                | IrType::A64
                | IrType::A128
                | IrType::B1
                | IrType::B8
                | IrType::B16
                | IrType::B32
                | IrType::B64
                | IrType::W8
                | IrType::W16
                | IrType::W32
                | IrType::W64
                | IrType::Ptr(_)
        )
    }

    /// Whether this is a floating-point type.
    pub fn is_float(&self) -> bool {
        matches!(self, IrType::F16 | IrType::F32 | IrType::F64 | IrType::F128)
    }

    /// Classify this type for struct-return ABI purposes.
    ///
    /// Returns `None` for `Void`, arrays, and compound types that are not
    /// themselves scalar leaf fields (struct fields are recursively flattened
    /// by the ABI classifier in `abi::classify`).
    pub fn abi_class(&self) -> Option<AbiClass> {
        if self.is_float() {
            Some(AbiClass::Float)
        } else if self.is_integer_like() {
            Some(AbiClass::Integer)
        } else {
            None
        }
    }

    /// The *storage* width of this type in bytes.
    ///
    /// For pointers the width is target-dependent; this implementation
    /// assumes a 64-bit target (8 bytes).  Arrays multiply element width by
    /// count.  Struct width is the sum of field widths (packed, no padding
    /// for now — the real layout is delegated to LLVM).
    pub fn width_bytes(&self) -> usize {
        match self {
            IrType::Void => 0,

            IrType::I8 | IrType::A8 | IrType::B8 | IrType::W8 => 1,
            IrType::I16 | IrType::A16 | IrType::B16 | IrType::W16 | IrType::F16 => 2,
            IrType::I32 | IrType::A32 | IrType::B32 | IrType::W32 | IrType::F32 => 4,
            IrType::I64 | IrType::A64 | IrType::B64 | IrType::W64 | IrType::F64 => 8,
            IrType::I128 | IrType::A128 | IrType::F128 => 16,

            IrType::B1 => 1, // stored as a byte

            // Assume 64-bit target
            IrType::Ptr(_) | IrType::FnPtr { .. } => 8,

            IrType::Struct { fields, .. } => {
                fields.iter().map(|(_, ty)| ty.width_bytes()).sum()
            }

            IrType::Array { element, count } => {
                element.width_bytes() * (*count as usize)
            }
        }
    }

    /// The alignment of this type in bytes.
    ///
    /// Simple rule: alignment == natural width for primitives, 8 for
    /// pointers on 64-bit, and 8 for structs/arrays (conservative —
    /// LLVM computes the exact ABI alignment later).
    pub fn alignment_bytes(&self) -> usize {
        match self {
            IrType::Void => 1,

            IrType::I8 | IrType::A8 | IrType::B1 | IrType::B8 | IrType::W8 => 1,
            IrType::I16 | IrType::A16 | IrType::B16 | IrType::W16 | IrType::F16 => 2,
            IrType::I32 | IrType::A32 | IrType::B32 | IrType::W32 | IrType::F32 => 4,
            IrType::I64 | IrType::A64 | IrType::B64 | IrType::W64 | IrType::F64 => 8,
            IrType::I128 | IrType::A128 | IrType::F128 => 16,

            IrType::Ptr(_) | IrType::FnPtr { .. } => 8,

            // Conservative: align structs/arrays to pointer width.
            // LLVM will tighten this during codegen.
            IrType::Struct { .. } | IrType::Array { .. } => 8,
        }
    }

    /// Human-readable Swa-like label used by `Display`.
    fn type_label(&self) -> String {
        match self {
            IrType::Void => "W0".to_string(),
            IrType::I8 => "N8".to_string(),
            IrType::I16 => "N16".to_string(),
            IrType::I32 => "N32".to_string(),
            IrType::I64 => "N64".to_string(),
            IrType::I128 => "N128".to_string(),
            IrType::A8 => "A8".to_string(),
            IrType::A16 => "A16".to_string(),
            IrType::A32 => "A32".to_string(),
            IrType::A64 => "A64".to_string(),
            IrType::A128 => "A128".to_string(),
            IrType::F16 => "D16".to_string(),
            IrType::F32 => "D32".to_string(),
            IrType::F64 => "D64".to_string(),
            IrType::F128 => "D128".to_string(),
            IrType::B1 => "B1".to_string(),
            IrType::B8 => "B8".to_string(),
            IrType::B16 => "B16".to_string(),
            IrType::B32 => "B32".to_string(),
            IrType::B64 => "B64".to_string(),
            IrType::W8 => "W8".to_string(),
            IrType::W16 => "W16".to_string(),
            IrType::W32 => "W32".to_string(),
            IrType::W64 => "W64".to_string(),
            IrType::Ptr(inner) => format!("*{}", inner),
            IrType::FnPtr { params, ret } => {
                let param_strs: Vec<String> = params.iter().map(|p| p.to_string()).collect();
                format!("kazi({}) -> {}", param_strs.join(", "), ret)
            }
            IrType::Struct { name, fields } => {
                let field_strs: Vec<String> = fields
                    .iter()
                    .map(|(n, t)| format!("{}: {}", n, t))
                    .collect();
                format!("{}{{{}}}", name, field_strs.join(", "))
            }
            IrType::Array { element, count } => {
                format!("[{}; {}]", element, count)
            }
        }
    }
}

impl fmt::Display for IrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.type_label())
    }
}

impl fmt::Display for AbiClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AbiClass::Integer => write!(f, "Integer"),
            AbiClass::Float => write!(f, "Float"),
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- from_swa_type ------------------------------------------------------

    #[test]
    fn test_from_swa_type_primitives() {
        assert_eq!(IrType::from_swa_type("W0"), Some(IrType::Void));
        assert_eq!(IrType::from_swa_type("N32"), Some(IrType::I32));
        assert_eq!(IrType::from_swa_type("N64"), Some(IrType::I64));
        assert_eq!(IrType::from_swa_type("A8"), Some(IrType::A8));
        assert_eq!(IrType::from_swa_type("A64"), Some(IrType::A64));
        assert_eq!(IrType::from_swa_type("D32"), Some(IrType::F32));
        assert_eq!(IrType::from_swa_type("D64"), Some(IrType::F64));
        assert_eq!(IrType::from_swa_type("B1"), Some(IrType::B1));
        assert_eq!(IrType::from_swa_type("B32"), Some(IrType::B32));
        assert_eq!(IrType::from_swa_type("W32"), Some(IrType::W32));
        assert_eq!(IrType::from_swa_type("W64"), Some(IrType::W64));
    }

    #[test]
    fn test_from_swa_type_all_widths() {
        // Namba (signed) — all widths
        for (name, expected) in [
            ("N8", IrType::I8),
            ("N16", IrType::I16),
            ("N32", IrType::I32),
            ("N64", IrType::I64),
            ("N128", IrType::I128),
        ] {
            assert_eq!(IrType::from_swa_type(name), Some(expected), "failed for {name}");
        }
        // Unsigned — all widths
        for (name, expected) in [
            ("A8", IrType::A8),
            ("A16", IrType::A16),
            ("A32", IrType::A32),
            ("A64", IrType::A64),
            ("A128", IrType::A128),
        ] {
            assert_eq!(IrType::from_swa_type(name), Some(expected), "failed for {name}");
        }
        // Float — all widths
        for (name, expected) in [
            ("D16", IrType::F16),
            ("D32", IrType::F32),
            ("D64", IrType::F64),
            ("D128", IrType::F128),
        ] {
            assert_eq!(IrType::from_swa_type(name), Some(expected), "failed for {name}");
        }
        // Buli — all widths
        for (name, expected) in [
            ("B1", IrType::B1),
            ("B8", IrType::B8),
            ("B16", IrType::B16),
            ("B32", IrType::B32),
            ("B64", IrType::B64),
        ] {
            assert_eq!(IrType::from_swa_type(name), Some(expected), "failed for {name}");
        }
        // Wosia — all widths
        for (name, expected) in [
            ("W8", IrType::W8),
            ("W16", IrType::W16),
            ("W32", IrType::W32),
            ("W64", IrType::W64),
        ] {
            assert_eq!(IrType::from_swa_type(name), Some(expected), "failed for {name}");
        }
    }

    #[test]
    fn test_from_swa_type_unknown() {
        assert_eq!(IrType::from_swa_type("foobar"), None);
        assert_eq!(IrType::from_swa_type("X64"), None);
        assert_eq!(IrType::from_swa_type(""), None);
    }

    // -- abi_class ----------------------------------------------------------

    #[test]
    fn test_abi_class_integer() {
        assert_eq!(IrType::I32.abi_class(), Some(AbiClass::Integer));
        assert_eq!(IrType::A64.abi_class(), Some(AbiClass::Integer));
        assert_eq!(IrType::B8.abi_class(), Some(AbiClass::Integer));
        assert_eq!(IrType::W32.abi_class(), Some(AbiClass::Integer));
        assert_eq!(
            IrType::Ptr(Box::new(IrType::I8)).abi_class(),
            Some(AbiClass::Integer)
        );
    }

    #[test]
    fn test_abi_class_float() {
        assert_eq!(IrType::F32.abi_class(), Some(AbiClass::Float));
        assert_eq!(IrType::F64.abi_class(), Some(AbiClass::Float));
    }

    #[test]
    fn test_abi_class_none() {
        assert_eq!(IrType::Void.abi_class(), None);
        // Arrays don't classify directly — the ABI classifier flattens first
        assert_eq!(
            IrType::Array {
                element: Box::new(IrType::I32),
                count: 4
            }
            .abi_class(),
            None
        );
    }

    // -- width_bytes --------------------------------------------------------

    #[test]
    fn test_width_bytes_primitives() {
        assert_eq!(IrType::Void.width_bytes(), 0);
        assert_eq!(IrType::I8.width_bytes(), 1);
        assert_eq!(IrType::I16.width_bytes(), 2);
        assert_eq!(IrType::I32.width_bytes(), 4);
        assert_eq!(IrType::I64.width_bytes(), 8);
        assert_eq!(IrType::I128.width_bytes(), 16);
        assert_eq!(IrType::F32.width_bytes(), 4);
        assert_eq!(IrType::F64.width_bytes(), 8);
        assert_eq!(IrType::B1.width_bytes(), 1);
    }

    #[test]
    fn test_width_bytes_ptr() {
        assert_eq!(IrType::Ptr(Box::new(IrType::I32)).width_bytes(), 8); // 64-bit
    }

    #[test]
    fn test_width_bytes_struct() {
        let s = IrType::Struct {
            name: "Foo".into(),
            fields: vec![
                ("a".into(), IrType::I32),
                ("b".into(), IrType::F64),
            ],
        };
        assert_eq!(s.width_bytes(), 12); // 4 + 8
    }

    #[test]
    fn test_width_bytes_array() {
        let a = IrType::Array {
            element: Box::new(IrType::I32),
            count: 4,
        };
        assert_eq!(a.width_bytes(), 16); // 4 * 4
    }

    // -- alignment_bytes ----------------------------------------------------

    #[test]
    fn test_alignment_bytes_primitives() {
        assert_eq!(IrType::I8.alignment_bytes(), 1);
        assert_eq!(IrType::I16.alignment_bytes(), 2);
        assert_eq!(IrType::I32.alignment_bytes(), 4);
        assert_eq!(IrType::I64.alignment_bytes(), 8);
        assert_eq!(IrType::F64.alignment_bytes(), 8);
    }

    // -- Display ------------------------------------------------------------

    #[test]
    fn test_display_primitives() {
        assert_eq!(IrType::Void.to_string(), "W0");
        assert_eq!(IrType::I32.to_string(), "N32");
        assert_eq!(IrType::A64.to_string(), "A64");
        assert_eq!(IrType::F64.to_string(), "D64");
        assert_eq!(IrType::B1.to_string(), "B1");
        assert_eq!(IrType::W32.to_string(), "W32");
    }

    #[test]
    fn test_display_ptr() {
        let p = IrType::Ptr(Box::new(IrType::I8));
        assert_eq!(p.to_string(), "*N8");
    }

    #[test]
    fn test_display_fnptr() {
        let fp = IrType::FnPtr {
            params: vec![IrType::I32, IrType::F64],
            ret: Box::new(IrType::Void),
        };
        assert_eq!(fp.to_string(), "kazi(N32, D64) -> W0");
    }

    #[test]
    fn test_display_struct() {
        let s = IrType::Struct {
            name: "Nukta".into(),
            fields: vec![("x".into(), IrType::F64), ("y".into(), IrType::F64)],
        };
        assert_eq!(s.to_string(), "Nukta{x: D64, y: D64}");
    }

    #[test]
    fn test_display_array() {
        let a = IrType::Array {
            element: Box::new(IrType::I32),
            count: 3,
        };
        assert_eq!(a.to_string(), "[N32; 3]");
    }
}
