# Ukaguzi wa Bafa za Ukubwa Thabiti

Hati hii ni matokeo ya ukaguzi wa kila mgawanyo wa ukumbusho wa
ukubwa thabiti (safu, dimbwi, majedwali) kwenye mzizi wa uaminifu
(msingi/awali.swm) na maktaba (msingi/maktaba/*.swa na msingi/mkusanyaji/*.swa). Kila kipengele kimoja
kimoja na uamuzi:

- **SALAMA KWA SAUTI** -- ukaguzi wa mpaka unaolia (kosa + exit) kabla
  ya kuandika nje ya uwezo.
- **SALAMA KWA UTHIBITISHO** -- ukubwa hauwezi kufikiwa kwa lugha/
  mipaka ya sasa, na sababu imeandikwa.
- **PENGO** -- hakuna ulinzi; limerekebishwa au limewekwa wazi hapa.

Historia ya darasa hili la mdudu: mizani ya safu ya ndani (star_count
= 0), dimbwi la majina la mkaguzi (8,192 dhidi ya hitaji ~9,500),
na majedwali ya acha/endelea (256) -- zote ziligunduliwa kwa bahati
baada ya kusababisha uharibifu. Kanuni ya kudumu baada ya ukaguzi
huu: **kila mgawanyo mpya wa ukubwa thabiti lazima uwe na ukaguzi wa
mpaka unaolia, au sababu ya uthibitisho iliyoandikwa.**

## mbegu.swm (mzizi wa uaminifu)

| Mgawanyo | Ukubwa | Ulinzi | Uamuzi |
|---|---|---|---|
| bafa_chanzo | 4 MB | sys_read(mpaka) + kosa la sauti | SALAMA KWA SAUTI |
| token_* | 262,144 | kikomo cha kitanzi + kosa la sauti (`Hitilafu: chanzo kina tokeni nyingi mno`) | SALAMA KWA SAUTI (ilirekebishwa -- ukataji wa kimya umefungwa; hati/mipaka.md #3) |
| ast_* | 262,144 | node_mpya -> kosa la sauti | SALAMA KWA SAUTI |
| dimbwi_mfuatano | 1 MB | .furika_mfuatano -> kosa la sauti (ilikuwa kimya) | SALAMA KWA SAUTI (ilirekebishwa) |
| bafa_msimbo | 1 MB | .furika za waandishi -> kosa la sauti (zilikuwa kimya) | SALAMA KWA SAUTI (ilirekebishwa) |
| bafa_data | 16 KB | .cr_nafasi + .sajili_ulimwengu -> sauti | SALAMA KWA SAUTI |
| label_* | 65,536 | .pita_lebo -> sauti | SALAMA KWA SAUTI |
| jina_nje | 65,536 | .nje_jaa -> sauti | SALAMA KWA SAUTI |
| rela_* | 65,536 | maeneo yote -> sauti | SALAMA KWA SAUTI |
| global_* | 4,096 | .vuta_shindwa_ulimwengu -> sauti | SALAMA KWA SAUTI |
| local_* | 1,024 | ukaguzi mpya -> sauti (haukuwepo) | SALAMA KWA SAUTI (ilirekebishwa) |
| lebo_vunja_mzunguko[16] | 16 | haijatumika kamwe (mabaki) | SALAMA KWA UTHIBITISHO (haijaandikwa) |
| mahali_rekebisho_vunja | 65,536 | ulikuwa 256 kimya -> sasa sauti | SALAMA KWA SAUTI (ilirekebishwa) |
| mahali_rekebisho_endelea | 65,536 | kama hapo juu | SALAMA KWA SAUTI (ilirekebishwa) |
| muundo_* | 64 | ilikuwa .shindwa kimya -> sasa sauti | SALAMA KWA SAUTI (ilirekebishwa) |
| nyuga_* | 512 | ilikuwa .shindwa_vuta kimya -> sasa sauti | SALAMA KWA SAUTI (ilirekebishwa) |
| kazi_ret_* | 256 | ukaguzi mpya -> sauti (haukuwepo) | SALAMA KWA SAUTI (ilirekebishwa) |
| zalisha_lebo_mahali | 1024 | ukaguzi mpya -> sauti (haukuwepo; kikomo kilikuwa 128) | SALAMA KWA SAUTI (ilirekebishwa) |
| gen_fixup_* | 65,536 | .furika -> sauti | SALAMA KWA SAUTI |
| bafa_muda | 4096 | maandishi <= baiti 8 | SALAMA KWA UTHIBITISHO (ukubwa wa maandishi umepangwa) |

## msingi/maktaba/*.swa na msingi/mkusanyaji/*.swa (maktaba) -- IMEKAGULIWA KAMILI

| Mgawanyo | Ukubwa | Ulinzi | Uamuzi |
|---|---|---|---|
| chanzo_buf (stage1) | 1 MB | "chanzo ni kikubwa mno" kwa sauti | SALAMA KWA SAUTI |
| kazi_jina_pool (mkaguzi) | 16,384 | ulinzi wa mpaka -> sauti | SALAMA KWA SAUTI |
| kazi_jina_off[1024] (mkaguzi) | 1024 | kazi_idadi >= 1024 -> rudisha | SALAMA KWA UTHIBITISHO (kazi ~350 < 1024) |
| kazi_param_enc[4096] (mkaguzi) | 4096 | kama < 4096 tu | SALAMA KWA UTHIBITISHO (vigezo vya kazi <= 8) |
| ast_*[65536] (msambazaji) | 65,536 | inachapisha + rudisha -1 | SALAMA KWA SAUTI (inachapisha) |
| ast_pool (msambazaji) | 4 MB | inachapisha + rudisha | SALAMA KWA SAUTI (inachapisha) |
| bafa_text (uzalishaji) | 512 KB | andika_baiti -> KOSA kwa sauti + flag | SALAMA KWA SAUTI |
| bafa_data (uzalishaji) | 256 KB | kama hapo juu | SALAMA KWA SAUTI |
| lebo_*[2048] (uzalishaji) | 2048 | lebo_idadi < 2048 | SALAMA KWA UTHIBITISHO (kazi ~350) |
| fixup_*[65536] (uzalishaji) | 65,536 | fixup_idadi < 65536 | SALAMA KWA UTHIBITISHO (fixups <= lebo ~2048) |
| rela_*[65536] (uzalishaji) | 65,536 | idadi_ya_urekebishaji_wa_elf < 65536 | SALAMA KWA UTHIBITISHO (RELA ~4,100 kwa kujikusanya) |
| data_ulimwengu_*[1024] | 1024 | idadi < 1024 | SALAMA KWA UTHIBITISHO (ulimwengu ~133) |
| sehemu_*/desimali_*[1024] | 1024 | >= 1024 -> -1 | SALAMA KWA UTHIBITISHO (tungo ~100) |
| hoja_*[32], hoja_bk[256] | 32/256 | idadi < 32 | SALAMA KWA UTHIBITISHO (hoja za wito <= 9) |
| param_*[16] | 16 | -- | SALAMA KWA UTHIBITISHO (vigezo <= 8) |
| mzunguko_lebo_*[16] | 16 | ulinzi mpya -> KOSA kwa sauti | SALAMA KWA SAUTI (ilirekebishwa) |
| var_pool[1024] | 1,024 | ulinzi mpya -> KOSA kwa sauti | SALAMA KWA SAUTI (ilirekebishwa) |
| nje_pool/str_buf (uzalishaji) | 32 KB/16 KB | nje_idadi < 2048; str kwa lebo | SALAMA KWA UTHIBITISHO (nje ~50) |
| Orodha/Ramani (orodha/ramani.swa) | dinamiki | ukuaji unakagua idadi >= uwezo lakini UNAANGUKA KWA VITENDO | PENGO -- orodha_ongeza zaidi ya uwezo inaanguka SEGV 139 kwa minyororo yote miwili (badili inatumia faharisi hasi yenye upanuzi wa sifuri; uthibitisho 2026-08-27, hati/uthibitisho-wa-lugha.md) |

## Hitimisho

Ukaguzi wa bafa UMEKAMILIKA kwa mzizi wa uaminifu (mbegu.swm) na
maktaba (msingi/maktaba/*.swa na msingi/mkusanyaji/*.swa). Kila mgawanyo wa ukubwa thabiti una ukaguzi
unaolia au uthibitisho ulioandikwa wa kutofikika. Darasa la mdudu wa
"ukubwa wa kukisia" limefungwa.

## Masomo

1. **Ukaguzi huu uligundua kufurika HALISI**: majedwali ya acha/
   endelea (256) yalifurika kwenye mkusanyiko mzima -- na kabla ya
   ulinzi, kufurika huko kulikuwa kimya (kurukaji kimya = kuruka
   mahali pasiposahihi kwenye njia za makosa). CI haikugundua kwa
   sababu inathibitisha fixpoint, si usahihi wa kila njia ya msimbo.
2. **Ukaguzi wa kwanza wa "abort za sauti" ulikuwa na mdudu wa
   makutano** (njia ya kawaida iliangukia kwenye lebo ya abort) --
   darasa lilelile lililotokea wakati wa PR #151. Kanuni ya nne ya
   sheria ya mzizi wa uaminifu (angalia fallthrough ya kila lebo)
   inathibitishwa tena. Jaribio la kabla ya kugandisha lililipata.
3. Majedwali yote ya mbegu sasa yana ukaguzi unaolia au uthibitisho
   ulioandikwa -- darasa la mdudu wa "ukubwa wa kukisia" limefungwa
   kwenye mzizi wa uaminifu.
