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

### Hali: **Inafanya kazi kwa sehemu**

### Kinachofanya kazi

```
N32 f() { rudisha 1; }
```

Kwenye O0, mchanganuzi unachanganua pembejeo hili kwa mafanikio na kurudisha `mzizi=3` (ikionyesha mzizi halali wa AST wenye nodi tatu).

### Kisichofanya kazi

Baada ya kuchanganua pembejeo hapo juu, mchanganuzi unachapisha:

```
unexpected token on line 1
```

Hili ni suala la tokeni iliyobaki: mchanganuzi hautumii mabano ya kufunga `}` (au tokeni nyingine ya mwisho), hivyo kitanzi cha kiendeshi kinapata tokeni iliyopotea baada ya uchanganuzi kukamilika kimantiki.

### Kisichojaribiwa

- Chanzo cha `.swa` cha faili nyingi (faili za chanzo za mkusanyaji wenyewe) kutokana na kikomo cha ukubwa wa safu.
- Kazi zenye vigezo, ufikiaji wa sehemu za muundo, mtiririko wa udhibiti (`kama`, `wakati`), au upeo uliowekwa ndani.
- Urejeshaji wa makosa: mchanganuzi unaweza kuanguka au kuingia kitanzi kisicho na mwisho kwenye pembejeo lililoharibika.

### Kinachohitajika Kufanywa

1. Rekebisha utumiaji wa mabano ya kufunga (au tokeni yoyote iliyobaki).
2. Baada ya suala la ukubwa wa safu kutatuliwa, endesha mchanganuzi kwenye faili halisi za chanzo za `.swa` na urekebishe kushindwa kwa uchanganuzi.
3. Ongeza urejeshaji wa msingi wa makosa ili mchanganuzi aweze kunusurika makosa ya sintaksia bila kuanguka.

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

## 5. Hatua Zinazofuata (Mpangilio wa Kipaumbele — Imesasishwa Juni 2026)

| Kipaumbele | Kazi | Hali |
|----------|------|--------|
| K0 | Jaribu kwenye Arch Linux na safu kubwa | Imekamilika — Linux ELF inashughulikia BSS kubwa bila tatizo. |
| K1 | Rekebisha hitilafu ya ufisadi wa `urefu` ya O1 | Bado wazi. Marekebisho yetu ya opaque-pointer yanaweza kuwa yametatua — inahitaji kujaribiwa tena. |
| K1b | Mchanganuzi wa kujikusanya unakwama kwenye vigezo 2+ | Ugunduzi mpya kwenye Linux. Pengine kikomo cha block cha O0 FastISel. |
| K2 | Jaribu kujikusanya na faili halisi za `.swa` | Imefanyika kwa sehemu. Imezuiliwa na K1b. |
| K3 | Tatua kesi ya pembeni ya tokeni iliyobaki | Bado wazi. |
| K4 | Ongeza utambuzi wa kudondosha kwa block ya FastISel | Bado wazi. |
| K5 | Rekebisha ugawaji wa muundo katika kuteremshaji | Bado wazi. |
| K6 | Teremsha ya vigezo vingi kwa kujikusanya kamili | Inategemea K1b. |
