# Vipimo Rasmi vya Lugha ya Swa

Hati hii ni marejeo rasmi ya lugha ya Swa. Kila kanuni hapa
imejaribiwa kwenye mnyororo wa mkusanyaji (mbegu na mnyororo wa
kujikusanya wa .swa) au imeandikwa wazi kama kikomo (tazama
`hati/mipaka.md`). Kila dai la 2026-09-04 linalohusu mbegu
linapima MBEGU ILIYOGANDISHWA ya 2026-09-04 (na stage1 iliyojengwa
kutoka kwake); kila dai la awali linabaki kwenye tarehe yake.
Toleo hili linalenga Swa 1.0.

KUMBUKA (2026-09-05): madai ya 2026-09-04 kuhusu dereva wa Rust ni
rekodi ya kihistoria -- dereva umehamishiwa lugha-swa/swa-dereva.

Uthibitisho kamili wa 2026-08-27 (kesi ~487 zilizokusanywa na
kuendeshwa kwenye minyororo yote miwili) umeandikwa kwenye
`hati/uthibitisho-wa-lugha.md` -- ambapo kipimo kinapinga kanuni ya
hati hii, kipimo ndicho cha kusadikiwa. Sehemu zilizobainishwa hapa
chini ("kilichopimwa") zinaonyesha mahali ambapo kipimo
kilipata tofauti na ahadi ya hati hii.

## 1. Muundo wa Kimsingi

Programu ya Swa ni mfuatano wa tangazo la kiwango cha juu:

- Tangazo la kazi (`n32 jumlisha(n32 a, n32 b) { ... }`)
- Tangazo la muundo (`muundo Nukta { n32 x; n32 y; };`)
- Tangazo la kigezo cha ulimwengu (`n32 KIKOMO = 0;` na safu za
  ulimwengu `n8 bafa[1024];`)
- Tangazo la utegemezi (`husisha { faili.swa }`) -- kwenye mnyororo
  wa uzalishaji ni kiungo HALISI: faili linatatuliwa na kujumuishwa
  kwa mpangilio wa utegemezi (sehemu 8). Mbegu haitatui -- inaruka
  mstari kimya. Kiungo cha C ni `husisha C::stdio`

Sehemu ya kuingia ni kazi `anza`. Katika hali ya `--programu`, sahihi ni
`n32 anza()` au `n32 anza(n32 argc, n8** argv)`.

Kila faili huchanganuliwa kwa mpangilio wa juu-chini; wito wa mbele
unaruhusiwa (kazi inaweza kuitwa kabla ya kutangazwa).

## 2. Leksia

### 2.1 Vitambulisho

Vitambulisho huanza na herufi (a-z, A-Z) au `_`, na vinaendelea kwa
herufi, tarakimu, au `_`. Majina yote ya lugha, maktaba, na maoni ni
Kiswahili kwa mkongwe.

### 2.2 Maneno Muhimu

`muundo`, `rudisha`, `kama`, `sivyo`, `wakati`, `kwa`, `fanya`,
`acha`, `endelea`, `chagua`, `hali`, `husisha`, `achilia`.

Kiingilio cha programu ni kazi `anza` (kama `anza:` ya lugha ya
mashine), na kazi ya ndani ya kuandika kwenye mkondo wa makosa ni
`andika_kosa`. Maneno ya v0.0.1 `vunja`, `main` na `andika_stderr`
(pamoja na `kernel_main`, `kbd_scancode`, bendera `--exe`/`--kernel`/`--ir`/`--jit`
na majina 33 ya maktaba) yameondolewa: si maneno muhimu tena, na
programu ikiyatumia bila kuyafafanua, kosa linataja jina jipya
(`kosa: 'vunja' imeondolewa -- tumia 'acha'`).

Maneno muhimu hayawezi kutumika kama majina ya vitambulisho.
`tenga` si neno muhimu -- ni kazi ya kawaida ya maktaba (sehemu 10):
`tenga(ukubwa(n32))` huweka kipande cha baiti `ukubwa(n32)`.
Aina za nambari haziko kwenye orodha hii -- hutambuliwa kisintaksia
kwa herufi NDOGO: `[n|a|d|b|w]` ikifuatiwa na tarakimu (mf. `n32`,
`d64`, `w0`). Herufi kubwa (`N32`, `D64`) si aina tena: ni
kitambulisho cha kawaida, kama `Nukta`. Jina lolote linalolingana na
muundo wa aina haliwezi kuwa jina la muundo (`muundo n32 { ... }`
linakataliwa kwa sauti).

Kilichopimwa (2026-09-04): madai ya zamani kwamba dereva wa Rust
(majaribio) una "maneno muhimu ya ziada" (`fanya`, `muungano`,
`kutoka`, `badili`, `nakili`, `ukubwa`, `nenda`) yalichanganya
leksia na sarufi -- dereva wa Rust huchambua maneno ya taarifa
yale yale kama mkusanyaji wa uzalishaji. Kila neno la orodha hiyo
ya zamani ni jambo jingine:
- `fanya` ni neno muhimu kamili -- kitanzi cha mwili-kwanza
  (sehemu 6.8). Kitanzi chenyewe bado hakijatekelezwa na sarufi
  ya mwisho wa Rust (kilichopimwa 2026-09-04) -- leksia pekee
  haitengenezi neno muhimu.
- `kutoka` si neno la taarifa -- kwenye mchanganuzi wa Rust wa
  majaribio linaonekana tu kama kifungu cha hiari cha saraka
  ndani ya `husisha`: `husisha { njia } kutoka { saraka }`
  (sehemu 8).
- `ukubwa` si neno muhimu -- ni KAZI ya ndani ya upimaji (sizeof):
  `ukubwa(n32)` inakokotwa wakati wa kukusanya na kurudisha
  ukubwa wa aina (n8=1, n16=2, n32=4, n64=8, d32=4, d64=8;
  muundo = ukubwa wake) -- si wito wa kazi wakati wa kukimbia.
  Maktaba hubeba kazi ya jina hilo (`n64 ukubwa(n32 aina)`,
  kumbukumbu.swa) kwa minyororo isiyo na ndani hiyo ya upimaji.
  Kwa kuwa si neno muhimu, `ukubwa` linaweza kuendelea kuwa jina
  la kigezo (kilichopimwa 2026-09-04).
- `badili` na `nakili` ni KAZI za maktaba
  (msingi/maktaba/kumbukumbu.swa) -- si maneno muhimu; saini zao
  kamili ziko sehemu 10.
- `muungano` na `nenda` si sehemu ya lugha -- hakuna mnyororo
  (uzalishaji wala majaribio) wenye sintaksia au semantiki kwao
  (kilichopimwa 2026-09-04).

### 2.3 Halisi

- Nambari kamili: mfuatano wa tarakimu. Aina yake ya chaguo-msingi ni
  n32 ikiwa inatoshea ndani ya 32-bit signed; nje ya hapo ni n64.
  Kilichopimwa (2026-09-04, mbegu iliyogandishwa 2026-09-04 na
  stage1): halisi za [2^31, 2^63) kwenye kigezo, ulimwengu, na kama
  hoja ya wito -- SAWA kwa minyororo yote miwili; halisi >= 2^63
  (2^63-1, 2^63, 2^64-1, 2^64) zinatoa imm64 KAMILI kwa minyororo
  yote miwili (2^63 = -2^63 kwa ishara; 2^64-1 = -1 kwa ishara;
  2^64 inazunguka kimya hadi 0 -- mod 2^64). Mgawanyo wa n64 wenye
  halisi kubwa (`9223372036854775807 / 1000000000`) hufanya kazi --
  FPE ya zamani haipo. Ukataji wa zamani wa "halisi >= 2^63 hadi
  biti 32" haushikiki, na kikomo cha zamani cha "2147483648
  inageuka -2147483648" kimeondoka kwenye mbegu pia (angalia
  `hati/mipaka.md` sehemu ya 5 kwa rekodi).
- Mfuatano: `"habari"` -- baiti za n8 zikifuatiwa na 0. Utorokaji:
  `\n`, `\\`, `\t` na `\"` zinafanya kazi kwenye minyororo yote
  miwili (kilichopimwa 2026-09-04: `\t` na `\"` kwenye mnyororo wa
  uzalishaji zilikuwa zikichapisha alama halisi 5c 74/5c 22 --
  zimefungwa).
- Desimali: `21.5` -- d64. Hesabu, ulinganisho na ukanushaji NDANI ya
  ulimwengu wa d64 zinafanya kazi kwenye minyororo yote miwili,
  ikijumuisha wito wa kazi kwa ABI ya xmm kwenye MBEGU NA mnyororo
  wa uzalishaji (kilichopimwa 2026-09-04: hoja 1-3 za d64 na
  kurudisha kwa xmm0 zimepimwa kwa mbegu iliyogandishwa 2026-09-04 --
  "ABI ya uhamisho wa GP kwenye mbegu" ya zamani haishikiki). Kikomo
  cha mbegu: d64 iliyochanganywa na hoja 7-9 inakataliwa kwa sauti
  ("hoja za d64 zilizochanganywa na hoja 7-9 hazisaidiwi bado na
  mbegu"); mnyororo wa uzalishaji hushughulikia hoja hiyo.
  Kilichopimwa (2026-09-04): MPAKA kati ya d64 na nambari kamili
  UMEFUNGWA kwenye minyororo yote miwili -- d64 hadi n32 (kianzio,
  ugawaji, hoja ya paramu, kurudisha) unakata hadi sifuri (`n32 x =
  2.5;` huweka 2; kazi ya n32 inayorudisha `21.5` inarudisha 21);
  n32 hadi d64 (kianzio, ugawaji, hoja, kurudisha) unabadilisha
  (`mara_mbili(3)` hupata 6.0); operesheni mchanganyiko (`1 + 2.5` =
  3.5, `5.5 * 2` = 11.0) na ulinganisho mchanganyiko (`2.9 < 3`,
  `3 == 3.0`) zinafanya kazi kwa ubadilishaji wa namba hadi d64.
  Mabaki yaliyopimwa 2026-09-04:
  - d64 MOJA KWA MOJA kama sharti la `kama` au `wakati` (`kama (2.5)`,
    `wakati (x)` kwa x ya d64) ni UONGO daima kwenye minyororo yote
    miwili (jibu baya la kimya) -- linganisha kwanza: `kama (x != 0.0)`.
  - Kigezo cha ULIMWENGU chenye aina ya d64 (`d64 G;`): mnyororo wa
    uzalishaji unakubali -- kianzio chake ni lazima kiwe halisi ya
    desimali MOJA (`d64 G = 2.5;`; halisi ya nambari `d64 G = 5;` na
    usemi `1.5 + 1.0` vinakataliwa kwa sauti). Mbegu inakataa kwa
    sauti kila kigezo cha ulimwengu cha d64 (hata `d64 G = 2.0;` na
    `d64 G;`) pamoja na kila kianzio chenye halisi ya desimali kwa
    aina yoyote (`n32 G = 2.5;`) -- kwa kosa lisilo sahihi linalosema
    "kianzio cha ulimwengu wa d64 lazima kiwe halisi ya desimali".
  - Halisi yenye nukta bila tarakimu ya sehemu (`5.`) inaangusha
    mkusanyaji wa mbegu (SEGV 139) wakati wa kukusanya; mnyororo wa
    uzalishaji unaikataa kwa sauti. Inarekebishwa.
  d32 (baiti 4) imefunga kwenye mnyororo wa uzalishaji: hesabu,
  ulinganisho, wito wa kazi, kigezo cha ulimwengu, na ubadilishaji
  d32<->d64 zote hufanya kazi (kilichopimwa 2026-09-04 -- poromoko la
  zamani halipo). Mbegu haijui aina ya d32 kabisa ("aina
  isiyojulikana 'd32'").

### 2.4 Maoni

`//` hadi mwisho wa mstari. Vizuizi `/* ... */` vinafanya kazi
kwenye mnyororo wa uzalishaji (upachikaji unaruhusiwa kwa vitendo,
kinyume cha maelekezo ya awali); mbegu inakataa vizuizi vyote vya
maoni kwa sauti (kosa la mchanganuzi "ulichanganuzi" kati ya
taarifa; "operanda ya kulia haipo" katikati ya usemi) -- poromoko
la zamani halipo. Kilichopimwa 2026-09-04.

### 2.5 Ishara

`+ - * / % << >> < > <= >= == != && || & | = ? : ( ) { } [ ]
-> * & , ;`

Kilichopimwa (2026-08-27): `!` na `~` zilikuwa zinakataliwa kwa
minyororo yote miwili (mbegu: "herufi isiyojulikana"; uzalishaji:
kosa la mchanganuzi) -- ahadi za sehemu ya 3 (matokeo ya `!` ni 1 au
0) na ya orodha hii hazikushikika. `^` ilichanganuliwa na mnyororo
wa uzalishaji lakini usemi wa kulia ulitupwa kimya (jibu baya);
mbegu inakataa. Viendeshaji vya kiwanja `+= -= *= /= %=` vilifanya
kazi kwenye uzalishaji pekee; `&= |= ^= <<= >>=` vilikubaliwa na
kuwa NO-OP kimya kwenye uzalishaji.
Kilichopimwa (2026-08-31): mnyororo wa uzalishaji sasa unatekeleza
`!`, `~`, `^` (utangulizi wa C: `&` juu ya `^` juu ya `|`), na
viendeshaji vya kiwanja `&= |= ^= <<= >>=` (pamoja na n64). `!`
na `~` kwa operesheni ya d64 zinakataliwa kwa sauti. Kilichopimwa
(2026-09-04, mbegu iliyogandishwa 2026-09-04): mbegu pia ina `^`,
`!`, na `~` kwa namba (sawa na uzalishaji), na inakataa `!`/`~`
kwa d64 kwa sauti ("kiambishi ! hakikubaliki kwa desimali").
Viendeshaji vya kiwanja vyote (pamoja na `+=` na `<<=`) bado
vinakataliwa na mbegu kwa sauti -- kwa kosa lisilo sahihi la
"operanda ya kulia haipo" -- uzalishaji unavitekeleza. `%` na `<<`
kwa d64 zinakataliwa kwa sauti kwenye minyororo yote miwili
("operesheni hii haifanyi kazi kwa desimali").
Kilichopimwa (2026-08-31): `++` na `--` HAVIPO katika lugha --
vipimo vya awali havikuwahi kuviahidi. Mnyororo wa uzalishaji
unavikataa kwa sauti ("kosa: '++' haitekelezwi; andika
'x = x + 1' badala yake"); zamani vilikubaliwa kimya kama no-op
(jibu baya la kimya: `x++;` ilirudisha thamani isiyobadilika).
Operanda ya kulia iliyopotea kwenye kiendeshi chochote binari
(mf. `x = x + ;`) pia inakataliwa kwa sauti na mchanganuzi
("operanda ya kulia haipo"). Majaribio:
jaribio_hitilafu_kata_ongezaji, jaribio_hitilafu_kata_punguzaji,
jaribio_hitilafu_kata_operanda_iliyopotea.

## 3. Aina

| Aina | Maelezo |
|---|---|
| `n8` | Nambari kamili yenye ishara, baiti 1 |
| `n16` | Nambari kamili yenye ishara, baiti 2 |
| `n32` | Nambari kamili yenye ishara, baiti 4 |
| `n64` | Nambari kamili yenye ishara, baiti 8 |
| `w0` | Bila thamani (void) -- kwa kazi tu. Kilichopimwa 2026-09-04: PARAMU ya w0 (`n32 f(w0 x)`) inakataliwa kwa sauti kwenye minyororo yote miwili ("parameta ya w0 haikubaliki -- w0 ni kwa kazi tu" kwenye mbegu; "kigezo 'x' cha w0" kwenye mkaguzi). Kigeu cha NDANI cha w0 chenye kianzio cha thamani (`w0 x = 5;`) kinakataliwa kwa sauti kwenye mnyororo wa uzalishaji ("kigeu 'x' cha w0 chenye kianzio -- w0 ni kwa kazi tu"); mbegu bado inakubali kimya -- inarekebishwa (mgandisho ujao wa mbegu). Tangazo tupu `w0 x;`, ugawaji wa baadaye `x = 5;`, na w0 ya ulimwengu bado wanakubaliwa kimya kwenye minyororo yote miwili. w0 katika usemi (mf. `y = x + 1`): uzalishaji unakataa kwa sauti; mbegu inakubali kimya na kuihesabu kama namba. Matokeo ya kazi ya w0 kama thamani: uzalishaji unakataa kwa sauti; mbegu inakubali kimya |
| `d64` | Desimali, baiti 8 -- mpaka wake na nambari kamili umefungwa; mabaki yaliyopimwa, angalia 2.3 |
| `b1` ... `b64` | Mifumo ya biti (bendera, maski) -- si nambari. Oparesheni: `& \| ^ ~ << >> == !=` pekee; `+ - * / %`, `< > <= >=` na `-x` zinakataliwa kwa sauti. Thamani inayoingia huhifadhi biti za chini za upana wake (`b8 x = 300` ni 44; `b1` ni biti moja: `b1 x = 5` ni 1). Upande wa kulia wa `<<`/`>>` ni idadi, namba kamili yoyote. |
| `T*` | Kielekezi kwa aina T |
| `T[n]` | Safu ya vitu n vya aina T |
| `muundo` | Muundo uliotangazwa na mtumiaji |

Familia za nambari ni n (kamili yenye ishara), a (asili/bila ishara),
d (desimali), b (mifumo ya biti -- angalia jedwali), na w (upana wa mashine; w0 = void).
Matokeo ya ulinganisho (`==`, `<`, ...) na `&&`/`||` ni **n32** yenye thamani 0 au 1 -- si aina ya familia b.
Upana unaosaidiwa na kufanya kazi kwa usahihi ni 8, 16, 32 na 64
pekee (d32/d64 kwa desimali; pia b1 -- biti moja). Kilichopimwa (2026-09-04): upana
mwingine wowote (n128, a128, d80, a3, n7, n.k.) unakataliwa kwa
SAUTI kwenye minyororo yote miwili ("aina isiyojulikana") --
ukubali wa kimya wa zamani na poromoko la mbegu havipo tena.
(Mbegu haijui familia nzima ya a/b/w na d32 pia -- kila moja ni
"aina isiyojulikana" kwenye mbegu; mnyororo wa uzalishaji unazijua
-- a64 na d32 zilizopimwa 2026-09-04.) Hakuna neno muhimu la
"tupu" au halisi za "kweli"/"uongo" -- w0 hutumika kwa bila-thamani
na 1/0 kwa ukweli.

Matokeo ya `==`, `!=`, `<`, `>`, `<=`, `>=`, `&&`, `||` ni thamani
ya 1 (kweli) au 0 (si kweli). Kiambishi `!` hutoa 1 (operesheni ni
0) au 0 (operesheni si 0) kwenye minyororo yote miwili (mbegu
imeipata -- kilichopimwa 2026-09-04).

## 4. Usemi na Utangulizi

Utangulizi wa ishara ni ule wa C (kutoka juu hadi chini):

| Kina | Ishara |
|---|---|
| 12 | `*` `/` `%` |
| 11 | `+` `-` |
| 10 | `<<` `>>` |
| 9 | `<` `>` `<=` `>=` |
| 8 | `==` `!=` |
| 7 | `&` |
| 6 | `^` |
| 5 | `|` |
| 4 | `&&` |
| 3 | `||` |
| 2 | `?:` (ternary) |
| 1 | `=` (ugawi) |

Mabano hubadilisha utangulizi. Chaguo la ternary `sharti ? kweli :
uwongo` linasaidiwa; upande wa uwongo wa ternary haushiki ugawi
(`a ? b : c = d` ni `(a ? b : c) = d`). Ugawi ni wa ushirika wa
kulia (`a = b = c` ni `a = (b = c)`); viendeshaji vingine vyote
vya binary ni vya ushirika wa kushoto.

Mifano: `4 | 2 & 1` ni `4 | (2 & 1)` = 4, `12 & 10 | 3` ni
`(12 & 10) | 3` = 11, `a = 1 && 0` inagawia a = 0,
`a = 2 == 2` inagawia a = 1, `x = 1 ? 2 : 3` inagawia x = 2.

Kilichopimwa (2026-08-31): jedwali hili linashikiliwa na minyororo
yote miwili (mbegu na mnyororo wa .swa) -- tofauti ya zamani kati
ya minyororo (mbegu ilikuwa na `&&` `||` `&` `|` `^` katika
kiwango kimoja na `=` pamoja na `==`) imefungwa kwa kufuata
utangulizi wa C kwenye mbegu. Pia: `<<`/`>>` upande wa KULIA wa
`< > <= >=` haubaliwi kwenye minyororo yote miwili (`1 < 2 << 1`
hutoa kosa la mchanganuzi) hata ingawa jedwali la juu linaweka
uhamishaji juu ya ulinganisho; `2 << 1 < 4` inafanya kazi.
`^` ilikuwa inakataliwa na mbegu (tangu 2026-09-02 mbegu ina
`^` kwa utangulizi wa C -- angalia 2.5); mnyororo wa uzalishaji
ulirudisha operanda ya kushoto pekee (jibu baya) na tangu
2026-08-31 unatekeleza `^` kwa utangulizi wa C.

### 4.1 Mantiki ya fupi-hali (short-circuit)

`&&` na `||` zinatathmini kwa fupi-hali KATIKA MBEGU NA MNYORORO WA
.SWA:

- `a && b`: `b` haitathminiwi ikiwa `a` ni 0. Matokeo: 1 ikiwa zote
  mbili si 0, sivyo 0.
- `a || b`: `b` haitathminiwi ikiwa `a` si 0. Matokeo: 1 ikiwa
  yoyote si 0, sivyo 0.

Hii inaruhusu `j >= 0 && a[j] == x` bila kusoma nje ya mipaka.
Rekebisho la 2026-08: mbegu ilikuwa inatathmini pande zote mbili
(kinyume na uzalishaji.swa) -- imewianishwa; majaribio ya kurejesha
ni `jaribio_awali_mzunguko_mfupi`.

### 4.2 Hesabu ya kielekezi

- `&x` -- anwani ya kigezo, sehemu ya muundo, au kazi (`&jina_la_kazi`
  hutoa anwani ya msimbo wa kazi -- kielekezi cha kazi, 4.3). Jina la
  kazi PEKEE (bila `&`) halitumiki kama usemi kwenye mnyororo wa
  uzalishaji -- linakataliwa kwa sauti ("jina la kazi halitumiki
  kama usemi"); mbegu inakubali kimya.
- `*p` -- nyoosha: thamani iliyoko kwenye anwani p.
- `p->sehemu` -- sehemu ya muundo kupitia kielekezi.
- `a[i]` -- safu au kielekezi: `*(a + i * ukubwa_wa_kipengele)`.
- Usemi wa `safu` pekee hutathminiwa kama kielekezi kwa kipengele
  chake cha kwanza.

Kilichopimwa (2026-09-04, minyororo yote miwili): faharisi `a[i]`
inafanya kazi kwa safu na kwa kielekezi, na `*(p + n)` inahamisha
kwa VIPENGELE (si baiti): `*(p + 3)` kwenye n32* inasoma kipengele
cha 3 -- kwa n8, n32, n64 na kwa kielekezi cha muundo (kipengele
kizima, k.m. muundo wa baiti 12). `p - n`, `n + p`, na kuoza kwa
safu `*(safu + n)` zinafanya kazi kwenye minyororo yote miwili;
safu za miundo zinafanya kazi kwa mwendo wa kipengele. Ukaguzi wa
aina kwenye mnyororo wa uzalishaji unakataa kwa sauti `p + p`,
`p - p`, na `n - p` ("kielekezi kwa kielekezi kwenye hesabu",
"namba toa kielekezi haina maana") -- mbegu bado inakubali kimya.
`%` kwenye kielekezi (`p % q`, `p % 2`) inakataliwa kwa sauti na
mkaguzi wa mnyororo wa uzalishaji ("kielekezi kwenye modulo haina
maana"); mbegu bado inakubali kimya -- inarekebishwa (mgandisho
ujao wa mbegu).

### 4.3 Anwani ya kazi na wito kupitia kielekezi cha kazi

Lugha haina saini za kazi kama aina -- kielekezi cha kazi ni thamani
ya baiti 8 inayoshikilia anwani ya msimbo, kama kielekezi kingine
chochote. Imepimwa (2026-09-04) kwenye minyororo yote miwili --
mbegu iliyogandishwa 2026-09-04 ina vielekezi vya kazi pamoja na
mnyororo wa uzalishaji, kwa tabia zile zile (kila kipengele cha
sehemu hii kimepimwa kwa zote mbili isipokuwa kinapobainishwa).
Mbegu haina familia ya a (a64 kama hifadhi ni ya uzalishaji
pekee).

Kuchukua anwani:

```
n8* kazi = &nyongeza;   // kielekezi chochote (T*) au n64 hushika anwani
n64 k2 = &jumlisha3;
```

- `&jina_la_kazi` hutoa anwani ya mwanzo wa kazi. Inafanya kazi kwa
  wito wa mbele (kazi iliyotangazwa baadaye) -- disp32 inarekebishwa
  mwishoni mwa kukusanya.
- Thamani inaweza kuhifadhiwa kwenye kigezo/paramu/kigeu cha
  ulimwengu cha aina ya KIELEKEZI (T* -- `n8*` ni aina ya jumla) au
  namba ya upana 64 (`n64` kwa minyororo yote miwili; `a64` kwenye
  mnyororo wa uzalishaji pekee -- mbegu haijui familia a). `&f` kama
  HOJA ya paramu ya n64/a64 inakubaliwa kwa minyororo yote miwili --
  anwani ya kazi ni thamani ya baiti 8 kama paramu yenyewe (mkaguzi
  anatoa ubaguzi wa usemi wa anwani ya kazi dhidi ya paramu ya namba
  ya upana 64); vigeu VINGINE vya kielekezi kwa paramu ya namba
  vinakataliwa kwa sauti kwenye minyororo yote miwili ("kielekezi
  dhidi ya namba"). Kigezo cha ulimwengu kinaweza kupewa kianzio cha
  anwani ya kazi: `n64 kazi_kuu = &nyongeza;` (hali ya --programu pekee
  -- imepimwa kwa minyororo yote miwili; aina ya baiti 8 -- kielekezi
  au n64/a64; si safu; si d64) -- anwani kamili ya .text huandikwa
  mwishoni mwa kukusanya, kwa hiyo .o na JIT hazikubali kianzio
  hicho (gawa ndani ya anza badala yake).

Wito kupitia kielekezi -- `kigezo(hoja...)` -- jina ambalo SI kazi
linalinganishwa na kigeu/paramu/kigeu cha ulimwengu:

- ABI ni ile ya wito wa kawaida: hoja 1-6 kwa rejesta (rdi..r9 kwa
  namba/kielekezi, xmm0..xmm7 kwa d64/d32), hoja 7+ kwenye rafu,
  matokeo kwenye rax/eax (na xmm0 kwa desimali). Kielekezi hupakiwa
  kwenye r11 na wito hutolewa kwa `call r11`.
- Aina za hoja huamuliwa na USEMI wa hoja kwenye wito (saini ya kazi
  haijulikani wakati wa kukusanya): `k(2.5)` hupita xmm0, `k(2)`
  hupita rdi. Hivyo mlangaji lazima alinganishe aina mwenyewe --
  hoja ya d64 kwa kazi inayotarajia namba (au kinyume) ni jibu baya
  lisilogunduliwa.
- Matokeo ya wito hayajulikani wakati wa kukusanya: hutumiwa kwa
  usalama kama namba au kielekezi (hifadhi kwenye kigezo chenye aina
  kabla ya hesabu za upana usiojulikana). MATOKEO YA d64
  HAYAKUBALIKI -- mkaguzi anakataa kwa sauti matumizi ya wito
  kupitia kigezo katika muktadha wa d64 (kianzio, ugawaji, rudisha,
  hoja ya paramu ya d64): matokeo yanakuja kwenye eax/rax, si xmm0.
- Kazi za w0 zinaitwa sawa (`k(40);` bila matumizi ya matokeo).
- Uhalali: kigezo kinachoitwa lazima kiwe kielekezi (T*) au namba ya
  upana 64 -- la sivyo kosa la aina (mnyororo wa uzalishaji unatoa
  kosa la aina "aina yake si kielekezi cha kazi"; mbegu inakataa
  kama wito wa kazi usiojulikana). Kikomo cha hoja ni 16 kama wito
  wa kawaida kwenye mnyororo wa uzalishaji; mbegu ina kikomo cha
  hoja 9 ("wito wenye hoja zaidi ya 9"). Kazi zinazorejesha muundo
  (sret) hazisaidiwi kupitia kielekezi (mpangaji wa sret anajulikana
  kwa wito wa kawaida pekee) -- wito kama huo unakusanywa na kuanguka
  SEGV wakati wa kukimbia kwenye minyororo yote miwili
  (kilichopimwa 2026-09-04).

Mfano (jaribio_kazi_kielekezi_kama_hoja):

```
n32 nyongeza(n32 x) { rudisha x + 1; }
n32 endesha(n8* kazi, n32 thamani) {
    rudisha kazi(thamani) + kazi(thamani);   // 21 + 21
}
n32 anza() {
    rudisha endesha(&nyongeza, 20) - 42;     // 0
}
```

## 5. Tangazo la Kazi

```
<aina> <jina>(<vigezo>) { <mwili> }
```

- Vigezo: `n32 a`, `n32* p`, `Orodha* o` (muundo kwa kielekezi au kwa
  thamani).
- `rudisha <usemi>;` kwa kazi yenye thamani; `rudisha;` kwa w0.
- Wito wa kujirudia na wito wa mbele unasaidiwa.
- Hoja zinazozidi rejesta (namba 6, desimali 8) huenda rafuni kwa
  mpangilio wa hoja (System V), hata hoja za rejesta zikizifuata
  (rekebisho la 2026-11-09; mbegu bado haina -- `jaribio_hoja_rafu_mchanganyiko`
  ni ya stage1 pekee).
- Kikomo cha hoja kwenye mnyororo wa uzalishaji: 16 -- wito wa hoja
  zaidi ya 16 unakataliwa kwa sauti ("una hoja nyingi mno (17) --
  kikomo ni 16"); mdudu wa 2026-09-01 (hoja zaidi zilitupwa kimya)
  umefunga. Mbegu ina kikomo cha hoja 9: wito wa hoja 10+ unakataliwa
  kwa sauti ("wito wenye hoja zaidi ya 9"). Kilichopimwa 2026-09-04.
- Kielekezi na namba hazichanganyiki kwenye hoja: kielekezi kwa
  kigezo cha namba -- au namba kwa kigezo cha kielekezi -- ni kosa
  la aina (mdudu wa 2026-09-01: kilikubaliwa kimya na kuanguka
  SEGV wakati wa kukimbia). `andika`/`andika_kosa` na mkia wao
  (pamoja na `andika_ndani` a1..a6) ni za kutofautiana: mkia hupita
  kwa rejesta za n64 iwe thamani au kielekezi.

## 6. Taarifa

### 6.1 Tangazo la kigezo

`n32 x = 5;` -- ndani ya block. Kianzilishi ni cha hiari
(`n32 x;` inaruhusiwa) lakini thamani ya kigezo kisichoanzishwa ni
ISYOFANULIWA (kumbukumbu ya rafu isiyoanzishwa) -- usiisome kabla ya
kuigawa.

### 6.2 Ugawi

`x = usemi;` -- ugawi ni usemi (matokeo yake ni thamani iliyogawiwa).

### 6.3 Kama/sivyo

```
kama (sharti) { ... }
sivyo { ... }              // hiari

// Tawi-jingine (else-if) huandikwa kwa kuingiza kama ndani ya sivyo:
kama (sharti1) { ... }
sivyo {
    kama (sharti2) { ... }
    sivyo { ... }
}
```

Mwili wa `kama` NA wa `sivyo` unaweza kuwa taarifa MOJA bila
mabano (semantiki ya C): `kama (1) a = 1; sivyo a = 2;` huweka
a = 1. Kilichopimwa 2026-09-01: mwili wa `sivyo` bila mabano
ulitekelezwa KILA mara (taarifa iliyotiririka nje kama taarifa
isiyo na masharti) -- sasa ni taarifa moja, sawa na `kama`.

### 6.4 Wakati

```
wakati (sharti) { ... }
```

### 6.5 Kwa (for)

```
kwa (kianzilishi; sharti; hatua) { ... }
```

Sehemu zote tatu ni za hiari; hali iliyoachwa wazi ina maana KWELI
DAIMA -- `kwa (i = 0; ; i = i + 1)` na `kwa (;;)` ni vitanzi
visivyoisha hadi `acha` au `rudisha` (semantiki ya C).
**Semantiki ya `endelea`:** inaruka kwenye HATUA (ya tatu), si
kwenye sharti -- semantiki ya C, katika mbegu NA mnyororo wa .swa
(uliowiana 2026-08; angalia mipaka.md 4b).

Kilichopimwa (2026-08-31): `endelea` kama taarifa ya MOJA KWA MOJA
ndani ya mwili wa `kwa` (pamoja na taarifa ya mwisho) imefungwa
kwenye mnyororo wa uzalishaji. Zamani mnyororo ulikwama kwenye
`endelea` wa mwisho (kitanzi kisichoisha, msimbo wa kutoka 124 --
kosa lililoelezwa kwenye hati/uthibitisho-wa-lugha.md 4.2 namba
16), na hali iliyoachwa ilichukuliwa kama uongo (mwili haukutekelezwa
kamwe). Majaribio: jaribio_hitilafu_endelea_moja_kwa_moja,
jaribio_hitilafu_kwa_hali_tupu, jaribio_hitilafu_kwa_mabano_matupu,
jaribio_hitilafu_endelea_na_acha_mwisho.

### 6.6 Chagua (switch)

```
chagua (usemi) {
    hali 1: ... ;
    hali 2: ... ;
    sivyo: ... ;
}
```

Kilichopimwa (2026-08-27): mbegu inakataa `chagua` kabisa (kosa la
mchanganuzi -- hata kwa selecta ya n32 rahisi, "ulichanganuzi";
imepimwa tena 2026-09-04: bado inakataa). Mnyororo wa uzalishaji
unafanya kazi kwa hali za nambari halisi ndogo; lebo hasi (`hali
-3`), lebo za n64 kubwa (> 2^31) na lebo za usemi au kigezo
zinakubaliwa LAKINI hazilingani kamwe (lebo inakuwa 0 kwenye
kizazi) -- jibu baya la kimya; lebo maradufu: ya kwanza inashinda
bila kosa; `kama` + `sivyo` ndani ya mkono wa `chagua` na bloku
`{ }` ndani ya mkono zinakataliwa na uzalishaji.
Kilichopimwa (2026-08-31): mnyororo wa uzalishaji unalinganisha
lebo hasi na lebo za n64 kwa usahihi (kwa upana wa selecta); lebo
za usemi au kigezo zinakataliwa kwa sauti na mkaguzi ("lebo ya
hali lazima iwe halisi ya nambari"). Mbegu bado inakataa `chagua`
kabisa (kilichopimwa 2026-09-04).

Kilichopimwa (2026-08-31) -- UAMIZI WA KIMUUNDO: HAKUNA mwanguko
wa mkono hadi mkono (fall-through) kwenye `chagua`. Kila mkono wa
`hali` unamalizika peke yake kwa kuruka hadi mwisho wa `chagua`
(sawa na `acha` ya kimya ya C); mkono usio na taarifa yoyote
haufanyi chochote; mkono wa `sivyo` (chaguo-msingi) unatekelezwa
ikiwa hali ZOTE hazikulingana. `acha` ndani ya mkono wa `chagua`
inamaliza KITANZI cha ndani kabisa (wakati au kwa -- semantiki ya
6.7), si `chagua` chenyewe; `acha` ndani ya `chagua` isiyo ndani
ya kitanzi inakataliwa na mkaguzi kwa sauti. Sababu: vipimo vya
awali havikutaja mwanguko wowote; mwanguko wa C ni chanzo cha
makosa ya kawaida; na lugha hii haina njia ya kusimamisha mkono
moja kwa moja (`acha` ni ya vitanzi pekee) -- kukubali mwanguko
bila njia ya kuusimamisha kungeacha tabia isiyo na udhibiti.
Majaribio: jaribio_hitilafu_chagua_bila_mwanguko,
jaribio_hitilafu_chagua_acha_ndani_ya_kitanzi,
jaribio_hitilafu_kata_acha_ndani_ya_chagua_nje.

### 6.7 Vunja na endelea

- `acha;` -- toka nje ya mzunguko wa ndani (wakati, kwa, au
  fanya -- 6.8).
- `endelea;` -- ruka hadi mwisho wa mwili na uendelee; kwa
  `fanya`, "mwisho wa mwili" ni sharti (6.8).

### 6.8 Fanya (do-while)

```
fanya { mwili } wakati (sharti);
```

Kitanzi cha fanya hutekeleza mwili MARA MOJA kabla ya sharti
kujaribiwa, kisha kurudia mwili mradi sharti ni kweli (semantiki
ya C ya do-while). Mwili hukimbia angalau mara moja -- hata sharti
la uwongo tangu mwanzo haliuzuii mwendo wa kwanza. Muundo ni
mkali: mabano ya wima ya mwili, mabano ya sharti, na nukta-mkato
wa mwisho ni ya lazima (sawa na mbegu).

`endelea` ndani ya fanya inaruka hadi SHARTI: taarifa zilizobaki
za mwili zinarukwa kwenye mwendo huo, na sharti linajaribiwa upya
(semantiki ya C ya do-while -- tofauti na `kwa`, ambapo endelea
inaruka hadi HATUA, 6.5). `acha` inaondoka kwenye kitanzi mara
moja bila kujaribu sharti tena. acha na endelea ndani ya fanya
zinahesabiwa kama zilivyo ndani ya wakati kwa ukaguzi wa
"acha/endelea nje ya kitanzi" (6.7).

Majaribio: jaribio_fanya_hukimbia_mara_moja,
jaribio_fanya_hali_ya_uwongo, jaribio_fanya_endelea,
jaribio_fanya_acha, na jaribio_fanya_kama_jina_la_kigezo (KATA --
neno muhimu kama jina linakataliwa, 2.2). Kilichopimwa
(2026-09-04): fanya inafanya kazi kwenye minyororo YOTE MIWILI --
mbegu iliyogandishwa 2026-09-04 ina kitanzi cha fanya (mwili
hukimbia mara moja, `endelea` inaruka hadi sharti, `acha` inaondoka
mara moja, viota ndani ya `kwa` -- vyote sawa na mnyororo wa
uzalishaji). Kumbuka la zamani la "majaribio ni ya stage1 pekee
hadi mbegu igandishwe" halishikiki.

## 7. Miundo

```
muundo Nukta {
    n32 x;
    n32 y;
};
```

- Upatikanaji wa sehemu: `p.x` (kwa thamani) na `p->x` (kwa
  kielekezi).
- Miundo inaweza KUREJESHWA kwa thamani (sret) kwenye minyororo
  yote miwili, na sret inayopitishwa moja kwa moja kama hoja ya
  wito mwingine (`jumlisha(tengeneza(10))`) inafanya kazi kwenye
  minyororo yote miwili.
- KUPITISHWA kwa thamani kunafanya kazi kwenye minyororo yote
  miwili: kazi inayopokea muundo kwa thamani na kusoma sehemu zake
  ni sahihi kwa miundo ya baiti 8, 12, 16, 20, 24, 32, 48 na 64
  (kilichopimwa 2026-09-04) -- takataka ya zamani kwenye mbegu na
  kikomo cha zamani cha "baiti <= 8 pekee" kwenye uzalishaji
  havipo tena.
- Ugawi wa muundo `b = a;` unanakili muundo mzima kwenye minyororo
  yote miwili (kilichopimwa 2026-09-04) -- SEGV/takataka ya zamani
  haipo.
- Safu za miundo zinafanya kazi kwa mwendo wa kipengele (`s[2].x`
  ni sahihi kwa miundo ya baiti 12 na 20) kwenye minyororo yote
  miwili -- SEGV ya zamani haipo.
- Muundo wa ulimwengu (`Jozi G;`): mnyororo wa uzalishaji unafanya
  kazi; mbegu inakataa kwa sauti kwa kosa lisilo sahihi linalotaja
  d64 (kilichopimwa 2026-09-04).
- Hakuna urithi, hakuna miundo ya kijiuzi.

## 8. Utegemezi wa faili (husisha)

- `husisha { faili.swa }` -- tangazo la utegemezi. Kwenye mnyororo wa
  uzalishaji (stage1) ni KIUNGO HALISI (kilichopimwa 2026-09-04;
  kipengele cha mfumo wa moduli, tangu 2026-09-02): mkusanyaji
  anatatua maelekezo ya `husisha { }` mwenyewe, sawa na msuluhishi
  wa `gharama/msuluhishi.swa` -- kila faili linalotajwa linatafutwa
  kwa mpangilio (1) `msingi/`, (2) `msingi/maktaba/`, (3) saraka ya
  sasa; utegemezi hufuatwa kwa mpangilio wa topolojia (tegemezi
  kabla ya tegemeziwa); kila faili linajumuishwa mara moja tu
  (marudio yanaondolewa); mzunguko unakataliwa kwa sauti ("mzunguko
  wa husisha umegunduliwa: y.swa -> x.swa -> y.swa"); faili
  lisilopatikana linalia kwa sauti ("faili la husisha
  halipatikani"). Njia ni za jamaa kwa saraka ya kazi ya mkusanyaji.
- MBEGU haitatui: inaruka mstari wa `husisha { }` kimya (hakuna
  faili linalosomwa), kama zamani -- programu za mbegu bado zinahitaji
  kuunganishwa kwa mkono (cat). Wito wa kazi isiyofafanuliwa unalia
  kwa sauti kwenye minyororo yote miwili (`Hitilafu: kazi
  haijafafanuliwa: <jina>` -- mbegu pia, kilichopimwa 2026-09-04).
- `husisha C::stdio` -- kiungo cha C: mstari unatupwa na mnyororo wa
  uzalishaji (kwenye towe lake la kutatua) na unarukwa na mchanganuzi
  wa mbegu -- haufanyi chochote kwenye mnyororo huu: hakuna jina la C
  linaloandikwa kwenye kitu (kilichopimwa 2026-09-04 kwenye pato la
  .o), na wito wa kazi yoyote isiyofafanuliwa (hata baada ya C::)
  bado unakataliwa kwa sauti kama "kazi haijafafanuliwa".

### 8.1 Kazi za nje (daraja la mashine)

Kazi iliyokwisha kukusanywa kwenye maktaba ya mfumo inatangazwa kwa
kichwa cha Swa na kuitwa kama kazi nyingine yoyote. Muundo kamili na
awamu zake: `hati/kiolesura-cha-mashine.md`.

```
nje "libm.so.6" {
    d64 kosini(d64 pembe) = "cos";
    d64 mzizi_mraba(d64 x) = "sqrt";
}
```

- `nje "<maktaba>" { ... }` ni tangazo la ngazi ya juu. Mfuatano ni
  jina la maktaba kama mfumo unavyoliita. `nje` SI neno muhimu:
  linatambuliwa tu likifuatiwa na mfuatano kwenye ngazi ya juu, hivyo
  `n8* nje` kama kigeu kinaendelea kufanya kazi.
- Kila mstari ni kichwa cha kazi bila mwili, kikiishia kwa `;`.
  `= "<alama>"` huunganisha jina la Swa na jina lao kwenye maktaba;
  bila `=`, alama ni jina la Swa lenyewe. Majina ya Kiingereza ya
  maktaba huingia kama mfuatano, si kama kitambulisho.
- Aina ni za Swa: `int` = `n32`, `long`/`size_t` = `n64`/`a64`,
  `double`/`float` = `d64`/`d32`, kielekezi chochote = `T*` (au `n8*`).
- Saini inajulikana, kwa hiyo mkaguzi hukagua hoja na matokeo kama
  wito wa kawaida (tofauti na wito kupitia kielekezi, 4.3), na matokeo
  ya `d64` huja kwenye xmm0.
- Makubaliano ya wito huamuliwa na lengo: `--programu` kwenye Linux x86-64
  ni System V. Kila wito hupitia kipande kinachopangilia rsp kwa baiti
  16. `&kazi_ya_swa` inaweza kupitishwa kwa msimbo wa nje kama kazi ya
  kurudi (mf. `qsort`).
- Programu yenye `nje` huwa ELF ya kiungo-hai (PT_INTERP
  `/lib64/ld-linux-x86-64.so.2`, PT_DYNAMIC, DT_NEEDED kwa kila
  maktaba, GOT inayojazwa na kipakiaji). Swa huiandika yenyewe --
  hakuna kiungo. Programu isiyo na `nje` haibadiliki hata baiti moja.
- Maktaba au alama isiyopatikana hukataliwa na kipakiaji cha mfumo
  wakati wa kuanza (kutoka 127).
- Hoja za rafu zinaruhusiwa (mpaka ni hoja 16, kama wito wote wa Swa).
- `...` kama kigezo cha mwisho (ndani ya `nje` pekee) huruhusu hoja za
  ziada: `n32 andika_kwa_umbo(n8* b, n64 u, n8* umbo, ...) = "snprintf";`.
  Hoja ya ziada ya `d32` ni kosa -- tumia `d64`.
- Data ya maktaba: `n8* mkondo = "stdout";` ndani ya `nje` ni kigezo cha
  ulimwengu ambacho kipakiaji hukijaza kwa thamani ya maktaba
  (R_X86_64_COPY). Namba, kielekezi au muundo.
- Muundo kwa thamani (hoja, matokeo na data) hufuata uainishaji wa
  System V; kipande hutafsiri kati ya njia ya Swa na ile ya C. Muundo
  lazima uwe na mpangilio wa C (sehemu kwenye ofseti za kizidisho cha
  upangiliaji wake, ukubwa kizidisho cha upangiliaji mkubwa) -- Swa
  haiongezi kujaza kimya, hivyo mpangilio mbaya ni kosa linalotaja
  kujaza kunakohitajika.
- Mipaka iliyobaki, yote kwa sauti: muundo kwenye kazi yenye `...`,
  data ya desimali au safu, na hali zisizo `--programu`/`--wasm`
  (`--kiini`, `--tafsiri`, `--kati`, `--papo`, `.o`). Jina linalogongana na
  kazi ya Swa yenye mwili, au kutangazwa mara mbili, ni kosa.
- `--wasm`: kila `nje "<moduli>"` huwa uingizaji wa WASM (moduli,
  alama); hoja na matokeo ni `n32`, kielekezi au `w0`. DOM hutoka
  `msingi/maktaba/kivinjari.swa` (`husisha { kivinjari.swa }`), na
  `swa --wasm-js` huandika daraja la jumla la JS.
- Majaribio: `gharama/jaribu-kiolesura.mapishi` (`majaribio/kiolesura/`),
  `gharama/jaribu-wasm.mapishi` (`majaribio/wasm/`).

## 9. Kazi za Ndani (builtins)

- `wito_wa_mfumo(n64 namba, n64 a1, ..., n64 a6)` -- simu ya syscall
  ya Linux moja kwa moja. ABI: rax=namba, rdi, rsi, rdx, r10, r8, r9.
  INAFANYA KAZI kwenye minyororo yote miwili (kilichopimwa
  2026-09-04: syscall ya exit, na mmap yenye hoja 6 kamili --
  anwani iliyorudishwa inatumika kwa kuandika na kusoma). Kikomo:
  mkaguzi wa mnyororo wa uzalishaji anakataa wito wenye hoja zaidi
  ya 7 ("idadi ya hoja kwa 'wito_wa_mfumo' si sahihi (kikomo ni 7,
  imepokea 8)") -- mdudu wa zamani (hoja za ziada zilimezwa kimya)
  umefunga. Mbegu bado inakubali wito wa hoja 8+ na kusawazisha
  rafu (hoja ya 7 inachukuliwa na pop r9, hoja 8+ zinafutwa, hakuna
  uharibifu wa vigezo vya ndani -- kilichopimwa 2026-09-04) --
  inarekebishwa (mgandisho ujao wa mbegu).
- `tekeleza(n8* kazi, n32 argc, n8** argv, n32 ofseti)` na
  `anwani_ya_kazi(n8* jina)` -- visaidizi vya NDANI vya daraja la JIT
  la mkusanyaji wa kujikusanya (`--papo`): havipo kwenye minyororo
  yote miwili kama kazi za lugha. Kipimo cha 2026-09-04: `tekeleza`
  inatambuliwa na MZALISHAJI kwa jina (badala ya wito halisi) hata
  katika hali ya --programu -- wito wa mtumiaji unakusanywa na kuanguka
  SEGV wakati wa kukimbia (hoja ya kwanza inachukuliwa kama anwani
  ya msimbo). `anwani_ya_kazi` inakataliwa kwa sauti ("kazi
  haijafafanuliwa") kwenye minyororo yote miwili. Lugha ya programu
  hutumia `&jina_la_kazi` na wito kupitia kielekezi (4.3) -- hakuna
  builtin ya pekee inayohitajika.

## 10. Maktaba ya Kawaida (msingi/maktaba/)

| Faili | Kazi muhimu |
|---|---|
| `kumbukumbu.swa` | nakili, weka_sifuri, linganisha_kumbukumbu, tenga/badili/achilia_eneo_la_utengaji (eneo la utengaji la mmap), mfumo_soma/mfumo_andika/mfumo_fungua/mfumo_funga, andika, andika_kosa, soma_mstari |
| `mfuatano.swa` | urefu_wa_mfuatano, linganisha_mfuatano, nakili_mfuatano, unganisha_mfuatano, tafuta_herufi, tafuta_mfuatano, kata_nafasi, nambari_kwa_mfuatano, mfuatano_hadi_n32/n64 |
| `hesabu.swa` | hesabu_kamili/kubwa, hesabu_ndogo/dogo, neneo_n32/n64, kigawo_kikuu_cha_shirika, kipeo_kamili, kipeuo_kamili, fibonacci_hesabu |
| `orodha.swa` | Orodha (orodha_mpya huchukua uwezo wa awali kama hoja; UKUAJI unafanya kazi -- orodha_ongeza zaidi ya uwezo inaongeza mara mbili kwa badili; kilichopimwa 2026-09-04: kujaza 30 kwenye uwezo wa 10 kunafanya kazi kwenye minyororo yote miwili): orodha_mpya, orodha_ongeza, orodha_pata, orodha_futa_mwisho, urefu_wa_orodha, orodha_huru |
| `mpangilio.swa` | pangilia_n32, pangilia_n32_kushuka, pangilia_n64, pangilia_n64_kushuka |
| `ramani.swa` | Ramani ya funguo za n32 hadi thamani (inafanya kazi kwenye minyororo yote miwili -- weka+pata kwa funguo kamili; kilichopimwa 2026-09-04, jibu baya la zamani la mbegu halipo) |

Saini za `nakili` na `badili` (kumbukumbu.swa) -- kazi za kawaida
za maktaba, si maneno muhimu (2.2):
- `w0 nakili(n8* lengwa, n8* chanzo, n64 n)` -- inanakili baiti
  `n` kutoka `chanzo` hadi `lengwa` (kama memcpy ya C).
- `n8* badili(n8* p, n64 ukubwa)` -- inarudisha kipande kipya cha
  baiti `ukubwa` kutoka eneo la utengaji, kikiwa kimenakili kipande cha
  zamani cha `p` (kama realloc ya C juu ya eneo la utengaji la mmap):
  `p == 0` inafanya kama `tenga`; `ukubwa == 0` inarudisha `p`;
  eneo la utengaji halifungui vipande -- kipande cha zamani kinabaki.

Kila kazi imejitosheleza; maktaba inaweza kuunganishwa kwa mkono
(`cat`) kwa matumizi na mbegu.

## 11. Mipaka

Tazama `hati/mipaka.md` kwa orodha kamili yenye viwango vya ukali.
Muhtasari wa kilichopimwa 2026-09-04 kwenye minyororo yote miwili
(mbegu iliyogandishwa 2026-09-04 na mnyororo wa uzalishaji):
- ABI ya wito wa kazi za d64 ni ya xmm kwenye minyororo yote miwili;
  mpaka wa d64 na nambari kamili (ugawaji, hoja, kurudisha,
  mchanganyiko, ulinganisho) umefunga; d64 kama sharti moja kwa
  moja ni uongo daima; mbegu inakataa kigezo cha ulimwengu cha d64;
  d32 inafanya kazi kwenye uzalishaji, mbegu haijui d32 (2.3).
- Halisi zote hadi 2^64-1 zinatoa imm64 kamili (2^64 inazunguka
  kimya mod 2^64); upana usio wa 8/16/32/64 unakataliwa kwa sauti
  kwenye minyororo yote miwili; `husisha { faili }` ni kiungo
  halisi kwenye mnyororo wa uzalishaji, mbegu inaruka mstari (8).
- Uvunjaji UNAOENDELEA: miundo ya ulimwengu na kigezo cha ulimwengu
  cha d64 kwenye mbegu (kikomo cha mbegu); w0 ya ndani yenye
  kianzio, asilimia kwenye kielekezi, na wito_wa_mfumo wa hoja 8+
  zimefungwa kwenye mnyororo wa uzalishaji lakini mbegu bado
  inakubali kimya (inarekebishwa -- mgandisho ujao); na `5.`
  inaangusha mbegu (inarekebishwa).
- Upeo wa tokeni 262,144 unalia kwa sauti.

## 12. Uthibitisho

Kila kanuni katika hati hii ina mwenzake kwenye majaribio
yanayoendeshwa na CI (`gharama/*.mapishi`: mnyororo huru, programu
za `majaribio/programu/MANIFEST.txt` kwenye minyororo yote miwili,
mkazo, mifano, usakinishaji, IR, daraja la mashine, WASM, kernel)
au kwenye fixpoint ya kujikusanya (stage2-exe == stage3-exe sawa
kwa baiti).

Kumbuka kilichopimwa (2026-08-27): fixpoint inathibitisha
kujikusanya, si usahihi wa semantiki. Uthibitisho kamili wa tabia
ya lugha (kesi ~487 zilizokusanywa na kuendeshwa kwenye minyororo
yote miwili) umeandikwa kwenye `hati/uthibitisho-wa-lugha.md`.
Tangu kipimo hicho, kanuni kadhaa ambazo hazikushikika wakati huo
zimefungwa kwenye minyororo yote miwili (imepimwa upya 2026-09-04):
desimali kwenye mpaka wa nambari kamili (2.3), `!`/`~`/`^` kwenye
mbegu (2.5), upana usio wa 8/16/32/64 unaokataliwa kwa sauti (3),
hesabu ya kielekezi kwa vipengele, miundo kwa thamani/safu/ugawi,
fanya, vielekezi vya kazi (4.2/4.3/6.8/7), `husisha` kama kiungo
halisi kwenye mnyororo wa uzalishaji (8), na ukuaji wa Orodha na
Ramani kwenye mbegu (10) -- zote kilichopimwa 2026-09-04.
