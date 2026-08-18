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

## 3. Upeo wa tokeni: 65,536 — ukataji kimya [UKALI: JUU]

Mbegu ina MAX_TOKENS 65,536. Chanzo kikubwa kuliko hicho kinakatwa
**kimya kimya** — hakuna ujumbe wa kosa, mkusanyaji unarudisha 0 na
program iliyokatwa (ambayo inakosea wakati wa utekelezaji). Hii ni
darasa lilelile la kushindwa kama kukubali ingizo baya (sehemu ya 1):
kuendelea kimya hadi ELF halali lakini program mbaya. Ukubwa halisi:
wito ~7,280 wa kazi kwa mistari ya tokeni 9, na ~5,957 kwa tokeni 11
— bajeti ya tokeni ni ileile (~65,52x) katika kesi zote mbili, na
imepimwa kwa majaribio (si makadirio). Upeo wa AST ni 65,536 sawa —
kukizidi, mbegu sasa INALIA kwa sauti (sehemu ya 4).

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

## 4b. `endelea` ndani ya `kwa` inaruka mwanzo, si hatua [UKALI: CHINI]

Desugaring ya `kwa` kwenye mbegu (sawa na msambazaji wa .swa)
inaambatisha hatua mwishoni mwa mwili; `endelea` inaruka mwanzo wa
kitanzi — kama hatua ndiyo njia pekee ya kuendeleza kitanzi, kitanzi
kitazunguka milele (mfano: kwa (i=0; i<6; i=i+1) { kama (i==2)
endelea; }). Semantiki ya C (endelea → hatua) ni kazi ya kufuatilia.

## 5. Maneno halisi ni 32-bit signed [UKALI: CHINI]

Neno halisi `2147483648` linatafsiriwa kama `-2147483648` (biti
zinahifadhiwa, ishara inaenea) — na mkusanyaji wa mbegu NA dereva wa
Rust KWA USAWA (uthabiti, si mgawanyiko). Thamani kubwa zaidi ya
32-bit lazima zijengwe wakati wa utekelezaji.

## 6. Dereva wa Rust/LLVM: NJIA YA MSIMBO ISIYOTHIBITISHWA KATIKA MATUMIZI [UKALI: JUU]

FastISel ya LLVM inaacha kimyakimya vizuizi zaidi ya ~50 (k.m.
`ni_neno_muhimu` — 247): mkusanyaji wa LLVM unatoa .o BILA alama na
usio na uthibitisho. Hili SI "kipengele kinachokosekana" — ni njia
ya msimbo isiyothibitishwa inayotumika kwa mkusanyaji wa pili (K6 na
majaribio ya maktaba). Ukali wake unalingana na mende za bafa za
ukubwa wa kukisia: matokeo yanaweza kuwa mabaya KIMYA bila kosa.
Uthibitisho wa nje wa mnyororo (GNU ld dhidi ya toa_exe ya mbegu)
unafunika njia ya mbegu/exe/RELA pekee — SI mwisho wa LLVM.

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
