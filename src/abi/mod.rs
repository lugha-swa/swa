//! ABI (Application Binary Interface) definitions for the Swa language.
//!
//! ## Swa ABI v1.0 — summary
//!
//! | Rule                               | Behaviour                          |
//! |------------------------------------|------------------------------------|
//! | Scalar return                     | In register (integer or float)     |
//! | Struct return, 1–2 leaf fields    | Direct (fields in registers)       |
//! | Struct return, > 2 leaf fields    | Hidden pointer (sret)              |
//! | Struct arguments > 2 leaf fields  | By reference / copy                |
//!
//! The classification logic lives in the [`classify`] submodule.

pub mod classify;

pub use classify::classify_return;
