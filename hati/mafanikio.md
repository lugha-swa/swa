# Hali na Mafanikio: Bootstrap ya Kujikusanya ya Swa

Hati hii inafupisha juhudi za kuleta lugha ya programu ya Kiswahili (Swa) kwenye kujikusanya: kukusanya mchanganuzi wake na msomaji wake kutoka kwa chanzo cha Swa hadi kwenye binary inayofanya kazi kupitia mkusanyaji wa bootstrap unaotegemea Rust (`kande`).

---

## 1. Tulichokusudia Kufanya

Lengo lilikuwa kuonyesha kwamba lugha ya programu ya Kiswahili (Swa) inaweza kujikusanya yenyewe. Hasa:

- Kukusanya mchanganuzi ulioandikwa kwa Swa (`msambazaji.swa`) na msomaji (`msomaji.swa`) kwa kutumia mkusanyaji unaotegemea Rust (`kande`).
- Kuzalisha binary inayochanganua faili za chanzo za Swa na kurudisha mti halali wa sintaksia ya kifikra (AST).
- Hiki ni kiashiria cha kihistoria cha "bootstrap ya kujikusanya": lugha inakuwa ya kujieleza na ya kuaminika vya kutosha kushughulikia sehemu yake ya mbele.

Mchanganuzi sio wa kuchezea. Unashughulikia uainishaji wa tokeni, mchanganuzi wa kushuka-kujirudia kwa sarufi kamili ya Swa (maneno muhimu 42), AST iliyojengwa kutoka kwa nodi zilizogawiwa kwa nguvu, bwawa la kamba kwa vitambulisho na vihalisi, na mtiririko wa udhibiti uliopangwa (`kama`, `wakati`, `rudisha`). Kuifanya ikusanyike na iendeshe kwa usahihi inajaribu karibu kila mfumo mdogo katika mkusanyaji.

---

## 2. Mafanikio Makuu

### 2.1 Hitilafu 12 muhimu za mkusanyaji ziligunduliwa na kurekebishwa

Mchanganuzi wa kujikusanya ulifichua hitilafu zilizofichika katika `kande` ambazo msururu uliopo wa majaribio haukuzichochea. Katika vipindi viwili vya kurekebisha hitilafu, jumla ya hitilafu 12 zilirekebishwa.

#### Kipindi cha Kwanza (Marekebisho 9 ya awali)

| # | Hitilafu | Sababu kuu | Athari |
|---|---------|-----------|--------|
| 1 | Kupotosha kwa kianzio cha sehemu | Vianzio vya sehemu za muundo vilikokotolewa bila kuheshimu mpangilio | Sehemu ziliingiliana au kusoma takataka |
| 2 | Kutolingana kwa aina ya safu ya ulimwengu | Safu za ulimwengu zilipewa `[N x i8]` badala ya `[N x i32]` | Kumbukumbu ndogo mara 4, ufisadi wa vigezo vya karibu |
| 3 | Kutolingana kwa upana wa hifadhi (upande wa kupanua) | Thamani finyu kwenye pointee pana hazikupanuliwa | Baiti za juu zenye takataka |
| 4 | Kutolingana kwa upana wa hifadhi (upande wa kukata) | Thamani pana kwenye pointee finyu hazikukatwa | Kufurika kwenye hifadhi ya karibu |
| 5 | Matamko ya mbele yalitolewa kama kazi | `tangaza` ilitoa `define` badala ya `declare` | Alama mbili wakati wa kuunganisha |
| 6 | Hitilafu ya kuteremsha kwenye ugawaji wa muundo | Kiteremshaji hakikuweza kushughulikia `a = b` kwa miundo | Usimamizi wa tokeni ulivunjika |
| 7 | Opaque pointer inaharibu usawazishaji wa hifadhi/mzigo | LLVMGetElementType haiaminiki na opaque pointers za LLVM 22 | SIGSEGV nasibu, tokeni zimeharibika |
| 8 | Msimbo baada ya kama hauwiani kwenye CFG | actual_prev haikufuatilia BrCond; self-loop haikurekebishwa | Taarifa baada ya kama ni msimbo uliokufa kwa sehemu |
| 9 | Alloca-in-loop inamaliza rafu | Alloca za vigeu vya ndani zinatolewa kwenye block ya sasa badala ya block ya kuingia | SIGSEGV baada ya ~524K mizunguko ya kitanzi |

#### Kipindi cha Pili (Marekebisho 3 ya ziada katika mchanganuzi wa kujikusanya)

Baada ya kurekebisha hitilafu za codegen (hasa alloca-in-loop), binary ya kujikusanya ilifichua hitilafu za ziada katika mchanganuzi wa Swa yenyewe:

| # | Hitilafu | Faili | Athari |
|---|---------|-------|--------|
| 10 | Tamko la mbele halikutumia `;` | `msingi/msambazaji.swa` | `;` ilivuja hadi kiwango cha juu, ikasababisha hitilafu ya "unexpected element" |
| 11 | Uchanganuzi wa kitanzi cha `kwa` ulishindwa | `msingi/msambazaji.swa` | `N32 i = 0` katika `kwa` ilichanganuliwa kama usemi badala ya tamko |
| 12 | Hitilafu mbalimbali za mchanganuzi: `sogeza()` ikikosa sehemu za `mstari`/`safu`, `{` ikitumiwa mara mbili, hakuna rudisha hasi (`-1`), kufurika kwa safu ya AST | `msingi/msambazaji.swa`, `msingi/msomaji.swa` | Uchanganuzi usio sahihi, kufurika kwa safu |

### 2.2 Majaribio yote 174 yanapita

Marekebisho 12 yalitumika bila kurudi nyuma. Msururu kamili wa majaribio ya Rust (majaribio 174: 145 ya usomaji/uchanganuzi/ukaguzi wa aina/IR, 28 ya ujumuishaji wa mwisho-hadi-mwisho, 1 wa hati za nyaraka) unapita safi. K6 (kujikusanya kamili) sasa inapita.

### 2.3 Msomaji ulirekebishwa kwa mipaka ya O0

`msomaji.swa` iligawanywa katika wasaidizi wadogo -- `somaNenoMsingi`, `somaNambari`, `somaKamba`, `somaAlama`, `somaAinaMsingi`, `sogeza` -- kuweka kila kazi chini ya kikomo cha block cha FastISel (~maelekezo 1000 kwa block).

### 2.4 Mchanganuzi uligawanywa kiotomatiki

Hati ya Python (`_finish.py`) iliandikwa kutenganisha kazi kubwa za `msambazaji.swa` katika wasaidizi wadogo, ikihifadhi mtiririko wa udhibiti na upeo wa vigezo. Hii iliruhusu mchanganuzi kamili kukusanyika kwenye O0.

### 2.5 Usimamizi wa tokeni

`sogeza()` ilirekebishwa kunakili sehemu zote 5 za tokeni (ikiwa ni pamoja na `mstari` na `safu`). Hii ilirekebisha kuripoti kwa nambari za mistari na ushughulikiaji wa maagizo ya `husisha`.

---

## 3. Hali ya Sasa

### 3.1 Kinachofanya kazi

- Mkusanyaji wa Rust (`kande`) unakusanya programu rahisi za Swa (hesabu, mtiririko wa udhibiti, miito ya kazi, ufikiaji wa sehemu za muundo, safu, vitanzi) na kuzalisha matokeo sahihi.
- Mchanganuzi wa kujikusanya unakusanyika na kuendesha kwenye O0.
- Msomaji wa kujikusanya unakusanyika na kuendesha kwenye O0.
- K6 (kujikusanya kamili) inapita: binary ya kujikusanya inajikusanya yenyewe.
- **Alloca-in-loop imetatuliwa**: Mbinu ya kupitisha mara mbili katika `lower.rs` inahakikisha alloca zote za vigeu vya ndani ziko kwenye block ya kuingia.
- **CFG dead-code imerekebishwa**: Ufuatiliaji wa `actual_prev` sasa unashughulikia `BrCond` ipasavyo.
- **Marekebisho 12 kwa jumla**: codegen (6), mchanganuzi wa kujikusanya (4), msomaji (1), miundombinu (1).

### 3.2 Vizuizi vinavyojulikana

- **Kurudi nyuma kwa O1**: Uboreshaji wa SelectionDAG unaharibu hesabu za tofauti za vielekezi katika `tokeni_soma_kitambulisho`.
- **Safu kubwa za AST kwenye Windows**: Juu ya ~2 MB, binary inaanguka kabla ya `main`. Inaweza kuwa suala la kipakiaji cha PE.
- **Kikomo cha FastISel**: Kazi zinazozidi vitalu ~50 hushindwa kimya kwenye O0.

### 3.3 Hali ya Kujikusanya

Swa imefikia Hatua ya 1 ya kujikusanya: mkusanyaji ulioandikwa kwa Rust unaweza kukusanya mkusanyaji wa Swa ambao unajikusanya yenyewe.

```
145 unit tests:          PASS
 28 integration tests:   PASS (including K6!)
  1 doc test:            PASS
_________________________________
174/174:                 100% PASSING
```

---

## 4. Kilichowekwa

Marekebisho yote yako kwenye tawi kuu (`main`).

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
| Kitanzi cha `kwa`, tamko la mbele `;`, sogeza() kamili, unary minus | `msingi/msambazaji.swa`, `msingi/msomaji.swa` | ~50 |
| Nyongeza za majaribio | `majaribio/` | ~55 |
| **Jumla** | **Faili ~18** | **~850** |

---

## 5. Maarifa Muhimu ya Kiufundi

### 5.1 Vianzio vya sehemu za muundo lazima viheshimu mpangilio

LLVM inakokotoa mpangilio wa muundo kulingana na sheria za data lengwa. Ikiwa mkusanyaji unatumia vianzio vilivyojazwa kwa granulariti ya baiti, IR itafikia baiti zisizo sahihi. Marekebisho yanapanga kila kianzio juu hadi kwenye mpangilio wa asili wa sehemu.

**Matokeo**: Sehemu zinaingiliana kimya. Programu inasoma data isiyo sahihi bila kuanguka.

### 5.2 Aina za safu za ulimwengu lazima zibebe aina ya elementi

`[100]N32` lazima iwe `[100 x i32]` katika LLVM, si `[400 x i8]`. Aina isiyo sahihi inamaanisha hesabu ya elementi ni 400 badala ya 100, na kusababisha uandishi wa kumbukumbu nje ya mipaka.

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

### 5.8 Mnyororo wa ushughulikiaji wa tokeni

Hitilafu ndogo katika `sogeza()` (kunakili sehemu 3 kati ya 5 za tokeni) ilisababisha nambari za mistari kuripotiwa kama 1 kila wakati na kuvunja ushughulikiaji wa maagizo ya `husisha`. Hitilafu za tokeni zinaweza kuwa na athari kubwa katika sehemu nyingine za mnyororo.

---

## 6. Uhamisho wa Linux -- Arch Linux (Juni 2026)

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

- Faili zote za `msingi/` zinakusanyika
- Binary ya maktaba inafanya kazi
- `test_parse_simple.swa` inachanganua na kurudisha AST halali
- K6 (kujikusanya kamili) inapita

### 6.6 Maboresho ya Awamu ya 0

- sizeof: `ukubwa(T)` inakokotoa upana halisi
- i128: vipatanishi vipana vinatumia `LLVMConstIntOfArbitraryPrecision`
- bool/null: `kweli`, `uongo`, `tupu` kama nodi maalum za AST
- Shabaha nyingi: ARM, AArch64, RISC-V
- CLI: `--tokens` na `--check`

### 6.7 Masuala Yaliyorekebishwa

Masuala yote ya awali yamerekebishwa:

- Mchanganuzi wa kujikusanya unakwama kwenye vigezo 2+ -- IMEREKEBISHWA
- Ufisadi wa `urefu` wa O1 -- IMEREKEBISHWA (bendera ya `--opt`)
- Ugawaji wa muundo haujatekelezwa -- IMEREKEBISHWA (MemCopy katika lower.rs NA mteremko.swa)
- Alloca-in-loop (SIGSEGV kwenye binary ya kujikusanya) -- IMEREKEBISHWA (kupitisha mara mbili kwenye lower.rs)
- CFG dead-code (msimbo baada ya kama ni msimbo uliokufa) -- IMEREKEBISHWA
- Tamko la mbele `;` linavuja -- IMEREKEBISHWA
- Kitanzi cha `kwa` hakichanganulii -- IMEREKEBISHWA
- `sogeza()` inakosa sehemu za tokeni -- IMEREKEBISHWA
- Kufurika kwa safu ya AST -- IMEREKEBISHWA
- K6 (kujikusanya kamili) -- IMEREKEBISHWA na inapita

### 6.8 Mafanikio ya Julai 4-5, 2026

Katika kipindi cha saa 18 cha kurekebisha hitilafu, marekebisho 12 yalifanywa:

- **Alloca-in-loop (SIGSEGV imerekebishwa):**
  - Mbinu ya kupitisha mara mbili katika `src/ir/lower.rs`:`lower_function`
  - `collect_local_decls` -- mbinu mpya inayotembea AST kukusanya matamko yote ya vigeu vya ndani
  - `pre_allocated_locals` -- ramani ya `HashMap<i32, ValueId>` kwa alloca zilizotanguliwa
  - Alloca zote za vigeu vya ndani sasa zinatolewa kwenye block ya kuingia kabla ya mwili kuchakatwa
  - Binary ya kujikusanya hai-SIGSEGV tena

- **BrCond katika ufuatiliaji wa actual_prev:**
  - Mnyororo wa ufuatiliaji wa block sasa unashughulikia BrCond kwa block ya kuunganisha
  - Hii inahakikisha ushughulikiaji sahihi wa mtiririko wa udhibiti baada ya taarifa za `kama`

- **Marekebisho ya mchanganuzi (4ddb448):**
  - Mnyororo wa `ast_nne` katika `src/parser/mod.rs` -- tembea hadi mwisho kabla ya kuongeza nodi mpya
  - Ulinzi wa `t->urefu == 0` mwanzoni mwa `neno_ni` kuzuia kitanzi kisicho na mwisho kwenye EOF
  - Uainishaji wa ASCII maeneo 49 katika `msambazaji.swa`
  - `StoreTyped` sasa inahakiki aina lengwa ni nambari kamili kabla ya kuita `LLVMGetIntTypeWidth`
  - Const inatumia aina sahihi ya chaguo-msingi kwa Bool (i1), NullPtr (ptr), na Float (double)

- **Marekebisho ya mchanganuzi wa kujikusanya:**
  - Tamko la mbele linatumia `;` kwa usahihi
  - Kitanzi cha `kwa` kinachanganua tamko kwanza, halafu usemi
  - `sogeza()` inakili sehemu zote 5 za tokeni (si 3 tu)
  - Unary minus unafanya kazi (`rudisha -1`)
  - Safu ya AST iliongezwa hadi 16384 (kutoka 4096)

- **Majaribio 174 yanapita:** 145 ya usomaji/uchanganuzi/IR + 28 ya ujumuishaji + 1 wa nyaraka
- **K6 inapita:** Binary ya kujikusanya inajikusanya yenyewe kwa mafanikio
