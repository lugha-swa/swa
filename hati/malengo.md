# Malengo ya Mradi wa Swa

## Lengo Kuu

**Swa itakuwa lugha ya kwanza duniani yenye uhuru kamili (0% bootstrap gap).**
Hakuna Rust, hakuna LLVM, hakuna GNU as, hakuna lugha nyingine yoyote
iliyowahi kuhusika katika mnyororo wake. Binary mbichi inayojitengeneza
yenyewe kupitia baiti za mkono.

## Jinsi Tutakavyofika

### Hatua ya 1: Kufa kwa LLVM [TUNAANZA SASA]
`uzalishaji.swa` inatoa **binary mbichi (.o)** moja kwa moja.
Hakuna assembly. Hakuna GNU as.
- ELF header + machine code (opcodes za x86-64)
- Inachukua nafasi ya mteremko.swa (LLVM) KABISA
- Lengo la kwanza: `N32 main() { rudisha 42; }` -> ELF binary inayofanya kazi

### Hatua ya 2: JIT halisi (kama HolyC)
Swa inatoa maelekezo moja kwa moja kwenye kumbukumbu na kuyatekeleza.
Hakuna faili. Hakuna binary. Hakuna OS kati.
- `mmap()` -> andika opcodes -> rukia

### Hatua ya 3: Kuziba pengo la bootstrap [MWISHO]
Andika baiti 500 za mkono (opcodes za x86-64) zinazounda mkusanyaji
mdogo wa Swa. Hii inavunja utegemezi wa mwisho kabisa.
- Hakuna lugha yoyote iliyowahi kuandika baiti za kwanza za mkusanyaji wake
- 0% bootstrap gap

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
- `msingi/uzalishaji.swa` — codegen asilia (inajengwa sasa)
- `msingi/mteremko.swa` — LLVM backend (itakufa baadaye)
- `msingi/stage1.swa` — kiendeshi cha bootstrap

## Kile Tunachoanza Sasa

**Jenga upya `uzalishaji.swa`** kutoa binary mbichi (ELF + opcodes)
badala ya assembly. Hii ndiyo hatua muhimu inayotutenganisha na LLVM.
