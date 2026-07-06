# Ramani ya Mradi / Project Roadmap

## Hatua ya 0: Mkusanyaji wa Bootstrap wa Rust [PASS] IMEFANIKIWA

- [x] Lexer, parser, semantic analyzer
- [x] IR lowering (AST -> Swa IR)
- [x] LLVM codegen (x86-64 native binaries)
- [x] ABI classification (sret, struct returns)
- [x] Majaribio 173/173 yanapita

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

## Hatua ya 4: Lugha Kamili ya Mifumo [FUTURE] BAADAYE

- [ ] Maktaba ya kawaida kamili
- [ ] Mfumo wa moduli / vifurushi
- [ ] Zana za ujenzi (build system)
- [ ] Kiunganishi cha kujikusanya (self-hosted linker)
- [ ] Mazingira kamili ya uundaji

---

## Jinsi ya Kuchangia

Angalia [`CONTRIBUTING.md`](CONTRIBUTING.md). Masuala yenye lebo `good-first-issue` ni mahali pazuri pa kuanzia.

## Vipaumbele vya Sasa (Julai 2026)

1. Rekebisha hitilafu ya O1 kwenye `tokeni_soma_kitambulisho`
2. `mteremko.swa` -- sret + alloca-in-loop fix (hiyo ndiyo kazi muhimu zaidi sasa)
3. `mkaguzi.swa` -- kukamilisha ukaguzi wa aina
4. `--opt` flag -- kuongeza LLVM pass manager
5. Urejeshaji wa makosa kwa mchanganuzi
