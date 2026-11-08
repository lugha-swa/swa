# Daraja la mashine: Swa ikiita msimbo wa nje

Hali: awamu 1 na 2 zimetekelezwa (`gharama/jaribu-daraja.mapishi`). Vipimo vya
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

n32 main() {
    d64 x = kosini(0.0);          // wito wa kawaida wa Swa
    ...
}
```

Kanuni:

1. `nje "<maktaba>" { ... }` ni tangazo la ngazi ya juu. Mfuatano ni
   jina la maktaba kama mfumo unavyoliita (data yao, si jina letu).
2. Kila mstari ni kichwa cha kazi ya Swa bila mwili. `= "<alama>"`
   huunganisha jina letu na jina lao. Bila `=`, alama ni jina la Swa
   lenyewe.
3. **Majina yetu ni ya Kiswahili kwa maana; majina yao ni mifuatano.**
   Kiingereza cha maktaba ya nje (`cos`, `vkCreateInstance`) huingia
   kama data, kamwe kama kitambulisho cha Swa.
4. `nje` HALIWI neno muhimu: linatambuliwa tu kwenye ngazi ya juu
   likifuatiwa na mfuatano. Hivyo `n8* nje` (kigeu cha "matokeo",
   kinachotumika kwenye gharama/jaribu_ir.swa, jaribu_mnyororo.swa,
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
   huamuliwa na lengo (`--exe` kwenye Linux x86-64 = System V; Windows
   x86-64 = Win64 baadaye). Hakuna mapambo, hakuna maneno ya ziada.
7. Kwa kuwa saini inajulikana, mkaguzi hukagua aina za hoja na matokeo
   (tofauti na wito kupitia kielekezi, sehemu 4.3), na matokeo ya `d64`
   huja kwenye xmm0 sahihi.

## Wito wa kurudi (callback)

`&linganisha_namba` ya Swa ikipitishwa kwa `panga` (qsort), msimbo wa
nje utaita Swa. Swa lazima:

- ihifadhi rbx, rbp, r12-r15 -- `majaribio/daraja/rudi.swa` (qsort
  ikiita kazi ya Swa mara nyingi) huthibitisha;
- iwe na rsp iliyopangiliwa kwa baiti 16 kwenye kila wito wa nje
  (msimbo wa C hutumia movaps; rafu isiyopangiliwa huanguka). Kipande
  cha kila kazi ya nje hupangilia; `majaribio/daraja/upangiliaji.swa`
  husoma rsp halisi kupitia getcontext kwenye kina tofauti na katikati
  ya usemi. Jaribio la mabadiliko: kipande bila `and rsp,-16` hushindwa
  (kutoka 20).

## Faili la programu

- Programu isiyo na `nje`: ELF tuli kama leo, **sawa kwa baiti** na
  sasa. Hakuna kinachobadilika kwa programu zilizopo wala kwa mnyororo.
- Programu yenye `nje`: ELF ya kiungo-hai (dynamic), inayoandikwa na
  Swa yenyewe:
  - PT_INTERP: kipakiaji cha mfumo (`/lib64/ld-linux-x86-64.so.2`);
  - PT_DYNAMIC: DT_NEEDED kwa kila maktaba, DT_STRTAB, DT_SYMTAB,
    DT_HASH, DT_RELA (hakuna PLT, hivyo hakuna ufungaji wa kuchelewa);
  - GOT: sloti ya baiti 8 kwa kila alama kwenye .data, iliyojazwa na
    kipakiaji (R_X86_64_GLOB_DAT);
  - kipande kimoja kwa kila kazi kwenye .text, chenye jina la Swa:
    `push rbp; mov rbp,rsp; and rsp,-16; call [rip+sloti]; mov rsp,rbp;
    pop rbp; ret`. Wito wa Swa hauhitaji kubadilika: unaelekea kipande
    kama kazi nyingine. Hoja za rejesta na matokeo (rax, xmm0)
    hazigusiwi.
  - Hoja za rafu (awamu 2): mpigaji wa Swa huziacha juu ya rafu kwa
    mpangilio wa System V, hivyo ndani ya kipande ziko `[rbp+16+8k]`.
    Kipande huzinakili chini ya rsp mpya iliyopangiliwa (`push qword
    [rbp+16+8k]`, kuanzia ya mwisho; sloti moja ya ziada ikiwa idadi ni
    witiri). Kazi bila `...`: idadi inajulikana wakati wa kukusanya.
  - Hoja za ziada (`...`, awamu 2): kipande hakijui idadi, hivyo mpigaji
    huweka `r11 = sloti za rafu` na `eax = rejesta za xmm zilizotumika`
    (al ya System V, kikomo 8) mara kabla ya `call`; kipande hunakili
    sloti r11 kwa kitanzi. Hoja ya ziada ya `d32` ni kosa: C hupandisha
    float kuwa double, hivyo `d64` itumike.
  - Data ya maktaba (awamu 2): `n8* mkondo = "stdout";` ndani ya `nje`
    ni kigezo cha ulimwengu cha Swa kwenye .data (kianzio 0). Alama yake
    imefafanuliwa kwenye dynsym (STT_OBJECT, SHN_ABS, ukubwa wake), na
    R_X86_64_COPY humfanya kipakiaji anakili thamani ya maktaba ndani
    yake; maktaba zenyewe hutumia nakala hiyo (programu iko kwanza
    kwenye utafutaji). DT_HASH ina ndoo moja na alama zote kwenye
    mnyororo mmoja -- hash yenyewe haihitajiki. Namba na vielekezi
    pekee; safu, desimali na muundo ni makosa.

### Rekebisho la wito wa Swa wenyewe (awamu 2)

Hoja za rafu zilifichua mdudu wa zamani wa Swa: mpigaji alivuta hoja
kwa mpangilio, hivyo hoja ya rejesta iliyofuata hoja ya rafu (mf. n64
saba + d64) ilisomwa kutoka sloti ya hoja ya rafu -- thamani ya uongo
kimya. Mwitwaji alihesabu nafasi ya rafu kwa hoja za namba pekee, na
desimali ya 9+ haikusomwa kabisa. Sasa mpangilio uliochanganyika
hupakiwa kwa ofseti na hoja za rafu hubanwa (`hoja_kwa_sysv`), na
mwitwaji ana kaunta moja ya rafu. Mipangilio iliyofanya kazi hubaki
baiti zile zile. Jaribio: `majaribio/programu/jaribio_hoja_rafu_mchanganyiko.swa`.
  - Jedwali lote (interp, dynstr, dynsym, hash, rela, dynamic, phdr)
    huandikwa mwishoni mwa .data; e_phoff huelekeza kwenye phdr hizo,
    na phdr ya baiti 64-119 hubaki nakala ya PT_LOAD ile ile, hivyo
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
| 1 (imekamilika) | `nje`, aina za namba/kielekezi/d64/d32, ELF ya kiungo-hai, ukaguzi wa aina, upangiliaji wa rafu | libm: `cos`, `pow`, `sqrt`; libc: `strlen`, `qsort` yenye kazi ya kurudi ya Swa, `getcontext` kwa upangiliaji; makosa 13 kwa ujumbe kamili; programu 460 zilizopo sawa kwa baiti (--exe na --wasm); fixpoint |
| 2 (imekamilika) | hoja za rafu, hoja za ziada (`...`, r11 + al), data ya maktaba (R_X86_64_COPY), rekebisho la wito wa Swa | `snprintf`: namba 9 za ziada (6 rafuni), desimali zikichanganyika, desimali 10 (2 rafuni) + namba; kichwa cha kudumu chenye sloti 3 za rafu; `optind` (thamani 1 inakiliwa) na `stdout` kwa `fputs`; makosa 19; majaribio 4 ya mabadiliko (kitanzi bila push, ofseti mbaya, COPY kama GLOB_DAT, al = 0) yote hukamatwa |
| 3 | muundo kwa thamani (uainishaji wa System V) | maktaba yenye muundo wa baiti 16 na 24 |
| 4 | `--wasm`: `nje "env" { ... }` huwa imports za WASM; daraja la JS huzalishwa na mkusanyaji; dom_* zilizowekwa ndani huondolewa | kaunta.swa bila swa-kivinjari.js iliyoandikwa kwa mkono |
| 5 | Win64 + PE (Windows), kisha ARM64 + Mach-O | programu ile ile kwenye kila jukwaa |

`--kernel` yenye `nje`: kosa la sauti (hakuna kipakiaji ring 0).

## Mipaka iliyobaki (kwa sauti, si kimya)

- muundo kwa thamani (hoja, matokeo, au data) -> kosa mpaka awamu 3;
- data ya desimali au safu -> kosa (tumia kielekezi);
- hoja 16 kwa wito (kikomo cha mkaguzi kwa wito wote wa Swa);
- mbegu haihitaji kubadilika: stage1 haitumii `nje`, kwa hiyo hakuna
  kugandisha upya.
