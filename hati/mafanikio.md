# Hali na Mafanikio: Bootstrap ya Kujikusanya ya Swa

Hati hii inafupisha juhudi za kuleta lugha ya programu ya Kiswahili (Swa) kwenye kujikusanya: kukusanya mchanganuzi wake na msomaji wake kutoka kwa chanzo cha Swa hadi kwenye binary inayofanya kazi kupitia mkusanyaji wa bootstrap unaotegemea Rust (`kande`).

---

## 1. Tulichokusudia Kufanya

Lengo lilikuwa kuonyesha kwamba lugha ya programu ya Kiswahili (Swa) inaweza kujikusanya yenyewe. Hasa:

- Kukusanya mchanganuzi ulioandikwa kwa Swa (`msambazaji.swa`) na msomaji (`msomaji.swa`) kwa kutumia mkusanyaji unaotegemea Rust (`kande`).
- Kuzalisha binary inayochanganua faili za chanzo za Swa na kurudisha mti halali wa sintaksia ya kifikra (AST).
- Hiki ni kiashiria cha kihistoria cha "bootstrap ya kujikusanya": lugha inakuwa ya kujieleza na ya kuaminika vya kutosha kushughulikia sehemu yake ya mbele.

Mchanganuzi sio wa kuchezea. Unashughulikia uainishaji wa tokeni, mchanganuzi wa kushuka-kujirudia kwa sarufi kamili ya Swa, AST iliyojengwa kutoka kwa nodi zilizogawiwa kwa nguvu, bwawa la kamba kwa vitambulisho na vihalisi, na mtiririko wa udhibiti uliopangwa (`kama`, `wakati`, `rudisha`). Kuifanya ikusanyike na iendeshe kwa usahihi inajaribu karibu kila mfumo mdogo katika mkusanyaji.

---

## 2. Mafanikio Makuu

### 2.1 Hitilafu tisa muhimu za mkusanyaji ziligunduliwa na kurekebishwa

Mchanganuzi wa kujikusanya ulifichua hitilafu zilizofichika katika `kande` ambazo msururu uliopo wa majaribio haukuzichochea. Kila hitilafu ilikuwa kizuizi kigumu — mchanganuzi ulizalisha matokeo mabaya, ulianguka, au ulishindwa kukusanyika kabisa hadi marekebisho yalipowekwa.

| # | Hitilafu | Sababu kuu | Athari |
|---|---------|-----------|--------|
| 1 | Kupotosha kwa kianzio cha sehemu | Vianzio vya sehemu za muundo vilikokotolewa bila kuheshimu mpangilio | Sehemu ziliingiliana au kusoma takataka |
| 2 | Kutolingana kwa aina ya safu ya ulimwengu | Safu za ulimwengu zilipewa `[N × i8]` badala ya `[N × i32]` | Kumbukumbu ndogo mara 4, ufisadi wa vigezo vya karibu |
| 3 | Kutolingana kwa upana wa hifadhi (upande wa kupanua) | Thamani finyu kwenye pointee pana hazikupanuliwa | Baiti za juu zenye takataka |
| 4 | Kutolingana kwa upana wa hifadhi (upande wa kukata) | Thamani pana kwenye pointee finyu hazikukatwa | Kufurika kwenye hifadhi ya karibu |
| 5 | Matamko ya mbele yalitolewa kama kazi | `tangaza` ilitoa `define` badala ya `declare` | Alama mbili wakati wa kuunganisha |
| 6 | Hitilafu ya kuteremsha kwenye ugawaji wa muundo | Kiteremshaji hakikuweza kushughulikia `a = b` kwa miundo | Usimamizi wa tokeni ulivunjika |
| 7 | Opaque pointer inaharibu usawazishaji wa hifadhi/mzigo | LLVMGetElementType haiaminiki na opaque pointers za LLVM 22 | SIGSEGV nasibu, tokeni zimeharibika |
| 8 | Msimbo baada ya kama hauwiani kwenye CFG | actual_prev haikufuatilia BrCond; self-loop haikurekebishwa | Taarifa baada ya kama ni msimbo uliokufa kwa sehemu |
| 9 | Alloca-in-loop inamaliza rafu | Alloca za vigeu vya ndani zinatolewa kwenye block ya sasa badala ya block ya kuingia | SIGSEGV baada ya ~524K mizunguko ya kitanzi |

### 2.2 Majaribio yote 172 yanapita

Marekebisho 9 yalitumika bila kurudi nyuma. Msururu kamili wa majaribio ya Rust (majaribio 172: 144 ya usomaji/uchanganuzi/ukaguzi wa aina/IR, 27 ya ujumuishaji wa mwisho-hadi-mwisho, 1 wa hati za nyaraka) unapita safi. K6 (kujikusanya kamili) imezimwa kwa sasa — inasubiri uchunguzi zaidi wa hitilafu ya mchanganuzi iliyobaki.

### 2.3 Msomaji ulirekebishwa kwa mipaka ya O0

`msomaji.swa` iligawanywa katika wasaidizi wadogo — `somaNenoMsingi`, `somaNambari`, `somaKamba`, `somaAlama`, `somaAinaMsingi`, `sogeza` — kuweka kila kazi chini ya kikomo cha block cha FastISel (~maelekezo 1000 kwa block).

### 2.4 Mchanganuzi uligawanywa kiotomatiki

Hati ya Python (`_finish.py`) iliandikwa kutenganisha kazi kubwa za `msambazaji.swa` katika wasaidizi wadogo, ikihifadhi mtiririko wa udhibiti na upeo wa vigezo. Hii iliruhusu mchanganuzi kamili kukusanyika kwenye O0.

### 2.5 Suluhisho la muda la usimamizi wa tokeni

Kwa kuwa ugawaji wa muundo umevunjika, `sogeza()` inatumia nakala ya sehemu-kwa-sehemu:
```
sasa.aina = kesho.aina;
sasa.urefu = kesho.urefu;
sasa.chanzo = kesho.chanzo;
```

---

## 3. Hali ya Sasa

### 3.1 Kinachofanya kazi

- Mkusanyaji wa Rust (`kande`) unakusanya programu rahisi za Swa (hesabu, mtiririko wa udhibiti, miito ya kazi, ufikiaji wa sehemu za muundo, safu, vitanzi) na kuzalisha matokeo sahihi.
- Mchanganuzi wa kujikusanya unakusanyika na kuendesha kwenye O0 (nodi 512 za AST, bwawa la 32 KB).
- Msomaji wa kujikusanya unakusanyika na kuendesha kwenye O0.
- Binary zote za majaribio ya uchanganuzi zinazalisha matokeo sahihi.
- **Masuala yote ya alloca-in-loop yametatuliwa**: Mbinu ya kupitisha mara mbili katika `lower.rs` inahakikisha alloca zote za vigeu vya ndani ziko kwenye block ya kuingia, na hivyo kuzuia uharibifu wa rafu katika vitanzi.
- **CFG dead-code imerekebishwa**: Ufuatiliaji wa `actual_prev` sasa unashughulikia `BrCond` ipasavyo, kuhakikisha mtiririko sahihi baada ya taarifa za `kama`.

### 3.2 Vizuizi vinavyojulikana

- **Kurudi nyuma kwa O1**: `urefu` wa tokeni umeharibiwa kwenye O1. Sababu kuu haijatengwa.
- **Safu kubwa za AST zinaanguka kwenye Windows**: Juu ya ~2 MB, binary inaanguka kabla ya `main`. Inaweza kuwa suala la kipakiaji cha PE.
- **Hitilafu ya mchanganuzi katika binary ya kujikusanya**: Baada ya kurekebisha alloca-in-loop (SIGSEGV), binary inaendelea hadi kwenye hitilafu ya uchanganuzi tofauti. Hii ilikuwa ipo awali lakini ilifichwa na SIGSEGV.
- **K6 imezimwa**: Jaribio kamili la kujikusanya linasubiri hitilafu ya mchanganuzi itatuliwe.

---

## 4. Kilichowekwa

Marekebisho yote yako kwenye tawi la `rekebisha/makosa-ya-kimsingi-ya-mkusanyaji` (PR #34) na tawi kuu (`main`).

| Wigo | Faili | Mistari |
|---|---|---|
| Mpangilio wa kianzio cha sehemu | `src/codegen.rs` | ~40 |
| Aina ya elementi ya safu ya ulimwengu | `src/codegen.rs` | ~30 |
| Ukataji wa hifadhi | `src/codegen.rs` | ~25 |
| Upanuzi wa hifadhi | `src/codegen.rs` | ~25 |
| Tangazo la mbele | `src/codegen.rs`, `src/ast.rs` | ~35 |
| Kiteremshaji + suluhisho la muda la muundo | `src/codegen.rs`, `src/lower.rs` | ~60 |
| Opaque pointer za LLVM 22, mgongano wa ValueId, builtins za kumbukumbu | `src/codegen/llvm/mod.rs`, `src/ir/lower.rs`, `src/ir/mod.rs`, `build.rs`, `ffi.rs` | ~200 |
| CFG dead-code (actual_prev, BrCond, self-loop) | `src/ir/lower.rs` | ~40 |
| Alloca-in-loop (kupitisha mara mbili, pre-allocated locals) | `src/ir/lower.rs` | ~100 |
| Usomaji wa safu (AST_SAFU, AST_TAJA, faharasa) | `src/parser/mod.rs`, `msingi/msambazaji.swa` | ~130 |
| Nyongeza za majaribio | `majaribio/` | ~55 |
| **Jumla** | **Faili ~15** | **~800** |

---

## 5. Maarifa Muhimu ya Kiufundi

### 5.1 Vianzio vya sehemu za muundo lazima viheshimu mpangilio

LLVM inakokotoa mpangilio wa muundo kulingana na sheria za data lengwa. Ikiwa mkusanyaji unatumia vianzio vilivyojazwa kwa granulariti ya baiti, IR itafikia baiti zisizo sahihi. Marekebisho yanapanga kila kianzio juu hadi kwenye mpangilio wa asili wa sehemu.

**Matokeo**: Sehemu zinaingiliana kimya. Programu inasoma data isiyo sahihi bila kuanguka.

### 5.2 Aina za safu za ulimwengu lazima zibebe aina ya elementi

`[100]N32` lazima iwe `[100 × i32]` katika LLVM, si `[400 × i8]`. Aina isiyo sahihi inamaanisha hesabu ya elementi ni 400 badala ya 100, na kusababisha uandishi wa kumbukumbu nje ya mipaka.

### 5.3 Hifadhi lazima zilingane na upana wa pointee

Kuhifadhi thamani pana kwenye pointee finyu kunahitaji `trunc`. Kuhifadhi thamani finyu kwenye pointee pana kunahitaji `zext`. Kukosea husababisha ufisadi wa kumbukumbu ya karibu.

### 5.4 Matamko ya mbele hayapaswi kuzalisha miili ya kazi

`tangaza kazi` inapaswa kutoa `declare` ya LLVM, si `define` yenye mwili tupu. Ufafanuzi wa mwili tupu unashindana na utekelezaji halisi.

### 5.5 Upakiaji wa Windows PE na sehemu kubwa za BSS

Safu za ulimwengu zenye ukubwa wa BSS juu ya ~2 MB zinaanguka kabla ya `main` kwenye Windows. Inaweza kuwa suala la CRT au kikomo cha sehemu ya PE. Kwenye Linux na ELF, safu kubwa hushughulikiwa bila tatizo.

### 5.6 CFG: Taarifa za udhibiti zinahitaji ushughulikiaji makini wa mwendelezo

Baada ya taarifa ya `kama`, block ya sharti ya `BrCond` haipaswi kuwa kiungo cha mwendelezo kwa taarifa inayofuata. `actual_prev` lazima ifuatilie block ya kuunganisha (`merge`). Kukosa kufanya hivyo kunasababisha msimbo usio na marejeleo (dead code) kwenye CFG na vitanzi vya kujirudia visivyo na mwisho.

### 5.7 Alloca za vigeu vya ndani lazima ziwe kwenye block ya kuingia

LLVM inatarajia alloca zote za vigeu vya ndani ziwe kwenye block ya kuingia ya kazi. Kutoa alloca kwenye block ya kitanzi kunasababisha kila mzunguko kugawa nafasi mpya ya rafu bila kuzirejesha. Hii inaisha kwa uharibifu wa rafu (SIGSEGV). Suluhisho ni mbinu ya kupitisha mara mbili: (1) tembea AST mapema kukusanya matamko yote ya vigeu, (2) toa alloca zote kwenye block ya kuingia kabla ya mwili wowote kuchakatwa.

---

## 6. Uhamisho wa Linux — Arch Linux (Juni 2026)

Mradi ulihamishwa kutoka Windows hadi Arch Linux. Hii ilifichua hitilafu zilizofichika katika mkusanyaji wa bootstrap wa Rust na mchanganuzi wa kujikusanya.

### 6.1 Mfumo wa Kujenga

`build.rs` ilibadilishwa kutumia `llvm-config` kwenye Linux. Sifa ya `#[link(name = "LLVM-C")]` iliondolewa kutoka `ffi.rs`.

### 6.2 Marekebisho ya Opaque Pointer ya LLVM

LLVM 22.1 inatumia opaque pointers. `LLVMGetElementType` inarudisha matokeo yasiyoaminika.

| Hitilafu | Dalili | Marekebisho |
|---|---|---|
| `i64` hadi `i32` katika vigezo | Kufurika kwa bwawa la `hifadhi_jina` | Store -> StoreTyped |
| Nakala za sehemu zinaharibu tokeni | `sogeza()` inaharibu `sasa.aina` | Store ya kawaida imerahisishwa |
| Mzigo unasoma upana usio sahihi | Sehemu za `i32` zinasoma takataka | Load tayari ilitumia IrType |

### 6.3 Mgongano wa ValueId

Iliongeza `Instruction::Const(Const)` kwenye IR. `emit()` sasa inatumia `func.values.len()` badala ya `values_initial_len`. Ilirekebisha `N32 sz = ukubwa(N32)`, `orodha.swa`, na `test_ops.swa`.

### 6.4 Builtins za Kumbukumbu

Iliongeza utambuzi wa `tenga` -> HeapAlloc, `achilia` -> HeapFree, `badili` -> realloc katika `lower_call`. Iliongeza `realloc` kwenye `pre_declare_libc`.

### 6.5 Mkusanyaji wa Kujikusanya kwenye Linux

- Faili zote 7 za `msingi/` zinakusanyika
- Binary ya maktaba 6 inafanya kazi
- `test_parse_simple.swa` inachanganua na kurudisha AST halali
- Teremsha inatoa LLVM IR halali kwa kazi za kigezo kimoja

### 6.6 Maboresho ya Awamu ya 0

- sizeof: `ukubwa(T)` inakokotoa upana halisi
- i128: vipatanishi vipana vinatumia `LLVMConstIntOfArbitraryPrecision`
- bool/null: `kweli`, `uongo`, `tupu` kama nodi maalum za AST
- Shabaha nyingi: ARM, AArch64, RISC-V
- CLI: `--tokens` na `--check`

### 6.7 Masuala Yanayobaki (Julai 4, 2026 — Masuala 5 ya awali yamerekebishwa)

- ~~Mchanganuzi wa kujikusanya unakwama kwenye vigezo 2+~~ — IMEREKEBISHWA (mnyororo wa vigezo)
- ~~Ufisadi wa `urefu` wa O1~~ — IMEREKEBISHWA (bendera ya `--opt`)
- ~~Ugawaji wa muundo haujatekelezwa~~ — IMEREKEBISHWA (MemCopy katika lower.rs NA mteremko.swa)
- ~~Alloca-in-loop (SIGSEGV kwenye binary ya kujikusanya)~~ — IMEREKEBISHWA (kupitisha mara mbili kwenye lower.rs)

**Kizuizi kipya kikuu:** Baada ya kurekebisha alloca-in-loop, SIGSEGV imeondolewa. Binary ya kujikusanya sasa inaendelea hadi kwenye hitilafu ya mchanganuzi: `unexpected token on line 1` (kwenye `}`). Hitilafu hii ilikuwa ipo awali lakini ilifichwa na SIGSEGV. Mkusanyaji wa Rust unafaulu kukusanya stage1.swa hadi IR na faili la kitu, binary inajenga na kuanza kutekelezwa, lakini mchanganuzi unashindwa kuchanganua msimbo wake mwenyewe. Uchunguzi zaidi unahitajika ili kubaini kama hitilafu iko kwenye codegen ya Rust au kwenye mantiki ya mchanganuzi yenyewe.

### 6.8 Mafanikio ya Julai 4, 2026

- **Alloca-in-loop (SIGSEGV imerekebishwa):**
  - Mbinu ya kupitisha mara mbili katika `src/ir/lower.rs`:`lower_function`
  - `collect_local_decls` — mbinu mpya inayotembea AST kukusanya matamko yote ya vigeu vya ndani
  - `pre_allocated_locals` — ramani ya `HashMap<i32, ValueId>` kwa alloca zilizotanguliwa
  - Alloca zote za vigeu vya ndani sasa zinatolewa kwenye block ya kuingia kabla ya mwili kuchakatwa
  - `lower_local_decl` inatumia alloca iliyotanguliwa badala ya kutoa Alloca mpya
  - `collect_constants` imeboreshwa kushughulikia `AST_KWELI`, `AST_UONGO`, na `AST_TUPU`
  - Binary ya kujikusanya hai-SIGSEGV tena; inaendelea hadi kwenye hitilafu ya mchanganuzi tofauti
- **BrCond katika ufuatiliaji wa actual_prev:**
  - Mnyororo wa ufuatiliaji wa block sasa unashughulikia BrCond kwa block ya kuunganisha
  - Hii inahakikisha ushughulikiaji sahihi wa mtiririko wa udhibiti baada ya taarifa za `kama`
- **Marekebisho ya mchanganuzi (4ddb448):**
  - Mnyororo wa `ast_nne` katika `src/parser/mod.rs` — tembea hadi mwisho kabla ya kuongeza nodi mpya
  - Ulinzi wa `t->urefu == 0` mwanzoni mwa `neno_ni` kuzuia kitanzi kisicho na mwisho kwenye EOF
  - Uainishaji wa ASCII maeneo 49 katika `msambazaji.swa` — herufi `[` na `]` sasa zinashughulikiwa kama vihusishi
  - `StoreTyped` sasa inahakiki aina lengwa ni nambari kamili kabla ya kuita `LLVMGetIntTypeWidth`
  - Const inatumia aina sahihi ya chaguo-msingi kwa Bool (i1), NullPtr (ptr), na Float (double)
- **Majaribio 172 yanapita:** 144 ya usomaji/uchanganuzi/IR + 27 ya ujumuishaji + 1 wa nyaraka
- **K6:** Binary inaendelea na hai-SIGSEGV; ina hitilafu tofauti ya mchanganuzi inayohitaji uchunguzi
