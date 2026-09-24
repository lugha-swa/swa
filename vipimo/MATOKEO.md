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

## Peephole: unganisha push/pop (rax/rcx) inayofuatana

Mkusanyaji huu hutumia rafu (push/pop) sana kama njia ya jumla ya
kuhamisha thamani kati ya sehemu za usemi (`sukuma_thamani`/
`vuta_thamani_rcx` na maeneo mengi ya moja kwa moja) -- disassembly
za PR nyingi za kikao hiki zimeonyesha mara kwa mara mfuatano kama
`push rax; pop rcx` (sawa na `mov rcx, rax` moja) au hata `push rax;
push rax; pop rax` (kashe ya anwani ya kiwanja, PR #239/#240 --
nakala ya pili + pop hulingana kabisa, hakuna athari).

**Njia**: badala ya kuandika upya baiti ZILIZOKWISHA-ANDIKWA (tatizo
la uhamishaji/relocation la kweli -- lebo/fixup/rela zote hurekodi
`bafa_text_wapi` moja kwa moja), `andika_push_rax`/`andika_push_rcx`
sasa HUAHIRISHA push MOJA (haiandiki baiti mara moja, huweka tu
`png_akiba_reg`). `andika_pop_rax`/`andika_pop_rcx` hukagua hali hiyo:
rejesta ile ile = hakuna athari (haiandiki KABISA); rejesta tofauti =
`mov` moja badala ya push+pop mbili. `andika_baiti` YENYEWE (kizuia-
njia cha PEKEE kinachopitiwa na kila agizo lingine) husafisha push
inayosubiri KABLA ya kuandika baiti yoyote nyingine -- hii
inashughulikia KILA agizo linaloweza kutokea kati ya push na pop
(jumlisha, wito, kuruka, LEBO) bila kuhitaji kila kazi ya `andika_*`
(kuna mamia) kujua kuhusu kipengele hiki. Kazi zinazorekodi
`bafa_text_wapi` nje ya `andika_baiti` (`weka_lebo`, `weka_fixup`,
`lebo_hifadhi`, `andika_rela_ulimwengu`, nafasi za mwisho za
`uzalishaji_kuu`/`uzalishaji_jit`) zote husafisha eksplisiti pia --
uhakika, si dhana ya mpangilio wa wito.

**Mdudu halisi uliogunduliwa na kurekebishwa KABLA ya kuunganishwa**:
jaribio la kwanza la utekelezaji lilisababisha SEGV/matokeo mabaya
kwenye majaribio 18 (ikiwa ni pamoja na yale ya kashe ya anwani ya
kiwanja) -- chanzo: `andika_pop_rcx`/`andika_pop_rax` ziliita
`andika_mov_rcx_rax`/`andika_mov_rax_rcx` KABLA ya kufuta
`png_akiba_reg`, hivyo wito huo wa ndani wa `andika_baiti` uliona hali
ya zamani ikiwa bado hai na kutoa PUSH YA PILI ISIYO SAHIHI (rafu
ikavurugika). Rekebisho: futa hali KWANZA, kisha ita `andika_mov_*`
-- muundo uleule uliokuwa tayari sahihi kwenye `png_safisha`.
Iligunduliwa kwa disassembly ya kesi ndogo (`c[i*n+j]+=7`), si dhana.

**Usalama uliothibitishwa**: jaribio maalum la hatari kuu (lebo halisi
kutoka `&&`/`||` NDANI ya usemi wenye push inayosubiri) limejengwa na
kuthibitishwa kwa disassembly -- push hutolewa kikamilifu KABLA ya
lebo yoyote, mtiririko wa udhibiti hauwezi kamwe kuruka juu ya push
isiyotolewa.

**Majaribio 4 mapya**: mnyororo mrefu wa jozi HURU za push/pop (=96),
kesi maalum push;push;pop kutoka kiwanja_tail (fahirisi tata i*n+j),
lebo katikati ya usemi (&&/||, =6), wito wa kazi katikati ya usemi
wenye push inayosubiri (=23). 381/381 (377+4), fixpoint na gen1/gen2
vyote vinapita, imethibitishwa mara 4 kutoka kwa ujenzi safi (idadi
kubwa kimakusudi kutokana na wigo mpana wa mabadiliko haya).

**Utendaji (uaminifu kamili)**: mabadiliko ya ukubwa wa binary kwenye
vipimo vilivyopo tayari (fibonacci, kupanga, matriki, mzunguko_mchezo)
ni SIFURI kabisa -- ugawaji wa rejesta (PR #226/227), Sethi-Ullman
(#238), na kashe ya anwani (#239/#240) tayari zimeondoa fursa nyingi
za push/pop zinazofuatana kwenye vitanzi vyao vikuu. `heshi` (djb2)
pekee ilionyesha tofauti ya baiti 1 (kokotoo moja ya rejesta-moja
imeondolewa). Kipimo kipya kilicholengwa
(`vipimo/png_peephole/`, `c[idx]+=1` mara milioni 40 kwa mzunguko
mmoja) kinakokotoa baiti 1 pungufu (kimethibitishwa), LAKINI muda wa
ukuta haubadiliki HATA KIDOGO (264-287ms zote mbili, ndani kabisa ya
kelele) -- push/pop ya rejesta MOJA (bila kugusa kumbukumbu ya
maana) tayari ni ya haraka SANA kwenye CPU za kisasa (injini ya rafu/
stack engine), na akiba ya maagizo 2-3 inapotea kabisa dhidi ya
gharama nyingine za kitanzi (hapa: `%` -- modulo). Hitimisho la
uaminifu: hii ni uboreshaji WA KWELI wa idadi ya maagizo (umethibitishwa
kwa disassembly), lakini SIO uboreshaji wa muda wa ukuta unaoweza
kupimwa kwenye mizigo ya kazi iliyojaribiwa -- thamani yake HALISI
kwa sasa ni ubora wa msimbo (maagizo machache), si kasi.
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

## Kupunguza nguvu kwa mgawanyo/modulo ya nguvu-ya-2 (signed)

Hadi sasa `/` na `%` zilikuwa zikiepukwa na KILA uboreshaji mwingine
kikao hiki (kunja-namba, kashe-ya-anwani, LICM, uvektishaji) kwa
sababu SAR (mabadiliko ya biti ya x86) huzunguka KUELEKEA -INFINITY
(floor) kwa nambari hasi, wakati mgawanyo wa nambari kamili wenye
ishara wa lugha hii (kama C) HUKATA KUELEKEA SIFURI (truncate) --
`-7/4` lazima iwe `-1`, LAKINI `-7>>2` ni `-2`. PR hii inashughulikia
hilo kwa mbinu ya KAWAIDA ya wakusanyaji halisi (GCC/LLVM/MSVC):
upendeleo (bias) wa `(d-1)` unaoongezwa kwa x TU pale x ni hasi
(kwa kutumia kinyago cha ishara `x >> (upana-1)`, branchless), kisha
SAR. Modulo hutumia tena hesabu ya mgawanyo: `r = x - (q << kipeo)`.
Bila ishara (A32/A64): hakuna upendeleo unaohitajika kabisa -- SHR/AND
rahisi ni sahihi moja kwa moja.

**Usahihi**: mzunguko kamili wa x kutoka -20 hadi 20 (na -200 hadi 200
kwa N64) dhidi ya kila kigawanyaji halisi (2,4,8,16 kwa N32; 2,8,64
kwa N64), ukilinganishwa dhidi ya mgawanyo wa D64 (njia tofauti kabisa,
isiyoathiriwa na uboreshaji huu) uliobadilishwa kuwa nambari kamili --
rejeleo huru la kweli, si kurudia fomula ile ile. Pia imethibitishwa:
`(x/d)*d + (x%d) == x` (utambulisho wa msingi) kwa kila jozi, thamani
ndogo kabisa ya N32 (-2147483648/2), na bila ishara na thamani kubwa
(4000000000) yenye biti ya juu iliyowekwa (kuthibitisha SHR, si SAR).

**UGUNDUZI WA PEKEE (nje ya wigo wa kazi hii, umeripotiwa hapa kwa
uwazi)**: wakati wa kuandaa majaribio haya, iligundulika kuwa
`jaribu_kunja_namba` (kukunja namba za kudumu, msambazaji.swa --
KIPENGELE TOFAUTI KABISA na kazi hii, kinachofanya kazi wakati wa
UCHANGANUZI si UZALISHAJI) kina mdudu WAKE MWENYEWE kwa mgawanyo wa
namba HASI ya kudumu (mfano `(0-17)/3` -- si nguvu ya 2, hivyo nje ya
wigo wa PR hii kabisa) -- imefuatiliwa kwa uhakika kamili hadi
`mbegu.bin` (mkusanyaji wa awali uliogandishwa): mbegu.bin YENYEWE
inakosea kuzalisha msimbo kwa muundo maalum wa "N64 = safu ya N32
[fahirisi]; ...; mgawanyo" (imethibitishwa kwa kuunda upya muundo huo
kama programu huru na kuikusanya MOJA KWA MOJA na mbegu.bin). Kwa
kuwa `jaribu_kunja_namba` ipo ndani ya stage1.swa (chanzo cha
mkusanyaji mwenyewe), hitilafu hii inaingia kwenye kizazi cha KWANZA
(gen1/stage1, kilichojengwa NA mbegu.bin moja kwa moja) -- LAKINI
INAJIPONYA (self-heals) KABISA kwenye kizazi cha PILI (gen2/stage2,
kilichojengwa na gen1, ambaye msimbo wake WENYEWE wa uzalishaji
(uzalishaji.swa) HAUNA mdudu huu) -- imethibitishwa moja kwa moja
kwa kujenga gen2 na kuonyesha `(0-17)/3` inatoa `-5` sahihi. Kwa
kuwa mbegu.bin imegandishwa milele, hitilafu hii haiwezi kurekebishwa
huko -- lakini haiathiri mkusanyaji halisi unaotumika (mnyororo wa
kujiendeleza zaidi ya kizazi kimoja), na majaribio ya PR hii
YAMEUNDWA KWA MAKUSUDI kuepuka mkondo wa kukunja namba kabisa
(kigawiwa daima ni kigezo, si namba halisi ya moja kwa moja) ili
yasiathiriwe.

**Vipimo halisi vilivyopo**: `matriki`'s `n/2` (mara moja tu, si
kwenye kitanzi kizito, faida haina maana), `simd_jozi_d64`'s
`sum/1000000000` (si nguvu ya 2, haihusiki), `kupanga`'s
`nasibu_ijayo()` `% 2147483648` (2^31 -- LAKINI namba hii inazidi
wigo chanya wa N32 (2147483647), hivyo inahifadhiwa kwenye dimbwi la
N64 -- `gwm_kipeo_kulia` inakataa kwa usahihi kigawanyaji cha dimbwi
kimakusudi, hivyo HAIFAIDIKI kabisa -- imethibitishwa kwa disassembly,
hakuna SAR, idiv bado inatumika).

**Kipimo kipya cha moja kwa moja (`vipimo/png_peephole/swa.swa`,
tayari zilizopo -- `i % 8` ndani ya kitanzi cha mizunguko 40,000,000)**:
binary ni TOFAUTI (kashe mpya inatumika, imethibitishwa kwa
disassembly -- SAR badala ya idiv). Muda halisi (interleaved, mizunguko
5):

| mzunguko | zamani | mpya |
|---|---|---|
| 1 | 290ms | 179ms |
| 2 | 264ms | 177ms |
| 3 | 266ms | 181ms |
| 4 | 269ms | 182ms |
| 5 | 266ms | 181ms |

Wastani: zamani 271.0ms, mpya 180.0ms -- **~33.6% haraka zaidi**,
thabiti kwenye mizunguko yote mitano (hakuna mwingiliano). `kupanga`
(kigawanyaji hakifaidiki, angalia juu): wastani zamani 283.2ms, mpya
273.8ms -- tofauti ndogo (~3.3%) inayotokana na mgawanyo MMOJA usio
wa kitanzi (`N_JUMLA/2` kwenye uchapishaji wa matokeo) uliopata njia
ya haraka, si `nasibu_ijayo()` -- ndani ya kelele.
## cmp+jcc ya moja kwa moja kwa sharti la kama/wakati/kwa

Njia ya JUMLA ya ulinganishi (uzalishaji_sawa/chini/juu/n.k.) hutoa
KILA MARA thamani KAMILI ya 0/1 (cmp/comisd, setcc al, movzx eax,al)
kwa sababu thamani hiyo inaweza kuhitajika kwa matumizi mengine
(kuhifadhiwa, kurudishwa, kutumika ndani ya usemi mwingine). Lakini
pale sharti la "kama"/"wakati"/"kwa" LENYEWE ni ulinganishi huo huo
moja kwa moja -- muundo unaotokea KATIKA KILA sharti la kitanzi na
tawi la mradi huu, ulioonekana kwenye kila disassembly ya kikao hiki
-- thamani ya 0/1 haihitajiki kabisa: mahali pekee inapotumika ni
kuamua tawi gani kuchukua, na cmp/comisd YENYEWE tayari imeweka
bendera za CPU zinazohitajika. Muundo wa zamani (cmp; setcc; movzx;
test; jcc, maagizo 5-6) sasa unakuwa cmp/comisd; jcc (maagizo 2) --
utambuzi ni WA KIMUUNDO TU (ast_aina[cond] ni mojawapo ya vinodi sita
vya ulinganishi), si uchambuzi wa matumizi (data-flow), kwa sababu
nafasi ya sharti la kama/wakati/kwa haina mtumiaji mwingine yeyote wa
thamani hiyo KWA UJENZI.

Wigo: vinodi sita vya ulinganishi (==,!=,<,>,<=,>=), ISHARA (jl/jg/
jle/jge) NA bila-ishara/desimali (jb/ja/jbe/jae, jedwali sawa na
weka_setcc_kwa_enc iliyopo tayari), kama/wakati/kwa/fanya (nodi moja
ya AST_WAKATI kwa zote). Njia ya jumla haijaguswa kabisa -- inatumika
kila usemi wa ulinganishi haupo MOJA KWA MOJA kwenye nafasi ya
sharti.

### Uthibitisho

Majaribio matano mapya: mzunguko wa mipaka wa vinodi sita (i=-5..5
dhidi ya 0, hesabu za mkono za kila operesheni -- disassembly
imefuatiliwa MKONONI, mwelekeo wa kila jcc umethibitishwa dhidi ya
chanzo moja kwa moja, hakuna hitilafu); matumizi mengine (usemi
mmoja wa ulinganishi kimuundo ukitumika njia ya jumla NA njia ya
haraka ndani ya kazi moja); kitanzi cha kwa; ulinganishi wa D64
(comisd, jb/ja/jbe/jae, ikiwemo mpaka wa usawa halisi); ulinganishi
wa A32 bila ishara (a=4000000000 -- ingeonekana hasi kama ingesomwa
kwa ishara, jaribio linatofautisha wazi kati ya njia mbili). 390/390
(385 zilizopo + 5 mpya). Fixpoint na ukaguzi wa gen1-dhidi-ya-gen2
vyote vinapita, imethibitishwa mara TATU kutoka kwa ujenzi safi
(zaidi ya kawaida -- hii ni msimbo unaoamua mtiririko wa udhibiti,
hatari ya hitilafu ni majibu MABAYA KIMYA, si kuanguka).

Disassembly ya jaribio_cmpjcc_sita_operesheni imefuatiliwa KABISA
mkononi: kila mojawapo ya vinodi sita, PAMOJA na sharti la wakati
lenyewe (i<=5), imethibitishwa kuwa cmp+jcc SAHIHI (mwelekeo sahihi
kabisa dhidi ya chanzo) -- SIFURI setcc/movzx popote kwenye binary
hiyo. jaribio_cmpjcc_bila_ishara/desimali zimethibitisha familia
sahihi ya jcc (jb/ja/jbe/jae, SI jl/jg/jle/jge) kwa A32 na D64.

### Utendaji (vipimo vyote vitano, interleaved, mizunguko 5)

| kipimo | zamani (wastani) | mpya (wastani) | tofauti |
|---|---|---|---|
| fibonacci | 97.2ms (90.25ms bila mzunguko 1 wa kwanza) | 81.8ms | ~9-16% haraka zaidi |
| matriki | 150.4ms | 142.0ms | ~5.6% haraka zaidi |
| heshi | 989.2ms | 955.2ms | ~3.4% haraka zaidi |
| kupanga | 280.8ms | 278.4ms | ndani ya kelele |
| mzunguko_mchezo | 20.6ms | 20.8ms | ndani ya kelele |

Uboreshaji wa kweli na thabiti kwenye vipimo vitatu (fibonacci,
matriki, heshi) -- vyote vina matawi/mizunguko mingi yenye
ulinganishi wa moja kwa moja. `matriki` inaendelea kufaidika juu ya
mnyororo mzima uliopo (LICM -> LICM CSE -> kashe ya anwani).
fibonacci (kazi ya kujirudia yenye "kama (n<2)" kila wito -- mamilioni
ya matawi) inaonyesha faida kubwa zaidi -- inatarajiwa, kwa kuwa
uwiano wa maagizo ya tawi dhidi ya kazi nyingine ni mkubwa zaidi
kwenye kipimo hicho. kupanga/mzunguko_mchezo hazionyeshi tofauti ya
wazi -- ndani ya kelele ya mzunguko wa CPU wa mashine hii, hazina
sehemu kubwa ya wakati inayotumika kwenye matawi rahisi kiasi hicho.

## cmp+jcc ya moja kwa moja ndani ya legs za && / ||

Kazi iliyotangulia (cmp+jcc kwa sharti la moja kwa moja) iliacha && na
|| nje ya wigo kimakusudi -- zina mzunguko mfupi wao wenyewe tayari.
Lakini kila leg moja moja ya && / || (mfano "a<b" ndani ya "kama (a<b
&& c<d)") ilikuwa ikitathminiwa kwa NJIA YA JUMLA (cmp;setcc;movzx)
kabla ya kujaribiwa TENA na test+jcc na uzalishaji_na/uzalishaji_au --
muundo ULE ULE uliopotea kwenye sharti rahisi, sasa umeondolewa kwa
legs mbili zilizo moja kwa moja (bapa -- si && / || iliyowekwa ndani
ya nyingine, ambayo inaendelea kwenye njia ya jumla iliyo sahihi
kabisa, polepole zaidi tu).

Hakuna kipimo kilichopo kinachotumia && au || (imethibitishwa kwa
grep kabla ya kuandika chochote), hivyo kipimo kipya `naau_haraka`
kimeundwa: kitanzi cha ndani chenye sharti "j<m && i<n" (muundo wa
"clamped nested bound" wa kawaida kwenye misimbo halisi), 4000x4000
mizunguko.

### Uthibitisho

Majaribio matatu mapya: mzunguko mfupi (leg ya pili haitathminiwi
KABISA pale leg ya kwanza tayari inaamua matokeo -- imethibitishwa
kwa kihesabu cha wito halisi, si kudhania -- kwa && na || zote mbili,
kama na wakati zote mbili); mchanganyiko (leg moja ni ulinganishi wa
moja kwa moja, nyingine ni kigezo cha kawaida, ndani ya && / || moja
-- kuthibitisha njia mbili zinaweza kuchanganyika kwa usahihi ndani
ya usemi mmoja); vinodi sita vya ulinganishi kama legs (==,!=,<,>,<=,
>=) ndani ya && na ||. 397/397 (394 zilizopo + 3 mpya). Fixpoint na
gen1-dhidi-ya-gen2 vinapita, imethibitishwa mara TATU kutoka kwa
ujenzi safi.

Disassembly ya jaribio_naau_vinodi_sita imefuatiliwa KIPOFU (baiti
kwanza, chanzo baadaye): leg za && (== na !=) zinatumia jcc ya
KINYUME sahihi (jne kwa ==, je kwa !=), zote mbili zikiruka kwenda
lebo ya pamoja ya "sivyo kweli" ya kama isiyo na else (inayolingana
na mwisho wa mwili wa then -- muunganiko sahihi wa udhibiti). Leg za
|| (!= na >) zinatumia mchanganyiko sahihi wa jcc ya MOJA KWA MOJA
(leg ya kwanza, kuruka kwenda lebo_kweli_mapema mbele ya mwili wa
then) na jcc ya KINYUME (leg ya pili, kuruka kwenda lebo ya "sivyo
kweli" ya jumla). SIFURI setcc/movzbl popote kwenye binary hiyo.

### Utendaji

Matokeo (jumla=16000000) ni sawa kati ya compiler ya zamani na mpya;
binary NI tofauti (uboreshaji unajihusisha). Uthibitisho tuli
(disassembly): setcc/movzbl 16 zimeondolewa kabisa, maagizo 72
machache jumla (3247->3175), binary ndogo kwa baiti 244 (8622->8378).

Muda halisi (interleaved): baada ya mzunguko wa kwanza wa joto
kutupwa, mizunguko 8 iliyofuata haikuonyesha tofauti ya wazi kati ya
zamani na mpya (zote karibu 47ms) -- HAINA UHAKIKA (inconclusive),
si tofauti hasi wala chanya thabiti. Sababu inayowezekana (kama
ilivyoonekana kwenye peephole ya push/pop, PR ya awali): CPU za
kisasa zenye utekelezaji usiofuata mpangilio (out-of-order) zinaweza
kuficha maagizo machache ya ziada ya bei nafuu (setcc/movzx) nyuma ya
gharama nyingine ya kitanzi (ongezeko, tawi lenyewe) bila kuongeza
njia muhimu (critical path) kwa kiasi kinachopimika kwa kitanzi hiki
maalum. Uboreshaji wa kweli, uliothibitishwa (uondoaji wa maagizo),
si wa kubuni -- lakini si kila uboreshaji wa maagizo unaonekana kwenye
muda wa ukuta, hasa pale kazi nyingine ya kitanzi (tawi lenyewe,
ongezeko) tayari inashinda njia muhimu.
## Zidisho-na-uchawi (magic-number multiplication) kwa mgawanyo/modulo ya kigawanyaji chochote cha kudumu

PR #243 ilishughulikia mgawanyo/modulo kwa nguvu-ya-2 tu (bias-kisha-
SAR). Kigawanyaji chochote KINGINE cha kudumu (3, 5, 7, 10, 100...)
kilikuwa kikiendelea kutumia idiv/div (maagizo 20-40+ ya mzunguko wa
saa dhidi ya 3-5 kwa zidisho). Suluhisho la KAWAIDA la wakusanyaji
halisi (GCC/LLVM/MSVC): zidisha kwa "namba ya uchawi" (M) iliyokokotolewa
wakati wa kukusanya, kisha uhamishe -- fomula kutoka kwenye karatasi ya
Granlund na Montgomery (1994) "Division by Invariant Integers using
Multiplication", iliyoelezwa zaidi kwenye "Hacker's Delight" (Warren),
Sura ya 10.

**KUMBUKUMBU KWA UWAZI**: mtandao haukupatikana wakati wa kazi hii
kuthibitisha fomula dhidi ya chapisho asilia moja kwa moja (chanzo-mbili
kilichokusudiwa awali). MAJARIBIO YA NAMBA HALISI (zaidi ya milioni
tano ya jozi (kigawanyaji, kigawiwa) kwa Python KABLA ya kutekelezwa
kwa Swa, kisha mamia ya maelfu zaidi ya majaribio ya moja kwa moja ya
Swa dhidi ya idiv) ndiyo msingi mkuu wa uthibitisho, si kumbukumbu ya
karatasi peke yake.

### Wigo (kwa makusudi mdogo)

- **N32 TU** -- N64 haiguswi kabisa (utafutaji wa namba ya uchawi
  ungehitaji usahihi wa 128-bit ambao lugha hii haina aina ya asili
  ya kuutoa salama -- imeachwa kama kazi ya baadaye).
- Kigawanyaji d: 2 <= d <= 65536, namba halisi ya moja kwa moja KUDUMU
  (si kigezo, si dimbwi la N64) -- mipaka hii inahakikisha hesabu za
  utafutaji (N64) zinabaki mbali sana na kikomo cha N64.
- **Bila ishara (A32)**: baadhi ya vigawanyaji (mfano maarufu: 7)
  HUHITAJI hatua ya ziada ya "ongeza" isiyotekelezwa kwa MAKUSUDI --
  hizo HUENDELEA kutumia idiv, salama TAYARI, ni fursa iliyokosekana
  TU. Karibu 2/3 ya vigawanyaji 3-2000 vinafaidika (imethibitishwa
  kwa Python), 1/3 (wakiwemo 7,14,19,21...) huendelea kwenye idiv.
- **Yenye ishara (N32)**: fomula ya "magic" inashughulikia hatua ya
  marekebisho (q=q+n endapo M>=2^31, yaani "hasi" kama N32) MOJA KWA
  MOJA kama SEHEMU ya kawaida ya fomula -- SI hali ya kushindwa, ni
  hatua inayotarajiwa (d=7 signed inahitaji hatua hii, imethibitishwa
  kwa disassembly ya jaribio_uchawi_gawanyo_ishara_msingi).

### Ugunduzi wa pekee (nje ya wigo, umeripotiwa kwa uwazi)

Wakati wa kuandaa majaribio, iligundulika mibegu miwili TOFAUTI KABISA
na kazi hii, isiyohusiana:
1. `{ N32 x = 1; }` (kitalu kisicho na kitanzi/tawi kinachofunga
   tangazo la kigezo cha ndani) hutoa "kosa: kianzilishi cha safu
   hakijaungwa mkono" kimakosa -- mdudu wa uchanganuzi (parser),
   HAUHUSIANI na zidisho-na-uchawi kabisa.
2. `A32 z = 4294964296;` (uanzishaji wa moja kwa moja wa A32 na namba
   halisi kubwa kuliko 2^31) haufanyi kazi sahihi -- kigawanyaji
   kilichotumika kwenye jaribio hilo (thamani ndogo, tayari
   umethibitishwa sahihi na Python) HAUHUSIKI, ni tabia ya UANZISHAJI
   pekee. Imeepukwa kwenye majaribio yote kwa kutumia hesabu ya
   wakati wa kukimbia badala ya namba halisi ya moja kwa moja.

Zote mbili zimeachwa bila kurekebishwa (nje ya wigo wa kazi hii),
zimeripotiwa hapa kwa uwazi kwa kazi ya baadaye.

### Uthibitisho

Majaribio manne mapya: kesi za msingi zenye ishara (d=10 bila
marekebisho, d=7 na hatua ya "ongeza", thamani za mkono zikiwemo
karibu na N32_CHINI); kesi za msingi bila ishara (d=10/100, thamani
karibu na 2^32-1 iliyoundwa kwa hesabu); kesi za kukataa (d=7 bila
ishara, d>65536, kigawanyaji si wa kudumu, N64); mfululizo mpana wa
vigawanyaji 14 dhidi ya idiv (N32 x=-600..600 ikiwemo karibu na
N32_CHINI/N32_JUU, A32 y=0..600). 398/398 (394 zilizopo + 4 mpya).
Fixpoint na gen1-dhidi-ya-gen2 vinapita, imethibitishwa mara TATU
kutoka kwa ujenzi safi.

Disassembly ya jaribio_uchawi_gawanyo_ishara_msingi imefuatiliwa
KABISA mkononi kwa d=7 (kesi ya "ongeza"): push rax; mov ecx,
0x92492493 (=2454267027, M iliyokokotolewa); imul ecx; mov eax,edx;
pop rcx; add eax,ecx (marekebisho); mov ecx,2; sar cl,eax; push rax;
mov ecx,31; shr cl,eax; pop rcx; add eax,ecx (usahihishaji wa ishara)
-- kila baiti inalingana KABISA na fomula iliyokokotolewa. d=10 (bila
marekebisho) imethibitishwa kutumia mov ecx,0x66666667 bila push/pop
ya ziada kabla ya imul. Majaribio ya kukataa (jaribio_uchawi_kataa)
yamethibitishwa kutumia idiv/div PEKEE (sifuri mul/imul).

Matriki na kupanga (pekee vinavyotumia mgawanyo, vyote nguvu-ya-2)
vimethibitishwa kuwa BAITI SAWA (`cmp`) kati ya mkusanyaji wa zamani
na mpya -- kazi hii haigusi njia iliyopo ya nguvu-ya-2 hata kidogo.

### Utendaji

Hakuna kipimo kilichopo kinachotumia mgawanyo/modulo ya kigawanyaji-
si-nguvu-ya-2 kwenye mzunguko mzito (matriki/kupanga ni nguvu-ya-2,
fibonacci/heshi/mzunguko_mchezo hazina mgawanyo kabisa) -- kipimo
kipya `vipimo/uchawi_gawanyo/` kimeundwa MAKUSUDI (i/7, i%7, mizunguko
50,000,000). Muda halisi (interleaved, mizunguko 5, dhidi ya
mkusanyaji wa zamani): wastani zamani 383.2ms, mpya 319.8ms -- **~16.5%
haraka zaidi**, thabiti kwenye mizunguko yote (hakuna mwingiliano).

## Vipimo vipya: mapungufu manne yasiyokuwa na kipimo, na lugha za ziada za kulinganisha

Kipimo cha awali (fibonacci, kupanga, matriki, heshi, mzunguko_mchezo)
hakigusi kabisa: ugawaji mdogo mdogo wa mara kwa mara/pointer-chasing
(zote hutumia ugawaji MMOJA mkubwa wa awali), D64 bila shinikizo la
kumbukumbu, mfumo tofauti wa ufikiaji wa safu ya n8, wala mfuatano/
maandishi kabisa. Vipimo vinne vipya vinaziba mapengo hayo -- kila
kimoja kina Swa, C, Rust, na Go (lengo halisi bado ni kushinda C;
Rust/Go ni muktadha wa ushindani, si lengo).

Uthibitisho wa usahihi: KILA lugha (Swa/C/Rust/Go) kwa KILA kipimo
inatoa checksum SAWA KABISA (si "inayoonekana sahihi" -- imelinganishwa
neno kwa neno) -- LCG ile ile (mbegu, kizidishi 1103515245, ongezeko
12345, modulo 2^31) inatumika katika lugha zote nne kwa vipimo
vinavyohitaji data ya nasibu, ili mfuatano wa thamani uwe SAWA kabisa.

### miti_bst -- mti wa BST (ugawaji mdogo mdogo + pointer-chasing)

Kuingiza nodi 150,000 (kwa njia ya kurudia-rudia, SI kujirudia, kuepuka
hatari ya kina cha rafu kwenye mti usio na usawa), kisha kutafuta
150,000 (checksum inajumuisha hatua za pointer-chase, si tu
imepatikana/haijapatikana, kuzuia kukatwa mapema). Checksum:
`idadi_patikana=1074 hatua_jumla=3560678` (SAWA lugha zote nne).

| lugha | wastani (ms, mizunguko 5) |
|---|---|
| swa  | 102.4 |
| c    | 58.4 |
| rust | 66.8 |
| go   | 55.6 |

Swa ni ~1.75x polepole kuliko C.

### mandelbrot -- D64 bila shinikizo la kumbukumbu (600x600, kiwango cha juu 200)

Hesabu safi ya vigezo vya ndani, hakuna ugawaji wowote. Checksum:
`jumla_iter=17315689` (SAWA lugha zote nne).

| lugha | wastani (ms, mizunguko 5) |
|---|---|
| swa  | 158.8 |
| c    | 61.6 |
| rust | 59.2 |
| go   | 64.0 |

Swa ni ~2.6x polepole kuliko C -- pengo pana kuliko matriki
(~1.5-2x kwenye vipimo vya sasa), ikionyesha kitanzi cha D64
kisicho na safu bado kina nafasi ya uboreshaji.

### mchujo -- Mchujo wa Eratosthenes (N=25,000,000, safu ya n8)

Muundo tofauti wa ufikiaji wa kumbukumbu kuliko matriki (uandishi wa
mfuatano usio wa mstari, si row/column). Checksum: `idadi_kuu=1565927`
(SAWA lugha zote nne).

| lugha | wastani (ms, mizunguko 5) |
|---|---|
| swa  | 314.0 |
| c    | 137.6 |
| rust | 162.8 |
| go   | 157.0 |

Swa ni ~2.3x polepole kuliko C. Kwa kushangaza, Rust na Go zote mbili
ni POLEPOLE kuliko C hapa (~1.15x-1.2x) -- muundo huu unaathiriwa na
ukaguzi wa mipaka ya safu (bounds checking) katika lugha zote mbili;
C pekee (bila ukaguzi) inaepuka gharama hiyo.

### maneno -- kuhesabu marudio ya maneno (maandishi 400,000, kamusi 16)

Ufikiaji wa safu ya n8/mfuatano, ulinganishaji wa herufi kwa herufi.
Checksum: `cheki=3400000 hesabu0=25000 hesabu15=25000` (SAWA lugha
zote nne).

| lugha | wastani (ms, mizunguko 5) |
|---|---|
| swa  | 123.0 |
| c    | 22.4 |
| rust | 9.2 |
| go   | 23.4 |

Swa ni ~5.5x polepole kuliko C, na ~13.4x polepole kuliko Rust.

SAHIHISHO (baada ya ukaguzi huru): madai ya awali hapa yalisema hili
ndilo pengo pana zaidi kuliko kipimo kingine chochote, cha zamani au
kipya -- si sahihi, kinyume na jedwali la muhtasari chini (matriki
~10.1x) ambalo halikulinganishwa nalo wakati wa kuandika sehemu hii.
maneno ndiyo pengo pana zaidi kati ya vipimo VINNE VIPYA pekee, na
pengo la PILI kwa ukubwa kwenye mfululizo mzima (nyuma ya matriki).
Bado ni ugunduzi halisi na wa thamani -- Rust inaonekana kuboresha
ulinganishaji wa mfuatano
(`kn.as_bytes() == &maandishi[mwanzo..p]`, huenda memcmp iliyoboreshwa
sana/vectorized) kwa njia ambayo Swa (wito wa kazi unaorudiwa kwa
kila neno la kamusi 16, ukubwa wa mfuatano ukikokotolewa upya kila
wakati kupitia urefu_mfuatano_ndani) haifanyi. Hii ni FURSA HALISI
ya kazi ya baadaye -- si wigo wa kazi hii, ambayo ni kupima TU.

### Muhtasari (kipimo kilichopo + kipya, dhidi ya C)

| kipimo | uwiano wa Swa/C |
|---|---|
| fibonacci | ~4.6x |
| kupanga | ~3.1x |
| matriki | ~10.1x |
| heshi | ~7.0x |
| mzunguko_mchezo | ~6.1x |
| miti_bst | ~1.75x |
| mandelbrot | ~2.6x |
| mchujo | ~2.3x |
| maneno | ~5.5x |

Kumbuka kuhusu kelele ya mashine: mashine hii ina tofauti halisi ya
muda (imethibitishwa mara kadhaa kikao hiki kwa kupima tena binary
ile ile ya C) -- nambari za mzunguko mmoja mmoja hapo juu zisichukuliwe
kama sahihi hadi desimali ya mwisho, lakini mwelekeo wa jumla na
ukubwa wa pengo umefuata mbinu ile ile ya kupima (interleaved,
mizunguko mingi) inayotumika kila mahali kwenye hati hii.
## SSCSE -- CSE ya msimbo wa mstari-moja-kwa-moja

LICM (kilichoungana kabla) inashughulikia usemi usiobadilika ndani ya
kitanzi tu -- usemi unaorudiwa NJE ya kitanzi (au ndani ya kitanzi
lakini usiokuwa invariant) haugusiki kabisa. SSCSE inashughulikia
hiyo: kigawanyaji cha "available expressions" cha kawaida (classical
CSE) kwa mnyororo wa taarifa za ngazi moja (straight-line), na jedwali
linalowekwa upya kabisa kila mpaka wa kama/wakati/kwa (angalia maelezo
kamili juu ya sscse_boresha, msambazaji.swa, kwa muundo kamili wa
uamuzi na sababu za kila kizuizi cha wigo).

### Uthibitisho

Majaribio matano mapya: kesi ya msingi (ulinganisho wa kina, si wa
ngazi ya juu tu -- "a*b" ndani ya "a*b+c" na "a*b-d", vitengo vizima
havifanani kamwe kimuundo); ubatilishaji (reassignment kati ya tokeo
mbili, lazima ikokotolewe upya); mpaka wa tawi; tokeo la ngazi ya juu
dhidi ya la ndani; na wito wa kazi kati ya tokeo mawili (wito wowote
hufuta jedwali, hata usiohusiana -- fursa iliyokosekana, si hitilafu).
406/406 (401 zilizopo + 5 mpya). Fixpoint na gen1-dhidi-ya-gen2
vinapita, imethibitishwa mara TATU kutoka kwa ujenzi safi.

Disassembly ya jaribio_sscse_msingi imethibitisha imul MOJA TU
inatokea (badala ya mbili) -- "a*b" inashirikiwa kati ya "x=a*b+c" na
"y=a*b-d". jaribio_sscse_ubatilishaji limethibitisha imul MBILI
zinatokea (hazikushirikiana, kama inavyotarajiwa baada ya
reassignment). Vipimo vyote vilivyopo (fibonacci, matriki, kupanga,
heshi, mzunguko_mchezo) vimethibitishwa kuwa BAITI SAWA kati ya
mkusanyaji wa zamani na mpya -- SSCSE haipati fursa mpya kwenye hivyo
(vyote ni vizito vya vitanzi, vilivyoshughulikiwa TAYARI na LICM, au
vina usomaji wa kumbukumbu ulio nje ya wigo wa SSCSE).

### Utendaji

Hakuna kipimo kilichopo kinachotumia mfano wa "usemi unaorudiwa lakini
si invariant ndani ya kitanzi" -- kipimo kipya `vipimo/sscse_mstari/`
kimeundwa MAKUSUDI: "i*i" hutokea mara mbili kwa kila mzunguko (h1=
i*i+i, h2=i*i-i), SIYO invariant (i inabadilika kila mzunguko, hivyo
LICM haiwezi kuigusa), lakini ni straight-line ndani ya mwili wa
kitanzi (hakuna tawi kati ya h1 na h2). Mizunguko 50,000,000.
Disassembly imethibitisha imul MOJA TU (32-bit, %ecx,%eax) ndani ya
kitanzi hicho badala ya MBILI. Muda halisi (interleaved, mizunguko 5,
dhidi ya mkusanyaji wa zamani): wastani zamani 214.6ms, mpya 204.4ms
-- **~4.8% haraka zaidi**, thabiti kwenye mizunguko yote (hakuna
mwingiliano). Ndogo kuliko ilivyotarajiwa kwa sababu ya kufurika kwa
N32 (overflow) kwenye mzunguko wa 50M -- lakini halisi na inayoweza
kuigwa (reproducible).

## Pande mbili bila push/pop kwenye rafu

Njia ya asili kwa kila operesheni ya jozi ilikuwa: tathmini kulia,
sukuma; tathmini kushoto; vuta. Kwa `c[i*n+j] + aik * b[k*n+j]` hiyo ni
safari nne za kumbukumbu za rafu kwa desimali (`sub rsp; movsd
[rsp]` ... `movsd xmm1,[rsp]; add rsp`) na mbili kwa kila namba
kamili, bila sababu yoyote -- disassembly ya kitanzi cha ndani cha
matriki ilionyesha mistari zaidi ya 50 kwa mzunguko mmoja.

`toa_pande_mbili` (uzalishaji.swa) inaacha kushoto kwenye rax/xmm0 na
kulia kwenye rcx/xmm1 kama zamani, kwa njia tatu:

- **A -- kushoto ni jani** (namba, kitambulisho): kulia kwanza (mpangilio
  ule ule wa asili), nakala ya rejesta hadi rejesta, kisha jani.
- **B -- namba kamili, kulia ni jani** (namba ya kudumu, kigezo cha
  rejesta/rafu, paramu; upana sawa na operesheni): kushoto kwanza, jani
  moja kwa moja rcx. Hii inabadilisha mpangilio wa kusoma kigezo, kwa
  hiyo kwa kigezo inahitaji kushoto isiyoweza kukibadilisha
  (`jr_ni_salama`: hakuna wito, hakuna uandishi). Namba ya kudumu
  hasi kwa upana wa 8 haikubaliwi (uzalishaji_usemi huipanua kwa
  ishara, `mov ecx` haifanyi hivyo) -- mdudu huu ulipatikana na
  `jaribio_uchawi_kataa` wakati wa kazi hii, na sasa una jaribio lake.
- **C -- desimali, kushoto haina wito**: kulia huhifadhiwa xmm2..xmm7
  (kina hadi 6), kushoto, kisha xmm1. Rejesta za xmm ni caller-saved,
  kwa hiyo wito ndani ya kushoto ungeziharibu -- ndiyo sababu
  `jr_ni_salama` inahitajika hapa pia.

Vilevile msingi wa `arr[i]` ukiwa kigezo cha kielekezi hupakiwa moja
kwa moja rcx (mpangilio wa asili haubadiliki: msingi ulikuwa unasomwa
BAADA ya faharasa).

Kinachobaki njia ya asili: namba kamili ambapo pande zote si majani
(Sethi-Ullman iliyopo inashughulikia baadhi), desimali ambapo kushoto
ina wito, kina cha xmm zaidi ya 6, upana usiolingana (N8/N16 na N32).

### Uthibitisho

- Majaribio manne mapya (410/410 yanapita, kutoka 406): `namba`,
  `mpangilio` (kushoto inabadilisha kigezo cha kulia kupitia kielekezi au
  ulimwengu), `desimali` (ikiwemo kazi inayotumia xmm2/xmm3 yenyewe
  ndani ya kushoto), `safu`. Kila jaribio limethibitishwa kwa
  *mutation*: kuvunja kila kanuni kwa makusudi (namba hasi bila panua
  ishara, `jr_ni_salama` ikirudisha 1 daima, kina cha xmm bila kikomo,
  `mov rcx` ya baiti 4 badala ya 8) hufanya angalau jaribio moja
  lishindwe.
- Fixpoint stage2 == stage3. Programu 307 (majaribio + vipimo)
  zimeundwa na stage1 na stage2 -- baiti sawa isipokuwa
  `jaribio_gawanyo_nguvu_pili_mipaka`, ambayo inatofautiana vilevile kwenye
  mkusanyaji wa zamani (mdudu unaojulikana wa mbegu.bin: mgawanyo hasi
  uliokunjwa hukusanywa vibaya kwenye gen1 tu).
- Programu 3401 za nasibu (mbegu 1..4000; N32, N64, D64; vielekezi na
  faharasa; ulinganishi) zimeundwa na mkusanyaji wa zamani na mpya --
  matokeo yaliyochapishwa yanafanana kwa zote, sifuri tofauti.
- Vipimo vyote tisa vinatoa matokeo yaleyale kwa mkusanyaji wa zamani na
  mpya.

### Utendaji

Interleaved (mizunguko 9, wastani wa kati), gcc -O2 kama msingi:

| Kipimo | C (ms) | zamani (ms) | mpya (ms) | zamani/C | mpya/C |
|---|---|---|---|---|---|
| fibonacci | 18.4 | 100.5 | 91.7 | 5.46x | 4.98x |
| kupanga | 90.8 | 309.8 | 248.1 | 3.41x | 2.73x |
| matriki | 15.3 | 170.9 | 111.0 | 11.14x | 7.23x |
| heshi | 161.4 | 1153.0 | 992.7 | 7.14x | 6.15x |
| mzunguko_mchezo | 4.0 | 21.5 | 21.5 | 5.26x | 5.27x |
| miti_bst | 62.1 | 108.3 | 108.6 | 1.74x | 1.74x |
| mandelbrot | 66.3 | 207.5 | 158.1 | 3.12x | 2.38x |
| mchujo | 137.9 | 385.0 | 359.4 | 2.79x | 2.60x |
| maneno | 28.1 | 156.3 | 103.5 | 5.55x | 3.67x |

matriki bado ni pengo kubwa zaidi (7.2x): kitanzi cha ndani bado kina
push/pop ya anwani ya `c[...] += ...`, `cltq` baada ya kila operesheni ya
N32, na kaunta ya kitanzi inapita rax kabla ya kurudi kwenye rejesta --
kazi ya baadaye. mzunguko_mchezo na miti_bst hazibadiliki (kazi yao
kuu ni wito wa kazi na miundo, si hesabu ya jozi).

## Awamu ya pili: hesabu bila rafu (kiwanja, kaunta, hoja ya kwanza, uchawi wa N64, cmp)

Inaendeleza `toa_pande_mbili` (sehemu iliyotangulia). Kila kipengele kina
jaribio lake la kudumu na kila kimoja kimevunjwa kwa makusudi (mutation) ili
kuthibitisha kuwa jaribio linakishika.

- **Ugawaji wa kiwanja** (`c[idx] += x`, `-=`, `*=`): thamani ya sasa
  haisukumwi rafuni -- kulia jani -> rcx; desimali isiyo na wito -> xmm2..xmm7;
  vinginevyo rafu. Pia ilikuwa na `push rax; push rax; pop rax` isiyo na maana.
- **Kaunta** `x = x + namba` / `x = x - namba` (N32 yenye ishara): `add
  r32, imm32` + `movsxd` kwa rejesta, `add dword [rbp-off], imm32` kwa rafu
  (badala ya maagizo 6 kupitia rax). Kuzunguka kwa 2^31 kunathibitishwa.
- **`movsxd`**: kusoma kigezo cha N32 chenye ishara kulikuwa `mov` + `cdqe`;
  sasa agizo moja. Rejesta za kudumu HAZIhakikishiwi kupanuliwa kwa ishara
  (`sawazisha_eax_kwa_enc` haifanyi kitu kwa upana wa 4), kwa hiyo `cdqe`
  haiwezi kuondolewa -- ila kuunganishwa kunaokoa agizo na kulipimwa ~10%
  kwenye vipimo kadhaa. (Jaribio la kuondoa `cdqe` ya operanda kabisa
  halikuleta faida yoyote inayopimika, na liliondolewa.)
- **Hoja ya kwanza ya wito** haipiti rafuni: `mov rdi, rax` (au inabaki
  xmm0). Inatumika tu kwa wito wa kawaida: si sret, si syscall, si
  `tekeleza`, si wito wa kielekezi, na hakuna hoja ya muundo (sloti za
  `[rsp + j*8]` zinategemea hoja zote kuwa rafuni).
- **`/ % << >> & | ^`** zinapita `toa_pande_mbili` (kigawanyaji jani).
- **Zidisho na uchawi kwa N64 yenye ishara**: kikwazo cha awali (hakuna hesabu
  ya biti 128 kwenye lugha) kimepitwa kwa jedwali la vigawanyaji 79 (3..64
  visivyo nguvu ya 2, na 22 vya kawaida: 100, 1000, 10^4..10^9, 10^9+7,
  998244353, 3600, 86400, ...) lililohesabiwa nje kwa algorithm ya Hacker's
  Delight na kuthibitishwa kwa C (`__int128`) dhidi ya mgawanyo halisi:
  mamilioni ya thamani nasibu za mizani yote na thamani za mpaka, sifuri
  tofauti. `jaribio_gawanyo_n64_uchawi` inarudia ulinganisho huo ndani ya
  mradi kwa kila kigawanyaji. Kigawanyaji kisicho kwenye jedwali kinabaki idiv.
- **`arr[i]` ya safu ya ndani**: msingi kwa `lea rcx, [rbp-off]`.
- **Ulinganishi wa namba kamili**: `cmp reg,reg`, `cmp reg,[mem]`,
  `cmp [mem],reg`, `cmp L,imm32`, `cmp L,rax`, `cmp rax,R` (REX.W/R/B kwa
  N64 na r8+) badala ya kupakia rax/rcx; mpangilio wa tathmini
  unahifadhiwa (kulia usemi kwanza, kigezo cha kushoto kinasomwa na cmp).

### Uthibitisho

- 419/419 (410 zilizopo + 8 mapya + jaribio la rekebisho la mgawanyo ulioota), imekimbizwa mara 3 kutoka ujenzi safi.
  Fixpoint stage2 == stage3.
- Programu 315 zimeundwa na stage1 na stage2: baiti sawa isipokuwa tatu --
  `jaribio_gawanyo_nguvu_pili_mipaka`, `jaribio_kaunta_namba` na
  `jaribio_wito_hoja_ya_kwanza` -- zote tatu zinatofautiana vilevile kwenye
  mkusanyaji wa kabla ya PR hii (mdudu unaojulikana wa mbegu.bin: baadhi ya
  namba hasi zilizokunjwa, mfano `0 - 2147483647 - 1`, hazikunjwi kwenye gen1;
  tofauti ni anwani za data tu, matokeo ya kukimbia ni sawa).
- Programu 6001 za nasibu (N32/N64/D64, vielekezi, faharasa, mgawanyo,
  hamisha, biti, ulinganishi wa vigezo/namba/usemi, kama sharti na kama
  thamani) zimeundwa na mkusanyaji wa kabla na wa baada: matokeo sawa kwa zote.
- Vipimo vyote tisa vinatoa matokeo yaleyale.

### Utendaji

Interleaved, mizunguko 9, wastani wa kati, gcc -O2 kama msingi; mizunguko
miwili kamili (nambari zilifanana ndani ya ~3%). "zamani" = main baada ya
awamu ya kwanza.

| Kipimo | C (ms) | zamani (ms) | mpya (ms) | zamani/C | mpya/C |
|---|---|---|---|---|---|
| fibonacci | 19.9 | 103.6 | 81.4 | 5.19x | 4.08x |
| kupanga | 97.7 | 263.9 | 229.0 | 2.69x | 2.34x |
| matriki | 16.2 | 117.7 | 78.4 | 7.25x | 4.83x |
| heshi | 161.7 | 976.6 | 613.2 | 6.03x | 3.79x |
| mzunguko_mchezo | 3.9 | 21.0 | 18.8 | 5.40x | 4.83x |
| miti_bst | 61.8 | 107.1 | 111.1 | 1.73x | 1.79x |
| mandelbrot | 65.0 | 155.1 | 149.3 | 2.38x | 2.29x |
| mchujo | 151.3 | 367.6 | 260.8 | 2.42x | 1.72x |
| maneno | 26.2 | 100.6 | 95.6 | 3.84x | 3.65x |

miti_bst ni kipimo cha wito/miundo na inaonekana kubaki sawa ndani ya
kelele (+1% hadi +4% kwenye mizunguko miwili; siwezi kudai ni uboreshaji au
hasara). Pengo kubwa lililobaki: matriki (4.8x), fibonacci (4.1x, gharama ya
wito na fremu), heshi (3.8x, bado maandishi-hadi-nambari ya maktaba).

### Masahihisho kwa awamu ya pili

Awamu hiyo ilikuja na rekebisho la mdudu uliokuwepo main tangu #244:
mgawanyo/modulo wa kudumu ulioota (`x / 10 / 100`) ulitumia namba ya uchawi ya
kigawanyaji cha ndani (`gwm_uchawi_*` na `gwm64_*` ni vigezo vya ulimwengu
vilivyoandikwa juu na mgawanyo wa ndani kabla ya wa nje kuvitumia). Sasa
vinatafutwa tena baada ya kushoto kutathminiwa; `jaribio_uchawi_ndani_ya_ndani`
linashindwa kabla ya rekebisho na linapita baada yake. Majaribio ya nasibu ya
awali hayakuushika kwa sababu vigawanyaji vyake vilikuwa vigezo; njia
iliyoushika ni kulinganisha toleo lenye vigawanyaji vya kudumu na toleo lenye
vigawanyaji vilivyofichwa nyuma ya kazi (idiv halisi), kwa mkusanyaji ule ule.

## Upakiaji wa arr[i] kwa agizo moja la SIB

`arr[i]` ya scalar (N8/N16/N32/N64, A8/A16/A32, D32/D64) yenye msingi jani
(kigezo cha kielekezi cha rejesta/rafu/paramu, au safu ya ndani) sasa
inapakiwa kwa agizo MOJA `[rcx + rax*saizi]`: faharasa kwenye rax (cdqe tu
kama `panua_ishara_ndogo` haikuiongeza tayari), msingi kwenye rcx BAADA ya
faharasa (mpangilio ule ule wa asili), kisha upakiaji. Awali ilikuwa `shl`,
`cdqe`, `add`, kisha upakiaji. Miundo (`z[i].a`), vielekezi vya vielekezi na
msingi usio jani hubaki njia ya asili. Uhifadhi (`arr[i] = x`) haujaguswa.

Mdudu mmoja ulipatikana na jaribio jipya wakati wa kuandika: `jr_msingi_off`
(kigezo cha ulimwengu kilichoshikilia ofseti ya safu ya msingi) kiliandikwa
juu na faharasa ya ndani (`b[a[1] / 10]`) kati ya uamuzi na upakiaji;
`jr_pakia_msingi_rcx` sasa inatafuta tena. Jaribio: `jaribio_safu_sib`.

### Uthibitisho

- 420/420 (419 + 1 mpya), mara 3 kutoka ujenzi safi; fixpoint stage2 == stage3.
- Programu 317 zimeundwa na stage1 na stage2: baiti sawa isipokuwa tatu
  zinazojulikana (mdudu wa mbegu.bin wa kukunja namba hasi; zinatofautiana
  vilevile kabla ya mabadiliko haya).
- Programu 4001 za nasibu, mkusanyaji wa kabla dhidi ya wa baada: sifuri
  tofauti. Programu 2501 za oracle (vigawanyaji vya kudumu dhidi ya
  vilivyofichwa): sifuri tofauti.
- Kila kanuni ya usimbaji (mizani ya SIB, movsx/movzx, REX.W, utafutaji wa
  msingi) imevunjwa kwa makusudi na jaribio limeshindwa.

### Utendaji

Sasa nimepima kwa `perf stat` (mizunguko ya CPU, mizunguko 9, wastani), si
muda wa ukuta, kwa sababu ya kelele ya mashine. "zamani" = main baada ya #250.

| Kipimo | C (mzunguko M) | zamani | mpya | zamani/C | mpya/C | maagizo mpya/zamani |
|---|---|---|---|---|---|---|
| kupanga | 338 | 720 | 639 | 2.13x | 1.89x | 0.88 |
| maneno | 90 | 306 | 226 | 3.39x | 2.51x | 0.91 |
| matriki | 48 | 236 | 214 | 4.92x | 4.47x | 0.91 |
| heshi | 519 | 1877 | 1839 | 3.61x | 3.54x | 0.92 |
| fibonacci | 67 | 266 | 254 | 3.98x | 3.81x | 1.00 |
| mandelbrot | 232 | 499 | 502 | 2.15x | 2.17x | 1.00 |
| mchujo | 596 | 946 | 991 | 1.59x | 1.66x | 0.97 |

Kuhusu mchujo: inaonekana +5% kwenye mizunguko yote, lakini si hasara halisi.
Nilipoongeza kazi tupu kabla ya `main` (kuhamisha msimbo kwa baiti kadhaa) muda
wa mkusanyaji wa ZAMANI na WA MPYA ulibadilika +-6% (915-994 M dhidi ya 912-975 M
mizunguko) -- ni athari ya mpangilio wa msimbo wa kitanzi (mipaka ya 32/64 baiti),
si gharama ya SIB (maagizo yamepungua 3%). Tofauti chini ya ~6% kwenye kipimo
kimoja haipaswi kudaiwa bila jaribio hilo la kuhamisha msimbo. fibonacci na
mandelbrot hazina `arr[i]` moto, kwa hiyo hazibadiliki (maagizo sawa kabisa).

## Rekebisho la mdudu: fahirisi kubwa (idx * saizi > 2^32)

Ukaguzi wa PR #251 uligundua kuwa `andika_ongea_kipengele_kwa_saizi`
(inayotumika kwa `arr[idx] = x`, `&arr[idx]`, na njia ya asili -- isiyo
ya SIB -- ya kusoma) ilizidisha faharasa kwenye `eax` (biti 32) KISHA
kuipanua kwa ishara (`cdqe`). Kwa safu kubwa (`idx * saizi > 2^32`,
mfano safu ya N64/D64 yenye zaidi ya vipengele milioni 537, au N32
yenye zaidi ya bilioni 1.07) zidisho hilo lilizunguka (overflow) KABLA
ya kupanuliwa, likitoa anwani mbaya bila kuanguka wazi.

Kila mwitaji wa kazi hii huiita mara moja tu, mara tu baada ya
`uzalishaji_usemi(idx_node, ...)` -- ambayo TAYARI imeacha `rax` ikiwa
faharasa sahihi ya biti-64 (`panua_ishara_ndogo`, mwisho wa
`uzalishaji_usemi`, tayari imefanya `cdqe` kwa aina zenye ishara; A32
inabaki imepanuliwa kwa sifuri kiasili na x86-64). Suluhisho: zidisha
kwenye `RAX` (biti 64) moja kwa moja (REX.W), bila `cdqe` ya ziada
(haihitajiki -- `rax` tayari ni sahihi). Ni muundo ule ule ambao
`andika_ongea_kielekezi_kipengele` (kwa `p + n`) ulikuwa akitumia
tayari -- kazi hiyo haikuwa na mdudu huu.

### Uthibitisho

- Jaribio jipya `jaribio_fahirisi_kubwa`: safu za N64/D64 (fahirisi
  milioni 550-700, kumbukumbu ya mmap ya uvivu -- RSS halisi ni ndogo)
  na N32 (fahirisi bilioni 1.1), zikiandika, kusoma na kuchukua anwani
  (`&arr[idx]`), kila anwani ikithibitishwa dhidi ya hesabu huru ya
  N64 (`msingi + idx*saizi`, ambayo haina mdudu huu). Jaribio hili
  linashindwa (msimbo 11) kwenye mkusanyaji wa kabla ya rekebisho hili
  (c09c3d5) na linapita baada yake. 421/421.
- Fixpoint stage2 == stage3, mara 3 kutoka ujenzi safi.
- Programu 318 zimeundwa na stage1 na stage2: baiti sawa isipokuwa
  tatu zinazojulikana (mdudu wa mbegu.bin).
- Programu 3001 za nasibu (kabla dhidi ya baada): sifuri tofauti.
  Programu 1500 za jenereta mpya ya faharasa (aina zote, faharasa
  ndani ya faharasa, wito): sifuri tofauti.
- Mutation: kuondoa REX.W (kurudi kwenye zidisho la biti 32)
  kunafanya `jaribio_fahirisi_kubwa` lishindwe (msimbo 11).

### Utendaji

Hakuna mabadiliko ya maana yanayotarajiwa (fahirisi za kawaida za
vipimo vyote tisa ni ndogo mno kufikia 2^32/saizi, na kazi hii
haiitwi tena kabisa kwenye njia ya haraka ya SIB ya kusoma -- #251).
Nilipima hata hivyo: matriki -4%, heshi -2%, mchujo -4% (mizunguko ya
CPU, `perf stat`), zingine ndani ya kelele. `maneno` ilionekana +27%
kwenye kipimo cha kwanza -- nikijaribu kuhamisha msimbo kwa kuongeza
kazi tupu (0..6), mizunguko ilibadilika kati ya 211M na 276M (~30%)
BILA kugusa msimbo wa kazi hii kabisa, ikithibitisha ni kelele ya
mpangilio wa msimbo (alignment), si hasara halisi -- angalia maelezo
sawa kwenye sehemu ya SIB hapo juu.

## Kuondoa "mov al, N" isiyo ya lazima kutoka kwa kila wito

Njia ya jumla ya wito (`uzalishaji_wito`) ilikuwa ikiandika `mov al, N`
(idadi ya rejesta za xmm zilizotumika) KABLA YA KILA wito bila
masharti -- hii ni sharti la ABI ya SysV kwa WITO WA NJE (external)
WA VARARGS PEKEE (mfano `printf` ya libc), ili kazi hiyo ijue idadi ya
hoja za xmm bila kuchunguza aina zake wakati wa kukimbia.

Lugha hii haina FFI (hakuna njia ya kuita kazi halisi ya nje yenye
varargs). Kila lengwa la wito ni mojawapo ya: kazi ya Swa yenye idadi
thabiti ya hoja (ikiwemo `andika`/`andika_stderr` pale njia ya haraka
haipo), wito wa kielekezi (kielekezi hakiwezi kuundwa isipokuwa
kupitia `&jina_la_kazi`, hivyo lengwa ni kazi ya Swa ile ile), syscall
ya moja kwa moja (agizo la `syscall` halisi haliangalii AL kamwe), au
`tekeleza`. `mov al` hii haikuwa na maana kamwe -- imeondolewa.

### Uthibitisho

- Hakuna mutation ya maana ya kujaribu (ni uondoaji wa msimbo mfu --
  hakuna tabia mpya ya kuthibitisha kwa "kuvunja kwa makusudi").
  Uthibitisho ni kwa upana: `jaribio_wito_gharama` (wito wa idadi
  tofauti za hoja za xmm 0..8, wito wa kielekezi, wito ndani ya wito,
  muundo kama matokeo, mfululizo wa wito 50) -- 422/422 kwa jumla,
  mara 3 kutoka ujenzi safi. Fixpoint stage2 == stage3.
- Programu 318 zimeundwa na stage1/stage2: baiti sawa isipokuwa tatu
  zinazojulikana. Programu 4001 za nasibu na 1501 za faharasa
  (mkusanyaji wa kabla dhidi ya wa baada): sifuri tofauti.

### Utendaji

`fibonacci` (kazi ndogo yenye wito mwingi) ndiyo iliyopata faida
kubwa zaidi: maagizo -4.6% (642M -> 612M), mizunguko -2.4% (perf
stat). Vipimo vingine vinabadilika kidogo au ndani ya kelele ya
mpangilio wa msimbo (nimethibitisha kwa jaribio la kuhamisha msimbo
kwa `heshi`, ambayo ilionekana +5% -- mizunguko ilibadilika kati ya
1.67B na 1.77B kwa kuongeza tu kazi tupu, bila kugusa msimbo wa
mabadiliko haya).

### Kazi ya baadaye iliyogunduliwa (haijafanyiwa kazi -- hatari kubwa)

`sub rsp, N` kwenye utangulizi wa kazi bado inahifadhi nafasi ya rafu
kwa vigezo VYOTE (kutoka `param_stack`/`var_stack_idadi`), HATA vile
vilivyopandishwa hadi rejesta za kudumu baadaye (havisomwi/kuandikwa
kamwe kupitia anwani yao ya rafu). Kwa `fib`, hii inamaanisha `sub
rsp, 0x10` inahifadhi baiti 16 zisizotumika kabisa. Kupunguza hili
kwa usalama kunahitaji kuunganisha uamuzi wa ugawaji wa rejesta NDANI
ya mchakato wa kutoa ofseti (si kupunguza jumla ya `sub rsp` peke
yake baada ya ofseti kukwisha gawiwa -- hilo lingehamisha vigezo
vingine chini ya rsp mpya, uharibifu wa rafu). Ni kazi kubwa zaidi,
haijaguswa kikao hiki.

## Kuondoa "cdqe" isiyo ya lazima baada ya kila wito wenye matokeo ya namba yenye ishara

Sawa na kuondoa "mov al" (juu): kila wito wa kazi wenye aina ya
kurudi yenye ishara ndogo (N8/N16/N32) ulikuwa ukipata `cdqe` ya
ZIADA mara tu baada ya "call" (kupitia `panua_ishara_ndogo`, njia ya
JUMLA inayotumika kwa usemi WOWOTE, si maalum kwa wito).

UTHIBITISHO (si dhana): kwa kila kazi ya Swa iliyosajiliwa, aina ya
usemi wa wito huo (`mkaguzi_kagua_wito` inarudisha `kazi_ret_aina[j]`)
ni CHANZO KILE KILE kinachotumika na taarifa ya "rudisha" ya kazi
hiyo hiyo (`AST_RUDISHA`: `badili_mpaka_wa_desimali(expr,
ast_thamani[kazi_node])`, ambapo `ast_thamani[kazi_node]` ni
`kazi_ret_aina[j]` ile ile) -- hivyo `rax` TAYARI imepanuliwa ipasavyo
na kazi ILIYOITWA kabla ya "ret", KILA WAKATI. Kwa wito usiojulikana
(kupitia kigezo, `wito_wa_mfumo`, `tekeleza`) `mkaguzi_kagua_wito`
hurudisha 0 ("aina yoyote"), na `enc == 0` HAIKUWA ikipitisha sharti
la awali la "enc > 0" -- njia hizo hazikuwa zikipata cdqe KABLA ya
rekebisho hili pia, hivyo hakuna mabadiliko ya tabia kwa kesi hizo.

### Uthibitisho

- Jaribio jipya `jaribio_wito_bila_cdqe`: thamani HASI za N8/N16/N32
  (ambapo cdqe iliyokosekana ingeonekana mara moja -- upanuzi wa
  sifuri ungegeuza namba hasi ndogo kuwa kubwa chanya) kupitia
  muktadha wa N64, ulinganishi wa moja kwa moja, mnyororo wa wito 21
  (ishara inabadilika kila hatua), usemi mkubwa wa jumla ya matokeo
  matatu ya wito, na faharasa ya safu. 423/423, mara 3 kutoka ujenzi
  safi. Fixpoint stage2 == stage3.
- Kama uondoaji wa "mov al", hakuna mutation ya maana ya "kuvunja
  kwa makusudi" -- ni uondoaji wa msimbo mfu uliothibitishwa kwa
  hoja hapo juu, si mdudu mpya wa kurekebisha. Programu 4001 za
  nasibu (kabla dhidi ya baada): sifuri tofauti. Nimethibitisha kwa
  disassembly ya moja kwa moja ya `fib` kuwa `cdqe` haitokei tena
  mara moja baada ya `call`.

### Utendaji

`fibonacci`: maagizo -4.9% (612M -> 582M), mizunguko -0.6% (ndogo --
`cdqe` ni agizo la bei nafuu sana kwenye CPU za kisasa, hivyo faida
kubwa zaidi ni shinikizo la mbele la CPU (front-end), si mizunguko ya
moja kwa moja). `mchujo` -1.7%, `maneno` -3.5%. Vingine ndani ya
kelele iliyoelezwa juu.

## Uhifadhi wa arr[idx] = thamani kwa SIB moja kwa moja

Sawa na #251 (ambayo iliongeza SIB kwa KUSOMA `arr[idx]` pekee): sasa
KUANDIKA `arr[idx] = thamani` (isiyo ya kiwanja `+=`/`-=`/`*=`, isiyo
ya muundo) pia hutumia agizo MOJA `mov [r9 + r8*saizi], thamani`
badala ya kukokotoa anwani kamili (shl+add), kuihifadhi kwenye rafu
(push), kutathmini RHS, kisha kuivuta (pop) kabla ya kuhifadhi.

Njia hii inatumika PEKEE pale: msingi ni jani (jr_msingi_aina, kama
kawaida), NA RHS haina wito (jr_ni_salama) -- vinginevyo njia ya asili
(push/pop) inabaki. `r8`/`r9` ni salama kushikilia faharisi na msingi
wakati RHS inatathminiwa kwa sababu jr_ni_salama inakataa wito wowote,
na r8/r9 hazitumiki KAMWE na msimbo wa hesabu safi (zinatumika TU na
wito za syscall/paramu 5+/6+).

### Uthibitisho

- Jaribio jipya `jaribio_uhifadhi_sib`: aina zote za kipengele
  (N8/A8/N16/N32/N64/D32/D64), RHS salama dhidi ya isiyo salama
  (wito), kiwanja (lazima libaki njia ya asili), muundo (lazima
  ubaki njia ya asili), fahirisi kubwa (mwingiliano na rekebisho la
  #252), safu ya ndani. 423/423, mara 3 kutoka ujenzi safi. Fixpoint
  stage2 == stage3.
- Mutation: kuharibu REX.X/B (agizo linatumia rax/rcx badala ya
  r8/r9) kunasababisha SEGV; kuharibu mizani ya SIB kwa saizi 4
  kunatoa matokeo mabaya (jaribio linashindwa); kuharibu lengwa la
  "mov r8, rax" kunasababisha SEGV. (Somo dogo: mutation ya kwanza
  niliyojaribu -- kubadilisha sib=1 kuwa sib=8 kwa saizi=1 -- HAIKUGUNDULIKA
  kwa sababu kwa scale=1 kubadilishana base<->index hakubadilishi jumla
  ya A+B; nilibadilisha kwa mutation yenye maana zaidi.)
- Disassembly ya moja kwa moja imethibitisha `43 89 04 81`
  (`mov %eax,(%r9,%r8,4)`) ikitokea kwenye programu za mfano.
- Programu 5001 za nasibu na 3001 za faharasa (mkusanyaji wa kabla
  dhidi ya wa baada): sifuri tofauti.

### Utendaji

Interleaved, `perf stat` (mizunguko, mizunguko 7):

| Kipimo | Mabadiliko |
|---|---|
| kupanga | -7.5% |
| maneno | -7.0% |
| heshi | -6.1% |
| mchujo | -4.5% |
| mzunguko_mchezo | -1.0% |
| matriki | -0.3% |
| mandelbrot | ~sawa |
| fibonacci, miti_bst | +2.9%/+3.8% (kelele ya mpangilio wa msimbo -- nimethibitisha kwa jaribio la kuhamisha msimbo: fibonacci ilibadilika 224M-237M, miti_bst 336M-351M, KWA KUONGEZA TU kazi tupu bila kugusa msimbo wa mabadiliko haya) |

Hakuna kipimo hata kimoja kilichoonyesha HASARA halisi (baada ya
kuondoa kelele). kupanga, maneno na heshi zinafaidika zaidi kwa
sababu zinaandika kwenye safu mara nyingi kwenye njia zao kuu.

## Uhifadhi wa *ptr = thamani kwa [r9] moja kwa moja

Kioo kidogo cha sehemu iliyotangulia lakini kwa `*ptr = thamani`
(hakuna faharasa/mizani): kielekezi kinashikiliwa kwenye `r9` (jr_jani_rcx)
wakati RHS salama inatathminiwa, kisha `mov [r9], thamani` moja kwa moja
-- badala ya push/pop ya anwani. `r9` ina low3=001 (sawa na rcx), hivyo
ModRM peke yake (bila SIB) inatosha (rm=001 si 100, hivyo hakuna
kulazimishwa kwa SIB byte).

### Uthibitisho

- `jaribio_uhifadhi_ptr`: aina zote, RHS salama/isiyo salama, kielekezi
  kilichotokana na pointer arithmetic, kiwanja (bado njia ya asili).
  425/425, mara 3, fixpoint stage2==stage3.
- Somo la pili la mutation kikao hiki: mutation ya kwanza (kuondoa
  REX.B kwa uandishi wa byte) HAIKUGUNDULIKA na RHS ndogo (namba ya
  moja kwa moja haigusi rcx KAMWE, hivyo rcx ilibaki sahihi kwa bahati
  hata bila REX.B). Nimeongeza kesi za RHS zinazogusa rcx (hesabu)
  kwa kila upana ikiwemo byte -- sasa mutation hiyo inasababisha SEGV.
- Programu 4001 za nasibu: sifuri tofauti.

### Utendaji

Hakuna kipimo cha vipimo tisa vinavyotumia `*ptr = thamani` (bila
faharasa) kwenye njia yake kuu -- vyote hutumia `arr[idx]`
(iliyofaidika tayari kwenye sehemu iliyotangulia). Kama ilivyotarajiwa,
mabadiliko ni ndani ya kelele kwa vipimo vyote tisa (nimethibitisha
matriki hasa kwa marudio matatu ya moja kwa moja: 198.0M/199.1M,
197.7M/198.2M, 198.4M/197.9M mizunguko -- kabla/baada, tofauti ndani
ya 0.5% kila wakati). Faida yake ni kwa msimbo unaotumia vielekezi
moja kwa moja (bila safu), mfano linked-list au miundo ya mtu binafsi
iliyofikiwa kupitia kielekezi kimoja bila faharasa.

## Uhifadhi wa kielekezi->sehemu = thamani kwa [r9+off] moja kwa moja

Kioo cha tatu cha mfululizo huu: `ptr->sehemu = thamani` sasa hutumia
`[r9+off]` moja kwa moja -- OFSETI YA SEHEMU (iliyowekwa na mkaguzi,
0/4/8 n.k.) inaandikwa ndani ya ModRM/disp8/disp32 ya agizo la
kuhifadhi lenyewe, badala ya "add rax, off" tofauti + push/pop ya
anwani. Kielekezi kinashikiliwa r9 (jr_jani_rcx) wakati RHS salama
inatathminiwa, kama sehemu mbili zilizotangulia. Kiwanja (`ptr->f +=
x`) HAIHITAJI hundi ya ziada: hujitokeza kama RHS inayosoma
SEHEMU_MSHALE ile ile, ambayo haiko kwenye orodha nyeupe ya
jr_ni_salama -- inarudi njia ya asili kiotomatiki.

### Uthibitisho

- `jaribio_uhifadhi_mshale`: sehemu ya kwanza (hakuna disp), disp8,
  disp32 (ofseti > 127), RHS salama inayogusa rcx, RHS isiyo salama
  (wito), kiwanja, kielekezi kilichotokana na pointer arithmetic.
  426/426, fixpoint stage2==stage3.
- Mdudu WA KANDO uliogunduliwa (nje ya wigo, HAUJAREKEBISHWA): muundo
  wenye SAFU kama sehemu (`N32 pad[40];` ndani ya `muundo`) unashindwa
  kukusanywa (`; KOSA: 1` kutoka kwa mchanganuzi wa stage2) -- ilikuwepo
  hata kwenye mkusanyaji wa KABLA ya kazi hii (f1v), hivyo si
  imesababishwa na mabadiliko haya. Jaribio hili liliepuka tatizo kwa
  kutumia sehemu nyingi za N32 badala ya safu moja kupata ofseti > 127.
- Mutation: kubadilisha kigezo cha disp8/disp32 (`off < 128` -> `off <
  256`) kulisababisha SEGV kwenye jaribio_uhifadhi_mshale (ofseti ya
  `mbali`, 140, ingeandikwa kama disp8 potovu) -- imekamatwa.
- Fuzz 5001 programu za nasibu: sifuri tofauti.

### Utendaji

**SAHIHISHO (baada ya ukaguzi wa Kandemark):** kipimo cha kwanza cha
kikao hiki kilidai `miti_bst -10.1%` kama faida kuu ya kazi hii, kwa
sababu kitanzi cha uingizaji cha `miti_bst` kinatumia `n->kushoto = n;
sasa->kulia = n;` n.k. moja kwa moja -- ndiyo mfano uliotumika
kuhalalisha kuchagua kipimo hicho. Dai hilo HALIKUWA SAHIHI, kwa
sababu MBILI zilizogundulika wakati wa ukaguzi:

1. **Njia mpya HAITUMIKI kwa sehemu za kielekezi-kwenda-muundo.**
   `uzalishaji_hifadhi_mshale_sib` inalindwa na `ast_thamani[lhs] > 0`
   -- lakini sehemu za aina `Nodi* kushoto;`/`Nodi* kulia;` zina ENC
   HASI kwenye usimbaji wa mkusanyaji huu (muundo uliopachikwa NA
   kielekezi-kwenda-muundo VYOTE hutumia hasi). Kwa hiyo `n->kushoto =
   n;` na `sasa->kulia = n;` -- HASA mfano uliotajwa -- HAZITUMII
   njia mpya kabisa. Imethibitishwa kwa disassembly (mara mbili, kwa
   ukaguzi wa Kandemark na tena hapa): kati ya maandiko matatu ya
   mshale kwa kila uingizaji (`thamani`, `kushoto`, `kulia`), MOJA TU
   (`thamani`, N32 ya kawaida) inaonyesha muundo mpya wa `(%r9)`.
2. **Kipimo cha awali hakikuwa marudio ya kubadilishana (interleaved)**
   -- kilikuwa old-kisha-new mfululizo mmoja. Baada ya marudio 10 ya
   kubadilishana (perf stat -r 5..8, taskset -c 7) siku hii hii: old
   372M-445M, new 349M-416M -- wastani unaonyesha uboreshaji (karibu
   -6% hadi -11% kutegemea kundi la marudio), LAKINI jaribio la pad
   (safu MOJA, mabadiliko YA MPANGILIO TU, sifuri mabadiliko ya
   semantiki) lilionyesha safu ya 322.9M-379.7M (~17.5%!) -- WIGO
   MKUBWA ZAIDI kuliko pengo la old-vs-new lenyewe. Kwa maneno
   mengine: kelele ya mpangilio wa msimbo peke yake (bila kazi hii
   kabisa) inaweza kutoa mabadiliko makubwa kuliko yale
   yanayodaiwa kuwa "faida" -- kipimo hiki, kwenye mzigo wa sasa wa
   mfumo, HAKINA UHAKIKA wa kutosha kutoa namba MOJA ya asilimia.

**Hitimisho la kweli:** njia mpya ni sahihi na salama (fuzzing na
majaribio yanathibitisha), na INAWEZEKANA ina faida ndogo kwa
`miti_bst` (kupitia sehemu ya `thamani` PEKEE, si `kushoto`/`kulia`),
lakini HAIWEZI kudaiwa kwa uhakika kwa idadi mahususi kwenye kipimo
hiki bila upimaji zaidi (mfumo tulivu zaidi, marudio mengi zaidi).
Faida HALISI na iliyothibitishwa vizuri zaidi ya sehemu hii ya
mfululizo (arr[idx]=/`*ptr=`) iko kwenye vipimo vingine (kupanga,
maneno, heshi, mchujo -- angalia sehemu za awali za hati hii).
Kazi ya baadaye inayoweza kuleta faida HALISI kwa miundo kama Nodi
(BST, orodha iliyounganishwa): kupanua `uzalishaji_hifadhi_mshale_sib`
kutambua sehemu za kielekezi-kwenda-muundo pia (maandiko ya kielekezi
ya baiti 8 sahihi kabisa, hayahitaji semantiki ya kunakili muundo).

Matokeo mengine (perf stat, mizunguko 7, siku ya kwanza ya kupima --
haya HAYAKUFUATWA na jaribio la pad kwa kila mmoja, chukua kwa
tahadhari ile ile): fibonacci -1.2%, kupanga ~sawa, matriki -2.4%,
heshi -0.7%, mzunguko_mchezo -2.9%, mandelbrot ~sawa, mchujo +1.0%,
maneno -1.0%. HAKUNA hasara halisi iliyothibitishwa kwenye kipimo
chochote -- lakini pia hakuna faida iliyothibitishwa kwa uhakika
zaidi ya kupanga/maneno/heshi/mchujo (kutoka sehemu za awali za
mfululizo huu, zilizopimwa kwa uangalifu zaidi).
