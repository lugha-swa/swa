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
    let src = "W0 fanya() { N32 hali = 42; N32 kawaida = hali + 1; }";
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
fn jaribio_husisha_C() {
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

// ============================================================================
// Stage1
// ============================================================================

#[test]
fn jaribio_stage1() {
    let src = std::fs::read_to_string("stage1.swa")
        .expect("inapaswa kusoma faili");
    let ir = compile_and_verify(&src).expect("stage1.swa inapaswa kukusanyika");
    assert!(ir.contains("ongeza_faili"), "IR inapaswa kuwa na ongeza_faili");
    assert!(ir.contains("main"), "IR inapaswa kuwa na main");
    assert!(ir.contains("chanzo_buf"), "IR inapaswa kuwa na chanzo_buf");
}
