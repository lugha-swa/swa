# Mipaka Inayojulikana ya Swa

Hati hii inaorodhesha mipaka inayojulikana ya mkusanyaji, kwa ukali.
"0% bootstrap gap" inahusu mnyororo wa UZALISHAJI (hakuna lugha nyingine
popote) — si uthabiti wa mchanganuzi dhidi ya ingizo baya. Uthabiti wa
makosa ni mhimili tofauti, bado wazi.

## 1. Mbegu: ingizo baya linakubaliwa au linasegfault [UKALI: JUU]

Mchanganuzi wa mbegu (msingi/mbegu.s) hauchunguzi ingizo kwa uthabiti:

- `@#!` — inakubaliwa kimya kimya (rc=0, ELF halali ya program tupu)
- `garba`, `kweli kweli kweli` — **SEGFAULT** (rc=139)
- `N32 main( {`, `N32 main() { rudisha 1; ` — chanzo lililokatwa
  linakubaliwa kimya kimya (rc=0)

Njia za makosa za mbegu (msg_parseerr/msg_lexerr, exit 1) hazifikiwi
kwa visa hivi. Hakuna ahadi ya kugundua makosa yote ya ulichanganuzi.

## 2. Mchanganuzi wa .swa: ingizo lililokatwa linaning'inia [UKALI: KATI]

`stage1 --exe` (mchanganuzi wa msomaji.swa/msambazaji.swa) unaning'inia
bila kikomo kwa ingizo kama `N32 main( {` (hakuna mwisho wa mabano).
Makosa YANAYOTAMBULIWA yanarudisha 1 kwa usahihi (`; KOSA: 1`), lakini
upotevu wa kurejesha kwa visa vya kukatwa haujashughulikiwa.

## 3. Upeo wa tokeni: 65,536 [UKALI: KATI]

Mbegu ina MAX_TOKENS 65,536. Chanzo kikubwa kuliko hicho kinakatwa
**kimya kimya** — hakuna ujumbe wa kosa, mkusanyaji unarudisha 0 na
program iliyokatwa (ambayo inakosea wakati wa utekelezaji). Kwa mfano,
wito ~7,280 wa kazi kwa usemi rahisi ndio kikomo halisi cha chanzo
kimoja. Upeo wa AST ni 65,536 sawa.

## 4. Jedwali la nje: ingizo moja kwa kila wito, bila dedup [UKALI: CHINI]

Kila wito wa mbele unachukua ingizo jipya la nje (MAX_EXTERNS 16,384)
na ingizo la RELA (MAX_RELOCS 16,384). Hakuna uondoaji wa marudio —
hii ni sahihi kiutendaji (imejaribiwa hadi wito 7,000), lakini:

- Upeo wa nje haufikiki kwa sasa kwa sababu upeo wa tokeni unapiga
  kwanza.
- Ikiwa upeo wa tokeni utainuliwa: `.extern_full` inarudia faharisi 0
  KIMYA (utatuzi mbaya) na `.skip_reloc` inaacha RELA KIMYA (wito kwa
  anwani isiyo sahihi) — si makosa, ni uharibifu wa kimya.

## 5. Maneno halisi ni 32-bit signed [UKALI: CHINI]

Neno halisi `2147483648` linatafsiriwa kama `-2147483648` (biti
zinahifadhiwa, ishara inaenea) — na mkusanyaji wa mbegu NA dereva wa
Rust KWA USAWA (uthabiti, si mgawanyiko). Thamani kubwa zaidi ya
32-bit lazima zijengwe wakati wa utekelezaji.

## 6. Dereva wa Rust/LLVM: FastISel inaacha vizuizi zaidi ya ~50 [UKALI: KATI]

Kazi zenye vizuizi vingi (k.m. `ni_neno_muhimu` — 247) hukatwa na
FastISel ya LLVM: mkusanyaji wa LLVM hauwezi kujithibitisha (inatoa
.o bila alama). Uthibitisho wa nje wa mnyororo (GNU ld dhidi ya toa_exe
ya mbegu) unafunika njia ya mbegu/exe/RELA pekee — SI mwisho wa LLVM.

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
