# Mpango wa Bootstrap ya Swa

## Lengo

**0% bootstrap gap.** Andika mkusanyaji wa kwanza wa Swa kwa baiti za mkono
(opcodes za x86-64). Hakuna lugha nyingine, hata assembly.

## Ilivyotimia

Mpango wa awali (Julai 2026) ulikuwa hatua nne za binary moja inayokua:
mzalishaji wa ELF (`rudisha 42`), kisha nambari, kisha maneno
(`N32 anza() { rudisha <N>; }`), kisha kujikusanya. Hatua tatu za kwanza
zilifanyika kama `msingi/bootstrap.bin` (baiti 1305: vigeu na `+ - *`),
lakini baiti zake zilizalishwa na `bootstrap.py` -- Python, si mkono -- na
hatua ya nne haikufikiwa kwa njia hiyo.

Lengo lenyewe lilitimizwa na **mnyororo huru** (`hati/mnyororo-huru.md`):

- `msingi/kwanza.bin` -- baiti 393 za mkono kweli (hex -> binary)
- maoni -> lebo -> kusanyiko0 -> kusanyiko: kila hatua imeandikwa kwa lugha
  ya iliyo chini yake, na kusanyiko (lugha ya mashine ya Swa) hujijenga lenyewe
- `msingi/mbegu.swm` -> `mbegu.bin`, mkusanyaji wa Swa -> stage1 -> `bin/swa`
- `msingi/mjenzi.bin` huendesha yote: `./msingi/mjenzi.bin gharama/jenga.mapishi`

Hakuna assembler, linker wala compiler ya nje popote kwenye mnyororo.

## bootstrap.bin ilikwenda wapi

Imeandikwa upya kwa maana, kutoka kwenye baiti zake, kama
`mifano/mkusanyaji_mdogo.swm` -- mfano wa mkusanyaji kamili mdogo kwa lugha
ya mashine ya Swa. Programu unazotoa zilithibitishwa kuwa sawa kwa baiti na
za `bootstrap.bin` kwa visa vyote vya `gharama/jaribu-mifano.mapishi`, ambayo
huujenga na kuujaribu kila CI. Marekebisho mawili: aina za herufi ndogo
(`n32`, lugha ya sasa), na `}` bila `rudisha` huandika 0 kwenye ofseti yake
halisi (0x81) badala ya ndani ya kitambulisho cha ELF (ofseti 9).
