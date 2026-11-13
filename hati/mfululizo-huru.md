# Mfululizo huru: Swa kutoka baiti 393, bila zana za nje

## Tatizo

`awali.bin` ilizalishwa mara moja kwa NASM (kusanyiko) na GNU ld (kiungaji) -- programu za C za nje. Ujenzi wa kawaida hautumii tena zana hizo (`kwanza.bin` hujenga awali kutoka `awali.heksa`), lakini baiti za `awali.heksa` zenyewe zilitoka kwao. Kosa au msimbo uliofichwa ndani ya NASM/ld wakati huo ungerithiwa na kila kitu kilichojengwa baadaye (Thompson, "Reflections on Trusting Trust", 1984).

Kuandika kusanyiko kwa Swa HAKUTATUI hili: kusanyiko hilo ingekusanywa na Swa, ambayo inatoka kwenye awali hiyo hiyo -- ingethibitisha awali kwa kutumia urithi wa awali.

## Suluhisho

Zana inayogeuza chanzo cha awali kuwa `awali.bin` lazima ijengwe kutoka `kwanza.bin` PEKEE, bila kupita kwenye `awali.bin`. Kila hatua imeandikwa kwa lugha ya hatua iliyo chini yake, na hujengwa nayo:

| Hatua | Zana | Imeandikwa kwa | Inaongeza |
|---|---|---|---|
| 0 | `kwanza` | baiti 393 za mkono | heksa -> binari |
| 1 | `maoni` | heksa safi (hujengwa na kwanza) | maoni (`;` `#`) |
| 2 | `mahali` | heksa yenye maoni | mahali na umbali wa matawi kwa majina |
| 3 | `kusanyiko0` | heksa yenye mahali | makro: majina ya maagizo |
| 4 | `kusanyiko` | kusanyiko0 | lugha ya mashine ya Swa (`hati/kusanyiko-x86.md`): husimba x86-64 na kuandika ELF |
| 4d | `kusanyiko` (kujijenga) | kusanyiko la hatua 4 (`msingi/mfululizo/kusanyiko.swm`) | kusanyiko limeandikwa kwa lugha yake; hujijenga lenyewe sawa kwa baiti |
| 5 | `awali.bin` | kusanyiko (`msingi/awali.swm`) | mkusanyaji wa Swa -- msimbo na data sawa kwa baiti na awali ya zamani ya NASM |

## Kanuni

1. **Zana za nje hukagua tu, hazijengi.** `llvm-mc`/`objdump` hutumika kama kitabu cha marejeo cha usimbaji wa maagizo wakati wa kuandika. Kinachoingia kwenye hazina ni heksa yenye maoni, ambayo kila baiti yake inakaguliwa na binadamu dhidi ya agizo lililo kando yake.
2. **Kila hatua inajithibitisha.** Hatua n hujengwa na hatua n-1 kutoka chanzo chake, kisha (pale inapowezekana) hujijenga yenyewe sawa kwa baiti.
3. **Uthibitisho wa mwisho ni baiti.** Hatua 5 lazima itoe msimbo (.text) na data (.data) sawa kwa baiti na `awali.bin` ya sasa. Kisha `awali.heksa` inagandishwa upya kutoka hatua 5, na NASM/ld zinabaki kwenye historia tu. (Imefanyika 2026-11-03.)

## Hali

- [x] Hatua 1: `msingi/mfululizo/hatua1_maoni.maoni` (+ `.heksa` safi kwa kwanza). Baiti 400 (msimbo 280). Hujijenga yenyewe; hujenga `kwanza.bin`.
- [x] Hatua 2: `msingi/mfululizo/hatua2_mahali.maoni` (hujengwa na maoni) + `.mahali` (chanzo chake kwa mahali zake mwenyewe). Baiti 730 (msimbo 610). Mapitio mawili; `:jina`, `!jina` (rel8), `%jina` (rel32), `&jina` (anwani). Hujijenga yenyewe sawa kwa baiti (matawi 61 kwa majina); mahali haijulikani -> kutoka 2, rel8 mbali -> 3.
- [x] Hatua 3: `msingi/mfululizo/hatua3_kusanyiko0.mahali` (hujengwa na mahali) + `.kus` (chanzo chake kwa lugha yake: makro 138 za maagizo kwa **majina ya Kiswahili**, rejista `r0`-`r15`). Baiti 1019. `FAFANUA jina baiti`, namba `!N %N ^N`, mifuatano `"..."`, mahali zinapitishwa; tokeni isiyojulikana -> kutoka 4. Hujijenga yenyewe sawa kwa baiti (.kus -> kusanyiko0 -> mahali).
- [x] Hatua 4: `msingi/mfululizo/hatua4_kusanyiko.kus` (hujengwa na kusanyiko0 + mahali; makro 774, baiti 9131). Lugha ya mashine ya Swa kwa maana, si tafsiri ya majina ya Intel: `r0 = r3`, `r0d += 5`, `n32[jedwali + r12*4] = r0d`, `linganisha r1d, 0` / `ruka .x kama ==`, `r0d = a r0b`, `x0 = n r3`, data kama Swa (`n8 ujumbe = "habari", 0`). Mapitio hadi kutulia; ukubwa wa matawi kwa sheria ile ile ya NASM. Majaribio: programu 4 zinaendeshwa, maumbo 107 ya maagizo sawa kwa baiti na NASM (`majaribio/maagizo.heksa`).
- [x] Hatua 4d: `msingi/mfululizo/kusanyiko.swm` (mistari 2,832) -- kusanyiko limeandikwa kwa lugha yake lenyewe. Kusanyiko la hatua 4 hulijenga (`kusanyiko2`), na `kusanyiko2` hujijenga lenyewe sawa kwa baiti; hutoa matokeo sawa kwa baiti na la hatua 4 kwa kila jaribio, maumbo yote 107 ya maagizo, kila kosa, na `awali.bin` nzima. `gandisha-awali.sh` hujenga awali kwa `kusanyiko2`. Kuanzia sasa kusanyiko hubadilishwa kwenye `kusanyiko.swm`; `hatua4_kusanyiko.kus` hubaki kama kiolesura cha kuanzia tu (zana za nje hazihitajiki tena hata kwa uandishi).
- [x] Hatua 5: `msingi/awali.swm` (mistari 15,492 -- awali yote kwa lugha ya mashine ya Swa). Kusanyiko likiikusanya: msimbo baiti 43,209, data baiti 3,032, mahali pa kuanzia 0x40ab65 -- **sawa kwa baiti** na awali ya NASM + ld (sha256 c2b38fba...a249209). Awali hiyo ndiyo `msingi/awali.bin` sasa (baiti 52,184, sha256 6f1d0df0...daf3e0e388; ELF ya kusanyiko -- bila sehemu wala jedwali la alama za ld), na `awali.heksa` imegandishwa upya kutoka kwake (`gharama/gandisha-awali.sh`). `msingi/awali.s` imeondolewa. Awali mpya hupita majaribio yote: mfululizo 470, mfululizo asili 453, WASM 42, --kiini 15, usakinishaji 5; kamili2 == kamili3.

- [x] NASM imeondolewa kila mahali: kusanyiko lina `picha`, `biti 32`, maagizo ya ring 0, `bandari`, `jumuisha` (hati/kusanyiko-x86.md). `majaribio/kiini/kianzishi.swm` (jaribio la --kiini) na kiini cha Kawira (lugha-swa/kawira `kiini.swm`) hujengwa nalo; picha zote mbili ni sawa kwa baiti na za NASM.

Uthibitisho: `gharama/jaribu-mfululizo-huru.mapishi` (kwenye CI; visa vyake viko `msingi/mfululizo/majaribio/`). 

## Ujenzi bila bash: mjenzi

`msingi/mfululizo/mjenzi.swm` ni kiendeshaji cha mapishi kwa lugha ya mashine ya Swa (amri: `saraka`, `endesha PROG ... < NDANI > NJE`, `endesha_kosa`, `sawa`, `nakili`, `unganisha`, `heksa`, `andika`). Huhifadhiwa kama `msingi/mjenzi.bin` + `msingi/mjenzi.heksa`, na `mfululizo.mapishi` huthibitisha kwamba vyote viwili ni sawa kwa baiti na kusanyiko(mjenzi.swm). Mapishi:

| Mapishi | Kazi |
|---|---|
| `gharama/mwanzo.kizinduzi` | binari mbili za mwanzo pekee -> mjenzi, awali na kizinduzi kutoka vyanzo; mjenzi huo hulinganisha vilivyogandishwa (`gharama/thibitisha-awali.mapishi`) |
| `gharama/kusanyiko.mapishi` | baiti 393 -> maoni -> mahali -> kusanyiko0 -> kusanyiko, kila hatua ikijijenga; `jengo/kusanyiko` |
| `gharama/awali-mpya.mapishi` | `jengo/awali`, `jengo/mjenzi` kutoka vyanzo vyao (kujaribu kabla ya kugandisha) |
| `gharama/mfululizo.mapishi` | vilivyogandishwa (`awali.bin`, `mjenzi.bin`, `.heksa` zao) == vinavyojengwa |
| `gharama/jenga.mapishi` | yote hadi `bin/swa` |
| `gharama/gandisha.mapishi` | gandisha awali na mjenzi kutoka vyanzo, kisha thibitisha |
| `gharama/jaribu-mfululizo-asili.mapishi` | majaribio ya Swa asili (`gharama/jaribu_mfululizo.swa`) |
| `gharama/jaribu-mfululizo.mapishi` | majaribio ya Swa asili + mkazo na semantiki (`majaribio/mkazo/`) |
| `gharama/jaribu-mfululizo-huru.mapishi` | kila hatua ya mfululizo huru, kila kipengele na kosa (`msingi/mfululizo/majaribio/`) |
| `gharama/jaribu-mifano.mapishi`, `jaribu-usakinishaji.mapishi` | mifano ya README; swa iliyosakinishwa nje ya hazina |
| `gharama/jaribu-wasm.mapishi`, `jaribu-kiini.mapishi` | `--wasm` (node kama mkaguzi), `--kiini` (QEMU kama mkaguzi) |

Hakuna bash kwenye ujenzi wala kwenye majaribio ya CI: zana za nje pekee ni wakaguzi wa majaribio (node na QEMU).

## Binari mbili za mwanzo: kwanza na kizinduzi

Kabla ya hili, mfululizo uliendeshwa na `msingi/mjenzi.bin` (baiti 8559, heksa bila maoni) -- binari iliyoendesha kila hatua NA kufanya kila ulinganisho `sawa`, ikijithibitisha yenyewe. Kuendesha `kwanza.bin` kwa mkono kungehitaji programu nyingine ya kuelekeza stdin/stdout (shell) -- programu ya nje.

Sasa mfululizo huanza na binari mbili za mwanzo zinazosomeka baiti kwa baiti, kama hex0 na kaem-optional-seed za stage0:

| Awali | Baiti | Kazi | Maelezo |
|---|---|---|---|
| `msingi/kwanza.bin` | 393 | heksa -> binari | `hati/kwanza-baiti.md` |
| `msingi/kizinduzi.bin` | 839 | huendesha orodha ya hatua (`PROG HOJA [< NDANI] [> NJE]`, `saraka`, maoni `;`) na husimama kwa kosa la kwanza | `msingi/kizinduzi.maoni`: kila agizo na baiti zake, anwani, na mstari wake wa `msingi/mfululizo/kizinduzi.swm` |

`./msingi/kizinduzi.bin gharama/mwanzo.kizinduzi` hujenga maoni, mahali, kusanyiko0 na kusanyiko kutoka binari hizo, kisha **mjenzi, awali na kizinduzi kutoka vyanzo vyao**. Mjenzi huo uliojengwa -- si `msingi/mjenzi.bin` -- huendesha `gharama/thibitisha-awali.mapishi`, inayolinganisha kila binari iliyogandishwa (`kwanza`, `kizinduzi` kwa njia tatu: heksa, hexc, swm; `mjenzi`, `awali` na `.heksa` zao) na inayojengwa. strace: programu zinazoendeshwa ni binari mbili za mwanzo na zilizojengwa kutoka kwake pekee; `msingi/mjenzi.bin` haiendeshwi kamwe. Baiti moja ikibadilishwa kwenye `mjenzi.bin`, `awali.bin` au `kizinduzi.bin`, mwanzo hushindwa. Hii ni hatua ya kwanza ya CI.

Chanzo cha kuaminiwa ni baiti 393 + 839, zote zimeelezwa. Jinsi ya kuukagua kwa jicho, hatua kwa hatua: `hati/mwongozo-wa-mkaguzi.md`. Kama kwanza, kizinduzi hakiwezi kujithibitisha chenyewe (kizinduzi kilichoharibiwa ndicho kinachoendesha ukaguzi wake); kinga yake ni ukaguzi wa jicho wa `kizinduzi.maoni`, na faili hilo lazima liwe sawa kwa baiti na `kizinduzi.swm` (CI hukataa tofauti). `kizinduzi.swm` ndicho chanzo; ukikibadilisha, sasisha `kizinduzi.maoni` na `kizinduzi.heksa` pamoja nacho.
