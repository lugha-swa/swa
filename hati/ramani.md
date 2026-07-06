# Ramani ya Mradi / Project Roadmap

## Muhtasari

- **Keywords:** 42 za Kiswahili (hakuna Kiingereza katika sintaksia)
- **Aina:** 25 za nambari (N8-N128, A8-A128, D16-D80, B1-B64, W0-W64)
- **Majaribio:** 174/174 yanapita
- **Backend:** LLVM (sasa), uzalishaji.swa (inajengwa -- lengo ni kuondoa LLVM)

## Hatua ya 0: Mkusanyaji wa Bootstrap wa Rust [PASS] IMEFANIKIWA

- [x] Lexer, parser, semantic analyzer
- [x] IR lowering (AST -> Swa IR)
- [x] LLVM codegen (x86-64 native binaries)
- [x] ABI classification (sret, struct returns)
- [x] Majaribio 174/174 yanapita

## Hatua ya 1: Kujikusanya kwa Msingi [PASS] IMEFANIKIWA

- [x] Msomaji wa kujikusanya (`msomaji.swa`)
- [x] Mchanganuzi wa kujikusanya (`msambazaji.swa`)
- [x] Mfumo wa aina wa kujikusanya
- [x] Binary inajikusanya (K6 inapita)
- [x] Alloca-in-loop imerekebishwa (mbinu ya kupitisha mara mbili)
- [x] CFG dead-code imerekebishwa (ufuatiliaji wa BrCond)
- [x] Marekebisho 12 ya hitilafu (6 codegen + 6 mchanganuzi)
- [x] Vipengele vya lugha vinavyotumika: functions, loops (wakati/kwa), if/else, structs, heap, unary minus, break/continue, short-circuit evaluation

## Hatua ya 2: Mkusanyaji Kamili wa Kujikusanya [IN PROGRESS] KAZI INAENDELEA

### Kipaumbele cha Juu
- [ ] **mteremko.swa** -- Kamilisha kiteremshi cha kujikusanya
  - [ ] Sret handling kwa miundo inayorudishwa
  - [ ] Urekebishaji wa alloca-in-loop (kwenye mteremko.swa)
  - [ ] Uzalishaji wa faili za `.o` (sasa LLVM IR tu)
- [ ] **mkaguzi.swa** -- Kamilisha mkaguzi wa kisemantiki
  - [ ] Uthibitishaji wa aina kwa taarifa zote
  - [ ] Uthibitishaji wa hoja za mwito wa kazi
  - [ ] Uthibitishaji wa matawi ya `chagua`
- [ ] **uzalishaji.swa** -- Anzisha native x86-64 backend iliyoandikwa kwa Swa
  - [ ] Uzalishaji wa maagizo ya msingi ya x86-64 (mov, add, sub, etc.)
  - [ ] Uteuzi wa maagizo kwa aina za kawaida (N32, N64, A64, D64)
  - [ ] Ushughulikiaji wa vitanzi na matawi (jmp, je, jne)
  - [ ] Ushughulikiaji wa ABI (kupitisha hoja, kurudisha thamani, sret)
  - [ ] Mpangilio wa rafu (stack frames, prologue/epilogue)
- [ ] **Rekebisha hitilafu ya O1** -- SelectionDAG inaharibu tofauti ya kielekezi katika `tokeni_soma_kitambulisho`

### Kipaumbele cha Kati
- [ ] **Pipeline ya uboreshaji** -- LLVM pass manager (`--opt`)
  - [ ] `mem2reg` (alloca -> SSA)
  - [ ] `instcombine`, `gvn`, `dce`
- [ ] **Maktaba ya Kawaida**
  - [ ] `orodha.swa` -- orodha inayobadilika (dynamic array)
  - [ ] `mfuatano.swa` -- shughuli za nyuzi kamili
  - [ ] `ramani.swa` -- jedwali la hashi

### Kipaumbele cha Chini
- [ ] **Malengo zaidi** -- ARM, AArch64, RISC-V codegen
- [ ] **Mkusanyiko mtambuka** -- cross-compilation
- [ ] **Kisafishaji** -- remove dead code (`values_initial_len`, unused fns)

## Hatua ya 3: Kuondoa Utegemezi wa Rust [GOAL] LENGO KUU

- [ ] Mkusanyaji wa Swa unajikusanya **bila kutumia kande**
- [ ] Bootstrap inafungwa: Swa -> Swa -> binary
- [ ] Uthibitisho: binary ya Swa inazalisha binary inayofanya kazi

## Hatua ya 4: Kuondoa Utegemezi wa LLVM [GOAL] LENGO KUU

- [ ] Native x86-64 backend (uzalishaji.swa) inazalisha binary bila LLVM
- [ ] Mkusanyaji wote unajitegemea -- hakuna Rust, hakuna LLVM
- [ ] Uthibitisho: Swa inajikusanya kupitia mnyororo kamili wa Swa -> Swa -> binary (bila LLVM)
- [ ] Hii inafanya Swa kuwa lugha ya kwanza ya Kiafrika yenye mkusanyaji anayejitegemea kikamilifu

## Hatua ya 5: Lugha Kamili ya Mifumo [FUTURE] BAADAYE

- [ ] Maktaba ya kawaida kamili
- [ ] Mfumo wa moduli / vifurushi
- [ ] Zana za ujenzi (build system)
- [ ] Kiunganishi cha kujikusanya (self-hosted linker)
- [ ] Mazingira kamili ya uundaji

---

## Jinsi ya Kuchangia

Angalia [`CONTRIBUTING.md`](CONTRIBUTING.md). Masuala yenye lebo `good-first-issue` ni mahali pazuri pa kuanzia.

## Vipaumbele vya Sasa (Julai 2026)

1. uzalishaji.swa -- anzisha native x86-64 backend (hatua muhimu kwa uhuru kutoka LLVM)
2. Rekebisha hitilafu ya O1 kwenye `tokeni_soma_kitambulisho`
3. `mteremko.swa` -- sret + alloca-in-loop fix (hiyo ndiyo kazi muhimu zaidi sasa)
4. `mkaguzi.swa` -- kukamilisha ukaguzi wa aina
5. `--opt` flag -- kuongeza LLVM pass manager
6. Urejeshaji wa makosa kwa mchanganuzi
