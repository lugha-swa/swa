# Hali ya Lugha ya Swa

Hati hii inaeleza hali halisi ya Swa kama LUGHA -- kila dai lina
msingi wa jaribio au ukweli uliothibitishwa kwenye mradi. Hakuna
lugha ya masoko: kilicho kamili kimeandikwa kamili, kilicho na
kikomo kimeandikwa na kikomo chake.

## Kilichokamilika (kila kimoja kimejaribiwa)

| Kipengele | Ushahidi |
|---|---|
| Kama/sivyo (if/else) | majaribio ya K-series, jaribio_kama_sivyo |
| Wakati (wakati) | jaribio_wakati, mfululizo mzima wa kujikusanya |
| Chagua (switch) -- kwenye uzalishaji PEKEE; awali inakataa kwa muundo (uthibitisho 2026-08-27); thamani hasi/N64 kubwa/usemi zinatoa jibu baya kwenye uzalishaji | jaribio_chagua_*; hati/uthibitisho-wa-lugha.md |
| Urejeshaji (kujirudia, wito wa mbele, pande mbili) | jaribio_mwito_wa_*, mkazo wa RELA (fibonacci=55) |
| kwa (for) -- KAMILI kwenye awali; `endelea` inaruka HATUA (semantiki ya C) kwenye mifululizo yote miwili | mkazo wa RELA (s=125), jaribio la CI; mipaka.md 4b |
| Miundo, kielekezi, &, nyoosha (sehemu, `->`, matokeo ya muundo) | jaribio_k13_*; kikomo kilichopimwa 2026-08-27: safu za miundo zinaanguka SEGV kwa zote mbili, muundo kwa thamani ni takataka kwenye awali na unapoteza baiti > 8 kwenye uzalishaji |
| JIT (--papo) NDANI ya programu | jaribio la CI: '; JIT: matokeo=42'; UND=0 kwenye programu |
| Kujikusanya (0% kujianzisha pengo) | kamili2 == kamili3 sawa kwa baiti; kwanza -> awali -> kamili |
| Uthibitisho wa nje wa RELA | GNU ld inakubaliana na zalisha_programu sawa kwa baiti |
| Ukaguzi wa chombo (awali) | hati/ukaguzi-chombo.md -- kila mpaka unalia kwa sauti |
| Mzunguko mfupi wa && na || KATIKA AWALI (sawa na uzalishaji.swa) | jaribio_awali_mzunguko_mfupi (SEGV ya zamani) |
| Usomaji wa stdin hadi EOF (bomba) -- matokeo yana uhakika | jaribio_awali_stdin_bomba_kubwa (mkato wa zamani) |
| Kazi isiyofafanuliwa inalia kwa sauti kwenye uzalishaji (si SEGV); awali inakataa kimya katika kesi kadhaa | jaribio_awali_kazi_kukosa; hati/uthibitisho-wa-lugha.md |
| Maktaba ya kawaida (hesabu, mifuatano, I/O, sort) -- kwa kiasi: badili na ukuaji wa Orodha zinaanguka (SEGV), ramani ni no-op kwenye awali, nambari_kwa_mfuatano_n64 imevunjika (uthibitisho 2026-08-27) | jaribio_maktaba_awali_exe; hati/uthibitisho-wa-lugha.md |
| Desimali (D64) -- hesabu, ulinganisho, ukanushaji NA WITO WA KAZI kwenye mifululizo yote miwili (awali inatumia ABI ya uhamisho wa GP; kilichopimwa 2026-08-27 -- mipaka.md 4c imerekebishwa). Kikomo: kila mpaka kati ya D64 na nambari kamili umevunjika (jibu baya); D32 imevunjika | jaribio_hitilafu_60_desimali_awali, jaribio_hitilafu_135_desimali; hati/uthibitisho-wa-lugha.md |
| Herufi ndogo za aina za msingi (n8/n16/n32/n64/a*/d32/d64/b*/w*) NDIYO PEKEE zinazokubalika sasa -- herufi KUBWA (N32, D64, W0 n.k.) ZIMEONDOLEWA KABISA (Awamu D, IMEKAMILIKA): chanzo chochote kinachotumia herufi kubwa ya aina hakikusanywi tena, kwenye awali WALA kwenye mkusanyaji uliojijenga (kamili/kamili2). awali (chanzo cha uaminifu) bado inatambua familia 6 TU (n8/n16/n32/n64/w0/d64 -- kugandisha kwa PILI ya chanzo cha uaminifu, 2026-09-26, iliyoondoa matawi ya herufi kubwa yaliyokuwepo, angalia CONTRIBUTING.md "Sheria ya Chanzo cha Uaminifu"); bado HAIJUI familia A/B wala D32 kwa hali yoyote ya herufi (hii haikubadilika -- familia hizo hazikuhitaji kuongezwa, kwa kuwa hakuna faili moja kwenye hazina inayozitumia kama msimbo halisi, maoni/nukuu za jedwali TU). Awamu B (#261-#271, faili 13/13 zinazojengwa moja kwa moja na awali.bin) na Awamu C (#272-278, faili zote zilizobaki za mradi) zilihamisha kila kitu kwenda herufi ndogo KWANZA (bila kuondoa uungwaji mkono wa herufi kubwa bado); Awamu D (#279) ndiyo iliyofuta uungwaji mkono wa herufi kubwa KABISA kwenye awali.swm (matawi 6) na msambazaji.swa (familia 5: N/A/D/B/W). jaribio_herufi_ndogo_kubwa_muunganiko.swa (lililokuwa likithibitisha N32/n32 ni aina moja) LIMEFUTWA na Awamu D -- lengo lake haliwezekani tena kisintaksia. jaribio_tenga_jina_la_aina.swa limebaki (bado linatumia `tenga N32` kimakusudi) kwa sababu linajaribu kazi TOFAUTI (ni_aina_msingi, ukaguzi wa "jina la aina kama usemi" -- angalia pengo hapa chini), isiyotegemea uwepo wa N32 kama aina halisi. Pengo lililojulikana (bado lipo, halijabadilishwa na Awamu D): ukaguzi wa "jina la aina halitumiki kama usemi" (`tenga N32` unakataliwa) haufanyi kazi kwa tahajia ya herufi ndogo (`tenga n32` hukubaliwa kimya) -- kwa makusudi, kuepuka mgongano na vitambulisho vya ndani vya mkusanyaji (a1..a6, b1..b3) -- angalia issue #255; vivyo hivyo `ukubwa(x)` kwenye awali (jina la muundo lililo HASA n8/n16/n32/n64/d64 linatafsiriwa kimya kama aina ya msingi -- hati/mipaka.md #10). Pengo JIPYA lililopatikana wakati wa Awamu D (halijarekebishwa, si zuiliwa na uondoaji wa herufi kubwa): jina lisilotambuliwa KABISA likitumika kama aina ya kurudi ya kazi ya ngazi ya juu (mfano `Kitu anza(){...}`) linakubaliwa kimya na changanua_aina_tokeni kama muundo wa mtumiaji asiyejulikana badala ya kukataliwa -- imethibitishwa kuwa tabia hii ilikuwepo KABLA ya Awamu D pia. | PR #225, #256-#278 (Awamu B/C), #279 (Awamu D -- uondoaji); jaribio_tenga_jina_la_aina.swa (kimebaki kimakusudi); jaribio_awali_herufi_ndogo (haina alama ya kamili) |
| Lengwa ya WebAssembly (`kamili --wasm`, kwa kivinjari) -- Awamu ya 1 TU: N32 pekee (hesabu, ulinganisho, &&/\|\| za mzunguko mfupi, kama/sivyo, wakati/kwa/acha/endelea, wito wa kazi ikiwemo kujirudia). HAKUNA: vielekezi, safu, D64/D32, miundo, vigezo vya ulimwengu, I/O, tenga(), wito_wa_mfumo, chagua -- ujenzi wowote ulio nje ya wigo huu unatoa KOSA LA WAZI la kukusanya (msimbo wa kutoka != 0), si moduli mbovu kimya. awali.bin HAIHITAJI --wasm kabisa (ni kipengele cha kamili pekee, sawa na muundo wa vipengele vingine vya hivi karibuni); node (WebAssembly.validate/.instantiate asili) ndiyo oracle pekee ya usahihi, hakuna wat2wasm/wasmtime | msingi/mkusanyaji/uzalishaji_wasm.swa; gharama/jaribu-wasm.mapishi (24 majaribio, ikiwemo 8 hasi za nje-ya-wigo); gharama/jaribu-wasm.mapishi |
| Vipimo rasmi vya lugha | hati/vipimo-vya-lugha.md |

## Kilicho na kikomo (kilichoandikwa kwa ukali)

| Kikomo | Ukali | Hali |
|---|---|---|
| Dereva wa Rust/LLVM: haipo tena kwenye hazina hii -- imehamishiwa lugha-swa/swa-dereva (iliyohifadhiwa 2026-09-05); mfululizo wa uzalishaji ni awali/exe pekee | HAKUNA | hati/mipaka.md #6 |
| Upeo wa tokeni 262,144 -- inalia kwa sauti | CHINI | hati/mipaka.md #3 |
| Maneno halisi kwenye awali ni 32-bit yenye ishara (2147483648 hutoa -2147483648); mfululizo wa .swa umerekebishwa hadi N64 2026-08-25 -- halisi >= 2^63 zinakatwa kimya, na halisi kubwa kama hoja ya wito zinavunjika (uthibitisho 2026-08-27) | CHINI | hati/mipaka.md #5 |
| Mpaka wa D64 na nambari kamili (kurudisha, ugawaji, operesheni mchanganyiko, ulinganisho mchanganyiko, upakiaji wa ulimwengu wa D64) -- jibu baya kwa mifululizo yote miwili; D32 imevunjika (poromoko) | JUU | hati/uthibitisho-wa-lugha.md (J1, J2, J7, J9) |
| Upana usio wa 8/16/32/64 (N128, D80, A128, n.k.) -- unakubaliwa kimya kwa semantiki za uongo kwenye uzalishaji; mkusanyaji wa awali unaanguka | CHINI | hati/uthibitisho-wa-lugha.md (J16, J12) |

## Kinachokosekana kabla ya 1.0

- Majukwaa ya asilia zaidi ya x86-64 Linux
- Zana (LSP, debugger, formatter, package manager)
- Bomba la desimali -- limefungwa 2026-08-20; kikomo cha awali kwenye
  wito wa D64 KIMEONDOKA (kilichopimwa 2026-08-27, mipaka.md 4c).
  Kilichobaki: mpaka wa D64 na nambari kamili (jibu baya) na D32
  (poromoko)

Kinachokamilika kwa 1.0 (2026-08): maktaba ya kawaida (kwa kiasi
kilichoorodheshwa hapo juu), vipimo rasmi vya lugha, na uamuzi wa
mwisho wa LLVM (MAJARIBIO -- mfululizo wa uzalishaji ni awali/exe
pekee). Dereva wa Rust/LLVM umehamishiwa hazina ya kumbukumbu
lugha-swa/swa-dereva (2026-09-05).

## Uthibitisho wa jumla

Majaribio 428/428 kwenye mfululizo wa Swa pekee (awali na kamili).
Uthabiti: kamili2 == kamili3 sawa kwa baiti baada ya kila
mchanganyiko. Alama za nje za exe: SIFURI. (Majaribio 24/24 ya
lengwa ya WebAssembly ni mfululizo TOFAUTI -- `gharama/jaribu-wasm.mapishi`
-- hayajumuishwi kwenye hesabu hii ya juu.)

Kumbuka: uthabiti inathibitisha kujikusanya, si usahihi wa
semantiki. Uthibitisho kamili wa lugha (2026-08-27, kesi ~487
zilizokusanywa na kuendeshwa kwenye mifululizo yote miwili)
umeandikwa kwenye `hati/uthibitisho-wa-lugha.md` -- pamoja na
jibu baya zote kwa kipimo chake, poromoko, na ambapo mifululizo
miwili inatofautiana.
