# Hati za Uboreshaji wa Swa

## Viwango vya Uboreshaji

Swa inasaidia viwango vitatu vya uboreshaji vinavyodhibitiwa na bendera za mstari
wa amri:

| Kiwango | Bendera           | Maelezo                                                       |
|---------|-------------------|---------------------------------------------------------------|
| `-O0`   | (msingi)          | Hakuna uboreshaji. FastISel hutumika kwa uteuzi wa maagizo.   |
| `-O1`   | `--opt`           | Pipeline ya kawaida ya LLVM pass kwenye kila kazi.             |
| `-O2`   | `--O2`            | O1 + uboreshaji zaidi wa LLVM (unakuja baadaye).              |

### `-O0` — Hakuna Uboreshaji (Msingi)

- **Uteuzi wa maagizo**: LLVM FastISel (uteuzi wa haraka wa maagizo)
- **Vikwazo**:
  - FastISel ina kikomo cha **vitalu 50 vya msingi** kwa kila kazi. Kazi
    zinazozidi kikomo hiki zinaweza kuwa na maagizo yanayodondoshwa na
    FastISel. LLVM huangukia kwenye SelectionDAG kiotomatiki kwa vitalu hivyo,
    lakini hii inaweza kuongeza muda wa kukusanya.
  - Hakuna upanuzi wa ndani (inlining).
  - Hakuna uondoaji wa msimbo uliokufa (DCE).
- **Utatuzi**: Msimbo unaotokana unalingana moja kwa moja na chanzo, hivyo
  kurahisisha utatuzi.
- **Utendaji**: Utekelezaji wa polepole zaidi — kwa maendeleo na utatuzi tu.

### `-O1` / `--opt` — Uboreshaji wa Kawaida

Pipeline ya LLVM inayotumika kwa **kila kazi** (`function` scope):

1. **mem2reg** — Pandisha `alloca` hadi SSA (Static Single Assignment).
   Hubadilisha uhifadhi wa rafu kuwa rejista za LLVM, kuwezesha uboreshaji
   unaofuata.

2. **instcombine** (`no-verify-fixpoint`) — Uchanganyaji wa maagizo
   (peephole optimization). Hurahisisha misemo ya mara kwa mara, huondoa
   maagizo yasiyo na maana, huchanganya shughuli zinazofanana.

   Bendera ya `no-verify-fixpoint` inaepuka uthibitishaji wa marudio baada ya
   kupita — kwa kuwa instcombine haijawahi kuhitaji marudio zaidi ya moja,
   hii inaokoa muda bila athari.

3. **dce** — Uondoaji wa msimbo uliokufa (Dead Code Elimination). Huondoa
   maagizo ambayo matokeo yake hayatumiki kamwe.

4. **GVN** — Global Value Numbering. Huondoa upakiaji unaorudiwa kutoka
   kumbukumbu, huchanganya mahesabu yanayofanana.

5. **simplifycfg** — Usafishaji wa CFG (Control Flow Graph). Hurahisisha
   matawi, huondoa vitalu visivyofikiwa, hufupisha misururu ya kuruka.

**Kiwango cha moduli** (baada ya kupita za kazi):

6. **always-inline** — Weka ndani kazi zilizowekwa alama ya `always_inline`.
   Muhimu kwa shughuli za bootstrap zinazohitaji upanuzi wa ndani.

**Mpangilio wa kupita** umechaguliwa kwa makini:
- `mem2reg` kwanza — inabadilisha allocas kuwa SSA, kuwezesha kupita zote
  zinazofuata kufanya kazi kwa ufanisi.
- `instcombine` pili — husafisha msimbo kabla ya DCE.
- `dce` tatu — huondoa msimbo uliokufa kabla ya GVN na simplifycfg.
- `gvn` na `simplifycfg` mwisho — hufanya kazi kwa msimbo uliosafishwa.

### `-O2` — Uboreshaji wa Juu (Unakuja Baadaye)

Itajumuisha O1 pamoja na:
- Upanuzi wa ndani kwa kazi za jumla (si `always_inline` pekee).
- Uchambuzi na uboreshaji wa vitanzi (loop unrolling, vectorization).
- Uboreshaji wa kati ya kazi (inter-procedural optimization).

## Athari ya Uboreshaji kwenye Utatuzi

- **O0**: Msimbo unalingana na chanzo — unaweza kutumia `printf`/`andika`
  kufuatilia utekelezaji. Mistari ya chanzo inalingana na maagizo ya mkutano.
- **O1**: Vigezo vya ndani vinaweza kupandishwa hadi rejista (kuondolewa
  kutoka rafu). Mistari ya chanzo bado inalingana lakini utaratibu wa
  maagizo unaweza kubadilika. Msimbo uliokufa unaondolewa.

## Utekelezaji

Pipeline ya LLVM inasanidiwa katika `LlvmBackend::optimize_module()`
(`src/codegen/llvm/mod.rs`). Mstari wa usindikaji unapitishwa kwa
`LLVMRunPasses` kwa kutumia sintaksia mpya ya meneja wa kupita wa LLVM.

## Marejeo

- LLVM Pass Manager: [https://llvm.org/docs/NewPassManager.html](https://llvm.org/docs/NewPassManager.html)
- LLVM opt levels: [https://llvm.org/docs/Passes.html](https://llvm.org/docs/Passes.html)
