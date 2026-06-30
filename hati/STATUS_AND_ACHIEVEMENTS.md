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

### 2.1 Hitilafu sita muhimu za mkusanyaji ziligunduliwa na kurekebishwa

Mchanganuzi wa kujikusanya ulifichua hitilafu zilizofichika katika `kande` ambazo msururu uliopo wa majaribio haukuzichochea. Kila hitilafu ilikuwa kizuizi kigumu — mchanganuzi ulizalisha matokeo mabaya, ulianguka, au ulishindwa kukusanyika kabisa hadi marekebisho yalipowekwa.

| # | Hitilafu | Sababu kuu | Athari |
|---|---------|-----------|--------|
| 1 | Kupotosha kwa kianzio cha sehemu | Vianzio vya sehemu za muundo vilikokotolewa bila kuheshimu mpangilio | Sehemu ziliingiliana au kusoma takataka |
| 2 | Kutolingana kwa aina ya safu ya ulimwengu | Safu za ulimwengu zilipewa `[N × i8]` badala ya `[N × i32]` | Kumbukumbu ndogo mara 4, ufisadi wa vigezo vya karibu |
| 3 | Kutolingana kwa upana wa hifadhi (upande wa kupanua) | Thamani finyu kwenye pointee pana hazikupanuliwa | Baiti za juu zenye takataka |
| 4 | Kutolingana kwa upana wa hifadhi (upande wa kukata) | Thamani pana kwenye pointee finyu hazikukatwa | Kufurika kwenye hifadhi ya karibu |
| 5 | Matamko ya mbele yalitolewa kama kazi | `tangaza` ilitoa `define` badala ya `declare` | Alama mbili wakati wa kuunganisha |
| 6 | Hitilafu ya kuteremsha kwenye ugawaji wa muundo | Kiteremshaji hakikuweza kushughulikia `a = b` kwa miundo | Usimamizi wa tokeni ulivunjika |

### 2.2 Majaribio yote 171 yanapita

Marekebisho 6 yalitumika bila kurudi nyuma. Msururu kamili wa majaribio ya Rust (majaribio 171: usomaji, uchanganuzi, ukaguzi wa aina, uzalishaji wa msimbo, mkusanyiko wa mwisho-hadi-mwisho) unapita safi.

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

- Mkusanyaji wa Rust (`kande`) unakusanya programu rahisi za Swa (hesabu, mtiririko wa udhibiti, miito ya kazi, ufikiaji wa sehemu za muundo, safu) na kuzalisha matokeo sahihi.
- Mchanganuzi wa kujikusanya unakusanyika na kuendesha kwenye O0 (nodi 512 za AST, bwawa la 32 KB).
- Msomaji wa kujikusanya unakusanyika na kuendesha kwenye O0.
- Binary zote za majaribio ya uchanganuzi zinazalisha matokeo sahihi.

### 3.2 Vizuizi vinavyojulikana

- **Kurudi nyuma kwa O1**: `urefu` wa tokeni umeharibiwa kwenye O1. Sababu kuu haijatengwa.
- **Safu kubwa za AST zinaanguka kwenye Windows**: Juu ya ~2 MB, binary inaanguka kabla ya `main`. Inaweza kuwa suala la kipakiaji cha PE.
- **Ugawaji wa muundo** bado haujatekelezwa katika kiteremshaji.

---

## 4. Kilichowekwa

Marekebisho yote yako kwenye tawi la `rekebisha/makosa-ya-kimsingi-ya-mkusanyaji` (PR #34).

| Wigo | Faili | Mistari |
|---|---|---|
| Mpangilio wa kianzio cha sehemu | `src/codegen.rs` | ~40 |
| Aina ya elementi ya safu ya ulimwengu | `src/codegen.rs` | ~30 |
| Ukataji wa hifadhi | `src/codegen.rs` | ~25 |
| Upanuzi wa hifadhi | `src/codegen.rs` | ~25 |
| Tangazo la mbele | `src/codegen.rs`, `src/ast.rs` | ~35 |
| Kiteremshaji + suluhisho la muda la muundo | `src/codegen.rs`, `src/lower.rs` | ~60 |
| Nyongeza za majaribio | `majaribio/` | ~55 |
| **Jumla** | **Faili 7** | **~270** |

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

### 6.7 Masuala Yanayobaki (Juni 30, 2026 — Masuala 3 ya awali yamerekebishwa)

- ~~Mchanganuzi wa kujikusanya unakwama kwenye vigezo 2+~~ — IMEREKEBISHWA (mnyororo wa vigezo)
- ~~Ufisadi wa `urefu` wa O1~~ — IMEREKEBISHWA (bendera ya `--opt`)
- ~~Ugawaji wa muundo haujatekelezwa~~ — IMEREKEBISHWA (MemCopy katika lower.rs NA mteremko.swa)

**Kizuizi kipya kikuu:** K6 — jaribio la kujikusanya kamili linaandika binary inayoanguka (SIGSEGV) kabla ya kutoa matokeo yoyote.
Mkusanyaji wa Rust unafaulu kukusanya stage1.swa hadi IR na faili la kitu, lakini binary inayotokana inaanguka wakati wa utekelezaji.
Hitilafu za codegen za msingi katika mwisho wa LLVM wa mkusanyaji wa Rust zinahitaji kutafitiwa.

### 6.8 Mafanikio ya Juni 30, 2026

- **K5:** MemCopy imeongezwa katika kiteremshaji cha kujikusanya (mteremko.swa):
  - `ukubwa_muundo()` — kokotoa ukubwa wa baiti kwa aina yoyote
  - `nakili_muundo()` — toa `@llvm.memcpy` kwa kunakili miundo
  - `sret_vid_sasa` — kielekezi cha sret kwa kazi zinazorudisha miundo
  - Ugawaji wa muundo, uanzishaji, na urejeshaji vyote vinatumia MemCopy
- **lower.rs:** Rekebisho la msururu wa `kama-sivyo` katika `lower_block` na `lower_if`
- **K6:** Jaribio kamili la kujikusanya limeandikwa (limezimwa kwa sasa)
