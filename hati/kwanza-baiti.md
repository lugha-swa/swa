# Kwanza — Baiti za Mkono

Kwanza ni mkusanyaji wa kwanza wa Swa: program ya **baiti za mkono**
(bila NASM, bila lugha nyingine) inayobadilisha hex → binary. Ukubwa:
**baiti 393** (kichwa cha ELF 64 + kichwa cha programu 56 + msimbo 273).

Kwanza inajijenga yenyewe na inazalisha `mbegu.bin` kutoka `mbegu.hex`.
Hii inafunga pengo la bootstrap: **hakuna zana ya nje inayohitajika
kuzalisha mbegu.bin tena** — baiti za mkono pekee.

Mnyororo:
```
kwanza.hex (baiti za mkono)  →  kwanza.bin  →  mbegu.hex  →  mbegu.bin
```

Uthibitisho: `gharama/jenga-kwanza.sh`.

## Muundo wa Baiti

### Kichwa cha ELF (baiti 64)

| Baiti | Thamani | Maana |
|---|---|---|
| 00-03 | 7f 45 4c 46 | alama ya ELF |
| 04 | 02 | aina 64-bit |
| 05 | 01 | mpangilio mdogo-kwanza |
| 10-11 | 02 00 | faili linalotekelezwa |
| 12-13 | 3e 00 | mashine ya x86-64 |
| 18-1f | 78 00 40 00 | kuingia: 0x400078 (msimbo unaanza hapo) |
| 20-27 | 40 00 | kichwa cha programu kiko baiti 64 |
| 34-35 | 40 00 | ukubwa wa kichwa cha ELF |
| 36-37 | 38 00 | ukubwa wa kichwa cha programu (56) |
| 38-39 | 01 00 | kichwa kimoja cha programu |

### Kichwa cha programu (baiti 56)

| Baiti | Thamani | Maana |
|---|---|---|
| 00-03 | 01 00 00 00 | sehemu ya kupakia |
| 04-07 | 07 00 00 00 | ruhusa: kusoma + kuandika + kutekeleza |
| 10-17 | 00 00 40 00 | anwani ya kuanza: 0x400000 |
| 32-39 | ukubwa wa faili | baiti za faili (393) |
| 40-47 | 00 09 00 00 | kumbukumbu yote: 0x900 (inajumuisha bafa) |

Kumbukumbu baada ya msimbo ni bafa iliyoandikwa sifuri:
- `0x400300-0x4006ff` — bafa ya kusoma (baiti 1024)
- `0x400700-0x4008ff` — bafa ya kuandika (baiti 512)

### Msimbo (baiti 273, kuanzia 0x400078)

Msimbo unasoma hex kutoka stdin, unaruka herufi zisizo hex
(nafasi, mistari mipya), unachanganya nibble mbili kuwa baiti moja,
na unaandika matokeo kwenye stdout.

Rejesta: `r12` — kielekezi cha bafa ya kuandika; `r13` — nibble ya
kwanza; `r14` — hali ya nibble. (`rcx` na `r11` hazitumiki: syscall
huwaharibu.)

| Baiti | Maana |
|---|---|
| 49 bc 00 07 40 00 00 00 00 00 | r12 = 0x400700 (bafa ya kuandika, mara moja tu) |
| 45 31 ed | r13d = 0 (nibble ya kwanza) |
| 45 31 f6 | r14d = 0 (hali ya nibble) |
| 31 ff | edi = 0 (stdin) |
| 48 be 00 03 40 00 00 00 00 00 | rsi = 0x400300 (bafa ya kusoma) |
| ba 00 04 00 00 | edx = 1024 (ukubwa wa kusoma) |
| b8 00 00 00 00 | eax = 0 (sys_read) |
| 0f 05 | syscall |
| 85 c0 | linganisha eax na 0 |
| 7f 05 | kama kubwa, ruka mbele |
| e9 xx xx xx xx | sivyo, ruka hadi mwisho (EOF) |
| 41 89 c0 | r8d = eax (idadi ya baiti zilizosomwa) |
| 48 89 f3 | rbx = rsi (kielekezi cha kusoma) |
| 4d 85 c0 | kama r8 == 0, rejesha kusoma |
| 75 05 | kama si 0, ruka mbele |
| e9 xx xx xx xx | sivyo, ruka hadi mwanzo (soma tena) |
| 0f b6 03 | eax = baiti ya kuingia |
| 48 ff c3 | rbx++ |
| 49 ff c8 | r8-- |
| 3d 30 00 00 00 | linganisha na 48 ('0') |
| 72 xx | chini — ruka (si hex) |
| 3d 39 00 00 00 | linganisha na 57 ('9') |
| 76 xx | chini au sawa — tarakimu |
| 3d 41 00 00 00 | linganisha na 65 ('A') |
| 72 xx | chini — ruka |
| 3d 46 00 00 00 | linganisha na 70 ('F') |
| 76 xx | chini au sawa — herufi kubwa |
| 3d 61 00 00 00 | linganisha na 97 ('a') |
| 72 xx | chini — ruka |
| 3d 66 00 00 00 | linganisha na 102 ('f') |
| 77 xx | juu — ruka |
| 2d 57 00 00 00 | eax -= 87 (a=10) |
| eb xx | ruka hadi nibble |
| 2d 30 00 00 00 | eax -= 48 (0=0) |
| eb xx | ruka hadi nibble |
| 2d 37 00 00 00 | eax -= 55 (A=10) |
| 4d 85 f6 | kama r14 != 0, changanya |
| 75 xx | ruka hadi changanya |
| 49 89 c5 | r13 = rax (nibble ya kwanza) |
| 41 be 01 00 00 00 | r14d = 1 (hali) |
| eb xx | ruka hadi mzunguko |
| 49 c1 e5 04 | r13 <<= 4 (hamisha nibble) |
| 49 09 c5 | r13 |= rax (unga nibble ya pili) |
| 45 88 2c 24 | [r12] = r13b (toa baiti kamili) |
| 49 ff c4 | r12++ |
| 49 81 fc 00 09 40 00 | linganisha r12 na mwisho wa bafa |
| 72 xx | chini — endelea |
| bf 01 00 00 00 | edi = 1 (stdout) |
| 48 be 00 07 40 00 00 00 00 00 | rsi = 0x400700 |
| ba 00 02 00 00 | edx = 512 (ukubwa wa kuandika) |
| b8 01 00 00 00 | eax = 1 (sys_write) |
| 0f 05 | syscall |
| 49 bc 00 07 40 00 00 00 00 00 | r12 = 0x400700 (anza bafa upya) |
| 4d 31 f6 | r14 = 0 (anza jozi mpya) |
| 4d 31 ed | r13 = 0 |
| e9 xx xx xx xx | ruka hadi mzunguko |
| 4c 89 e2 | rdx = r12 (baiti zilizobaki) |
| 48 81 ea 00 07 40 00 | rdx -= 0x400700 (urefu wa pato) |
| 48 85 d2 | kama hakuna, toka |
| 74 xx | ruka hadi toka |
| bf 01 00 00 00 | edi = 1 (stdout) |
| 48 be 00 07 40 00 00 00 00 00 | rsi = 0x400700 |
| b8 01 00 00 00 | eax = 1 (sys_write) |
| 0f 05 | syscall |
| b8 3c 00 00 00 | eax = 60 (sys_exit) |
| 31 ff | edi = 0 (hali ya kutoka) |
| 0f 05 | syscall |

Kuruka zenye `xx` ni displacements zinazokokotolewa kutoka nafasi za
lebo za ndani — zinaonekana kwenye `kwanza.hex`.

## Historia ya Hitilafu Zilizopatikana Wakati wa Kuandika

1. `syscall` huharibu `rcx` na `r11` — rejesta za hali zilihamishwa
   hadi `r12/r13/r14`.
2. `mov r14d, imm32` inahitaji modrm `be` (si `ba` — hiyo ni edx).
3. Kuruka kwa masharti juu ya `jmp rel32` inahitaji skip ya **baiti 5**
   (si 2): `jg 05; jmp ...`.
4. Uanzishaji wa bafa ya kuandika ufanyike **mara moja tu** — katika
   kila mzunguko wa kusoma ulikuwa unafuta pato lililojazwa.
5. Maoni katika faili la hex hayaruhusiwi kuwa na tarakimu au herufi
   a-f — yangechakatwa kama nibbles. Kwanza.hex ni hex safi.
