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
