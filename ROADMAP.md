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
- [x] Vipengele vya lugha vinavyotumika: functions, loops, if/else, structs, heap

## Hatua ya 2: Mkusanyaji Kamili wa Kujikusanya [IN PROGRESS] KAZI INAENDELEA

### Kipaumbele cha Juu
- [ ] **mteremko.swa** -- Kamilisha kiteremshi cha kujikusanya
  - [ ] Sret handling kwa miundo inayorudishwa
  - [ ] Urekebishaji wa alloca-in-loop
  - [ ] Uzalishaji wa .o faili (sasa LLVM IR tu)
- [ ] **mkaguzi.swa** -- Kamilisha mkaguzi wa kisemantiki
  - [ ] Uthibitishaji wa aina kwa taarifa zote
  - [ ] Uthibitishaji wa hoja za mwito wa kazi
  - [ ] Uthibitishaji wa matawi ya `chagua`

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

1. `mteremko.swa` -- sret + alloca-in-loop fix (hii ndio kazi muhimu zaidi sasa)
2. `mkaguzi.swa` -- kukamilisha ukaguzi wa aina
3. `--opt` flag -- kuongeza LLVM pass manager
