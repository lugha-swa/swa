# Lugha ya mashine ya Swa (x86-64) -- hatua 4 ya mnyororo huru

Lugha ambayo `mbegu` itaandikwa nayo, na ambayo assembler ya hatua 4 husoma. Si tafsiri ya majina ya Intel: imebuniwa kwa **maana**, kwa familia za aina za Swa.

## Kanuni

1. **Mstari mmoja ni agizo moja la mashine.** Hakuna msimbo uliofichwa; kila mstari una usimbaji mmoja unaotabirika.
2. **Mpanga programu anaaminiwa kikamilifu.** Kila kitu ambacho CPU inaweza kufanya kinaweza kuandikwa. Upana na ishara huandikwa na mpanga programu kwenye kila agizo -- hayakisiwi wala hayakatazwi na matangazo.
3. **Kinachokataliwa ni kisichokuwepo tu:** lebo isiyofafanuliwa, thamani isiyotosha sehemu yake, au umbo ambalo CPU haina (mf. kumbukumbu kwenda kumbukumbu).
4. **Umbo la agizo ndilo umbo la mashine.** x86 ina hoja mbili (`lengo op= chanzo`); lugha inaonyesha hivyo, haiahidi misemo ambayo mashine haina.

## Rejista

`r0`-`r15` (biti 64) -- namba zile ambazo CPU hutumia kwenye usimbaji. Upana ni sehemu ya jina: `r0d` (32), `r0w` (16), `r0b` (8). Rejista za desimali: `x0`-`x15`.

## Kumbukumbu: familia na upana kila mara

Kila ufikiaji wa kumbukumbu huandika familia na upana, kama aina za Swa:

```
n64[r5 - 8]        a8[r6 + r1]        n32[aina_ast + r12*4]        d64[r10 + r0*8]
```

Familia ina maana pale tu mashine inapohitaji kuchagua:
- kupakia kipana zaidi: `n` = panua kwa ishara, `a` = panua kwa sifuri
- `d` = desimali (rejista za `x`)
- vinginevyo `n`, `a`, `b` ni sawa kwa mashine -- mpanga programu anaandika anachomaanisha.

`[lebo]` peke yake ni anwani inayohusiana na agizo (RIP-relative).

## Kuhamisha na kuhesabu

| Lugha | Maana |
|---|---|
| `r0 = r3` | nakili rejista |
| `r0d = 5` | weka namba |
| `r0 = n64[r5 - 8]` / `n64[r5 - 8] = r0` | soma / andika kumbukumbu |
| `r0d = a8[r6]` / `r0d = n8[r6]` | soma baiti 1 ukipanua kwa sifuri / kwa ishara |
| `r0 = &ujumbe` / `r0 = &[r2 + r1 + 1]` | anwani (hesabu ya anwani bila kusoma) |
| `r0 += 5` `-=` `&=` `\|=` `^=` | hesabu ya hoja mbili |
| `r3 *= r1` / `r3 = r3 * 10` | zidisha (n) |
| `r0++` / `r0--` | ongeza / punguza moja (tofauti na `r0 += 1`: usimbaji mwingine) |
| `r0 = -r0` / `r0 = ~r0` | hasi / geuza biti |
| `r0 <<= 4` / `r0 >>=a 4` / `r0 >>=n 4` | sogeza; kulia kunahitaji familia (a = sifuri juu, n = ishara juu) |
| `gawanya n r1d` / `gawanya a r1d` | gawanya `r2:r0` kwa `r1d` -- matokeo `r0`, baki `r2` (umbo la mashine) |
| `panua r0d` | panua ishara ya `r0d` kwenda `r2d` (kabla ya `gawanya n`) |

`r0d ^= r0d` ni agizo lake (si `r0d = 0`) -- mpanga programu huchagua usimbaji.

## Kulinganisha na kuruka

Kulinganisha hakutoi thamani; huweka hali ambayo kuruka kunafuata -- kama mashine ilivyo.

```
linganisha r0, 5          ; cmp
pima r1, r1               ; test (biti za pamoja)
ruka .kitanzi             ; daima
ruka .mwisho kama ==      ; == !=
ruka .chini kama <n       ; < <= > >=  pamoja na familia: n (ishara) au a (bila ishara)
ruka .kosa kama hasi      ; hasi / si_hasi
r0b = kama <n             ; weka 1/0 kulingana na hali
zunguka .kitanzi          ; punguza r1, ruka ikiwa si sifuri
```

Hali inatoka kwa agizo la mwisho lililoiweka (`linganisha`, `pima`, au hesabu kama `r1--`).

## Kazi, rafu, mfumo

| Lugha | Maana |
|---|---|
| `ita jina` / `rudi` | wito / rudisha |
| `sukuma r0` / `vuta r0` / `sukuma n64[hesabu]` | rafu |
| `mfumo` | wito wa mfumo (Linux syscall) |
| `nakili` | nakili `r1` baiti kutoka `[r6]` kwenda `[r7]` |
| `mwelekeo_mbele` | nakili kwenda mbele (cld) |

## Desimali

`x0 = d64 r3` (namba kamili -> desimali), `x0 += x1`, `x1 /= d64[r10 + r0*8]`, `r3 = x0` (biti za desimali kwenda rejista).

## Data na majina

Data hutangazwa kama Swa; mahali pake (data au tupu) hufuata kama ina kianzio:

```
fafanua UPEO_CHANZO = 4194304
n64 idadi_tokeni                  ; sifuri mwanzoni
n32 aina_ast[262144]
n8  ujumbe = "kosa: chanzo kikubwa mno", 10, 0
```

Lebo: `jina:` na za ndani `.jina:` (za wigo wa lebo iliyotangulia). Programu huanzia `anza:`. Mpangilio wa faili ni sawa na `mbegu.bin` ya sasa (msimbo 0x401000, data 0x40c000, tupu baadaye), ili msimbo ulinganishwe kwa baiti.

## Picha tambarare na biti 32

Kwa kianzio cha kernel (Multiboot) na programu zisizo na mfumo wa uendeshaji:

| Agizo | Maana |
|---|---|
| `picha 0x100000` | matokeo ni baiti tupu (hakuna ELF) zinazoanzia anwani hii. Msimbo, data na tupu hufuatana kwa mpangilio wa chanzo; tupu (`n64 pml4[512]`) huwa sifuri ndani ya picha. Lazima iwe kabla ya msimbo wowote. |
| `biti 32` / `biti 64` | usimbaji wa maagizo yanayofuata. Kwenye biti 32 `[lebo]` ni anwani kamili, hakuna REX (rejista `r8`-`r15`, upana 8 na `r4b`-`r7b` ni kosa 5), na `n32[lebo] = r0d` huchukua umbo fupi (A3). |
| `pangilia 4096` | sogeza hadi anwani inayogawanyika kwa N (nguvu ya 2); nafasi hujazwa `0x90` (agizo tupu). |

## Mfumo (ring 0)

| Agizo | Maana |
|---|---|
| `katizo_zima` / `katizo_washa` | zima / washa katizo (cli / sti) |
| `simama` | simamisha kichakataji hadi katizo (hlt) |
| `soma_msr` / `andika_msr` | `r2d:r0d` = / -> MSR namba `r1d` |
| `rudi_katizo` | rudi kutoka katizo (iretq) |
| `cr0 = r0d`, `r3d = cr4` | rejista za udhibiti `cr0`-`cr8` (upana: `d` kwenye biti 32, kamili kwenye biti 64) |
| `ds = r0w`, `r0w = ss` | rejista za sehemu `es cs ss ds fs gs` |
| `pakia_gdt [x]` / `pakia_idt [x]` | pakia jedwali la sehemu / la katizo (lgdt / lidt) |
| `ruka_mbali 8, anza64` | ruka mbali kwenda sehemu:anwani (biti 32) |

`majaribio/kernel/stub.swm` ni mfano kamili: Multiboot1, kurasa, biti 32 -> 64, kisha `ita kazi_kuu`. Picha yake ni sawa kwa baiti na ya NASM iliyotangulia.
