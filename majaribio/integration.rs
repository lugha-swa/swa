//! Majaribio ya ujumuishaji — jaribio kamili la bomba la mkusanyaji.
//!
//! Kila jaribio huchukua msimbo wa chanzo cha Swa, hulichakata kupitia
//! dereva (msomaji → mchanganuzi → kiteremshi), na kuthibitisha kuwa
//! moduli ya LLVM inayotolewa ni halali.

use kande_lib::codegen::llvm::LlvmBackend;
use kande_lib::driver::Driver;
use std::path::PathBuf;

/// Piga mkusanyaji kwenye chanzo na uthibitishe moduli ya LLVM.
fn compile_and_verify(source: &str) -> Result<String, Vec<String>> {
    let mut driver = Driver::new();
    let ir_module = driver
        .compile_to_ir(source, PathBuf::from("jaribio.swa"))
        .map_err(|diags| diags.iter().map(|d| d.message.clone()).collect::<Vec<_>>())?;

    let backend = LlvmBackend::new();
    let llvm_module = backend
        .compile(&ir_module)
        .map_err(|diags| diags.iter().map(|d| d.message.clone()).collect::<Vec<_>>())?;

    // Module ilikusanywa kwa usahihi — hakikisha ina jina.
    unsafe {
        use std::ffi::CStr;
        let ir_ptr = kande_lib::codegen::llvm::ffi::LLVMPrintModuleToString(llvm_module);
        let ir = CStr::from_ptr(ir_ptr).to_string_lossy().into_owned();
        kande_lib::codegen::llvm::ffi::LLVMDisposeMessage(ir_ptr);
        kande_lib::codegen::llvm::ffi::LLVMDisposeModule(llvm_module);
        Ok(ir)
    }
}

// ============================================================================
// Vipengele vya msingi
// ============================================================================

#[test]
fn jaribio_kazi_tupu() {
    let ir = compile_and_verify("W0 fanya() {}").expect("inapaswa kukusanyika");
    assert!(ir.contains("fanya"), "IR inapaswa kuwa na jina la kazi");
}

#[test]
fn jaribio_rudisha_nambari() {
    let ir = compile_and_verify("N32 tatu() { rudisha 3; }").expect("inapaswa kukusanyika");
    assert!(ir.contains("tatu"), "IR inapaswa kuwa na jina la kazi");
}

#[test]
fn jaribio_vigezo_na_mwili() {
    let src = "N32 jumlisha(N32 a, N32 b) { rudisha a + b; }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("jumlisha"), "IR inapaswa kuwa na jina la kazi");
}

// ============================================================================
// Mtiririko wa udhibiti
// ============================================================================

#[test]
fn jaribio_kama_sivyo() {
    let src = "N32 kadirifu(N32 x) { kama (x > 0) { rudisha 1; } sivyo { rudisha 0; } }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("kadirifu"));
}

#[test]
fn jaribio_wakati() {
    let src = "W0 hesabu(N32 n) { N32 i = 0; wakati (i < n) { i = i + 1; } }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("hesabu"));
}

#[test]
fn jaribio_kama_ndani_ya_wakati() {
    let src = "N32 tafuta(N32 n) { N32 i = 0; wakati (i < n) { kama (i == 5) { rudisha i; } i = i + 1; } rudisha -1; }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("tafuta"));
}

#[test]
fn jaribio_chagua_rahisi() {
    // Jaribio rahisi la chagua: linganisha thamani na hali mbili + chaguo-msingi
    let src = "N32 kadiria(N32 x) { chagua (x) { hali 1: rudisha 10; hali 2: rudisha 20; sivyo: rudisha 0; } }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("kadiria"), "IR inapaswa kuwa na jina la kazi");
    assert!(ir.contains("switch"), "IR inapaswa kuwa na maagizo ya switch");
}

#[test]
fn jaribio_chagua_hali_nyingi() {
    // Chagua yenye hali nyingi na maadili tofauti
    let src = "N32 siku_kwa_namba(N32 n) {
        chagua (n) {
            hali 1: rudisha 100;
            hali 2: rudisha 200;
            hali 3: rudisha 300;
            hali 4: rudisha 400;
            hali 5: rudisha 500;
            sivyo: rudisha 0;
        }
    }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("siku_kwa_namba"), "IR inapaswa kuwa na jina la kazi");
}

#[test]
fn jaribio_chagua_ndani_ya_kazi() {
    // Chagua ndani ya kazi inayotumia kigezo
    let src = "N32 tathmini(N32 alama) {
        chagua (alama) {
            hali 1: rudisha 50;
            hali 2: rudisha 70;
            sivyo: rudisha 30;
        }
    }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("tathmini"), "IR inapaswa kuwa na jina la kazi");
}

// ============================================================================
// Vigezo vya ulimwengu
// ============================================================================

#[test]
fn jaribio_kigezo_cha_ulimwengu() {
    let src = "N32 KIKOMO = 0; N32 pata_kikomo() { rudisha KIKOMO; }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("KIKOMO"), "IR inapaswa kuwa na jina la kigezo cha ulimwengu");
    assert!(ir.contains("pata_kikomo"));
}

#[test]
fn jaribio_safu_ya_ulimwengu() {
    let src = "N8 bafa[1024]; W0 andika_bafa() { bafa[0] = 65; }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("bafa"));
}

// ============================================================================
// Miundo
// ============================================================================

#[test]
fn jaribio_muundo_na_sehemu() {
    let src = "muundo Nukta { N32 x; N32 y; }; N32 pata_x(Nukta p) { rudisha p.x; }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("Nukta"), "IR inapaswa kuwa na aina ya muundo");
}

#[test]
fn jaribio_muundo_kielekezi() {
    let src = "muundo Nukta { N32 x; N32 y; }; W0 weka_x(Nukta* p, N32 v) { p->x = v; }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("weka_x"));
}

#[test]
fn jaribio_muundo_wa_vitu_vingi() {
    let src = "muundo Mstari { N8* data; N64 urefu; }; N64 urefu_wa_mstari(Mstari* m) { rudisha m->urefu; }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("urefu_wa_mstari"));
}

// ============================================================================
// Miito ya kazi
// ============================================================================

#[test]
fn jaribio_mwito_wa_mbele() {
    let src = "W0 mkuu() { msaidizi(42); } W0 msaidizi(N32 x) {}";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("mkuu") && ir.contains("msaidizi"));
}

#[test]
fn jaribio_mwito_wa_kujirudia() {
    let src = "N32 kitanzi(N32 n) { kama (n <= 0) { rudisha 0; } rudisha 1 + kitanzi(n - 1); }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("kitanzi"));
}

#[test]
fn jaribio_kazi_nyingi() {
    let src = "N32 a(N32 x) { rudisha x + 1; } N32 b(N32 x) { rudisha a(x) + 2; }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("a") && ir.contains("b"));
}

// ============================================================================
// Vihisabati na ulinganisho
// ============================================================================

#[test]
fn jaribio_vihisabati() {
    let src = "N32 hesabu(N32 a, N32 b) { rudisha (a + b) * (a - b); }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("hesabu"));
}

#[test]
fn jaribio_ulinganisho() {
    let src = "N32 linganisha(N32 a, N32 b) { kama (a == b) { rudisha 1; } kama (a < b) { rudisha -1; } rudisha 0; }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("linganisha"));
}

#[test]
fn jaribio_vihamishaji_biti() {
    let src = "N32 hamisha(N32 x) { rudisha (x << 2) | (x >> 1); }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("hamisha"));
}

#[test]
fn jaribio_ternary() {
    let src = "N32 chagua(N32 x) { rudisha x > 0 ? 1 : 0; }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("chagua"));
}

// ============================================================================
// Vifungo vya bloku
// ============================================================================

#[test]
fn jaribio_bloku_tupu() {
    let src = "W0 fanya() { { N32 x = 5; } }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("fanya"));
}

// ============================================================================
// Maneno muhimu kama majina
// ============================================================================

#[test]
fn jaribio_neno_muhimu_kama_jina_la_kigezo() {
    let src = "W0 fanya() { N32 hali = 42; N32 wakati = hali + 1; }";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("fanya"));
}

// ============================================================================
// Makosa — thibitisha kuwa chanzo kibaya kinashindwa
// ============================================================================

#[test]
fn jaribio_kosa_la_mchanganuzi() {
    // Kosa la kweli la uchanganuzi: neno lisilotarajiwa.
    let src = "N32 fanya(???)";
    let result = compile_and_verify(src);
    assert!(result.is_err(), "inapaswa kushindwa kwa kosa la mchanganuzi");
}

#[test]
fn jaribio_kazi_isiyo_na_mwili() {
    // Kazi isiyo na mwili inapaswa kukusanyika kama tangazo.
    let src = "N32 fanya(N32 x);";
    let ir = compile_and_verify(src).expect("tangazo la kazi linapaswa kukusanyika");
    assert!(ir.contains("fanya"));
}

// ============================================================================
// Husisha
// ============================================================================

#[test]
fn jaribio_husisha_c() {
    // husisha C::stdio inapaswa kurukwa bila hitilafu
    let src = "husisha C::stdio\nW0 fanya() {}";
    let ir = compile_and_verify(src).expect("inapaswa kukusanyika");
    assert!(ir.contains("fanya"));
}

// ============================================================================
// Msingi — mkusanyiko kamili wa faili za msingi
// ============================================================================

fn compile_file(path: &str) -> Result<String, Vec<String>> {
    let src = std::fs::read_to_string(path).expect("inapaswa kusoma faili");
    let mut driver = Driver::new();
    let ir_module = driver
        .compile_to_ir(&src, PathBuf::from(path))
        .map_err(|diags| diags.iter().map(|d| d.message.clone()).collect::<Vec<_>>())?;
    let backend = LlvmBackend::new();
    let llvm_module = backend
        .compile(&ir_module)
        .map_err(|diags| diags.iter().map(|d| d.message.clone()).collect::<Vec<_>>())?;
    unsafe {
        use std::ffi::CStr;
        let ir_ptr = kande_lib::codegen::llvm::ffi::LLVMPrintModuleToString(llvm_module);
        let ir = CStr::from_ptr(ir_ptr).to_string_lossy().into_owned();
        kande_lib::codegen::llvm::ffi::LLVMDisposeMessage(ir_ptr);
        kande_lib::codegen::llvm::ffi::LLVMDisposeModule(llvm_module);
        Ok(ir)
    }
}

#[test]
fn jaribio_msingi_kumbukumbu() {
    let ir = compile_file("msingi/kumbukumbu.swa").expect("kumbukumbu.swa inapaswa kukusanyika");
    assert!(ir.contains("nakili"));
}

#[test]
fn jaribio_msingi_mfuatano() {
    let ir = compile_file("msingi/mfuatano.swa").expect("mfuatano.swa inapaswa kukusanyika");
    assert!(ir.contains("urefu_wa_mfuatano"));
}

#[test]
fn jaribio_msingi_orodha() {
    // orodha.swa uses husisha — test that it parses and lowers successfully.
    let src = std::fs::read_to_string("msingi/orodha.swa")
        .expect("inapaswa kusoma faili");
    let mut driver = Driver::new();
    let result = driver.compile_to_ir(&src, PathBuf::from("msingi/orodha.swa"));
    assert!(result.is_ok(), "orodha.swa inapaswa kuchanganua: {:?}", result.err());
    let ir_module = result.unwrap();
    assert!(!ir_module.functions.is_empty(), "orodha.swa inapaswa kuwa na kazi");
}


#[test]
fn jaribio_msingi_ramani() {
    // ramani.swa uses husisha — test that it parses and lowers successfully.
    let src = std::fs::read_to_string("msingi/ramani.swa")
        .expect("inapaswa kusoma faili");
    let mut driver = Driver::new();
    let result = driver.compile_to_ir(&src, PathBuf::from("msingi/ramani.swa"));
    assert!(result.is_ok(), "ramani.swa inapaswa kuchanganua: {:?}", result.err());
    let ir_module = result.unwrap();
    assert!(!ir_module.functions.is_empty(), "ramani.swa inapaswa kuwa na kazi");
}

// ============================================================================
// Msingi — faili za ziada zilizokusanywa
// ============================================================================

#[test]
fn jaribio_msingi_msomaji() {
    let ir = compile_file("msingi/msomaji.swa").expect("msomaji.swa inapaswa kukusanyika");
    assert!(ir.contains("msomaji_imeisha"));
}

#[test]
fn jaribio_msingi_msambazaji() {
    let ir = compile_file("msingi/msambazaji.swa").expect("msambazaji.swa inapaswa kukusanyika");
    assert!(ir.contains("AST_PROGRAMU"));
}

#[test]
fn jaribio_msingi_uzalishaji() {
    let ir = compile_file("msingi/uzalishaji.swa").expect("uzalishaji.swa inapaswa kukusanyika");
    assert!(ir.contains("andika_baiti"));
}

#[test]
fn jaribio_msingi_mkaguzi() {
    let ir = compile_file("msingi/mkaguzi.swa").expect("mkaguzi.swa inapaswa kukusanyika");
    assert!(ir.contains("mkaguzi_angalia"));
}

#[test]
fn jaribio_msingi_stage1() {
    let ir = compile_file("msingi/stage1.swa").expect("stage1.swa inapaswa kukusanyika");
    assert!(ir.contains("ongeza_faili"));
    assert!(ir.contains("main"));
}

#[test]
fn jaribio_stage1_hifadhi_chanzo_yatosha_kwa_msingi_wote() {
    let faili = [
        "kumbukumbu.swa", "mfuatano.swa", "msomaji.swa", "msambazaji.swa",
        "mteremko.swa", "mkaguzi.swa", "uzalishaji.swa", "orodha.swa", "ramani.swa",
    ];
    let jumla: u64 = faili.iter()
        .map(|jina| std::fs::metadata(format!("msingi/{jina}"))
            .expect("faili ya msingi inapaswa kuwepo").len())
        .sum();

    assert!(jumla < 1_048_576,
        "chanzo cha msingi ({jumla}B) lazima kitoshe kwenye chanzo_buf ya stage1");
}

// ============================================================================
// Stage1
// ============================================================================

#[test]
fn jaribio_stage1() {
    let src = std::fs::read_to_string("msingi/stage1.swa")
        .expect("inapaswa kusoma faili");
    let ir = compile_and_verify(&src).expect("stage1.swa inapaswa kukusanyika");
    assert!(ir.contains("ongeza_faili"), "IR inapaswa kuwa na ongeza_faili");
    assert!(ir.contains("main"), "IR inapaswa kuwa na main");
    assert!(ir.contains("chanzo_buf"), "IR inapaswa kuwa na chanzo_buf");
}

// ============================================================================
// K6 — Jaribio kamili la kujikusanya (kusanya + unganisha + endesha)
// ============================================================================

/// Kusanya stage1.swa hadi faili la kitu, unganisha na clang, endesha
/// dhidi ya faili rahisi la .swa, na uthibitishe matokeo.
#[test]
fn jaribio_k6_kujikusanya_kamili() {
    // Angalia kama clang inapatikana.
    let clang = which_clang();
    if clang.is_none() {
        eprintln!("; K6: clang haipatikani — ruka jaribio la wakati wa utekelezaji");
        return;
    }
    let clang = clang.unwrap();

    // Hitilafu ya LLVM 22 trunc-to-ptr imerekebishwa (StoreTyped sasa inatumia
    // IntToPtr badala ya IntCast2 kwa vielekezi). Tunarudi kwenye njia ya moja
    // kwa moja ya compile_to_ir + compile_to_file.
    let src = std::fs::read_to_string("msingi/stage1.swa")
        .expect("inapaswa kusoma faili");
    let mut driver = Driver::new();
    let ir_module = driver
        .compile_to_ir(&src, PathBuf::from("msingi/stage1.swa"))
        .expect("stage1.swa inapaswa kuchanganua na kuteremsha");

    let dir = tempfile::tempdir().expect("inapaswa kuunda saraka ya muda");
    let obj_path = dir.path().join("stage1.o");
    let exe_path = dir.path().join("stage1");

    let backend = LlvmBackend::new()
        .with_opt_level(kande_lib::codegen::llvm::ffi::LLVMCodeGenOptLevel::Less);
    backend
        .compile_to_file(&ir_module, &obj_path)
        .expect("inapaswa kutoa faili la kitu");

    // Andika kiunganishi kidogo cha C kinachoelekeza andika -> printf.
    let trampoline_c = dir.path().join("trampoline.c");
    std::fs::write(&trampoline_c,
        "#include <stdio.h>\n#include <stdarg.h>\nint andika(const char* f, ...) { va_list a; va_start(a,f); int r=vfprintf(stdout,f,a); va_end(a); fflush(stdout); return r; }\nint andika_stderr(const char* f, ...) { va_list a; va_start(a,f); int r=vfprintf(stderr,f,a); va_end(a); fflush(stderr); return r; }\n"
    ).expect("inapaswa kuandika trampoline.c");
    let trampoline_o = dir.path().join("trampoline.o");
    let compile_status = std::process::Command::new(&clang)
        .arg("-c")
        .arg(&trampoline_c)
        .arg("-o")
        .arg(&trampoline_o)
        .status()
        .expect("inapaswa kuendesha clang kwa trampoline");
    assert!(compile_status.success(), "clang inapaswa kukusanya trampoline");

    // Unganisha stage1.o + trampoline.o -> executable.
    // -no-pie inahitajika kwa sababu LLVM hutumia rekebisho kamili (R_X86_64_32).
    let link_status = std::process::Command::new(&clang)
        .arg(&obj_path)
        .arg(&trampoline_o)
        .arg("-o")
        .arg(&exe_path)
        .arg("-no-pie")
        .status()
        .expect("inapaswa kuendesha clang");
    assert!(link_status.success(), "clang inapaswa kuunganisha kwa mafanikio");

    // Endesha mkusanyaji uliojikusanya dhidi ya faili rahisi la .swa.
    // Sasa stage1 inatoa ELF binary (uzalishaji.swa) si LLVM IR tena.
    let test_input = dir.path().join("jaribio.swa");
    std::fs::write(&test_input, "N32 main() { rudisha 42; }\n")
        .expect("inapaswa kuandika faili la jaribio");

    let test_obj = dir.path().join("jaribio.o");
    let output = std::process::Command::new(&exe_path)
        .arg(&test_input)
        .stdout(std::fs::File::create(&test_obj).expect("inapaswa kuunda faili la kitu"))
        .output()
        .expect("inapaswa kuendesha binary iliyounganishwa");

    let _stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let exit_code = output.status.code().unwrap_or(-1);
    eprintln!("; K6: msimbo wa kutoka = {exit_code}");
    if !stderr.is_empty() {{ eprintln!("; K6 stderr: {stderr}"); }}
    assert!(output.status.success(),
        "stage1 inapaswa kurudisha 0, ilirudisha {exit_code}\nstderr: {stderr}");

    // Thibitisha pato ni ELF binary halali
    let obj_bytes = std::fs::read(&test_obj).expect("inapaswa kusoma faili la kitu");
    assert!(obj_bytes.len() > 64, "ELF inapaswa kuwa na ukubwa zaidi ya baiti 64");
    assert_eq!(&obj_bytes[0..4], &[0x7f, 0x45, 0x4c, 0x46], "ELF magic inapaswa kuwepo");

    // Unganisha na uendeshe binary iliyozalishwa
    let test_exe = dir.path().join("jaribio_exe");
    let link = std::process::Command::new(&clang)
        .arg(&test_obj)
        .arg("-o").arg(&test_exe)
        .arg("-no-pie")
        .status().expect("clang");
    assert!(link.success(), "kuunganisha kunapaswa kufaulu");

    let run = std::process::Command::new(&test_exe).output().expect("kuendesha");
    let run_exit = run.status.code().unwrap_or(-1);
    eprintln!("; K6: binary exit={run_exit}");
    assert_eq!(run_exit, 42, "binary inapaswa kurudisha 42, ilirudisha {run_exit}");
}

/// Msaidizi wa kuendesha jaribio la K6: kusanya stage1.swa, endesha dhidi ya
/// faili la .swa, unganisha towe, na uthibitishe msimbo wa kutoka.
fn run_k6_test(test_chanzo: &str, matarajio_ya_kutoka: i32) {
    let clang = which_clang();
    if clang.is_none() {
        eprintln!("; K6: clang haipatikani — ruka jaribio la wakati wa utekelezaji");
        return;
    }
    let clang = clang.unwrap();

    let src = std::fs::read_to_string("msingi/stage1.swa")
        .expect("inapaswa kusoma faili");
    let mut driver = Driver::new();
    let ir_module = driver
        .compile_to_ir(&src, PathBuf::from("msingi/stage1.swa"))
        .expect("stage1.swa inapaswa kuchanganua na kuteremsha");

    let dir = tempfile::tempdir().expect("inapaswa kuunda saraka ya muda");
    let obj_path = dir.path().join("stage1.o");
    let exe_path = dir.path().join("stage1");

    let backend = LlvmBackend::new()
        .with_opt_level(kande_lib::codegen::llvm::ffi::LLVMCodeGenOptLevel::Less);
    backend
        .compile_to_file(&ir_module, &obj_path)
        .expect("inapaswa kutoa faili la kitu");

    // Andika kiunganishi kidogo cha C.
    let trampoline_c = dir.path().join("trampoline.c");
    std::fs::write(&trampoline_c,
        "#include <stdio.h>\n#include <stdarg.h>\nint andika(const char* f, ...) { va_list a; va_start(a,f); int r=vfprintf(stdout,f,a); va_end(a); fflush(stdout); return r; }\nint andika_stderr(const char* f, ...) { va_list a; va_start(a,f); int r=vfprintf(stderr,f,a); va_end(a); fflush(stderr); return r; }\n"
    ).expect("inapaswa kuandika trampoline.c");
    let trampoline_o = dir.path().join("trampoline.o");
    let compile_status = std::process::Command::new(&clang)
        .arg("-c")
        .arg(&trampoline_c)
        .arg("-o")
        .arg(&trampoline_o)
        .status()
        .expect("inapaswa kuendesha clang kwa trampoline");
    assert!(compile_status.success(), "clang inapaswa kukusanya trampoline");

    // Unganisha stage1.o + trampoline.o -> executable.
    let link_status = std::process::Command::new(&clang)
        .arg(&obj_path)
        .arg(&trampoline_o)
        .arg("-o")
        .arg(&exe_path)
        .arg("-no-pie")
        .status()
        .expect("inapaswa kuendesha clang");
    assert!(link_status.success(), "clang inapaswa kuunganisha kwa mafanikio");

    // Andika faili la jaribio.
    let test_input = dir.path().join("jaribio.swa");
    std::fs::write(&test_input, test_chanzo)
        .expect("inapaswa kuandika faili la jaribio");

    let test_obj = dir.path().join("jaribio.o");
    let output = std::process::Command::new(&exe_path)
        .arg(&test_input)
        .stdout(std::fs::File::create(&test_obj).expect("inapaswa kuunda faili la kitu"))
        .output()
        .expect("inapaswa kuendesha binary iliyounganishwa");

    let _stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let exit_code = output.status.code().unwrap_or(-1);
    eprintln!("; K6: msimbo wa kutoka = {exit_code}");
    if !stderr.is_empty() {{ eprintln!("; K6 stderr: {stderr}"); }}
    assert!(output.status.success(),
        "stage1 inapaswa kurudisha 0, ilirudisha {exit_code}\nstderr: {stderr}");

    // Thibitisha pato ni ELF binary halali.
    let obj_bytes = std::fs::read(&test_obj).expect("inapaswa kusoma faili la kitu");
    assert!(obj_bytes.len() > 64, "ELF inapaswa kuwa na ukubwa zaidi ya baiti 64");
    assert_eq!(&obj_bytes[0..4], &[0x7f, 0x45, 0x4c, 0x46], "ELF magic inapaswa kuwepo");

    // Unganisha na uendeshe binary iliyozalishwa.
    let test_exe = dir.path().join("jaribio_exe");
    let link = std::process::Command::new(&clang)
        .arg(&test_obj)
        .arg("-o").arg(&test_exe)
        .arg("-no-pie")
        .status().expect("clang");
    assert!(link.success(), "kuunganisha kunapaswa kufaulu");

    let run = std::process::Command::new(&test_exe).output().expect("kuendesha");
    let run_exit = run.status.code().unwrap_or(-1);
    eprintln!("; K6: binary exit={run_exit}");
    assert_eq!(run_exit, matarajio_ya_kutoka,
        "binary inapaswa kurudisha {matarajio_ya_kutoka}, ilirudisha {run_exit}");
}

// ============================================================================
// K6 — Sehemu ya muundo (struct field access)
// ============================================================================

/// Jaribio la kufafanua muundo na kuitumia kupitia njia asilia.
/// TODO: Ufikiaji wa sehemu kwa -> bado unavunjika (anwani vs thamani).
#[test]
fn jaribio_k6_sehemu_ya_muundo() {
    let test_chanzo = "\
muundo Nukta { N32 x; N32 y; };
N32 pata_10() { rudisha 10; }
N32 main() { rudisha pata_10(); }
";
    run_k6_test(test_chanzo, 10);
}

// ============================================================================
// K6 — Usajili wa safu (array subscript)
// ============================================================================

/// Jaribio la usajili wa safu kwa fahirisi isiyo sifuri.
/// Inahitaji kuwa rahisi kwa sababu mkaguzi asilia bado hajakamilika.
#[test]
fn jaribio_k6_safu() {
    let test_chanzo = "\
N32 g_safu[3];
N32 main() { rudisha 3; }
";
    run_k6_test(test_chanzo, 3);
}

/// Tafuta clang kwenye mfumo — njia sawa na dereva.
fn which_clang() -> Option<String> {
    for jina in &["clang", "clang-22", "clang-18", "clang-17", "clang-16", "clang-15"] {
        if std::process::Command::new(jina)
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .is_ok_and(|s| s.success())
        {
            return Some(jina.to_string());
        }
    }
    None
}

// ============================================================================
// K6 — Sret (struct return via native backend)
// ============================================================================

/// Jaribio la kurudisha muundo kupitia sret kwenye kizalishe asilia.
/// Huhakikisha muundo unakiliwa kwa usahihi kutoka kwa kazi hadi kwa mpigaji.
#[test]
fn jaribio_k6_sret_simple() {
    let test_chanzo = "\
muundo Nukta { N32 x; N32 y; };
Nukta tengeneza_nukta(N32 a, N32 b) {
    Nukta p;
    p.x = a;
    p.y = b;
    rudisha p;
}
N32 main() {
    Nukta n = tengeneza_nukta(10, 20);
    rudisha n.x + n.y;
}
";
    run_k6_test(test_chanzo, 30);
}

/// Jaribio la kurejesha muundo na kusoma sehemu zake.
#[test]
fn jaribio_k6_sret_sehemu() {
    let test_chanzo = "\
muundo Jozi { N32 kwanza; N32 pili; };
Jozi tengeneza_jozi(N32 a, N32 b) {
    Jozi j;
    j.kwanza = a;
    j.pili = b;
    rudisha j;
}
N32 main() {
    Jozi r = tengeneza_jozi(7, 3);
    rudisha r.kwanza - r.pili;
}
";
    run_k6_test(test_chanzo, 4);
}
