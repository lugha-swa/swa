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

KILICHOPIMWA 2026-08-27 (hati/uthibitisho-wa-lugha.md): mbegu bado
inakataa KIMYA (msimbo 1, hakuna ujumbe) kwa kesi kadhaa: kigezo
cha ulimwengu cha muundo, kianzilishi cha safu, safu ya pande
mbili, halisi za herufi, kazi yenye hoja 10, ukubwa(Muundo), na
moduli za libc (faili, nasibu, wakati). "Ingizo baya linalia kwa
sauti" bado si kamili — mkusanyaji wa uzalishaji pia anaanguka
(SEGV) kwa `sivyo` bila `kama`.

## 2. Mchanganuzi wa .swa: ingizo lililokatwa linaning'inia [IMEREKEBISHWA]

Ilikuwa: kitanzi cha changanua_kazi_vigezo hakikuwa na mwendo wala toka
kwa tokeni isiyotarajiwa (`{` baada ya `(`) — hang bila kikomo. Mabano
yasiyofungwa yalikubaliwa kimya (rc=0).

Sasa: tokeni isiyo aina kwenye vigezo inaweka kosa na kurudi; EOF ndani
ya mwili/bloku/hoja/orodha-ya-vianzisha/hali za chagua inaweka kosa —
yote yanarudisha 1 kwa sauti (`; KOSA: 1`). Kumbuka: kazi hizi sasa
zinakaribia kikomo cha vizuizi vya FastISel cha LLVM — mwisho wa LLVM
unabaki tete (kikomo kilichojulikana, kipengee 6).

## 3. Upeo wa tokeni: 262,144 [IMEREKEBISHWA — SASA INALIA]

Ilikuwa: chanzo kikubwa kuliko kikomo kinakatwa KIMYA — ELF halali
lakini program mbaya (JUU). Ilipimwa (kwa kikomo cha zamani cha
65,536): wito ~7,280 kwa mistari ya tokeni 9, ~5,957 kwa tokeni 11.

Sasa: lexer inalia `Hitilafu: chanzo kina tokeni nyingi mno` + exit 1
— hakuna ukataji wa kimya tena. Kikomo kiliinuliwa hadi 262,144
(MAX_TOKENS; vikomo viliinuliwa Agosti 2026, angalia kipengee 4).
Uthibitisho: chanzo cha wito 65,600 (juu ya kikomo) kinarudisha 1
kwa sauti; wito 65,500 (chini) inapita.

## 4. Mipaka ya majedwali — KOSA LAUTI, si uharibifu wa kimya [UKALI: CHINI]

Kila mpaka wa jedwali ndani ya mbegu unaangalia na KULIA kwa
sauti (`Hitilafu: ... limejaa` + exit 1) badala ya kuendelea kimya.
Vikomo viliinuliwa Agosti 2026 (4x) ili kupunguza mzunguko:

- jedwali la nje (MAX_EXTERNS 65,536) — kila wito wa mbele unachukua
  ingizo jipya bila dedup
- jedwali la RELA (MAX_RELOCS 65,536) — maeneo yote: mizigo ya
  ulimwengu, uhifadhi wa ulimwengu, tungo, nafasi za sret, na wito
- jedwali la fixup, la ulimwengu (MAX_GLOBALS 4096), la lebo
  (MAX_LABELS 65,536), na la AST (MAX_AST_NODES 262,144)
- chanzo kikubwa kuliko baiti 4,194,304 (MAX_SOURCE) — mbegu inalia
  kwa sauti badala ya kusoma sehemu tu
- tokeni (MAX_TOKENS 262,144), msimbo (TEXT_BUF_SIZE 1,048,576),
  na bwawa la herufi (STR_POOL_SIZE 1,048,576)

Uthibitisho: vikomo hufikiwa kwa KOSA LAUTI, si uharibifu wa kimya.

## 4b. `endelea` ndani ya `kwa` [IMEREKEBISHWA]

Ilikuwa: endelea inaruka mwanzo wa kitanzi, si hatua — kitanzi
kilizunguka milele kama hatua ndiyo njia pekee ya kuendelea.

Sasa: semantiki ya C — hatua imefungwa kwenye block-mini yenye
alama, uzalishaji_block hurekodi nafasi yake, na endelea inaruka
hapo. Uthibitisho: kwa (i=0; i<6; i=i+1) { kama (i==2) endelea; s++ }
hutoa s=5, na kesi ya endelea-pekee inapita. Mnyororo wa .swa UMEWIWA
tarehe 2026-08 (AST_BLOCK yenye alama -777777, lebo ya hatua
iliyotengwa mapema) — minyororo yote miwili sasa ina semantiki ya C
(uthibitisho: jaribio_exe_kujijenga sehemu ya 11 inaendesha kupitia
mbegu NA stage1).

## 4c. Desimali (D32/D64) — minyororo ya uzalishaji IMEREKEBISHWA [kilichobaki: CHINI]

Hali halisi (iliyothibitishwa 2026-08-20, kwa ushahidi wa kila mnyororo):
- Mnyororo wa mbegu: IMEREKEBISHWA — vitambulisho vya desimali
  (kigeuzi cha desimali hadi double kwenye lexer), AST_HALISI_D, hesabu
  za kuelea (addsd/subsd/mulsd/divsd), ulinganisho (ucomisd+setcc),
  na ukanushaji (mulsd kwa -1.0 — xorpd ya kumbukumbu ilionekana
  kuvunjika kwenye VM ya mtumiaji). Jaribio:
  jaribio_mende_60_desimali_mbegu.
- Mnyororo wa .swa (kujikusanya): IMEREKEBISHWA — bits_ya_d64_swa
  inakusanywa kwa usahihi na mbegu mpya (FPE ya zamani imetoweka);
  ABI kamili ya xmm0-xmm7 (hoja na kurejesha) inafanya kazi.
- Mnyororo wa LLVM (dereva wa Rust): IMEREKEBISHWA 2026-08 (suala
  #135) — jaribio_mende_135_desimali.

KILICHOPIMWA 2026-08-27 (hati/uthibitisho-wa-lugha.md): kauli ya
zamani "mbegu bado HAIJATEKELEZA ABI ya xmm kwenye wito wa kazi —
program za mbegu zenye kazi za D64 zinalia kwa sauti" SI KWELI tena.
Mbegu inatekeleza wito wa kazi za D64 (parameta NA kurudisha) kwa
usahihi (jaribio u062/u063 hutoa matokeo 1) bila kosa lolote. ABI yake
hutumia uhamisho wa GP (`movq rax, xmm0`; `movq xmm1, rcx`) si xmm
moja kwa moja — lakini inafanya kazi kwa hesabu, ulinganisho na
ukanushaji ndani ya ulimwengu wa D64.

Kilichobaki cha desimali (CHINI, kilichopimwa 2026-08-27):
- Kila mpaka kati ya D64 na nambari kamili umevunjika kwa minyororo
  yote miwili: kurudisha D64 kwenye kazi ya N32 (J1), operesheni
  mchanganyiko (J2), upakiaji wa ulimwengu wa D64 (J7) — jibu baya
  la kimya.
- D32 imevunjika (poromoko kwenye wito wa kazi; J9).
- `D64 % int` na `D64 << int` zinakubaliwa kimya na kutoa takataka.

## 5. Maneno halisi ni 32-bit signed [IMEREKEBISHWA kwa mnyororo wa .swa]

Ilikuwa: neno halisi `2147483648` linatafsiriwa kama `-2147483648`
(biti zinahifadhiwa, ishara inaenea) — thamani kubwa zaidi ya 32-bit
lazima zijengwe wakati wa utekelezaji.

Sasa (mnyororo wa .swa, uliothibitishwa 2026-08-25): mkusanyaji wa
kujikusanya unachanganua maneno halisi hadi N64 (mpaka wake wa
kikamilifu umepimwa 2026-08-27 — angalia hapa chini). `tokeni_kwa_nambari_n64`
inakusanya thamani kama N64; `changanua_primary` huhifadhi baiti 8
kwenye dimbwi la AST kwa thamani > 2147483647 (alama `ast_tiga=1`);
mkaguzi huweka usimbaji wa N64 (upana 64); `uzalishaji_nambari` hutoa
`mov rax, imm64`; na kianzio cha ulimwengu kinakili baiti 8 kutoka
dimbwi. Uthibitisho: `N64 x = 4294967296; rudisha x / 4294967296` hutoa 1
(juu) na `x % 4294967296` hutoa 0 (chini), kwa kigeu cha ndani NA cha
ulimwengu.

Kikomo kilichobaki (CHINI): mbegu (bootstrap pekee) na dereva wa Rust
bado zinachanganua maneno halisi kama 32-bit. Mbegu haihitaji maneno
ya N64 kwa kujikusanya (chanzo cha .swa kinatumia thamani chini ya
2^31 pekee) — lakini kama mkusanyaji wa kujitegemea, mbegu bado hukata
thamani kubwa (k.m. `4294967296` hukatwa hadi 0, na kusababisha FPE
katika ugawanyo).

KILICHOPIMWA 2026-08-27 (hati/uthibitisho-wa-lugha.md) — mpaka wa
mnyororo wa .swa ni finyu kuliko ilivyoandikwa:
- Ahadi "maneno halisi hadi N64 kamili" inashikilia kwa [2^31, 2^63)
  pekee. Thamani >= 2^63 zinakatwa KIMYA hadi biti 32 kwa minyororo
  yote miwili (uthibitisho wa mashine: `mov eax, 0xFFFFFFFF`) — J3.
- Halisi kubwa kama HOJA ya wito inavunjika: uzalishaji unakataa kwa
  kosa lisilo sahihi, hoja ya katikati inapoteza thamani (J4).
- Halisi ÷ halisi yenye N64 inaanguka kwa FPE (J5).
- Kigezo na ulimwengu wa N64 kwenye mnyororo wa .swa hufanya kazi
  (`N64 x = 4294967296; x / 4294967296` hutoa 1).

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

**KUFUNGWA (2026-09-05):** dereva wa Rust/LLVM umehamishiwa hazina ya
kumbukumbu [lugha-swa/swa-dereva](https://github.com/lugha-swa/swa-dereva)
(iliyohifadhiwa). Hazina kuu sasa ina lugha moja tu: Swa. Historia
kamili ya dereva ipo kwenye hazina kuu hadi commit 8fd71b2 (v0.1.0).

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

## 8. Hakuna mnyororo unaochakata `husisha { faili.swa }` [UKALI: CHINI — SASA INALIA]

Hakuna mkusanyaji (mbegu WALA mnyororo wa .swa) anayechambua faili
lililotajwa na `husisha { faili.swa }` — mstari unarukwa na
mchanganuzi. Hii ni kwa makusudi: mkusanyaji wa .swa unajijenga kwa
chanzo KILICHOUNGANISHWA (cat msingi/maktaba/*.swa na msingi/mkusanyaji/*.swa), na husisha C::xxx
bado inachakatwa kwa usahihi.

Hatari ya zamani: wito wa kazi kutoka faili "lililoingizwa" bila
kuunganisha ulikuwa ukitulia kimya kwa anwani 0 — mchakato
ulivunjika SEGV wakati wa utekelezaji. Sasa toa_exe inachapisha
`Hitilafu: kazi haijafafanuliwa: <jina>` na kutoka kwa msimbo 1
kwenye mnyororo wa uzalishaji (jaribio la kurejesha:
jaribio_mbegu_kazi_kukosa). Mbegu inakataa kimya katika kesi
kadhaa (tazama sehemu ya 1).

KILICHOPIMWA 2026-08-27 (hati/uthibitisho-wa-lugha.md): mnyororo
wa .swa (stage1+) pia HAUSOMI faili lililotajwa — ahadi ya zamani
"mkusanyaji wa .swa unawiwa viungo vya ndani" HAIKUBALIKI. Kesi
iliyopimwa: `husisha { kumbukumbu.swa }` + wito wa `weka_sifuri`
bila cat hutoa "kazi haijafafanuliwa: weka_sifuri" kwa minyororo yote
miwili. Ahadi ya vipimo-vya-lugha.md sehemu ya 8 ("mkusanyaji wa
.swa hulichakata faili lililotajwa") imerekebishwa pamoja na rekodi
hii.

Kanuni kwa watumiaji: faili lazima ziunganishwe kwanza
(`cat msingi/maktaba/mfuatano.swa msingi/maktaba/hesabu.swa program.swa`) —
hakuna mnyororo unaoingiza faili kwa sasa.

## 9. Rekodi ya uzingatiaji (2026-08-27)

Uthibitisho kamili wa lugha (mfumo wa aina, taarifa, viendeshaji,
miundo, kumbukumbu, maktaba; kesi ~487 zilizokusanywa na kuendeshwa
kwenye minyororo yote miwili) umeandikwa kwenye
`hati/uthibitisho-wa-lugha.md`: jedwali za uzingatiaji, kila jibu
baya kwa kipimo chake, poromoko zote, ukataaji unaokiuka hati, na
ambapo minyororo miwili inatofautiana. Mipaka iliyorekebishwa na
rekodi hiyo: kipengee 4c (ABI ya D64 kwenye mbegu — imefanya kazi),
kipengee 5 (N64 — kwa [2^31, 2^63) pekee), na kipengee 8 (husisha —
hakuna mnyororo unaoingiza faili).

## 10. `ukubwa(x)` kwenye mbegu: jina la muundo/kigezo lililo HASA `n8`/`n16`/`n32`/`n64`/`d64` (herufi ndogo) linatafsiriwa kimya kama aina ya msingi [UKALI: CHINI — TAHADHARI]

Tangu mbegu.bin kugandishwa upya 2026-09-24 kutambua herufi ndogo za
aina 6 za msingi (n8/n16/n32/n64/w0/d64 — angalia hali-ya-lugha.md),
mnyororo wa `ukubwa(x)` (builtin ya sizeof, `msingi/mbegu.s`) huchunguza
jina la hoja dhidi ya majina hayo 6 (herufi kubwa NA ndogo) KABLA ya
kutafuta jina la muundo. Tofauti na maeneo mengine ya kuchanganua aina
(`changanua_aina` — matangazo ya kigezo/paramu/kurudi/muundo/ulimwengu),
mnyororo huu WA `ukubwa` PEKEE HAUNA "backtrack" salama.

Athari: `ukubwa(x)` ambapo `x` ni jina la muundo (au kigezo) LILILO
HASA `n8`/`n16`/`n32`/`n64`/`d64` (herufi ndogo) litarudisha 1/2/4/8/8
(ukubwa wa aina ya msingi) badala ya kutafuta ukubwa halisi wa muundo
huo — bila kosa la sauti. Hii ni tofauti na `changanua_aina` (matangazo),
ambayo ina backtrack salama kwa vitambulisho halisi.

Kikomo hiki KIPO KWA MAKUSUDI (si uzembe): kuruhusu `ukubwa` kubackrack
kama `changanua_aina` kungehitaji mabadiliko makubwa zaidi ya mnyororo
wake (haufuati muundo wa "jaribu, kama haipo rudi nyuma" — ni
ulinganisho wa mfuatano wa moja kwa moja). Kwa kuwa jina la muundo/
kigezo lililo HASA `n8`/`n16`/`n32`/`n64`/`d64` (herufi tatu au chini,
herufi moja ya n/d ikifuatiwa na tarakimu TU) si jina la kawaida la
mradi huu (vitambulisho vya kweli huwa na maneno kamili ya Kiswahili),
hatari ni ndogo kivitendo — imethibitishwa: hakuna mgongano wowote wa
kweli kwenye faili 10 zinazojengwa moja kwa moja na mbegu.bin.

## 11. Lengwa ya WebAssembly (`stage1 --wasm`) — Awamu ya 1 pekee: N32, hakuna kumbukumbu [UKALI: KWA KUBUNI]

`msingi/mkusanyaji/uzalishaji_wasm.swa` (backend ya PILI ya
uzalishaji msimbo, sambamba na `uzalishaji.swa` ya x86-64) inatoa
moduli za WebAssembly kwa Awamu ya 1 TU: N32 pekee — hesabu,
ulinganisho, `&&`/`||` za mzunguko mfupi, kama/sivyo, wakati (+kwa/
vunja/endelea), wito wa kazi (ikiwemo kujirudia), rudisha. Ujenzi
wowote ulio nje ya wigo huu (vielekezi, safu, D64/D32, miundo,
vigezo vya ulimwengu, I/O, `tenga()`, `wito_wa_mfumo`, `chagua`)
UNAKATALIWA kwa sauti (`wasm_kosa`) wakati wa kukusanya kupitia
`wasm_kagua_enc_kigezo` (paramu/vigezo vya ndani/aina ya matokeo,
ikijumuisha `ast_tiga` — idadi ya safu, si `enc` tu) na ukaguzi wa
AST kwenye `wasm_usemi`/`wasm_taarifa` (kielekezi/safu kwa AST_TAJA,
miundo/vigezo-vya-ulimwengu/chagua kwa aina ya AST ya moja kwa moja).

Kikomo hiki KIPO KWA MAKUSUDI (angalia mpango wa Awamu ya 1 — hati ya
mradi) — si mdudu, ni wigo uliobuniwa kimakusudi kudumisha PR ya
kwanza ndogo. Awamu 2+ (kumbukumbu/`tenga()`/vielekezi kupitia
`memory.grow`, I/O kupitia Import section, D32/D64/miundo/vigezo vya
ulimwengu) ni kazi ya baadaye, haijaanzishwa.

TANGAZO LA MUUNDO PEKEE (bila kutumika kama aina ya kigezo popote)
LINAKUBALIWA KIMYA — sio kosa, ni la KUBUNI: tangazo la aina peke
yake halitoi msimbo wowote wa WASM, hivyo halina sababu ya kukataliwa
(ni MATUMIZI ya muundo kama aina ya kigezo ndiyo yanayokataliwa).
