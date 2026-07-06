//! Mfumo wa aina za uwakilishi wa kati (IR) wa Swa.
//!
//! Hufafanua enum ya `IrType` inayojumuisha aina zote za awali na mchanganyiko
//! katika lugha ya Swa, pamoja na uainishaji wa ABI na visaidizi vya ukubwa.
//!
//! ## Ramani ya majina ya aina za Swa
//!
//! | Kiambishi Swa | Maana            | Kiambishi Rust |
//! |---------------|------------------|----------------|
//! | `N`           | Namba (sahihi)   | `I`            |
//! | `A`           | Asili (sahihi)   | `U`            |
//! | `D`           | Desimali (float) | `F`            |
//! | `B`           | Buli (boolean)   | `B`            |
//! | `W`           | Wazi (word)      | `W`            |
//!
//! Viambishi vya upana viko kwenye biti: `N32` == `i32`, `D64` == `f64`, n.k.

use std::fmt;

// ---------------------------------------------------------------------------
// IrType
// ---------------------------------------------------------------------------

/// Kila aina inayowakilishwa katika IR ya Swa.
///
/// Lahaja zinaitwa kwa viambishi vinavyofaa LLVM ili kodejeni iweze
/// kuzitafsiri kimitambo (`I` → namba sahihi, `U` → namba sahihi isiyo na alama,
/// `F` → namba sehemu-desimali, `B` → buli/bati lisilo wazi, `W` → neno).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrType {
    /// Aina tupu (`void`).
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

    // -- word-sized opaque types (Wazi) -----------------------------------
    W8,
    W16,
    W32,
    W64,

    // -- compound types ----------------------------------------------------
    /// Kielekezi kisicho wazi kwa `element`.
    Ptr(Box<IrType>),

    /// Kielekezi cha kazi: `fn (vigezo) -> rudisha`.
    FnPtr {
        params: Vec<IrType>,
        ret: Box<IrType>,
    },

    /// Muundo ulio na jina (aina ya bidhaa). Sehemu zimepangwa.
    Struct {
        name: String,
        fields: Vec<(String, IrType)>,
    },

    /// Safu ya ukubwa uliojulikana ya `element` ikirudiwa mara `count`.
    Array {
        element: Box<IrType>,
        count: u64,
    },
}

// ---------------------------------------------------------------------------
// Kisaidizi cha uainishaji ABI
// ---------------------------------------------------------------------------

/// Darasa finyu la ABI linalotumika wakati wa uainishaji wa kurudisha muundo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbiClass {
    Integer,
    Float,
}

// ---------------------------------------------------------------------------
// Ujenzi
// ---------------------------------------------------------------------------

impl IrType {
    /// Ramani jina la aina ya Swa ngazi ya chanzo hadi aina ya IR kanoni.
    ///
    /// Hurejesha `None` kwa majina yasiyotambulika au mchanganyiko (miundo,
    /// safu, vielekezi, vielekezi vya kazi — hizo hujengwa nodi kwa nodi
    /// wakati wa uchanganuzi wa kisemantiki badala ya kuchanganuliwa kutoka
    /// neno muhimu moja).
    ///
    /// # Mifano
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

            // Wazi (word)
            "W8" => Some(IrType::W8),
            "W16" => Some(IrType::W16),
            "W32" => Some(IrType::W32),
            "W64" => Some(IrType::W64),

            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Maswali
// ---------------------------------------------------------------------------

impl IrType {
    /// Je, hii ni aina kama namba sahihi (sahihi, sahihi isiyo na alama, buli, au
    /// kielekezi)?
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

    /// Je, hii ni aina ya namba sehemu-desimali?
    pub fn is_float(&self) -> bool {
        matches!(self, IrType::F16 | IrType::F32 | IrType::F64 | IrType::F128)
    }

    /// Ainisha aina hii kwa madhumuni ya ABI ya kurudisha muundo.
    ///
    /// Hurejesha `None` kwa `Void`, safu, na aina mchanganyiko ambazo si
    /// sehemu za mwisho za skala (sehemu za muundo hupanuliwa kwa kujirudia
    /// na kiainishi ABI katika `abi::classify`).
    pub fn abi_class(&self) -> Option<AbiClass> {
        if self.is_float() {
            Some(AbiClass::Float)
        } else if self.is_integer_like() {
            Some(AbiClass::Integer)
        } else {
            None
        }
    }

    /// Upana wa *uhifadhi* wa aina hii kwa baiti.
    ///
    /// Kwa vielekezi upana unategemea lengwa; utekelezaji huu
    /// unachukulia lengwa la 64-bit (baiti 8).  Safu huzidisha upana wa
    /// kipengele kwa idadi.  Upana wa muundo ni jumla ya upana wa sehemu
    /// (zimebanwa, hakuna ujazo kwa sasa — mpangilio halisi umekabidhiwa
    /// kwa LLVM).
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
                // Khesabu kwa upatanisho ili kuendana na mpangilio wa muundo wa LLVM.
                let mut off: usize = 0;
                for (_, fty) in fields {
                    let fw = fty.width_bytes();
                    let align = std::cmp::min(fw, 8);
                    off = (off + align - 1) & !(align - 1);
                    off += fw;
                }
                let max_align = fields.iter()
                    .map(|(_, ty)| std::cmp::min(ty.width_bytes(), 8))
                    .max().unwrap_or(4);
                off = (off + max_align - 1) & !(max_align - 1);
                off
            }

            IrType::Array { element, count } => {
                element.width_bytes() * (*count as usize)
            }
        }
    }

    /// Upatanisho wa aina hii kwa baiti.
    ///
    /// Kanuni rahisi: upatanisho == upana asilia kwa aina za awali, 8 kwa
    /// vielekezi kwenye 64-bit, na 8 kwa miundo/safu (hifadhi —
    /// LLVM hukokotoa upatanisho halisi wa ABI baadaye).
    pub fn alignment_bytes(&self) -> usize {
        match self {
            IrType::Void => 1,

            IrType::I8 | IrType::A8 | IrType::B1 | IrType::B8 | IrType::W8 => 1,
            IrType::I16 | IrType::A16 | IrType::B16 | IrType::W16 | IrType::F16 => 2,
            IrType::I32 | IrType::A32 | IrType::B32 | IrType::W32 | IrType::F32 => 4,
            IrType::I64 | IrType::A64 | IrType::B64 | IrType::W64 | IrType::F64 => 8,
            IrType::I128 | IrType::A128 | IrType::F128 => 16,

            IrType::Ptr(_) | IrType::FnPtr { .. } => 8,

            // Hifadhi: panga miundo/safu kwa upana wa kielekezi.
            // LLVM itakaza hili wakati wa kodejeni.
            IrType::Struct { .. } | IrType::Array { .. } => 8,
        }
    }

    /// Lebo inayosomwa na binadamu kama Swa inayotumiwa na `Display`.
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

    // -- kutoka_kwa_aina_ya_swa --------------------------------------------

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
        // Wazi — all widths
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

    // -- darasa_la_abi ------------------------------------------------------

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

    // -- upana_katika_baiti --------------------------------------------------

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
        assert_eq!(s.width_bytes(), 16); // 4 + 4(pad) + 8 (alignment-aware)
    }

    #[test]
    fn test_width_bytes_array() {
        let a = IrType::Array {
            element: Box::new(IrType::I32),
            count: 4,
        };
        assert_eq!(a.width_bytes(), 16); // 4 * 4
    }

    // -- upatanisho_katika_baiti ---------------------------------------------

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
