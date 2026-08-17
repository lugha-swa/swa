# Malengo ya Mradi wa Swa

## Lengo Kuu

**Swa itakuwa lugha ya kwanza duniani yenye uhuru kamili (0% bootstrap gap).**
Hakuna Rust, hakuna LLVM, hakuna GNU as, hakuna lugha nyingine yoyote
iliyowahi kuhusika katika mnyororo wake. Binary mbichi inayojitengeneza
yenyewe kupitia baiti za mkono.

## Jinsi Tutakavyofika

### Hatua ya 1: Kufa kwa LLVM [IMEFANIKIWA]
`uzalishaji.swa` inatoa **binary mbichi (.o)** moja kwa moja.
Hakuna assembly. Hakuna GNU as.
- [x] ELF header + machine code (opcodes za x86-64)
- [x] Inachukua nafasi ya mteremko.swa (LLVM) KABISA katika mnyororo wa kujikusanya
- [x] `N32 main() { rudisha 42; }` -> ELF binary inayofanya kazi
- [x] Mnyororo mzima unajithibitisha: stage2.o == stage3.o == stage4.o (sawa kwa baiti)

### Hatua ya 2: JIT halisi (kama HolyC) [IMEFANYIKA]
Swa inatoa maelekezo moja kwa moja kwenye kumbukumbu na kuyatekeleza.
Hakuna faili. Hakuna binary. Hakuna OS kati.
- [x] `mmap()` -> andika opcodes -> rukia (PR #143)
- [x] Stub ya `jmp main` na tungo mwishoni mwa bafa
- [x] Thamani ya kurudi inarudi kwa usahihi (vipimo 5/5 vya JIT)
- [ ] Mipaka iliyobaki: wito wa kazi za nje ndani ya msimbo wa JIT
      (relocations hazijarekebishwa bado) na kupitisha argv

### Hatua ya 3: Kuziba pengo la bootstrap [IMEFANYIKA KWA MBEGU]
Andika baiti za mkono (opcodes za x86-64) zinazounda mkusanyaji wa kwanza.
- [x] Kwanza (msingi/kwanza.bin): baiti 393 zilizoandikwa kwa mkono
      (kichwa cha ELF 64 + kichwa cha programu 56 + msimbo 273) — hubadilisha
      hex hadi binary na inajijenga yenyewe (kwanza.hex -> kwanza.bin)
- [x] mbegu.bin inazalika kutoka mbegu.hex kupitia Kwanza — NASM
      si sehemu ya mnyororo wa uzalishaji tena; mbegu.s ni rejea tu
- [x] Kiunganishi cha kujitegemea: bendera ya `--exe` inatoa **ET_EXEC
      tuli** (bila kichwa cha sehemu, bila PT_INTERP) — mnyororo wa
      kujikusanya wa exe ni safi kabisa: stage2-exe == stage3-exe
      (sawa kwa baiti) bila ld, gcc, wala libc.
- [x] Runtime ya syscalls: mkusanyaji hautekelezi kupitia libc tena —
      sys_open/sys_read/sys_write/sys_mmap kupitia wito_wa_mfumo;
      undefined za nje za stage2.o ni mbili tu (daraja za JIT).
- [x] 0% bootstrap gap — mbegu inatoa ET_EXEC tuli moja kwa moja
      (`--exe`): kichwa + phdr + stub ya `_start` + urekebishaji wa
      RELA wa ndani. Hakuna gcc, ld, clang, muda.c, wala libc popote
      kwenye mnyororo: kwanza → mbegu → stage1-exe → stage2-exe →
      stage3-exe (stage2-exe == stage3-exe, sawa kwa baiti).
      Kumbuka: hili linahusu mnyororo wa uzalishaji pekee — uthabiti
      wa mchanganuzi dhidi ya ingizo baya ni mhimili tofauti, bado
      wazi (angalia hati/mipaka.md).

## Kanuni Zisizobadilika

1. **Hakuna lugha nyingine.** Swa inajitegemea kabisa.
2. **Hakuna LLVM.** Inakufa mara tu uzalishaji.swa inapokamilika.
3. **Hakuna assembly/GNU as.** Machine code moja kwa moja.
4. **Kiswahili pekee.** Maoni yote, majina ya faili, maneno muhimu.
5. **Hakuna bloat.** Kila neno linafanya jambo moja.
6. **Maneno 42.** Hayabadiliki bila sababu.
7. **Hakuna emoji.**
8. **Bootstrap ya mwisho kwa baiti za mkono.**

## Machapisho Muhimu

- `readme.md` — ukurasa wa kwanza
- `hati/ramani.md` — ramani ya mradi
- `hati/malengo.md` — huu hapa (lengo letu halisi)
- `hati/mipaka.md` — mipaka inayojulikana (kwa ukali)
- `msingi/uzalishaji.swa` — codegen asilia (inajikusanya)
- `msingi/mteremko.swa` — LLVM backend ya dereva wa Rust (msimbo mfu kwa kujikusanya)
- `msingi/stage1.swa` — kiendeshi cha bootstrap
- `msingi/mbegu.s` — mbegu ya NASM (itatoweka kwenye Hatua ya 3)

## Kile Tunachofanya Sasa (Agosti 2026)

Lengo kuu limefikiwa: mnyororo wa kujikusanya umefungwa kikamilifu na
**hakuna lugha nyingine** — baiti za mkono (kwanza, 393) → mbegu →
stage1-exe → stage2-exe == stage3-exe, bila gcc/ld/clang/libc popote.
(0% gap ni mhimili wa mnyororo wa uzalishaji; uthabiti wa makosa ni
mhimili tofauti — hati/mipaka.md.) Kazi inayofuata kwa umuhimu:

1. **Uthabiti wa makosa ya mchanganuzi** — mbegu inakubali/segfault
   kwa ingizo baya; mchanganuzi wa .swa unaning'inia kwa ingizo
   lililokatwa (hati/mipaka.md sehemu 1-2).
2. **JIT ndani ya exe** — jedwali la anwani za ndani (tekeleza na
   anwani_ya_kazi kama kazi za .swa) ili --jit ifanye kazi kwenye exe.
3. **ABI ya desimali** — hoja za float/double kupitia xmm0-xmm7.
4. **JIT kamili** — relocations za wito wa nje na kupitisha argv.
5. **Uamuzi wa mteremko.swa** — kuifuta au kuikamilisha.
