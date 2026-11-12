# Kwanza -- Baiti za Mkono

Kwanza ni mkusanyaji wa kwanza wa Swa: program ya **baiti za mkono**
(bila NASM, bila lugha nyingine) inayobadilisha heksa -> binary. Ukubwa:
**baiti 393** (utangulizi wa ELF 64 + maelezo ya kipande 56 + msimbo 273).

Kwanza inajijenga yenyewe na inazalisha `awali.bin` kutoka `awali.heksa`.
Hii inafunga pengo la bootstrap: **hakuna zana ya nje inayohitajika
kuzalisha awali.bin tena** -- baiti za mkono pekee.

Mfululizo:
```
kwanza.heksa (baiti za mkono)  ->  kwanza.bin  ->  awali.heksa  ->  awali.bin
```

Uthibitisho: `gharama/kusanyiko.mapishi` (kwanza hujijenga yenyewe, kisha mfululizo huru) na `gharama/mfululizo.mapishi`.

## Muundo wa Baiti

### Utangulizi wa ELF (baiti 64)

| Baiti | Thamani | Maana |
|---|---|---|
| 00-03 | 7f 45 4c 46 | alama ya ELF |
| 04 | 02 | aina 64-bit |
| 05 | 01 | mpangilio mdogo-kwanza |
| 10-11 | 02 00 | faili linalotekelezwa |
| 12-13 | 3e 00 | mashine ya x86-64 |
| 18-1f | 78 00 40 00 | kuingia: 0x400078 (msimbo unaanza hapo) |
| 20-27 | 40 00 | maelezo ya kipande yako baiti 64 |
| 34-35 | 40 00 | ukubwa wa utangulizi wa ELF |
| 36-37 | 38 00 | ukubwa wa maelezo ya kipande (56) |
| 38-39 | 01 00 | maelezo ya kipande kimoja |

### Maelezo ya kipande (baiti 56)

| Baiti | Thamani | Maana |
|---|---|---|
| 00-03 | 01 00 00 00 | sehemu ya kupakia |
| 04-07 | 07 00 00 00 | ruhusa: kusoma + kuandika + kutekeleza |
| 10-17 | 00 00 40 00 | anwani ya kuanza: 0x400000 |
| 32-39 | ukubwa wa faili | baiti za faili (393) |
| 40-47 | 00 09 00 00 | kumbukumbu yote: 0x900 (inajumuisha chombo) |

Kumbukumbu baada ya msimbo ni chombo kilichoandikwa sifuri:
- `0x400300-0x4006ff` -- chombo cha kusoma (baiti 1024)
- `0x400700-0x4008ff` -- chombo cha kuandika (baiti 512)

### Msimbo (baiti 273, kuanzia 0x400078)

Msimbo unasoma heksa kutoka stdin, unaruka herufi zisizo heksa
(nafasi, mistari mipya), unachanganya nibble mbili kuwa baiti moja,
na unaandika matokeo kwenye stdout.

Rejesta: `r12` -- kielekezi cha chombo cha kuandika; `r13` -- nibble ya
kwanza; `r14` -- hali ya nibble. (`r1` na `r11` hazitumiki: mfumo
huwaharibu.)

| Baiti | Maana |
|---|---|
| 49 bc 00 07 40 00 00 00 00 00 | r12 = 0x400700 (chombo cha kuandika, mara moja tu) |
| 45 31 ed | r13d = 0 (nibble ya kwanza) |
| 45 31 f6 | r14d = 0 (hali ya nibble) |
| 31 ff | r7d = 0 (stdin) |
| 48 be 00 03 40 00 00 00 00 00 | r6 = 0x400300 (chombo cha kusoma) |
| ba 00 04 00 00 | r2d = 1024 (ukubwa wa kusoma) |
| b8 00 00 00 00 | r0d = 0 (sys_read) |
| 0f 05 | mfumo |
| 85 c0 | linganisha r0d na 0 |
| 7f 05 | kama kubwa, ruka mbele |
| e9 xx xx xx xx | sivyo, ruka hadi mwisho (EOF) |
| 41 89 c0 | r8d = r0d (idadi ya baiti zilizosomwa) |
| 48 89 f3 | r3 = r6 (kielekezi cha kusoma) |
| 4d 85 c0 | kama r8 == 0, rejesha kusoma |
| 75 05 | kama si 0, ruka mbele |
| e9 xx xx xx xx | sivyo, ruka hadi mwanzo (soma tena) |
| 0f b6 03 | r0d = baiti ya kuingia |
| 48 ff c3 | r3++ |
| 49 ff c8 | r8-- |
| 3d 30 00 00 00 | linganisha na 48 ('0') |
| 72 xx | chini -- ruka (si heksa) |
| 3d 39 00 00 00 | linganisha na 57 ('9') |
| 76 xx | chini au sawa -- tarakimu |
| 3d 41 00 00 00 | linganisha na 65 ('A') |
| 72 xx | chini -- ruka |
| 3d 46 00 00 00 | linganisha na 70 ('F') |
| 76 xx | chini au sawa -- herufi kubwa |
| 3d 61 00 00 00 | linganisha na 97 ('a') |
| 72 xx | chini -- ruka |
| 3d 66 00 00 00 | linganisha na 102 ('f') |
| 77 xx | juu -- ruka |
| 2d 57 00 00 00 | r0d -= 87 (a=10) |
| eb xx | ruka hadi nibble |
| 2d 30 00 00 00 | r0d -= 48 (0=0) |
| eb xx | ruka hadi nibble |
| 2d 37 00 00 00 | r0d -= 55 (A=10) |
| 4d 85 f6 | kama r14 != 0, changanya |
| 75 xx | ruka hadi changanya |
| 49 89 c5 | r13 = r0 (nibble ya kwanza) |
| 41 be 01 00 00 00 | r14d = 1 (hali) |
| eb xx | ruka hadi mzunguko |
| 49 c1 e5 04 | r13 <<= 4 (hamisha nibble) |
| 49 09 c5 | r13 |= r0 (unga nibble ya pili) |
| 45 88 2c 24 | [r12] = r13b (toa baiti kamili) |
| 49 ff c4 | r12++ |
| 49 81 fc 00 09 40 00 | linganisha r12 na mwisho wa chombo |
| 72 xx | chini -- endelea |
| bf 01 00 00 00 | r7d = 1 (stdout) |
| 48 be 00 07 40 00 00 00 00 00 | r6 = 0x400700 |
| ba 00 02 00 00 | r2d = 512 (ukubwa wa kuandika) |
| b8 01 00 00 00 | r0d = 1 (sys_write) |
| 0f 05 | mfumo |
| 49 bc 00 07 40 00 00 00 00 00 | r12 = 0x400700 (anza chombo upya) |
| 4d 31 f6 | r14 = 0 (anza jozi mpya) |
| 4d 31 ed | r13 = 0 |
| e9 xx xx xx xx | ruka hadi mzunguko |
| 4c 89 e2 | r2 = r12 (baiti zilizobaki) |
| 48 81 ea 00 07 40 00 | r2 -= 0x400700 (urefu wa pato) |
| 48 85 d2 | kama hakuna, toka |
| 74 xx | ruka hadi toka |
| bf 01 00 00 00 | r7d = 1 (stdout) |
| 48 be 00 07 40 00 00 00 00 00 | r6 = 0x400700 |
| b8 01 00 00 00 | r0d = 1 (sys_write) |
| 0f 05 | mfumo |
| b8 3c 00 00 00 | r0d = 60 (sys_exit) |
| 31 ff | r7d = 0 (hali ya kutoka) |
| 0f 05 | mfumo |

Kuruka zenye `xx` ni displacements zinazokokotolewa kutoka nafasi za
mahali pa ndani -- zinaonekana kwenye `kwanza.heksa`.

## Historia ya Hitilafu Zilizopatikana Wakati wa Kuandika

1. `mfumo` huharibu `r1` na `r11` -- rejesta za hali zilihamishwa
   hadi `r12/r13/r14`.
2. `hamisha r14d, namba32` inahitaji modrm `be` (si `ba` -- hiyo ni r2d).
3. Kuruka kwa masharti juu ya `ruka rel32` inahitaji skip ya **baiti 5**
   (si 2): `ruka_kama_juu_n 05; ruka ...`.
4. Uanzishaji wa chombo cha kuandika ufanyike **mara moja tu** -- katika
   kila mzunguko wa kusoma ulikuwa unafuta pato lililojazwa.
5. Maoni katika faili la heksa hayaruhusiwi kuwa na tarakimu au herufi
   a-f -- yangechakatwa kama nibbles. Kwanza.heksa ni heksa safi.
