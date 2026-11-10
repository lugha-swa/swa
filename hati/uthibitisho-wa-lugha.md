# Uthibitisho wa Lugha ya Swa

Rekodi rasmi ya uzingatiaji wa vipimo -- kila matokeo hapa yalipimwa
kwa kukusanya na kuendesha programu halisi kwenye mifululizo yote
miwili ya mkusanyaji. Hakuna kitu kilichokisiwa.

KUMBUKA (2026-09-05): dereva wa Rust/LLVM (kande) umehamishiwa
hazina ya kumbukumbu lugha-swa/swa-dereva. Vipimo vilivyoandikwa
hapa kuhusu dereva (k.m. 10.5) ni rekodi ya kihistoria.

Tarehe ya kipimo: 2026-09-01.

Hali ya hazina wakati wa kipimo: tawi `kipengele/uthibitisho-kamili`,
HEAD 7399e97.

Mifululizo iliyopimwa:

- **mbegu** -- `msingi/awali.bin`, mkusanyaji wa bootstrap. Kipimo cha
  awali kilitumia gandisho f12e134 (2026-09-01 12:03); kundi la
  marekebisho f13416d (2026-09-02) lilifunga vipengee vya mbegu
  vilivyoorodheshwa katika sehemu ya 8 na 9, na mbegu iligandishwa
  tena. Gandisho la sasa la awali.bin: 415e692 (2026-09-04).
  Vipengee vya sehemu ya 8 na 9 vinarejelea mbegu ya sasa.
- **uzalishaji** -- stage1 iliyojengwa leo kutoka `msingi/maktaba/*.swa na msingi/mkusanyaji/*.swa` kupitia
  mbegu (`--programu`, chmod +x). Mfululizo wa kujikusanya: fixpoint
  stage2-exe == stage3-exe imepimwa leo, sawa kwa baiti.

Rekodi hii inachukua nafasi ya rekodi ya 2026-08-27 (hali ya
kabla ya ukarabati). Kesi 467 zilipimwa kwenye kila mfululizo kwenye
majaribio ya mwisho ya mfumo wa aina, udhibiti wa mwendo na
miundo/maktaba (2026-09-01, HEAD 1c525ce), na marekebisho ya
baada ya skani yalipimwa tena leo kwa kirejeshi kidogo cha kila
hitilafu -- tazama sehemu ya 3 na 12 kwa kile kilichopimwa upya.

## 1. Uainishaji

Kila kesi inaainishwa mojawapo ya nne:

- **SAWA** -- inakusanya, inaendesha, na jibu lake ni sahihi (msimbo
  wa kutoka na pato vinalingana na matarajio).
- **JIBU-BAYA** -- inakusanya na kuendesha bila kosa, lakini jibu si
  sahihi -- kimyakimya. Programu inaonekana imefanikiwa na inatoa
  jibu lingine.
- **HUANGUKA** -- inaanguka: wakati wa kukusanya (mkusanyaji mwenyewe)
  au wakati wa kuendesha (SEGV 139, FPE 136, n.k.).
- **IMEKATALIWA** -- inakataa kukusanya kwa msimbo wa kutoka usio
  sifuri. Mfululizo wa uzalishaji huandika ujumbe kwenye stderr;
  mbegu huandika kwenye stdout katika kesi chache, na katika kesi
  nyingine inakataa kimya (msimbo 1, hakuna ujumbe).

Jibu baya ni mbaya kuliko kukataliwa. Ukataaji ni sauti: mwandishi
anajua lugha haikubali msimbo wake na anaweza kuchukua njia nyingine.
Jibu baya ni kimya: mkusanyaji anakubali programu, anaizalisha kwa
muonekano kamili, na jibu lake ni potofu -- hitilafu inaingia kwenye
programu zilizojengwa bila ishara yoyote. Sehemu ya 8 inaorodhesha
kila jibu baya lililobaki, kwa kipimo cha chini, kilichoonekana,
na kinachotarajiwa.

Poromoko la mkusanyaji lenyewe kwenye chanzo halali linachukuliwa
kuwa mbaya zaidi kuliko poromoko la programu: mkusanyaji lazima
asikatee, asikatwe kimya, wala asianguke kwa ingizo lolote.

Msimbo wa kutoka wa programu hupimwa kwa mod 256 (baiti 8). SEGV ni
139, FPE ni 136. Ishara inathibitishwa kwa msimbo wa kutoka hasi wa
`subprocess`. Pale ambapo ilikuwa muhimu, tabia imethibitishwa kwa
uchunguzi wa mashine (od/objdump/ndisasm).

## 2. Mbinu ya kipimo

Mkusanyaji wa uzalishaji hujengwa kutoka chanzo cha .swa kupitia
mbegu (mfululizo wa kujikusanya):

```bash
cd /home/kandemark/Projects/compilers/swa
cat msingi/maktaba/{kumbukumbu,mfuatano}.swa msingi/mkusanyaji/{msomaji,msambazaji,mteremko,mkaguzi,uzalishaji}.swa msingi/maktaba/{orodha,ramani}.swa msingi/mkusanyaji/kiendeshaji.swa > /tmp/zima.swa
msingi/awali.bin --programu /tmp/zima.swa > /tmp/stage1.bin
chmod +x /tmp/stage1.bin
```

Kila kesi inakusanywa na kuendeshwa kwenye mifululizo yote miwili:

```bash
msingi/awali.bin --programu kesi.swa > matokeo.bin 2> makosa.txt
chmod +x matokeo.bin
./matokeo.bin; echo $?

/tmp/stage1.bin --programu kesi.swa > matokeo.bin 2> makosa.txt
chmod +x matokeo.bin
./matokeo.bin; echo $?
```

Kesi zinazotumia `andika` zinaunganishwa na `msingi/maktaba/kumbukumbu.swa`
(njia ya `cat`, kama hati 8). Kipimo kwenye mfululizo wa mbegu
kinatumia binary iliyogandishwa; kipimo kwenye mfululizo wa
uzalishaji kinatumia stage1 iliyojengwa siku ya kipimo kutoka
`msingi/maktaba/*.swa na msingi/mkusanyaji/*.swa` vya sasa.

Uthibitisho wa papo hapo ulifanyika leo kwa kesi za kirejeshi za
marekebisho yote yaliyoorodheshwa katika sehemu ya 3 na kwa kesi
za zilizobaki zilizoorodheshwa katika sehemu ya 8, 9 na 10 -- kila
moja ilikusanywa na kuendeshwa kwenye mifululizo yote miwili leo,
siku ya kuandika rekodi hii. Orodha kamili ya kesi zilizopimwa
upya iko katika sehemu ya 12.

## 3. Marekebisho ya baada ya skani

Majaribio ya mwisho (467 kesi) yalipimwa kwenye HEAD 1c525ce.
Baada ya hapo, hitilafu zifuatazo zilirekebishwa kwenye tawi hili;
kila kirejeshi kilipimwa tena leo na uthibitisho uliobainishwa:

1. **Mstari wowote unaoanza na "hu" ulifutwa kimya** (dereva
   alifuta mstari WOWOTE wenye kiambishi "hu", si mstari wa
   `husisha` pekee) -- kimefunga (commit 3bbe422). Imepimwa leo:
   `huduma = 6;` (bila kufanya nafasi) inasalimika kwenye mifululizo
   yote miwili.
2. **Muundo > baiti 8 kwa thamani haukunakiliwa** (ubadilishaji
   ndani ya kazi ulimwengu mwitaji), **sret kwa mfululizo ilitoa
   takataka** (`g(f())`), na **ufikiaji wa sehemu ya muundo wa
   ulimwengu ulianguka SEGV** -- vyote vimefunga (commit 1eb55b0).
   Imepimwa leo: kirejeshi cha a09 (rc=0 kwa zote mbili), kirejeshi
   cha J7 `rudisha g(f());` (rc=7 kwa zote mbili), na muundo wa
   ulimwengu (rc=12 kwenye uzalishaji). Kumbuka: mchanganyiko wa
   hoja kwa thamani NA sret pamoja (a19) BADO umevunjika -- tazama
   sehemu ya 8, kipengee 8.1.
3. **Mgawanyo bila ishara wenye biti ya juu ulianguka FPE** (A32,
   B32, W32, A64), **upanuzi wa sifuri kwa N8/N16 hasi** ulitoa
   jibu baya, na **upana usioahidiwa (N128, A128, D80) ulikubaliwa
   kimya** -- vyote vimefunga (commit 57d9694). Imepimwa leo:
   `A32 x = 3000000000; x / 2` inatoa 1500000000 (rc=1),
   `A64` za biti ya juu zote tatu (rc=1), `N8 x = -1; N32 y = x;`
   inatoa -1 (rc=1), na `N128 x = 5;` inakataliwa kwa sauti
   ("kosa: aina isiyojulikana: N128") kwenye uzalishaji.
4. **Kielekezi na N32 kama hoja kilikubaliwa kimya** (kisha SEGV
   wakati wa kukimbia), **maneno muhimu kama vitambulisho
   yalikubaliwa**, **wito wenye hoja 17+ ulitupa hoja za ziada
   kimya**, **mwili wa `sivyo` usio na mabano ulitekelezwa daima**,
   na **%u/%x za thamani hasi zilichapisha takataka** -- vyote
   vimefunga (commit b827c91, pamoja na ufuatiliaji wa halisi
   kwa kielekezi). Imepimwa leo: hoja za kielekezi/N32 zinakataliwa
   kwa sauti ("kielekezi dhidi ya namba"), maneno muhimu
   yanakataliwa ("'muundo' ni neno muhimu"), hoja 17 inakataliwa
   ("kikomo ni 16"), `kama (1) a = 1; sivyo a = 2;` inatoa a=1,
   na %u ya -1 inachapisha 18446744073709551615 kwenye uzalishaji.
5. **Mbegu: upanuzi wa ishara kwa matokeo ya hesabu ya jozi**
   (`N64 y = 0 - 90` ilitoa 0x00000000FFFFFFA6 badala ya kueneza
   ishara) -- kimefunga katika mbegu iliyogandishwa (commit
   f12e134). Imepimwa leo: `N64 x = 0 - 90; kama (x == -90)`
   inatoa rc=1 kwenye mbegu na uzalishaji.

Kesi za kirejeshi zilizopimwa upya leo, zote zikiwa na hali hiyo
iliyobainishwa: f01 (hu), f02 (muundo kwa thamani), f03 (sret
mfululizo), f04 (muundo wa ulimwengu), f05 + halisi_2p63_a64 +
halisi_2p64m1_a64 + halisi_2p64m1_a64_gawanya2 (mgawanyo bila
ishara), f06 (upanuzi wa ishara N8), f07 + uchunguzi_a128_ndani +
uchunguzi_d80_ndani + uchunguzi_n128_ndani + upana_d80 +
upana_n128 (upana usioahidiwa), f08/f09 + _v2/_v3 (hoja za
kielekezi na N32), f10/f11 (maneno muhimu), f12 (hoja 17), f13 +
_t141 (sivyo bila mabano), f14/f15 + t99 (umbizo %u/%x), f16
(ishara ya jozi ya mbegu), f17 (N32;), _w0_kigezo, _w0_thamani,
_d32_ulimwengu, _lit_kielekezi/_lit_kielekezi0 (halisi kwa
kielekezi), _a19/_a19b/_a19c/_a19d (mchanganyiko wa kwa-thamani
na sret), _t75, _t94, _b12, _n2/_n3 (utorokaji \n katika kamba),
_halisi63 (halisi 2^63-1), na kiunganishi cha dereva (kande).

## 4. Jedwali la Mfumo wa Aina (kesi 191 kwa kila mfululizo)

Safu "tarajio" ni ahadi ya `hati/vipimo-vya-lugha.md`; "hukumu" ni
kubaliana kwa mifululizo yote miwili na tarajio (SAWA), au tofauti
kubwa zaidi kati ya mifululizo/tarajio. Marekebisho ya sehemu ya 3
yamewekwa kwenye jedwali hili.

| Kipengele | Mbegu | Uzalishaji | Tarajio (hati) | Hukumu |
|---|---|---|---|---|
| N8: hesabu, ulimwengu, ndani | SAWA | SAWA | N8 ni nambari baiti 1 | SAWA |
| N16: hesabu, ulimwengu, ndani | SAWA | SAWA | N16 | SAWA |
| N32: hesabu, ulimwengu, ndani | SAWA | SAWA | N32 | SAWA |
| N64: hesabu, ulimwengu, ndani | SAWA | SAWA | N64 | SAWA |
| N64: `4294967296 % 4294967296`, `x / 4294967296` | SAWA | SAWA | mipaka 5: N64 kwenye mfululizo wa .swa | SAWA |
| A8/A16/A32/A64: hesabu, ulimwengu, ndani | IMEKATALIWA ("aina isiyojulikana 'A8'") | SAWA | A ni familia ya lugha | tofauti: mbegu haijui familia A (kikomo kilichoandikwa) |
| A32 mgawanyo wenye biti ya juu (`3000000000 / 2`) | IMEKATALIWA (A32) | SAWA (1500000000) | jibu sahihi | imefunga (zamani HUANGUKA FPE kwenye uzalishaji) |
| A64 mgawanyo wenye biti 63 (`2^63 / 2^32`, `2^64-1 / 2`) | IMEKATALIWA (A64) | SAWA | jibu sahihi | imefunga (zamani HUANGUKA FPE) |
| B8/B16/B32/B64: hesabu | IMEKATALIWA (B8) | SAWA | B ni familia | tofauti (kikomo cha mbegu) |
| W8/W16/W32/W64: hesabu | IMEKATALIWA (W8) | SAWA | W ni familia | tofauti (kikomo cha mbegu) |
| B1: tangazo, parameta, ulimwengu, hesabu | IMEKATALIWA (B1) | SAWA | B1 ni ya ndani | tofauti (kikomo cha mbegu) |
| B1 kutoka ulinganisho (`B1 b = (2 < 3)`) | IMEKATALIWA (B1) | IMEKATALIWA (kosa la aina: matokeo ya ulinganisho ni N32, si B1) | B1 ni matokeo ya ulinganisho | IMEKATALIWA kwa sauti kwa uzalishaji, lakini kinyume cha hati |
| B1 hadi N32 (`B1 b = 1; N32 y = b;`) | IMEKATALIWA (B1) | IMEKATALIWA (kosa la aina) | upanuzi | IMEKATALIWA kwa sauti |
| W0 kama kigezo chenye thamani (`W0 x = 5;`) | JIBU-BAYA (inakubaliwa kimya, rc=1) | JIBU-BAYA (inakubaliwa kimya, rc=1) | W0 ni kwa kazi tu | uvunjaji wa hati, mifululizo inakubaliana (tazama 8.3) |
| W0 katika usemi (`W0 x = 5; y = x + 1;`) | JIBU-BAYA (rc=6: inaihesabu kama thamani) | IMEKATALIWA (kosa la aina) | W0 ni kwa kazi tu | tofauti: uzalishaji unakataa kwa sauti, mbegu inahesabu kimya |
| N128: `N128 x = 5;` | IMEKATALIWA (sauti) | IMEKATALIWA (sauti: "aina isiyojulikana: N128") | upana usio wa 8/16/32/64 | imefunga (zamani JIBU-BAYA kimya kwenye uzalishaji) |
| A128: `A128 x = 5;` | IMEKATALIWA | IMEKATALIWA (sauti) | upana usio wa 8/16/32/64 | imefunga kwa upana; A128 kama kurudi kwenye N32 bado tofauti |
| D80: `D80 x = 2.5;` | IMEKATALIWA | IMEKATALIWA (sauti) | upana usio wa 8/16/32/64 | imefunga (zamani JIBU-BAYA kimya) |
| Halisi ndogo (`5`) | SAWA (5) | SAWA (5) | N32 | SAWA |
| Halisi 2^31-1 (`2147483647`) | SAWA | SAWA | N32 | SAWA |
| Halisi 2^31 (`2147483648`) | SAWA | SAWA | N64 | SAWA |
| Halisi 2^32 (`4294967296`) | SAWA | SAWA | N64 | SAWA |
| Halisi 2^63-1 (`9223372036854775807`) | SAWA | SAWA | N64 kamili | SAWA |
| Halisi 2^63 (`9223372036854775808`) | SAWA (mov rax, imm64; ishara = -2^63) | SAWA | N64 | SAWA (ukataji wa zamani haupo) |
| Halisi 2^64-1 (`18446744073709551615`) | SAWA (mov rax, 0xFFFFFFFFFFFFFFFF; = -1 kwa ishara) | SAWA | N64 | SAWA |
| Halisi 2^64 (`18446744073709551616`) | SAWA (wrap hadi 0, mov eax 0) | SAWA | nje ya N64 | SAWA kwa mod 2^64; ukataji wa kimya wa toleo la juu |
| Halisi hasi (`-5`) | SAWA (251; sio ishara -- imethibitishwa) | SAWA (251) | N32 hasi | SAWA |
| Halisi `-2147483648`, `-4294967296`, `-90` (N64) | SAWA | SAWA | N64 hasi | SAWA (rekodi ya zamani haishikiki) |
| `0 - 10000000000` (mchanganyiko N32-N64) | JIBU-BAYA (rc=2; hesabu 32-bit, `sub eax,ecx` kwenye mashine) | SAWA | -10000000000 | JIBU-BAYA kwenye mbegu (tazama 8.4) |
| `10000000000 - 20000000000` (N64-N64) | SAWA | SAWA | -10000000000 | SAWA |
| `4294967296 / 4294967296` (halisi / halisi) | SAWA (1) | SAWA (1) | 1 | SAWA (FPE ya zamani haipo) |
| Halisi kubwa kama hoja (`jumlisha(4294967296, 5)`) | SAWA | SAWA | hoja ya N64 | SAWA (ukataji wa zamani haupo) |
| Shift N64 (`1 << 32`, `4294967296 >> 1`) | JIBU-BAYA (shift 32-bit) | SAWA | mabadiliko ya 64-bit | JIBU-BAYA kwenye mbegu (tazama 8.5) |
| D64: hesabu, ulinganisho, ukanushaji | SAWA | SAWA | D64 | SAWA |
| D64 katika kazi (parameta + rudisha) | SAWA (ABI ya GP) | SAWA (ABI ya xmm) | D64 | SAWA |
| D64 ugawaji upya (`x = 0.0; x = 2.5;`) | SAWA | SAWA | 2.5 | SAWA (rekodi ya zamani haishikiki) |
| D64 ya ulimwengu (`D64 G = 2.5; G == 2.5`) | IMEKATALIWA (mchanganuzi haukubali) | SAWA | upakiaji wa ulimwengu wa D64 | tofauti: mbegu inakataa, uzalishaji umefunga |
| D64 hadi N32: kianzio (`N32 x = 2.5`) | JIBU-BAYA (0) | SAWA (2 -- cvttsd2si) | kukata hadi sifuri | JIBU-BAYA kwenye mbegu (tazama 8.6) |
| D64 hadi N32: ugawaji (`x = 2.5`) | JIBU-BAYA (0) | SAWA (2) | kukata | JIBU-BAYA kwenye mbegu |
| D64 hadi N32: hoja (`pata(2.5)`) | JIBU-BAYA (1) | SAWA (2) | kukata | JIBU-BAYA kwenye mbegu |
| D64 hadi N32: rudisha (`rudisha 21.5`) | JIBU-BAYA (0) | SAWA (21) | kukata | JIBU-BAYA kwenye mbegu |
| N32 hadi D64: kianzio, ugawaji, hoja, rudisha | JIBU-BAYA | SAWA | ubadilishaji | JIBU-BAYA kwenye mbegu |
| Mchanganyiko wa hesabu (`(1 + 2.5) * 2`, `5.5 * 2`) | JIBU-BAYA (6; 0) | SAWA (7.0; 11.0) | 7.0; 11.0 | JIBU-BAYA kwenye mbegu |
| Ulinganisho mchanganyiko (`2.5 < 3`, `3 == 3.0`, `1 > 2.5`, `0 == 2.5`) | JIBU-BAYA | SAWA | 1;1;0;0 | JIBU-BAYA kwenye mbegu |
| Ternary mchanganyiko (`1 ? 2.5 : 3` hadi D64/N32) | JIBU-BAYA | SAWA | 2.0; 2 | JIBU-BAYA kwenye mbegu |
| `D64 % 2`, `D64 << 1` | JIBU-BAYA (inakubaliwa kimya) | IMEKATALIWA (sauti: "operesheni haifanyi kazi kwa desimali") | inapaswa kukataliwa | tofauti: uzalishaji unakataa kwa sauti, mbegu inakubali kimya |
| `N32 G = 2.5;` (ulimwengu) | JIBU-BAYA (0) | IMEKATALIWA (sauti) | inapaswa kukataliwa au kukatwa | tofauti |
| D32: hesabu, ulinganisho, kazi | IMEKATALIWA (D32 isiyojulikana) | SAWA | D32 | tofauti (kikomo cha mbegu) |
| D32 ya ulimwengu (`D32 G = 2.5; G == 2.5`) | IMEKATALIWA (D32) | JIBU-BAYA (rc=2) | D32 | JIBU-BAYA kwenye uzalishaji (tazama 8.7) |
| D64 hadi D32, D32 hadi D64 | IMEKATALIWA (D32) | SAWA | ubadilishaji | uzalishaji SAWA |
| Halisi za radiksi (`0x10`) | IMEKATALIWA (sauti) | IMEKATALIWA (sauti) | 16 au kukataliwa kwa sauti | SAWA (maboresho dhidi ya rekodi) |
| N8 hadi N32 (chanya) | SAWA | SAWA | upanuzi | SAWA |
| N8/N16 hasi hadi N32 (`N8 x = -1; N32 y = x;`) | JIBU-BAYA (255/65535 -- movzx) | SAWA (-1, cdqe) | -1 (upanuzi wa ishara) | JIBU-BAYA kwenye mbegu (imefunga kwenye uzalishaji) |
| N8 overflow (`N8 x = 127; x = x + 1;` hadi N32) | JIBU-BAYA (128 -- movzx, sio -128) | SAWA (-128) | -128 | JIBU-BAYA kwenye mbegu (imefunga kwenye uzalishaji) |
| N32 hadi N64; N64 hadi N32 (mod); N32 hadi N8 (mod); N64 hadi N8 | SAWA | SAWA | ukataji kwa mod | SAWA |
| N32 hadi A32; A32 hadi N32 | IMEKATALIWA (A32) | IMEKATALIWA (kosa la aina la familia) | ubadilishaji wa familia | IMEKATALIWA kwa sauti kwa uzalishaji |
| A32 hadi A64 | IMEKATALIWA (A32) | SAWA | upanuzi wa familia | uzalishaji SAWA |
| Matokeo ya ulinganisho hadi N32/N64 (`N32 x = (2 == 2)`) | SAWA | SAWA | 1 | SAWA |
| `N32 x = 1 + (2 == 2);` | SAWA (2) | SAWA (2) | 2 | SAWA |
| Miundo: ndani, sehemu | SAWA | SAWA | muundo | SAWA |
| Miundo: ulimwengu (`Nukta G;`) | IMEKATALIWA (sauti: mchanganuzi) | SAWA (12) | muundo wa ulimwengu | imefunga kwenye uzalishaji (zamani HUANGUKA SEGV); mbegu bado inakataa |
| Muundo kwa thamani kama hoja (baiti 8 na 16) | SAWA | SAWA | muundo kwa thamani | SAWA (rekodi ya zamani haishikiki) |
| Muundo kurejeshwa (sret; baiti 8 na 16) | SAWA | SAWA | sret | SAWA |
| sret inayopitishwa moja kwa moja kwa kazi (`g(f())`) | SAWA (7; 33) | SAWA (7; 33) | 7; 33 | imefunga kwenye uzalishaji (zamani JIBU-BAYA) |
| Ugawi wa muundo (`b = a;`) | SAWA | SAWA | nakala | SAWA (rekodi ya zamani haishikiki) |
| Safu ya miundo (`s[i].x`) | SAWA | SAWA | aina halali | SAWA (rekodi ya zamani SEGV haishikiki) |
| Sehemu za N64 na D64 ndani ya muundo | SAWA | SAWA | muundo | SAWA |
| Ulimwengu wa N64 wenye thamani kubwa (`G = 4294967296`, `G = 2^63-1`) | SAWA | SAWA | mipaka 5 | SAWA (rekodi ya zamani FPE haishikiki) |

## 5. Jedwali la Udhibiti wa Mwendo na Viendeshaji (kesi 138 kwa kila mfululizo)

Kila kundi linalopita kwenye mifululizo yote miwili limefupishwa
katika mstari mmoja; kila kesi isiyo SAWA imeorodheshwa peke yake.
Marekebisho ya sehemu ya 3 yamewekwa.

| Kipengele | Mbegu | Uzalishaji | Kinachotarajiwa | Hali |
|---|---|---|---|---|
| kama/sivyo: rahisi, viota, mfululizo, mwili usio na mabano (taarifa moja), ukweli wa thamani, mwili tupu, tangazo ndani ya mwili (kesi 9) | SAWA | SAWA | kama hati | SAWA |
| kama/sivyo bila mabano pande zote mbili (`kama (1) a = 1; sivyo a = 2;`) | SAWA (a=1) | SAWA (a=1) | a=1 | imefunga kwenye uzalishaji (zamani JIBU-BAYA: sivyo daima) |
| mfululizo wa sivyo-kama bila mabano (t141) | SAWA (a=1) | SAWA (a=1) | a=1 | imefunga kwenye uzalishaji |
| wakati: rahisi, (0), acha, endelea, viota, && mzunguko mfupi, mwili bila mabano, mwili tupu (kesi 9) | SAWA | SAWA | kama hati | SAWA |
| kwa: rahisi, hali iliyoachwa, (;;), kianzilishi cha tangazo, endelea, acha, viota, mwili bila mabano, mwili tupu (kesi 11) | SAWA | SAWA | kama hati | SAWA |
| chagua: kila muundo (mikono, sivyo, viota, lebo 0/hasi/N64, selecta ya usemi, rudisha ndani, endelea ndani) (kesi 13) | IMEKATALIWA (mchanganuzi haujui chagua) | SAWA | hati 6.6 | tofauti (kikomo cha mbegu kilichoandikwa) |
| chagua: lebo ya usemi, lebo ya kigezo, bloku ndani ya mkono, acha nje ya kitanzi, sivyo pekee, hali nje ya chagua | IMEKATALIWA | IMEKATALIWA | kukataliwa kwa sauti | IMEKATALIWA kwa sauti (sawa) |
| hesabu za N32: + - * / % | SAWA | SAWA | 26 14 120 3 2 | SAWA |
| hesabu za N64 (> 2^31), mgawanyo na modulo | SAWA | SAWA | kama hati | SAWA |
| modulo na mgawo wa namba hasi (-7 % 3) | SAWA | SAWA | -1; -2 | SAWA (kwa ishara; fomu isiyo na ishara kwenye mbegu kwa %d -- tazama 8.10) |
| ulinganisho: == != < > <= >= | SAWA | SAWA | 0 1 1 0 1 0 | SAWA |
| && na || zenye mzunguko mfupi (ugawaji ndani, wito ndani) | SAWA | SAWA | kama hati | SAWA |
| bitwise: & | << >> ^ ~ | SAWA | SAWA | kama hati | SAWA |
| shift ya N64 (`1 << 32`) | SAWA (kwa matarajio ya mbegu: 1) | SAWA (4294967296) | 4294967296 (mipaka 5) | tofauti ya shift 32-bit ya mbegu (tazama 8.5) |
| ! (kanusha mantiki): !0 !5 !1 !(-1) | SAWA | SAWA | 1 0 0 0 | SAWA |
| ! kwenye D64 | JIBU-BAYA (rc=1, inakubali kimya) | IMEKATALIWA (sauti) | kukataliwa kwa sauti | tofauti: mbegu inakubali kimya (tazama 8.8) |
| viendeshaji vya kiwanja: += -= *= /= %= &= |= ^= <<= >>= (N32 na N64) | IMEKATALIWA | SAWA | si ya hati, lakini zinafanya kazi | tofauti (kikomo cha mbegu kilichoandikwa) |
| ternary rahisi, viota, tawi moja pekee linatathminiwa | SAWA | SAWA | kama hati | SAWA |
| toa unary: -x, -(-x), -x * 2, -(x * 2) | SAWA | SAWA | kama hati | SAWA |
| anwani & na kunyooosha *: *p = 9 | SAWA | SAWA | x=9 y=9 | SAWA |
| kielekezi mara mbili N32** (kunyooosha mara mbili) | HUANGUKA (SEGV 139) | SAWA (x=11) | x=11 | mbegu inaanguka (tazama 9.2) |
| p->sehemu, n.sehemu na a[i] | SAWA | SAWA | kama hati | SAWA |
| *(p + n): hesabu ya baiti (hati 4.2) | JIBU-BAYA (v=2816) | JIBU-BAYA (v=2816) | v=13 (kwa C) | hati 4.2 inasema hesabu ni ya BAITI -- jibu baya kwa mwandishi anayetegemea C (tazama 8.9) |
| ugawi kama usemi, mfululizo wa ugawi (a = b = 3) | SAWA | SAWA | kama hati | SAWA |
| ugawi wa ulinganisho/mantiki (a = 2 == 2, a = 0 \|\| 3) | SAWA | SAWA | 1; 1 | SAWA |
| utangulizi: hesabu, bitwise, && juu ya \|\|, shift juu ya <, ^ (kesi 8) | SAWA | SAWA | jedwali la hati 4 | SAWA |
| `1 < 2 << 1` | IMEKATALIWA | IMEKATALIWA | kukataliwa kwa sauti | IMEKATALIWA (sawa) |
| `a ? b : c = d` (upande wa uwongo haushiki ugawi) | IMEKATALIWA | IMEKATALIWA | kukataliwa kwa sauti | IMEKATALIWA (sawa) |
| maoni ya mstari // | SAWA | SAWA | kama hati | SAWA |
| maoni ya kizuizi /* */, viota, ndani ya usemi | IMEKATALIWA | SAWA | hati 2.4 | tofauti (kikomo cha mbegu kilichoandikwa) |
| kamba rahisi, utorokaji \n \\ \t \" \% | SAWA | JIBU-BAYA kwa \% pekee (t94: "asilimia \\n") | asilimia % | tazama 8.2 |
| %% katika muundo halisi | SAWA | SAWA | 100% sahihi | SAWA |
| %d (chanya, hasi, N64) katika muundo halisi | SAWA | SAWA | 42 -7 4294967300 | SAWA |
| %s, %c katika muundo halisi | SAWA | SAWA | kama hati | SAWA |
| %u %x %d katika muundo wa NGUVI (kigezo) | JIBU-BAYA (4294967295; 2a; 4294967254) | SAWA (18446744073709551615; 2a; -42) | 18446744073709551615; 2a; -42 | imefunga kwenye uzalishaji (zamani JIBU-BAYA "/"); mbegu bado inachapisha fomu ya biti 32 (tazama 8.10) |
| %d %s %c katika muundo wa nguvu | SAWA | SAWA | kama hati | SAWA |
| %f %l %n katika muundo wa nguvu (chapishwa kama halisi) | SAWA | SAWA | passthrough | SAWA |
| %f katika muundo halisi -- inakataliwa | SAWA (inachapisha %f) | IMEKATALIWA | kukataliwa kwa sauti | tofauti (uzalishaji mkali zaidi) |
| %u %x katika muundo halisi -- inakataliwa | JIBU-BAYA (inakubali kimya) | IMEKATALIWA (sauti) | kukataliwa kwa sauti | tofauti (tazama 8.11) |
| neno muhimu kama kitambulisho (muundo, kama, rudisha, acha) | JIBU-BAYA (inakubali kimya) | IMEKATALIWA (sauti: "'muundo' ni neno muhimu") | kukataliwa kwa sauti (hati 2.2) | imefunga kwenye uzalishaji; mbegu bado inakubali (tazama 8.12) |
| husisha { faili.swa }, husisha C::stdio, achilia | SAWA | SAWA | kama hati 8 | SAWA |
| W0 na rudisha; (bila thamani), rudisha mapema | SAWA | SAWA | kama hati | SAWA |
| kurudisha muundo kwa thamani | SAWA | SAWA | kama hati | SAWA |
| tangazo la kigezo bila kianzilishi | SAWA | SAWA | kama hati | SAWA |
| kazi yenye mwili tupu { } | SAWA | SAWA | kama hati | SAWA |
| ++, -- | IMEKATALIWA | IMEKATALIWA | si ya hati | IMEKATALIWA (sawa) |
| halisi za radiksi (0x10) | IMEKATALIWA | IMEKATALIWA | kukataliwa kwa sauti | SAWA |
| acha/endelea nje ya kitanzi | JIBU-BAYA (inapuuzwa kimya) | IMEKATALIWA (sauti) | kukataliwa kwa sauti | tofauti (tazama 8.13) |
| sivyo bila kama (pekee) | IMEKATALIWA | IMEKATALIWA | kukataliwa kwa sauti | IMEKATALIWA (sawa) |
| kama ya nusu: kama (1) kama (0) x=1; sivyo y=2 | SAWA | SAWA | x=0 y=2 | SAWA |
| sharti la kama ni ugawaji (kama (a = 5)) | SAWA | SAWA | a=5 r=1 | SAWA |
| shift kwa kiasi cha kigezo | SAWA | SAWA | 32 | SAWA |
| hesabu na ulinganisho za N8/N16 | SAWA | SAWA | kama hati | SAWA |
| mgawanyo/modulo kwa sifuri (uchunguzi) | HUANGUKA (FPE 136) | HUANGUKA (FPE 136) | kama C (hakuna ahadi) | HUANGUKA kwa zote mbili (tazama 9.1) |
| ulinganisho wa N64 kubwa (> 2^31) | SAWA | SAWA | kama hati | SAWA |

## 6. Jedwali la Miundo, Kumbukumbu na Maktaba (kesi 138 kwa kila mfululizo)

Marekebisho ya sehemu ya 3 yamewekwa.

| Kipengele | Mbegu | Uzalishaji | Kinachotarajiwa | Hali |
|---|---|---|---|---|
| Tangazo la muundo, sehemu kwa thamani `p.x`, `->`, upachikaji, `&p.x`, `&(q->x)`, nukta ya mwisho `};` | SAWA | SAWA | muundo | SAWA |
| Hoja kwa thamani, baiti 8/12/16/20/24 (usomaji) | SAWA | SAWA | kama hati | SAWA |
| Hoja kwa thamani, kubadilishwa ndani ya kazi (a09, baiti 12) | SAWA | SAWA (0) | chanzo kisibadilike | imefunga kwenye uzalishaji (zamani JIBU-BAYA) |
| Hoja kwa thamani + kurudisha sret pamoja (a19) | SAWA (0) | JIBU-BAYA (1; t.a=1541242656, takataka) | 0 | BADO JIBU-BAYA kwenye uzalishaji (tazama 8.1) |
| Muundo kurejeshwa kwa thamani (sret), baiti 8/12/16/20/24 | SAWA | SAWA | sret | SAWA |
| sret ndani ya hoja ya wito mwingine (`jumlisha(tengeneza(10))`) | HUANGUKA (SEGV 139) | SAWA (46) | 46 | mbegu inaanguka (tazama 9.2) |
| Safu za miundo (vipengele 12/16/20/24), anwani ya kipengele | SAWA | SAWA | kama hati | SAWA (rekodi ya zamani SEGV haishikiki) |
| Safu ya miundo kama hoja ya kazi | JIBU-BAYA (mwendo mbovu: 1563515763) | SAWA (20) | 20 | tofauti (kikomo cha mbegu) |
| Muundo kwenye tenga (heap), kielekezi | SAWA | SAWA | kama hati | SAWA |
| Ugawaji wa muundo `t = s` (nakala) | SAWA | SAWA | nakala | SAWA (rekodi ya zamani haishikiki) |
| `ukubwa()` kwa miundo ya upana mmoja | SAWA | SAWA | kama hati | SAWA |
| `ukubwa()` kwa muundo mchanganyiko (N8+N16+N32) | JIBU-BAYA (8; mpangilio 1-0-2-0-3) | SAWA (7; mpangilio 1-2-0-3-0) | 7 | tofauti: mpangilio wa sehemu ni tofauti kati ya mifululizo (hati haina ahadi) |
| Muundo wa ulimwengu (kigezo cha kimataifa) | IMEKATALIWA (sauti: mchanganuzi) | SAWA (12) | muundo wa ulimwengu | imefunga kwenye uzalishaji (zamani HUANGUKA); mbegu bado inakataa |
| Muundo uliowekwa kama hoja (`jumlisha(s.nd)`) | SAWA | SAWA | kama hati | SAWA |
| Muundo wenye padding N8/N64/N8 | SAWA | SAWA | kama hati | SAWA |
| Muundo kutumika KABLA ya tangazo lake | IMEKATALIWA | SAWA | kama hati | tofauti (kikomo cha mbegu) |
| Muundo wenye jina lenye umbo la aina (`B12`) | JIBU-BAYA (inakubali kama muundo, 18) | IMEKATALIWA (sauti: "aina isiyojulikana: B12") | kukataliwa au kufanya kazi | maboresho kwenye uzalishaji (zamani SEGV); mbegu inakubali kimya (tazama 8.14) |
| Anwani `&x`, kunyoosha `*p`, ugawaji kupitia `*p` | SAWA | SAWA | kama hati | SAWA |
| Faharisi hasi/chanya `p[-1]`, `p[1]` (kuzidisha kwa ukubwa) | SAWA | SAWA | kama hati | SAWA |
| Hesabu ya kielekezi `p + 2`, `p - 2`, `p - q` | JIBU-BAYA (hesabu ya baiti) | JIBU-BAYA (hesabu ya baiti) | hesabu ya baiti (hati 4.2) | inafuata hati 4.2; jibu baya kwa mwandishi anayetegemea C (tazama 8.9) |
| Kielekezi kwa kielekezi N32** | HUANGUKA (SEGV 139) | SAWA | kama hati | mbegu inaanguka (tazama 9.2) |
| Kielekezi cha N8, hesabu ya herufi | SAWA | SAWA | kama hati | SAWA |
| Null: `p == 0`, `p != 0`, kugawa 0 | SAWA | SAWA | kama hati | SAWA |
| Kunyoosha null (`*p = 5` na p=0) | SAWA (SEGV kama inavyotarajiwa) | SAWA (SEGV kama inavyotarajiwa) | anguko (hakuna ulinzi) | SAWA (kama C) |
| Kielekezi kwa N32 na N32 kwa kielekezi kama hoja (d30, d31) | JIBU-BAYA (inakubali kimya; N32 kwa kielekezi inaanguka SEGV wakati wa kukimbia) | IMEKATALIWA (sauti: "kielekezi dhidi ya namba") | kukataliwa (aina inakaguliwa) | imefunga kwenye uzalishaji; mbegu bado inakubali kimya (tazama 8.15) |
| Halisi kwa kigezo cha kielekezi (`kazi_p(5)`, `kazi_p(0)`) | HUANGUKA (SEGV 139 wakati wa kukimbia) | IMEKATALIWA (sauti) | kukataliwa | imefunga kwenye uzalishaji; mbegu inakubali na kuanguka (tazama 8.15) |
| Safu ya ndani, ulimwengu, kuoza kama hoja, safu ya N64 | SAWA | SAWA | kama hati | SAWA |
| Safu ya kamba (N8*[3]) | SAWA | SAWA | kama hati | SAWA |
| Faharisi nje ya mipaka (ndani ya ukurasa -- kimya; mbali -- SEGV) | SAWA (kama inavyotarajiwa) | SAWA (kama inavyotarajiwa) | hakuna ukaguzi unaoahidiwa | SAWA |
| Hoja 0-9 | SAWA | SAWA | kama hati | SAWA |
| Hoja 10-16 (mpaka wa rafu) | IMEKATALIWA (zaidi ya 9) | SAWA | kama hati | tofauti (kikomo cha mbegu) |
| Hoja 17 (ya 17 inatupwa kimya) | IMEKATALIWA (zaidi ya 9) | IMEKATALIWA (sauti: "kikomo ni 16") | kukataliwa | imefunga kwenye uzalishaji (zamani JIBU-BAYA) |
| Hoja 7 za N64 (rafu) | SAWA | SAWA | kama hati | SAWA |
| Wito wa mbele (kabla ya ufafanuzi) | SAWA | SAWA | kama hati | SAWA |
| Kujirudia (kipengele 6), kirefu (kina 40,000 na 100,000), pande mbili | SAWA | SAWA | kama hati | SAWA (rekodi ya zamani SEGV haishikiki) |
| W0: `rudisha;` tupu, wito wa W0 | SAWA | SAWA | kama hati | SAWA |
| Matokeo ya W0 yanatumika kama thamani | JIBU-BAYA (inakubali kimya, takataka) | IMEKATALIWA (sauti) | kukataliwa | tofauti (tazama 8.3) |
| Kurudisha N8/N16/N32/N64/D32/D64 | SAWA (D32 IMEKATALIWA) | SAWA | kama hati | SAWA (D32 kikomo cha mbegu) |
| Wito wa kazi za D64 (hoja + hesabu) | SAWA | SAWA | kama hati | SAWA (rekodi ya zamani haishikiki) |
| Hoja mchanganyiko kamili/desimali (d26) | JIBU-BAYA | SAWA | kama hati | tofauti (kikomo cha mbegu: mpaka wa D64) |
| Idadi ya hoja isiyo sahihi -- nafasi ya taarifa/usemi (d27-d29) | JIBU-BAYA (inakubali kimya) | SAWA (KATA) | kukataliwa | tofauti (tazama 8.16) |
| tenga msingi, nyingi, kubwa (500,000B), inayozidi arena inarudisha 0 | SAWA | SAWA | kama hati | SAWA |
| badili (kukua, data inahifadhiwa) | SAWA | SAWA | kama hati | SAWA (rekodi ya zamani SEGV haishikiki) |
| achilia (no-op, hakuna anguko) | SAWA | SAWA | kama hati | SAWA |
| wito_wa_mfumo hoja 1/2/3/6/8; rafu baada ya wito; O_CREAT mode 0644 | SAWA | SAWA | kama hati | SAWA |
| mfuatano: urefu, linganisha, nakili, unganisha, tafuta, kata_nafasi, geuza, juu/chini, heshi (moduli nzima) | SAWA (isipokuwa nambari_kwa_mfuatano_n64 hasi) | SAWA | kama hati | SAWA kwa uzalishaji; mbegu inavunjika kwa N64 hasi (tazama 8.17) |
| hesabu: kamili/ndogo/kubwa/dogo, neneo_n32, gcd, lcm, pow, isqrt, fibonacci, kipengele, kuu, pow_mod | SAWA | SAWA | kama hati | SAWA |
| neneo_n64 (kubwa hasi) | JIBU-BAYA (3589934592) | SAWA (5000000000) | 5000000000 | tofauti (kikomo cha mbegu: N64 hasi kubwa) |
| orodha: msingi, UKUAJI (uwezo 2 hadi 100, 1 hadi 128) | SAWA | SAWA | kama hati 10 | SAWA (rekodi ya zamani SEGV haishikiki) |
| mpangilio: pangilia_n32/n64 (+kushuka) | SAWA | SAWA | kama hati | SAWA |
| ramani: weka/pata/ina, mgongano, futa, kaburi, kujaza tena | SAWA | SAWA | kama hati 10 | SAWA (rekodi ya zamani "weka ni no-op" haishikiki) |
| faili: fungua "r"/"w"/"a"/"r+"/"w+"/"a+", soma/andika, yote, mstari, futa, ipo, ruhusa 0644 | SAWA | SAWA | kama hati | SAWA (faili.swa ni asilia -- syscalls pekee) |
| kumbukumbu: nakili, weka_sifuri, linganisha, andika_u64, andika_heksa, andika_sehemu, soma_mstari | SAWA | SAWA | kama hati | SAWA |
| andika: %d %s %c %% (halisi na kigezo) | SAWA | SAWA | kama hati | SAWA |
| andika: %u | SAWA (kwa chanya) | IMEKATALIWA (sauti kwenye muundo halisi) | si ya hati | uzalishaji mkali kwa muundo halisi |
| andika: %f (isiyosaidiwa) | SAWA (inachapisha "%f") | IMEKATALIWA (sauti) | passthrough | tofauti (uzalishaji mkali) |
| nasibu, wakati (zinahitaji libc) | IMEKATALIWA | IMEKATALIWA | zinahitaji libc | hazifanyi kazi kwenye mfululizo asilia (zimeandikwa) |
| Mstari wa chanzo unaoanza na "hu" (sio husisha) | SAWA (mstari unasalimika) | SAWA (mstari unasalimika) | mstari usifutwe | imefunga kwenye uzalishaji (zamani JIBU-BAYA, commit 3bbe422) |
| Utorokaji `\n` katika kamba ya DATA ya ndani | SAWA (0x0A) | JIBU-BAYA (5c 6e -- herufi halisi) | 0x0A (hati 2.3) | BADO JIBU-BAYA kwenye uzalishaji (tazama 8.18) |
| Utorokaji `\n` katika kamba ya DATA ya ULIMWENGU | IMEKATALIWA (kimya) | IMEKATALIWA (sauti: "kianzio cha ulimwengu lazima kiwe halisi ya nambari") | 0x0A | mabadiliko: sasa inakataliwa kwa sauti kwenye uzalishaji (zamani 5c 6e kimya) |

## 7. Kwa muhtasari: ni nini kimepita na ni nini kilichobaki

Kinachofanya kazi kwa mifululizo yote miwili, kilichopimwa leo:
kama/sivyo kwa kila muundo, wakati, kwa, acha na endelea, hesabu
na ulinganisho za N32 na N64, mzunguko mfupi wa && na ||, ternary,
ugawi, kurudia na wito wa mbele, miundo kwa thamani, kwa sret na
kwa mchanganyiko wao, safu za miundo, muundo wa ulimwengu
(kwenye uzalishaji), D64 ndani ya ulimwengu wake (pamoja na wito wa
kazi), familia A/B/W na D32 kwenye uzalishaji, tenga/badili/achilia,
orodha inayokua, ramani, faili, mfuatano na hesabu (kwa uzalishaji),
na maoni ya `//`.

Marekebisho ya ziada (kipindi cha 2026-09) yamefunga vipengee 21
vilivyokuwa vimeorodheshwa katika sehemu ya 8 na 9: 8.1 (commit
1d4648f) na 8.2/8.7/8.18 (commit b93b2b2, vipimo a9af87a) kwenye
uzalishaji, na vipengee vyote vya mbegu vikiwemo 9.2 (kundi
f13416d, 2026-09-02 -- mbegu imegandishwa upya baada yake).
JIBU-BAYA tatu zimebaki: 8.3 na 8.9 kwenye mifululizo yote miwili,
na 8.19 kwenye mbegu pekee; zote tatu zinarekebishwa sambamba na
kirejeshi cha kila moja kimeorodheshwa katika sehemu ya 8.
Mgawanyo kwa sifuri (9.1) unabaki kwenye zote mbili kama tabia
isiyofafanuliwa -- sawa na C -- si kasoro. Kila kilichofunga
kimeorodheshwa katika "zilizofunga", mwishoni mwa sehemu ya 8.

## 8. JIBU-BAYA zilizobaki (majibu potofu ya kimya)

Vipengee vitatu vimebaki: 8.3 na 8.9 kwenye mifululizo yote miwili,
na 8.19 kwenye mbegu pekee (uzalishaji unaikataa kwa sauti). Kila
kimoja kinarekebishwa sambamba; kirejeshi chake kitabaki hapa hadi
kipimo kithibitishe ufungaji wake. Vipengee vingine vyote vya
orodha ya awali -- pamoja na 9.2 -- vimefunga; orodha yao iko katika
"zilizofunga" mwishoni mwa sehemu hii.

Kila kipimo ni cha chini kabisa; "kilichoonekana" ni cha mifululizo
yote miwili isipokuwa ilivyobainishwa. Msimbo wa kutoka ni mod 256.
Mbegu: gandisho la sasa (415e692, 2026-09-04); uzalishaji: stage1
iliyojengwa kutoka chanzo cha sasa. Kila kirejeshi kilipimwa upya
kwenye mifululizo yote miwili katika kipindi cha 2026-09.

### 8.3 ZOTE MBILI -- parameta ya W0 inakubali thamani kimya (uvunjaji wa hati 3)

```swa
W0 f(W0 x) { } N32 anza() { f(5); rudisha 0; }
```

- Kilichoonekana (zote mbili): inakusanya na kuendesha kimya --
  hoja ya nambari 5 inakubaliwa kwenye parameta ya W0 bila kosa
  (rc=0). Kinachotarajiwa: kukataliwa kwa sauti.
- Hati 3 inafafanua W0 kama "bila thamani (void) -- kwa kazi tu";
  parameta ya W0 inayopokea thamani inakiuka ufafanuzi huo.

### 8.9 ZOTE MBILI -- *(p + n) ni hesabu ya BAITI (t75, b02, b06, c03)

```swa
N32 anza() { N32 a[4]; a[1] = 7; N32* p = a; rudisha *(p + 1); }
```

- Kilichoonekana (zote mbili): rc=0 -- `p + 1` husogea baiti moja,
  si kipengele kimoja, hivyo `*(p + 1)` husoma baiti 4 zilizo kwenye
  anwani p+1. Kinachotarajiwa: 7 (kipengele cha 1 -- kama `a[i]`
  inavyozidisha kwa ukubwa wa kipengele, na kama C).
- `a[i]` inafanya kazi kwenye mifululizo yote miwili; hesabu ya
  `p + n` ndiyo iliyovunjika -- jibu baya la kimya kwa mwandishi
  anayetegemea C.

### 8.19 MBEGU -- `N32;` (taarifa ya jina la aina pekee) inakubaliwa kimya

```swa
N32 anza() { N32; rudisha 0; }
```

- Kilichoonekana (mbegu): inakusanya na kuendesha kimya -- taarifa
  inatendewa kama no-op (rc=0). Kinachotarajiwa: kukataliwa kwa
  sauti.
- Uzalishaji: IMEKATALIWA kwa sauti ("kosa: jina la aina halitumiki
  kama usemi: N32"). Hii ndiyo mabaki pekee ambapo mbegu pekee
  ndiyo inayokubali kimya.

### Zilizofunga (kipindi cha 2026-09)

Vipengee vifuatavyo viliorodheshwa zamani kama mabaki katika
sehemu ya 8 na 9; vyote vimefunga na kuthibitishwa kwenye mifululizo
yote miwili. Nambari zao za zamani zimehifadhiwa kwa marejeleo ya
jedwali katika sehemu ya 4 hadi 6. Mikusanyiko ya marekebisho:
uzalishaji b93b2b2 (vipimo a9af87a) -- 2026-09-01; a19 kwa 1d4648f --
2026-09-01; kundi la mbegu f13416d -- 2026-09-02 (mbegu iligandishwa
upya baada yake).

**Uzalishaji:**

- 8.1 (a19) -- muundo kwa thamani + sret pamoja ulitoa takataka
  kwenye matokeo -- imefunga (1d4648f).
- 8.2 (t94) -- utorokaji \% katika muundo halisi ulikula asilimia --
  imefunga (b93b2b2).
- 8.7 (J8) -- D32 ya ulimwengu haikulingana kamwe na thamani yake --
  imefunga (b93b2b2).
- 8.18 -- \n katika kamba za DATA za ndani ulihifadhiwa kama herufi
  halisi (5c 6e) -- imefunga (b93b2b2).

**Mbegu (kundi f13416d, 2026-09-02):**

- 8.4 (J1) -- mchanganyiko N32-N64 ulihesabiwa kwa biti 32 -- imefunga.
- 8.5 (J2) -- shift za N64 zilikuwa za biti 32 -- imefunga.
- 8.6 (J3) -- mpaka mzima wa D64/nambari kamili (kianzio, ugawaji,
  hoja, rudisha, hesabu, ulinganisho, ternary) -- imefunga.
- 8.8 (t62) -- ! na ~ kwenye D64 zilikubaliwa kimya -- imefunga.
- 8.10 -- %d/%u/%x za matokeo ya hesabu hasi zilichapisha fomu ya
  biti 32 -- imefunga.
- 8.11 (t103, t104) -- %u na %x katika muundo halisi zilikubaliwa
  kimya -- imefunga.
- 8.12 (t105-t108) -- maneno muhimu kama vitambulisho yalikubaliwa
  kimya -- imefunga.
- 8.13 (t121, t122) -- acha na endelea nje ya kitanzi zilikubaliwa
  kimya -- imefunga.
- 8.14 (a28) -- muundo wenye jina lenye umbo la aina ulikubaliwa
  kimya -- imefunga.
- 8.15 (d30, d31) -- ukaguzi wa aina haukushika kielekezi na N32
  kama hoja -- imefunga.
- 8.16 (d27-d29) -- ukaguzi wa idadi ya hoja haukushika -- imefunga.
- 8.17 (f09, f18) -- N64 hasi kwenye maktaba ilikatwa hadi biti 32 --
  imefunga.
- 8.20 (J5) -- upanuzi wa sifuri wa N8/N16 hasi hadi N32 ulitoa
  255/65535 badala ya -1 -- imefunga.
- 8.21 (J4) -- D64 % 2 na D64 << 1 zilikubaliwa kimya -- imefunga.
- 8.22 -- safu ya miundo kama hoja ya kazi ilikuwa na mwendo mbovu --
  imefunga.
- 8.23 (a23) -- ukubwa na mpangilio wa muundo mchanganyiko ulikuwa
  tofauti kati ya mifululizo -- imefunga.
- 9.2 (t73, b03, a26) -- kielekezi kwa kielekezi N32** na sret
  ndani ya hoja ya wito mwingine zilianguka SEGV kwenye mbegu --
  zimefunga.

## 9. HUANGUKA na tabia isiyofafanuliwa

Hakuna HUANGUKA ya mkusanyaji iliyobaki kwenye mifululizo yote
miwili: vipengee vya 9.2 vimefunga katika kundi la mbegu f13416d,
na maboresho ya uthabiti ya awali yanasimama -- familia A/B/W,
upana usio wa kawaida, `sivyo` bila `kama`, maoni katikati ya
usemi, safu za miundo na ugawi wa muundo, ambavyo zamani vilikuwa
vinaangusha mkusanyaji, sasa vinakataliwa kwa sauti au vinafanya
kazi. Kilichobaki ni kigawanya-sifuri, ambacho si kasoro bali
tabia isiyofafanuliwa sawa na C:

### 9.1 ZOTE MBILI -- mgawanyo/modulo kwa sifuri (t136, t137) -- tabia isiyofafanuliwa, si kasoro

`a / b` na `a % b` kwa b = 0 zinaanguka kwa SIGFPE (ishara 8,
msimbo 136) kwenye mifululizo yote miwili. Hati haiahidi tabia
yoyote kwa kigawanya sifuri -- sawa na C (tabia isiyofafanuliwa,
hakuna ukaguzi unaoahidiwa). Kipimo cha uchunguzi pekee: si kasoro
ya mkusanyaji, hivyo hakijahesabiwa kati ya mabaki ya kasoro.

## 10. Tofauti kati ya mifululizo zilizobaki

Kwa lugha inayojijenga, tofauti kati ya mifululizo ni tatizo la
usahihi lenyewe: mkusanyaji unajijenga kwa chanzo kile kile, lakini
tabia ya lugha inabadilika kulingana na mfululizo uliotumika.

### 10.1 Hesabu za tofauti (baada ya marekebisho)

| Eneo | Tofauti zilizobaki | Chanzo |
|---|---|---|
| Mfumo wa aina (191) | 84 (zamani 86) | kikomo cha mbegu kilichoandikwa (79); uzalishaji mkali zaidi (4); uzalishaji jibu baya (1) -- tazama 10.2 |
| Udhibiti na viendeshaji (138) | 38 (zamani 37) | kikomo cha mbegu kilichoandikwa (26); mbegu inakubali kimya (9); mbegu HUANGUKA (1); uzalishaji jibu baya (1); uzalishaji mkali (1) -- tazama 10.3 |
| Miundo na maktaba (138) | 18 (zamani 17) | kikomo cha mbegu (12); mbegu HUANGUKA (2); mbegu inakubali kimya (2); uzalishaji jibu baya (1); uzalishaji mkali (1) -- tazama 10.4 |

### 10.2 Mfumo wa aina (84)

- **Mbegu inakataa kwa sauti, uzalishaji SAWA (48 kesi)** -- kikomo
  kilichoandikwa: familia A/B/W na B1/D32 hazijatekelezwa kwenye
  mbegu ("aina isiyojulikana"), D64 ya ulimwengu, muundo wa
  ulimwengu, upana usioahidiwa, halisi za radiksi. Hii ndiyo
  sehemu kubwa ya ukataaji wa mbegu (64 kati ya 191).
- **Mbegu JIBU-BAYA, uzalishaji SAWA (31 kesi)** -- kikomo
  kilichoandikwa: mpaka mzima wa D64/nambari kamili (8.6),
  mchanganyiko N32-N64 (8.4), shift N64 (8.5), na upanuzi wa
  sifuri N8/N16 (8.20 -- kesi 5 ambazo zilikuwa JIBU-BAYA kwa
  mifululizo yote miwili kwenye skani na baada ya 57d9694
  zilikuwa tofauti). Mbegu imegandishwa kabla ya marekebisho haya.
- **Mbegu JIBU-BAYA, uzalishaji IMEKATALIWA (4 kesi)** -- tofauti
  halisi: W0 katika usemi, `D64 % 2` / `D64 << 1` (8.21),
  `N32 G = 2.5;`. Mbegu inakubali kimya; uzalishaji unakataa kwa
  sauti (mwelekeo sahihi).
- **Mbegu IMEKATALIWA, uzalishaji JIBU-BAYA (1 kesi)** -- D32 ya
  ulimwengu (8.7): mbegu haijui D32, uzalishaji anakubali lakini
  jibu si sahihi. Hii ndiyo tofauti pekee iliyobaki ambapo mfululizo
  "mkubwa" ndio wenye jibu baya.

### 10.3 Udhibiti na viendeshaji (38)

- **Kikomo cha mbegu kilichoandikwa (26 kesi)** -- `chagua` nzima
  (17 kesi, mchanganuzi haujui neno), maoni ya kizuizi (3),
  viendeshaji vya kiwanja (6).
- **Mbegu inakubali kimya, uzalishaji unakataa kwa sauti (9 kesi)**
  -- !/~ kwenye D64 (t62), %u/%x katika muundo halisi (t103/t104),
  acha/endelea nje ya kitanzi (t121/t122), maneno muhimu kama
  vitambulisho (t105-t108). Mwelekeo wa uzalishaji ndio sahihi.
- **Mbegu HUANGUKA, uzalishaji SAWA (1 kesi)** -- N32** (9.2).
- **Mbegu SAWA, uzalishaji JIBU-BAYA (1 kesi)** -- utorokaji \%
  (8.2): hitilafu iko kwenye uzalishaji.
- **Mbegu SAWA, uzalishaji IMEKATALIWA (1 kesi)** -- %f katika
  muundo halisi: mbegu inachapisha "%f" kama herufi halisi,
  uzalishaji unakataa kwa sauti (uamuzi ulioandikwa: viungwa
  %d %s %c %% pekee).

### 10.4 Miundo na maktaba (18)

- **Kikomo cha mbegu kilichoandikwa (12 kesi)** -- ukubwa/mpangilio
  wa muundo mchanganyiko (a23, 8.23), hesabu ya baiti ya kielekezi
  (b02/b06/c03 -- inalingana na hati 4.2), hoja mchanganyiko
  kamili/desimali (d26), ukaguzi wa idadi ya hoja (d27-d29,
  8.16), N64 hasi kwenye maktaba (f09/f18, 8.17), safu ya miundo
  kama hoja (8.22), muundo kutumika kabla ya tangazo, D32, muundo
  wa ulimwengu (uzalishaji umefunga; mbegu inakataa), na muundo
  wenye jina la aina (a28, 8.14).
- **Mbegu HUANGUKA, uzalishaji SAWA (2 kesi)** -- N32** (b03),
  sret ndani ya hoja (a26) -- 9.2.
- **Mbegu inakubali kimya, uzalishaji unakataa kwa sauti (2 kesi)**
  -- kielekezi na N32 kama hoja (d30/d31, 8.15): mwelekeo wa
  uzalishaji ndio sahihi.
- **Mbegu SAWA, uzalishaji JIBU-BAYA (1 kesi)** -- hoja kwa thamani
  + sret pamoja (a19, 8.1): hitilafu iko kwenye uzalishaji.
- **Mbegu SAWA, uzalishaji IMEKATALIWA (1 kesi)** -- %f kwenye
  andika (passthrough ya mbegu dhidi ya kata ya uzalishaji).

### 10.5 Dereva wa Rust (kande) -- kiunganishi kinashindwa kwenye mashine hii

Dereva wa LLVM (`kande` kutoka `src/`) hukusanya kwa usahihi hadi
faili la kitu (.o), lakini hatua ya kiunganishi inashindwa kwenye
binutils za mashine hii: `clang -Wl,--defsym,andika=printf` inarudisha
"unresolvable symbol `printf` referenced in expression" (kiunganishi
cha kisasa hakiwezi kutatua printf kwenye usemi wa --defsym kabla
ya libc kuunganishwa), kisha "final link failed". Imepimwa leo:
`kande rahisi.swa -o rahisi.exe` inashindwa; `rahisi.o` inahifadhiwa.
Mfululizo wa kujikusanya (mbegu --programu) hauhusiki na kiunganishi
hiki -- unatoa ELF moja kwa moja.

## 11. Ahadi za hati zisizoshikika (zilizobaki)

1. **Hati 3 -- "A, B, W ni familia za lugha"**: mbegu ya sasa haijui
   A/B/W/D32/B1 kabisa ("Hitilafu: aina isiyojulikana 'X'").
   Mfululizo wa uzalishaji unazijua. Kikomo kilichoandikwa kwenye
   hati za mbegu; familia zilizoahidiwa hazijatekelezwa kwenye
   mbegu.
2. **Hati 3 -- "B1 -- matokeo ya ulinganisho na mantiki"**: kwenye
   uzalishaji, `B1 b = (2 < 3)` inakataliwa kwa kosa la aina
   (matokeo ya ulinganisho yanaainishwa kama N32, si B1).
   Kukataliwa ni kwa sauti (si jibu baya), lakini kinyume cha hati.
3. **Hati 3 -- "W0 ni kwa kazi tu"**: mifululizo yote miwili
   inakubali `W0 x = 5;` kimya (8.3).
4. **Hati 2.3 -- "utorokaji \n ... zinafanya kazi kwenye mifululizo
   yote miwili"**: kwenye kamba za DATA za ndani, uzalishaji
   huhifadhi `\n` kama herufi halisi (5c 6e) -- 8.18. (Kwa kamba
   za data za ulimwengu sasa unakataa kwa sauti.)
5. **Hati 2.3 -- "Desimali inafanya kazi kwenye mifululizo yote"**:
   kweli kwenye uzalishaji; mpaka mzima wa D64/nambari kamili
   bado umevunjika kwenye mbegu (8.6).
6. **Uthibitisho #25 -- "%u ya hasi = mfuatano kamili wa bits"**:
   uzalishaji umeufunga (18446744073709551615); mbegu inachapisha
   fomu ya biti 32 (4294967295) kwa matokeo yaliyokokotwa -- 8.10.
7. **Hati 4.2 -- hesabu ya kielekezi ni ya baiti**: inashikilia kwa
   mifululizo yote miwili (8.9), lakini tofauti ya `p - q`
   (delta ghafi ya baiti) haijaelezwa kwenye hati.
8. **Hati 2.2 -- "maneno muhimu hayawezi kutumika kama majina"**:
   imefunga kwenye uzalishaji; mbegu bado inakubali (8.12).
9. **Hati 6.6 -- "chagua"**: inafanya kazi kwenye uzalishaji;
   mbegu inakataa kwa muundo (kikomo kilichoandikwa).
10. **Hati 8 -- "mkusanyaji wa .swa hulichakata faili lililotajwa"**:
    hakuna mfululizo unaosoma faili la `husisha { }`; kiungo
    kinatoshelezwa kwa cat pekee (maboresho: mstari wowote wenye
    "hu" haufutwi tena).
11. **Hati 9 -- `tekeleza`, `anwani_ya_kazi`**: hazipo kwenye
    mifululizo yote miwili; `wito_wa_mfumo` inafanya kazi.
12. **Hati 10 -- "Orodha -- safu inayokua", "Ramani"**: zimefunga
    kwenye mifululizo yote miwili (rekodi ya zamani haishikiki).
13. **Hati 10 -- nasibu/wakati**: zinahitaji libc; hazifanyi kazi
    kwenye mfululizo asilia (kukataliwa kwa sauti kwenye uzalishaji).
14. **Hati 7 -- "KUPITISHWA kwa thamani"**: miundo > baiti 8 kwa
    hoja (a09), sret kwa mfululizo (J7) na muundo wa ulimwengu
    zimefunga; mpaka uliobaki ni hoja kwa thamani + sret pamoja
    (a19, 8.1) -- sehemu ya hati haijafunikwa na hitilafu hii.

## 12. Hesabu za mwisho

Kesi 467 kwa kila mfululizo (191 mfumo wa aina + 138 udhibiti +
138 miundo/maktaba). Hesabu zilizoonyeshwa ni BAADA ya marekebisho
ya sehemu ya 3 kwa vitu vilivyoorodheshwa katika sehemu hiyo.

### 12.1 Mfumo wa aina (191)

| Mfululizo | SAWA | JIBU-BAYA | HUANGUKA | IMEKATALIWA | Jumla |
|---|---|---|---|---|---|
| **Mbegu** | 91 | 36 | 0 | 64 | 191 |
| **Uzalishaji** | 170 | 2 | 0 | 19 | 191 |

Marekebisho yaliyoathiri hesabu hizi (aina za kesi zilizopimwa
tena leo): sret kwa mfululizo (2 kesi: JIBU-BAYA inakuwa SAWA),
upanuzi wa ishara N8/N16 (5 kesi: JIBU-BAYA inakuwa SAWA),
mgawanyo bila ishara (6 kesi: HUANGUKA inakuwa SAWA), muundo wa
ulimwengu (2 kesi: HUANGUKA inakuwa SAWA), upana usioahidiwa
(5 kesi: JIBU-BAYA inakuwa IMEKATALIWA).

### 12.2 Udhibiti na viendeshaji (138)

| Mfululizo | SAWA | JIBU-BAYA | HUANGUKA | IMEKATALIWA | Jumla |
|---|---|---|---|---|---|
| **Mbegu** | 88 | 10 | 3 | 37 | 138 |
| **Uzalishaji** | 113 | 2 | 2 | 21 | 138 |

Marekebisho yaliyoathiri hesabu hizi: kama/sivyo bila mabano (2
kesi: JIBU-BAYA inakuwa SAWA), %u hasi kwenye muundo wa nguvu
(1 kesi: JIBU-BAYA inakuwa SAWA), maneno muhimu kama vitambulisho
(4 kesi: JIBU-BAYA inakuwa IMEKATALIWA).

### 12.3 Miundo, kumbukumbu na maktaba (138)

| Mfululizo | SAWA | JIBU-BAYA | HUANGUKA | IMEKATALIWA | Jumla |
|---|---|---|---|---|---|
| **Mbegu** | 117 | 13 | 2 | 6 | 138 |
| **Uzalishaji** | 129 | 4 | 0 | 5 | 138 |

Marekebisho yaliyoathiri hesabu hizi: muundo kwa thamani > baiti 8
(1 kesi: JIBU-BAYA inakuwa SAWA), muundo wa ulimwengu (1 kesi:
HUANGUKA inakuwa SAWA), kielekezi na N32 kama hoja (2 kesi:
JIBU-BAYA inakuwa IMEKATALIWA).
Muundo wenye jina la aina (a28) limebadilika kutoka anguko hadi
kukataliwa kwa sauti kwenye uzalishaji -- lilihesabiwa kama SAWA
(kulikuwa "anguko kama ilivyotarajiwa" katika kipimo cha skani)
na linabaki kama hivyo, kwa maelezo ya 8.14.

### 12.4 Jumla (467)

| Mfululizo | SAWA | JIBU-BAYA | HUANGUKA | IMEKATALIWA | Jumla |
|---|---|---|---|---|---|
| **Mbegu** | 296 | 59 | 5 | 107 | 467 |
| **Uzalishaji** | 412 | 8 | 2 | 45 | 467 |

### 12.5 Ufafanuzi wa uaminifu wa hesabu

- Kwa kila eneo, kesi za kirejeshi za marekebisho pekee ndizo
  zilipimwa tena leo (orodha kamili katika sehemu ya 3) -- SI
  skani nzima ya 467. Hesabu za msingi (zilizopimwa HEAD 1c525ce)
  zimechukuliwa kutoka rekodi za skani za siku hiyo
  (/tmp/swa-t6-mwisho/aina, /udhibiti, /miundo) na marekebisho
  yamewekwa kwenye kesi za kirejeshi zilizoorodheshwa hapo juu.
- Kesi za zilizobaki zilizopimwa tena leo na kuthibitishwa:
  halisi_ukanusha_n64 na halisi_hesabu_n64_2 (J1: rc=2 kwenye
  mbegu), t94 (\%: jibu baya kwenye uzalishaji), t75 na
  _t141 (_t141 imefunga), _b12 (uzalishaji unakataa kwa sauti),
  _w0_kigezo (zote mbili kimya), _w0_thamani (mbegu kimya,
  uzalishaji kukataa), _d32_ulimwengu (uzalishaji rc=2),
  _lit_kielekezi/_lit_kielekezi0 (uzalishaji kukataa, mbegu SEGV),
  _a19/_a19b/_a19c/_a19d (8.1: bado jibu baya kwenye uzalishaji),
  _n2/_n3 (8.18), _halisi63 (SAWA zote mbili), na kiunganishi cha
  kande (10.5: kinashindwa).
- Mbegu haijabadilika tangu skani (commit f12e134 ndio gandisho
  la mwisho la `msingi/awali.bin`); hesabu zake zinasimama kama
  zilivyopimwa. Tofauti moja ya mbinu: kesi t99 (muundo wa nguvu
  %u) katika skani ya udhibiti ililinganishwa dhidi ya pato la
  mbegu lenyewe (fomu ya biti 32) na kuhesabiwa SAWA; kipimo cha
  leo cha moja kwa moja dhidi ya ahadi #25 kinaonyesha mbegu
  inachapisha 4294967295 -- tazama 8.10. Hii haibadilishi hesabu
  ya mbegu (kesi hiyo ilihesabiwa SAWA katika skani na imebaki
  hivyo), lakini inabainishwa kwa uaminifu.
- Fixpoint stage2-exe == stage3-exe imepimwa leo, sawa kwa baiti.
  Stage1-exe ni tofauti na stage2: mbegu imeoka makosa yake (8.4
  hadi 8.8, 8.12 hadi 8.17) ndani ya stage1 iliyojengwa kupitia
  yeye; stage2 inayojijenga ndiyo ushahidi wa fixpoint.

## 13. Hitimisho

**Fixpoint inathibitisha nini**: mfululizo wa kujikusanya unafanya
kazi kikamilifu -- kutoka kwanza hadi mbegu hadi stage1 hadi
stage2 == stage3 (sawa kwa baiti), bila lugha nyingine popote.
Mkusanyaji unaweza kujijenga yenyewe kwa kudumu.

**Fixpoint haithibitishi nini**: usahihi wa semantiki. Mkusanyaji
anaweza kujijenga yenyewe na bado kuwa na makosa kwenye mipaka ya
lugha -- ushahidi: hitilafu ya a19 (8.1) iko kwenye mfululizo wa
uzalishaji uliojijenga, na mbegu iliyogandishwa ina makosa yake
(8.4 hadi 8.8, 8.12 hadi 8.17) ambayo stage1 inayojijenga
haijarithishi. Fixpoint inathibitisha uthabiti wa kujijenga, si
usahihi wa tabia.

**Kanuni inayoongoza mradi huu**: programu inakusanywa kwa usahihi
au inakataliwa kwa sauti; jibu baya la kimya halikubaliki kamwe.
Ukataji wa sauti unagharimu muda wa kukusanya; jibu baya
linaweza kugharimu data au programu zilizotumwa. Mwelekeo wa
majaribio ya mwisho unathibitisha kanuni hii: kila jibu baya la
uzalishaji lililotambuliwa limefungwa kwa ukataji wa sauti au
jibu sahihi (sehemu ya 3), na kilichobaki (8 jibu baya na 2
huanguka kati ya 467 kwenye uzalishaji; 59 jibu baya na 5
huanguka kwenye mbegu iliyogandishwa) kimeorodheshwa hapa kwa
kirejeshi kidogo, kilichoonekana, na kinachotarajiwa -- ili
kila hitilafu iweze kuthibitishwa na kufungwa. Mbegu imegandishwa;
kazi ya sasa na ya baadaye iko kwenye mfululizo wa uzalishaji,
ambapo ukali wa mkaguzi (kukataa kwa sauti badala ya kuhesabu
kwa bahati) ndio mwamba wa mfumo mzima.
