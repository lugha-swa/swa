# Marekebisho ya Mkusanyaji: Bootstrap ya Kujikusanya

Wakati wa juhudi za kufanya mkusanyaji wa Rust wa kande uweze kujikusanya (kujikusanya yenyewe), hitilafu tisa za usahihi ziligunduliwa na kurekebishwa. Hati hii inaelezea kila hitilafu kwa usahihi -- ilipoishi, nini kiliharibika, jinsi ilivyojitokeza, na jinsi ilivyorekebishwa.

---

## 1. Matamko ya Mbele Yalitengeneza Vijisabuni Vitupu

**Faili:** `src/ir/lower.rs`

**Hitilafu.** Wakati kiteremshaji cha IR kilipokutana na *tangazo la mbele* -- saini ya kazi isiyo na mwili, k.m.

```c
N32 foo(Msambazaji* p);
```

ilitoa ufafanuzi wa kazi ya LLVM bila masharti. Kwa kazi ambazo hazikuwa na mwili katika kitengo cha sasa cha utafsiri, hii ilizalisha *mwili tupu wa kazi* (ufafanuzi usio na vitalu vya msingi). Ikiwa ufafanuzi halisi wa kazi hiyo hiyo ulionekana baadaye katika kitengo cha utafsiri, kijisabuni tupu *kilifunika* -- kiunganishi cha LLVM kingeona ufafanuzi mbili kwa alama hiyo hiyo na kuchagua ya kwanza (tupu). Utekelezaji halisi ulikuwa msimbo uliokufa.

**Marekebisho.** Uchunguzi wa awali wa AST uliongezwa ili kukusanya seti ya majina ya kazi ambazo *zina* miili. Katika `lower_function`, ukaguzi uliingizwa: ikiwa kazi haina mwili **na** kazi yenye jina sawa na mwili ipo mahali pengine, kijisabuni tupu kinarukwa na ufafanuzi halisi pekee ndio unaotolewa.

---

## 2. Kutolingana kwa Upana wa Hifadhi -- `i64` hadi `i32`

**Faili:** `src/codegen/llvm/mod.rs`

**Hitilafu.** Kibadala cha `Const::Int` kilikuwa kikijitokeza kama `i64` (baiti 8) katika LLVM IR, bila kujali aina lengwa. Wakati msimbo ulipogawa `Const::Int` kwa mgao wa rafu wa `i32`, elekezo la hifadhi liliandika baiti 8 kwenye baiti 4 za nafasi ya alloca. Baiti za ziada zilifurika kwenye vigezo vya karibu vya rafu, na kuharibu thamani zao kimya.

**Marekebisho.** Kidhibiti cha hifadhi kilibadilishwa kuuliza aina ya kielekezi cha mgao lengwa kupitia `LLVMGetElementType`, kisha kuingiza `LLVMBuildIntCast2` ili kukata (au kupanua) nambari kamili ili ilingane na upana wa lengwa kabla ya kutoa elekezo la hifadhi.

---

## 3. Kutolingana kwa Upana wa Hifadhi -- `i32` hadi `i64` (Sehemu za Muundo)

**Faili:** `src/codegen/llvm/mod.rs`

**Hitilafu.** Wakati wa kugawa kihalisi cha `N32` (i32, baiti 4) kwa sehemu ya muundo ya `N64` (i64, baiti 8), kidhibiti cha hifadhi kiliandika baiti 4 pekee. Baiti 4 za juu za sehemu lengwa zilihifadhi takataka yoyote iliyokuwa tayari kwenye kumbukumbu. Hali ya awali ilinda uigizaji tu kwa kesi ambapo upana wa chanzo ulikuwa *mkubwa zaidi kuliko* upana wa lengwa (`>`), hivyo upanuzi wa kupanua haukuwahi kutokea.

**Marekebisho.** Hali ya ulinganifu wa upana ilibadilishwa kutoka `>` hadi `!=`, ili ukataji (chanzo cha baiti 4 kwenye sehemu ya baiti 2) na upanuzi (chanzo cha baiti 4 kwenye sehemu ya baiti 8) vishughulikiwe. Msaidizi wa `StoreTyped` pia ulirekebishwa ili ulingane.

---

## 4. FieldAddr Ilipuuza Mpangilio

**Faili:** `src/codegen/llvm/mod.rs`

**Hitilafu.** Kidhibiti cha `FieldAddr` kilikokotoa vianzio vya baiti kwenye aina za mkusanyiko za LLVM kwa kujumlisha *ukubwa mbichi wa elementi* za sehemu zilizotangulia bila kutumia padding ya mpangilio. Mpangilio wa muundo wa LLVM unahitaji kila sehemu ipangiliwe kwa mpangilio wake wa asili; mkusanyaji alikuwa akiweka sehemu kwenye kianzio kisichopangiliwa.

Kwa mfano, kwa muundo `{i32, ptr, i64}`:
- `i32` kwenye kianzio 0 (ukubwa 4)
- `ptr` kwenye kianzio 4 (4 imepangiliwa kwa kielekezi cha baiti 8 kwenye 64-bit? Hapana -- inahitaji kianzio 8)
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

Kwa mpangilio (mpangilio wa juu = 8): `4 + 4(pad) + 8 + 8 + 4 + 4 + 4(pad) = 36...` kwa kweli:
- i32 kwenye 0..4
- padding 4..8
- i8* kwenye 8..16
- i64 kwenye 16..24
- i32 kwenye 24..28
- i32 kwenye 28..32
- padding ya mwisho kwa kizidishio cha 8: 32

Hivyo muundo ni baiti 32. `width_bytes()` bila padding ilirudisha 28 (4+8+8+4+4).

Hii ilisababisha allocas za `sret` (kurejesha muundo) kuwa na ukubwa mdogo. Wakati kazi iliporudisha muundo kwa kielekezi kilichofichwa, mpigaji aligawa nafasi kulingana na `width_bytes()` -- ndogo sana -- na mpigiwa aliandika kupita mgao.

**Marekebisho.** `width_bytes()` iliandikwa upya ili kukokotoa ukubwa kwa mpangilio sahihi: kianzio cha kila sehemu kinapangiliwa kwa mpangilio wa asili wa sehemu kabla ya kuiweka, na ukubwa wote unapigwa padding kwa kizidishio cha mpangilio wa juu wa sehemu ya muundo. Hii inalingana na upangaji wa `DataLayout` ya LLVM.

---

## 6. Aina za Safu za Ulimwengu Zilitangazwa kama `[Nxi8]` Badala ya `[Nxi32]`

**Faili:** `src/ir/mod.rs`, `src/ir/lower.rs`, `src/codegen/llvm/mod.rs`

**Hitilafu.** Muundo wa `IrGlobal` haukuwa na sehemu ya aina -- ulibeba urefu wa baiti pekee. Wakati mwisho wa LLVM ulipohitaji kutangaza safu ya ulimwengu, ilikisia aina ya elementi kutoka kwa urefu wa baiti: ikiwa safu ilikuwa zaidi ya baiti 8, kila mara ilikuwa `[Nxi8]`, kwa sababu taarifa pekee iliyopatikana ilikuwa hesabu ya baiti wote na mwisho ulidhani kila kitu kinachoweza kushughulikiwa kwa baiti kilikuwa na aina ya baiti.

Fikiria `N32 ast_aina[2048]` -- safu ya nambari kamili za baiti nne 2048 (jumla ya baiti 8192). Mwisho uliitangaza kama `[2048xi8]` (baiti 2048 pekee). Kila uandishi ulioorodheshwa kupitia `GEP i32` ulifikia kumbukumbu kwenye `msingi + faharisi * 4`, ambayo ilifurika haraka mgao wa baiti 2048 na kuharibu vigezo vya karibu vya ulimwengu.

**Marekebisho.** Sehemu ya `ty: IrType` iliongezwa kwa `IrGlobal`. Mwisho wa LLVM sasa unatumia `ir_type_to_llvm()` kutoa aina sahihi ya safu ya LLVM (k.m., `[2048xi32]`) kwa aina za elementi changamani, ukihifadhi `[Nxi8]` kwa data yenye aina ya baiti pekee.

---

## 7. Marekebisho ya Opaque Pointer ya LLVM -- Usawazishaji wa Aina ya Hifadhi/Mzigo (Uhamisho wa Linux)

**Faili:** `src/codegen/llvm/mod.rs`, `src/ir/lower.rs`, `src/ir/mod.rs`

**Hitilafu.** LLVM 22.1 kwenye Arch Linux inatumia opaque pointers kwa chaguo-msingi. Kazi ya API ya C ya `LLVMGetElementType`, ambayo usawazishaji wa upana wa hifadhi ulitegemea, inarudisha matokeo yasiyoaminika na opaque pointers. Hii ilisababisha:

- Vigezo vya `i64` vilikatwa hadi `i32` wakati wa kuhifadhiwa kwenye alloca, na kusababisha kufurika kwa bwawa katika `hifadhi_jina` na hitilafu za sehemu nasibu
- Nakala za sehemu za muundo katika `sogeza()` ziliharibu hali ya tokeni, na kufanya ulinganifu wa `== TOKENI_ISHARA` ushindwe
- Mizigo ilisoma upana usio sahihi kutoka kwa opaque pointers

**Marekebisho.**
- Hifadhi za vigezo zilibadilishwa kutoka `Instruction::Store` hadi `Instruction::StoreTyped`, ambayo hubeba IrType kwa uwazi na inapita API iliyopitwa na wakati
- Kidhibiti cha `Instruction::Store` cha kawaida kiliondoa usawazishaji wa upana wa `LLVMGetElementType` kabisa -- njia zote zinazohitaji upana tayari zinatumia `StoreTyped`
- `Instruction::Const(Const)` iliongezwa kwenye IR kwa vipatanishi vinavyobadilika, kuzuia migongano ya ValueId
- `emit()` ilibadilishwa kutumia `func.values.len()` inayobadilika badala ya `values_initial_len` tuli

---

## 8. Msimbo Baada ya Kama Hauwiani kwenye CFG -- Ufuatiliaji wa `actual_prev` Unakosa `BrCond`

**Faili:** `src/ir/lower.rs`

**Hitilafu.** Katika `lower_block`, baada ya kutekeleza `lower_if`, kitanzi cha taarifa kiliunganisha taarifa inayofuata moja kwa moja kutoka kwa block iliyorudishwa na `lower_if`. Kabla ya marekebisho, `lower_if` ilirudisha `merge_blk` -- block ya kuunganisha baada ya sharti na mwili wa `kama`. Hii ilifanya taarifa inayofuata ianze kutoka kwenye block ya kuunganisha, ambayo ilikuwa sahihi kwa mtiririko.

Lakini tatizo kubwa zaidi lilikuwa kwamba `lower_if` ilirudisha `cond_blk` baada ya kubadilisha muundo (marekebisho ya 8a), na `lower_block` haikuwa na mantiki ya kufuatilia block halisi ya mwendelezo. Baada ya sharti la `BrCond`, block inayofuata katika mnyororo inapaswa kuwa `merge_blk` (block ya kuunganisha), si `cond_blk` (block ya sharti). Bila ufuatiliaji huu, taarifa baada ya `kama` ilikuwa haiwezi kufikiwa kutoka kwenye njia za `then` na `else`, na block ya sharti ilianguka moja kwa moja kwenye taarifa inayofuata bila kupitia `merge_blk`.

Kwa kuongezea, vitalu vilivyomalizika kwa `Br(jikite)` (kitanzi cha kujirudia) havikuwa na urekebishaji wa kutosha. Baada ya kazi kukamilika, block iliyokuwa na kituo cha `Br(self)` ilibaki kwenye mzunguko usio na mwisho badala ya kurejea kwa `RetVoid` au `Ret` sahihi.

**Marekebisho.**

1. **`lower_if` inarudisha `cond_blk` badala ya `merge_blk`**:
   - `lower_block` sasa inaunganisha kianzio cha kitalu -> block ya sharti, si kianzio -> block ya kuunganisha.
   - Hii inahakikisha kwamba sharti linatathminiwa kabla ya mwili wowote wa `kama` au `sivyo`.

2. **`lower_block` inafuatilia `actual_prev` kutoka `BrCond`**:
   ```rust
   let actual_prev = match &self.func.blocks[stmt_blk.0].terminator {
       Terminator::BrCond(_, _, merge) => *merge,
       _ => stmt_blk,
   };
   ```
   Baada ya `kama`, taarifa inayofuata inaunganishwa kutoka `merge_blk`, si kutoka `cond_blk`. Hii inazuia msimbo usio na marejeleo (dead code) na inahakikisha mtiririko sahihi wa CFG.

3. **Urekebishaji wa kituo cha kujirudia (self-loop)**:
   Vitalu vyenye `Br(jikite)` hubadilishwa wakati wa ukamilishaji wa kazi:
   - Kwa kazi za `W0` (void): badilisha kwa `RetVoid`
   - Kwa kazi zenye thamani ya kurudi: badilisha kwa `Ret(0)` (thamani chaguo-msingi)
   Hii inahakikisha hakuna block iliyoachwa kwenye kitanzi kisicho na mwisho.

**Athari.** Marekebisho haya yalirekebisha utiririkaji wa udhibiti baada ya taarifa za `kama`. Kabla ya marekebisho, mchanganuzi wa kujikusanya ulizalisha CFG isiyo sahihi ambapo taarifa baada ya `kama` haikuwiani vizuri, na msimbo uliokuwa nyuma ya sharti ulikuwa msimbo uliokufa kwa sehemu. Pia ulirekebisha tabia ya kazi kukwama kwenye vitanzi visivyo na mwisho badala ya kurejea ipasavyo.

---

## 9. Alloca-in-Loop Inamaliza Rafu -- Alloca za Vigeu vya Ndani Zinapaswa Kuwa kwenye Kitalu cha Kuingia

**Faili:** `src/ir/lower.rs`

**Hitilafu.** Mbinu ya `lower_local_decl` ilikuwa ikitoa alloca (nafasi ya rafu) kwa vigeu vya ndani kwenye block ya sasa ya kuteremsha. Kwa mpangilio wa mstari, hii inafanya kazi kwa usahihi -- kila kigezo kina alloca yake kwenye block moja. Lakini ndani ya kitanzi (`wakati`), kila mzunguko uligawa *alloca mpya* kwenye block ya kitanzi, na alloca za zamani hazikuwekwa huru. Baada ya takriban mizunguko 524,288, rafu ya MB 8 iliisha na kugonga ukurasa wa ulinzi, na kusababisha SIGSEGV.

Uchunguzi wa gdb ulionyesha kuanguka kwenye `changanua()+54`:
```
movl %r11d, -0x10(%rax)    # jaribu kuandika kwenye ukurasa wa ulinzi
```

Rejesta zilionyesha:
- `rdx` = 36797 (anwani ya kianzio cha kuanguka inatarajiwa ndogo zaidi)
- `i` = 36794 (faharisi ya elementi -- inapaswa kuwa 0 kwa tokeni ya kitambulisho kimoja)
- Tofauti 36794 = 36797 - 3 ilionyesha kwamba urefu wa tokeni ulikuwa umeharibika

Chanzo cha msingi: Kila mzunguko wa kitanzi cha `wakati` katika `changanua()` uligawa baiti 16 za alloca kwa vigeu vya ndani kwenye block ya kitanzi, na alloca hizi hazikuwekwa huru wakati mzunguko ulipomalizika. Rafu ilikua hadi ikagonga ukurasa wa ulinzi.

**Marekebisho.** Suluhisho linatumia mbinu ya kupitisha mara mbili katika `lower_function`:

1. **Kupitisha awali (pre-pass) -- `collect_local_decls`**: Mbinu mpya inatembea AST ya mwili wa kazi na kukusanya nodi zote za `AST_TANGAZO` (matamko ya vigeu vya ndani) pamoja na aina zao zilizotatuliwa. Inajirudia kupitia `kushoto`, `kulia`, `tatu`, na `nne` ili kupata matamko yaliyowekwa ndani.

2. **Utoaji wa alloca mapema**: Baada ya kupitisha awali, `lower_function` inatoa Alloca kwa kila kigezo cha ndani kwenye *block ya kuingia* (entry block) -- kabla ya mwili wowote kuchakatwa. Hii inahakikisha kwamba alloca zote za vigeu vya ndani ziko kwenye block ya kuingia, ambako zinatolewa mara moja tu kwa mzigo wote wa kazi. Ramani ya `pre_allocated_locals: HashMap<i32, ValueId>` inahifadhi uhusiano kati ya faharisi ya nodi ya AST na alloca ValueId.

3. **`lower_local_decl` inatumia alloca iliyotanguliwa**: Badala ya kutoa `Instruction::Alloca` mpya, `lower_local_decl` sasa inaangalia ramani ya `pre_allocated_locals` na kutumia ValueId iliyotengwa mapema. Kwa miundo ya sret, inaendelea kutumia kielekezi cha sret moja kwa moja (hakuna alloca ya ziada inayohitajika).

4. **Uboreshaji wa `collect_constants`**: Kupitisha awali pia kuliboresha `collect_constants` ili kushughulikia `AST_KWELI` (Bool true), `AST_UONGO` (Bool false), na `AST_TUPU` (NullPtr) kwa kuweka constants zao kwenye `intern_const`, na kuhakikisha constants hizi hazijakosa wakati wa uteuzi wa kabla ya utoaji wa alloca.

5. **BrCond kwenye ufuatiliaji wa `actual_prev`**: Katika mnyororo wa ufuatiliaji wa block (sehemu ya utaratibu wa `actual_prev`), sasa `BrCond` inafuatiliwa kwa block ya kuunganisha (`merge`):
   ```rust
   Terminator::BrCond(_, _, merge) if *merge != b => { b = *merge; }
   ```
   Hii inahakikisha kwamba wakati block ina kituo cha masharti, ufuatiliaji unaendelea kutoka kwenye block ya kuunganisha badala ya kukwama.

**Athari.** Marekebisho haya yalirekebisha hitilafu ya SIGSEGV kwenye jaribio la K6 (kujikusanya kamili). Kabla ya marekebisho, binary ya kujikusanya ilianguka mara moja kwa SIGSEGV wakati wa kutekelezwa kwa sababu rafu iliisha ndani ya kitanzi cha `changanua()`. Baada ya marekebisho, binary ya kujikusanya iliendelea hadi kwenye hitilafu ya uchanganuzi tofauti (tokeni iliyobaki ya `}`), ambayo pia ilirekebishwa hatimaye.

---

## 10. Tamko la Mbele Halikutumia Nukta Mkato (Semicolon Leak)

**Faili:** `msingi/msambazaji.swa`

**Hitilafu.** Kazi ya `changanua_kazi` katika mchanganuzi wa kujikusanya haikutumia `;` iliyofuata matamko ya mbele (forward declarations / prototypes). Nukta mkato ilivuja hadi kiwango cha juu cha mchanganuzi, na kusababisha hitilafu ya "unexpected element" wakati wa kuchanganua faili kama `msomaji.swa` zilizokuwa na matamko ya mbele:

```swa
W0 ruka_nafasi_na_maelezo(Msomaji* m);
```

**Marekebisho.** Baada ya `changanua_kazi_mwili` kurudisha -1 (hakuna mwili), tumia `;` inayofuata.

---

## 11. Kitanzi cha `kwa` (For) Kinashindwa Kuchanganua Kiiniti

**Faili:** `msingi/msambazaji.swa`

**Hitilafu.** Mchanganuzi wa kitanzi cha `kwa` ulijaribu kuchanganua kiiniti kama usemi kupitia `changanua_usemi`. Lakini `N32 i = 0` ni tamko la ndani, si usemi. Mchanganuzi ulichanganua `N32` kama kitambulisho na kuacha `i = 0;` bila kutumiwa.

**Marekebisho.** Jaribu `changanua_taarifa_tangazo` (mchanganuzi wa matamko) kwanza kwa kiiniti cha `kwa`. Rudia kwa uchanganuzi wa usemi ikiwa mchanganuzi wa matamko unarudisha -1.

---

## 12. Hitilafu Mbalimbali za Mchanganuzi wa Kujikusanya

**Faili:** `msingi/msambazaji.swa`, `msingi/msomaji.swa`

### 12.1 `sogeza()` Inakosa Sehemu za Tokeni

`sogeza()` ilinakili sehemu 3 tu kati ya 5 za tokeni (aina, urefu, chanzo) lakini ikakosa `mstari` na `safu`. Hii ilisababisha nambari za mistari kuripotiwa kama 1 kila wakati na kuvunja ushughulikiaji wa maagizo ya `husisha`.

**Marekebisho.** Ongeza nakala za `mstari` na `safu` katika `sogeza()`.

### 12.2 Matumizi Maradufu ya `{`

`changanua_kazi_vigezo` ilitumia `{` iliyowekwa ndani wakati wa kuchanganua vigezo vya kazi -- `{` hii ilikuwa ya mwili wa kazi.

**Marekebisho.** Usitumie `{` katika `changanua_kazi_vigezo` ikiwa `{` si sehemu ya vigezo.

### 12.3 Hakuna Rudisha Hasi (Unary Minus)

Mchanganuzi haukushughulikia `rudisha -1;` -- alama ya `-` iliachwa bila kutumiwa.

**Marekebisho.** Ongeza ushughulikiaji wa unary minus katika mchanganuzi wa usemi.

### 12.4 Kufurika kwa Safu ya AST

Safu ya AST (AST_SAFU) yenye elementi 4096 haikutosha kwa faili zote za maktaba ya msingi zilizounganishwa. Hii ilisababisha kufurika kwa safu wakati wa kujaribu kuchanganua faili nyingi za `msingi/`.

**Marekebisho.** Ongeza ukubwa wa AST_SAFU hadi 16384.

---

## Muhtasari

| # | Hitilafu | Faili | Dalili | Sababu Kuu |
|---|---------|-------|--------|------------|
| 1 | Matamko ya mbele yanatoa vijisabuni vitupu | `src/ir/lower.rs` | Utekelezaji halisi umefunikwa | Hakuna ukaguzi wa kuwepo kwa mwili |
| 2 | Hifadhi ya i64 kwa alloca ya i32 | `src/codegen/llvm/mod.rs` | Vigezo vya karibu vya rafu vimeharibika | Hakuna usawazishaji wa upana kwenye hifadhi |
| 3 | Hifadhi ya i32 kwa sehemu ya muundo ya i64 | `src/codegen/llvm/mod.rs` | Takataka katika baiti 4 za juu | Ukaguzi wa upana ulitumia `>` badala ya `!=` |
| 4 | FieldAddr inapuuza mpangilio | `src/codegen/llvm/mod.rs` | Sehemu isiyo sahihi ya muundo imefikiwa | Vianzio vimejumlishwa bila mpangilio |
| 5 | width_bytes ya muundo inakosa padding | `src/ir/types.rs` | Allocas za sret ndogo sana | Hakuna padding ya mpangilio wa mwisho |
| 6 | Safu za ulimwengu zimeainishwa kama [Nxi8] | `src/ir/mod.rs`, `lower.rs`, `llvm/mod.rs` | Vigezo vya karibu vya ulimwengu vimeharibika | IrGlobal ilikosa sehemu ya aina |
| 7 | Opaque pointer inaharibu usawazishaji wa hifadhi | `src/codegen/llvm/mod.rs`, `src/ir/lower.rs`, `src/ir/mod.rs` | Hitilafu za sehemu nasibu, ulinganifu wa tokeni unashindwa | LLVMGetElementType haiaminiki na opaque pointers |
| 8 | Msimbo baada ya kama hauwiani kwenye CFG | `src/ir/lower.rs` | Taarifa baada ya kama ni msimbo uliokufa, vitanzi vya kujirudia | actual_prev haikufuatilia BrCond; self-loop haikurekebishwa |
| 9 | Alloca-in-loop inamaliza rafu | `src/ir/lower.rs` | SIGSEGV kwenye kitanzi cha wakati (rafu inaisha) | Alloca za vigeu vya ndani zinatolewa kwenye block ya sasa badala ya block ya kuingia |
| 10 | Tamko la mbele halitumii `;` | `msingi/msambazaji.swa` | Hitilafu ya "unexpected element" | Nukta mkato inavuja hadi kiwango cha juu |
| 11 | Kitanzi cha `kwa` kinashindwa | `msingi/msambazaji.swa` | `N32 i = 0` haichanganuliwi | Mchanganuzi unatumia usemi badala ya tamko |
| 12a | `sogeza()` inakosa sehemu za tokeni | `msingi/msambazaji.swa`, `msingi/msomaji.swa` | Nambari za mistari ni 1 kila wakati | Kunakili sehemu 3 kati ya 5 tu |
| 12b | Matumizi maradufu ya `{` | `msingi/msambazaji.swa` | Mwili wa kazi hauchanganuliwi | `{` inatumiwa katika vigezo |
| 12c | Hakuna unary minus | `msingi/msambazaji.swa` | `rudisha -1` haichanganuliwi | Alama ya `-` inaachwa |
| 12d | Kufurika kwa safu ya AST | `msingi/msambazaji.swa` | Kufurika kwa safu kwenye faili nyingi | AST_SAFU (4096) ni ndogo sana |
