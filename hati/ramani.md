# Ramani ya Mradi / Project Roadmap

## Muhtasari

- **Keywords:** 13 za Kiswahili (aina za nambari hutambuliwa kisintaksia kwa herufi kubwa)
- **Aina:** familia 5 za nambari (N, A, D, B, W — upana halisi
  8/16/32/64; upana mwingine haujaungwa mkono, uthibitisho 2026-08-27)
- **Majaribio:** 304/304 mnyororo wa Swa (dereva wa Rust umehamia
  hazina ya kumbukumbu lugha-swa/swa-dereva, 2026-09-05)
- **Backend:** uzalishaji.swa (asilia x86-64, inajikusanya) ndiyo njia PEKEE; LLVM ilikuwa ya MAJARIBIO kwenye dereva wa Rust na sasa iko kwenye hazina ya kumbukumbu (mipaka.md 6)
- **Bootstrap:** kwanza (baiti za mkono) → mbegu → stage1-exe → stage2-exe == stage3-exe — sawa kwa baiti

## Hatua ya 0: Mkusanyaji wa Bootstrap wa Rust [PASS] IMEFANIKIWA

- [x] Lexer, parser, semantic analyzer
- [x] IR lowering (AST -> Swa IR)
- [x] LLVM codegen (x86-64 native binaries)
- [x] ABI classification (sret, struct returns)
- [x] Majaribio 265/265 mnyororo wa Swa + 232/232 Rust

## Hatua ya 1: Kujikusanya kwa Msingi [PASS] IMEFANIKIWA

- [x] Msomaji wa kujikusanya (`msomaji.swa`)
- [x] Mchanganuzi wa kujikusanya (`msambazaji.swa`)
- [x] Mkaguzi wa kisemantiki (`mkaguzi.swa`) — makosa 0 kwenye kujikusanya
- [x] Kizalishaji cha native x86-64 (`uzalishaji.swa`)
- [x] Binary inajikusanya (K6 inapita)
- [x] Vipengele vya lugha vinavyotumika: functions, loops (wakati/hali), if/else, structs, heap, unary minus, break/continue, short-circuit evaluation, assignment, bitwise ops, ternary
- [x] Safu za ndani, hoja za rafu (7+), disp32, sret, vigezo vya ulimwengu kwa RIP-relative

## Hatua ya 2: Mkusanyaji Kamili wa Kujikusanya [PASS] IMEFANIKIWA

### Mnyororo wa Kujikusanya wa Sasa

1. `kwanza.bin` (baiti za mkono, hex → binary) hutoa `mbegu.bin` kutoka `mbegu.hex`
2. `mbegu --exe` (syscalls pekee, hakuna kiunganishi) hukusanya `msingi/maktaba/*.swa na msingi/mkusanyaji/*.swa` → `stage1-exe`
3. `stage1-exe --exe` inajikusanya maktaba → `stage2-exe`
4. `stage2-exe --exe` inajikusanya → `stage3-exe`

**Uthibitisho:** stage2-exe == stage3-exe sawa kwa baiti, bila gcc/ld/clang/libc
popote kwenye mnyororo. Kumbuka: hili linahusu mnyororo wa uzalishaji pekee —
uthabiti wa mchanganuzi dhidi ya ingizo baya ni mhimili tofauti
(angalia hati/mipaka.md).

**JIT inafanya kazi (PR #143):** mmap → opcodes → rukia, na thamani ya
kurudi inarudi kwa usahihi. Stub ya `jmp main`, tungo mwishoni mwa bafa,
na daraja la C `tekeleza` kwa wito wa bafa (mbegu hana wito wa kielekezi).

**Uchunguzi muhimu wa usanifu:** `uzalishaji.swa` (mistari ~3,900) hukusanya
moja kwa moja kutoka AST, SI kutoka IR. `mteremko.swa` hutoa IR lakini
towe lake halitumiki katika mnyororo wa kujikusanya — IR inatumika TU
kwenye njia ya Rust → LLVM.

### Kipaumbele cha Juu (kilichobaki)
- [x] **Pengo la ABI la desimali** — IMEFUNGWA kwenye minyororo yote
      miwili (mbegu inatumia ABI ya uhamisho wa GP; kilichopimwa
      2026-08-27 — mipaka.md 4c imerekebishwa). Kilichobaki: mpaka
      wa D64 na nambari kamili (jibu baya) na D32 (poromoko)
- [x] **AST_BADILI (48)** — IMEFANYIKA: kishikizi kipo katika mkaguzi
      (mkaguzi.swa) na katika uzalishaji (badili kumbukumbu → realloc)
- [x] **Uthibitishaji wa aina za hali za `chagua`** — IMEFANYIKA: lebo za usemi au kigezo zinakataliwa kwa sauti; hasi na N64 zinalingana (T5)
- [x] **mteremko.swa** — IMEFANYIKA (2026-11-03): imejengwa upya kuwa IR ya
      mstari yenye aina (hati/ir.md), inatekelezwa na mtafsiri.swa
      (`swa --tafsiri`) na kukaguliwa dhidi ya msimbo wa mashine kwenye CI
      (programu 161 sawa kabisa). Ni msingi wa ARM64 na Mizani.
- [x] **JIT kamili** — IMEFANYIKA: relocations za wito wa nje ndani ya
      msimbo wa JIT (thunks za jmp + jedwali la anwani) na kupitisha
      argv (uzalishaji_jit inapokea argc/argv)

### Kipaumbele cha Kati
- [x] **Maktaba ya Kawaida**
  - [x] `orodha.swa` — orodha (inavyofanya kazi kwa uwezo uliotengwa
        mapema; UKUAJI umevunjika — orodha_ongeza zaidi ya uwezo
        inaanguka SEGV kwa minyororo yote miwili, uthibitisho
        2026-08-27)
  - [x] `mfuatano.swa` — shughuli za nyuzi kamili
  - [x] `ramani.swa` — jedwali la hashi (kwenye uzalishaji; kwenye
        mbegu weka ni no-op — jibu baya, uthibitisho 2026-08-27)
  - [x] `faili.swa` — shughuli za faili
  - [x] `hesabu.swa` — hesabu za ziada
  - [x] `kumbukumbu.swa` — usimamizi wa kumbukumbu
  - [x] `mpangilio.swa` — upangaji
  - [x] `nasibu.swa` — nambari nasibu
  - [x] `wakati.swa` — vipimo vya wakati

Kumbuka (uthibitisho 2026-08-27): moduli za `faili.swa`, `nasibu.swa`
na `wakati.swa` zinahitaji libc na hazifanyi kazi kwenye mnyororo
asilia (mbegu/exe) — uzalishaji unakataa kwa sauti, mbegu kimya.

## Hatua ya 3: Kuondoa Utegemezi wa Rust [PASS] IMEFANIKIWA

- [x] Mkusanyaji wa Swa unajikusanya **bila kutumia kande**
- [x] Bootstrap inafungwa: mbegu -> Swa -> Swa -> binary
- [x] Uthibitisho: stage2-exe == stage3-exe sawa kwa baiti
- [!] Rust `kande` inabaki kama chombo cha vipimo na ukuzaji (CI) tu —
      si sehemu ya mnyororo wa uzalishaji
      (2026-09-05: imehamishiwa lugha-swa/swa-dereva)

## Hatua ya 4: Kuondoa Utegemezi wa LLVM [PASS kwa mnyororo] IMEFANIKIWA

- [x] Native x86-64 backend (uzalishaji.swa) inazalisha binary bila LLVM
- [x] Mnyororo wa kujikusanya haugusi LLVM kabisa
- [x] Uthibitisho: Swa inajikusanya kupitia mnyororo kamili wa Swa -> Swa -> binary
- [!] LLVM inabaki ndani ya dereva wa Rust wa vipimo pekee
      (2026-09-05: imehamishiwa lugha-swa/swa-dereva)

## Hatua ya 5: Kuziba Pengo la Mwisho la Bootstrap [IMEFANYIKA]

- [x] **Baiti za mkono:** Kwanza (msingi/kwanza.bin) — baiti 393 zilizoandikwa
      kwa mkono (ELF 64 + phdr 56 + msimbo 273) hubadilisha hex hadi binary.
      Inajijenga (kwanza.hex -> kwanza.bin) na inazalisha mbegu.bin kutoka
      mbegu.hex — NASM si sehemu ya mnyororo wa uzalishaji tena.
      Uthibitisho: gharama/mnyororo.mapishi (pia kwenye CI).
- [x] **Kiunganishi cha kujitegemea:** bendera ya `--exe` inatoa ET_EXEC
      tuli (syscalls pekee, hakuna kichwa cha sehemu) — stage2-exe
      inajijenga yenyewe sawa kwa baiti (stage3-exe) bila ld/gcc/libc.
      Marejeo ya ndani yanatatuliwa na mkusanyaji mwenyewe (RELA-patch
      + jedwali la lebo); `_start` inaitwa moja kwa moja na kernel.
- [x] **Runtime ya syscalls:** mkusanyaji hautekelezi kupitia libc tena
      (PR #149) — faili, mmap, na uchapishaji kupitia syscalls.
- [x] **0% bootstrap gap:** mbegu inatoa ET_EXEC tuli moja kwa moja
      (`--exe`) — kichwa (64) + phdr (56) + stub ya `_start` (28) +
      urekebishaji wa RELA wa ndani (ulimwengu, tungo, na wito wa
      mbele) — hakuna gcc/ld/clang/muda.c/libc popote kwenye mnyororo.
      Uthibitisho: jaribio_exe_kujijenga (CI).
      Kumbuka: hili linahusu mnyororo wa uzalishaji pekee — uthabiti
      wa mchanganuzi dhidi ya ingizo baya ni mhimili tofauti, bado
      wazi (angalia hati/mipaka.md).

## Hatua ya 6: Lugha Kamili ya Mifumo [FUTURE] BAADAYE

- [ ] Maktaba ya kawaida kamili
- [ ] Mfumo wa moduli / vifurushi
- [ ] Zana za ujenzi (build system)
- [ ] Mazingira kamili ya uundaji

---

## Jinsi ya Kuchangia

Angalia [`CONTRIBUTING.md`](CONTRIBUTING.md). Masuala yenye lebo `good-first-issue` ni mahali pazuri pa kuanzia.

## Vipaumbele vya Sasa (Agosti 2026)

1. ~~Baiti za mkono~~ — IMESHAFANYIKA (PR #147): Kwanza (baiti 393)
   inazalisha mbegu.bin kutoka hex; NASM imeondolewa kwenye mnyororo
2. ~~Kiunganishi cha kujitegemea~~ — IMESHAFANYIKA: `--exe` inatoa ET_EXEC
   tuli inayojijenga yenyewe bila ld/gcc/libc (stage2-exe == stage3-exe)
3. ~~Runtime ya syscalls~~ — IMESHAFANYIKA (PR #149): syscalls moja kwa moja
4. ~~Kuziba hatua ya stage1~~ — IMESHAFANYIKA: mbegu inatoa stage1-exe
   moja kwa moja (`--exe`) — 0% bootstrap gap imefungwa
5. ~~Uthabiti wa makosa ya mchanganuzi~~ — IMESHAFANYIKA: mbegu na
   mchanganuzi wa .swa dhidi ya ingizo baya (hati/mipaka.md sehemu 1-2)
6. ~~JIT ndani ya exe~~ — IMESHAFANYIKA: tekeleza kama builtin,
   anwani_ya_kazi ya ndani (jedwali la anwani 0x400078+ofseti) —
   exe haina alama za nje kabisa
7. ~~ABI ya wito wa kazi za D64~~ — IMESHAFANYIKA kwenye minyororo
   yote miwili (kilichopimwa 2026-08-27 — mipaka.md 4c imerekebishwa).
   Kilichobaki: mpaka wa D64 na nambari kamili (jibu baya) na D32
   (poromoko)
8. ~~Dereva wa Rust~~ — IMESHAFANYIKA: desimali katika codegen ya LLVM
   (suala #135; jaribio_mende_135_desimali — mipaka.md 4c)
9. **Uamuzi wa mteremko.swa** — kuifuta au kuikamilisha

## Historia Fupi ya Milestone (Julai-Agosti 2026)

- PR #117: modulo, maoni ya bloku, radiksi, asimilia mchanganyiko — MERGED
- PR #140: kuondoa maneno muhimu ya bloat (na, au, si, tupu, kweli, uongo) — MERGED
- PR #131-133, #141: marejeo ya mbele, hifadhi/upakiaji wa aina, husisha nukuu,
  AST_KWELI/UONGO/TUPU — MERGED
- PR #142: mnyororo kamili wa kujikusanya na makosa 0 ya mkaguzi — MERGED
- PR #143: JIT — thamani ya kurudi, stub ya main, daraja la tekeleza — MERGED
