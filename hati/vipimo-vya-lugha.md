# Vipimo Rasmi vya Lugha ya Swa

Hati hii ni marejeo rasmi ya lugha ya Swa. Kila kanuni hapa
imejaribiwa kwenye mnyororo wa mkusanyaji (mbegu na mnyororo wa
kujikusanya wa .swa) au imeandikwa wazi kama kikomo (tazama
`hati/mipaka.md`). Toleo hili linalenga Swa 1.0.

## 1. Muundo wa Kimsingi

Programu ya Swa ni mfuatano wa tangazo la kiwango cha juu:

- Tangazo la kazi (`N32 jumlisha(N32 a, N32 b) { ... }`)
- Tangazo la muundo (`muundo Nukta { N32 x; N32 y; };`)
- Tangazo la kigezo cha ulimwengu (`N32 KIKOMO = 0;` na safu za
  ulimwengu `N8 bafa[1024];`)
- Kiungo cha kuingiza (`husisha { faili.swa }`) au kiungo cha C
  (`husisha C::stdio`)

Sehemu ya kuingia ni kazi `main`. Katika hali ya `--exe`, sahihi ni
`N32 main()` au `N32 main(N32 argc, N8** argv)`.

Kila faili huchanganuliwa kwa mpangilio wa juu-chini; wito wa mbele
unaruhusiwa (kazi inaweza kuitwa kabla ya kutangazwa).

## 2. Leksia

### 2.1 Vitambulisho

Vitambulisho huanza na herufi (a-z, A-Z) au `_`, na vinaendelea kwa
herufi, tarakimu, au `_`. Majina yote ya lugha, maktaba, na maoni ni
Kiswahili kwa mkongwe.

### 2.2 Maneno Muhimu

`muundo`, `rudisha`, `kama`, `sivyo`, `kamasivyo`, `wakati`, `kwa`,
`vunja`, `endelea`, `chagua`, `hali`, `husisha`, `ukubwa`, `tengeneza`.

Maneno muhimu hayawezi kutumika kama majina ya vitambulisho.

### 2.3 Halisi

- Nambari kamili: mfuatano wa tarakimu. Aina yake ya chaguo-msingi ni
  N32 ikiwa inatoshea ndani ya 32-bit signed; nje ya hapo ni N64.
  Kikomo: halisi ya 2147483648 inageuka -2147483648 (angalia
  `hati/mipaka.md` sehemu ya 5).
- Mfuatano: `"habari"` — baiti za N8 zikifuatiwa na 0. Utorokaji
  (`\n`, `\t`, `\\`, `\"`) unasaidiwa.
- Desimali: `21.5` — D64. **Kikomo:** bomba la desimali bado
  limevunjika kwenye minyororo yote (`hati/mipaka.md` 4c — ukali
  JUU); D64 haijafanyiwa kazi kama sehemu ya 1.0.

### 2.4 Maoni

`//` hadi mwisho wa mstari, na `/* ... */` kwa vizuizi. Maoni ya
kijiuzi hayaruhusiwi.

### 2.5 Ishara

`+ - * / % << >> < > <= >= == != && || & | ^ ~ ! = ? : ( ) { } [ ]
-> * & , ;`

## 3. Aina

| Aina | Maelezo |
|---|---|
| `N8` | Nambari kamili isiyo na ishara, baiti 1 |
| `N16` | Nambari kamili isiyo na ishara, baiti 2 |
| `N32` | Nambari kamili yenye ishara, baiti 4 |
| `N64` | Nambari kamili yenye ishara, baiti 8 |
| `W0` | Bila thamani (void) — kwa kazi tu |
| `D64` | Desimali, baiti 8 — **kikomo, angalia 2.3** |
| `B1` | Boolean — ya ndani; matokeo ya ulinganisho na mantiki |
| `T*` | Kielekezi kwa aina T |
| `T[n]` | Safu ya vitu n vya aina T |
| `muundo` | Muundo uliotangazwa na mtumiaji |

Matokeo ya `==`, `!=`, `<`, `>`, `<=`, `>=`, `&&`, `||`, `!` ni
thamani ya 1 (kweli) au 0 (si kweli).

## 4. Usemi na Utangulizi

Utangulizi wa ishara (kutoka juu hadi chini):

| Kina | Ishara |
|---|---|
| 6 | `*` `/` `%` |
| 5 | `+` `-` |
| 4 | `<<` `>>` |
| 3 | `<` `>` `<=` `>=` |
| 2 | `==` `!=` `=` (ugawi) |
| 1 | `&&` `||` `&` `|` `^` |

Mabano hubadilisha utangulizi. Chaguo la ternary `sharti ? kweli :
uwongo` linasaidiwa.

### 4.1 Mantiki ya fupi-hali (short-circuit)

`&&` na `||` zinatathmini kwa fupi-hali KATIKA MBEGU NA MNYORORO WA
.SWA:

- `a && b`: `b` haitathminiwi ikiwa `a` ni 0. Matokeo: 1 ikiwa zote
  mbili si 0, sivyo 0.
- `a || b`: `b` haitathminiwi ikiwa `a` si 0. Matokeo: 1 ikiwa
  yoyote si 0, sivyo 0.

Hii inaruhusu `j >= 0 && a[j] == x` bila kusoma nje ya mipaka.
Rekebisho la 2026-08: mbegu ilikuwa inatathmini pande zote mbili
(kinyume na uzalishaji.swa) — imewianishwa; majaribio ya kurejesha
ni `jaribio_mbegu_mzunguko_mfupi`.

### 4.2 Hesabu ya kielekezi

- `&x` — anwani ya kigezo au sehemu ya muundo.
- `*p` — nyoosha: thamani iliyoko kwenye anwani p.
- `p->sehemu` — sehemu ya muundo kupitia kielekezi.
- `a[i]` — safu au kielekezi: `*(a + i * ukubwa_wa_kipengele)`.
- Usemi wa `safu` pekee hutathminiwa kama kielekezi kwa kipengele
  chake cha kwanza.

## 5. Tangazo la Kazi

```
<aina> <jina>(<vigezo>) { <mwili> }
```

- Vigezo: `N32 a`, `N32* p`, `Orodha* o` (muundo kwa kielekezi au kwa
  thamani).
- `rudisha <usemi>;` kwa kazi yenye thamani; `rudisha;` kwa W0.
- Wito wa kujirudia na wito wa mbele unasaidiwa.

## 6. Taarifa

### 6.1 Tangazo la kigezo

`N32 x = 5;` — ndani ya block. Kianzilishi ni cha hiari
(`N32 x;` inaruhusiwa) lakini thamani ya kigezo kisichoanzishwa ni
ISYOFANULIWA (kumbukumbu ya rafu isiyoanzishwa) — usiisome kabla ya
kuigawa.

### 6.2 Ugawi

`x = usemi;` — ugawi ni usemi (matokeo yake ni thamani iliyogawiwa).

### 6.3 Kama/sivyo/kamasivyo

```
kama (sharti) { ... }
sivyo { ... }              // hiari
kamasivyo (sharti) { ... } // hiari, mnyororo wa sivyo-kama
```

### 6.4 Wakati

```
wakati (sharti) { ... }
```

### 6.5 Kwa (for)

```
kwa (kianzilishi; sharti; hatua) { ... }
```

Sehemu zote tatu ni za hiari. **Semantiki ya `endelea`:** inaruka
kwenye HATUA (ya tatu), si kwenye sharti — semantiki ya C. Hii
inatekelezwa kwenye mbegu; mkusanyaji wa .swa bado una semantiki ya
zamani (inaruka kwenye sharti) — kikomo kilichoandikwa
(`hati/mipaka.md` 4b, inafuatiliwa).

### 6.6 Chagua (switch)

```
chagua (usemi) {
    hali 1: ... ;
    hali 2: ... ;
    sivyo: ... ;
}
```

### 6.7 Vunja na endelea

- `vunja;` — toka nje ya mzunguko wa ndani (wakati au kwa).
- `endelea;` — ruka hadi mwisho wa mwili na uendelee.

## 7. Miundo

```
muundo Nukta {
    N32 x;
    N32 y;
};
```

- Upatikanaji wa sehemu: `p.x` (kwa thamani) na `p->x` (kwa
  kielekezi).
- Miundo inaweza kupitishwa kwa thamani na kurejeshwa kwa thamani.
- Hakuna urithi, hakuna miundo ya kijiuzi.

## 8. Kuingiza (husisha)

- `husisha { faili.swa }` — kiungo cha ndani: mkusanyaji wa .swa
  hulichakata faili lililotajwa (njia ya jamaa kwa faili la sasa).
  **Kikomo cha mbegu:** mbegu HAILICHAMBUZI faili la kiungo —
  faili lazima ziunganishwe kwanza (mf. `cat msingi/*.swa`), kama
  jinsi mkusanyaji wa .swa unavyojijenga. Kuanzia rekebisho la
  2026-08, wito wa kazi isiyofafanuliwa kwenye hali ya exe UNALIA
  kwa sauti (`Hitilafu: kazi haijafafanuliwa: <jina>`) badala ya
  kuvunja mchakato kimya kimya (SEGV) — jaribio la kurejesha:
  `jaribio_mbegu_kazi_kukosa`.
- `husisha C::stdio` — kiungo cha C: hakiathiri mchanganuzi; jina la
  kumbukumbu la C limeandikwa kwenye kitu kilichotolewa.

## 9. Kazi za Ndani (builtins)

- `wito_wa_mfumo(N64 namba, N64 a1, ..., N64 a6)` — simu ya syscall
  ya Linux moja kwa moja. ABI: rax=namba, rdi, rsi, rdx, r10, r8, r9.
- `tekeleza(N8* kazi, N32 argc, N8** argv, N32 ofseti)` — tekeleza
  msimbo wa JIT: r11=kazi, rdi=argc, rsi=argv+ofseti*8, al=0, call
  r11.
- `anwani_ya_kazi(N8* jina)` — anwani ya kazi ya ndani (JIT).

## 10. Maktaba ya Kawaida (msingi/)

| Faili | Kazi muhimu |
|---|---|
| `kumbukumbu.swa` | nakili, weka_sifuri, linganisha_kumbukumbu, tengeneza/achilia (arena), sys_soma/sys_andika/sys_fungua/sys_funga, andika, soma_mstari |
| `mfuatano.swa` | urefu_wa_mfuatano, linganisha_mfuatano, nakili_mfuatano, unganisha_mfuatano, tafuta_herufi, tafuta_mfuatano, kata_nafasi, nambari_kwa_mfuatano, mfuatano_hadi_n32/n64 |
| `hesabu.swa` | hesabu_kamili/kubwa, hesabu_ndogo/dogo, neneo_n32/n64, gcd_hesabu, pow_kamili, isqrt_hesabu, fibonacci_hesabu |
| `orodha.swa` | Orodha (safu inayokua): orodha_mpya, orodha_ongeza, orodha_pata, orodha_futa_mwisho, orodha_urefu, orodha_huru |
| `mpangilio.swa` | pangilia_n32, pangilia_n32_kushuka, pangilia_n64, pangilia_n64_kushuka |
| `ramani.swa` | Ramani ya jina → thamani |

Kila kazi imejitosheleza; maktaba inaweza kuunganishwa kwa mkono
(`cat`) kwa matumizi na mbegu.

## 11. Mipaka ya 1.0

Tazama `hati/mipaka.md` kwa orodha kamili yenye viwango vya ukali.
Muhtasari: desimali (JUU — bomba limevunjika kwenye minyororo yote),
mwisho wa LLVM wa dereva wa Rust (JUU — ulioshushwa hadhi kuwa wa
MAJARIBIO; mnyororo wa uzalishaji ni mbegu/exe pekee), upeo wa tokeni
65,536 (JUU — unalia kwa sauti), na semantiki ya zamani ya `endelea`
kwenye mkusanyaji wa .swa (inajulikana, inafuatiliwa).

## 12. Uthibitisho

Kila kanuni katika hati hii ina mwenzake kwenye majaribio ya
ujumuishaji (`majaribio/integration.rs`) au kwenye mnyororo wa
kujikusanya (fixpoint: stage2-exe == stage3-exe sawa kwa baiti).
Majaribio yote: 146 ya maktaba + 72 ya ujumuishaji + 1 ya nyaraka.
