# Mnyororo huru: Swa kutoka baiti 393, bila zana za nje

## Tatizo

`mbegu.bin` ilizalishwa mara moja kwa NASM (assembler) na GNU ld (linker) -- programu za C za nje. Ujenzi wa kawaida hautumii tena zana hizo (`kwanza.bin` hujenga mbegu kutoka `mbegu.hex`), lakini baiti za `mbegu.hex` zenyewe zilitoka kwao. Kosa au msimbo uliofichwa ndani ya NASM/ld wakati huo ungerithiwa na kila kitu kilichojengwa baadaye (Thompson, "Reflections on Trusting Trust", 1984).

Kuandika assembler kwa Swa HAKUTATUI hili: assembler hiyo ingekusanywa na Swa, ambayo inatoka kwenye mbegu hiyo hiyo -- ingethibitisha mbegu kwa kutumia urithi wa mbegu.

## Suluhisho

Zana inayogeuza chanzo cha mbegu kuwa `mbegu.bin` lazima ijengwe kutoka `kwanza.bin` PEKEE, bila kupita kwenye `mbegu.bin`. Kila hatua imeandikwa kwa lugha ya hatua iliyo chini yake, na hujengwa nayo:

| Hatua | Zana | Imeandikwa kwa | Inaongeza |
|---|---|---|---|
| 0 | `kwanza` | baiti 393 za mkono | hex -> binary |
| 1 | `maoni` | hex safi (hujengwa na kwanza) | maoni (`;` `#`) |
| 2 | `lebo` | hex yenye maoni | lebo na umbali wa matawi kwa majina |
| 3 | `kusanyiko0` | hex yenye lebo | makro: majina ya maagizo |
| 4 | `kusanyiko` | kusanyiko0 | lugha ya mashine ya Swa (`hati/kusanyiko-x86.md`): husimba x86-64 na kuandika ELF |
| 4d | `kusanyiko` (kujijenga) | kusanyiko la hatua 4 (`msingi/mnyororo/kusanyiko.swm`) | assembler imeandikwa kwa lugha yake; hujijenga lenyewe sawa kwa baiti |
| 5 | `mbegu.bin` | kusanyiko (`msingi/mbegu.swm`) | mkusanyaji wa Swa -- msimbo na data sawa kwa baiti na mbegu ya zamani ya NASM |

## Kanuni

1. **Zana za nje hukagua tu, hazijengi.** `llvm-mc`/`objdump` hutumika kama kitabu cha marejeo cha usimbaji wa maagizo wakati wa kuandika. Kinachoingia kwenye hazina ni hex yenye maoni, ambayo kila baiti yake inakaguliwa na binadamu dhidi ya agizo lililo kando yake.
2. **Kila hatua inajithibitisha.** Hatua n hujengwa na hatua n-1 kutoka chanzo chake, kisha (pale inapowezekana) hujijenga yenyewe sawa kwa baiti.
3. **Uthibitisho wa mwisho ni baiti.** Hatua 5 lazima itoe msimbo (.text) na data (.data) sawa kwa baiti na `mbegu.bin` ya sasa. Kisha `mbegu.hex` inagandishwa upya kutoka hatua 5, na NASM/ld zinabaki kwenye historia tu. (Imefanyika 2026-11-03.)

## Hali

- [x] Hatua 1: `msingi/mnyororo/hatua1_maoni.hexc` (+ `.hex` safi kwa kwanza). Baiti 400 (msimbo 280). Hujijenga yenyewe; hujenga `kwanza.bin`.
- [x] Hatua 2: `msingi/mnyororo/hatua2_lebo.hexc` (hujengwa na maoni) + `.hex2` (chanzo chake kwa lebo zake mwenyewe). Baiti 730 (msimbo 610). Mapitio mawili; `:jina`, `!jina` (rel8), `%jina` (rel32), `&jina` (anwani). Hujijenga yenyewe sawa kwa baiti (matawi 61 kwa majina); lebo haijulikani -> kutoka 2, rel8 mbali -> 3.
- [x] Hatua 3: `msingi/mnyororo/hatua3_kusanyiko0.hex2` (hujengwa na lebo) + `.kus` (chanzo chake kwa lugha yake: makro 138 za maagizo kwa **majina ya Kiswahili**, rejista `r0`-`r15`). Baiti 1019. `FAFANUA jina baiti`, namba `!N %N ^N`, mifuatano `"..."`, lebo zinapitishwa; tokeni isiyojulikana -> kutoka 4. Hujijenga yenyewe sawa kwa baiti (.kus -> kusanyiko0 -> lebo).
- [x] Hatua 4: `msingi/mnyororo/hatua4_kusanyiko.kus` (hujengwa na kusanyiko0 + lebo; makro 774, baiti 9131). Lugha ya mashine ya Swa kwa maana, si tafsiri ya majina ya Intel: `r0 = r3`, `r0d += 5`, `n32[jedwali + r12*4] = r0d`, `linganisha r1d, 0` / `ruka .x kama ==`, `r0d = a r0b`, `x0 = n r3`, data kama Swa (`n8 ujumbe = "habari", 0`). Mapitio hadi kutulia; ukubwa wa matawi kwa sheria ile ile ya NASM. Majaribio: programu 4 zinaendeshwa, maumbo 107 ya maagizo sawa kwa baiti na NASM (`majaribio/maagizo.tarajiwa`).
- [x] Hatua 4d: `msingi/mnyororo/kusanyiko.swm` (mistari 2,832) -- kusanyiko limeandikwa kwa lugha yake lenyewe. Kusanyiko la hatua 4 hulijenga (`kusanyiko2`), na `kusanyiko2` hujijenga lenyewe sawa kwa baiti; hutoa matokeo sawa kwa baiti na la hatua 4 kwa kila jaribio, maumbo yote 107 ya maagizo, kila kosa, na `mbegu.bin` nzima. `gandisha-mbegu.sh` hujenga mbegu kwa `kusanyiko2`. Kuanzia sasa assembler hubadilishwa kwenye `kusanyiko.swm`; `hatua4_kusanyiko.kus` hubaki kama daraja la kuanzia tu (zana za nje hazihitajiki tena hata kwa uandishi).
- [x] Hatua 5: `msingi/mbegu.swm` (mistari 15,492 -- mbegu yote kwa lugha ya mashine ya Swa). Kusanyiko likiikusanya: msimbo baiti 43,209, data baiti 3,032, mahali pa kuanzia 0x40ab65 -- **sawa kwa baiti** na mbegu ya NASM + ld (sha256 c2b38fba...a249209). Mbegu hiyo ndiyo `msingi/mbegu.bin` sasa (baiti 52,184, sha256 6f1d0df0...daf3e0e388; ELF ya kusanyiko -- bila sehemu wala symtab za ld), na `mbegu.hex` imegandishwa upya kutoka kwake (`gharama/gandisha-mbegu.sh`). `msingi/mbegu.s` imeondolewa. Mbegu mpya hupita majaribio yote: mnyororo 470, mnyororo asili 453, WASM 42, --kernel 15, usakinishaji 5; stage2 == stage3.

- [x] NASM imeondolewa kila mahali: kusanyiko lina `picha`, `biti 32`, maagizo ya ring 0, `bandari`, `jumuisha` (hati/kusanyiko-x86.md). `majaribio/kernel/stub.swm` (jaribio la --kernel) na kiini cha Kawira (lugha-swa/kawira `kiini.swm`) hujengwa nalo; picha zote mbili ni sawa kwa baiti na za NASM.

Uthibitisho: `gharama/jaribu-mnyororo-huru.sh` (kwenye CI). 

## Ujenzi bila bash: mjenzi

`msingi/mnyororo/mjenzi.swm` ni kiendeshaji cha mapishi kwa lugha ya mashine ya Swa (amri: `saraka`, `endesha PROG ... < NDANI > NJE`, `endesha_kosa`, `sawa`, `nakili`, `unganisha`, `hex`, `andika`). Huhifadhiwa kama `msingi/mjenzi.bin` + `msingi/mjenzi.hex`, na `mnyororo.mapishi` huthibitisha kwamba vyote viwili ni sawa kwa baiti na kusanyiko(mjenzi.swm). Mapishi:

| Mapishi | Kazi |
|---|---|
| `gharama/kusanyiko.mapishi` | baiti 393 -> maoni -> lebo -> kusanyiko0 -> kusanyiko, kila hatua ikijijenga; `jengo/kusanyiko` |
| `gharama/mbegu-mpya.mapishi` | `jengo/mbegu`, `jengo/mjenzi` kutoka vyanzo vyao (kujaribu kabla ya kugandisha) |
| `gharama/mnyororo.mapishi` | vilivyogandishwa (`mbegu.bin`, `mjenzi.bin`, `.hex` zao) == vinavyojengwa |
| `gharama/jenga.mapishi` | yote hadi `bin/swa` |
| `gharama/gandisha.mapishi` | gandisha mbegu na mjenzi kutoka vyanzo, kisha thibitisha |
| `gharama/jaribu-mnyororo-asili.mapishi` | majaribio ya Swa asili (`gharama/jaribu_mnyororo.swa`) |

Pengo pekee la kuzaliwa lililobaki ni kuanzisha programu moja (`./msingi/mjenzi.bin ...`). Mwenye shaka anaweza kujenga mjenzi kwa mkono kutoka `msingi/kwanza.bin` na `msingi/mjenzi.hex`.
