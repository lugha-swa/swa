# Mipaka Inayojulikana ya Swa

Hati hii inaorodhesha mipaka inayojulikana ya mkusanyaji, kwa ukali.
"0% bootstrap gap" inahusu mnyororo wa UZALISHAJI (hakuna lugha nyingine
popote) — si uthabiti wa mchanganuzi dhidi ya ingizo baya. Uthabiti wa
makosa ni mhimili tofauti, bado wazi.

## 1. Mbegu: ingizo baya linakubaliwa au linasegfault [IMEREKEBISHWA]

Ilikuwa: `@#!` inakubaliwa kimya, `garba`/`kweli kweli kweli`
zinasegfault, chanzo lililokatwa linakubaliwa.

Sasa: herufi isiyojulikana inalia (`Hitilafu: herufi isiyojulikana`),
tokeni isiyojulikana kwenye kiwango cha juu inalia (`Hitilafu:
uliichanganuzi`), mabano yasiyofungwa (block/muundo/husisha) yanalia,
na kitanzi cha programu kina kinga ya mipaka ya mkondo wa tokeni.
Pia rekebisho la upande: maneno ya lexer sasa huandika token_line
(zamani: maneno yote yalikuwa na mstari 0 — husisha C:: ilivunjika).

Uthibitisho: majaribio ya kudumu kwenye jaribio_exe_kujijenga.

## 2. Mchanganuzi wa .swa: ingizo lililokatwa linaning'inia [IMEREKEBISHWA]

Ilikuwa: kitanzi cha changanua_kazi_vigezo hakikuwa na mwendo wala toka
kwa tokeni isiyotarajiwa (`{` baada ya `(`) — hang bila kikomo. Mabano
yasiyofungwa yalikubaliwa kimya (rc=0).

Sasa: tokeni isiyo aina kwenye vigezo inaweka kosa na kurudi; EOF ndani
ya mwili/bloku/hoja/orodha-ya-vianzisha/hali za chagua inaweka kosa —
yote yanarudisha 1 kwa sauti (`; KOSA: 1`). Kumbuka: kazi hizi sasa
zinakaribia kikomo cha vizuizi vya FastISel cha LLVM — mwisho wa LLVM
unabaki tete (kikomo kilichojulikana, kipengee 6).

## 3. Upeo wa tokeni: 65,536 [IMEREKEBISHWA — SASA INALIA]

Ilikuwa: chanzo kikubwa kuliko kikomo kinakatwa KIMYA — ELF halali
lakini program mbaya (JUU). Ilipimwa: wito ~7,280 kwa mistari ya
tokeni 9, ~5,957 kwa tokeni 11.

Sasa: lexer inalia `Hitilafu: chanzo kina tokeni nyingi mno` + exit 1
— hakuna ukataji wa kimya tena. Uthibitisho: chanzo cha wito 7,500
(juu ya kikomo) kinarudisha 1 kwa sauti; wito 7,000 (chini) inapita.

## 4. Mipaka ya majedwali — KOSA LAUTI, si uharibifu wa kimya [UKALI: CHINI]

Kila mpaka wa jedwali ndani ya mbegu sasa unaangalia na KULIA kwa
sauti (`Hitilafu: ... limejaa` + exit 1) badala ya kuendelea kimya:

- jedwali la nje (MAX_EXTERNS 16,384) — kila wito wa mbele unachukua
  ingizo jipya bila dedup (sahihi kiutendaji, imejaribiwa hadi wito
  7,000; kikomo hakifikiwi kwa sasa kwa sababu kikomo cha tokeni
  kinapiga kwanza)
- jedwali la RELA (MAX_RELOCS 16,384) — maeneo yote: mizigo ya
  ulimwengu, uhifadhi wa ulimwengu, tungo, nafasi za sret, na wito
- jedwali la fixup, la ulimwengu (MAX_GLOBALS 512), la lebo
  (MAX_LABELS 16,384), na la AST (MAX_AST_NODES 65,536)
- chanzo kikubwa kuliko baiti 1,048,576 (MAX_SOURCE) — mbegu inalia
  kwa sauti badala ya kusoma sehemu tu

Uthibitisho: toleo la jaribio lenye MAX_TOKENS/MAX_AST_NODES
lililoinuliwa hufikia kikomo cha nje kwa wito 16,500 na inalia
`Hitilafu: jedwali la nje limejaa` — si uharibifu tena.

## 4b. `endelea` ndani ya `kwa` [IMEREKEBISHWA]

Ilikuwa: endelea inaruka mwanzo wa kitanzi, si hatua — kitanzi
kilizunguka milele kama hatua ndiyo njia pekee ya kuendelea.

Sasa: semantiki ya C — hatua imefungwa kwenye block-mini yenye
alama, uzalishaji_block hurekodi nafasi yake, na endelea inaruka
hapo. Uthibitisho: kwa (i=0; i<6; i=i+1) { kama (i==2) endelea; s++ }
→ s=5, na kesi ya endelea-pekee inapita. Mnyororo wa .swa UMEWIWA
tarehe 2026-08 (AST_BLOCK yenye alama -777777, lebo ya hatua
iliyotengwa mapema) — minyororo yote miwili sasa ina semantiki ya C
(uthibitisho: jaribio_exe_kujijenga sehemu ya 11 inaendesha kupitia
mbegu NA stage1).

## 4c. Desimali (D32/D64) — minyororo ya uzalishaji IMEREKEBISHWA [kilichobaki: CHINI]

Hali halisi (iliyothibitishwa 2026-08-20, kwa ushahidi wa kila mnyororo):
- Mnyororo wa mbegu: IMEREKEBISHWA — vitambulisho vya desimali
  (kigeuzi cha desimali→double kwenye lexer), AST_HALISI_D, hesabu
  za kuelea (addsd/subsd/mulsd/divsd), ulinganisho (ucomisd+setcc),
  na ukanushaji (mulsd kwa -1.0 — xorpd ya kumbukumbu ilionekana
  kuvunjika kwenye VM ya mtumiaji). Jaribio:
  jaribio_mende_60_desimali_mbegu.
- Mnyororo wa .swa (kujikusanya): IMEREKEBISHWA — bits_ya_d64_swa
  inakusanywa kwa usahihi na mbegu mpya (FPE ya zamani imetoweka);
  ABI kamili ya xmm0-xmm7 (hoja na kurejesha) inafanya kazi.
- Mnyororo wa LLVM (dereva wa Rust): IMEREKEBISHWA 2026-08 (suala
  #135) — jaribio_mende_135_desimali.

Kikomo kilichobaki (CHINI): mbegu bado HAIJATEKELEZA ABI ya xmm
kwenye wito wa kazi — program za mbegu zenye kazi za D64 (hoja au
kurejesha kwa desimali) zinalia kwa sauti (`Hitilafu: D64 kwenye
wito wa kazi haisaidiwi bado na mbegu`). Mnyororo wa .swa unashughulikia
hali hiyo kikamilifu — tumia mkusanyaji wa .swa kwa program zenye
kazi za D64.

## 4d. Mbegu ina mende ya ukusanyaji kwa kazi ndefu zenye mchanganyiko wa N64 na vitanzi [UKALI: CHINI — kazi imegawanywa]

Iligunduliwa 2026-08-20: bits_ya_d64_swa ya zamani (kazi moja ndefu,
~60 taarifa, N64 nyingi na vitanzi vingi) ilikusanywa vibaya na mbegu
— FPE wakati wa kukusanya desimali (mgawanyiko uliokuwa ukisukuma
kigawanyo mara mbili). Taarifa zake zote zikijaribiwa PEKEE zilipita
— mzizi ni mchanganyiko wa ukubwa na umbo, bado haujachimbuliwa.

Njia ya kukwepa (iliyotumika): gawanya kazi kama hiyo katika kazi
mbili ndogo — bits_ya_d64_swa sasa inaita kamilisha_bits_swa.
Mnyororo mzima unafanya kazi: fixpoint + desimali kupitia stage1.

Kazi ya kufuatilia: kutafuta mzizi wa mende hiyo kwenye mbegu
(inashukiwa kuwa ukosefu wa usawa wa rafu ya CT wakati wa ukusanyaji
wa misemo ya N64 ndani ya vitanzi).

## 5. Maneno halisi ni 32-bit signed [UKALI: CHINI]

Neno halisi `2147483648` linatafsiriwa kama `-2147483648` (biti
zinahifadhiwa, ishara inaenea) — na mkusanyaji wa mbegu NA dereva wa
Rust KWA USAWA (uthabiti, si mgawanyiko). Thamani kubwa zaidi ya
32-bit lazima zijengwe wakati wa utekelezaji.

## 6. Dereva wa Rust/LLVM: njia ya MAJARIBIO yenye ulinzi wa sauti [IMEREKEBISHWA — JUU imefungwa 2026-08-20]

Zamani: FastISel ya LLVM (O0) iliacha kimyakimya vizuizi zaidi ya
~50 (k.m. `ni_neno_muhimu` — 247) — mkusanyaji wa LLVM ulitoa .o
BILA alama na usio na uthibitisho. Ukali wake ulilingana na mende za
bafa za ukubwa wa kukisia: matokeo mabaya KIMYA bila kosa.

Sasa:
- O0 (FastISel) inakataa kwa KOSA LAUTI kazi yenye vizuizi zaidi ya
  40 — hakuna ukataji wa kimya tena. Ujumbe unaonyesha njia ya
  kupita (with_opt_level(O1) au mnyororo wa mbegu/exe).
- O1 (Less) inatumia ISel kamili — majaribio YOTE ya dereva wa Rust
  (compile_and_verify, compile_file, run_msingi_test) sasa
  yanaendeshwa kwa O1 na kupita 227/227, pamoja na I/O ya faili.
- Mwisho wa LLVM unabaki MAJARIBIO (mnyororo wa uzalishaji ni
  mbegu/exe pekee) — lakini hakuna njia ya msimbo isiyothibitishwa
  inayotumika kimya tena.

Uthibitisho wa nje wa mnyororo (GNU ld dhidi ya toa_exe ya mbegu)
unafunika njia ya mbegu/exe/RELA — SI mwisho wa LLVM; ndiyo maana
mwisho wa LLVM unabaki MAJARIBIO, si kitu cha uzalishaji.

**UAMUZI (uliojaribiwa 2026-08-19):** mwisho wa LLVM umeSHUSHWA hadhi
kuwa wa MAJARIBIO (experimental). Ushahidi wa zamani: O0 inakata vizuizi >~50
kimya; O1/O2 zinakusanya mkusanyaji mzima (alama zipo) lakini
mfumo wa faili unashindwa kwenye mnyororo kamili; uthibitishaji wa
moduli unashindwa kwa desimali. Wigo uliojaribiwa na unaofanya kazi
ni majaribio ya maktaba ya K-series (program ndogo). Mnyororo wa
uzalishaji ni mbegu/exe PEKEE.

## 7. Kwanza haina chanzo chenye maelezo [UKALI: TAARIFA]

Kwanza (msingi/kwanza.bin, baiti 393) ni mzizi usioweza kupunguzwa wa
uaminifu — baiti mbichi bila chanzo chenye maelezo kwenye repo.
Inajithibitisha kwa kujizalisha (kwanza.hex -> kwanza.bin) na ndiyo
inayoganda mbegu.

## Nini HAKIKO katika mipaka

- Mnyororo wa uzalishaji: kwanza -> mbegu -> stage1-exe -> stage2-exe
  == stage3-exe, bila gcc/ld/clang/libc popote. Hakuna lugha nyingine.
- RELA za mbegu zinakubaliana na GNU ld (rejea huru) sawa kwa baiti.
- Utekelezaji wa makosa: exit 0/1 inafanya kazi kwa makosa
  YANAYOTAMBULIWA.

## 8. Mbegu haiwi viungo vya ndani vya `husisha { faili.swa }` [UKALI: CHINI — SASA INALIA]

Mbegu HAICHAMBUZI faili lililotajwa na `husisha { faili.swa }` —
linarukwa na mchanganuzi. Hii ni kwa makusudi: mkusanyaji wa .swa
unajijenga kwa chanzo KILICHOUNGANISHWA (cat msingi/*.swa), na
husisha C::xxx bado inachakatwa kwa usahihi.

Hatari ya zamani: wito wa kazi kutoka faili "lililoingizwa" bila
kuunganisha ulikuwa ukitulia kimya kwa anwani 0 — mchakato
ulivunjika SEGV wakati wa utekelezaji. Sasa toa_exe inachapisha
`Hitilafu: kazi haijafafanuliwa: <jina>` na kutoka kwa msimbo 1
(jaribio la kurejesha: jaribio_mbegu_kazi_kukosa).

Kanuni kwa watumiaji wa mbegu: faili lazima ziunganishwe kwanza
(`cat msingi/mfuatano.swa msingi/hesabu.swa program.swa`), au
tumia mkusanyaji wa .swa (stage1+) ambao unawiwa viungo vya ndani.
