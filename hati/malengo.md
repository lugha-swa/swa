# Malengo ya Mradi wa Swa

## Lengo Kuu

**Swa itakuwa lugha ya kwanza duniani yenye uhuru kamili (0% kujianzisha pengo).**
Hakuna Rust, hakuna LLVM, hakuna GNU as, hakuna lugha nyingine yoyote
iliyowahi kuhusika katika mfululizo wake. Binary mbichi inayojitengeneza
yenyewe kupitia baiti za mkono.

## Jinsi Tutakavyofika

### Hatua ya 1: Kufa kwa LLVM [IMEFANIKIWA]
`uzalishaji.swa` inatoa **binari mbichi (.o)** moja kwa moja.
Hakuna kusanyiko. Hakuna GNU as.
- [x] ELF utangulizi + machine msimbo (opkodi za x86-64)
- [x] Inachukua nafasi ya mteremko.swa (LLVM) KABISA katika mfululizo wa kujikusanya
- [x] `N32 anza() { rudisha 42; }` -> ELF binari inayofanya kazi
- [x] Mfululizo mzima unajithibitisha: kamili2 == kamili3 (sawa kwa baiti)

### Hatua ya 2: JIT halisi (kama HolyC) [IMEFANYIKA]
Swa inatoa maelekezo moja kwa moja kwenye kumbukumbu na kuyatekeleza.
Hakuna faili. Hakuna binari. Hakuna OS kati.
- [x] `mmap()` -> andika opkodi -> rukia (PR #143)
- [x] Stub ya `ruka main` na tungo mwishoni mwa chombo
- [x] Thamani ya kurudi inarudi kwa usahihi (vipimo 5/5 vya JIT)
- [x] Wito wa kazi za ndani ndani ya msimbo wa JIT (jedwali la anwani
      la programu -- anwani_ya_kazi ya ndani + tekeleza kama kazi asili) na
      kupitisha argv; programu haina alama za nje kabisa

### Hatua ya 3: Kuziba pengo la kujianzisha [IMEFANYIKA KWA AWALI]
Andika baiti za mkono (opkodi za x86-64) zinazounda mkusanyaji wa kwanza.
- [x] Kwanza (msingi/kwanza.bin): baiti 393 zilizoandikwa kwa mkono
      (utangulizi wa ELF 64 + maelezo ya kipande 56 + msimbo 273) -- hubadilisha
      heksa hadi binari na inajijenga yenyewe (kwanza.heksa -> kwanza.bin)
- [x] awali.bin inazalika kutoka awali.heksa kupitia Kwanza -- NASM
      si sehemu ya mfululizo tena; tangu 2026-11-03 awali.bin yenyewe hujengwa
      kutoka awali.swm na mfululizo huru (hati/mfululizo-huru.md)
- [x] Kiunganishi cha kujitegemea: bendera ya `--programu` inatoa **ET_EXEC
      tuli** (bila maelezo ya sehemu, bila PT_INTERP) -- mfululizo wa
      kujikusanya wa programu ni safi kabisa: kamili2 == kamili3
      (sawa kwa baiti) bila ld, gcc, wala libc.
- [x] Runtime ya syscalls: mkusanyaji hautekelezi kupitia libc tena --
      mfumo_fungua/mfumo_soma/mfumo_andika/mfumo_ramani_kumbukumbu kupitia wito_wa_mfumo;
      undefined za nje za kamili2 ni mbili tu (violesura vya JIT).
- [x] 0% kujianzisha pengo -- awali inatoa ET_EXEC tuli moja kwa moja
      (`--programu`): utangulizi + maelezo ya kipande + kianzishi cha `_start` + urekebishaji wa
      RELA wa ndani. Hakuna gcc, ld, clang, muda.c, wala libc popote
      kwenye mfululizo: kwanza -> awali -> kamili -> kamili2 ->
      kamili3 (kamili2 == kamili3, sawa kwa baiti).
      Kumbuka: hili linahusu mfululizo wa uzalishaji pekee -- uthabiti
      wa mchanganuzi dhidi ya ingizo baya ni mhimili tofauti, bado
      wazi (angalia hati/mipaka.md).

## Kanuni Zisizobadilika

1. **Hakuna lugha nyingine.** Swa inajitegemea kabisa.
2. **Hakuna LLVM.** Inakufa mara tu uzalishaji.swa inapokamilika.
3. **Hakuna assembly/GNU as.** Machine msimbo mmoja kwa moja.
4. **Kiswahili pekee.** Maoni yote, majina ya faili, maneno muhimu.
5. **Hakuna bloat.** Kila neno linafanya jambo moja.
6. **Maneno muhimu 13.** Seti ndogo isiyobadilika; aina za nambari
   hutambuliwa kisintaksia (herufi kubwa), si kama maneno muhimu.
   (fanya iliongezwa 2026-09: mzunguko wa do-wakati. Maneno:
   muundo, rudisha, kama, sivyo, wakati, kwa, fanya, acha,
   endelea, chagua, hali, husisha, achilia.)
7. **Hakuna emoji.**
8. **Bootstrap ya mwisho kwa baiti za mkono.**

## Machapisho Muhimu

- `readme.md` -- ukurasa wa kwanza
- `hati/ramani.md` -- ramani ya mradi
- `hati/malengo.md` -- huu hapa (lengo letu halisi)
- `hati/mipaka.md` -- mipaka inayojulikana (kwa ukali)
- `msingi/mkusanyaji/uzalishaji.swa` -- uzalishaji asilia (inajikusanya)
- `msingi/mkusanyaji/mteremko.swa` -- kiteremshi cha AST->IR ya mstari (hati/ir.md); `mtafsiri.swa` huitekeleza (`swa --tafsiri`), na ndiyo msingi wa sehemu za nyuma zijazo
- `msingi/mkusanyaji/kiendeshaji.swa` -- kiendeshi cha kujianzisha
- `msingi/awali.swm` -- awali kwa lugha ya mashine ya Swa; hujengwa na
  mfululizo huru kutoka kwanza.bin (baiti 393), bila NASM wala ld

## Kile Tunachofanya Sasa (Agosti 2026)

Lengo kuu limefikiwa: mfululizo wa kujikusanya umefungwa kikamilifu na
**hakuna lugha nyingine** -- baiti za mkono (kwanza, 393) -> awali ->
kamili -> kamili2 == kamili3, bila gcc/ld/clang/libc popote.
(0% pengo ni mhimili wa mfululizo wa uzalishaji; uthabiti wa makosa ni
mhimili tofauti -- hati/mipaka.md.)

Hali ya Agosti 2026 (kuelekea 1.0):

1. **Uthabiti wa makosa ya mchanganuzi -- IMEKAMILIKA** -- ingizo baya
   linalia kwa sauti (mipaka.md 1-3, 8), kusoma bomba hadi EOF,
   mzunguko mfupi wa && na ||, na kazi isiyofafanuliwa inalia.
2. **Maktaba ya kawaida -- IMEKAMILIKA** -- hesabu (gcd, pow, isqrt,
   fibonacci), mifuatano kamili (tafuta, kata), I/O (soma_mstari),
   sort -- jaribio_maktaba_awali_exe.
3. **Vipimo rasmi vya lugha -- IMEKAMILIKA** -- hati/vipimo-vya-lugha.md.
4. **Uamuzi wa mteremko.swa/LLVM -- IMEFANYIKA** -- mwisho wa LLVM ni
   wa MAJARIBIO; mfululizo wa uzalishaji ni awali/exe pekee (mipaka.md 6).
   Dereva wa Rust/LLVM umehamishiwa hazina ya kumbukumbu
   lugha-swa/swa-dereva (2026-09-05); hazina kuu ina Swa pekee.
5. **ABI ya wito wa kazi za D64** -- IMEFANYIKA kwenye mifululizo
   yote miwili (awali inatumia uhamisho wa GP; kilichopimwa
   2026-08-27 -- mipaka.md 4c imerekebishwa). Mpaka wa D64 na
   nambari kamili umefunga (T2c), na D32 sasa inafanya kazi
   kikamilifu kwenye uzalishaji (T4/T8) -- hakuwezi kuorodheshwa
   kama kilichobaki tena.
6. **Mfumo wa moduli** -- IMEFANYIKA (2026-09) -- husisha ni kiungo
   halisi ndani ya mkusanyaji; msingi/maktaba/ na msingi/mkusanyaji/
   ndiyo mirundikano miwili ya maktaba moja.
7. **Uzingatiaji uliopimwa** -- rekodi kamili (hati/
   uthibitisho-wa-lugha.md) inahesabu kila kipengele kilichojaribiwa;
   majibu mabaya ya kimya ya uzalishaji yamepungua kutoka 96 hadi
   wachache walioorodheshwa na kirejeshi chao.

## Kuhusu Toleo la 1.0

1.0 si mwisho wa hitilafu. Ni AHADI: kuanzia siku hiyo, programu
zilizoandikwa dhidi ya vipimo hivi zinaendelea kufanya kazi.
Ahadi hiyo haifanywi kwa skani safi -- inafanywa wakati kuna watumiaji
wanaostahili kupewa ahadi. V (vlang) bado ni 0.4 baada ya miaka sita,
na ahadi za mapema za 1.0 ziliumiza sifa yake kuliko hitilafu zozote.

Njia ya Swa: 0.0.x kwa marekebisho, 0.1.0 kuanzia mfumo wa moduli na
vielekezi vya kazi, na 1.0.0 siku ambayo watumiaji wa kweli
wameandika programu za kweli -- si mapema.
