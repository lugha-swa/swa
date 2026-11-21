# Mafunzo kutoka HolyC

HolyC ni lugha ya TempleOS, iliyoandikwa na Terry A. Davis. Ni mmoja wa ndugu wa karibu zaidi wa Swa: mkusanyaji wake ni wake mwenyewe (hakuna LLVM, hakuna GCC), hutekeleza msimbo papo hapo (JIT), na huendesha kila kitu kwenye ring 0. Hati hii inaorodhesha tunachoweza kujifunza kutoka kwake, kipi tayari kipo kwenye Swa, na kipi hatutachukua.

## Kilichopo tayari

| HolyC | Swa leo |
|---|---|
| JIT: msimbo hukusanywa na kutekelezwa kwenye kumbukumbu | `--papo` ([malengo.md](malengo.md), hatua 2) |
| Ring 0, bila mfumo mwingine chini yake | `--kiini` na [Kawira](https://github.com/lugha-swa/kawira) |
| Aina `U0 U8 I64 F64` | familia `w0 a8 n64 d64` |
| Mkusanyaji, kiunganishi na zana zote ni vyake | mfululizo huru kutoka baiti 393 ([mfululizo-huru.md](mfululizo-huru.md)) |

## Tunachoweza kuchukua

### 1. Lugha ndiyo ganda
TempleOS haina ganda tofauti: mstari unaoandikwa kwenye kidokezo ni HolyC, hukusanywa na kutekelezwa papo hapo.

Kwa Swa: Kawira ipokee mistari ya Swa kutoka kibodi na kuitekeleza. Hatua ya kwanza haihitaji JIT: `mtafsiri.swa` tayari hutekeleza IR moja kwa moja, kwa hiyo ganda la kwanza linaweza kuwa mteremko + mtafsiri ndani ya Kawira. JIT ya `--papo` huja baadaye kwa kasi.

### 2. Mfumo hujikusanya wakati wa kuwasha
TempleOS hukusanya kiini chake kutoka chanzo kila inapowashwa, kwa sekunde chache.

Kwa Swa: mkusanyaji ukiwa ndani ya Kawira (kipengele 1), Kawira inaweza kujijenga yenyewe kutoka chanzo chake. Hilo ni uthibitisho mzuri wa mfululizo huru: mfumo unaojieleza wenyewe.

### 3. Taarifa za ngazi ya juu
HolyC haihitaji `main`: taarifa zilizo nje ya kazi hutekelezwa kwa mpangilio.

Kwa Swa: programu za kawaida zibaki na `anza`. Kwenye ganda (kipengele 1) taarifa za ngazi ya juu ndizo njia ya asili ya kufanya kazi. IR tayari ina kazi ya `<vianzio vya ulimwengu>` inayoendeshwa kabla ya `anza`, kwa hiyo msingi upo.

### 4. Hoja za chaguo-msingi
HolyC: `U0 Chora(I64 x, I64 rangi=15)`; wito `Chora(3)` hutumia 15.

Kwa Swa: `w0 chora(n32 x, n32 rangi = 15)`. Mteremko huongeza hoja zinazokosekana mahali pa wito, kwa hiyo sehemu za nyuma (x86-64, WASM, RISC-V 64, ARM64) hazibadiliki. Kazi ndogo.

### 5. Masafa kwenye `chagua`
HolyC: `case 5...10:`.

Kwa Swa: `hali 5 hadi 10`. Inategemea `chagua` kuingia kwenye IR kwanza (leo iko nje ya wigo, [ir.md](ir.md)).

### 6. Lugha ya mashine ndani ya msimbo
HolyC huruhusu vipande vya kusanyiko ndani ya kazi. Ndivyo vishikizi vya katizo vya TempleOS vimeandikwa bila faili tofauti.

Kwa Swa: kipande cha lugha ya mashine ya Swa ndani ya kazi, kwa muundo uliobuniwa kwa maana (si majina ya Intel yaliyotafsiriwa). Kawira ingeweza kuandika vishikizi vya katizo na kuingia long mode kwa Swa pekee, na `kiini.swm` ingepungua.

### 7. Kujichunguza kwa miundo
Mkusanyaji wa HolyC huhifadhi majina, umbali na aina za sehemu za kila `class` ili programu ziweze kuzisoma wakati wa kuendesha. TempleOS huzitumia kutengeneza fomu na kuhifadhi data bila msimbo wa ziada.

Kwa Swa: `ukubwa(T)` ipo. Hatua inayofuata ni `umbali(T, sehemu)`, kisha jedwali la sehemu za muundo ambalo programu inaweza kulipitia.

### 8. Kubadilisha kazi ikiendesha
Kwenye TempleOS kazi inaweza kufafanuliwa upya wakati mfumo unaendesha, na wito unaofuata hutumia toleo jipya.

Kwa Swa: JIT tayari ina jedwali la anwani za kazi. Kubadilisha anwani moja kwenye jedwali hilo ndiko kubadilisha kazi ikiendesha. Inafaa ganda la Kawira na michezo.

### 9. Hati hai
DolDoc ya TempleOS ni maandishi yenye viungo, picha na msimbo unaotekelezeka, yote kwenye faili moja. Msaada wa mfumo umeandikwa hivyo.

Kwa Swa: muundo mdogo wa hati kwa Kawira (maandishi, viungo, msimbo unaoweza kuendeshwa). Baadaye, baada ya ganda.

### 10. Picha na sauti rahisi kwenye msingi
TempleOS ina skrini moja ya 640x480 yenye rangi 16 na spika ya PC. Kwa sababu hakuna chaguo, kuchora ni wito mmoja.

Kwa Swa: Kawira leo huandika kwenye skrini ya maandishi ya VGA. Hatua inayofuata ni skrini ya picha (fremu ya VGA/VBE) yenye kazi chache: weka pikseli, mstari, mstatili, herufi; na sauti ya spika. Ni msingi wa michezo kwenye Kawira.

### 11. Nidhamu ya ukubwa
Davis aliweka kikomo cha takriban mistari 100,000 kwa TempleOS nzima.

Kwa Swa: kanuni ya "hakuna bloat" tayari ipo. Tunaweza kuifanya ipimike: CI ichapishe idadi ya mistari ya `msingi/` kwa kila PR ili ukuaji uonekane.

## Tusichochukua

- **x86-64 pekee.** Swa ni ya ulimwengu wote: IR moja, sehemu za nyuma nyingi.
- **Ring 0 kama lazima.** Kwenye TempleOS hakuna ulinzi wa kumbukumbu wala ring 3. Swa humpa mpanga programu uchaguzi: programu za Linux, ring 0 kupitia Kawira, na baadaye ring 3 ndani ya Kawira.
- **Mnyororo usioweza kufuatiliwa.** HolyC hujikusanya yenyewe, lakini mkusanyaji wake wa kwanza haukutokana na chanzo kinachoweza kukaguliwa. Swa huanza na baiti 393 za mkono.

## Mpangilio unaopendekezwa

| Hatua | Kipengele | Ukubwa | Kinachohitajika kwanza |
|---|---|---|---|
| 1 | Hoja za chaguo-msingi (4) | kidogo | hakuna |
| 2 | Skrini ya picha na spika kwenye Kawira (10) | wastani | hakuna |
| 3 | Masafa kwenye `chagua` (5) | kidogo | `chagua` kwenye IR |
| 4 | Ganda la Swa kwenye Kawira kupitia mtafsiri (1, 3) | kubwa | mkusanyaji kukusanyika kwa `--kiini` |
| 5 | JIT ndani ya Kawira na kubadilisha kazi ikiendesha (8) | kubwa | hatua 4 |
| 6 | `umbali` na jedwali la sehemu za muundo (7) | wastani | hakuna |
| 7 | Lugha ya mashine ndani ya msimbo (6) | kubwa | muundo wa lugha ya mashine ukubaliwe |
| 8 | Kawira kujikusanya wakati wa kuwasha (2) | kubwa | hatua 4 |
| 9 | Hati hai (9) | wastani | hatua 4 |
