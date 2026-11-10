# IR ya mstari ya Swa

`msingi/mkusanyaji/mteremko.swa` huteremsha AST iliyokaguliwa (msambazaji + mkaguzi) kuwa **IR ya mstari**: kila kazi ni orodha ya amri za anwani-tatu juu ya rejista pepe (vreg), zenye aina wazi. `msingi/mkusanyaji/mtafsiri.swa` huitekeleza moja kwa moja.

```sh
bin/swa --tafsiri programu.swa [hoja...]   # teremsha hadi IR, kisha itekeleze
bin/swa --kati programu.swa                  # chapisha IR
```

## Kwa nini

- **Sehemu za nyuma nyingi, sehemu ya mbele moja.** x86-64 (`uzalishaji.swa`) na WebAssembly (`uzalishaji_wasm.swa`) kila moja hutafsiri AST nzima. ARM64 (lugha-swa/swa#321) na Mizani (`swa --mizani`) zinapaswa kutafsiri amri hizi rahisi badala yake: miundo, upeo wa majina, `&&`/`||` za mzunguko mfupi, `kwa`/`endelea` na hesabu ya vielekezi tayari vimeteremshwa.
- **Mkaguzi huru.** `gharama/jaribu-ir.mapishi` huendesha kila programu ya `majaribio/ir/WIGO.txt` kama msimbo wa mashine NA kupitia IR; namba ya kutoka na stdout lazima viwe sawa kabisa. Hitilafu ya sehemu ya mbele au ya nyuma hujitokeza kama tofauti.
- **Rejea.** Matokeo ya mtafsiri ndiyo jibu sahihi ambalo kila sehemu mpya ya nyuma lazima ilingane nalo.

## Amri

Kila amri ina `ir_amri`, `ir_lengo` (vreg ya matokeo au -1), `ir_a`, `ir_b`, `ir_c`, `ir_namba` (n64), na aina: `ir_upana` (baiti 1/2/4/8) na `ir_ishara` (1 = yenye ishara). Thamani ya kila vreg huhifadhiwa ikiwa **imesawazishwa** kwa aina yake: imekatwa kwa upana wake na kupanuliwa (kwa ishara au sifuri) hadi biti 64.

| Amri | Maana |
|---|---|
| `namba` | `lengo = ir_namba` |
| `nakili` | `lengo = v[a]`, ikisawazishwa kwa (upana, ishara) |
| `binari` | `lengo = v[a] (c) v[b]`; tendo `c`: jumlisha, tofauti, zidisha, gawanya, modulo, `& \| ^ << >>`, `== != < > <= >=`. Ishara huamua kugawanya, `>>` na ulinganisho; ulinganisho hutoa 0/1 |
| `unari` | `lengo = (c) v[a]`: hasi, kinyume (`~`), si (`!`) |
| `pakia` | `lengo = kumbukumbu[v[a]]` kwa (upana, ishara) |
| `hifadhi` | `kumbukumbu[v[a]] = v[b]` kwa upana |
| `fremu` | `lengo = fremu + ir_namba` -- anwani ya nafasi ya kigeu cha ndani au kigezo |
| `anwani` | `lengo = ir_namba` -- anwani halisi ya kigeu cha ulimwengu au mfuatano halisi |
| `lebo` / `ruka` | lebo `a`; ruka hadi lebo `a` |
| `ruka_kama` / `ruka_sivyo` | ruka hadi lebo `b` kama `v[a] != 0` / `== 0` |
| `wito` | `lengo = kazi a(v[ir_hoja[b..b+c]])`; matokeo husawazishwa kwa aina ya kurudi |
| `wito_asili` | kazi ya asili `a`: `wito_wa_mfumo`, `tenga`, `achilia`, na vipande vya `andika` (sehemu ya maandishi, namba, mfuatano, herufi) |
| `rudisha` | rudisha `v[a]` (`a = -1`: 0) |

Kazi (`irk_*`) zina amri zao `[irk_mwanzo, irk_mwisho)`, idadi ya vreg, ukubwa wa fremu, na vigezo (`irp_*`: ofseti ya nafasi na upana). Kazi `<vianzio vya ulimwengu>` huendeshwa kabla ya `main`.

## Semantiki zinazofuata msimbo wa mashine

- Upande wa **kulia** wa tendo hutathminiwa kwanza, kisha kushoto.
- `x op= y`: anwani ya `x` hukokotolewa mara moja, kabla ya `y`.
- `kwa`: `endelea` huruka kwenye hatua; `fanya { } wakati (s);` hutekeleza mwili kwanza.
- Baada ya `rudisha`/`acha`/`endelea` ndugu wa kifungu hawatolewi, isipokuwa hatua ya `kwa`.
- `andika`: umbo hugawanywa wakati wa kuteremsha; `%d` ya aina ndogo kuliko baiti 8 hupanuliwa kutoka biti 32 zenye ishara.

## Wigo

Ndani: nambari kamili za upana wote (n, a, b, w), vielekezi na safu za nambari kamili, vigeu vya ndani na vya ulimwengu, waendeshaji wote wa nambari kamili, `kama`/`wakati`/`kwa`/`fanya`, `acha`/`endelea`, `&& || ! ?:`, wito na kujirudia (bila kikomo cha rafu ya mtafsiri: wito ni rekodi kwenye rafu yake), mifuatano halisi, `andika`/`andika_kosa`, `wito_wa_mfumo`, `tenga`, `ukubwa`.

Nje (hukataliwa kwa sauti, kutoka 2: `IR: haijaungwa mkono: ...`): miundo, desimali, vielekezi vya kazi, `chagua`. Kila kipengele kikiongezwa, programu zinazopita huongezwa kwenye `majaribio/ir/WIGO.txt`.
