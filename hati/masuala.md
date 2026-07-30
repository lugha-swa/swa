# Masuala Yanayobaki -- Mkusanyaji wa Kujikusanya wa Kiswahili

Hati hii inafuatilia hitilafu zinazojulikana, vizuizi, na hatua zinazofuata kwa mradi wa mkusanyaji wa kujikusanya wa Kiswahili. Vipengee vimeorodheshwa takriban kwa mpangilio wa ukali na utegemezi.

---

## 1. Hitilafu ya Uboreshaji wa O1 (Less) -- Ufisadi wa Urefu wa `tokeni_soma_kitambulisho`

### Hali: **IMEREKEBISHWA (kwa LLVM 22.1.8)**

### Muhtasari

Kwenye O1 na LLVM 18.1, SelectionDAG ya LLVM ilikusanya vibaya utoaji katika usemi ufuatao kutoka `tokeni_soma_kitambulisho` (ndani ya `msomaji.swa`):

```
t->urefu = m->nafasi - anza;
```

Utoaji wa `anza` ulipotea kabisa, hivyo `urefu` ilipokea thamani ghafi ya `m->nafasi` (nambari kubwa kamili) badala ya urefu sahihi `m->nafasi - anza`. Hii ilisababisha uandishi unaofuata kwenye `ast_pool` kutua mbali kupita mipaka ya safu, na kuangusha mchanganuzi.

### Ushahidi (gdb, LLVM 18.1)

Katika nukta ya kuanguka:

| Rejesta | Thamani | Maana |
|----------|-------|---------|
| `rdx`    | 36797 | Anwani ya kianzio cha kuanguka (inatarajiwa ndogo) |
| `off`    | 3     | Kianzio sahihi cha nodi ya AST ndani ya elementi |
| `i`      | 36794 | Mbaya -- inapaswa kuwa 0 kwa kitambulisho cha tokeni moja |

Thamani 36794 = 36797 - 3, ikimaanisha faharisi ya elementi `i` ilikokotolewa kutoka kwa `urefu` iliyoharibika badala ya thamani sahihi ya 1.

### Uchunguzi na Urekebishaji (Julai 28, 2026)

Uchunguzi kamili uliofanywa:

1. **Uthibitishaji wa IR ya Swa**: Upunguzaji wa `AST_TOFAUTI` hadi `Instruction::Sub` katika `src/ir/lower.rs` ni sahihi. Hakuna hitilafu katika kizazi cha IR cha mkusanyaji wa Kiswahili.

2. **Uthibitishaji wa kupunguza LLVM**: `Instruction::Sub` inapunguzwa hadi `LLVMBuildSub` na `coerce_int_binop` sahihi katika `src/codegen/llvm/mod.rs`. Vivyo hivyo hakuna hitilafu.

3. **Jaribio la O1 kwenye LLVM 22.1.8**: Jaribio la `test_o1_sub_preserved` linaunda ruwaza halisi ya `GEP+pakia+utoa` inayotumika kwenye `msomaji.swa` na kukusanya kwenye O0, O1, na O2. Kwenye LLVM 22.1.8, amri ya `sub` ipo katika msimbo wa mkutano kwenye viwango vyote -- hitilafu HAITOKEI.

**Hitimisho**: Hitilafu ilikuwa suala la juu la LLVM lililokuwepo kwenye LLVM 18.1. Ilirekebishwa na toleo la LLVM lililoko kati ya 18.1 na 22.1.8. Hakuna mabadiliko yanayohitajika kwenye msimbo wa mkusanyaji wa Kiswahili.

### Rekebisho la Awali (LLVM 18.1)

Kwenye LLVM 18.1, bendera ya `--opt` ilianzishwa kama suluhisho mbadala: inaendesha njia za uboreshaji wa IR (mem2reg, instcombine, GVN, simplifycfg) lakini inatumia FastISel kwa kizazi cha msimbo, ikiepuka SelectionDAG kabisa. Suluhisho hili mbadala halihitajiki tena kwa hitilafu hii mahususi kwenye LLVM 22.1.8, lakini bado inatoa thamani kwa uboreshaji wa kiwango cha IR.

### Hatua Zilizochukuliwa

Hatua zilizotekelezwa:
1. Kutenga `.ll` iliyozalishwa kwa `tokeni_soma_kitambulisho` na kuthibitisha IR ni sahihi
2. Kulinganisha msimbo wa O0 na O1 kwenye LLVM 22.1.8 -- zote zinatoa `sub` kwa usahihi
3. Kuongeza jaribio la regression (`test_o1_sub_preserved`) kuzuia kurudi nyuma

---

## 2. Kizuizi cha Ukubwa wa Safu -- BSS > ~47KB Inaanguka kwenye Uanzishaji

### Hali: **Haijatatuliwa -- pengine maalum kwa Windows**

### Muhtasari

Wakati safu za bwawa la AST ni ndogo (elementi 512, ~32 KB `ast_pool`), binary ya mchanganuzi inafanya kazi kwa usahihi kwenye O0. Kuongeza safu (elementi 2048, bwawa la ~128 KB) kunasababisha hitilafu ya sehemu mara moja **kabla ya `main()` kutekelezwa** -- hata kwa msimbo wa chanzo unaofanana na hakuna mabadiliko ya mantiki.

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

1. **Ilijaribiwa kwenye Arch Linux** -- Linux ELF inashughulikia BSS kubwa bila tatizo. Hii imethibitishwa kuwa maalum kwa Windows.
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

### Kinachohitajika Kufanywa

1. Ongeza urejeshaji wa msingi wa makosa ili mchanganuzi aweze kunusurika makosa ya sintaksia bila kuanguka.
2. Endesha mchanganuzi kwenye faili zaidi za majaribio za `.swa` kuthibitisha uthabiti.

---

## 4. Mgawanyo wa Kazi kwa O0 -- Kikomo cha Block cha FastISel

### Hali: **Suluhisho la muda lipo, udhaifu unabaki**

### Historia

Kwenye O0, FastISel ya LLVM inadondosha vitalu vya msingi kimya kupita takriban 50 kwa kila kazi. Hiki sio kikomo kinachoweza kusanidiwa -- ni mbadala uliokodishwa ngumu ambapo FastISel inakata tamaa na kutoa hakuna msimbo kwa vitalu hivyo, na kusababisha tabia mbaya bila onyo.

### Mikakati ya Sasa

- **Msomaji** (`msomaji.swa`): Kazi ndefu ziligawanywa kwa mikono katika wasaidizi.
- **Mchanganuzi** (`msambazaji.swa`): Uligawanywa kiotomatiki kupitia `_finish.py` katika kazi nyingi kukaa chini ya kikomo cha block.

### Hatari Iliyobaki

Ikiwa kazi yoyote -- baada ya marekebisho ya baadaye au vipengele vipya -- itazidi ~vitalu 50 vya msingi, FastISel itazalisha msimbo mbaya kimya kwenye O0. Hakuna ukaguzi wa wakati wa mkusanyiko au wakati wa utekelezaji kwa hali hii.

### Kinachohitajika Kufanywa

1. Ongeza dai au ukaguzi wa baada ya codegen unaothibitisha hakuna vitalu vilivyodondoshwa na FastISel.
2. Vinginevyo, hama hadi O1 kwa kazi zote mara tu hitilafu ya O1 (sehemu ya 1) itakaporekebishwa, na kuondoa kikomo cha FastISel kabisa.
3. Ikiwa unabaki kwenye O0, andika kikwazo cha hesabu ya block kwa uwazi katika mwongozo wa mchangiaji.

---

## 5. Hatua Zinazofuata (Mpangilio wa Kipaumbele -- Imesasishwa Julai 6, 2026)

| Kipaumbele | Kazi | Hali |
|----------|------|--------|
| K1 | Hitilafu ya O1 kwenye `tokeni_soma_kitambulisho` | Imerekebishwa (hitilafu ya juu ya LLVM, LLVM 22.1.8). Jaribio la regression lipo. |
| K2 | Kamilisha `mteremko.swa` (kiteremshi cha kujikusanya) | Inaendelea. Sret, alloca-in-loop, uzalishaji wa `.o` |
| K3 | Kamilisha `mkaguzi.swa` (mkaguzi wa kisemantiki) | Inaendelea. Uthibitishaji wa aina, hoja, matawi |
| K4 | Ongeza urejeshaji wa makosa kwa mchanganuzi | Bado wazi. Mchanganuzi haushughulikii sintaksia mbaya vizuri. |
| K5 | Pipeline ya uboreshaji (`--opt` flag) | Bado wazi. LLVM pass manager kwa mem2reg, instcombine, GVN, DCE |
| K6 | Maktaba ya kawaida kamili | Inaendelea. `orodha.swa`, `mfuatano.swa`, `ramani.swa` |
| K7 | Malengo zaidi: ARM, AArch64, RISC-V | Bado wazi. Lengo la muda mrefu. |
