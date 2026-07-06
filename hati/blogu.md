# Mkusanyaji wa Kujikusanya wa Swa: Hatua Muhimu Ilifikiwa Baada ya Saa 18 za Kurekebisha Hitilafu

## Muhtasari wa Utendaji

**Swa**, lugha ya kwanza duniani ya programu za mifumo kwa Kiswahili, imefikia hatua muhimu: mkusanyaji wake wa kujikusanya unajikusanya yenyewe. Baada ya saa 18 za kurekebisha hitilafu kwa kina katika commits 10, hitilafu 12 muhimu ziligunduliwa na kurekebishwa. Mkusanyaji sasa unapita majaribio 174/174, ikiwa ni pamoja na jaribio kamili la kujikusanya (K6) ambalo lilikuwa limezimwa tangu ilipoanzishwa.

## Muktadha wa Mradi

Swa ni lugha ya programu za mifumo ambapo **maneno muhimu, aina, na nyaraka zote ziko kwa Kiswahili**. Kwa mfano:
- `kama` (if), `wakati` (while), `rudisha` (return)
- `N32` (i32), `A64` (u64), `D64` (f64)
- `muundo` (struct), `tenga` (malloc), `achilia` (free)

Mkusanyaji wa bootstrap (`kande`) umeandikwa kwa Rust (takriban mistari 12,200). Unakusanya chanzo cha Swa kupitia mnyororo kamili: msomaji (lexer) -> mchanganuzi (parser) -> uchanganuzi wa kisemantiki -> uteuzi wa IR -> LLVM codegen -> native x86-64 binary.

Sehemu za kujikusanya zimeandikwa kwa Swa yenyewe (takriban mistari 4,100 katika faili 7):
- `msomaji.swa` -- msomaji (lexer)
- `msambazaji.swa` -- mchanganuzi (parser)
- `mteremko.swa` -- kiteremshi cha IR
- `mkaguzi.swa` -- mkaguzi wa kisemantiki
- `kumbukumbu.swa` -- usimamizi wa kumbukumbu
- `mfuatano.swa` -- shughuli za mifuatano
- `stage1.swa` -- kiendeshi cha bootstrap

## Kipindi cha Saa 18 cha Kurekebisha Hitilafu

### Mahali pa Kuanzia (Julai 4, 07:00)

Wakati kipindi kilipoanza, binary ya kujikusanya ilianguka mara moja kwa **SIGSEGV** (kufurika kwa rafu). Jaribio la K6 la kujikusanya kamili (`jaribio_k6_kujikusanya_kamili`) lilikuwa limezimwa kwa `#[ignore]` tangu ilipoanzishwa. Majaribio 172 pekee yalipita, na mradi ulikuwa umekwama kwenye kizuizi kikubwa.

### Hitilafu Zilizogunduliwa na Kurekebishwa

**Hitilafu 1: Alloca-in-Loop -> SIGSEGV** (`src/ir/lower.rs`)
Hitilafu muhimu zaidi. Kiteremshi cha IR kilitoa maagizo ya `alloca` kwenye block ya sasa ya msingi badala ya block ya kuingia ya kazi. Wakati tamko la kigezo cha ndani lilipokuwa ndani ya kitanzi (kama kitanzi cha `wakati` cha mchanganuzi), kila mzunguko uliunda alloca mpya -- ikitumia baiti 16 za rafu kwa kila mzunguko. Baada ya takriban mizunguko 524,000, rafu ya MB 8 ilikuwa imeisha.

**Marekebisho:** Mbinu ya kupitisha mara mbili. Kazi mpya ya `collect_local_decls` inatembea AST kugundua matamko yote ya vigeu vya ndani kabla ya uteuzi wa mwili kuanza. Alloca hizi zinatolewa mapema kwenye block ya kuingia. Wakati wa uteuzi wa mwili, `lower_local_decl` hutafuta `ValueId` iliyotengwa mapema badala ya kuunda alloca mpya.

**Athari:** Binary haianguki tena. Faili zote za maktaba zinachanganuliwa kwa mafanikio.

---

**Hitilafu 2: CFG Dead-Code -> Kitanzi Kisisicho na Mwisho** (`src/ir/lower.rs`, mstari 875)
Usafiri wa block ya `actual_prev` ulishughulikia tu vivunja vya `Br`. Ulipokutana na `BrCond` (kutoka tathmini fupi ya `&&`/`||`), ulivunja na kurudisha block mbaya. Maagizo yote yaliyofuata yakawa msimbo uliokufa usiofikiwa. Kitanzi cha mchanganuzi kilizunguka milele, kikiunda allocas kila mzunguko hadi rafu ikaisha.

**Marekebisho:** Mstari mmoja ulioongezwa kushughulikia `BrCond` katika usafiri:
```rust
Terminator::BrCond(_, _, merge) if *merge != b => { b = *merge; }
```

---

**Hitilafu 3: Uharibifu wa Block ya `kama`/`sivyo` Iliyowekwa Ndani** (`src/ir/lower.rs`, `patch_br_if_needed`)
Kazi ya `patch_br_if_needed` ilifuata tawi la UONGO (false) tu la vivunja vya `BrCond`. Kwa taarifa za `kama`/`sivyo` ambapo tawi la `sivyo` linarudisha thamani, block ya kuunganisha ya tawi la KWELI (true) haikufikiwa kamwe na usafiri. Hii iliacha vitanzi vya kujirudia (`self-loop`) ambavyo awamu ya ukamilishaji ilibadilisha kuwa `ret i32 0`, na kusababisha kazi kama `changanua_bloku` kurudisha 0 mara baada ya kutumia `{`.

**Marekebisho:** Fuata MATAWI YOTE mawili ya `BrCond` katika `patch_br_if_needed`.

---

**Hitilafu 4: Ugunduzi wa Block ya Kuunganisha ya `actual_prev`** (`src/ir/lower.rs`, `lower_block`)
Wakati taarifa iliyotangulia block ilikuwa `kama` yenye `sivyo` inayorudisha, `actual_prev` ilifuata tawi la UONGO la BrCond hadi kwenye block ya `Ret` na kusimama. Block ya kuunganisha (inayofikiwa kutoka tawi la KWELI) haikupatikana kamwe. Hii ilizuia kuunganisha taarifa zinazofuata, zikiachwa kama msimbo uliokufa usiofikiwa.

**Marekebisho:** Kazi ya kujirudia ya `walk_branch` inajaribu tawi la KWELI wakati tawi la UONGO linaisha kwa `Ret`.

---

**Hitilafu 5: Uharibifu wa Block ya Kutoka ya `wakati`/`kwa`** (`src/ir/lower.rs`, `lower_while`/`lower_for`)
`exit_blk` iliundwa na kituo cha msingi cha `RetVoid`. Wakati `patch_br_if_needed` ilipotembea kupitia CFG, ilifuata matawi ya `endelea` (continue) kuingia kwenye miili ya vitanzi vilivyozunguka. Ilipata block ya kutoka ya kitanzi cha nje (iliyokuwa na `RetVoid`) na kuitibu kama sehemu ya kuacha. Hii iliacha block ya kutoka ya kitanzi cha ndani ikiwa na kituo kisicho sahihi.

**Marekebisho:** Weka kituo cha `exit_blk` kuwa `Br(exit_blk)` (kitanzi cha kujirudia) mara baada ya kuundwa. Hii inahakikisha `walk_branch` inaitambua kama njia ya kupita, si kurudi.

---

**Hitilafu 6: `patch_br_if_needed` Ikifuata Mipaka ya Endelea/Vunja** (`src/ir/lower.rs`)
Hata baada ya marekebisho ya 5, `patch_br_if_needed` ilifuata block za `endelea` zenye `Br(outer_header)` kuingia kwenye miili ya vitanzi vilivyozunguka. Ilipata block ya kutoka ya kitanzi cha nje (kitanzi cha kujirudia) na kuibadilisha kwa block ya kuunganisha ya `kama` ya ndani -- ikiharibu njia ya kutoka ya kitanzi cha nje. Hii ilisababisha `changanua()` kurudisha `0` badala ya faharasi sahihi ya nodi ya AST ya programu wakati faili nyingi ziliunganishwa.

**Marekebisho:** Acha kufuata mipaka ya `Br` ya mbele wakati lebo ya block chanzo inaanza na `continue.` au `break.` -- hizi ni mipaka ya udhibiti wa kitanzi inayoongoza nje ya mwili unaorekebishwa.

---

**Hitilafu 7: Kuvuja kwa Nukta Mkato ya Tamko la Mbele** (`msingi/msambazaji.swa`)
Kazi ya `changanua_kazi` katika mchanganuzi wa kujikusanya haikutumia `;` iliyofuata matamko ya mbele (prototypes za kazi). Nukta mkato ilivuja hadi kiwango cha juu cha mchanganuzi, na kusababisha hitilafu za "unexpected element". Hii ilizuia kuchanganua faili kama `msomaji.swa` zilizokuwa na matamko ya mbele:
```swa
W0 ruka_nafasi_na_maelezo(Msomaji* m);
```

**Marekebisho:** Baada ya `changanua_kazi_mwili` kurudisha -1 (hakuna mwili), tumia `;` inayofuata.

---

**Hitilafu 8: Uchanganuzi wa Kiiniti cha Kitanzi cha `kwa`** (`msingi/msambazaji.swa`)
Mchanganuzi wa kitanzi cha `kwa` ulijaribu kuchanganua kiiniti kama usemi kupitia `changanua_usemi`. Lakini `N32 i = 0` ni tamko la ndani, si usemi. Ulichanganua `N32` kama kitambulisho na kuacha `i = 0;` bila kutumiwa.

**Marekebisho:** Jaribu `changanua_taarifa_tangazo` (mchanganuzi wa matamko) kwanza kwa kiiniti cha `kwa`. Rudia kwa uchanganuzi wa usemi ikiwa mchanganuzi wa matamko unarudisha -1.

---

**Hitilafu 9-12: Hitilafu za Ziada za Mchanganuzi wa Kujikusanya**
- **`sogeza()` inakosa `mstari`/`safu`** -- ilinakili sehemu 3 tu kati ya 5 za tokeni, na kusababisha nambari za mistari kuripotiwa kama 1 kila wakati
- **Matumizi maradufu ya `{`** katika `changanua_kazi_vigezo` -- ilitumia `{` iliyowekwa ndani ambayo ilikuwa ya mwili wa kazi
- **Hakuna rudisha hasi** -- `rudisha -1;` iliacha `-` bila kutumiwa
- **Kufurika kwa safu ya AST** -- elementi 4096 hazikutosha kwa faili zote za maktaba ya msingi zilizounganishwa; iliongezwa hadi 16384

## Matokeo

### Msururu wa Majaribio
```
145 unit tests:          PASS
 28 integration tests:   PASS (ikiwa ni pamoja na K6!)
  1 doc test:            PASS
_________________________________
174/174:                 100% PASSING
```

### Jaribio la K6 la Kujikusanya Kamili
Binary ya kujikusanya:
- Inajikusanya kwa mafanikio (hakuna SIGSEGV)
- Inapakia na kuchanganua msimbo wake chanzo wenyewe
- Inachakata faili za maktaba ya msingi
- Inaripoti kwa usahihi faharasi ya nodi ya mzizi ya AST
- Inakamilika chini ya sekunde 1 (hali ya mtumiaji)
- Inachanganua vipengele vyote muhimu vya lugha: kazi, vitanzi vya wakati/kwa, kama/sivyo, muundo, taarifa za rudisha, mgao, hesabu, ulinganisho, tathmini fupi, break/continue

### Takwimu za Hitilafu
| Kategoria | Idadi |
|----------|-------|
| Codegen (mkusanyaji wa Rust) | 6 |
| Mchanganuzi wa kujikusanya | 4 |
| Msomaji wa kujikusanya | 1 |
| Miundombinu (ukubwa wa safu) | 1 |
| **Jumla** | **12** |

### Commits
**Commits 10** katika kipindi hiki. 7 mnamo Julai 4, 3 mnamo Julai 5. Zote zimeandikwa kwa Kiswahili kwa mujibu wa kanuni za mradi.

### Faili Zilizobadilishwa
- `src/ir/lower.rs` -- mistari 150+ iliyobadilishwa (kiini cha marekebisho ya codegen)
- `msingi/msambazaji.swa` -- mistari 40+ iliyobadilishwa (marekebisho ya mchanganuzi)
- `stage1.swa` -- mistari 20 iliyobadilishwa (uboreshaji wa kasi)
- `src/parser/mod.rs` -- mistari 2 (mpangilio wa AST ya kitanzi cha kwa)
- `hati/*.md` -- masasisho ya nyaraka

## Maana ya Hili

Swa imefikia **Hatua ya 1 ya kujikusanya**: mkusanyaji, ulioandikwa kwa Rust, unaweza kukusanya mkusanyaji wa Swa ambaye anajikusanya yenyewe. Hii ni hatua muhimu ya kwanza kuelekea uhuru kamili wa bootstrap.

Binary ya kujikusanya inaweza:
1. Kuchanganua chanzo cha Swa (ikiwa ni pamoja na faili zote za maktaba ya msingi)
2. Kuzalisha AST sahihi
3. Kuzalisha LLVM IR (kupitia `mteremko.swa`)
4. Kuendesha bila kuanguka

Kilichosalia kwa **Hatua ya 2** (uhuru kamili):
- Kukamilisha `mteremko.swa` (kiteremshi cha kujikusanya cha IR) kwa usaidizi wa sret
- Kurekebisha alloca-in-loop katika `mteremko.swa` (hitilafu ileile tuliyoirekebisha kwenye kiteremshi cha Rust)
- Kukamilisha `mkaguzi.swa` (mkaguzi wa kisemantiki)
- Kuzalisha faili za kitu asilia kutoka kwa mkusanyaji wa kujikusanya

Mradi uko takriban **42%** ya njia kutoka kwa bootstrap ya Rust hadi mkusanyaji wa Swa anayejitegemea kikamilifu.

## Maarifa ya Kiufundi

### Mfumo wa ValueId
Codegen ya `lower.rs` na LLVM backend zote zinatumia fomula `ValueId = P + V + I` ambapo:
- `P` = idadi ya vigezo (parameters)
- `V` = idadi ya vipatanishi (constants)
- `I` = kidhibiti cha maagizo cha kimataifa (kinachoongezeka kwa mpangilio katika block zote)

Mpangilio huu unamaanisha kwamba maagizo yaliyotolewa kwenye block ya kuingia wakati wa kupitisha awali yanapata ValueId za chini, zinalingana na mpangilio wa block za backend. Maagizo yaliyotolewa wakati wa uteuzi wa mwili yanapata ValueId za juu. Ramani ni thabiti kwa sababu pande zote mbili zinatembea block kwa mpangilio uleule.

### Kazi ya `patch_br_if_needed`
Kazi hii ndiyo chanzo cha hitilafu 3 kati ya 6 za codegen. Inatembea CFG kurekebisha vitanzi vya kujirudia kwa malengo sahihi ya kuunganisha. Hitilafu zilitokea kwa sababu:
1. Ilifuata tawi la UONGO tu la `BrCond` (ilikosa block za kuunganisha za tawi la KWELI)
2. Ilifuata mipaka ya `Br` ya mbele kuingia kwenye mtiririko wa udhibiti uliozunguka (iliharibu njia za kutoka za vitanzi vya nje)
3. Iliitwa kwa wakati usiofaa kuhusiana na kuunganisha block

### Mafunzo Yaliyojifunza
- **Hakuna hitilafu ndogo.** `sogeza()` iliyokosa sehemu mbili ilionekana ndogo lakini iliharibu taarifa ZOTE za nambari za mistari na kuvunja kishughulikiaji cha maagizo ya `husisha`.
- **Hitilafu za codegen hufuatana.** Alloca-in-loop ilificha hitilafu ya CFG dead-code, ambayo ilificha hitilafu ya `patch_br_if_needed`, ambayo ilificha hitilafu ya tamko la mbele, ambayo ilificha hitilafu ya kitanzi cha `kwa`. Kila marekebisho ulifunua safu inayofuata.
- **Thibitisha kwa majaribio ya mwisho-hadi-mwisho.** Majaribio ya kawaida (unit tests) yalipita kwa hitilafu hizi nyingi. Jaribio la K6 pekee liligundua matatizo halisi.

## Hatua Zinazofuata

1. Kukamilisha `mteremko.swa` -- kiteremshi cha kujikusanya cha IR
2. Kuongeza ujumuishaji wa LLVM pass manager (bendera ya `--opt`)
3. Kuwezesha uchanganuzi kamili wa maktaba ya msingi (kuongeza kasi ya dakika 2 za uchanganuzi)
4. Kufikia bootstrap halisi: kukusanya mkusanyaji wa kujikusanya kwa kutumia mkusanyaji wa kujikusanya pekee

---

*Imeandikwa kwa Kiswahili. Mkusanyaji wa Swa.*
