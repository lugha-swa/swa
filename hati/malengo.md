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

### Hatua ya 3: Kuziba pengo la bootstrap [MWISHO]
Andika baiti 500 za mkono (opcodes za x86-64) zinazounda mkusanyaji
mdogo wa Swa. Hii inavunja utegemezi wa mwisho kabisa.
- [ ] mbegu.s bado inategemea NASM (mistari 9,146)
- [ ] Hakuna lugha yoyote iliyowahi kuandika baiti za kwanza za mkusanyaji wake
- [ ] 0% bootstrap gap

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
- `msingi/uzalishaji.swa` — codegen asilia (inajikusanya)
- `msingi/mteremko.swa` — LLVM backend ya dereva wa Rust (msimbo mfu kwa kujikusanya)
- `msingi/stage1.swa` — kiendeshi cha bootstrap
- `msingi/mbegu.s` — mbegu ya NASM (itatoweka kwenye Hatua ya 3)

## Kile Tunachofanya Sasa (Agosti 2026)

Mnyororo wa kujikusanya umefungwa na JIT inafanya kazi. Kazi inayofuata
kwa umuhimu:

1. **Hatua ya 3: baiti za mkono** — andika mkusanyaji mdogo wa kwanza
   kwa baiti za opcodes za mkono, ukiondoa NASM kutoka kwenye mnyororo.
2. **Kuunganisha na runtime** — kuondoa utegemezi wa ld/gcc na muda.c
   kwenye mnyororo (kiunganishi cha kujikusanya au ELF ya kujitegemea,
   na syscalls moja kwa moja badala ya libc).
3. **ABI ya desimali** — hoja za float/double kupitia xmm0-xmm7.
4. **JIT kamili** — relocations za wito wa nje na kupitisha argv.
5. **Uamuzi wa mteremko.swa** — kuifuta au kuikamilisha.
