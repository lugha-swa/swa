# Matokeo ya vipimo vya utendaji: Swa dhidi ya C

Vipimo hivi ni tofauti na vile vya awali vya kikao hiki -- vile
vilijengwa mara MBILI kabla ya hii lakini vilipotea kila wakati kwa
sababu havikuwahi kuingizwa kwenye git (viliachwa kwenye saraka ya
kazi ambayo baadaye ilibadilishwa/kurudishwa). Vipimo hivi
VIMEINGIZWA KWENYE GIT moja kwa moja ili kuwa msingi wa kudumu wa
kupima kazi yoyote ijayo ya utendaji, si kujengwa upya kila wakati.

## Muktadha wa mkusanyaji

Ujenzi wa `stage1` ulithibitishwa dhidi ya suti kamili KABLA ya
kupima (329/329 yamefaulu). Mkusanyaji HAUNA bendera yoyote ya
uboreshaji (`-O` na kadhalika) -- `--exe` na `--jit` PEKEE
zinakubaliwa (imethibitishwa kwa kusoma `stage1.swa` moja kwa moja,
si kudhani).

## Maunzi

- CPU: Intel(R) Core(TM) i7-8650U @ 1.90GHz (msingi 4, nyuzi 8),
  MHz ya juu 4200 -- **mizani ya masafa (frequency scaling)
  IMEWASHWA**, chanzo kinachowezekana cha kelele ya vipimo.
  Fikiria matokeo haya kama makadirio, si vipimo vya maabara
  sahihi kabisa.
- RAM: 15Gi jumla, ~9-11Gi ilikuwa huru wakati wa vipimo.
- GCC: 16.2.1 (2026-08-10), `-O2`.

## Mbinu

Kila kipimo kina toleo la Swa na la C, zote mbili zikitekeleza
ALGORITHM MOJA HASA kwenye DATA MOJA HASA (LCG yenye mbegu ileile
kwa vipimo vinavyohitaji data ya kubahatisha), na kila moja
imethibitishwa kutoa MATOKEO SAWA KABISA kati ya lugha mbili kabla
ya kuamini muda wake -- kipimo cha haraka lakini chenye makosa
hakina thamani. Kila binary lilipimwa mara 7, wastani wa kati
(median, si wastani wa hesabu) umeripotiwa pamoja na kiwango cha
chini/juu ili kuonyesha kelele halisi badala ya kuificha.

## Matokeo

| Kipimo | Swa (wastani wa kati) | Swa (chini-juu) | C -O2 (wastani wa kati) | Uwiano (Swa/C) |
|---|---|---|---|---|
| fibonacci(35), urudiaji halisi | 107ms | 105-114ms | 17ms | 6.3x |
| kupanga (quicksort, nambari milioni 1) | 280ms | 272-311ms | 85ms | 3.3x |
| matriki (kuzidisha matriki 300x300, D64) | 190ms | 183-222ms | 13ms | 14.6x |
| heshi (djb2 kwenye mfuatano milioni 2, kila mmoja tofauti) | 972ms | 960-1007ms | 126ms | 7.7x |
| mzunguko_mchezo (mzunguko wa mchezo, mifumo 8, mizunguko 200,000) | 20ms | 19-27ms | 3ms | 6.7x |

Matokeo yote yamethibitishwa SAWA KABISA kati ya Swa na C (thamani
halisi za matokeo, si tu "haikuanguka") kabla ya kuamini muda wowote
hapo juu -- angalia chanzo cha kila kipimo (`swa.swa`/`c.c` katika
kila saraka) kwa maelezo ya uthibitisho.

## Ugunduzi muhimu: `matriki` (D64) sasa inawezekana kupimwa KWA MARA YA
KWANZA

Kikao hiki kilichotangulia kilijaribu kupima kuzidisha kwa matriki
lakini ililazimika kutumia namba kamili (integer) badala ya D64 kwa
sababu `D64*` (kielekezi kwenda desimali) haikufanya kazi KABISA
(lugha-swa/swa#181). Mdudu huo sasa umerekebishwa (PR #213/#221),
kwa hiyo kipimo hiki sasa kinatumia D64 HALISI kwenye safu
zilizotengwa (`tenga`), kama ilivyokusudiwa awali. Hii ndiyo
tovuti pekee ambapo Swa inabaki karibu na uwiano wa mwanzo wa kikao
(3.7x-17.6x) -- 14.6x ni mbaya zaidi kati ya vipimo vitano, ikiendana
na ugunduzi wa awali kuwa kitanzi kigumu cha nambari (bila SIMD/
vectorization) ndicho kinachoathiriwa zaidi na ukosefu wa ugawaji
wa rejesta wa jumla (register allocation ya sasa inashughulikia
vigezo vya ndani na paramu PEKEE, si matokeo ya kati ya usemi --
angalia majadiliano ya awamu inayofuata).

## LICM (loop-invariant code motion) -- kipimo halisi cha kabla/baada

Awamu hii inahamisha usemi wa hesabu/linganisho/biti usiobadilika
(mf. `i * n` ndani ya fahirisi ya safu `a[i*n+j]`) nje ya kitanzi cha
"wakati", ukiuhesabu mara moja badala ya kila mzunguko. Wigo
umepunguzwa kwa makusudi (angalia maoni kamili juu ya `licm_boresha`,
`msambazaji.swa`): mgawanyo/modulo hazihamishwi kamwe, usemi wowote
unaogusa kumbukumbu (safu/nyoosha/muundo) haustahili kamwe, kitanzi
chenye wito wowote hakigombei kabisa, na uhamishaji unafanyika TU
kwa taarifa za ngazi ya juu za mwili wa kitanzi (ugawaji au tamko
lenye kianzio) -- si ndani ya matawi ya "kama".

Mbinu: mkusanyaji wa KABLA na wa BAADA ya mabadiliko haya ulijengwa
KWENYE KIKAO KIMOJA HASA (angalia tahadhari ya kelele hapa chini),
kila mmoja akikusanya `vipimo/matriki/swa.swa` na
`vipimo/mzunguko_mchezo/swa.swa`. Matokeo ya programu yalithibitishwa
SAWA KABISA (baiti kwa baiti ya `stdout`) kati ya toleo la kabla na
la baada KABLA ya kuamini muda wowote. Mara 7 kila moja, wastani wa
kati umeripotiwa.

| Kipimo | Kabla (wastani wa kati) | Baada (wastani wa kati) | Mabadiliko |
|---|---|---|---|
| matriki (300x300, D64) | 188ms | 164ms | ~13% haraka zaidi |
| mzunguko_mchezo (mifumo 8, mizunguko 200,000) | 20ms | 19ms | hakuna mabadiliko ya maana (ndani ya kelele) |

**matriki inafaidika kwa kweli** -- kitanzi cha ndani kabisa cha
pande zote mbili za kuzidisha (uanzishaji wa `a`/`b`/`c` na mzunguko
halisi wa kuzidisha) una fahirisi za safu kama `i*n+j` na `k*n+j`
ambapo `i*n` (au `k*n`) ni sawa kwa kila mzunguko wa kitanzi cha
ndani kabisa (`j`) -- LICM inaihamisha nje, ikipunguza `imul` moja
kwa kila tukio la fahirisi hiyo kwa kila mzunguko wa `j` kuwa `imul`
MOJA TU kwa kila mzunguko wa `i` (au `k`). (Kumbuka: kila tukio la
`i*n` linahamishwa KIVYAKE -- angalia mstari 4 wa maoni ya wigo juu
ya `licm_boresha` -- LICM hii HAIFANYI CSE ya kuunganisha matukio
yanayofanana kuwa kigezo kimoja cha muda, hivyo `a[i*n+j]`,
`b[i*n+j]`, na `c[i*n+j]` kila moja hupata kigezo chake CHA PEKEE
badala ya kushiriki kimoja -- fursa iliyoachwa kwa makusudi kwa
awamu ijayo, si mdudu).

**mzunguko_mchezo HAIFAIDIKI KABISA (kama ilivyotarajiwa, si
kushangaza)** -- kitanzi chake pekee (`wakati (t < MZUNGUKO_JUMLA)`)
kina wito mmoja tu wa `mchezo_sasisha(g)` ndani ya mwili wake wote --
hatari #3 (kizuizi kikali dhidi ya wito wowote ndani ya kitanzi)
inazuia kitanzi hicho kabisa kuwa mgombea wa LICM, bila kujali usemi
mwingine wowote ndani yake. Hii ni matokeo ya moja kwa moja ya wigo
uliochaguliwa kwa makusudi (salama zaidi kuliko kufaidika), sio
upungufu wa bahati mbaya -- sawa na jinsi kuunganisha kazi ndogo
(#235) kulivyoripoti faida ndogo halisi badala ya kubuni moja.

## Uvektishaji (SIMD/SSE2), awamu ya kwanza -- kipimo halisi

Awamu hii inatambua muundo mwembamba SANA wa kitanzi cha "wakati":
`c[i] = a[i] OP b[i]; i = i + 1;` (taarifa MBILI hasa, `OP` ni `+`,
`-`, au `*`, safu zote tatu D64, fahirisi ZOTE ni kitambulisho sawa
na kihesabu cha kitanzi bila ongezeko). Ikilingana, hutoa maagizo ya
SSE2 packed-double (`movupd`/`addpd`/`subpd`/`mulpd`) yanayoshughulikia
elementi MBILI kwa wakati mmoja badala ya moja, na awamu ya pili ya
"mabaki" (scalar, njia ya kawaida) kwa elementi ya mwisho iliyobaki
ikiwa idadi ni isiyo sawa. Angalia maoni kamili juu ya
`simd_jaribu_kitanzi` (`uzalishaji.swa`) kwa masharti yote manane.

**`matriki` HAIFAIDIKI kabisa kwa awamu hii -- kimakusudi, si
upungufu.** Kitanzi cha ndani kabisa cha `matriki` ni
`c[i*n+j] = c[i*n+j] + aik*b[k*n+j];` -- fahirisi zake ni tata
(`i*n+j`, si kitambulisho kimoja `j`), na upande wa kulia wa `+` ni
UZIDISHO (`aik*b[k*n+j]`), si fahirisi ya safu ya moja kwa moja.
Vigezo vyote viwili vinakataliwa na muundo mwembamba uliochaguliwa
kwa makusudi kwa awamu hii ya kwanza (angalia #7 kwenye orodha ya
masharti). Kukuza muundo huu kushughulikia usemi ulioungwa (kuzidisha
NDANI ya kujumlisha) na fahirisi tata ni kazi halisi ya awamu ijayo,
si urekebishaji mdogo -- imeachwa kimakusudi kwa awamu hii nyembamba.

Kwa sababu `matriki` haiendani na muundo, kipimo cha kabla/baada cha
`matriki` chenyewe hakina maana hapa (hakuna njia mpya ya msimbo
inayotekelezwa kabisa) -- kipimo cha chini kinaonyesha badala yake
kipimo kipya cha lengo (`vipimo/simd_jozi_d64/`), kilichoundwa MAALUM
kuendana na muundo unaokubalika, kikilinganisha toleo linaloustahili
uvektishaji dhidi ya toleo la scalar lenye hesabu ile ile HASA
(N=2,000,000 elementi, marudio 60) lakini mwili wa kitanzi wenye
taarifa TATU (kigezo cha muda `tmp` cha ziada) badala ya mbili
kimakusudi, ili kukikwepa. Matokeo ya programu (jumla ya `c` yote)
yalithibitishwa SAWA KABISA kati ya matoleo mawili kabla ya kuamini
muda wowote.

| Kipimo | Scalar (msingi) | SSE2 (uvektishaji) | Mabadiliko |
|---|---|---|---|
| c[i]=a[i]+b[i], N=2,000,000, marudio 60 | ~950ms (wastani wa marudio 3) | ~560ms (wastani wa marudio 3) | ~40% haraka zaidi |

Faida haifikii mara 2 (2x) ya kinadharia ya SSE2 (elementi mbili kwa
agizo moja) -- inayotarajiwa: mzigo huu umefungwa na kipimo cha
kumbukumbu (arrays tatu za D64 za MB 16 kila moja, mara nyingi zaidi
ya akiba ya L2/L3), sio hesabu safi ya ALU, hivyo kupunguza maagizo
ya ALU pekee hakuondoi kizuizi cha bandwidth ya kumbukumbu kabisa.
Toleo la scalar pia lina mzigo mdogo wa ziada (kigezo `tmp` cha ziada
kwenye rafu kwa kila mzunguko) ambao si sehemu safi ya uvektishaji
wenyewe -- tofauti halisi ya "SIMD dhidi ya scalar safi" pengine ni
kidogo zaidi ya 40% iliyoripotiwa hapa, si kidogo.

## Tahadhari ya kelele

Kikao kilichotangulia kiliona TOFAUTI YA MARA 2 (2x) kwenye binary
zilezile wakati wa kupima kazi ya ugawaji wa rejesta wa paramu.
Vipimo hivi vilionyesha kelele ndogo zaidi (chini ya 15% kati ya
chini na juu kwa vipimo vingi), LAKINI mzani wa masafa ya CPU
umewashwa kwenye mashine hii -- usilinganishe wastani wa kati kati
ya vikao viwili tofauti bila kuzingatia hili. Kwa kulinganisha halisi
kwa kazi ijayo, pima toleo la ZAMANI na JIPYA la mkusanyaji KWENYE
KIKAO KIMOJA HASA, karibu wakati mmoja, ili mizani ya masafa iwe
sawa kwa pande zote mbili.

## Sethi-Ullman: ugawaji wa rejesta wa vigezo vya muda vya usemi

Ugawaji wa rejesta uliopo (PR #226/#227) unashughulikia vigezo vya
NDANI na paramu PEKEE -- matokeo ya KATI ya usemi (mfano (a+b) ndani
ya (a+b)*(c+d)) yaliendelea kusukumwa/kutolewa (push/pop) kwenye rafu
kila wakati. Sehemu hii inaongeza njia mbadala inayotumia rejesta za
callee-saved ZISIZOTUMIKA (baada ya vigezo vya ndani/paramu) kushikilia
matokeo ya kati, ikitumia fomula ya Sethi-Ullman kukokotoa haja ya
chini kabisa ya rejesta -- LAKINI SI upangaji upya wake wa mpangilio
wa utathmini (mpangilio unabaki KULIA-KWENDA-KUSHOTO ulio WA KUDUMU
kila mahali, sawa na njia ya asili -- angalia maelezo kamili karibu na
su_ni_kushughulikiwa/su_haja_rejista, uzalishaji.swa).

Uchunguzi mkubwa mbili za gharama zilizogunduliwa kwa KUPIMA HALISI
(si dhana) wakati wa maendeleo:

1. **Hitilafu ya ABI iliyorekebishwa**: rejesta za "ziada" zilizokopwa
   lazima zihifadhiwe/kurudishwe na utangulizi/mwisho wa kazi
   INAYOZITUMIA (si kuachwa bila kuguswa) -- vinginevyo thamani ya
   mpigaji kwenye rejesta hiyo hiyo huharibika kimya. Suluhisho: pitio
   la awali (kwa kila kazi) hukokotoa idadi kubwa zaidi ya rejesta za
   SU zinazohitajika, na utangulizi/mwisho hutenga RASMI idadi hiyo
   (kama vigezo vya ndani).
2. **Gharama isiyo na faida kwa usemi wa haja=1 au usemi unaotokea
   mara moja tu kwa kila wito**: kipimo halisi cha fibonacci
   kilionyesha upungufu wa kasi wa ~35% (fib(n-1)+fib(n-2), haja=1,
   kuitwa milioni 30 za mara) kabla ya masharti mawili kuongezwa: SU
   inatengwa TU kwa (a) haja>=2, NA (b) usemi ulio NDANI ya kitanzi
   (angalia su_kina_kitanzi, uzalishaji.swa) -- gharama ya kutenga
   rejesta (mara moja kwenye utangulizi/mwisho) inalipwa tena
   (amortized) TU pale usemi unaporudiwa mara nyingi ndani ya wito
   mmoja.

| Kipimo | Kabla (push/pop) | Baada (SU) | Mabadiliko |
|---|---|---|---|
| fibonacci (fib(35), haja=1, milioni 30 za wito) | ~150ms | ~155ms | sawa (ndani ya kelele -- ilikuwa ~35% pungufu KABLA ya masharti #2 kuongezwa) |
| heshi (djb2, haja=2, ndani ya kitanzi) | ~1800ms | ~1800ms | sawa (ndani ya kelele -- SU inatumika kwa kweli, lakini faida ndogo mno ikilinganishwa na gharama kubwa ya uumbizaji wa mfuatano inayotawala kipimo hiki) |
| kupanga (nasibu_ijayo, haja=2, LAKINI HAINA kitanzi chake chenyewe) | ~490ms | ~495ms | sawa (SU imekataliwa kikamilifu kwa usahihi -- disassembly inathibitisha msimbo sawa KABISA na kabla) |
| su_rejesta_muda (kipimo kipya, haja=3, safu ndani ya kitanzi, marudio milioni 20) | ~188ms (wastani wa marudio 5) | ~154ms (wastani wa marudio 5) | ~18% haraka zaidi, THABITI kwenye marudio YOTE 5 yaliyopishana |

Kama `matriki` kwa uvektishaji wa SIMD, benchmark za kawaida
hazikuwa na muundo unaoendana vizuri na hali BORA ya SU (usemi wa haja
kubwa, unaorudiwa mara nyingi ndani ya kitanzi kimoja) -- kipimo kipya
maalum (`vipimo/su_rejesta_muda/`) kinathibitisha faida HALISI
inapotokea kwenye hali inayolengwa. Matokeo ya programu (jumla)
yalithibitishwa SAWA KABISA kati ya matoleo mawili kabla ya kuamini
muda wowote.

## Kashe mpya ya anwani kwa ugawaji wa kawaida (arr[e] = arr[e] OP x)

Kupanua `uzalishaji_asimilia_kiwanja_tail` (kashe ya anwani, awali kwa
`+=`/`-=` tu, D64/D32 zikizuiwa kwa makusudi) kwa njia ya PILI ya
ugunduzi: ugawaji wa KAWAIDA ulioandikwa kwa mkono ambapo mwandishi
amerudia `arr[e]` yenyewe kwenye RHS (mfano `c[i]=c[i]+a[i]*b[i]`),
pamoja na desimali (D32/D64) na ZIDISHA (*) sasa zikiungwa mkono.
Ugunduzi wa kwanza (kiwanja halisi, `+=`/`-=`) haujabadilika hata
kidogo -- njia mpya inajaribiwa TU pale ya kwanza haikupata kitu.

**Uthibitisho wa mfumo (disassembly, si dhana)**: kwa muundo rahisi
(`c[i]=c[i]+a[i]*b[i]`, fahirisi "i" moja kwa moja), anwani ya `c[i]`
sasa inakokotolewa MARA MOJA kwa kila mzunguko (shl mara 3: a, b, c)
badala ya MBILI (shl mara 4: a, b, c-kwa-uandishi, c-kwa-usomaji tena)
-- kupungua kwa maagizo 48->46 kwa kila mzunguko, kulikothibitishwa
kwa kulinganisha baiti za utoaji sawia kati ya mkusanyaji wa zamani
na mpya (majaribio 372 sawa kabisa kwa pande zote mbili kabla ya
mabadiliko haya).

**UGUNDUZI MUHIMU (la kweli, si dhana): LICM huzuia kashe hii kwa
`matriki` yenyewe.** Kitanzi cha ndani cha matriki
(`c[i*n+j]=c[i*n+j]+aik*b[k*n+j]`) kina "i*n" inayojitokeza MARA MBILI
(fahirisi ya LHS na ile ya tokeo la kwanza la c kwenye RHS) -- LICM
(iliyoungana KABLA ya kazi hii, PR #236) huhamisha KILA tokeo
kivyake nje ya kitanzi, ikitengeneza VIGEZO VYA MUDA VIWILI TOFAUTI
vinavyoshikilia thamani MOJA (imethibitishwa kwa disassembly: maagizo
YALE YALE ya kuzidisha `rbx*r13` yanaonekana MARA MBILI mfululizo,
yakihifadhiwa kwenye nafasi mbili tofauti za rafu). Kwa kuwa vigezo
hivyo vina MAJINA tofauti, ulinganisho wa muundo (`keq_taja_sawa`)
kwa usahihi HAUKUBALI ulinganifu -- si mdudu, ni tabia salama
inayozuia matokeo yasiyo sahihi. Matokeo: binary ya matriki
iliyotolewa na mkusanyaji mpya ni SAWA KABISA kwa baiti na ile ya
zamani (`cmp` imethibitisha usawa kamili) -- kashe hii HAIBADILISHI
`matriki` hata kidogo kwa sasa. Hii ni fursa halisi ya baadaye
(LICM ingehitaji kutambua vielelezo viwili vinavyofanana kimuundo
kwenye taarifa moja na kuvitumia tena, badala ya kuvihamisha kivyake
-- kazi tofauti kabisa, nje ya wigo wa kazi hii).

**Muda halisi (kipimo kipya `vipimo/kiwanja_mpya_d64/`, safu 2,000,000
D64, marudio 20 ya kitanzi kizima, matokeo YALIYOTHIBITISHWA sawa kati
ya matoleo mawili kabla ya kuamini muda wowote)**: hakuna tofauti ya
wazi ya muda halisi (256-288ms zote mbili, ndani ya kelele kamili ya
mashine hii) licha ya kupungua kwa maagizo kulikothibitishwa -- kipimo
hiki ni MEMORY-BANDWIDTH BOUND (safu tatu za D64 zinazosomwa/
kuandikwa kila mzunguko, baiti 24 za trafiki ya kumbukumbu kwa kila
elementi), si ALU-bound, hivyo kuondoa hesabu chache za integer
(zinazoungana vizuri na ucheleweshaji wa kumbukumbu) hakuonekani
kwenye muda wa ukuta. Sawa na SIMD (`matriki` yenyewe) na SU
(`kupanga`), faida ya kweli ya uboreshaji huu ni ya kukokotoa
(maagizo machache, imethibitishwa), si lazima iwe ya muda wa ukuta
kwa kila mzigo wa kazi -- inategemea kama mzigo huo ni ALU-bound au
memory-bound.

## LICM CSE (kushiriki kigezo cha muda kati ya vielelezo vinavyofanana)

Fursa ya "UGUNDUZI MUHIMU" iliyotajwa hapo juu (kashe mpya ya anwani
haikubadilisha `matriki` kwa sababu LICM ilikuwa ikihamisha "i*n"
MARA MBILI kwa vigezo viwili tofauti vya jina) sasa imefungwa: LICM
(`msambazaji.swa`, `licm_badilisha_kama_inavyowezekana`) sasa
inatambua pale usemi usiobadilika unaotarajiwa kuhamishwa unafanana
KIMUUNDO KABISA (`licm_keq`, ulinganisho wa AST wa kina -- jina la
kitambulisho, namba halisi, +/-/*/ulinganisho/biti, wigo ULE ULE wa
`licm_naingiliana` uliopo tayari) na usemi ULIOSHAHAMISHWA TAYARI
ndani ya kitanzi hicho hicho -- badala ya kutengeneza kigezo kipya
cha muda, tokeo la PILI (na lolote linalofuata) linarejelea TENA
kigezo kilichotengenezwa na tokeo la KWANZA.

**Usalama**: kila mgombea tayari amepitisha ukaguzi kamili wa
`licm_naingiliana` (hauguswi popote ndani ya kitanzi) KIVYAKE kabla
ya kufikia ulinganisho huu -- `licm_keq` haiongezi uthibitisho mpya
wa "hauguswi", inaongeza TU uthibitisho kuwa MBILI ya vitu
tayari-vilivyothibitishwa vinakokotoa thamani SAWA. Kushiriki kigezo
kimoja badala ya viwili ni salama KWA UJENZI. Wigo ni ULE ULE wa LICM
iliyopo (hakuna upanuzi) -- na kwa kuwa `licm_badilisha_kama_inavyowezekana`
tayari haiingii ndani ya matawi ya "kama" kabisa (kizuizi cha awali cha
LICM), swali la "tokeo la ndani ya tawi linashiriki na tokeo la nje"
haliwezi kutokea.

**Uthibitisho wa mfumo (disassembly, si dhana)**: kwa muundo mdogo
uliojitenga (`c[i*n+j]=c[i*n+j]+x` ndani ya kitanzi cha j ndani ya
kitanzi cha i), "i*n" sasa inakokotolewa MARA MOJA TU kwa kila
mzunguko wa i (`imul` moja tu, thamani ikihifadhiwa kwenye r15,
kabla ya kitanzi cha j kuanza) -- si mara mbili (moja kwa fahirisi ya
LHS, moja kwa fahirisi ya kwanza ya RHS) kama awali. Zaidi ya hayo,
kwa kuwa fahirisi mbili za `c[i*n+j]` sasa zinarejelea KIGEZO KIMOJA
(r15+j pande zote mbili), kashe ya anwani ya ugawaji wa kawaida (PR
iliyopita) SASA INAJIHUSISHA PIA kwa muundo huu -- disassembly
inaonyesha anwani moja ikikokotolewa (shl+add mara moja), ikihifadhiwa
kwa push/pop mara mbili, ikitumika kwa usomaji NA uandishi -- mnyororo
mzima wa hoja tatu (LICM ya msingi -> LICM CSE -> kashe ya anwani)
sasa unafanya kazi PAMOJA kwa muundo huu.

**`matriki` halisi**: matokeo (c00/cmid/clast) ni SAWA KABISA kati ya
mkusanyaji wa zamani na mpya (usahihi umethibitishwa). Binary
ILIYOTOLEWA SASA NI TOFAUTI kati ya matoleo mawili (kinyume na kashe
ya anwani peke yake, ambayo ilitoa binary SAWA KWA BAITI) --
`matriki` sasa INAFAIDIKA kwa kweli.

Muda halisi (interleaved, mizunguko 5, kulinganishwa dhidi ya
mkusanyaji wa zamani NA rejeleo la C kwenye mzunguko ule ule kupunguza
kelele ya mzunguko wa CPU wa mashine hii):

| mzunguko | zamani | mpya | C |
|---|---|---|---|
| 1 | 182ms | 150ms | 13ms |
| 2 | 162ms | 154ms | 13ms |
| 3 | 164ms | 148ms | 15ms |
| 4 | 161ms | 150ms | 13ms |
| 5 | 162ms | 156ms | 13ms |

Wastani: zamani 166.2ms, mpya 151.6ms -- **~8.8% haraka zaidi**,
thabiti kwenye mizunguko yote mitano (hakuna mwingiliano kati ya
matokeo ya "zamani" ya chini kabisa (161ms) na "mpya" ya juu kabisa
(156ms)). Ni ndogo kuliko ile ya LICM ya msingi (~13%, PR #236) --
inayotarajiwa, kwa kuwa `matriki` bado ina uzidishaji wa D64
(`aik*b[k*n+j]`) usiobadilika ambao hauhusiani na kazi hii, na hesabu
za anwani zinazoondolewa ni sehemu ndogo tu ya kazi ya jumla ya
kitanzi cha ndani -- lakini ni HALISI, thabiti, na ya kwanza kutoka
kwenye mnyororo wa kashe-ya-anwani/LICM-CSE tangu PR #239.
