# Kuchangia Mradi wa Swa / Contributing to Swa

Karibu! Swa ni lugha ya programu ya Kiswahili inayojikusanya. Tunafurahi unapotaka kuchangia.

**Lugha:** Michango yote (commits, PR, nyaraka, majadiliano) lazima iwe **kwa Kiswahili**. Hii ni sehemu ya dhamira ya mradi.

---

## Njia za Kuchangia

### 1. Kwa Waanzishaji -- "Good First Issues"

Tafuta lebo `good-first-issue` kwenye [ukurasa wa masuala](https://github.com/lugha-swa/swa/issues). Masuala haya yamechaguliwa kwa wachangiaji wapya:

- Kuongeza maoni ya Kiswahili kwenye msimbo
- Kutafsiri nyaraka
- Kuandika majaribio rahisi
- Kurekebisha makosa ya mkusanyaji

### 2. Kuripoti Hitilafu

Tumia kiolezo cha **Ripoti ya Hitilafu**. Hakikisha umejumuisha:
- Hatua za kuzalisha hitilafu
- Matokeo halisi na yanayotarajiwa
- Mazingira yako (OS, mnyororo uliotumika -- mbegu au stage1, ukubwa wa mbegu.bin, hali ya fixpoint)

### 3. Kupendekeza Vipengele

Tumia kiolezo cha **Ombi la Kipengele**. Kumbuka:
- Swa inalenga kuwa lugha rahisi ya mifumo
- Vipengele vinapaswa kuendana na falsafa ya Kiswahili
- Jadili kwanza kabla ya kuanza kutekeleza

### 4. Kutuma Mabadiliko (Pull Requests)

1. **Fork** repo na unda tawi lako
2. Andika msimbo wako kwa Kiswahili
3. Hakikisha mnyororo mzima unapita: `./msingi/mjenzi.bin gharama/jaribu-mfululizo.mapishi`
4. Tumia ujumbe wa commit kwa Kiswahili
5. Eleza **kwa nini** unafanya mabadiliko, si **nini** tu

---

## Mazingira ya Ujenzi

### Mahitaji
- x86-64 Linux
- bash na zana za kawaida za mfumo (cat, cmp, chmod, mktemp, timeout, grep, md5sum)
- Hakuna kingine: mnyororo wa uzalishaji hauhitaji gcc, ld, clang, libc, Rust, wala LLVM

### Kujenga
```sh
git clone https://github.com/lugha-swa/swa.git
cd swa
./msingi/mjenzi.bin gharama/jenga.mapishi   # baiti 393 -> kusanyiko -> mbegu -> bin/swa
./msingi/mjenzi.bin gharama/jaribu-mfululizo.mapishi   # mnyororo mzima + majaribio
```

### Kujaribu Mkusanyaji
```sh
./msingi/awali.bin --programu mfano.swa > mfano.bin
chmod +x mfano.bin && ./mfano.bin
```

---

## Muundo wa Mradi

| Saraka | Maelezo |
|--------|---------|
| `msingi/` | Mzizi wa uaminifu: kwanza (baiti za mkono), mbegu, bootstrap |
| `msingi/maktaba/` | Maktaba ya kawaida ya Swa (kumbukumbu, mfuatano, orodha, ramani, hesabu, faili) |
| `msingi/mkusanyaji/` | Mkusanyaji wa kujikusanya wa Swa (msomaji, msambazaji, mkaguzi, uzalishaji, mteremko -- IR, mtafsiri, stage1) |
| `majaribio/` | Programu za majaribio za Swa (MANIFEST.txt) |
| `gharama/` | Zana za ujenzi na majaribio |
| `hati/` | Nyaraka za mradi |

Dereva wa zamani wa Rust/LLVM uko kwenye hazina ya kumbukumbu
[lugha-swa/swa-dereva](https://github.com/lugha-swa/swa-dereva).

---

## Sheria ya Mzizi wa Uaminifu

`msingi/awali.swm` na `msingi/kwanza.bin` ni MZIZI WA UAMINIFU wa
mnyororo mzima. Sheria isiyo na mbadala:

**Mabadiliko ya mzizi wa uaminifu hujengwa na Kujaribiwa KABLA ya
kugandishwa -- hakuna ubaguzi.**

Kwa nini: hitilafu kwenye mzizi ni ya darasa baya zaidi -- mbegu
iliyovunjika inaweza bado kutoa fixpoint imara LAKINI mbaya
(stage2 == stage3 == takataka), na mnyororo mzima unaweza kushindwa
kimya kimya. Mfano halisi: uhariri mdogo wa njia ya fixup uliwahi
kufanya kila mkusanyiko kushindwa -- ulipatikana tu kwa sababu jaribio
la wito 1,000 lilikimbizwa kabla ya kugandisha. Jaribio hilo sasa ni
mkazo wa RELA ndani ya `gharama/jaribu-mfululizo.mapishi` (`majaribio/mkazo/rela_1000.swa`) -- la kudumu.

Mpangilio wa lazima:
1. Badilisha `msingi/awali.swm`.
2. `./msingi/mjenzi.bin gharama/awali-mpya.mapishi` -> mbegu ya muda
   `jengo/awali` (mnyororo huru: kwanza.bin -> maoni -> lebo -> kusanyiko0
   -> kusanyiko) -> jaribu
   kwa chanzo KIDOGO (sio mkusanyiko mzima kwanza) na kwa mkusanyiko
   mzima.
3. Jaribio la mkazo la wito wa mbele (angalau 1,000) -- njia za RELA
   na nje zinazopigwa mara nyingi.
4. Baada ya hapo tu: gandisha (`./msingi/mjenzi.bin gharama/gandisha.mapishi`
   huandika `mbegu.bin` + `mbegu.hex` na `mjenzi.bin` + `mjenzi.hex`,
   kisha huthibitisha mnyororo mzima), endesha
   `./msingi/mjenzi.bin gharama/jaribu-mfululizo-huru.mapishi`, na endesha
   `./msingi/mjenzi.bin gharama/jaribu-mfululizo.mapishi` nzima.
5. Tazama `hati/mipaka.md` kwa mipaka inayojulikana ya mbegu.

---

## Falsafa ya Msimbo

1. **Kiswahili kwanza.** Vigeu, kazi, na maoni yote kwa Kiswahili.
2. **Rahisi.** Swa haihitaji kuwa na kila kipengele. Inalenga kuwa mbadala wa C, si C++ au Rust.
3. **Imara.** Hakuna paniki, hakuna tabia isiyotabirika. Kila hitilafu lazima ishughulikiwe.
4. **Inayojikusanya.** Utegemezi wa Rust na LLVM umeondolewa kabisa -- mnyororo wa uzalishaji ni Swa pekee.

---

## Mawasiliano

- [GitHub Discussions](https://github.com/lugha-swa/swa/discussions)
- [GitHub Issues](https://github.com/lugha-swa/swa/issues)

---

*Asante kwa kuchangia Swa!*
