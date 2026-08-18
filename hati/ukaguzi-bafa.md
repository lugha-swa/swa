# Ukaguzi wa Bafa za Ukubwa Thabiti

Hati hii ni matokeo ya ukaguzi wa kila mgawanyo wa ukumbusho wa
ukubwa thabiti (safu, dimbwi, majedwali) kwenye mzizi wa uaminifu
(msingi/mbegu.s) na maktaba (msingi/*.swa). Kila kipengele kimoja
kimoja na uamuzi:

- **SALAMA KWA SAUTI** — ukaguzi wa mpaka unaolia (kosa + exit) kabla
  ya kuandika nje ya uwezo.
- **SALAMA KWA UTHIBITISHO** — ukubwa hauwezi kufikiwa kwa lugha/
  mipaka ya sasa, na sababu imeandikwa.
- **PENGO** — hakuna ulinzi; limerekebishwa au limewekwa wazi hapa.

Historia ya darasa hili la mdudu: mizani ya safu ya ndani (star_count
= 0), dimbwi la majina la mkaguzi (8,192 dhidi ya hitaji ~9,500),
na majedwali ya vunja/endelea (256) — zote ziligunduliwa kwa bahati
baada ya kusababisha uharibifu. Kanuni ya kudumu baada ya ukaguzi
huu: **kila mgawanyo mpya wa ukubwa thabiti lazima uwe na ukaguzi wa
mpaka unaolia, au sababu ya uthibitisho iliyoandikwa.**

## mbegu.s (mzizi wa uaminifu)

| Mgawanyo | Ukubwa | Ulinzi | Uamuzi |
|---|---|---|---|
| source_buf | 1 MB | sys_read(mpaka) + kosa la sauti | SALAMA KWA SAUTI |
| token_* | 65,536 | kikomo cha kitanzi; **ukataji kimya** | PENGO (Juu, hati/mipaka.md #3 — uamuzi wa makusudi) |
| ast_* | 65,536 | node_mpya → kosa la sauti | SALAMA KWA SAUTI |
| str_pool | 256 KB | .str_overflow → kosa la sauti (ilikuwa kimya) | SALAMA KWA SAUTI (ilirekebishwa) |
| text_buf | 256 KB | .overflow za waandishi → kosa la sauti (zilikuwa kimya) | SALAMA KWA SAUTI (ilirekebishwa) |
| data_buf | 4 KB | .cr_nafasi + .global_register → sauti | SALAMA KWA SAUTI |
| label_* | 16,384 | .skip_label → sauti | SALAMA KWA SAUTI |
| extern_name | 16,384 | .extern_full → sauti | SALAMA KWA SAUTI |
| rela_* | 16,384 | maeneo yote → sauti | SALAMA KWA SAUTI |
| global_* | 512 | .global_fail_pops → sauti | SALAMA KWA SAUTI |
| local_* | 512 | ukaguzi mpya → sauti (haukuwepo) | SALAMA KWA SAUTI (ilirekebishwa) |
| loop_break_label[16] | 16 | haijatumika kamwe (mabaki) | SALAMA KWA UTHIBITISHO (haijaandikwa) |
| break_fixup_pos | 65,536 | ulikuwa 256 kimya → sasa sauti | SALAMA KWA SAUTI (ilirekebishwa) |
| continue_fixup_pos | 65,536 | kama hapo juu | SALAMA KWA SAUTI (ilirekebishwa) |
| muundo_* | 64 | ilikuwa .fail kimya → sasa sauti | SALAMA KWA SAUTI (ilirekebishwa) |
| nyuga_* | 512 | ilikuwa .fail_pop kimya → sasa sauti | SALAMA KWA SAUTI (ilirekebishwa) |
| kazi_ret_* | 256 | ukaguzi mpya → sauti (haukuwepo) | SALAMA KWA SAUTI (ilirekebishwa) |
| gen_label_pos | 1024 | ukaguzi mpya → sauti (haukuwepo; kikomo kilikuwa 128) | SALAMA KWA SAUTI (ilirekebishwa) |
| gen_fixup_* | 16,384 | .overflow → sauti | SALAMA KWA SAUTI |
| tmp_buf | 4096 | maandishi ≤ baiti 8 | SALAMA KWA UTHIBITISHO (ukubwa wa maandishi umepangwa) |

## msingi/*.swa (maktaba)

| Mgawanyo | Ukubwa | Ulinzi | Uamuzi |
|---|---|---|---|
| chanzo_buf (stage1) | 1 MB | "chanzo ni kikubwa mno" kwa sauti | SALAMA KWA SAUTI |
| kazi_jina_pool (mkaguzi) | 16,384 | ulinzi mpya wa mpaka → sauti | SALAMA KWA SAUTI (ilirekebishwa) |
| kazi_jina_off[1024] (mkaguzi) | 1024 | kazi_idadi >= 1024 → rudisha kimya | PENGO (dhaifu: kirukaji kimya; haiwezi kufikiwa kwa sasa — kazi ~350) |
| kazi_param_enc[4096] (mkaguzi) | 4096 | kama < 4096 tu | PENGO (kama hapo juu) |
| ast_*[65536] (msambazaji) | 65,536 | inachapisha + rudisha -1 | PENGO (dhaifu: inachapisha lakini inaendelea) |
| ast_pool (msambazaji) | 4 MB | inachapisha + rudisha | PENGO (kama hapo juu) |
| bafa_text/bafa_data (uzalishaji) | 256 KB/256 KB | kuangaliwa | BADO (kazi ya kufuatilia) |
| majedwali ya lebo/fixup/rela (uzalishaji) | 2048/…/65536 | kuangaliwa | BADO (kazi ya kufuatilia) |
| dimbwi za orodha/ramani | — | kuangaliwa | BADO (kazi ya kufuatilia) |

## Nini hakikufanyiwa ukaguzi kamili bado

Majedwali ya ndani ya uzalishaji.swa (lebo, fixup, rela, sehemu,
desimali, var_pool, nje_pool, str_buf) na miundo ya orodha.swa/
ramani.swa yameorodheshwa hapa lakini uchunguzi wao wa kina ni kazi
ya kufuatilia. Mbegu (mzizi wa uaminifu) IMEKAGULIWA KAMILI.

## Masomo

1. **Ukaguzi huu uligundua kufurika HALISI**: majedwali ya vunja/
   endelea (256) yalifurika kwenye mkusanyiko mzima — na kabla ya
   ulinzi, kufurika huko kulikuwa kimya (kurukaji kimya = kuruka
   mahali pasiposahihi kwenye njia za makosa). CI haikugundua kwa
   sababu inathibitisha fixpoint, si usahihi wa kila njia ya msimbo.
2. **Ukaguzi wa kwanza wa "abort za sauti" ulikuwa na mdudu wa
   makutano** (njia ya kawaida iliangukia kwenye lebo ya abort) —
   darasa lilelile lililotokea wakati wa PR #151. Kanuni ya nne ya
   sheria ya mzizi wa uaminifu (angalia fallthrough ya kila lebo)
   inathibitishwa tena. Jaribio la kabla ya kugandisha lililipata.
3. Majedwali yote ya mbegu sasa yana ukaguzi unaolia au uthibitisho
   ulioandikwa — darasa la mdudu wa "ukubwa wa kukisia" limefungwa
   kwenye mzizi wa uaminifu.
