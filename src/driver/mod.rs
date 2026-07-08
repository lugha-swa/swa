//! Kiendeshi cha mkusanyaji — soma, chambua, na teremsha chanzo cha Swa hadi IR.

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

    pub fn print_tokens(&mut self, source: &str, _path: PathBuf) -> Result<(), Vec<Diagnostic>> {
        let lexer = crate::lexer::Lexer::new(source);
        let tokens = lexer.tokenize();
        for tok in &tokens {
            println!("{}:{}: {:?}  {}", tok.span.start.line, tok.span.start.column, tok.kind, tok.lexeme);
        }
        Ok(())
    }

    pub fn check(&mut self, source: &str, path: PathBuf) -> Result<(), Vec<Diagnostic>> {
        match self.compile_to_ir(source, path) {
            Ok(_) => {
                if self.diagnostics.has_errors() {
                    let diags: Vec<Diagnostic> = self.diagnostics.all().to_vec();
                    Err(diags)
                } else {
                    Ok(())
                }
            }
            Err(diags) => Err(diags),
        }
    }

    /// Chakia maelekezo ya `husisha` kwa kusoma na kupachika faili zilizojumuishwa.
    /// Hushughulikia `husisha C::stdio` (imechukuliwa — vichwa vya C) na
    /// `husisha "path.swa"` (iliyopachikwa).
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
            // Angalia "husisha" mwanzoni mwa mstari.
            let at_line_start = i == 0 || bytes[i - 1] == b'\n';
            if at_line_start
                && i + 7 <= len
                && &bytes[i..i + 7] == b"husisha"
                && (i + 7 >= len || bytes[i + 7].is_ascii_whitespace())
            {
                // Ruka "husisha" na nafasi nyeupe.
                i += 7;
                while i < len && bytes[i].is_ascii_whitespace() {
                    i += 1;
                }

                if i < len && bytes[i] == b'"' {
                    // Njia iliyonukuliwa: husisha "path/file.swa"
                    i += 1;
                    let path_start = i;
                    while i < len && bytes[i] != b'"' {
                        i += 1;
                    }
                    let rel_path = std::str::from_utf8(&bytes[path_start..i])
                        .unwrap_or("");
                    if i < len { i += 1; } // ruka nukta ya kufunga "
                    // Ruka semicolon ya mwisho na mstari mpya.
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
                } else if i < len && bytes[i] == b'{' {
                    // Sintaksia: husisha { njia }  au  husisha { njia };
                    i += 1; // ruka '{'
                    while i < len && (bytes[i] == b' ' || bytes[i] == b'\t') {
                        i += 1;
                    }
                    let path_start = i;
                    while i < len && bytes[i] != b'}' && bytes[i] != b'\n' {
                        i += 1;
                    }
                    let rel_path = std::str::from_utf8(&bytes[path_start..i])
                        .map(|s| s.trim())
                        .unwrap_or("");
                    while i < len && bytes[i] != b'}' { i += 1; }
                    if i < len { i += 1; } // ruka '}'
                    // Ruka semicolon na mstari mpya.
                    while i < len && (bytes[i] == b';' || bytes[i].is_ascii_whitespace()) {
                        i += 1;
                    }

                    if !rel_path.is_empty() {
                        let include_path = parent_dir.join(rel_path);
                        let canon = include_path.canonicalize().unwrap_or(include_path.clone());
                        let canon_str = canon.to_string_lossy().to_string();

                        if !already_included.contains(&canon_str) && canon.exists() {
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
                    }
                } else if i < len && bytes[i] == b'C' {
                    // husisha C::stdio — ruka mstari mzima (vichwa vya C).
                    while i < len && bytes[i] != b'\n' {
                        i += 1;
                    }
                    if i < len { i += 1; } // ruka mstari mpya
                } else {
                    // husisha ikifuatiwa na kitu kisichotambulika — ruka mstari.
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
        // 0. Tatua husisha zilizojumuishwa.
        let parent_dir = path.parent().unwrap_or(Path::new("."));
        let mut already_included: Vec<String> = Vec::new();
        let full_source = self.resolve_husisha(source, parent_dir, &mut already_included)?;

        // 1. Changanua (lex)
        let lexer = crate::lexer::Lexer::new(&full_source);
        let tokens = lexer.tokenize();

        // 2. Changanua (parse)
        let (aina, thamani, kushoto, kulia, tiga, nne, jina_off, pool, count) =
            crate::parser::parse_full(&tokens)
                .map_err(|e| vec![Diagnostic::error(e, crate::diagnostics::SourceSpan::point(0, 0))])?;

        // 2b. Ukaguzi wa haraka wa kisemantiki
        let diags_sema = crate::sema::kagua_asti(
            &aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, count,
        );
        for d in diags_sema {
            self.diagnostics.push(d);
        }

        // 3. Teremsha hadi IR
        let module = crate::ir::lower::lower(
            &aina, &kushoto, &kulia, &tiga, &nne, &thamani, &jina_off, &pool, count,
        );

        Ok(module)
    }
}
