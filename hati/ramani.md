# Ramani ya Mradi / Project Roadmap

## Muhtasari

- **Keywords:** 42 za Kiswahili (hakuna Kiingereza katika sintaksia)
- **Aina:** 25 za nambari (N8-N128, A8-A128, D16-D80, B1-B64, W0-W64)
- **Majaribio:** 199/199 yanapita (146 maktaba + 52 ujumuishaji + 1 nyaraka)
- **Backend:** LLVM (bootstrap ya Rust), uzalishaji.swa (native x86-64 kwa kujikusanya)

## Hatua ya 0: Mkusanyaji wa Bootstrap wa Rust [PASS] IMEFANIKIWA

- [x] Lexer, parser, semantic analyzer
- [x] IR lowering (AST -> Swa IR)
- [x] LLVM codegen (x86-64 native binaries)
- [x] ABI classification (sret, struct returns)
- [x] Majaribio 199/199 yanapita

## Hatua ya 1: Kujikusanya kwa Msingi [PASS] IMEFANIKIWA

- [x] Msomaji wa kujikusanya (`msomaji.swa`)
- [x] Mchanganuzi wa kujikusanya (`msambazaji.swa`)
- [x] Mkaguzi wa kisemantiki (`mkaguzi.swa`)
- [x] Kizalishaji cha native x86-64 (`uzalishaji.swa`)
- [x] Binary inajikusanya (K6 inapita)
- [x] Alloca-in-loop imerekebishwa (mbinu ya kupitisha mara mbili)
- [x] CFG dead-code imerekebishwa (ufuatiliaji wa BrCond)
- [x] Hitilafu ya O1 (SelectionDAG) imetatuliwa kwenye LLVM 22.1.8
- [x] Vipengele vya lugha vinavyotumika: functions, loops (wakati/hali), if/else, structs, heap, unary minus, break/continue, short-circuit evaluation, assignment, bitwise ops, ternary

## Hatua ya 2: Mkusanyaji Kamili wa Kujikusanya [IN PROGRESS] KAZI INAENDELEA

### Usanifu wa Sasa wa Kujikusanya

Bootstrap inafanya kazi hivi:
1. Rust `kande` husoma na kuchambua `msingi/*.swa` kupitia lexer/parser/IR lowering yake
2. Rust huzaa binary ya Swa kupitia LLVM codegen
3. Binary hiyo ya Swa inajikusanya yenyewe kwa kutumia pipeline yake asilia:
   - `msomaji.swa` → usomaji (lexing)
   - `msambazaji.swa` → uchanganuzi (parsing → AST)
   - `mkaguzi.swa` → ukaguzi wa kisemantiki (aina, mawanda)
   - `uzalishaji.swa` → uzalishaji wa msimbo wa mashine (ELF64 .o)

**Uchunguzi muhimu wa usanifu:** `uzalishaji.swa` (mistari 2,913) hukusanya moja kwa moja
kutoka AST (`husisha { msambazaji.swa }`), SI kutoka IR. `mteremko.swa` (mistari 649)
hutoa IR lakini towe lake halitumiki katika mnyororo wa sasa wa kujikusanya.
IR inatumika TU kwenye njia ya Rust → LLVM.

### Kipaumbele cha Juu
- [ ] **PR #117** (feat/k12-modulo-v2) — ongeza opereta ya modulo (`%`), maoni ya bloku (`/* */`),
      nambari za radiksi (0x/0o/0b), tokeni za asimilia mchanganyiko (+=, -=, *=, /=, %=)
  - [x] Imepitiwa — mistari +325/-16, majaribio yote 199 yanapita
  - [ ] Inahitaji kuunganishwa (merge)
- [ ] **mkaguzi.swa** — kamilisha ukaguzi wa aina
  - [x] Aina nyingi za AST zinashughulikiwa: KAMA, WAKATI, CHAGUA, TANGAZO, ASIMILIA, n.k.
  - [x] Uthibitishaji wa hoja za mwito wa kazi (idadi + aina)
  - [x] Utafutaji wa sehemu za muundo kwa utendaji (kache ya nodi 32)
  - [ ] Hakuna kishikizi cha AST_MODULO (49) — kitakuja na PR #117
  - [ ] Uthibitishaji wa aina za hali za `chagua` dhidi ya usemi unaojaribiwa
- [ ] **uzalishaji.swa** — kithibitisho na ukamilishaji
  - [x] Inashughulikia aina zote za usemi (4-48) kupitia kitatuzi chenye safu
  - [x] Inashughulikia taarifa: KAMA, WAKATI, CHAGUA, RUDISHA, VUNJA, ENDELEA
  - [x] Inasaidia SSE/SSE2 kwa desimali (F32 na D64)
  - [x] Inasaidia miundo (sehemu kwa nukta na mshale)
  - [x] Inashughulikia sret kwa urejeshaji wa miundo
  - [x] Inashughulikia vigezo vya ulimwengu kupitia RIP-relative addressing
  - [ ] **Pengo la ABI la desimali:** Hoja za kazi hupitishwa kwenye rejesta kamili
        (rdi, rsi, rdx, rcx, r8, r9), sio xmm0-xmm7. Hii haiathiri kujikusanya
        kwa sababu mkusanyaji wa Swa unatumia N32 pekee.
  - [ ] Hakuna kishikizi cha AST_MODULO (49) — kitakuja na PR #117
- [ ] **mteremko.swa** — Hii SI kipaumbele tena kwa kujikusanya
  - [x] Ina vishikizi vya aina zote za AST (isipokuwa AST_MODULO)
  - [x] Inadai sret na alloca-in-loop lakini HAZIJATEKELEZWA kwenye mwili wa kazi
  - [!] **Towe lake la IR ni msimbo uliokufa (dead code)** — uzalishaji.swa hukusanya moja kwa moja kutoka AST
  - [ ] Inaweza kuwa muhimu baadaye kwa hatua za uboreshaji (optimization passes)
  - [ ] Ikiwa tutaamua kuondoa, tunaweza kuifuta. Ikiwa tutaamua kuitumia, inahitaji
        sret na alloca-in-loop kutekelezwa KWELI na uzalishaji.swa kubadilishwa kusoma IR

### Kipaumbele cha Kati
- [ ] **Maktaba ya Kawaida**
  - [x] `orodha.swa` — orodha inayobadilika (dynamic array)
  - [x] `mfuatano.swa` — shughuli za nyuzi kamili
  - [x] `ramani.swa` — jedwali la hashi
  - [x] `faili.swa` — shughuli za faili
  - [x] `hesabu.swa` — hesabu za ziada
  - [x] `kumbukumbu.swa` — usimamizi wa kumbukumbu
  - [x] `mpangilio.swa` — upangaji
  - [x] `nasibu.swa` — nambari nasibu
  - [x] `wakati.swa` — vipimo vya wakati

### Urekebishaji kutoka feat/k4 (ya kuchukuliwa kwa kuchagua)
- [ ] **fgetc: N8 + 0xFF** — kwenye `faili.swa`, fgetc inarudisha N8 lakini inahitaji
      kuficha na 0xFF ili kushughulikia kwa usahihi thamani hasi za EOF
- [ ] **ramani_weka: ulinzi wa kufurika** — kwenye `ramani.swa`, hakikisha
      uwekaji kwenye jedwali la hashi hauzidi uwezo

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

1. Unganisha PR #117 (k12-modulo-v2) — tayari imepitiwa
2. Chukua marekebisho 2 kutoka feat/k4 (fgetc + ramani_weka)
3. Ongeza AST_MODULO kwa mkaguzi.swa na uzalishaji.swa (pamoja na PR #117)
4. Thibitisha ABI ya desimali kwenye uzalishaji.swa (rejesta za xmm)
5. Jaribio la bootstrap kamili: Swa asilia → Swa inayojikusanya → pato linalofanana
6. Ondoa utegemezi wa LLVM kwenye njia chaguo-msingi ya ujenzi
