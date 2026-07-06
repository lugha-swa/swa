# Masuala Yanayobaki — Mkusanyaji wa Kujikusanya wa Kiswahili

Hati hii inafuatilia hitilafu zinazojulikana, vizuizi, na hatua zinazofuata kwa mradi wa mkusanyaji wa kujikusanya wa Kiswahili. Vipengee vimeorodheshwa takriban kwa mpangilio wa ukali na utegemezi.

---

## 1. Hitilafu ya Uboreshaji wa O1 (Less) — Ufisadi wa Urefu wa `tokeni_soma_kitambulisho`

### Hali: **Imevunjika kwenye O1, inafanya kazi kwenye O0**

### Muhtasari

Kwenye O1, SelectionDAG ya LLVM inakusanya vibaya utoaji katika usemi ufuatao kutoka `tokeni_soma_kitambulisho` (ndani ya `msomaji.swa`):

```
t->urefu = m->nafasi - anza;
```

Utoaji wa `anza` unaonekana kupotea kabisa, hivyo `urefu` inapokea thamani ghafi ya `m->nafasi` (nambari kubwa kamili) badala ya urefu sahihi `m->nafasi - anza`. Hii inasababisha uandishi unaofuata kwenye `ast_pool` kutua mbali kupita mipaka ya safu, na kuangusha mchanganuzi.

### Ushahidi (gdb)

Katika nukta ya kuanguka:

| Rejesta | Thamani | Maana |
|----------|-------|---------|
| `rdx`    | 36797 | Anwani ya kianzio cha kuanguka (inatarajiwa ndogo) |
| `off`    | 3     | Kianzio sahihi cha nodi ya AST ndani ya elementi |
| `i`      | 36794 | Mbaya — inapaswa kuwa 0 kwa kitambulisho cha tokeni moja |

Thamani 36794 = 36797 - 3, ikimaanisha faharisi ya elementi `i` ilikokotolewa kutoka kwa `urefu` iliyoharibika badala ya thamani sahihi ya 1.

### Sababu Inayowezekana

SelectionDAG (inayotumika O1 na juu) inaweza:
- Kushindwa kutoa elekezo la `sub` kwa usemi wa tofauti ya kielekezi, au
- Kupanga upya mizigo/hifadhi kiasi kwamba `anza` haijakokotolewa wakati utoaji unatathminiwa.

Hitilafu ni maalum kwa LLVM IR inayozalishwa kwa `tokeni_soma_kitambulisho`. Kwenye O0, FastISel inashughulikia kazi hii kwa usahihi; kwenye O1, SelectionDAG inachukua na kuzalisha msimbo mbaya.

### Kwa Nini O1 Ni Muhimu

FastISel inadondosha vitalu vya msingi kimya kupita takriban 50 kwa kila kazi. Msomaji na mchanganuzi wa kujikusanya tayari wamegawanywa katika kazi nyingi ndogo za wasaidizi ili kukaa chini ya kikomo hiki. Ikiwa kazi yoyote iliyobaki itavuka kizingiti, O1 ndio mbadala pekee — na O1 kwa sasa imevunjika.

### Kinachohitajika Kufanywa

1. Tenga `.ll` iliyozalishwa kwa `tokeni_soma_kitambulisho` na uthibitishe IR ni sahihi (`sub` ipo kabla ya njia za uboreshaji).
2. Ikiwa IR ni sahihi, gawanya njia ipi ya LLVM inaharibu thamani (pengine njia ya awali ya kuteremsha ya SelectionDAG).
3. Ikiwa IR ni mbaya, rekebisha codegen kwa usemi wa tofauti ya kielekezi katika mwisho wa LLVM wa mkusanyaji wa Kiswahili.
4. Fikiria ikiwa hii ni hitilafu inayojulikana ya LLVM na toleo maalum — ikiwa ndivyo, kuboresha au kurekebisha LLVM kunaweza kurekebisha.

---

## 2. Kizuizi cha Ukubwa wa Safu — BSS > ~47KB Inaanguka kwenye Uanzishaji

### Hali: **Haijatatuliwa — pengine maalum kwa Windows**

### Muhtasari

Wakati safu za bwawa la AST ni ndogo (elementi 512, ~32 KB `ast_pool`), binary ya mchanganuzi inafanya kazi kwa usahihi kwenye O0. Kuongeza safu (elementi 2048, bwawa la ~128 KB) kunasababisha hitilafu ya sehemu mara moja **kabla ya `main()` kutekelezwa** — hata kwa msimbo wa chanzo unaofanana na hakuna mabadiliko ya mantiki.

### Tabia Iliyozingatiwa

- Safu za elementi 512: inafanya kazi kwenye O0.
- Safu za elementi 2048: hitilafu ya sehemu kabla ya `main()`.
- Binary iliyokuwa ikifanya kazi hapo awali na safu kubwa baadaye iliacha kufanya kazi, ikipendekeza suala la mazingira badala ya kiwango cha msimbo.
- Kuanguka ni katika uanzishaji wa CRT au kipakiaji cha PE, si katika msimbo wa mtumiaji.

### Nadharia

- **Windows ASLR / kipakiaji cha PE**: Sehemu kubwa za BSS zinaweza kuchochea tabia tofauti ya kipakiaji au ushughulikiaji wa uhamishaji.
- **Uanzishaji wa sifuri wa CRT (`__security_init_cookie` au `memset` ya BSS)**: CRT inaweza kutembea BSS tofauti kwa sehemu kubwa, na kugonga mpaka wa ukurasa au ukurasa wa ulinzi.
- **Uchunguzi wa rafu / ukurasa wa ulinzi**: Windows inaweza kugusa kurasa za BSS wakati wa uanzishaji na kutengeneza hitilafu kwenye ukurasa wa ulinzi karibu na BSS.
- **Hati ya kiunganishi au mpangilio wa sehemu ya PE**: Kiunganishi kinaweza kuweka BSS katika eneo lisilotarajiwa inapozidi ukubwa fulani.

### Kilichohitajika Kufanywa

1. **Ilijaribiwa kwenye Arch Linux** — Linux ELF inashughulikia BSS kubwa bila tatizo. Hii imethibitishwa kuwa maalum kwa Windows.
2. Kwa Windows, chunguza kichwa cha PE na uwekaji wa sehemu ya `.bss`, au fikiria kutumia `calloc`/`malloc` kwa safu kubwa badala ya mgao wa BSS tuli/wa ulimwengu.

---

## 3. Kesi za Pembeni za Mchanganuzi wa Kujikusanya

### Hali: **Inafanya kazi, lakini urejeshaji wa makosa haujakamilika**

### Kinachofanya kazi

Mchanganuzi wa kujikusanya sasa unachanganua kwa mafanikio:
- Faili za chanzo rahisi: `N32 f() { rudisha 1; }`
- Faili zote za maktaba ya msingi: `msomaji.swa`, `msambazaji.swa`, `mteremko.swa`, `mkaguzi.swa`, `kumbukumbu.swa`, `mfuatano.swa`
- Faili nyingi zilizounganishwa (AST_SAFU = 16384)
- K6 (kujikusanya kamili) inapita

### Kisichofanya kazi

- Urejeshaji wa makosa: mchanganuzi unaweza kuanguka au kuingia kitanzi kisicho na mwisho kwenye pembejeo lililoharibika.
- Uboreshaji wa O1 (SelectionDAG) una tatizo la tofauti ya kielekezi.

### Kinachohitajika Kufanywa

1. Ongeza urejeshaji wa msingi wa makosa ili mchanganuzi aweze kunusurika makosa ya sintaksia bila kuanguka.
2. Endesha mchanganuzi kwenye faili zaidi za majaribio za `.swa` kuthibitisha uthabiti.

---

## 4. Mgawanyo wa Kazi kwa O0 — Kikomo cha Block cha FastISel

### Hali: **Suluhisho la muda lipo, udhaifu unabaki**

### Historia

Kwenye O0, FastISel ya LLVM inadondosha vitalu vya msingi kimya kupita takriban 50 kwa kila kazi. Hiki sio kikomo kinachoweza kusanidiwa — ni mbadala uliokodishwa ngumu ambapo FastISel inakata tamaa na kutoa hakuna msimbo kwa vitalu hivyo, na kusababisha tabia mbaya bila onyo.

### Mikakati ya Sasa

- **Msomaji** (`msomaji.swa`): Kazi ndefu ziligawanywa kwa mikono katika wasaidizi.
- **Mchanganuzi** (`msambazaji.swa`): Uligawanywa kiotomatiki kupitia `_finish.py` katika kazi nyingi kukaa chini ya kikomo cha block.

### Hatari Iliyobaki

Ikiwa kazi yoyote — baada ya marekebisho ya baadaye au vipengele vipya — itazidi ~vitalu 50 vya msingi, FastISel itazalisha msimbo mbaya kimya kwenye O0. Hakuna ukaguzi wa wakati wa mkusanyiko au wakati wa utekelezaji kwa hali hii.

### Kinachohitajika Kufanywa

1. Ongeza dai au ukaguzi wa baada ya codegen unaothibitisha hakuna vitalu vilivyodondoshwa na FastISel.
2. Vinginevyo, hama hadi O1 kwa kazi zote mara tu hitilafu ya O1 (sehemu ya 1) itakaporekebishwa, na kuondoa kikomo cha FastISel kabisa.
3. Ikiwa unabaki kwenye O0, andika kikwazo cha hesabu ya block kwa uwazi katika mwongozo wa mchangiaji.

---

## 5. Hatua Zinazofuata (Mpangilio wa Kipaumbele — Imesasishwa Julai 6, 2026)

| Kipaumbele | Kazi | Hali |
|----------|------|--------|
| K1 | Hitilafu ya O1 kwenye `tokeni_soma_kitambulisho` | Bado wazi. SelectionDAG inaharibu tofauti ya kielekezi. |
| K2 | Kamilisha `mteremko.swa` (kiteremshi cha kujikusanya) | Inaendelea. Sret, alloca-in-loop, uzalishaji wa `.o` |
| K3 | Kamilisha `mkaguzi.swa` (mkaguzi wa kisemantiki) | Inaendelea. Uthibitishaji wa aina, hoja, matawi |
| K4 | Ongeza urejeshaji wa makosa kwa mchanganuzi | Bado wazi. Mchanganuzi haushughulikii sintaksia mbaya vizuri. |
| K5 | Pipeline ya uboreshaji (`--opt` flag) | Bado wazi. LLVM pass manager kwa mem2reg, instcombine, GVN, DCE |
| K6 | Maktaba ya kawaida kamili | Inaendelea. `orodha.swa`, `mfuatano.swa`, `ramani.swa` |
| K7 | Malengo zaidi: ARM, AArch64, RISC-V | Bado wazi. Lengo la muda mrefu. |
