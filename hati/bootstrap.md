# Mpango wa Bootstrap ya Swa

## Lengo

**0% bootstrap gap.** Andika mkusanyaji wa kwanza wa Swa kwa baiti za mkono
(opcodes za x86-64). Hakuna lugha nyingine, hata assembly.

## Hatua za Bootstrap

### Hatua ya 0: Mzalishaji wa ELF (baiti ~100)
Binary inayotoa ELF inayorudisha 42. Haichanganui chochote —
inachapa tu pato lililowekwa ngumu.

### Hatua ya 1: Mchanganuzi wa Nambari (baiti ~250)
Ongeza uwezo wa kusoma nambari kutoka kwenye hoja ya mstari.
`rudisha 42` inakuwa `rudisha <nambari yoyote>`.

### Hatua ya 2: Mchanganuzi wa Neno (baiti ~400)
Ongeza utambuzi wa maneno muhimu: `rudisha`, `N32`, `main`.
Inaweza kuchanganua `N32 main() { rudisha <N>; }`.

### Hatua ya 3: Kujikusanya (baiti ~500)
Binary inaweza kujikusanya yenyewe. Inachukua chanzo chake
mwenyewe na kutoa binary inayofanana.

## Muundo wa Faili

Faili litakuwa `msingi/bootstrap.bin` — baiti mbichi za x86-64.
Hakuna kichwa, hakuna muundo — opcodes safi.
