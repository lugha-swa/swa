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
| Chagua (switch) — kwenye uzalishaji PEKEE; mbegu inakataa kwa muundo (uthibitisho 2026-08-27); lebo hasi/N64 kubwa/usemi zinatoa jibu baya kwenye uzalishaji | jaribio_chagua_*; hati/uthibitisho-wa-lugha.md |
| Urejeshaji (kujirudia, wito wa mbele, pande mbili) | jaribio_mwito_wa_*, mkazo wa RELA (fibonacci=55) |
| kwa (for) — KAMILI kwenye mbegu; `endelea` inaruka HATUA (semantiki ya C) kwenye minyororo yote miwili | mkazo wa RELA (s=125), jaribio la CI; mipaka.md 4b |
| Miundo, kielekezi, &, nyoosha (sehemu, `->`, sret) | jaribio_k13_*; kikomo kilichopimwa 2026-08-27: safu za miundo zinaanguka SEGV kwa zote mbili, muundo kwa thamani ni takataka kwenye mbegu na unapoteza baiti > 8 kwenye uzalishaji |
| JIT (--jit) NDANI ya exe | jaribio la CI: '; JIT: matokeo=42'; UND=0 kwenye exe |
| Kujikusanya (0% bootstrap gap) | stage2 == stage3 sawa kwa baiti; kwanza → mbegu → stage1-exe |
| Uthibitisho wa nje wa RELA | GNU ld inakubaliana na toa_exe sawa kwa baiti |
| Ukaguzi wa bafa (mbegu) | hati/ukaguzi-bafa.md — kila mpaka unalia kwa sauti |
| Mzunguko mfupi wa && na || KATIKA MBEGU (sawa na uzalishaji.swa) | jaribio_mbegu_mzunguko_mfupi (SEGV ya zamani) |
| Usomaji wa stdin hadi EOF (bomba) — matokeo yana uhakika | jaribio_mbegu_stdin_bomba_kubwa (mkato wa zamani) |
| Kazi isiyofafanuliwa inalia kwa sauti kwenye uzalishaji (si SEGV); mbegu inakataa kimya katika kesi kadhaa | jaribio_mbegu_kazi_kukosa; hati/uthibitisho-wa-lugha.md |
| Maktaba ya kawaida (hesabu, mifuatano, I/O, sort) — kwa kiasi: badili na ukuaji wa Orodha zinaanguka (SEGV), ramani ni no-op kwenye mbegu, nambari_kwa_mfuatano_n64 imevunjika (uthibitisho 2026-08-27) | jaribio_maktaba_mbegu_exe; hati/uthibitisho-wa-lugha.md |
| Desimali (D64) — hesabu, ulinganisho, ukanushaji NA WITO WA KAZI kwenye minyororo yote miwili (mbegu inatumia ABI ya uhamisho wa GP; kilichopimwa 2026-08-27 — mipaka.md 4c imerekebishwa). Kikomo: kila mpaka kati ya D64 na nambari kamili umevunjika (jibu baya); D32 imevunjika | jaribio_mende_60_desimali_mbegu, jaribio_mende_135_desimali; hati/uthibitisho-wa-lugha.md |
| Herufi ndogo za aina za msingi (n8/n16/n32/n64/a*/d32/d64/b*/w* — sanjari na herufi kubwa, aina MOJA HASA, si aina mpya) — kwenye mkusanyaji uliojijenga (stage1/stage2) KAMILI; kwenye mbegu YENYEWE kwa aina 6 TU (n8/n16/n32/n64/w0/d64 — mzizi wa bootstrap umegandishwa upya 2026-09-24, angalia CONTRIBUTING.md "Sheria ya Mzizi wa Uaminifu") — mbegu bado HAIJUI familia A/B wala D32 kwa hali yoyote ya herufi. Majaribio 179 na maktaba 5 za jumla (mpangilio/nasibu/orodha/ramani/wakati.swa) zimehamishiwa herufi ndogo 2026-09-23/24. Faili zinazojengwa moja kwa moja na mbegu.bin zilikuwa HAZIWEZI kuhamishwa kabla ya mbegu kugandishwa upya (#257) — sasa Awamu B inaendelea (PR kadhaa zinazoshirikiana): msingi/mkusanyaji/msambazaji.swa (parser, PR hii); mkaguzi.swa (#263); msomaji.swa na mteremko.swa (#262); kumbukumbu.swa, mfuatano.swa, faili.swa, hesabu.swa, gharama/msuluhishi.swa, zana/umbizaji.swa (#261). Zilizobaki: uzalishaji.swa (inatumia PIA A*/B*/D32 -- itahitaji uangalifu wa ziada) na stage1.swa/uzalishaji_wasm.swa (sasa zinawezekana pia, tangu WebAssembly -- #260 -- imeunganishwa). Pengo lililojulikana: ukaguzi wa "jina la aina halitumiki kama usemi" (`tenga N32` unakataliwa) haufanyi kazi kwa tahajia ya herufi ndogo (`tenga n32` hukubaliwa kimya) — kwa makusudi, kuepuka mgongano na vitambulisho vya ndani vya mkusanyaji (a1..a6, b1..b3) — angalia issue #255; vivyo hivyo `ukubwa(x)` kwenye mbegu (jina la muundo lililo HASA n8/n16/n32/n64/d64 linatafsiriwa kimya kama aina ya msingi — hati/mipaka.md #10) | PR #225, #256, #257; jaribio_tenga_jina_la_aina.swa na jaribio_herufi_ndogo_kubwa_muunganiko.swa (zimebaki herufi maalum kimakusudi); jaribio_mbegu_herufi_ndogo (haina alama ya stage1) |
| Lengwa ya WebAssembly (`stage1 --wasm`, kwa kivinjari) — Awamu ya 1 TU: N32 pekee (hesabu, ulinganisho, &&/\|\| za mzunguko mfupi, kama/sivyo, wakati/kwa/vunja/endelea, wito wa kazi ikiwemo kujirudia). HAKUNA: vielekezi, safu, D64/D32, miundo, vigezo vya ulimwengu, I/O, tenga(), wito_wa_mfumo, chagua — ujenzi wowote ulio nje ya wigo huu unatoa KOSA LA WAZI la kukusanya (msimbo wa kutoka != 0), si moduli mbovu kimya. mbegu.bin HAIHITAJI --wasm kabisa (ni kipengele cha stage1 pekee, sawa na muundo wa vipengele vingine vya hivi karibuni); node (WebAssembly.validate/.instantiate asili) ndiyo oracle pekee ya usahihi, hakuna wat2wasm/wasmtime | msingi/mkusanyaji/uzalishaji_wasm.swa; majaribio/wasm/MANIFEST.txt (24 majaribio, ikiwemo 8 hasi za nje-ya-wigo); gharama/jaribu-wasm.sh |
| Vipimo rasmi vya lugha | hati/vipimo-vya-lugha.md |

## Kilicho na kikomo (kilichoandikwa kwa ukali)

| Kikomo | Ukali | Hali |
|---|---|---|
| Dereva wa Rust/LLVM: haipo tena kwenye hazina hii — imehamishiwa lugha-swa/swa-dereva (iliyohifadhiwa 2026-09-05); mnyororo wa uzalishaji ni mbegu/exe pekee | HAKUNA | hati/mipaka.md #6 |
| Upeo wa tokeni 262,144 — inalia kwa sauti | CHINI | hati/mipaka.md #3 |
| Maneno halisi kwenye mbegu ni 32-bit signed (2147483648 hutoa -2147483648); mnyororo wa .swa umerekebishwa hadi N64 2026-08-25 — halisi >= 2^63 zinakatwa kimya, na halisi kubwa kama hoja ya wito zinavunjika (uthibitisho 2026-08-27) | CHINI | hati/mipaka.md #5 |
| Mpaka wa D64 na nambari kamili (kurudisha, ugawaji, operesheni mchanganyiko, ulinganisho mchanganyiko, upakiaji wa ulimwengu wa D64) — jibu baya kwa minyororo yote miwili; D32 imevunjika (poromoko) | JUU | hati/uthibitisho-wa-lugha.md (J1, J2, J7, J9) |
| Upana usio wa 8/16/32/64 (N128, D80, A128, n.k.) — unakubaliwa kimya kwa semantiki za uongo kwenye uzalishaji; mkusanyaji wa mbegu unaanguka | CHINI | hati/uthibitisho-wa-lugha.md (J16, J12) |

## Kinachokosekana kabla ya 1.0

- Majukwaa ya asilia zaidi ya x86-64 Linux
- Zana (LSP, debugger, formatter, package manager)
- Bomba la desimali — limefungwa 2026-08-20; kikomo cha mbegu kwenye
  wito wa D64 KIMEONDOKA (kilichopimwa 2026-08-27, mipaka.md 4c).
  Kilichobaki: mpaka wa D64 na nambari kamili (jibu baya) na D32
  (poromoko)

Kinachokamilika kwa 1.0 (2026-08): maktaba ya kawaida (kwa kiasi
kilichoorodheshwa hapo juu), vipimo rasmi vya lugha, na uamuzi wa
mwisho wa LLVM (MAJARIBIO — mnyororo wa uzalishaji ni mbegu/exe
pekee). Dereva wa Rust/LLVM umehamishiwa hazina ya kumbukumbu
lugha-swa/swa-dereva (2026-09-05).

## Uthibitisho wa jumla

Majaribio 428/428 kwenye mnyororo wa Swa pekee (mbegu na stage1).
Fixpoint: stage2-exe == stage3-exe sawa kwa baiti baada ya kila
mchanganyiko. Alama za nje za exe: SIFURI. (Majaribio 24/24 ya
lengwa ya WebAssembly ni mnyororo TOFAUTI — `gharama/jaribu-wasm.sh`
— hayajumuishwi kwenye hesabu hii ya juu.)

Kumbuka: fixpoint inathibitisha kujikusanya, si usahihi wa
semantiki. Uthibitisho kamili wa lugha (2026-08-27, kesi ~487
zilizokusanywa na kuendeshwa kwenye minyororo yote miwili)
umeandikwa kwenye `hati/uthibitisho-wa-lugha.md` — pamoja na
jibu baya zote kwa kipimo chake, poromoko, na ambapo minyororo
miwili inatofautiana.
