//! Compiler driver (stub).

use crate::diagnostics::{Diagnostic, DiagnosticBag};
use crate::ir::Module as IrModule;
use std::path::PathBuf;

pub struct Driver {
    pub diagnostics: DiagnosticBag,
}

impl Driver {
    pub fn new() -> Self {
        Self {
            diagnostics: DiagnosticBag::new(),
        }
    }

    pub fn print_tokens(&mut self, _source: &str, _path: PathBuf) -> Result<(), Vec<Diagnostic>> {
        Ok(())
    }

    pub fn check(&mut self, _source: &str, _path: PathBuf) -> Result<(), Vec<Diagnostic>> {
        Ok(())
    }

    pub fn compile_to_ir(
        &mut self,
        source: &str,
        path: PathBuf,
    ) -> Result<IrModule, Vec<Diagnostic>> {
        // 1. Lex
        let lexer = crate::lexer::Lexer::new(source);
        let tokens = lexer.tokenize();

        // 2. Parse
        let (aina, thamani, kushoto, kulia, tiga, nne, jina_off, pool, count) =
            crate::parser::parse_full(&tokens)
                .map_err(|e| vec![Diagnostic::error(e, crate::diagnostics::SourceSpan::point(0, 0))])?;

        // 3. Lower to IR
        let module = crate::ir::lower::lower(
            &aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, count,
        );

        Ok(module)
    }
}
