# Ramani ya Zana na Majukwaa ya Swa (baada ya 1.0)

Hati hii inaeleza wigo halisi wa zana na majukwaa zinazobaki baada ya
1.0, kwa ukali wa uaminifu: kilichofanyika, kilicho karibu, na
kilichopo mbali.

## Kilichofanyika

| Zana | Hali | Ushahidi |
|---|---|---|
| Mkusanyaji wa kujikusanya (mbegu -> stage1 -> stage2 == stage3) | KAMILI | fixpoint sawa kwa baiti |
| Vipimo rasmi vya lugha | KAMILI | hati/vipimo-vya-lugha.md |
| Maktaba ya kawaida | KAMILI | jaribio_maktaba_mbegu_exe |
| JIT (--papo) ndani ya exe | KAMILI | UND=0; jaribio la CI |
| Formatter (umbizaji) | INAFANYIKA | zana/umbizaji.swa (kwa Swa yenyewe) |

## Kilicho karibu (muda wa wiki)

1. **Mfumo wa moduli kwa watumiaji** -- HAKUNA mfululizo (mbegu wala
   .swa) unaosoma husisha { } (chanzo kinapaswa kuunganishwa;
   kilichopimwa 2026-08-27). Hati/vipimo-vya-lugha.md 8 linaeleza
   hili. Kazi: mfululizo ujifunze kupakia viungo vya ndani (msomaji
   wa faili kwa kujirudia) -- pengo la urahisi wa matumizi, si la
   usalama.
2. **Mpaka wa D64 na nambari kamili** -- kurudisha, ugawaji,
   operesheni mchanganyiko na ulinganisho mchanganyiko vina jibu
   baya kwa mifululizo yote miwili; D32 imevunjika (uthibitisho
   2026-08-27). ABI ya wito wa kazi za D64 imefungwa kwenye
   mifululizo yote miwili (mipaka.md 4c imerekebishwa).

## Kilicho mbali (muda wa miezi)

1. **LSP (server ya lugha)** -- uchanganuzi wa ziada, kukamilisha
   majina, ufafanuzi, rejeleo. Inahitaji: mchanganuzi wa kuvumilia
   makosa (error-tolerant) -- mchanganuzi wa sasa unakataa ingizo
   lisilokamilika. Kazi kubwa ya kujitegemea.
2. **Debugger** -- msaada wa DWARF (mstari, vigezo, rafu) kwenye
   uzalishaji.swa na mbegu. Inahitaji ujumuishaji na gdb/lldb.
3. **Majukwaa zaidi ya x86-64 Linux** -- ARM64 (aarch64) na riscv64.
   Hii inamaanisha: baiti mpya za mkono za kwanza kwa kila usanifu,
   codegen mpya ya mbegu, na ABI ya jukwaa. Kila usanifu ni mradi
   wake mwenyewe.
4. **Kidhibiti cha vifurushi (package manager)** -- baada ya mfumo wa
   moduli kuwa thabiti.

## Kanuni ya kazi

Kila hatua inapaswa kuingia kupitia PR, kwa Kiswahili, na kila moja
ijulikane na majaribio ya kurejesha kabla ya kuchanganywa. Mabadiliko
ya mzizi wa uaminifu hujengwa na kujaribiwa KABLA ya kugandishwa
(angalia CONTRIBUTING.md).
