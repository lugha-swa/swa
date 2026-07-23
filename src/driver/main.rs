//! Kuingilia kwa mkusanyaji wa Swa (`kande`).
//!
//! Matumizi:
//!   kande file.swa -o file.o   — sanya hadi faili la kitu
//!   kande file.swa -o file.exe — sanya na unganisha hadi faili linaloweza kutekelezwa
//!   kande --ir file.swa         — chapisha Swa IR hadi stdout
//!   kande --llvm file.swa       — chapisha LLVM IR hadi stdout
//!   kande --ll file.ll          — sanya maandishi ya LLVM IR hadi .o
//!   kande --tokens file.swa     — chapisha mkondo wa tokeni

use kande_lib::codegen::llvm::ffi::LLVMCodeGenOptLevel;
use kande_lib::codegen::llvm::LlvmBackend;
use kande_lib::driver::Driver;
use std::env;
use std::path::{Path, PathBuf};
use std::process;

/// Jaribu kuunganisha faili la kitu hadi faili linaloweza kutekelezwa kupitia clang.
/// Hurejesha hali ya kutoka ya clang kwenye mafanikio, None kama clang haipatikani.
fn try_link(obj: &Path, exe: &Path) -> Option<i32> {
    let clang_paths = ["clang", "C:\\LLVM18\\bin\\clang.exe"];
    let clang = clang_paths.iter().find(|p| {
        let c = std::process::Command::new(p).arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
        c.map(|s| s.success()).unwrap_or(false)
    });
    let clang = match clang {
        Some(c) => c,
        None => return None,
    };
    // Tumia lengwa la GNU kwenye Windows — linalingana na tatu ya IR iliyowekwa na nyuma.
    let target = if cfg!(windows) { "x86_64-pc-windows-gnu" } else { "x86_64-unknown-linux-gnu" };
    // Tafuta libgcc kwa __chkstk (mfumo mkubwa wa rafu kutoka safu kubwa).
    let gcc_base = if cfg!(windows) {
        std::path::PathBuf::from("C:\\ProgramData\\mingw64\\mingw64\\lib\\gcc\\x86_64-w64-mingw32")
    } else {
        std::path::PathBuf::from("/usr/lib/gcc/x86_64-linux-gnu")
    };
    let gcc_lib = if gcc_base.exists() {
        std::fs::read_dir(&gcc_base).ok()
            .and_then(|d| {
                d.filter_map(|e| e.ok())
                    .filter(|e| e.path().is_dir())
                    .map(|e| e.path())
                    .next()
            })
            .unwrap_or(gcc_base.clone())
    } else {
        gcc_base.clone()
    };
    let mut cmd = std::process::Command::new(clang);
    cmd.arg("-target").arg(target)
       .arg(obj).arg("-o").arg(exe)
       .arg("-Wl,--defsym,andika=printf");  // ramani printf ya Swa hadi printf ya libc

    if cfg!(windows) {
        cmd.arg("-L").arg(&gcc_lib)
           .arg("-lgcc")                     // kwa __chkstk (mfumo mkubwa wa rafu)
           .arg("-Wl,--stack,8388608");      // 8MB hifadhi ya rafu kwa BSS kubwa
    } else {
        // Linux: ukubwa wa rafu unadhibitiwa na ulimit, hakuna bendera ya wazi inayohitajika.
        // libgcc inaunganishwa kiotomatiki na clang kwenye Linux.
    }

    let status = cmd.status().ok()?;
    Some(status.code().unwrap_or(1))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Kande — mkusanyaji wa Swa v0.1.0");
        eprintln!("Lugha ya mfumo kwa Kiswahili safi.");
        eprintln!();
        eprintln!("Matumizi:");
        eprintln!("  kande file.swa -o file.o   — sanya hadi faili la kitu (object)");
        eprintln!("  kande file.swa -o file.exe — sanya na unganisha (compile + link)");
        eprintln!("  kande --ir file.swa        — toa Swa IR");
        eprintln!("  kande --llvm file.swa      — toa LLVM IR");
        eprintln!("  kande --ll file.ll         — sanya LLVM IR faili");
        eprintln!("  kande --check file.swa     — kagua bila kutoa msimbo");
        eprintln!("  kande --tokens file.swa    — toa tokeni");
        process::exit(1);
    }

    // Changanua bendera ya hiari ya pato -o na bendera ya uboreshaji --opt / -O.
    let mut output_path: Option<PathBuf> = None;
    let mut opt_flag = false;
    let mut positional: Vec<String> = Vec::new();
    {
        let mut i = 1;
        while i < args.len() {
            if args[i] == "-o" {
                i += 1;
                if i < args.len() {
                    output_path = Some(PathBuf::from(&args[i]));
                }
            } else if args[i] == "--opt" || args[i] == "-O" {
                opt_flag = true;
            } else {
                positional.push(args[i].clone());
            }
            i += 1;
        }
    }

    let (mode, file_arg) = if positional.is_empty() {
        eprintln!("hitilafu: faili halijabainishwa");
        process::exit(1);
    } else if positional[0] == "--ir" {
        ("ir", positional.get(1).map(|s| s.as_str()).unwrap_or(""))
    } else if positional[0] == "--llvm" {
        ("llvm", positional.get(1).map(|s| s.as_str()).unwrap_or(""))
    } else if positional[0] == "--ll" {
        ("ll", positional.get(1).map(|s| s.as_str()).unwrap_or(""))
    } else if positional[0] == "--check" {
        ("check", positional.get(1).map(|s| s.as_str()).unwrap_or(""))
    } else if positional[0] == "--tokens" {
        ("tokens", positional.get(1).map(|s| s.as_str()).unwrap_or(""))
    } else {
        ("compile", positional[0].as_str())
    };

    if file_arg.is_empty() {
        eprintln!("hitilafu: faili halijabainishwa");
        process::exit(1);
    }

    let file_path = PathBuf::from(file_arg);
    if !file_path.exists() {
        eprintln!("hitilafu: faili halipo: {}", file_arg);
        process::exit(1);
    }

    let source = match std::fs::read_to_string(&file_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("hitilafu: haikuweza kusoma faili: {}", e);
            process::exit(1);
        }
    };

    let mut driver = Driver::new();
    match mode {
        "tokens" => {
            match driver.print_tokens(&source, file_path) {
                Ok(()) => {}
                Err(diags) => {
                    for d in &diags {
                        eprintln!("{}", d.render(&source));
                    }
                    process::exit(1);
                }
            }
        }
        "check" => {
            match driver.check(&source, file_path) {
                Ok(()) => {}
                Err(diags) => {
                    for d in &diags {
                        eprintln!("{}", d.render(&source));
                    }
                    process::exit(1);
                }
            }
        }
        "ir" => {
            match driver.compile_to_ir(&source, file_path) {
                Ok(module) => { for d in driver.diagnostics.all() { eprintln!("{}", d.render(&source)); }
                    println!("moduli @{}", module.name);
                    for (name, ty) in &module.types {
                        println!("aina {} = {}", name, ty);
                    }
                    for global in &module.globals {
                        println!("data @{} = ... ({} bytes)", global.name, global.bytes.len());
                    }
                    for func in &module.functions {
                        print!("fani @{} : ", func.name);
                        print!("{} ", func.return_ty);
                        match kande_lib::abi::classify_return(&func.return_ty) {
                            kande_lib::ir::IrReturnClass::Direct => print!("[moja_kwa_moja]"),
                            kande_lib::ir::IrReturnClass::HiddenPtr => print!("[kiashiria_fiche]"),
                        }
                        println!();
                        println!("  vigezo: {:?}", func.params.iter().map(|(n, t)| format!("{}: {}", n, t)).collect::<Vec<_>>());
                        println!("  vizuizi: {}", func.blocks.len());
                    }
                }
                Err(diags) => {
                    for d in &diags {
                        eprintln!("{}", d.render(&source));
                    }
                    process::exit(1);
                }
            }
        }
        "llvm" => {
            match driver.compile_to_ir(&source, file_path) {
                Ok(module) => { for d in driver.diagnostics.all() { eprintln!("{}", d.render(&source)); }
                    let backend = kande_lib::codegen::llvm::LlvmBackend::new();
                    match backend.compile(&module) {
                        Ok(llvm_module) => {
                            unsafe {
                                let ir_str = kande_lib::codegen::llvm::ffi::LLVMPrintModuleToString(llvm_module);
                                if !ir_str.is_null() {
                                    use std::ffi::CStr;
                                    println!("{}", CStr::from_ptr(ir_str).to_string_lossy());
                                    kande_lib::codegen::llvm::ffi::LLVMDisposeMessage(ir_str);
                                }
                                kande_lib::codegen::llvm::ffi::LLVMDisposeModule(llvm_module);
                            }
                        }
                        Err(diags) => {
                            for d in &diags {
                                eprintln!("{}", d.render(&source));
                            }
                            process::exit(1);
                        }
                    }
                }
                Err(diags) => {
                    for d in &diags {
                        eprintln!("{}", d.render(&source));
                    }
                    process::exit(1);
                }
            }
        }
        "compile" => {
            let out_path = output_path.unwrap_or_else(|| file_path.with_extension("o"));
            let want_link = out_path.extension()
                .and_then(|e| e.to_str())
                .map(|e| e.eq_ignore_ascii_case("exe"))
                .unwrap_or(false);
            // Wakati wa kuunganisha, toa kitu kwa njia ya .o ili clang ipate
            // faili halisi la kitu; vinginevyo toa moja kwa moja kwa njia iliyoombwa.
            let obj_path: PathBuf = if want_link {
                out_path.with_extension("o")
            } else {
                out_path.clone()
            };
            match driver.compile_to_ir(&source, file_path) {
                Ok(module) => { for d in driver.diagnostics.all() { eprintln!("{}", d.render(&source)); }
                    let opt_level = if opt_flag {
                        LLVMCodeGenOptLevel::Default
                    } else {
                        LLVMCodeGenOptLevel::None
                    };
                    let backend = LlvmBackend::new().with_opt_level(opt_level);
                    match backend.compile_to_file(&module, &obj_path) {
                        Ok(()) => {
                            if want_link {
                                match try_link(&obj_path, &out_path) {
                                    Some(0) => {
                                        println!("Kande: {} → {} (linked)", file_arg, out_path.display());
                                    }
                                    Some(code) => {
                                        eprintln!("onyo: kiunganishi kilishindwa (msimbo={}), {} imehifadhiwa", code, obj_path.display());
                                        println!("Kande: {} → {}", file_arg, obj_path.display());
                                    }
                                    None => {
                                        eprintln!("onyo: clang haipatikani — inahitajika kwa kuunganisha");
                                        println!("Kande: {} → {}", file_arg, obj_path.display());
                                    }
                                }
                                if out_path.exists() {
                                    let _ = std::fs::remove_file(&obj_path);
                                }
                            } else {
                                println!("Kande: {} → {}", file_arg, obj_path.display());
                            }
                        }
                        Err(diags) => {
                            for d in &diags {
                                eprintln!("{}", d.render(&source));
                            }
                            process::exit(1);
                        }
                    }
                }
                Err(diags) => {
                    for d in &diags {
                        eprintln!("{}", d.render(&source));
                    }
                    process::exit(1);
                }
            }
        }
        "ll" => {
            let obj_path = output_path.unwrap_or_else(|| file_path.with_extension("o"));
            let ll_text = match std::fs::read_to_string(&file_path) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("hitilafu: haikuweza kusoma faili: {}", e);
                    process::exit(1);
                }
            };
            let backend = LlvmBackend::new();
            match backend.compile_ll(&ll_text, &obj_path) {
                Ok(()) => {
                    println!("Kande: {} → {}", file_arg, obj_path.display());
                }
                Err(diags) => {
                    for d in &diags {
                        eprintln!("{}", d.message);
                    }
                    process::exit(1);
                }
            }
        }
        _ => unreachable!(),
    }
}
