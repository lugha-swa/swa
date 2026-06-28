# Marekebisho ya Mkusanyaji: Bootstrap ya Kujikusanya

Wakati wa juhudi za kufanya mkusanyaji wa Rust wa kande uweze kujikusanya (kujikusanya yenyewe), hitilafu sita za usahihi ziligunduliwa na kurekebishwa. Hati hii inaelezea kila hitilafu kwa usahihi — ilipoishi, nini kiliharibika, jinsi ilivyojitokeza, na jinsi ilivyorekebishwa.

---

## 1. Matamko ya Mbele Yalitengeneza Vijisabuni Vitupu

**Faili:** `src/ir/lower.rs`

**Hitilafu.** Wakati kiteremshaji cha IR kilipokutana na *tangazo la mbele* — saini ya kazi isiyo na mwili, k.m.

```c
N32 foo(Msambazaji* p);
```

ilitoa ufafanuzi wa kazi ya LLVM bila masharti. Kwa kazi ambazo hazikuwa na mwili katika kitengo cha sasa cha utafsiri, hii ilizalisha *mwili tupu wa kazi* (ufafanuzi usio na vitalu vya msingi). Ikiwa ufafanuzi halisi wa kazi hiyo hiyo ulionekana baadaye katika kitengo cha utafsiri, kijisabuni tupu *kilifunika* — kiunganishi cha LLVM kingeona ufafanuzi mbili kwa alama hiyo hiyo na kuchagua ya kwanza (tupu). Utekelezaji halisi ulikuwa msimbo uliokufa.

**Marekebisho.** Uchunguzi wa awali wa AST uliongezwa ili kukusanya seti ya majina ya kazi ambazo *zina* miili. Katika `lower_function`, ukaguzi uliingizwa: ikiwa kazi haina mwili **na** kazi yenye jina sawa na mwili ipo mahali pengine, kijisabuni tupu kinarukwa na ufafanuzi halisi pekee ndio unaotolewa.

---

## 2. Kutolingana kwa Upana wa Hifadhi — `i64` hadi `i32`

**Faili:** `src/codegen/llvm/mod.rs`

**Hitilafu.** Kibadala cha `Const::Int` kilikuwa kikijitokeza kama `i64` (baiti 8) katika LLVM IR, bila kujali aina lengwa. Wakati msimbo ulipogawa `Const::Int` kwa mgao wa rafu wa `i32`, elekezo la hifadhi liliandika baiti 8 kwenye baiti 4 za nafasi ya alloca. Baiti za ziada zilifurika kwenye vigezo vya karibu vya rafu, na kuharibu thamani zao kimya.

**Marekebisho.** Kidhibiti cha hifadhi kilibadilishwa kuuliza aina ya kielekezi cha mgao lengwa kupitia `LLVMGetElementType`, kisha kuingiza `LLVMBuildIntCast2` ili kukata (au kupanua) nambari kamili ili ilingane na upana wa lengwa kabla ya kutoa elekezo la hifadhi.

---

## 3. Kutolingana kwa Upana wa Hifadhi — `i32` hadi `i64` (Sehemu za Muundo)

**Faili:** `src/codegen/llvm/mod.rs`

**Hitilafu.** Wakati wa kugawa kihalisi cha `N32` (i32, baiti 4) kwa sehemu ya muundo ya `N64` (i64, baiti 8), kidhibiti cha hifadhi kiliandika baiti 4 pekee. Baiti 4 za juu za sehemu lengwa zilihifadhi takataka yoyote iliyokuwa tayari kwenye kumbukumbu. Hali ya awali ilinda uigizaji tu kwa kesi ambapo upana wa chanzo ulikuwa *mkubwa zaidi kuliko* upana wa lengwa (`>`), hivyo upanuzi wa kupanua haukuwahi kutokea.

**Marekebisho.** Hali ya ulinganifu wa upana ilibadilishwa kutoka `>` hadi `!=`, ili ukataji (chanzo cha baiti 4 kwenye sehemu ya baiti 2) na upanuzi (chanzo cha baiti 4 kwenye sehemu ya baiti 8) vishughulikiwe. Msaidizi wa `StoreTyped` pia ulirekebishwa ili ulingane.

---

## 4. FieldAddr Ilipuuza Mpangilio

**Faili:** `src/codegen/llvm/mod.rs`

**Hitilafu.** Kidhibiti cha `FieldAddr` kilikokotoa vianzio vya baiti kwenye aina za mkusanyiko za LLVM kwa kujumlisha *ukubwa mbichi wa elementi* za sehemu zilizotangulia bila kutumia padding ya mpangilio. Mpangilio wa muundo wa LLVM unahitaji kila sehemu ipangiliwe kwa mpangilio wake wa asili; mkusanyaji alikuwa akiweka sehemu kwenye kianzio kisichopangiliwa.

Kwa mfano, kwa muundo `{i32, ptr, i64}`:
- `i32` kwenye kianzio 0 (ukubwa 4)
- `ptr` kwenye kianzio 4 (4 imepangiliwa kwa kielekezi cha baiti 8 kwenye 64-bit? Hapana — inahitaji kianzio 8)
- `i64` kwenye kianzio `4 + 8 = 12` (inapaswa kuwa 16)

Mkusanyaji alikokotoa `4 + 8 = 12` kwa kianzio cha sehemu ya `i64`, lakini mpangilio wa LLVM wenyewe uliiweka kwenye kianzio 16. GEP zilizokokotwa kwa kianzio kisicho sahihi zilifikia baiti zisizo sahihi.

**Marekebisho.** Kidhibiti cha `FieldAddr` sasa kinatumia mpangilio kwa kila upana wa sehemu kabla ya kujumlisha: kianzio cha kila sehemu kinazungushwa juu hadi kizidishio kinachofuata cha mpangilio wa sehemu, kisha ukubwa wa sehemu unaongezwa. Hii inazalisha vianzio vinavyolingana na matarajio ya `getelementptr` ya LLVM.

---

## 5. `width_bytes` ya Muundo Ilikosa Padding ya Mwisho

**Faili:** `src/ir/types.rs`

**Hitilafu.** Mbinu ya `width_bytes()` kwenye aina za muundo ilikokotoa ukubwa wote kwa kujumlisha upana mbichi wa kila sehemu, bila padding kati au baada ya sehemu. Kwa mfano, muundo `Tokeni { i32, i8*, i64, i32, i32 }` una sehemu:

| sehemu | ukubwa | mpangilio wa asili |
|-------|--------|---------------------|
| i32   | 4      | 4                   |
| i8*   | 8      | 8                   |
| i64   | 8      | 8                   |
| i32   | 4      | 4                   |
| i32   | 4      | 4                   |

Kwa mpangilio (mpangilio wa juu = 8): `4 + 4(pad) + 8 + 8 + 4 + 4 + 4(pad) = 36…` kwa kweli:
- i32 kwenye 0..4
- padding 4..8
- i8* kwenye 8..16
- i64 kwenye 16..24
- i32 kwenye 24..28
- i32 kwenye 28..32
- padding ya mwisho kwa kizidishio cha 8: 32

Hivyo muundo ni baiti 32. `width_bytes()` bila padding ilirudisha 28 (4+8+8+4+4).

Hii ilisababisha allocas za `sret` (kurejesha muundo) kuwa na ukubwa mdogo. Wakati kazi iliporudisha muundo kwa kielekezi kilichofichwa, mpigaji aligawa nafasi kulingana na `width_bytes()` — ndogo sana — na mpigiwa aliandika kupita mgao.

**Marekebisho.** `width_bytes()` iliandikwa upya ili kukokotoa ukubwa kwa mpangilio sahihi: kianzio cha kila sehemu kinapangiliwa kwa mpangilio wa asili wa sehemu kabla ya kuiweka, na ukubwa wote unapigwa padding kwa kizidishio cha mpangilio wa juu wa sehemu ya muundo. Hii inalingana na upangaji wa `DataLayout` ya LLVM.

---

## 6. Aina za Safu za Ulimwengu Zilitangazwa kama `[N×i8]` Badala ya `[N×i32]`

**Faili:** `src/ir/mod.rs`, `src/ir/lower.rs`, `src/codegen/llvm/mod.rs`

**Hitilafu.** Muundo wa `IrGlobal` haukuwa na sehemu ya aina — ulibeba urefu wa baiti pekee. Wakati mwisho wa LLVM ulipohitaji kutangaza safu ya ulimwengu, ilikisia aina ya elementi kutoka kwa urefu wa baiti: ikiwa safu ilikuwa zaidi ya baiti 8, kila mara ilikuwa `[N×i8]`, kwa sababu taarifa pekee iliyopatikana ilikuwa hesabu ya baiti wote na mwisho ulidhani kila kitu kinachoweza kushughulikiwa kwa baiti kilikuwa na aina ya baiti.

Fikiria `N32 ast_aina[2048]` — safu ya nambari kamili za baiti nne 2048 (jumla ya baiti 8192). Mwisho uliitangaza kama `[2048×i8]` (baiti 2048 pekee). Kila uandishi ulioorodheshwa kupitia `GEP i32` ulifikia kumbukumbu kwenye `msingi + faharisi * 4`, ambayo ilifurika haraka mgao wa baiti 2048 na kuharibu vigezo vya karibu vya ulimwengu.

**Marekebisho.** Sehemu ya `ty: IrType` iliongezwa kwa `IrGlobal`. Mwisho wa LLVM sasa unatumia `ir_type_to_llvm()` kutoa aina sahihi ya safu ya LLVM (k.m., `[2048×i32]`) kwa aina za elementi changamani, ukihifadhi `[N×i8]` kwa data yenye aina ya baiti pekee.

---

## 7. Marekebisho ya Opaque Pointer ya LLVM — Usawazishaji wa Aina ya Hifadhi/Mzigo (Uhamisho wa Linux)

**Faili:** `src/codegen/llvm/mod.rs`, `src/ir/lower.rs`, `src/ir/mod.rs`

**Hitilafu.** LLVM 22.1 kwenye Arch Linux inatumia opaque pointers kwa chaguo-msingi. Kazi ya API ya C ya `LLVMGetElementType`, ambayo usawazishaji wa upana wa hifadhi ulitegemea, inarudisha matokeo yasiyoaminika na opaque pointers. Hii ilisababisha:

- Vigezo vya `i64` vilikatwa hadi `i32` wakati wa kuhifadhiwa kwenye alloca, na kusababisha kufurika kwa bwawa katika `hifadhi_jina` na hitilafu za sehemu nasibu
- Nakala za sehemu za muundo katika `sogeza()` ziliharibu hali ya tokeni, na kufanya ulinganifu wa `== TOKENI_ISHARA` ushindwe
- Mizigo ilisoma upana usio sahihi kutoka kwa opaque pointers

**Marekebisho.**
- Hifadhi za vigezo zilibadilishwa kutoka `Instruction::Store` hadi `Instruction::StoreTyped`, ambayo hubeba IrType kwa uwazi na inapita API iliyopitwa na wakati
- Kidhibiti cha `Instruction::Store` cha kawaida kiliondoa usawazishaji wa upana wa `LLVMGetElementType` kabisa — njia zote zinazohitaji upana tayari zinatumia `StoreTyped`
- `Instruction::Const(Const)` iliongezwa kwenye IR kwa vipatanishi vinavyobadilika, kuzuia migongano ya ValueId
- `emit()` ilibadilishwa kutumia `func.values.len()` inayobadilika badala ya `values_initial_len` tuli

---

## Muhtasari

| # | Hitilafu | Faili | Dalili | Sababu Kuu |
|---|---------|-------|--------|------------|
| 1 | Matamko ya mbele yanatoa vijisabuni vitupu | `src/ir/lower.rs` | Utekelezaji halisi umefunikwa | Hakuna ukaguzi wa kuwepo kwa mwili |
| 2 | Hifadhi ya i64 kwa alloca ya i32 | `src/codegen/llvm/mod.rs` | Vigezo vya karibu vya rafu vimeharibika | Hakuna usawazishaji wa upana kwenye hifadhi |
| 3 | Hifadhi ya i32 kwa sehemu ya muundo ya i64 | `src/codegen/llvm/mod.rs` | Takataka katika baiti 4 za juu | Ukaguzi wa upana ulitumia `>` badala ya `!=` |
| 4 | FieldAddr inapuuza mpangilio | `src/codegen/llvm/mod.rs` | Sehemu isiyo sahihi ya muundo imefikiwa | Vianzio vimejumlishwa bila mpangilio |
| 5 | width_bytes ya muundo inakosa padding | `src/ir/types.rs` | Allocas za sret ndogo sana | Hakuna padding ya mpangilio wa mwisho |
| 6 | Safu za ulimwengu zimeainishwa kama [N×i8] | `src/ir/mod.rs`, `lower.rs`, `llvm/mod.rs` | Vigezo vya karibu vya ulimwengu vimeharibika | IrGlobal ilikosa sehemu ya aina |
| 7 | Opaque pointer inaharibu usawazishaji wa hifadhi | `src/codegen/llvm/mod.rs`, `src/ir/lower.rs`, `src/ir/mod.rs` | Hitilafu za sehemu nasibu, ulinganifu wa tokeni unashindwa | LLVMGetElementType haiaminiki na opaque pointers |
