//! Compiler driver — lex, parse, and lower Swa source to IR.

use crate::diagnostics::{Diagnostic, DiagnosticBag};
use crate::ir::Module as IrModule;
use std::fs;
use std::path::{Path, PathBuf};

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

    /// Preprocess `husisha` directives by reading and inlining included files.
    /// Handles both `husisha C::stdio` (skipped — C headers) and
    /// `husisha "path.swa"` (inlined).
    fn resolve_husisha(
        &mut self,
        source: &str,
        parent_dir: &Path,
        already_included: &mut Vec<String>,
    ) -> Result<String, Vec<Diagnostic>> {
        let mut out = String::with_capacity(source.len());
        let mut i = 0;
        let bytes = source.as_bytes();
        let len = bytes.len();

        while i < len {
            // Check for "husisha" at start of line.
            let at_line_start = i == 0 || bytes[i - 1] == b'\n';
            if at_line_start
                && i + 7 <= len
                && &bytes[i..i + 7] == b"husisha"
                && (i + 7 >= len || bytes[i + 7].is_ascii_whitespace())
            {
                // Skip past "husisha" and whitespace.
                i += 7;
                while i < len && bytes[i].is_ascii_whitespace() {
                    i += 1;
                }

                if i < len && bytes[i] == b'"' {
                    // Quoted path: husisha "path/file.swa"
                    i += 1;
                    let path_start = i;
                    while i < len && bytes[i] != b'"' {
                        i += 1;
                    }
                    let rel_path = std::str::from_utf8(&bytes[path_start..i])
                        .unwrap_or("");
                    if i < len { i += 1; } // skip closing "
                    // Skip trailing semicolon and newline.
                    while i < len && (bytes[i] == b';' || bytes[i].is_ascii_whitespace()) {
                        i += 1;
                    }

                    let include_path = parent_dir.join(rel_path);
                    let canon = include_path.canonicalize().unwrap_or(include_path.clone());
                    let canon_str = canon.to_string_lossy().to_string();

                    if !already_included.contains(&canon_str) {
                        already_included.push(canon_str.clone());
                        match fs::read_to_string(&canon) {
                            Ok(inc_source) => {
                                let resolved = self.resolve_husisha(
                                    &inc_source,
                                    canon.parent().unwrap_or(parent_dir),
                                    already_included,
                                )?;
                                out.push_str(&resolved);
                                out.push('\n');
                            }
                            Err(e) => {
                                return Err(vec![Diagnostic::error(
                                    format!("haiwezi kufungua faili '{}': {}", rel_path, e),
                                    crate::diagnostics::SourceSpan::point(0, 0),
                                )]);
                            }
                        }
                    }
                } else if i < len && bytes[i] == b'C' {
                    // husisha C::stdio — skip entire line (C headers).
                    while i < len && bytes[i] != b'\n' {
                        i += 1;
                    }
                    if i < len { i += 1; } // skip newline
                } else {
                    // husisha followed by something unrecognised — skip line.
                    while i < len && bytes[i] != b'\n' {
                        i += 1;
                    }
                    if i < len { i += 1; }
                }
            } else {
                out.push(bytes[i] as char);
                i += 1;
            }
        }

        Ok(out)
    }

    pub fn compile_to_ir(
        &mut self,
        source: &str,
        path: PathBuf,
    ) -> Result<IrModule, Vec<Diagnostic>> {
        // 0. Resolve husisha includes.
        let parent_dir = path.parent().unwrap_or(Path::new("."));
        let mut already_included: Vec<String> = Vec::new();
        let full_source = self.resolve_husisha(source, parent_dir, &mut already_included)?;

        // 1. Lex
        let lexer = crate::lexer::Lexer::new(&full_source);
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
