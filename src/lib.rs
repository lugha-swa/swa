//! Kande — mkusanyaji wa Swa.
//!
//! Lugha ya programu za mfumo yenye sintaksia ya Kiswahili.
//! Swa inachukua nafasi ya C kama safu ya mkataba wa mashine.

pub mod abi;
pub mod ast;
pub mod codegen;
pub mod diagnostics;
pub mod driver;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod sema;

// Safirisha tena aina muhimu kwa urahisi.
pub use diagnostics::{Diagnostic, DiagnosticBag, Severity, SourceLocation, SourceSpan};
