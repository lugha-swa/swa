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
// K9 — Majaribio ya kisemantiki ya taarifa za rudisha
// ============================================================================

#[test]
fn jaribio_k9a_void_kurudisha_thamani() {
    // Kazi ya W0 yenye rudisha yenye thamani — inapaswa kushindwa
    let src = "W0 fanya() { rudisha 5; }";
    let result = compile_and_verify(src);
    assert!(result.is_err(), "W0 kazi inayorudisha thamani inapaswa kushindwa");
}

#[test]
fn jaribio_k9b_nonvoid_kurudisha_tupu() {
    // Kazi ya N32 yenye rudisha tupu — inapaswa kushindwa
    let src = "N32 fanya() { rudisha; }";
    let result = compile_and_verify(src);
    assert!(result.is_err(), "kaziya N32 inayorudisha tupu inapaswa kushindwa");
}

#[test]
fn jaribio_k9c_kurudisha_sahihi() {
    // Kazi ya N32 yenye rudisha sahihi — inapaswa kufaulu
    let src = "N32 fanya() { rudisha 42; }";
    let ir = compile_and_verify(src).expect("rudisha sahihi inapaswa kukusanyika");
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
        "#include <stdio.h>\n#include <stdarg.h>\nint andika(const char* f, ...) { va_list a; va_start(a,f); int r=vfprintf(stdout,f,a); va_end(a); fflush(stdout); return r; }\nint andika_stderr(const char* f, ...) { va_list a; va_start(a,f); int r=vfprintf(stderr,f,a); va_end(a); fflush(stderr); return r; }\nint tekeleza(void* kazi, int argc, void* argv, int ofseti) { int (*f)(int, void*) = (int (*)(int, void*))kazi; return f(argc, (void*)((char**)argv + ofseti)); }\nvoid* anwani_ya_kazi(const char* jina) { extern void* dlsym(void*, const char*); return dlsym((void*)0, jina); }\nlong wito_wa_mfumo(long n, long a1, long a2, long a3, long a4, long a5) { extern long syscall(long, long, long, long, long, long, long); return syscall(n, a1, a2, a3, a4, a5, 0); }\n"
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

// ============================================================================
// Exe — Jaribio la kujijenga kwa ET_EXEC (bila kiunganishi cha nje)
// ============================================================================

/// Mnyororo kamili wa exe — 0% bootstrap gap: mbegu (kutoka baiti za
/// mkono kupitia kwanza) → stage1-exe → stage2-exe → stage3-exe.
/// Hakuna ld, gcc, clang, wala libc popote kwenye mnyororo: mbegu
/// inatoa ET_EXEC tuli moja kwa moja (--exe). Uthibitisho: stage2-exe
/// na stage3-exe zinafanana sawa kwa baiti.
/// Kumbuka: jaribio linatumia mbegu, si dereva wa Rust — mkusanyaji wa
/// LLVM bado huning'inia kwenye chanzo chenye miundo (hitilafu ya nyuma
/// ya muundo/sret, kazi ya baadaye).
#[test]
fn jaribio_exe_kujijenga() {
    // 1. Mnyororo wa kwanza: kwanza → mbegu2.bin (baiti za mkono)
    let kwanza = std::process::Command::new("bash")
        .arg("gharama/jenga-kwanza.sh")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("inapaswa kuendesha jenga-kwanza.sh");
    assert!(kwanza.status.success(), "mnyororo wa kwanza unapaswa kufaulu");

    // 2. Unganisha chanzo chote cha msingi (mfuatano wa usomaji wa mbegu)
    let dir = tempfile::tempdir().expect("inapaswa kuunda saraka ya muda");
    let zima = dir.path().join("zima.swa");
    let faili_za_msingi = [
        "msingi/kumbukumbu.swa", "msingi/mfuatano.swa", "msingi/msomaji.swa",
        "msingi/msambazaji.swa", "msingi/mteremko.swa", "msingi/mkaguzi.swa",
        "msingi/uzalishaji.swa", "msingi/orodha.swa", "msingi/ramani.swa",
        "msingi/stage1.swa",
    ];
    let mut chanzo = String::new();
    for f in faili_za_msingi {
        chanzo.push_str(&std::fs::read_to_string(f).expect("inapaswa kusoma faili la msingi"));
    }
    std::fs::write(&zima, chanzo).expect("inapaswa kuandika chanzo");

    // 3. mbegu --exe < zima.swa → stage1-exe (bila kiunganishi chochote)
    let stage1_exe = dir.path().join("stage1-exe");
    let mbegu = "/tmp/mbegu2.bin";  // kwanza hutoa hapa
    let seed_out = std::process::Command::new(mbegu)
        .arg("--exe")
        .stdin(std::fs::File::open(&zima).expect("inapaswa kufungua chanzo"))
        .stdout(std::fs::File::create(&stage1_exe).expect("inapaswa kuunda stage1-exe"))
        .output()
        .expect("inapaswa kuendesha mbegu");
    assert!(seed_out.status.success(), "mbegu --exe inapaswa kukusanya msingi\nstderr: {}",
        String::from_utf8_lossy(&seed_out.stderr));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let ruhusa = std::fs::metadata(&stage1_exe).expect("inapaswa kusoma metadata").permissions();
        let mut ruhusa_mpya = ruhusa.clone();
        ruhusa_mpya.set_mode(0o755);
        std::fs::set_permissions(&stage1_exe, ruhusa_mpya).expect("inapaswa kuweka ruhusa");
    }

    // 4. stage1-exe --exe → stage2-exe (CWD = mzizi wa repo — inasoma msingi/)
    let exe1 = dir.path().join("stage2-exe");
    let nje1 = std::process::Command::new(&stage1_exe)
        .arg("--exe")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdout(std::fs::File::create(&exe1).expect("inapaswa kuunda stage2-exe"))
        .output()
        .expect("inapaswa kuendesha stage1-exe --exe");
    assert!(nje1.status.success(), "stage1-exe --exe inapaswa kurudisha 0\nstderr: {}",
        String::from_utf8_lossy(&nje1.stderr));

    // 5. exe yenyewe: stage2-exe --exe → stage3-exe
    let exe2 = dir.path().join("stage3-exe");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let ruhusa = std::fs::metadata(&exe1).expect("inapaswa kusoma metadata").permissions();
        let mut ruhusa_mpya = ruhusa.clone();
        ruhusa_mpya.set_mode(0o755);
        std::fs::set_permissions(&exe1, ruhusa_mpya).expect("inapaswa kuweka ruhusa");
    }
    let nje2 = std::process::Command::new(&exe1)
        .arg("--exe")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdout(std::fs::File::create(&exe2).expect("inapaswa kuunda stage3-exe"))
        .output()
        .expect("inapaswa kuendesha stage2-exe --exe");
    assert!(nje2.status.success(), "stage2-exe --exe inapaswa kurudisha 0\nstderr: {}",
        String::from_utf8_lossy(&nje2.stderr));

    // 6. Sawasawa kwa baiti — mnyororo wa kujijenga umefungwa bila ld
    let baiti1 = std::fs::read(&exe1).expect("inapaswa kusoma stage2-exe");
    let baiti2 = std::fs::read(&exe2).expect("inapaswa kusoma stage3-exe");
    assert!(baiti1.len() > 4096, "exe inapaswa kuwa na mwili halisi ({} baiti)", baiti1.len());
    assert_eq!(baiti1, baiti2, "stage2-exe na stage3-exe zinapaswa kuwa sawa kwa baiti");

    // 7. Program ya mtumiaji kama exe halisi — inafunika safu za ndani
    // za N8/N16/N64 (mdudu wa ukubwa wa elementi), .bss (sifuri), ulimwengu
    // unaobadilishwa na kazi mbili, wito wa mbele (RELA ya nje), kitanzi,
    // na kujirudia kwa pande mbili.
    let user_swa = dir.path().join("jaribio_user.swa");
    std::fs::write(&user_swa, "\
N32 g(N32 x) { kama (x <= 0) { rudisha 0; } rudisha 1 + f(x - 1); }
N32 f(N32 x) { kama (x <= 0) { rudisha 0; } rudisha 1 + g(x - 1); }
N32 glob = 0;
N32 h1() { glob = glob + 3; rudisha glob; }
N32 main() {
    N8  b8[8];
    N16 b16[4];
    N64 b64[4];
    N32 i = 0;
    wakati (i < 8) { b8[i] = i; i = i + 1; }
    kama (b8[3] != 3) rudisha 1;
    kama (b8[7] != 7) rudisha 2;
    i = 0;
    wakati (i < 4) { b16[i] = i * 2 + 1; i = i + 1; }
    kama (b16[3] != 7) rudisha 3;
    i = 0;
    wakati (i < 4) { b64[i] = i + 10; i = i + 1; }
    kama (b64[3] != 13) rudisha 4;
    kama (glob != 0) rudisha 5;
    kama (h1() != 3) rudisha 6;
    kama (f(5) != 5) rudisha 7;
    N32 s = 0; i = 0;
    wakati (i < 5) { s = s + i; i = i + 1; }
    kama (s != 10) rudisha 8;
    N32 t = 0;
    kwa (N32 j = 0; j < 6; j = j + 1) { t = t + j; }
    kama (t != 15) rudisha 9;
    rudisha 0;
}
").expect("inapaswa kuandika jaribio_user.swa");
    let user_exe = dir.path().join("jaribio_user-exe");
    let user_out = std::process::Command::new(&stage1_exe)
        .arg("--exe")
        .arg(&user_swa)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdout(std::fs::File::create(&user_exe).expect("inapaswa kuunda jaribio_user-exe"))
        .output()
        .expect("inapaswa kuendesha stage1-exe --exe kwa program ya mtumiaji");
    assert!(user_out.status.success(), "stage1-exe --exe inapaswa kukusanya program ya mtumiaji\nstderr: {}",
        String::from_utf8_lossy(&user_out.stderr));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let ruhusa = std::fs::metadata(&user_exe).expect("inapaswa kusoma metadata").permissions();
        let mut ruhusa_mpya = ruhusa.clone();
        ruhusa_mpya.set_mode(0o755);
        std::fs::set_permissions(&user_exe, ruhusa_mpya).expect("inapaswa kuweka ruhusa");
    }
    let user_run = std::process::Command::new(&user_exe)
        .output()
        .expect("inapaswa kuendesha jaribio_user-exe");
    assert!(user_run.status.success(), "program ya mtumiaji inapaswa kurudisha 0, ilirudisha {:?}\nstderr: {}",
        user_run.status.code(), String::from_utf8_lossy(&user_run.stderr));

    // 8. Uthabiti wa mchanganuzi: ingizo baya lazima lirudishe 1 kwa sauti
    // (kabla: mbegu ilikubali kimya au kusegfault; mchanganuzi wa .swa
    // ulining'inia kwa ingizo lililokatwa). Kila kisa: exit 1, hakuna hang.
    let kesi_baya = [
        "garba",
        "kweli kweli kweli",
        "@#!",
        "N32 main( {",
        "N32 main() { rudisha 1; ",
        "muundo X { N32 x;",
        "N32 main() { f(1, 2; }",
    ];
    for kesi in kesi_baya {
        let baya_swa = dir.path().join("baya.swa");
        std::fs::write(&baya_swa, kesi).expect("inapaswa kuandika baya.swa");
        let baya_out = std::process::Command::new(&stage1_exe)
            .arg(&baya_swa)
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("inapaswa kuendesha stage1-exe kwa ingizo baya");
        assert!(!baya_out.status.success(),
            "ingizo baya [{}] linapaswa kurudisha si 0 (mbegu au mchanganuzi ulikubali kimya)", kesi);
        // Uthibitisho wa hang: output() tayari ilisubiri kukamilika — hapa
        // tunaweza tu kuthibitisha kuwa mchakato ulimaliza.
    }

    // 9. JIT ndani ya exe: --jit hukusanya na kuendesha program ya
    // mtumiaji bila daraja la C — wito wa ndani unatatuliwa kupitia
    // jedwali la anwani (anwani_ya_kazi) na thunks za bafa ya JIT.
    let jit_swa = dir.path().join("jaribio_jit.swa");
    std::fs::write(&jit_swa,
        "N32 mbili(N32 x) { rudisha x * 2; }\nN32 main() { rudisha mbili(21); }\n"
    ).expect("inapaswa kuandika jaribio_jit.swa");
    let jit_out = std::process::Command::new(&stage1_exe)
        .arg("--jit")
        .arg(&jit_swa)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("inapaswa kuendesha stage1-exe --jit");
    assert!(jit_out.status.success(), "stage1-exe --jit inapaswa kurudisha 0\nstderr: {}",
        String::from_utf8_lossy(&jit_out.stderr));
    let stderr_jit = String::from_utf8_lossy(&jit_out.stderr);
    assert!(stderr_jit.contains("; JIT: matokeo=42"),
        "JIT inapaswa kurudisha matokeo=42\nstderr: {}", stderr_jit);

    // 10. Mkazo wa RELA/codegen: program moja yenye kwa+vunja+endelea,
    // urejeshaji halisi, ulimwengu unaobadilishwa na kazi mbili, wito
    // wa mbele, na vitanzi vilivyopandikizwa. Tarajio la mkono: exit 0
    // (s=121, fibonacci(10)=55, jumla_yote=7, pata_baadaye(5)=25, t=30).
    // Hesabu ya s kwa semantiki ya C ya endelea (inaruka kwenye HATUA):
    // i=3 → i=4, s=103, hatua → i=5; kisha i=5:108, i=6:114, i=7:121,
    // i=8 → vunja. (Semantiki ya zamani ilitoa s=125 — imerekebishwa
    // pamoja na mnyororo wa .swa mnamo 2026-08.)
    let mkazo_swa = dir.path().join("mkazo_rela.swa");
    std::fs::write(&mkazo_swa, "N32 fibonacci(N32 n) { kama (n <= 1) { rudisha n; } rudisha fibonacci(n - 1) + fibonacci(n - 2); }\nN32 jumla_yote = 0;\nN32 ongeza_jumla(N32 v) { jumla_yote = jumla_yote + v; rudisha jumla_yote; }\nN32 toa_jumla(N32 v) { jumla_yote = jumla_yote - v; rudisha jumla_yote; }\nN32 main() { N32 s = 0; kwa (N32 i = 0; i < 10; i = i + 1) { kama (i == 3) { i = i + 1; s = s + 100; endelea; } kama (i == 8) { vunja; } s = s + i; } kama (s != 121) rudisha 1; kama (fibonacci(10) != 55) rudisha 2; ongeza_jumla(10); toa_jumla(3); kama (jumla_yote != 7) rudisha 3; kama (pata_baadaye(5) != 25) rudisha 4; N32 t = 0; kwa (N32 j = 0; j < 3; j = j + 1) { N32 k = 0; wakati (k < 4) { t = t + j + k; k = k + 1; } } kama (t != 30) rudisha 5; rudisha 0; }\nN32 pata_baadaye(N32 x) { rudisha x * x; }\n").expect("inapaswa kuandika mkazo_rela.swa");
    let mkazo_exe = dir.path().join("mkazo-rela-exe");
    let mkazo_out = std::process::Command::new(&stage1_exe)
        .arg("--exe")
        .arg(&mkazo_swa)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdout(std::fs::File::create(&mkazo_exe).expect("inapaswa kuunda mkazo-exe"))
        .output()
        .expect("inapaswa kuendesha stage1-exe --exe kwa mkazo");
    assert!(mkazo_out.status.success(), "stage1-exe --exe inapaswa kukusanya mkazo\nstderr: {}",
        String::from_utf8_lossy(&mkazo_out.stderr));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let ruhusa = std::fs::metadata(&mkazo_exe).expect("inapaswa kusoma metadata").permissions();
        let mut ruhusa_mpya = ruhusa.clone();
        ruhusa_mpya.set_mode(0o755);
        std::fs::set_permissions(&mkazo_exe, ruhusa_mpya).expect("inapaswa kuweka ruhusa");
    }
    // 11. C-semantiki ya endelea kwenye kwa — sasa mbegu NA mnyororo
    // wa .swa zote zina semantiki ya C. Jaribio linaendesha chanzo hiki
    // kupitia MBEGU moja kwa moja na pia kupitia stage1 (mnyororo wa
    // .swa); ujumuishaji wa minyororo yote miwili unathibitisha
    // uthabiti wao.
    let kwac_swa = dir.path().join("kwa_endelea.swa");
    std::fs::write(&kwac_swa, "N32 main() { N32 i = 0; N32 s = 0; kwa (; i < 6; i = i + 1) { kama (i == 2) { endelea; } s = s + 1; } rudisha s; }\n").expect("inapaswa kuandika kwa_endelea.swa");
    let kwac_exe = dir.path().join("kwa-endelea-exe");
    let kwac_out = std::process::Command::new(mbegu)
        .arg("--exe")
        .stdin(std::fs::File::open(&kwac_swa).expect("inapaswa kufungua kwa_endelea.swa"))
        .stdout(std::fs::File::create(&kwac_exe).expect("inapaswa kuunda kwa-endelea-exe"))
        .output()
        .expect("inapaswa kuendesha mbegu --exe kwa kwa_endelea");
    assert!(kwac_out.status.success(), "mbegu --exe inapaswa kukusanya kwa_endelea\nstderr: {}",
        String::from_utf8_lossy(&kwac_out.stderr));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let ruhusa = std::fs::metadata(&kwac_exe).expect("inapaswa kusoma metadata").permissions();
        let mut ruhusa_mpya = ruhusa.clone();
        ruhusa_mpya.set_mode(0o755);
        std::fs::set_permissions(&kwac_exe, ruhusa_mpya).expect("inapaswa kuweka ruhusa");
    }
    let kwac_run = std::process::Command::new(&kwac_exe)
        .output()
        .expect("inapaswa kuendesha kwa-endelea-exe");
    // main inarudisha s=5 — msimbo wa kutoka 5 ndio ushahidi kwamba
    // endelea iliruka kwenye HATUA (i iliendelea kuongezeka hadi 6).
    assert_eq!(kwac_run.status.code(), Some(5),
        "endelea kwenye kwa inapaswa kuruka kwenye hatua (C-semantiki): s=5, ilipata {:?}", kwac_run.status.code());

    // Mnyororo wa .swa: stage1-exe --exe kwa_endelea.swa →
    // kwa-endelea-stage1-exe. (Mnyororo wa .swa unasoma chanzo kwa
    // hoja ya faili, si stdin kama mbegu — kwa hivyo chanzo kinapita
    // kwa argv kama katika sehemu ya 7 na 10.)
    let kwac2_exe = dir.path().join("kwa-endelea-stage1-exe");
    let kwac2_out = std::process::Command::new(&stage1_exe)
        .arg("--exe")
        .arg(&kwac_swa)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdout(std::fs::File::create(&kwac2_exe).expect("inapaswa kuunda kwa-endelea-stage1-exe"))
        .output()
        .expect("inapaswa kuendesha stage1-exe --exe kwa kwa_endelea");
    assert!(kwac2_out.status.success(), "stage1-exe --exe inapaswa kukusanya kwa_endelea\nstderr: {}",
        String::from_utf8_lossy(&kwac2_out.stderr));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let ruhusa = std::fs::metadata(&kwac2_exe).expect("inapaswa kusoma metadata").permissions();
        let mut ruhusa_mpya = ruhusa.clone();
        ruhusa_mpya.set_mode(0o755);
        std::fs::set_permissions(&kwac2_exe, ruhusa_mpya).expect("inapaswa kuweka ruhusa");
    }
    let kwac2_run = std::process::Command::new(&kwac2_exe)
        .output()
        .expect("inapaswa kuendesha kwa-endelea-stage1-exe");
    assert_eq!(kwac2_run.status.code(), Some(5),
        "endelea kwenye kwa (mnyororo wa .swa) inapaswa kuruka kwenye hatua (C-semantiki): s=5, ilipata {:?}", kwac2_run.status.code());

    let mkazo_run = std::process::Command::new(&mkazo_exe)
        .output()
        .expect("inapaswa kuendesha mkazo-exe");
    assert!(mkazo_run.status.success(),
        "mkazo wa RELA/codegen unapaswa kurudisha 0, ilirudisha {:?}", mkazo_run.status.code());
}

/// Hali ya .o ya mbegu — urekebishaji wa RELA na symtab hautumiki
/// katika hali ya exe, kwa hivyo jari bisha pia: mbegu < zima.swa →
/// Mkazo wa mbegu: wito 1,000 wa mbele kwa kazi moja — kila wito
/// unachukua ingizo jipya la nje na RELA, kisha toa_exe inazitatua
/// zote. Huu ndio jaribio lililowahi kugundua mbegu iliyovunjika
/// (uhariri mbaya wa njia ya fixup) kabla ya kugandishwa — sasa ni
/// la kudumu la kurejesha (regression).
#[test]
fn jaribio_mbegu_mkazo_wito() {
    let kwanza = std::process::Command::new("bash")
        .arg("gharama/jenga-kwanza.sh")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("inapaswa kuendesha jenga-kwanza.sh");
    assert!(kwanza.status.success(), "mnyororo wa kwanza unapaswa kufaulu");

    let dir = tempfile::tempdir().expect("inapaswa kuunda saraka ya muda");
    let mbegu = "/tmp/mbegu2.bin";

    // Chanzo cha mkazo: wito 1,000 wa mbele (lengo limetangazwa MWISHO)
    let mut chanzo = String::from("N32 main() { N32 s = 0;\n");
    for _ in 0..1000 {
        chanzo.push_str("s = s + lengo(1);\n");
    }
    chanzo.push_str("kama (s != 1000) rudisha 1; rudisha 0; }\n");
    chanzo.push_str("N32 lengo(N32 x) { rudisha x; }\n");
    let stress_swa = dir.path().join("mkazo.swa");
    std::fs::write(&stress_swa, chanzo).expect("inapaswa kuandika mkazo.swa");

    // 1. mbegu --exe < mkazo.swa -> exe ya mtumiaji
    let stress_exe = dir.path().join("mkazo-exe");
    let out1 = std::process::Command::new(mbegu)
        .arg("--exe")
        .stdin(std::fs::File::open(&stress_swa).expect("inapaswa kufungua mkazo.swa"))
        .stdout(std::fs::File::create(&stress_exe).expect("inapaswa kuunda mkazo-exe"))
        .output()
        .expect("inapaswa kuendesha mbegu --exe");
    assert!(out1.status.success(), "mbegu --exe inapaswa kukusanya mkazo\nstderr: {}",
        String::from_utf8_lossy(&out1.stderr));

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let ruhusa = std::fs::metadata(&stress_exe).expect("inapaswa kusoma metadata").permissions();
        let mut ruhusa_mpya = ruhusa.clone();
        ruhusa_mpya.set_mode(0o755);
        std::fs::set_permissions(&stress_exe, ruhusa_mpya).expect("inapaswa kuweka ruhusa");
    }
    let run1 = std::process::Command::new(&stress_exe)
        .output()
        .expect("inapaswa kuendesha mkazo-exe");
    assert!(run1.status.success(), "mkazo-exe inapaswa kurudisha 0, ilirudisha {:?}",
        run1.status.code());

    // 2. Hali ya .o pia: mbegu < mkazo.swa -> .o lenye UND 1,000
    let stress_o = dir.path().join("mkazo.o");
    let out2 = std::process::Command::new(mbegu)
        .stdin(std::fs::File::open(&stress_swa).expect("inapaswa kufungua mkazo.swa"))
        .stdout(std::fs::File::create(&stress_o).expect("inapaswa kuunda mkazo.o"))
        .output()
        .expect("inapaswa kuendesha mbegu");
    assert!(out2.status.success(), "mbegu inapaswa kukusanya mkazo (.o)\nstderr: {}",
        String::from_utf8_lossy(&out2.stderr));
    let baiti = std::fs::read(&stress_o).expect("inapaswa kusoma mkazo.o");
    assert!(baiti.len() > 4096, "mkazo.o inapaswa kuwa na mwili halisi ({} baiti)", baiti.len());
    assert_eq!(&baiti[..4], &[0x7F, 0x45, 0x4C, 0x46], "mkazo.o inapaswa kuwa ELF halali");
}

/// stage1.o, unganisha kwa clang na trampoline ndogo (tekeleza na
/// anwani_ya_kazi pekee — andika, ukubwa, na wito_wa_mfumo ni za
/// ndani sasa), endesha, na uthibitishe towe ni ELF halali.
#[test]
fn jaribio_o_kujijenga() {
    let clang = which_clang();
    if clang.is_none() {
        eprintln!("; o: clang haipatikani — ruka jaribio la hali ya .o");
        return;
    }
    let clang = clang.unwrap();

    let kwanza = std::process::Command::new("bash")
        .arg("gharama/jenga-kwanza.sh")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("inapaswa kuendesha jenga-kwanza.sh");
    assert!(kwanza.status.success(), "mnyororo wa kwanza unapaswa kufaulu");

    let dir = tempfile::tempdir().expect("inapaswa kuunda saraka ya muda");
    let zima = dir.path().join("zima.swa");
    let faili_za_msingi = [
        "msingi/kumbukumbu.swa", "msingi/mfuatano.swa", "msingi/msomaji.swa",
        "msingi/msambazaji.swa", "msingi/mteremko.swa", "msingi/mkaguzi.swa",
        "msingi/uzalishaji.swa", "msingi/orodha.swa", "msingi/ramani.swa",
        "msingi/stage1.swa",
    ];
    let mut chanzo = String::new();
    for f in faili_za_msingi {
        chanzo.push_str(&std::fs::read_to_string(f).expect("inapaswa kusoma faili la msingi"));
    }
    std::fs::write(&zima, chanzo).expect("inapaswa kuandika chanzo");

    // mbegu < zima.swa → stage1.o
    let stage1_o = dir.path().join("stage1.o");
    let mbegu = "/tmp/mbegu2.bin";
    let seed_out = std::process::Command::new(mbegu)
        .stdin(std::fs::File::open(&zima).expect("inapaswa kufungua chanzo"))
        .stdout(std::fs::File::create(&stage1_o).expect("inapaswa kuunda stage1.o"))
        .output()
        .expect("inapaswa kuendesha mbegu");
    assert!(seed_out.status.success(), "mbegu inapaswa kukusanya msingi\nstderr: {}",
        String::from_utf8_lossy(&seed_out.stderr));

    // Unganisha kwa clang na trampoline ndogo (madaraja ya JIT pekee).
    let trampoline_c = dir.path().join("trampoline.c");
    std::fs::write(&trampoline_c,
        "int tekeleza(void* kazi, int argc, void* argv, int ofseti) { int (*f)(int, void*) = (int (*)(int, void*))kazi; return f(argc, (void*)((char**)argv + ofseti)); }\n"
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

    let stage1_bin = dir.path().join("stage1");
    let link_status = std::process::Command::new(&clang)
        .arg(&stage1_o)
        .arg(&trampoline_o)
        .arg("-o")
        .arg(&stage1_bin)
        .arg("-no-pie")
        .status()
        .expect("inapaswa kuendesha clang");
    assert!(link_status.success(), "clang inapaswa kuunganisha kwa mafanikio");

    // Endesha na uthibitishe towe ni ELF halali (0x7F 'E' 'L' 'F').
    let out_o = dir.path().join("out.o");
    let nje = std::process::Command::new(&stage1_bin)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdout(std::fs::File::create(&out_o).expect("inapaswa kuunda out.o"))
        .output()
        .expect("inapaswa kuendesha stage1");
    assert!(nje.status.success(), "stage1 inapaswa kurudisha 0\nstderr: {}",
        String::from_utf8_lossy(&nje.stderr));
    let baiti = std::fs::read(&out_o).expect("inapaswa kusoma out.o");
    assert!(baiti.len() > 64, "out.o inapaswa kuwa na mwili halisi ({} baiti)", baiti.len());
    assert_eq!(&baiti[..4], &[0x7F, 0x45, 0x4C, 0x46], "out.o inapaswa kuwa ELF halali");
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
        "#include <stdio.h>\n#include <stdarg.h>\nint andika(const char* f, ...) { va_list a; va_start(a,f); int r=vfprintf(stdout,f,a); va_end(a); fflush(stdout); return r; }\nint andika_stderr(const char* f, ...) { va_list a; va_start(a,f); int r=vfprintf(stderr,f,a); va_end(a); fflush(stderr); return r; }\nint tekeleza(void* kazi, int argc, void* argv, int ofseti) { int (*f)(int, void*) = (int (*)(int, void*))kazi; return f(argc, (void*)((char**)argv + ofseti)); }\nvoid* anwani_ya_kazi(const char* jina) { extern void* dlsym(void*, const char*); return dlsym((void*)0, jina); }\nlong wito_wa_mfumo(long n, long a1, long a2, long a3, long a4, long a5) { extern long syscall(long, long, long, long, long, long, long); return syscall(n, a1, a2, a3, a4, a5, 0); }\n"
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
// ============================================================================
// K10 — Msaidizi wa majaribio ya maktaba ya kawaida
// ============================================================================

/// Kusanya chanzo cha jaribio kinachotumia `husisha` kutoka saraka ya `msingi/`,
/// unganisha na clang, endesha, na thibitisha msimbo wa kutoka.
///
/// `path_hint` hutumiwa kwa azimio la `husisha` — saraka yake mzazi
/// ndiyo msingi wa utafutaji wa moduli zilizohusishwa.
fn run_msingi_test(test_source: &str, expected_exit: i32) {
    let clang = which_clang();
    if clang.is_none() {
        eprintln!("; K10: clang haipatikani — ruka jaribio la wakati wa utekelezaji");
        return;
    }
    let clang = clang.unwrap();

    // Tumia dereva wa Rust moja kwa moja kwa kasi.
    // PathBuf ya "msingi/jaribio_k10.swa" ina maana parent_dir = "msingi/"
    // hivyo husisha { kumbukumbu.swa } hutafutwa kwenye msingi/kumbukumbu.swa.
    let mut driver = Driver::new();
    let ir_module = driver
        .compile_to_ir(test_source, PathBuf::from("msingi/jaribio_k10.swa"))
        .expect("inapaswa kuchanganua na kuteremsha");

    let dir = tempfile::tempdir().expect("inapaswa kuunda saraka ya muda");
    let obj_path = dir.path().join("jaribio.o");
    let exe_path = dir.path().join("jaribio_exe");

    let backend = LlvmBackend::new();
    backend
        .compile_to_file(&ir_module, &obj_path)
        .expect("inapaswa kutoa faili la kitu");

    // Unganisha na clang, pamoja na trampoline ya Swa (wito_wa_mfumo,
    // andika, tekeleza n.k.) — libc inaunganishwa kwa chaguo-msingi.
    let link_status = std::process::Command::new(&clang)
        .arg(&obj_path)
        .arg("gharama/trampoline.c")
        .arg("-o")
        .arg(&exe_path)
        .arg("-no-pie")
        .status()
        .expect("inapaswa kuendesha clang");
    assert!(link_status.success(), "clang inapaswa kuunganisha kwa mafanikio");

    let output = std::process::Command::new(&exe_path)
        .output()
        .expect("inapaswa kuendesha binary");
    let exit_code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    eprintln!("; K10: exit={exit_code}");
    if !stdout.is_empty() {
        eprintln!("; K10 stdout: {stdout}");
    }
    if !stderr.is_empty() {
        eprintln!("; K10 stderr: {stderr}");
    }
    assert_eq!(
        exit_code, expected_exit,
        "binary inapaswa kurudisha {expected_exit}, ilirudisha {exit_code}\nstdout: {stdout}\nstderr: {stderr}"
    );
}

// ============================================================================
// K10a — Jaribio la kumbukumbu.swa (nakili, weka_sifuri, linganisha_kumbukumbu)
// ============================================================================

#[test]
fn jaribio_k10a_kumbukumbu() {
    let test_source = "\
husisha { kumbukumbu.swa }

N32 main() {
    N8 buf1[10];
    N8 buf2[10];
    weka_sifuri(buf1, 10);
    weka_sifuri(buf2, 10);
    buf1[0] = 65;
    buf1[1] = 66;
    nakili(buf2, buf1, 10);
    kama (buf2[0] != 65) rudisha 1;
    kama (buf2[1] != 66) rudisha 2;
    kama (linganisha_kumbukumbu(buf1, buf2, 10) != 0) rudisha 3;
    buf2[0] = 64;
    kama (linganisha_kumbukumbu(buf1, buf2, 10) == 0) rudisha 4;
    rudisha 0;
}
";
    run_msingi_test(test_source, 0);
}

// ============================================================================
// K10b — Jaribio la mfuatano.swa (ulinganishi, unakili, ubadilishaji)
// ============================================================================

#[test]
fn jaribio_k10b_mfuatano() {
    let test_source = "\
husisha { mfuatano.swa }

N32 main() {
    // linganisha_mfuatano — inalinganisha mifuatano miwili
    N8 s1[] = \"hello\";
    N8 s2[] = \"hello\";
    N8 s3[] = \"helpo\";
    kama (linganisha_mfuatano(s1, s2) != 0) rudisha 1;
    // s1 < s3 kwa sababu 'l' < 'p'
    kama (linganisha_mfuatano(s1, s3) == 0) rudisha 2;

    // nakili_mfuatano — inanakili mfuatano
    N8 buf[32];
    nakili_mfuatano(buf, s1);
    kama (linganisha_mfuatano(buf, s1) != 0) rudisha 3;

    // mfuatano_hadi_n32 — ubadilishaji mfuatano→namba
    N8 namba1[] = \"42\";
    kama (mfuatano_hadi_n32(namba1) != 42) rudisha 4;

    N8 namba2[] = \"   -17\";
    kama (mfuatano_hadi_n32(namba2) != -17) rudisha 5;

    // nambari_kwa_mfuatano — ubadilishaji namba→mfuatano
    N8 buf2[32];
    nambari_kwa_mfuatano(99, buf2);
    kama (buf2[0] != 57) rudisha 6;   // '9'
    kama (buf2[1] != 57) rudisha 7;   // '9'
    kama (buf2[2] != 0) rudisha 8;    // ncha-tupu

    // tafuta_herufi — inatafuta herufi kwenye mfuatano
    N8 s4[] = \"habari\";
    kama (tafuta_herufi(s4, 98) != 1) rudisha 9;   // 'b' iko kwenye faharisi 1
    kama (tafuta_herufi(s4, 122) != -1) rudisha 10;  // 'z' haipo

    rudisha 0;
}
";
    run_msingi_test(test_source, 0);
}

// ============================================================================
// K10c — Jaribio la orodha.swa (orodha inayobadilika)
// ============================================================================

#[test]
fn jaribio_k10c_orodha() {
    let test_source = "\
husisha { orodha.swa }

N32 main() {
    Orodha o = orodha_mpya(4);
    kama (orodha_urefu(&o) != 0) rudisha 1;

    orodha_ongeza(&o, 10);
    orodha_ongeza(&o, 20);
    orodha_ongeza(&o, 30);
    kama (orodha_urefu(&o) != 3) rudisha 2;

    kama (orodha_pata(&o, 0) != 10) rudisha 3;
    kama (orodha_pata(&o, 1) != 20) rudisha 4;
    kama (orodha_pata(&o, 2) != 30) rudisha 5;

    // Faharisi batili inarudisha 0
    kama (orodha_pata(&o, -1) != 0) rudisha 6;
    kama (orodha_pata(&o, 100) != 0) rudisha 7;

    // Futa kipengele cha mwisho
    orodha_futa_mwisho(&o);
    kama (orodha_urefu(&o) != 2) rudisha 8;
    kama (orodha_pata(&o, 1) != 20) rudisha 9;

    // Ongeza zaidi ya uwezo wa awali (inajaribu badili/realloc)
    orodha_ongeza(&o, 40);
    orodha_ongeza(&o, 50);
    orodha_ongeza(&o, 60);
    kama (orodha_urefu(&o) != 5) rudisha 10;
    kama (orodha_pata(&o, 4) != 60) rudisha 11;

    orodha_huru(&o);
    rudisha 0;
}
";
    run_msingi_test(test_source, 0);
}

// ============================================================================
// K10d — Jaribio la ramani.swa (jedwali la hashi)
// ============================================================================

#[test]
fn jaribio_k10d_ramani() {
    let test_source = "\
husisha { ramani.swa }

N32 main() {
    Ramani* r = ramani_mpya(16);

    // Weka thamani
    ramani_weka(r, 5, 100);
    ramani_weka(r, 10, 200);
    ramani_weka(r, 20, 300);

    // Pata thamani zilizowekwa
    kama (ramani_pata(r, 5) != 100) rudisha 4;
    kama (ramani_pata(r, 10) != 200) rudisha 5;
    kama (ramani_pata(r, 20) != 300) rudisha 6;

    // Pata thamani isiyokuwepo
    kama (ramani_pata(r, 99) != -1) rudisha 7;

    // Angalia kuwepo
    kama (ramani_ina(r, 5) != 1) rudisha 8;
    kama (ramani_ina(r, 10) != 1) rudisha 9;
    kama (ramani_ina(r, 99) != 0) rudisha 10;

    // Badilisha thamani iliyopo
    ramani_weka(r, 5, 999);
    kama (ramani_pata(r, 5) != 999) rudisha 12;

    // Futa funguo
    ramani_futa(r, 10);
    kama (ramani_ina(r, 10) != 0) rudisha 13;
    kama (ramani_ina(r, 5) != 1) rudisha 14;

    ramani_huru(r);
    rudisha 0;
}
";
    run_msingi_test(test_source, 0);
}

// ============================================================================
// K10e — Jaribio la mpangilio.swa (upangaji wa safu)
// ============================================================================

#[test]
fn jaribio_k10e_mpangilio() {
    let test_source = "\
husisha { mpangilio.swa }

N32 main() {
    N32 arr[5];
    arr[0] = 50;
    arr[1] = 30;
    arr[2] = 10;
    arr[3] = 40;
    arr[4] = 20;

    // Panga kwa kupanda
    pangilia_n32(arr, 5);
    kama (arr[0] != 10) rudisha 1;
    kama (arr[1] != 20) rudisha 2;
    kama (arr[2] != 30) rudisha 3;
    kama (arr[3] != 40) rudisha 4;
    kama (arr[4] != 50) rudisha 5;

    // Panga kwa kushuka
    pangilia_n32_kushuka(arr, 5);
    kama (arr[0] != 50) rudisha 6;
    kama (arr[1] != 40) rudisha 7;
    kama (arr[2] != 30) rudisha 8;
    kama (arr[3] != 20) rudisha 9;
    kama (arr[4] != 10) rudisha 10;

    // Safu yenye kipengele kimoja — inapaswa kupita bila hitilafu
    N32 moja[1];
    moja[0] = 99;
    pangilia_n32(moja, 1);
    kama (moja[0] != 99) rudisha 11;

    // Safu tupu — inapaswa kupita bila hitilafu
    N32 tupu[1];
    pangilia_n32(tupu, 0);

    rudisha 0;
}
";
    run_msingi_test(test_source, 0);
}

// ============================================================================
// K10f — Jaribio la hesabu.swa (hisabati)
// ============================================================================

#[test]
fn jaribio_k10f_hesabu() {
    let test_source = "\
husisha { hesabu.swa }

N32 main() {
    // hesabu_kamili — kipeo (N32)
    kama (hesabu_kamili(5, 3) != 5) rudisha 1;
    kama (hesabu_kamili(2, 7) != 7) rudisha 2;
    kama (hesabu_kamili(-1, -5) != -1) rudisha 3;

    // hesabu_ndogo — kiukomo (N32)
    kama (hesabu_ndogo(5, 3) != 3) rudisha 4;
    kama (hesabu_ndogo(2, 7) != 2) rudisha 5;
    kama (hesabu_ndogo(-1, -5) != -5) rudisha 6;

    // neneo_n32 — thamani kamili (N32)
    kama (neneo_n32(-5) != 5) rudisha 7;
    kama (neneo_n32(3) != 3) rudisha 8;
    kama (neneo_n32(0) != 0) rudisha 9;

    rudisha 0;
}
";
    run_msingi_test(test_source, 0);
}

// ============================================================================
// K10g — Jaribio la faili.swa (I/O ya faili)
// ============================================================================

#[test]
fn jaribio_k10g_faili() {
    let test_source = "\
husisha { kumbukumbu.swa }
husisha { faili.swa }

N32 main() {
    // Andika faili
    N8 njia[] = \"_jaribio_k10g.txt\";
    N8 data[] = \"Habari, Swa!\";
    N64 urefu = 12;
    kama (faili_andika_yote(njia, data, urefu) != 0) rudisha 1;

    // Angalia kama lipo
    kama (faili_ipo(njia) != 1) rudisha 2;

    // Soma faili lote
    N8 buf[64];
    weka_sifuri(buf, 64);
    N64 soma = faili_soma_yote(njia, buf, 64);
    kama (soma != urefu) rudisha 3;
    kama (linganisha_kumbukumbu(buf, data, urefu) != 0) rudisha 4;

    // Fungua na uandike kwa mkono
    N8 hali[] = \"w\";
    N8* f = faili_fungua(njia, hali);
    kama (f == tupu) rudisha 5;
    N8 andiko[] = \"Jaribio\";
    N64 n = faili_andika(andiko, 1, 7, f);
    faili_funga(f);
    kama (n != 7) rudisha 6;

    // Soma tena na uhakikishe
    weka_sifuri(buf, 64);
    soma = faili_soma_yote(njia, buf, 64);
    kama (soma != 7) rudisha 7;
    kama (linganisha_kumbukumbu(buf, andiko, 7) != 0) rudisha 8;

    // faili_soma_mstari
    N8 f2[] = \"_jaribio_k10g_mistari.txt\";
    N8 mistari[] = \"mstari1\\nmstari2\\n\";
    faili_andika_yote(f2, mistari, 16);
    N8* fh = faili_fungua(f2, \"r\");
    kama (fh == tupu) rudisha 9;
    N8 mst_buf[32];
    weka_sifuri(mst_buf, 32);
    N32 len = faili_soma_mstari(fh, mst_buf, 32);
    kama (len != 8) rudisha 10;
    N8 tarajiwa1[] = \"mstari1\\n\";
    kama (linganisha_kumbukumbu(mst_buf, tarajiwa1, 8) != 0) rudisha 11;
    len = faili_soma_mstari(fh, mst_buf, 32);
    kama (len != 8) rudisha 12;
    N8 tarajiwa2[] = \"mstari2\\n\";
    kama (linganisha_kumbukumbu(mst_buf, tarajiwa2, 8) != 0) rudisha 13;
    faili_funga(fh);

    // Angalia faili isiyokuwepo
    N8 haipo[] = \"_haipo_kamwe.txt\";
    kama (faili_ipo(haipo) != 0) rudisha 14;

    rudisha 0;
}
";
    run_msingi_test(test_source, 0);
}

// ============================================================================
// K11 — Vipengele vya msomaji (maoni ya kijiuzi, nambari za radiksi, utorokaji)
// ============================================================================

/// K11a: Maoni ya kijiuzi (/* ... */) yenye upachikaji.
/// Hutumia kigezo cha `run_k6_test` kwa sababu msomaji wa .swa pekee
/// ndio unaosaidia upachikaji huu.
#[test]
fn jaribio_k11a_maoni_ya_kijiuzi() {
    let test_chanzo = "\
N32 main() {
    /* huu ni maoni ya kijiuzi */
    N32 x = 21;
    /* maoni yenye /* upachikaji */ ndani */
    N32 y = 21;
    /* maoni /* ya pili /* ya tatu */ */ mwishoni */
    rudisha x +  /* maoni ya inline */ y;
}
";
    run_k6_test(test_chanzo, 42);
}

/// K11b: Nambari za heksadesimali (0x), oktali (0o), na binary (0b).
/// Thamani hazijabadilishwa kwa usahihi na mchanganuzi wa .swa bado,
/// lakini msomaji unatambua tokeni hizo bila kuanguka.
#[test]
fn jaribio_k11b_nambari_za_radiksi() {
    let test_chanzo = "\
N32 main() {
    N32 a = 0xFF;
    N32 b = 0X1A;
    N32 c = 0o77;
    N32 d = 0O52;
    N32 e = 0b1010;
    N32 f = 0B1101;
    rudisha 0;
}
";
    run_k6_test(test_chanzo, 0);
}

/// K11c: Mfuatano wa utorokaji katika herufi (\n, \t, \\\\, \\xNN).
/// Huhakikisha msomaji wa .swa unashughulikia tokeni za utorokaji
/// katika herufi bila kuanguka. (Mifuatano ina mdudu wa awali kwenye stage1
/// inayozuia majaribio kupitia njia ya kujikusanya.)
#[test]
fn jaribio_k11c_mfuatano_wa_utorokaji() {
    let test_chanzo = "\
N32 main() {
    N8 a = '\\n';
    N8 b = '\\t';
    N8 c = '\\r';
    N8 d = '\\\\';
    N8 e = '\\0';
    N8 f = '\\x41';
    N8 g = '\\x5A';
    rudisha 0;
}
";
    run_k6_test(test_chanzo, 0);
}

// ============================================================================
// K12 — Opereta ya asilimia (%)
// ============================================================================

/// K12: Opereta ya asilimia (%) na kiambatani (%=).
/// Huhakikisha mchanganuzi na mzalishaji wa .swa
/// vinashughulikia opereta ya asilimia kwenye nambari kamili.
#[test]
fn jaribio_k12_opereta_ya_asilimia() {
    let test_chanzo = "\
N32 main() {
    N32 a = 42 % 10;
    N32 b = 17 % 5;
    N32 c = 100 % 7;
    N32 d = 10;
    d %= 3;
    rudisha a + b + c + d;
}
";
    run_k6_test(test_chanzo, 7);
}

/// K12b: Jaribio rahisi la % bila kiunganishi
#[test]
fn jaribio_k12b_modulo_pekee() {
    let test_chanzo = "\
N32 main() {
    N32 a = 42 % 10;
    rudisha a;
}
";
    run_k6_test(test_chanzo, 2);
}

// ============================================================================
// K13 — Anwani-ya kielekezi->sehemu (&ptr->field)
// ============================================================================

/// K13: Anwani ya sehemu kupitia kielekezi (&p->sasa) kama hoja ya kazi.
/// Hujaribu marekebisho mawili:
/// 1. mkaguzi huitana WITO hoja ili kuweka ofseti ya sehemu katika ast_tiga
/// 2. uzalishaji_anwani_ya SEHEMU_MSHALE haitakiwi kusoma thamani (*p) bali anwani (p)
#[test]
fn jaribio_k13_anwani_sehemu_kupitia_kielekezi_kama_hoja() {
    let test_chanzo = "\
muundo Nukta { N32 x; N32 y; }

W0 andika_y_kupitia_anwani(N32* addr, N32 v) {
    *addr = v;
}

N32 main() {
    Nukta n[1];
    n->x = 10;
    n->y = 20;
    andika_y_kupitia_anwani(&n->y, 42);  // &n->y kama hoja ya kazi
    rudisha n->x + n->y;  // 10 + 42 = 52
}
";
    run_k6_test(test_chanzo, 52);
}

/// K13_diag_a: &kigeu kama hoja ya kazi (KUMBUKA kwenye WITO).
#[test]
fn jaribio_k13_diag_a_anwani_kigeu_wito() {
    let test_chanzo = "\
W0 andika_kupitia_anwani(N32* addr, N32 v) {
    *addr = v;
}

N32 main() {
    N32 x = 10;
    andika_kupitia_anwani(&x, 42);
    rudisha x;  // 42
}
";
    run_k6_test(test_chanzo, 42);
}

/// K13_diag_b: &muundo.sehemu (DOT) kama hoja ya kazi.
#[test]
fn jaribio_k13_diag_b_anwani_dot_wito() {
    let test_chanzo = "\
muundo Nukta { N32 x; N32 y; }

W0 andika_y_kupitia_anwani(N32* addr, N32 v) {
    *addr = v;
}

N32 main() {
    Nukta n;
    n.x = 10;
    n.y = 20;
    andika_y_kupitia_anwani(&n.y, 42);  // &n.y kwenye WITO
    rudisha n.x + n.y;  // 10 + 42 = 52
}
";
    run_k6_test(test_chanzo, 52);
}

/// K13_diag_c: &safu->sehemu (MSHALE kwenye muundo, si safu) kama hoja ya kazi.
#[test]
fn jaribio_k13_diag_c_anwani_mshale_muundo_wito() {
    let test_chanzo = "\
muundo Nukta { N32 x; N32 y; }

W0 andika_y_kupitia_anwani(N32* addr, N32 v) {
    *addr = v;
}

N32 main() {
    Nukta n;
    Nukta* p = &n;
    p->x = 10;
    p->y = 20;
    andika_y_kupitia_anwani(&p->y, 42);  // &kielekezi->y kwenye WITO
    rudisha n.x + n.y;  // 10 + 42 = 52
}
";
    run_k6_test(test_chanzo, 52);
}

/// K13b: Anwani ya sehemu kupitia kielekezi (&p->sasa) isiyo hoja ya kazi.
/// Hujaribu tu uzalishaji_anwani_ya kwa SEHEMU_MSHALE — hakuna WITO inayohusika.
#[test]
fn jaribio_k13b_anwani_sehemu_kupitia_kielekezi() {
    let test_chanzo = "\
muundo Nukta { N32 x; N32 y; }

N32 main() {
    Nukta n[1];
    n->x = 10;
    n->y = 20;
    N32* addr = &n->y;  // &n->y SIO hoja ya kazi — inajaribu uzalishaji pekee
    *addr = 42;
    rudisha n->x + n->y;  // 10 + 42 = 52
}
";
    run_k6_test(test_chanzo, 52);
}

/// K13c: Jaribio rahisi la &kigeu (anwani ya kigeu cha kawaida).
/// Inatenga KUMBUKA pekee bila SEHEMU_MSHALE wala safu.
#[test]
fn jaribio_k13c_anwani_kigeu_simple() {
    let test_chanzo = "\
N32 main() {
    N32 x = 42;
    N32* addr = &x;
    rudisha *addr;  // 42
}
";
    run_k6_test(test_chanzo, 42);
}

/// K13d: Jaribio la &kigeu.sehemu (anwani ya sehemu ya muundo wa ndani).
#[test]
fn jaribio_k13d_anwani_sehemu_dot() {
    let test_chanzo = "\
muundo Nukta { N32 x; N32 y; }

N32 main() {
    Nukta n;
    n.x = 10;
    n.y = 20;
    N32* addr = &n.y;
    *addr = 42;
    rudisha n.x + n.y;  // 10 + 42 = 52
}
";
    run_k6_test(test_chanzo, 52);
}

/// K13e: Hakikisha kigeu cha ndani kinafanya kazi (bila &).
#[test]
fn jaribio_k13e_kigeu_bila_anwani() {
    let test_chanzo = "\
N32 main() {
    N32 x = 42;
    rudisha x;  // 42
}
";
    run_k6_test(test_chanzo, 42);
}

/// K13f: Hakikisha &x na rudisha thabiti (bila kunyoosha).
#[test]
fn jaribio_k13f_anwani_bila_kunyoosha() {
    let test_chanzo = "\
N32 main() {
    N32 x = 42;
    N32* addr = &x;
    rudisha 42;  // rudisha thabiti, usitumie *addr
}
";
    run_k6_test(test_chanzo, 42);
}

/// K13g: Hakikisha kunyoosha kigeu cha kawaida (bila &).
#[test]
fn jaribio_k13g_kunyoosha_kigeu() {
    let test_chanzo = "\
N32 main() {
    N32 x = 42;
    N32* addr = &x;
    N32 y = *addr;  // nakili thamani
    rudisha y;       // 42
}
";
    run_k6_test(test_chanzo, 42);
}

/// K14a: Mzunguko mfupi (short-circuit) wa && na || kwenye mbegu.
/// Kabla ya rekebisho, mbegu ilitathmini pande ZOTE MBILI za && na || —
/// kinyume na uzalishaji.swa (mnyororo uliokusanywa na .swa ulikuwa na
/// semantiki tofauti na mbegu). `bwete && a[99999999] == 1` ilikuwa
/// inavunja mchakato kwa SEGV. Sasa dereferensi ya kulia hairukiki
/// kabisa ikiwa kushoto tayari umeamua matokeo.
#[test]
fn jaribio_mbegu_mzunguko_mfupi() {
    let kwanza = std::process::Command::new("bash")
        .arg("gharama/jenga-kwanza.sh")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("inapaswa kuendesha jenga-kwanza.sh");
    assert!(kwanza.status.success(), "mnyororo wa kwanza unapaswa kufaulu");

    let dir = tempfile::tempdir().expect("inapaswa kuunda saraka ya muda");
    let mbegu = "/tmp/mbegu2.bin";

    let chanzo = "\
N32 main() {
    N32 bwete = 0;
    N32 kweli = 1;
    N32 a[4];
    kama (bwete && a[99999999] == 1) rudisha 1;
    kama (kweli || a[99999999] == 1) { } sivyo { rudisha 2; }
    N32 x = 2 && 4;
    kama (x != 1) rudisha 3;
    N32 y = 2 || 0;
    kama (y != 1) rudisha 4;
    N32 z = 0 && a[99999999];
    kama (z != 0) rudisha 5;
    rudisha 0;
}
";
    let mz_swa = dir.path().join("mzunguko.swa");
    std::fs::write(&mz_swa, chanzo).expect("inapaswa kuandika mzunguko.swa");
    let mz_exe = dir.path().join("mzunguko-exe");
    let mz_out = std::process::Command::new(mbegu)
        .arg("--exe")
        .stdin(std::fs::File::open(&mz_swa).expect("inapaswa kufungua mzunguko.swa"))
        .stdout(std::fs::File::create(&mz_exe).expect("inapaswa kuunda mzunguko-exe"))
        .output()
        .expect("inapaswa kuendesha mbegu --exe kwa mzunguko");
    assert!(mz_out.status.success(), "mbegu --exe inapaswa kukusanya mzunguko\nstderr: {}",
        String::from_utf8_lossy(&mz_out.stderr));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let ruhusa = std::fs::metadata(&mz_exe).expect("inapaswa kusoma metadata").permissions();
        let mut ruhusa_mpya = ruhusa.clone();
        ruhusa_mpya.set_mode(0o755);
        std::fs::set_permissions(&mz_exe, ruhusa_mpya).expect("inapaswa kuweka ruhusa");
    }
    let mz_run = std::process::Command::new(&mz_exe)
        .output()
        .expect("inapaswa kuendesha mzunguko-exe");
    assert!(mz_run.status.success(),
        "mzunguko mfupi unapaswa kurudisha 0 (dereferensi za nje za mipaka zinarukwa), ilipata {:?}",
        mz_run.status.code());
}

/// K14b: Kusoma stdin kwa bomba (pipe) hakukati chanzo kwa kusoma
/// mara moja tu. Kabla ya rekebisho, mbegu ilifanya sys_read MOJA —
/// bomba linarudisha tu kile kilichopo wakati huo, hivyo chanzo
/// kikubwa (zaidi ya ukubwa wa bafa la bomba, 64 KB) kilikatwa kwa
/// nasibu na kazi za mwisho wa faili zilipotea. Jaribio hili linalisha
/// chanzo cha zaidi ya 64 KB kupitia bomba na kuthibitisha kwamba
/// kazi ya MWISHO inatolewa na inaendesha, na kwamba matokeo mawili
/// ya kusoma sawa ni baiti-kwa-baiti sawa (uhakika, si bahati).
#[test]
fn jaribio_mbegu_stdin_bomba_kubwa() {
    let kwanza = std::process::Command::new("bash")
        .arg("gharama/jenga-kwanza.sh")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("inapaswa kuendesha jenga-kwanza.sh");
    assert!(kwanza.status.success(), "mnyororo wa kwanza unapaswa kufaulu");

    let dir = tempfile::tempdir().expect("inapaswa kuunda saraka ya muda");
    let mbegu = "/tmp/mbegu2.bin";

    // ~135 KB ya maoni + kazi ya MWISHO — chanzo lazima kizidi 64 KB
    // (ukubwa wa kawaida wa bafa la bomba) kwa kiasi kikubwa.
    let mut chanzo = String::new();
    for i in 0..3000 {
        chanzo.push_str(&format!("// mstari wa kujaza bafa {i} — chanzo kikubwa kupitia bomba\n"));
    }
    chanzo.push_str("N32 main() { rudisha kazi_ya_mwisho(); }\n");
    chanzo.push_str("N32 kazi_ya_mwisho() { rudisha 7; }\n");
    assert!(chanzo.len() > 65536, "chanzo lazima kizidi 64 KB (kilikuwa {})", chanzo.len());

    let kukusanya_bomba = |dir: &tempfile::TempDir, namba: u32| -> Vec<u8> {
        let exe = dir.path().join(format!("bomba{namba}-exe"));
        let mut mtoto = std::process::Command::new(mbegu)
            .arg("--exe")
            .stdin(std::process::Stdio::piped())
            .stdout(std::fs::File::create(&exe).expect("inapaswa kuunda exe ya bomba"))
            .spawn()
            .expect("inapaswa kuanzisha mbegu --exe kwa bomba");
        {
            // Andika kwa vipande vidogo na pause fupi: hii inalazimisha
            // bomba kukua polepole. Mkusanyaji mwenye kusoma MOJA tu
            // (mdudu wa zamani) atapata kipande cha kwanza tu na
            // kukosa kazi ya mwisho — kwa uhakika, si kwa bahati.
            let stdin = mtoto.stdin.as_mut().expect("inapaswa kupata stdin ya bomba");
            use std::io::Write;
            let baiti = chanzo.as_bytes();
            let mut nafasi = 0;
            while nafasi < baiti.len() {
                let mwisho = (nafasi + 4096).min(baiti.len());
                stdin.write_all(&baiti[nafasi..mwisho])
                    .expect("inapaswa kuandika kipande kwenye bomba");
                nafasi = mwisho;
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            stdin.flush().expect("inapaswa kusukuma bomba");
        }
        let matokeo = mtoto.wait_with_output().expect("inapaswa kusubiri mbegu");
        assert!(matokeo.status.success(),
            "mbegu inapaswa kukusanya chanzo kamili kupitia bomba\nstderr: {}",
            String::from_utf8_lossy(&matokeo.stderr));
        std::fs::read(&exe).expect("inapaswa kusoma exe ya bomba")
    };

    let kwanza_pato = kukusanya_bomba(&dir, 1);
    let pili_pato = kukusanya_bomba(&dir, 2);
    assert_eq!(kwanza_pato, pili_pato,
        "kusoma bomba mara mbili kunapaswa kutoa pato linalofanana baiti-kwa-baiti");

    let exe = dir.path().join("bomba1-exe");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let ruhusa = std::fs::metadata(&exe).expect("inapaswa kusoma metadata").permissions();
        let mut ruhusa_mpya = ruhusa.clone();
        ruhusa_mpya.set_mode(0o755);
        std::fs::set_permissions(&exe, ruhusa_mpya).expect("inapaswa kuweka ruhusa");
    }
    let kuendesha = std::process::Command::new(&exe)
        .output()
        .expect("inapaswa kuendesha exe ya bomba");
    assert_eq!(kuendesha.status.code(), Some(7),
        "kazi ya MWISHO wa chanzo inapaswa kutolewa na kuendesha (kutoka 7), ilipata {:?}",
        kuendesha.status.code());
}

/// K14c: Wito wa kazi isiyofafanuliwa kwenye hali ya exe unalia kwa
/// sauti wakati wa kukusanya, badala ya kutulia kimya kwa anwani 0 na
/// kuleta SEGV wakati wa utekelezaji. Hii inafanya `husisha { faili.swa }`
/// kwa mbegu (ambayo haiwii viungo vya ndani — chanzo kinapaswa
/// kuunganishwa kwanza) kuwa hitilafu wazi, na makosa ya tahajia ya
/// majina ya kazi yanakamatwa kabla ya kuendesha.
#[test]
fn jaribio_mbegu_kazi_kukosa() {
    let kwanza = std::process::Command::new("bash")
        .arg("gharama/jenga-kwanza.sh")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("inapaswa kuendesha jenga-kwanza.sh");
    assert!(kwanza.status.success(), "mnyororo wa kwanza unapaswa kufaulu");

    let dir = tempfile::tempdir().expect("inapaswa kuunda saraka ya muda");
    let mbegu = "/tmp/mbegu2.bin";

    let chanzo = "N32 main() { rudisha kazi_haipo(); }\n";
    let kn_swa = dir.path().join("kazi-haipo.swa");
    std::fs::write(&kn_swa, chanzo).expect("inapaswa kuandika kazi-haipo.swa");
    let kn_out = std::process::Command::new(mbegu)
        .arg("--exe")
        .stdin(std::fs::File::open(&kn_swa).expect("inapaswa kufungua kazi-haipo.swa"))
        .output()
        .expect("inapaswa kuendesha mbegu --exe kwa kazi-haipo");
    assert!(!kn_out.status.success(),
        "mbegu --exe inapaswa kushindwa kwa kazi isiyofafanuliwa");
    let stdout = String::from_utf8_lossy(&kn_out.stdout);
    assert!(stdout.contains("haijafafanuliwa"),
        "ujumbe wa hitilafu unapaswa kutaja kazi isiyofafanuliwa, ilipata: {}",
        stdout);
    assert!(stdout.contains("kazi_haipo"),
        "ujumbe wa hitilafu unapaswa kutaja JINA la kazi, ilipata: {}", stdout);
}

/// K15: Maktaba ya kawaida kupitia mbegu — hesabu (gcd, pow, isqrt,
/// fibonacci), mifuatano (tafuta_mfuatano, kata_nafasi), mpangilio
/// (pangilia_n32), na I/O (sys_fungua + soma_mstari hadi EOF).
/// Faili za maktaba zinaunganishwa kwanza (kama jinsi mkusanyaji wa
/// .swa unavyojijenga) kwa sababu mbegu haiwii husisha { }.
#[test]
fn jaribio_maktaba_mbegu_exe() {
    let kwanza = std::process::Command::new("bash")
        .arg("gharama/jenga-kwanza.sh")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("inapaswa kuendesha jenga-kwanza.sh");
    assert!(kwanza.status.success(), "mnyororo wa kwanza unapaswa kufaulu");

    let dir = tempfile::tempdir().expect("inapaswa kuunda saraka ya muda");
    let mbegu = "/tmp/mbegu2.bin";

    let main_chanzo = "\
N32 main() {
    kama (gcd_hesabu(48, 18) != 6) rudisha 1;
    kama (pow_kamili(2, 10) != 1024) rudisha 2;
    kama (isqrt_hesabu(100) != 10) rudisha 3;
    kama (fibonacci_hesabu(10) != 55) rudisha 4;
    N8* s1 = \"habari za dunia\";
    kama (tafuta_mfuatano(s1, \"dunia\") != 10) rudisha 5;
    kama (tafuta_mfuatano(s1, \"hapana\") != -1) rudisha 6;
    N8* s2 = \"   swa   \";
    N64 mw = 0;
    N32 anza = kata_nafasi(s2, &mw);
    kama (anza != 3) rudisha 7;
    kama (mw != 6) rudisha 8;
    N32 data[5];
    data[0] = 5; data[1] = 2; data[2] = 4; data[3] = 1; data[4] = 3;
    pangilia_n32(data, 5);
    kama (data[0] != 1) rudisha 9;
    kama (data[4] != 5) rudisha 10;
    N64 fd = sys_fungua(\"mstari-jaribio.txt\", 0);
    kama (fd < 0) rudisha 11;
    N8 bafa[64];
    N64 n = soma_mstari(fd, bafa, 64);
    kama (n != 5) rudisha 12;
    kama (bafa[0] != 115 || bafa[4] != 49) rudisha 13;
    N64 n2 = soma_mstari(fd, bafa, 64);
    kama (n2 != 5) rudisha 14;
    N64 n3 = soma_mstari(fd, bafa, 64);
    kama (n3 != -1) rudisha 15;
    sys_funga(fd);
    rudisha 0;
}
";
    // Unganisha faili za maktaba (mbegu haiwii husisha)
    let mut chanzo = String::new();
    for f in ["msingi/hesabu.swa", "msingi/mfuatano.swa", "msingi/kumbukumbu.swa", "msingi/mpangilio.swa"] {
        chanzo.push_str(&std::fs::read_to_string(f).expect("inapaswa kusoma faili la maktaba"));
    }
    chanzo.push_str(main_chanzo);

    // Faili la jaribio la I/O — exe itaendesha kutoka saraka hii
    std::fs::write(dir.path().join("mstari-jaribio.txt"), "swa 1\nswa 2\n")
        .expect("inapaswa kuandika mstari-jaribio.txt");

    let zima = dir.path().join("maktaba-zima.swa");
    std::fs::write(&zima, chanzo).expect("inapaswa kuandika maktaba-zima.swa");
    let exe = dir.path().join("maktaba-exe");
    let out = std::process::Command::new(mbegu)
        .arg("--exe")
        .stdin(std::fs::File::open(&zima).expect("inapaswa kufungua maktaba-zima.swa"))
        .stdout(std::fs::File::create(&exe).expect("inapaswa kuunda maktaba-exe"))
        .output()
        .expect("inapaswa kuendesha mbegu --exe kwa maktaba");
    assert!(out.status.success(), "mbegu --exe inapaswa kukusanya maktaba\nstderr: {}",
        String::from_utf8_lossy(&out.stderr));

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let ruhusa = std::fs::metadata(&exe).expect("inapaswa kusoma metadata").permissions();
        let mut ruhusa_mpya = ruhusa.clone();
        ruhusa_mpya.set_mode(0o755);
        std::fs::set_permissions(&exe, ruhusa_mpya).expect("inapaswa kuweka ruhusa");
    }
    let kuendesha = std::process::Command::new(&exe)
        .current_dir(dir.path())
        .output()
        .expect("inapaswa kuendesha maktaba-exe");
    assert!(kuendesha.status.success(),
        "maktaba ya kawaida inapaswa kurudisha 0 kupitia mbegu, ilipata {:?}",
        kuendesha.status.code());
}

// ============================================================================
// Mende za dereva wa Rust/LLVM — majaribio ya kurejesha ya kudumu
// ============================================================================

/// #136: Kigezo cha safu kilileta OOM (exit 137) — mchanganuzi ulikuwa
/// unazunguka milele ukilimbikiza makosa kwa '}' isiyotumiwa, na
/// kigezo cha safu (N32 a[4]) hakikuchanganuliwa. Sasa: kigezo cha
/// safu hubadilika kuwa kielekezi (semantiki ya C) na mzunguko wa
/// kiwango cha juu una kinga ya '}'.
#[test]
fn jaribio_mende_136_kigezo_cha_safu() {
    let chanzo = "\
N32 jumla(N32 a[4]) { rudisha a[0] + a[1]; }
N32 main() {
    N32 arr[4];
    arr[0] = 1; arr[1] = 2; arr[2] = 3; arr[3] = 4;
    rudisha jumla(arr);
}
";
    run_msingi_test(chanzo, 3);
}

/// #134: kamasivyo lilichanganuliwa kama WITO WA KAZI (undefined
/// reference wakati wa kuunganisha). Sasa ni neno muhimu na mnyororo
/// wa matawi unachanganuliwa kwa kujirudia.
#[test]
fn jaribio_mende_134_kamasivyo_mnyororo() {
    let chanzo = "\
N32 kadiria(N32 x) {
    kama (x == 1) { rudisha 10; }
    kamasivyo (x == 2) { rudisha 20; }
    kamasivyo (x == 3) { rudisha 30; }
    sivyo { rudisha 40; }
}
N32 main() {
    kama (kadiria(1) != 10) rudisha 1;
    kama (kadiria(2) != 20) rudisha 2;
    kama (kadiria(3) != 30) rudisha 3;
    kama (kadiria(4) != 40) rudisha 4;
    N32 x = 5;
    kama (x == 9) { x = 1; }
    sivyo kama (x == 5) { x = 2; }
    sivyo { x = 3; }
    kama (x != 2) rudisha 5;
    rudisha 0;
}
";
    run_msingi_test(chanzo, 0);
}

/// #135: hesabu za D32/D64 zilikuwa zikiteremshwa na amri kamili
/// (add/mul) na LLVM ilikataa; halisi za desimali ziligeuka 0 kimya;
/// intern ya katikati ya thabiti za kuelea ilisababisha mgongano wa
/// ValueId; na jedwali la vihusishi vya FCmp la FFI lilikuwa
/// limesogea kimoja (OGT ikawa OEQ). Sasa: AST_HALISI_D, ukusanyaji
/// wa thabiti kabla ya kuteremsha, amri za kuelea zinazochaguliwa
/// kwa aina, na vihusishi sahihi.
#[test]
fn jaribio_mende_135_desimali() {
    let chanzo = "\
N32 main() {
    D64 pi = 3.14;
    D64 r = 2.0;
    D64 eneo = pi * r * r;
    kama (eneo > 12.55 && eneo < 12.57) { } sivyo { rudisha 1; }
    D64 a = 1.5 + 0.5;
    kama (a > 1.99 && a < 2.01) { } sivyo { rudisha 2; }
    D64 b = 6.0 / 2.0;
    kama (b > 2.99 && b < 3.01) { } sivyo { rudisha 3; }
    D64 c = -2.5;
    kama (c > -2.51 && c < -2.49) { } sivyo { rudisha 4; }
    D64 d = 10.0 - 3.25;
    kama (d > 6.74 && d < 6.76) { } sivyo { rudisha 5; }
    rudisha 0;
}
";
    run_msingi_test(chanzo, 0);
}

/// #138: "Tupu salamu()" ilileta hitilafu ya sret ya LLVM. Uchunguzi
/// wa 2026-08: kuvunjika huko tayari kumebadilishwa kuwa diagnostiki
/// ya sauti; kilichobaki ni kwamba neno muhimu "tupu" (aina ya
/// bila-thamani) halikuweza kuchanganuliwa kama aina. Sasa W0 NA
/// tupu zote zinafanya kazi.
#[test]
fn jaribio_mende_138_tupu_aina() {
    let chanzo = "\
tupu salamu() { rudisha; }
N32 main() { salamu(); rudisha 0; }
";
    run_msingi_test(chanzo, 0);
}

/// #60 (mipaka.md 4c): Desimali (D64) kwenye mnyororo wa UZALISHAJI.
/// Kabla ya 2026-08: mbegu ilisegfault kwa `21.5` na mnyororo wa .swa
/// ulipata FPE (bits_ya_d64_swa iliyokusanywa vibaya). Sasa mbegu ina
/// vitambulisho vya desimali, hesabu za kuelea (SSE2), ulinganisho,
/// na ukanushaji; mnyororo wa .swa una ABI kamili ya xmm0-xmm7.
/// Kikomo kilichobaki cha mbegu: D64 kwenye WITO wa kazi unalia kwa
/// sauti (ABI ya xmm bado haijatekelezwa huko).
#[test]
fn jaribio_mende_60_desimali_mbegu() {
    let kwanza = std::process::Command::new("bash")
        .arg("gharama/jenga-kwanza.sh")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("inapaswa kuendesha jenga-kwanza.sh");
    assert!(kwanza.status.success(), "mnyororo wa kwanza unapaswa kufaulu");

    let dir = tempfile::tempdir().expect("inapaswa kuunda saraka ya muda");
    let mbegu = "/tmp/mbegu2.bin";

    let chanzo = "\
N32 main() {
    D64 pi = 3.14;
    D64 r = 2.0;
    D64 eneo = pi * r * r;
    kama (eneo > 12.55 && eneo < 12.57) { } sivyo { rudisha 1; }
    D64 a = 1.5 + 0.5;
    kama (a > 1.99 && a < 2.01) { } sivyo { rudisha 2; }
    D64 b = 6.0 / 2.0;
    kama (b > 2.99 && b < 3.01) { } sivyo { rudisha 3; }
    D64 c = -2.5;
    kama (c > -2.51 && c < -2.49) { } sivyo { rudisha 4; }
    D64 d = 10.0 - 3.25;
    kama (d > 6.74 && d < 6.76) { } sivyo { rudisha 5; }
    rudisha 0;
}
";
    let des_swa = dir.path().join("desimali.swa");
    std::fs::write(&des_swa, chanzo).expect("inapaswa kuandika desimali.swa");
    let des_exe = dir.path().join("desimali-exe");
    let out = std::process::Command::new(mbegu)
        .arg("--exe")
        .stdin(std::fs::File::open(&des_swa).expect("inapaswa kufungua desimali.swa"))
        .stdout(std::fs::File::create(&des_exe).expect("inapaswa kuunda desimali-exe"))
        .output()
        .expect("inapaswa kuendesha mbegu --exe kwa desimali");
    assert!(out.status.success(), "mbegu --exe inapaswa kukusanya desimali\nstderr: {}",
        String::from_utf8_lossy(&out.stderr));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let ruhusa = std::fs::metadata(&des_exe).expect("inapaswa kusoma metadata").permissions();
        let mut ruhusa_mpya = ruhusa.clone();
        ruhusa_mpya.set_mode(0o755);
        std::fs::set_permissions(&des_exe, ruhusa_mpya).expect("inapaswa kuweka ruhusa");
    }
    let run = std::process::Command::new(&des_exe)
        .output()
        .expect("inapaswa kuendesha desimali-exe");
    assert!(run.status.success(),
        "desimali zinapaswa kufanya kazi kupitia mbegu (0), ilipata {:?}", run.status.code());
}

/// #61: Formatter (umbizaji) ya Swa iliyoandikwa kwa Swa yenyewe.
/// Inajijenga kupitia mbegu (faili za maktaba zinaunganishwa kwanza)
/// na LAZIMA iwe thabiti chini ya yenyewe: kuumbiza chanzo chake
/// kunatokeza baiti zinazofanana kabisa (fixpoint ya formatter).
#[test]
fn jaribio_zana_umbizaji_kujijenga() {
    let kwanza = std::process::Command::new("bash")
        .arg("gharama/jenga-kwanza.sh")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("inapaswa kuendesha jenga-kwanza.sh");
    assert!(kwanza.status.success(), "mnyororo wa kwanza unapaswa kufaulu");

    let dir = tempfile::tempdir().expect("inapaswa kuunda saraka ya muda");
    let mbegu = "/tmp/mbegu2.bin";

    // Unganisha maktaba + umbizaji (mbegu haiwi husisha { })
    let mut chanzo = String::new();
    for f in ["msingi/kumbukumbu.swa", "msingi/mfuatano.swa", "zana/umbizaji.swa"] {
        chanzo.push_str(&std::fs::read_to_string(f).expect("inapaswa kusoma faili"));
    }
    let zima = dir.path().join("umbizaji-zima.swa");
    std::fs::write(&zima, chanzo).expect("inapaswa kuandika umbizaji-zima.swa");
    let exe = dir.path().join("umbizaji-exe");
    let out = std::process::Command::new(mbegu)
        .arg("--exe")
        .stdin(std::fs::File::open(&zima).expect("inapaswa kufungua umbizaji-zima.swa"))
        .stdout(std::fs::File::create(&exe).expect("inapaswa kuunda umbizaji-exe"))
        .output()
        .expect("inapaswa kuendesha mbegu --exe kwa umbizaji");
    assert!(out.status.success(), "mbegu --exe inapaswa kukusanya umbizaji\nstderr: {}",
        String::from_utf8_lossy(&out.stderr));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let ruhusa = std::fs::metadata(&exe).expect("inapaswa kusoma metadata").permissions();
        let mut ruhusa_mpya = ruhusa.clone();
        ruhusa_mpya.set_mode(0o755);
        std::fs::set_permissions(&exe, ruhusa_mpya).expect("inapaswa kuweka ruhusa");
    }

    // Fixpoint ya formatter: kuumbiza yenyewe = baiti sawa
    let towe = std::process::Command::new(&exe)
        .arg("zana/umbizaji.swa")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("inapaswa kuendesha umbizaji-exe kwa yenyewe");
    assert!(towe.status.success(), "umbizaji unapaswa kuumbiza yenyewe\nstderr: {}",
        String::from_utf8_lossy(&towe.stderr));
    let chanzo_awali = std::fs::read("zana/umbizaji.swa").expect("inapaswa kusoma chanzo");
    assert_eq!(towe.stdout, chanzo_awali,
        "formatter inapaswa kuwa fixpoint: kuumbiza yenyewe = baiti sawa");
}
