# Hali ya Lugha ya Swa

Hati hii inaeleza hali halisi ya Swa kama LUGHA — kila dai lina
msingi wa jaribio au ukweli uliothibitishwa kwenye mradi. Hakuna
lugha ya masoko: kilicho kamili kimeandikwa kamili, kilicho na
kikomo kimeandikwa na kikomo chake.

## Kilichokamilika (kila kimoja kimejaribiwa)

| Kipengele | Ushahidi |
|---|---|
| Kama/sivyo (if/else) | majaribio ya K-series, jaribio_kama_sivyo |
| Wakati (while) | jaribio_wakati, mnyororo mzima wa kujikusanya |
| Chagua (switch) | jaribio_chagua_* |
| Urejeshaji (kujirudia, wito wa mbele, pande mbili) | jaribio_mwito_wa_*, mkazo wa RELA (fibonacci=55) |
| kwa (for) — KAMILI kwenye mbegu | mkazo wa RELA (s=125), jaribio la CI; desugaring sawa na msambazaji wa .swa |
| Miundo, kielekezi, &, nyoosha | jaribio_k13_* |
| JIT (--jit) NDANI ya exe | jaribio la CI: '; JIT: matokeo=42'; UND=0 kwenye exe |
| Kujikusanya (0% bootstrap gap) | stage2 == stage3 sawa kwa baiti; kwanza → mbegu → stage1-exe |
| Uthibitisho wa nje wa RELA | GNU ld inakubaliana na toa_exe sawa kwa baiti |
| Ukaguzi wa bafa (mbegu) | hati/ukaguzi-bafa.md — kila mpaka unalia kwa sauti |
| Mzunguko mfupi wa && na || KATIKA MBEGU (sawa na uzalishaji.swa) | jaribio_mbegu_mzunguko_mfupi (SEGV ya zamani) |
| Usomaji wa stdin hadi EOF (bomba) — matokeo yana uhakika | jaribio_mbegu_stdin_bomba_kubwa (mkato wa zamani) |
| Kazi isiyofafanuliwa inalia kwa sauti (si SEGV) | jaribio_mbegu_kazi_kukosa |
| Maktaba ya kawaida kamili (hesabu, mifuatano, I/O, sort) | jaribio_maktaba_mbegu_exe |
| Vipimo rasmi vya lugha | hati/vipimo-vya-lugha.md |

## Kilicho na kikomo (kilichoandikwa kwa ukali)

| Kikomo | Ukali | Hali |
|---|---|---|
| `endelea` ndani ya `kwa` inaruka mwanzo, si hatua (sio semantiki ya C; sawa na msambazaji wa .swa) | CHINI (kikomo cha semantiki) | Issue #156, pre-1.0 |
| Dereva wa Rust/LLVM: MAJARIBIO; O0 inakataa kwa kosa lauti kazi >40 ya vizuizi; majaribio yote kwa O1 (ISel kamili) | CHINI (njia ya kupita ipo; mnyororo wa uzalishaji ni mbegu/exe) | hati/mipaka.md #6 |
| Upeo wa tokeni 65,536 — inalia kwa sauti | CHINI | hati/mipaka.md #3 |
| Maneno halisi 32-bit signed (2147483648 → -2147483648 kwa uthabiti) | CHINI | hati/mipaka.md #5 |
| ABI ya desimali (xmm0-xmm7) — D64 inafanya kazi, ABI kamili haijafanyika | KATI | ramani |
| Majedwali ya ndani ya uzalishaji.swa na orodha/ramani — ukaguzi wa bafa bado haujakamilika | KATI | hati/ukaguzi-bafa.md |

## Kinachokosekana kabla ya 1.0

- Majukwaa ya asilia zaidi ya x86-64 Linux
- Zana (LSP, debugger, formatter, package manager)
- Bomba la desimali — LIMEFUNGWA 2026-08-20 (mipaka.md 4c; kikomo
  cha CHINI cha mbegu kwenye wito wa D64 kinaelekeza kwa .swa)

Kinachokamilika kwa 1.0 (2026-08): maktaba ya kawaida kamili,
vipimo rasmi vya lugha, na uamuzi wa mwisho wa LLVM (MAJARIBIO —
mnyororo wa uzalishaji ni mbegu/exe pekee).

## Uthibitisho wa jumla

Majaribio 219/219 (146 maktaba + 72 ujumuishaji + 1 nyaraka).
Fixpoint: stage2-exe == stage3-exe sawa kwa baiti baada ya kila
mchanganyiko. Alama za nje za exe: SIFURI.
