# Ramani ya Mradi / Project Roadmap

## Muhtasari

- **Keywords:** 42 za Kiswahili (hakuna Kiingereza katika sintaksia)
- **Aina:** 25 za nambari (N8-N128, A8-A128, D16-D80, B1-B64, W0-W64)
- **Majaribio:** 67/67 ya ujumuishaji + 146 za kitengo (Rust), vipimo 5/5 vya mbegu, JIT 5/5
- **Backend:** uzalishaji.swa (native x86-64, inajikusanya); LLVM inabaki kwenye dereva wa Rust kwa vipimo pekee
- **Bootstrap:** mbegu (NASM) → stage1 → stage2 → stage3 → stage4 — sawa kwa baiti

## Hatua ya 0: Mkusanyaji wa Bootstrap wa Rust [PASS] IMEFANIKIWA

- [x] Lexer, parser, semantic analyzer
- [x] IR lowering (AST -> Swa IR)
- [x] LLVM codegen (x86-64 native binaries)
- [x] ABI classification (sret, struct returns)
- [x] Majaribio 67/67 ya ujumuishaji + 146 za kitengo

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

1. `mbegu.bin` (NASM, syscalls pekee) husoma `msingi/*.swa` na kutoa `stage1.o`
2. `stage1.bin` (codegen ya mbegu) inajikusanya maktaba → `stage2.o`
3. `stage2.bin` (codegen ya Swa) inajikusanya maktaba → `stage3.o`
4. `stage3.bin` inajikusanya → `stage4.o`

**Uthibitisho:** stage2.o == stage3.o == stage4.o sawa kwa baiti (209KB),
s3.err na s4.err tupu, vipimo 5/5 (ts_tupu→0, ts_le_uongo→0,
ts_le_kweli→7, ts_lt_kweli→5, ts_argc→2).

**JIT inafanya kazi (PR #143):** mmap → opcodes → rukia, na thamani ya
kurudi inarudi kwa usahihi. Stub ya `jmp main`, tungo mwishoni mwa bafa,
na daraja la C `tekeleza` kwa wito wa bafa (mbegu hana wito wa kielekezi).

**Uchunguzi muhimu wa usanifu:** `uzalishaji.swa` (mistari ~3,900) hukusanya
moja kwa moja kutoka AST, SI kutoka IR. `mteremko.swa` hutoa IR lakini
towe lake halitumiki katika mnyororo wa kujikusanya — IR inatumika TU
kwenye njia ya Rust → LLVM.

### Kipaumbele cha Juu (kilichobaki)
- [ ] **Pengo la ABI la desimali:** hoja za desimali bado hupitishwa kwenye
      rejesta kamili (rdi...r9), si xmm0-xmm7. Haiathiri kujikusanya
      (N32 pekee) lakini inahitajika kwa lugha kamili.
- [ ] **AST_BADILI (48)** — haina kishikizi maalum katika mkaguzi
- [ ] **Uthibitishaji wa aina za hali za `chagua`** dhidi ya usemi unaojaribiwa
- [ ] **mteremko.swa** — towe lake ni msimbo mfu; uamuzi: kuifuta au kuikamilisha
      kwa hatua za uboreshaji wa baadaye
- [ ] **JIT kamili** — relocations za wito wa nje ndani ya msimbo wa JIT na
      kupitisha argv (kwa sasa ni 0)

### Kipaumbele cha Kati
- [x] **Maktaba ya Kawaida**
  - [x] `orodha.swa` — orodha inayobadilika (dynamic array)
  - [x] `mfuatano.swa` — shughuli za nyuzi kamili
  - [x] `ramani.swa` — jedwali la hashi
  - [x] `faili.swa` — shughuli za faili
  - [x] `hesabu.swa` — hesabu za ziada
  - [x] `kumbukumbu.swa` — usimamizi wa kumbukumbu
  - [x] `mpangilio.swa` — upangaji
  - [x] `nasibu.swa` — nambari nasibu
  - [x] `wakati.swa` — vipimo vya wakati

## Hatua ya 3: Kuondoa Utegemezi wa Rust [PASS] IMEFANIKIWA

- [x] Mkusanyaji wa Swa unajikusanya **bila kutumia kande**
- [x] Bootstrap inafungwa: mbegu -> Swa -> Swa -> binary
- [x] Uthibitisho: stage2.o == stage3.o == stage4.o sawa kwa baiti
- [!] Rust `kande` inabaki kama chombo cha vipimo na ukuzaji (CI) tu —
      si sehemu ya mnyororo wa uzalishaji

## Hatua ya 4: Kuondoa Utegemezi wa LLVM [PASS kwa mnyororo] IMEFANIKIWA

- [x] Native x86-64 backend (uzalishaji.swa) inazalisha binary bila LLVM
- [x] Mnyororo wa kujikusanya haugusi LLVM kabisa
- [x] Uthibitisho: Swa inajikusanya kupitia mnyororo kamili wa Swa -> Swa -> binary
- [!] LLVM inabaki ndani ya dereva wa Rust wa vipimo pekee

## Hatua ya 5: Kuziba Pengo la Mwisho la Bootstrap [IN PROGRESS] KAZI INAENDELEA

- [ ] **Baiti za mkono:** andika mkusanyaji mdogo wa kwanza kwa opcodes
      za x86-64 zilizoandikwa kwa mkono (bila NASM) — lengo: baiti 500
- [ ] **Kiunganishi cha kujitegemea:** kuondoa ld/gcc kwenye mnyororo
      (self-hosted linker au ELF inayojitegemea)
- [ ] **Runtime ya syscalls:** kuondoa libc/muda.c (fopen/fread/andika
      kupitia syscalls moja kwa moja)
- [ ] 0% bootstrap gap — hakuna lugha nyingine popote kwenye mnyororo

## Hatua ya 6: Lugha Kamili ya Mifumo [FUTURE] BAADAYE

- [ ] Maktaba ya kawaida kamili
- [ ] Mfumo wa moduli / vifurushi
- [ ] Zana za ujenzi (build system)
- [ ] Mazingira kamili ya uundaji

---

## Jinsi ya Kuchangia

Angalia [`CONTRIBUTING.md`](CONTRIBUTING.md). Masuala yenye lebo `good-first-issue` ni mahali pazuri pa kuanzia.

## Vipaumbele vya Sasa (Agosti 2026)

1. **Baiti za mkono (Hatua ya 5)** — mkusanyaji mdogo wa kwanza kwa opcodes
   za mkono, kuondoa NASM kwenye mnyororo
2. **ABI ya desimali** — hoja za float/double kupitia xmm0-xmm7
3. **JIT kamili** — relocations za wito wa nje na kupitisha argv
4. **Uamuzi wa mteremko.swa** — kuifuta au kuikamilisha
5. **Kiunganishi cha kujitegemea** — kuondoa ld/gcc kwenye mnyororo
6. **Runtime ya syscalls** — kuondoa libc/muda.c

## Historia Fupi ya Milestone (Julai-Agosti 2026)

- PR #117: modulo, maoni ya bloku, radiksi, asimilia mchanganyiko — MERGED
- PR #140: kuondoa maneno muhimu ya bloat (na, au, si, tupu, kweli, uongo) — MERGED
- PR #131-133, #141: marejeo ya mbele, hifadhi/upakiaji wa aina, husisha nukuu,
  AST_KWELI/UONGO/TUPU — MERGED
- PR #142: mnyororo kamili wa kujikusanya na makosa 0 ya mkaguzi — MERGED
- PR #143: JIT — thamani ya kurudi, stub ya main, daraja la tekeleza — MERGED
