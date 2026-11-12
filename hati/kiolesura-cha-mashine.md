# Kiolesura cha mashine: Swa ikiita msimbo wa nje

Hali: awamu 1, 2, 3 na 4 zimetekelezwa (`gharama/jaribu-kiolesura.mapishi`,
`gharama/jaribu-wasm.mapishi`). Vipimo vya
lugha: `hati/vipimo-vya-lugha.md` sehemu 8.1.

## Lengo

Programu ya Swa iweze kuita kazi yoyote iliyokwisha kukusanywa kwenye
mfumo -- maktaba za mfumo wa uendeshaji, viendeshaji vya GPU, maktaba za
watu wengine -- kwa kuzungumza makubaliano yao ya baiti (ABI) moja kwa
moja. Hakuna chanzo cha C, hakuna mkusanyaji wa C, hakuna kiungo cha C:
Swa yenyewe huandika wito na faili la programu.

Hili si `husisha C::` iliyoondolewa. Ile ilitegemea zana za C kujenga
Swa; hili ni Swa ikizungumza kiolesura cha mashine, kama inavyozungumza
syscalls za Linux leo.

## Sintaksia

```
nje "libm.so.6" {
    d64 kosini(d64 pembe) = "cos";
    d64 kipeo(d64 msingi, d64 kiwango) = "pow";
}

nje "libc.so.6" {
    w0 panga(w0* safu, a64 idadi, a64 ukubwa, n8* linganisha) = "qsort";
}

n32 anza() {
    d64 x = kosini(0.0);          // wito wa kawaida wa Swa
    ...
}
```

Kanuni:

1. `nje "<maktaba>" { ... }` ni tangazo la ngazi ya juu. Mfuatano ni
   jina la maktaba kama mfumo unavyoliita (data yao, si jina letu).
2. Kila mstari ni tangazo la kazi ya Swa bila maudhui. `= "<alama>"`
   huunganisha jina letu na jina lao. Bila `=`, alama ni jina la Swa
   lenyewe.
3. **Majina yetu ni ya Kiswahili kwa maana; majina yao ni mifuatano.**
   Kiingereza cha maktaba ya nje (`cos`, `vkCreateInstance`) huingia
   kama data, kamwe kama kitambulisho cha Swa.
4. `nje` HALIWI neno muhimu: linatambuliwa tu kwenye ngazi ya juu
   likifuatiwa na mfuatano. Hivyo `n8* nje` (kigeu cha "matokeo",
   kinachotumika kwenye gharama/jaribu_ir.swa, jaribu_mfululizo.swa,
   mizani/kusanyiko.swa) kinaendelea kufanya kazi.
5. Aina ni za Swa pekee. Ramani ya kawaida:

   | Upande wa nje | Swa |
   |---|---|
   | `int` / `unsigned` | `n32` / `a32` |
   | `long`, `ssize_t` / `size_t` | `n64` / `a64` |
   | `double` / `float` | `d64` / `d32` |
   | `char*`, kielekezi chochote | `n8*`, `T*` |
   | `void` | `w0` |
   | kitambulisho kisichoonekana (handle) | `n8*` |

6. Makubaliano ya wito (calling convention) hayaandikwi na mpangaji:
   huamuliwa na lengo (`--programu` kwenye Linux x86-64 = System V; Windows
   x86-64 = Win64 baadaye). Hakuna mapambo, hakuna maneno ya ziada.
7. Kwa kuwa saini inajulikana, mkaguzi hukagua aina za hoja na matokeo
   (tofauti na wito kupitia kielekezi, sehemu 4.3), na matokeo ya `d64`
   huja kwenye xmm0 sahihi.

## Wito wa kurudi (callback)

`&linganisha_namba` ya Swa ikipitishwa kwa `panga` (qsort), msimbo wa
nje utaita Swa. Swa lazima:

- ihifadhi r3, r5, r12-r15 -- `majaribio/kiolesura/rudi.swa` (qsort
  ikiita kazi ya Swa mara nyingi) huthibitisha;
- iwe na r4 iliyopangiliwa kwa baiti 16 kwenye kila wito wa nje
  (msimbo wa C hutumia movaps; mrundikano usiopangiliwa huanguka). Kipande
  cha kila kazi ya nje hupangilia; `majaribio/kiolesura/upangiliaji.swa`
  husoma r4 halisi kupitia getcontext kwenye kina tofauti na katikati
  ya usemi. Jaribio la mabadiliko: kipande bila `na r4,-16` hushindwa
  (kutoka 20).

## Faili la programu

- Programu isiyo na `nje`: ELF tuli kama leo, **sawa kwa baiti** na
  sasa. Hakuna kinachobadilika kwa programu zilizopo wala kwa mfululizo.
- Programu yenye `nje`: ELF ya kiungo-hai (dynamic), inayoandikwa na
  Swa yenyewe:
  - PT_INTERP: kipakiaji cha mfumo (`/lib64/ld-linux-x86-64.so.2`);
  - PT_DYNAMIC: DT_NEEDED kwa kila maktaba, DT_STRTAB, DT_SYMTAB,
    DT_HASH, DT_RELA (hakuna PLT, hivyo hakuna ufungaji wa kuchelewa);
  - GOT: sloti ya baiti 8 kwa kila alama kwenye .data, iliyojazwa na
    kipakiaji (R_X86_64_GLOB_DAT);
  - kipande kimoja kwa kila kazi kwenye .text, chenye jina la Swa:
    `sukuma r5; hamisha r5, r4; na r4,-16; ita [kutoka_agizo+sloti]; hamisha r4, r5;
    vuta r5; rudi`. Wito wa Swa hauhitaji kubadilika: unaelekea kipande
    kama kazi nyingine. Hoja za rejesta na matokeo (r0, xmm0)
    hazigusiwi.
  - Hoja za mrundikano (awamu 2): mpigaji wa Swa huziacha juu ya mrundikano kwa
    mpangilio wa System V, hivyo ndani ya kipande ziko `[r5+16+8k]`.
    Kipande huzinakili chini ya r4 mpya iliyopangiliwa (`sukuma qword
    [r5+16+8k]`, kuanzia ya mwisho; sloti moja ya ziada ikiwa idadi ni
    witiri). Kazi bila `...`: idadi inajulikana wakati wa kukusanya.
  - Hoja za ziada (`...`, awamu 2): kipande hakijui idadi, hivyo mpigaji
    huweka `r11 = sloti za mrundikano` na `r0d = rejesta za xmm zilizotumika`
    (r0b ya System V, kikomo 8) mara kabla ya `ita`; kipande hunakili
    sloti r11 kwa mzunguko. Hoja ya ziada ya `d32` ni kosa: C hupandisha
    desimali kuwa d64, hivyo `d64` itumike.
  - Data ya maktaba (awamu 2): `n8* mkondo = "stdout";` ndani ya `nje`
    ni kigezo cha ulimwengu cha Swa kwenye .data (kianzio 0). Alama yake
    imefafanuliwa kwenye dynsym (STT_OBJECT, SHN_ABS, ukubwa wake), na
    R_X86_64_COPY humfanya kipakiaji anakili thamani ya maktaba ndani
    yake; maktaba zenyewe hutumia nakala hiyo (programu iko kwanza
    kwenye utafutaji). DT_HASH ina ndoo moja na alama zote kwenye
    mfululizo mmoja -- hash yenyewe haihitajiki. Namba na vielekezi
    pekee; safu, desimali na muundo ni makosa.
  - Muundo kwa thamani (awamu 3): tazama sehemu inayofuata. Data ya
    aina ya muundo (`Anwani6 anwani_ya_ndani6 = "in6addr_loopback";`) hupata
    alama ya ukubwa wa muundo, hivyo COPY hunakili baiti zote.

### Muundo kwa thamani (awamu 3)

Swa na System V hupitisha muundo kwa njia tofauti:

| | Swa | System V |
|---|---|---|
| hoja, baiti <= 8 | baiti zake kwenye rejesta ya GP | kwa darasa la baiti-8 |
| hoja, baiti 9-16 | kielekezi cha nakala kwenye GP | kila baiti-8 kwa darasa lake |
| hoja, baiti > 16 | kielekezi cha nakala kwenye GP | MEMORY: nakala kwenye mrundikano |
| matokeo, baiti <= 16 | kielekezi kilichofichwa (r7 -> r0) | r0/r2 na xmm0/xmm1 |
| matokeo, baiti > 16 | kielekezi kilichofichwa (r7 -> r0) | sawa na Swa |

Darasa la baiti-8: INTEGER ikiwa sehemu yoyote ndani yake si desimali
(kielekezi na namba ni INTEGER), SSE vinginevyo; `{ n32; d32; }` ni
INTEGER moja, `{ d32; d32; }` ni SSE moja (desimali mbili ndani ya xmm
moja). Muundo ndani ya muundo huainishwa kwa sehemu zake za ndani.
Muundo wa baiti <= 16 usiopata rejesta za kutosha kwa baiti-8 zake
zote huenda kwenye mrundikano wote -- lakini hoja zinazofuata bado hupata rejesta
zilizobaki.

Kwa hiyo kazi ya nje yenye muundo (hoja au matokeo) hupata kipande cha
kutafsiri badala ya kile kifupi:

```
sukuma r5; hamisha r5, r4; toa r4,112
hifadhi r7..r9 kwenye [r5-8..-48], xmm0..7 kwenye [r5-56..-112]
na r4,-16; [toa r4,8 sloti zikiwa witiri]; toa r4,8*sloti
kwa kila hamisho la mpango: mrundikano kupitia r0, kisha GP, kisha xmm
  (chanzo: sloti iliyohifadhiwa, hoja ya mrundikano wa Swa [r5+16+8k], au
  [kielekezi + umbali] kupitia r11)
hamisha r0d, <xmm zilizotumika>          ; r0b ya System V
ita [kutoka_agizo+sloti]
[matokeo <= 16: soma r11, [r5-8]; r0/r2, xmm0/xmm1 -> [r11]; hamisha r0, r11]
hamisha r4, r5; vuta r5; rudi
```

Mpango wote huhesabiwa wakati wa kukusanya kutoka tangazo la `nje`;
vyanzo vyote ni kumbukumbu iliyohifadhiwa, hivyo hakuna rejesta
inayoharibu nyingine. Kazi zisizo na muundo hubaki na kipande kifupi,
baiti zile zile.

**Mpangilio wa C, si kujaza kimya.** Swa haiongezi kujaza kati ya
sehemu (`{ n8 a; n32 b; }` ni baiti 5); C huongeza (baiti 8). Muundo
unaovuka kiolesura lazima uwe tayari na mpangilio wa C: kila sehemu
kwenye umbali wa kizidisho cha upangiliaji wake na ukubwa kizidisho
cha upangiliaji mkubwa. Vinginevyo ni kosa linalotaja sehemu, umbali,
na baiti za kujaza zinazohitajika -- mpangaji huongeza sehemu ya
kujaza mwenyewe, na mpangilio unaonekana kwenye chanzo.

Mipaka ya awamu 3 (kwa sauti): muundo kwenye kazi yenye `...`, au kama
hoja ya ziada; muundo mtupu. Kazi ya Swa inayoitwa na msimbo wa nje
(callback) ikipokea au kurudisha muundo kwa thamani bado hutumia njia
ya Swa -- tumia kielekezi kwenye kazi za kurudi.

### Rekebisho la wito wa Swa wenyewe (awamu 2)

Hoja za mrundikano zilifichua hitilafu ya zamani ya Swa: mpigaji alivuta hoja
kwa mpangilio, hivyo hoja ya rejesta iliyofuata hoja ya mrundikano (mf. n64
saba + d64) ilisomwa kutoka sloti ya hoja ya mrundikano -- thamani ya uongo
kimya. Mwitwaji alihesabu nafasi ya mrundikano kwa hoja za namba pekee, na
desimali ya 9+ haikusomwa kabisa. Sasa mpangilio uliochanganyika
hupakiwa kwa umbali na hoja za mrundikano hubanwa (`hoja_kwa_sysv`), na
mwitwaji ana kaunta moja ya mrundikano. Mipangilio iliyofanya kazi hubaki
baiti zile zile. Jaribio: `majaribio/programu/jaribio_hoja_mrundikano_mchanganyiko.swa`.
  - Jedwali lote (interp, dynstr, dynsym, hash, urekebishaji, dynamic, maelezo ya kipande)
    huandikwa mwishoni mwa .data; umbali wa maelezo ya vipande huelekeza kwenye maelezo ya kipande hizo,
    na maelezo ya kipande ya baiti 64-119 hubaki nakala ya PT_LOAD ile ile, hivyo
    .text bado huanza 0x400078.

### Uamuzi: kipakiaji cha mfumo, si chetu (kwa sasa)

Maktaba za mfumo (libm, libvulkan, libwayland) zenyewe zinategemea
libc na kipakiaji chake (TLS, vianzishi). Kipakiaji cha Swa
kingehitaji kuanzisha glibc -- kazi kubwa isiyo na faida. Kwenye
Windows na macOS kipakiaji cha mfumo ni lazima hata hivyo. Kwa hiyo:

- programu inayochagua `nje` hutumia kipakiaji cha jukwaa lake --
  utegemezi wa programu hiyo pekee, kwa chaguo lake;
- ujenzi wa Swa na programu zote zisizo na `nje` hubaki bila utegemezi
  wowote;
- kipakiaji cha Swa (kwa maktaba zilizojengwa na Swa, na Kawira) ni
  awamu ya baadaye, tunapoamua umbo la faili sisi wenyewe.

## Awamu

| Awamu | Kazi | Jaribio linalothibitisha |
|---|---|---|
| 1 (imekamilika) | `nje`, aina za namba/kielekezi/d64/d32, ELF ya kiungo-hai, ukaguzi wa aina, upangiliaji wa mrundikano | libm: `cos`, `pow`, `sqrt`; libc: `strlen`, `qsort` yenye kazi ya kurudi ya Swa, `getcontext` kwa upangiliaji; makosa 13 kwa ujumbe kamili; programu 460 zilizopo sawa kwa baiti (--programu na --wasm); uthabiti |
| 2 (imekamilika) | hoja za mrundikano, hoja za ziada (`...`, r11 + r0b), data ya maktaba (R_X86_64_COPY), rekebisho la wito wa Swa | `snprintf`: namba 9 za ziada (6 kwenye mrundikano), desimali zikichanganyika, desimali 10 (2 kwenye mrundikano) + namba; tangazo la kudumu lenye sloti 3 za mrundikano; `optind` (thamani 1 inakiliwa) na `stdout` kwa `fputs`; makosa 19; majaribio 4 ya mabadiliko (mzunguko bila sukuma, umbali mbaya, COPY kama GLOB_DAT, r0b = 0) yote hukamatwa |
| 3 (imekamilika) | muundo kwa thamani: uainishaji wa System V (INTEGER/SSE/MEMORY), kipande cha kutafsiri, matokeo kwenye r0:r2/xmm0:xmm1, data ya muundo; ukaguzi wa mpangilio wa C | libc/libm halisi: `div`, `ldiv` (matokeo baiti 8 na 16), `cabs`/`csqrt`/`cpow` (`double complex`, SSE,SSE), `cabsf`/`csqrtf` (`float complex`, SSE moja kutoka GP ya Swa), `inet_ntoa` (`struct in_addr`), `memcpy` kama matokeo MEMORY, `in6addr_loopback` kama data; `snprintf` huonyesha mrundikano: MEMORY ya baiti 24, muundo usiotosha rejesta, INTEGER+SSE, darasa ndani ya baiti-8 moja, xmm zilizojaa, desimali ya 9, miundo kutoka mrundikano wa Swa, muundo ndani ya muundo; makosa 7; programu 968 zilizopo sawa kwa baiti; majaribio 14 ya mabadiliko: 13 hukamatwa, moja ni sawa kimaana (bila kupakia r7 kwa matokeo MEMORY -- r7 ya Swa tayari ni ile ya System V) |
| 4 (imekamilika) | `--wasm`: kila `nje "<moduli>"` huwa uingizaji wa WASM (moduli, alama); `dom_*` 8 zilizowekwa ndani ya mkusanyaji zimeondolewa; DOM ni Swa (`msingi/maktaba/kivinjari.swa`) juu ya kiolesura cha jumla kinachoandikwa na `swa --wasm-js`; `msingi/kivinjari/swa-kivinjari.js` ya mkono imefutwa | majaribio yote ya DOM na matukio hupitia mfululizo mzima (Swa -> kivinjari.swa -> swa-js.js -> hati bandia); `jaribio_nje_js` hutumia uingizaji moja kwa moja; makosa 3 (desimali, `...`, data); mabadiliko 2 (kiolesura chenye hoja zilizobadilishwa, `dom_unda` bila maktaba) hukamatwa |
| 5 | Win64 + PE (Windows), kisha ARM64 + Mach-O | programu ile ile kwenye kila jukwaa |

`--kiini` yenye `nje`: kosa la sauti (hakuna kipakiaji ring 0).

## WASM (awamu 4)

Kivinjari hukubali JS na WASM pekee, na hufichua DOM kupitia JS pekee --
hilo ni mpaka wa usalama wa kivinjari, si jambo la kuzunguka. Kwa hiyo
kiolesura kipo, lakini kina umbo la `syscall/js` ya Go na `wasm_exec.js`:

- **Kiolesura cha jumla, kinachoandikwa na mkusanyaji.** `swa --wasm-js`
  huandika `swa-js.js`: shughuli 14 juu ya vitu vya JS vinavyoshikwa kwa
  namba (vishiko) -- `kimataifa`, `pata`, `weka`, `ita0..ita3`,
  `mfuatano`, `namba`, `kwa_namba`, `kazi` (kazi ya Swa kama kazi ya
  kurudi), `achilia`, `nodi_kuu`. Halina mantiki ya programu; ni sawa kwa
  kila programu, kama utangulizi wa ELF.
- **`nje` ni uingizaji wa WASM.** `nje "js" { n32 js_pata(n32 kitu, n8* sifa) = "pata"; }`
  huwa uingizaji (moduli `js`, alama `pata`). Moduli nyingine yoyote
  (mf. `wasi_snapshot_preview1`) hufanya kazi vivyo hivyo; mwenyeji
  huitoa kupitia hoja ya tatu ya `anzishaSwa(chanzo, nodi_kuu, ziada)`.
- **DOM ni Swa.** `msingi/maktaba/kivinjari.swa` hujenga `dom_unda`,
  `dom_ambatanisha`, `dom_sikiliza`, `dom_weka_namba`, `andika_ujumbe_wa_kivinjari`
  n.k. juu ya shughuli hizo. Kuongeza API mpya ya kivinjari ni kuandika
  Swa, si kubadilisha mkusanyaji wala JS.
- Hoja na matokeo ya uingizaji: `n32`, kielekezi au `w0` (wigo wa WASM wa
  Swa). `n8*` sasa inakubalika kama aina ya kielekezi (anwani ya i32);
  kusoma baiti kupitia `n8*` bado kunakataliwa. `...` na data ya nje ni
  makosa kwenye WASM.

## Mipaka iliyobaki (kwa sauti, si kimya)

- muundo kwa thamani kwenye `...`, na callback ya Swa yenye muundo kwa
  thamani (tumia kielekezi);
- data ya desimali au safu -> kosa (tumia kielekezi);
- hoja 16 kwa wito (kikomo cha mkaguzi kwa wito wote wa Swa);
- awali haihitaji kubadilika: kamili haitumii `nje`, kwa hiyo hakuna
  kugandisha upya.
