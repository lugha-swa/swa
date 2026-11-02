# Kusanyiko la x86 kwa Kiswahili (hatua 4 ya mnyororo huru)

Lugha ya assembly ya x86-64 ambayo `mbegu` imeandikwa nayo, na ambayo assembler ya hatua 4 husoma. Maana ya kila agizo ni ile ile ya x86-64 (Intel/AMD); majina yetu ni ya Kiswahili.

## Rejista

Namba za rejista ni zile ambazo CPU yenyewe hutumia kwenye usimbaji -- si majina ya Intel:

| Kiswahili | biti 64 | biti 32 | biti 16 | biti 8 |
|---|---|---|---|---|
| `r0` | rax | `r0d` (eax) | `r0w` (ax) | `r0b` (al) |
| `r1` | rcx | `r1d` | `r1w` | `r1b` |
| `r2` | rdx | `r2d` | `r2w` | `r2b` |
| `r3` | rbx | `r3d` | `r3w` | `r3b` |
| `r4` | rsp | `r4d` | `r4w` | `r4b` (spl) |
| `r5` | rbp | `r5d` | `r5w` | `r5b` (bpl) |
| `r6` | rsi | `r6d` | `r6w` | `r6b` (sil) |
| `r7` | rdi | `r7d` | `r7w` | `r7b` (dil) |
| `r8`-`r15` | r8-r15 | `r8d`... | `r8w`... | `r8b`... |
| `x0`-`x15` | xmm0-xmm15 | | | |

## Maagizo

| Kiswahili | x86 | Kiswahili | x86 |
|---|---|---|---|
| `hamisha` | mov | `linganisha` | cmp |
| `hamisha_sifuri` | movzx | `pima` | test |
| `hamisha_ishara` | movsxd | `ongeza` / `toa` | add / sub |
| `anwani` | lea | `zidisha` | imul |
| `sukuma` / `vuta` | push / pop | `gawanya` | idiv |
| `ita` / `rudi` | call / ret | `gawanya_bila_ishara` | div |
| `ruka` | jmp | `ongeza_moja` / `toa_moja` | inc / dec |
| `ruka_sawa` / `ruka_si_sawa` | je / jne | `kinyume` | neg |
| `ruka_dogo` / `ruka_dogo_sawa` | jl / jle | `geuza` | not |
| `ruka_kubwa` / `ruka_kubwa_sawa` | jg / jge | `na` / `au` / `au_pekee` | and / or / xor |
| `ruka_chini` / `ruka_chini_sawa` | jb / jbe | `sogeza_kushoto` | shl |
| `ruka_juu` / `ruka_juu_sawa` | ja / jae | `sogeza_kulia` | shr |
| `ruka_hasi` / `ruka_si_hasi` | js / jns | `sogeza_kulia_ishara` | sar |
| `weka_sawa` / `weka_si_sawa` | sete / setne | `weka_dogo` / `weka_dogo_sawa` | setl / setle |
| `weka_kubwa` / `weka_kubwa_sawa` | setg / setge | `zunguka` | loop |
| `ita_mfumo` | syscall | `panua_ishara` | cdq |
| `futa_mwelekeo` | cld | `rudia nakili_baiti` | rep movsb |
| `badili_kuwa_desimali` | cvtsi2sd | `ongeza_desimali` / `gawanya_desimali` | addsd / divsd |
| `hamisha_q` | movq | | |

`ruka_sawa` pia ndiyo `jz`; `ruka_si_sawa` pia ndiyo `jnz`.

## Hoja

- **Kumbukumbu**: `[msingi + faharisi*kipimo + umbali]`, mf. `[r5-24]`, `[ast_aina + r12*4]`, `[r13 + r12 + 1]`.
- **Ukubwa** pale usipojulikana kutoka rejista: `baiti`, `neno2`, `neno4`, `neno8` mbele ya `[`, mf. `ongeza_moja neno8 [kiwango]`.
- **`[lebo]` peke yake** ni anwani inayohusiana na agizo (RIP-relative), kama `default rel` ya NASM.
- **Namba**: desimali, `0x...`, herufi `'a'`, na misemo yenye `+ - * ~` na majina ya `%fafanua`.

## Lebo na maelekezo

| Kiswahili | Maana |
|---|---|
| `jina:` | lebo |
| `.jina:` | lebo ya ndani -- inahusiana na lebo ya kawaida iliyotangulia (kama NASM) |
| `sehemu msimbo` / `sehemu data` / `sehemu tupu` | .text / .data / .bss |
| `%fafanua JINA thamani` | %define |
| `baiti`, `neno2`, `neno4`, `neno8` (kama maelekezo) | db, dw, dd, dq |
| `hifadhi1`, `hifadhi2`, `hifadhi4`, `hifadhi8` | resb, resw, resd, resq |
| `anza:` | mahali programu inapoanzia (ELF entry) |

## Mpangilio wa faili

Sehemu zinawekwa kwenye anwani zile zile ambazo `mbegu.bin` ya sasa inazo, ili msimbo uwe sawa kwa baiti: `msimbo` 0x401000, `data` 0x40c000, `tupu` mara baada ya `data`. Assembler huandika ELF yenyewe -- hakuna linker.

## Uchaguzi wa usimbaji (sawa na NASM, ili uthibitisho wa baiti ufanye kazi)

- Tawi fupi (baiti 1 ya umbali) likitosha; vinginevyo refu. Mapitio hurudiwa mpaka urefu wa kila tawi uache kubadilika.
- Namba inayotosha baiti 1 yenye ishara hutumia umbo fupi (`83 /n ib`); `r0d`/`r0` hutumia umbo la `3D`/`05`/... pale linapokuwa fupi.
- `hamisha r7, 1` (64-bit, namba chanya ya biti 32) husimbwa kama `hamisha r7d, 1` -- baiti 5.
