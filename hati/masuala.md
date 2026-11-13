# Masuala Yanayobaki -- Mkusanyaji wa Kujikusanya wa Kiswahili

Hati hii inafuatilia masuala yaliyo wazi na ramani ya kazi. Mipaka
inayojulikana imeandikwa kwa undani kwenye `hati/mipaka.md`.

## Masuala yaliyo wazi

### 1. D64 kwenye wito wa kazi wa awali [IMEFUNGWA -- uthibitisho 2026-08-27]

Imekuwa: awali haijatekeleza ABI ya xmm0-xmm7 kwa wito wa kazi;
programu za awali zenye kazi za D64 zinalia kwa sauti.

Sasa (kilichopimwa 2026-08-27): awali inatekeleza wito wa kazi za
D64 (parameta na kurudisha) kwa usahihi -- ABI yake hutumia uhamisho
wa GP (`hamisha_x r0, xmm0`; `hamisha_x xmm1, r1`), si xmm moja kwa moja,
lakini inafanya kazi. Kilichobaki cha desimali: mpaka wa D64 na
nambari kamili (jibu baya kwa mifululizo yote miwili) na D32
(poromoko). Angalia `hati/mipaka.md` 4c na `hati/uthibitisho-wa-lugha.md`.

### 2. Maneno halisi 64-bit (awali na dereva wa Rust)

Mfululizo wa .swa umerekebishwa 2026-08-25 -- maneno halisi hadi N64
kwa [2^31, 2^63) pekee (mipaka.md 5). Kilichobaki: awali (kujianzisha
pekee) na dereva wa Rust bado zinachanganua maneno halisi kama 32-bit
-- `2147483648` inatafsiriwa kama `-2147483648` (biti zinahifadhiwa,
ishara inaenea); halisi >= 2^63 zinakatwa KIMYA hadi biti 32 kwa
mifululizo yote miwili, na halisi kubwa kama hoja ya wito zinavunjika
(uthibitisho 2026-08-27). Angalia `hati/mipaka.md` 5.

### 3. Hakuna mfululizo unaochakata `husisha { faili.swa }`

Hakuna mkusanyaji (awali WALA mfululizo wa .swa) anayechambua faili
lililotajwa -- faili lazima ziunganishwe kwanza (`cat`). Ahadi ya
zamani kwamba "mfululizo wa .swa unaunga viungo vya ndani" haishikiki
(kilichopimwa 2026-08-27). Angalia `hati/mipaka.md` 8.

### 4. Kizuizi cha BSS kwenye Windows

BSS kubwa kuliko ~47KB inaanguka kwenye uanzishaji -- maalum kwa Windows.
Linux ELF ni safi. Haijatatuliwa.

### 5. Mwisho wa LLVM ni wa majaribio [IMEFUNGWA -- kuhamishiwa kumbukumbu 2026-09-05]

O0 (FastISel) inakataa kazi yenye vizuizi zaidi ya 40; majaribio hutumia
O1. Mfululizo wa uzalishaji ni awali/exe pekee. Dereva wa Rust/LLVM
umehamishiwa hazina ya kumbukumbu lugha-swa/swa-dereva. Angalia
`hati/mipaka.md` 6.

## Ramani ya Kazi

| Kipaumbele | Kazi | Ukubwa | Hali |
|-----------|------|--------|------|
| 1 | ABI ya wito wa D64 kwenye awali | kubwa | IMEFUNGWA (uthibitisho 2026-08-27) |
| 2 | Maneno halisi 64-bit kwenye awali (mfululizo wa .swa umeshafanyika kwa [2^31, 2^63); halisi >= 2^63 na hoja za wito bado wazi) | wastani | wazi |
| 3 | Viungo vya ndani (`husisha`) -- mifululizo yote miwili | wastani | wazi |
| 4 | Mwisho wa LLVM kuwa wa kuaminika (si majaribio) | kubwa | IMEFUNGWA -- dereva kuhamishiwa lugha-swa/swa-dereva (2026-09-05) |
| 5 | Malengo zaidi (ARM64, RISC-V) | kubwa | wazi |

## Kilichorekebishwa (Agosti 2026)

- **Awali -- hesabu za baiti 8**: hitilafu ya FPE kwenye hesabu za N64
  ilikuwa na vyanzo vinne (.load_n64 haikuwa na ruka, hesabu za 32-bit pekee,
  hifadhi ya 32-bit pekee, na kazi_ret_aina iliyowekwa 0). Zote zimetatuliwa
  na awali imegandishwa upya.
- **Literal kamili kama hoja**: halisi (k.m. 1000) ilikuwa inapitishwa kama
  desimali kwenye wito wa kazi. Imerekebishwa.
- **Sintaksia**: maneno `kamasivyo` na `sivyo kama` yameondolewa -- tawi-jingine
  huandikwa kwa kuingiza `kama` ndani ya `sivyo`. Dereva wa Rust na mchanganuzi
  wa kujikusanya zimewianishwa.
- **D64 kwenye wito**: awali sasa inatekeleza wito wa kazi za D64
  (kilichopimwa 2026-08-27) -- kikomo cha awali kimeondolewa.
