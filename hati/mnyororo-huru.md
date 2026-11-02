# Mnyororo huru: Swa kutoka baiti 393, bila zana za nje

## Tatizo

`mbegu.bin` ilizalishwa mara moja kwa NASM (assembler) na GNU ld (linker) -- programu za C za nje. Ujenzi wa kawaida hautumii tena zana hizo (`kwanza.bin` hujenga mbegu kutoka `mbegu.hex`), lakini baiti za `mbegu.hex` zenyewe zilitoka kwao. Kosa au msimbo uliofichwa ndani ya NASM/ld wakati huo ungerithiwa na kila kitu kilichojengwa baadaye (Thompson, "Reflections on Trusting Trust", 1984).

Kuandika assembler kwa Swa HAKUTATUI hili: assembler hiyo ingekusanywa na Swa, ambayo inatoka kwenye mbegu hiyo hiyo -- ingethibitisha mbegu kwa kutumia urithi wa mbegu.

## Suluhisho

Zana inayogeuza `mbegu.s` kuwa `mbegu.bin` lazima ijengwe kutoka `kwanza.bin` PEKEE, bila kupita kwenye `mbegu.bin`. Kila hatua imeandikwa kwa lugha ya hatua iliyo chini yake, na hujengwa nayo:

| Hatua | Zana | Imeandikwa kwa | Inaongeza |
|---|---|---|---|
| 0 | `kwanza` | baiti 393 za mkono | hex -> binary |
| 1 | `maoni` | hex safi (hujengwa na kwanza) | maoni (`;` `#`) |
| 2 | `lebo` | hex yenye maoni | lebo na umbali wa matawi kwa majina |
| 3 | `kusanyiko0` | hex yenye lebo | makro: majina ya maagizo |
| 4 | assembler ya x86 | kusanyiko0 | husoma `mbegu.s` moja kwa moja, huandika ELF |
| 5 | `mbegu.bin` mpya | hatua 4 | msimbo na data sawa kwa baiti na matokeo ya NASM |

## Kanuni

1. **Zana za nje hukagua tu, hazijengi.** `llvm-mc`/`objdump` hutumika kama kitabu cha marejeo cha usimbaji wa maagizo wakati wa kuandika. Kinachoingia kwenye hazina ni hex yenye maoni, ambayo kila baiti yake inakaguliwa na binadamu dhidi ya agizo lililo kando yake.
2. **Kila hatua inajithibitisha.** Hatua n hujengwa na hatua n-1 kutoka chanzo chake, kisha (pale inapowezekana) hujijenga yenyewe sawa kwa baiti.
3. **Uthibitisho wa mwisho ni baiti.** Hatua 5 lazima itoe msimbo (.text) na data (.data) sawa kwa baiti na `mbegu.bin` ya sasa. Kisha `mbegu.hex` inagandishwa upya kutoka hatua 5, na NASM/ld zinabaki kwenye historia tu.

## Hali

- [x] Hatua 1: `msingi/mnyororo/hatua1_maoni.hexc` (+ `.hex` safi kwa kwanza). Baiti 400 (msimbo 280). Hujijenga yenyewe; hujenga `kwanza.bin`.
- [x] Hatua 2: `msingi/mnyororo/hatua2_lebo.hexc` (hujengwa na maoni) + `.hex2` (chanzo chake kwa lebo zake mwenyewe). Baiti 730 (msimbo 610). Mapitio mawili; `:jina`, `!jina` (rel8), `%jina` (rel32), `&jina` (anwani). Hujijenga yenyewe sawa kwa baiti (matawi 61 kwa majina); lebo haijulikani -> kutoka 2, rel8 mbali -> 3.
- [ ] Hatua 3: `kusanyiko0`
- [ ] Hatua 4: assembler ya x86 (maagizo 63 ambayo `mbegu.s` hutumia)
- [ ] Hatua 5: kugandisha upya `mbegu.hex` kutoka hatua 4

Uthibitisho: `gharama/jaribu-mnyororo-huru.sh` (kwenye CI).
