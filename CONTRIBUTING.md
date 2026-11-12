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
- Mazingira yako (OS, mfululizo uliotumika -- awali au kamili, ukubwa wa awali.bin, hali ya uthabiti)

### 3. Kupendekeza Vipengele

Tumia kiolezo cha **Ombi la Kipengele**. Kumbuka:
- Swa inalenga kuwa lugha rahisi ya mifumo
- Vipengele vinapaswa kuendana na falsafa ya Kiswahili
- Jadili kwanza kabla ya kuanza kutekeleza

### 4. Kutuma Mabadiliko (Pull Requests)

1. **Fork** repo na unda tawi lako
2. Andika msimbo wako kwa Kiswahili
3. Hakikisha mfululizo mzima unapita: `./msingi/mjenzi.bin gharama/jaribu-mfululizo.mapishi`
4. Tumia ujumbe wa commit kwa Kiswahili
5. Eleza **kwa nini** unafanya mabadiliko, si **nini** tu

---

## Mazingira ya Ujenzi

### Mahitaji
- x86-64 Linux
- bash na zana za kawaida za mfumo (cat, cmp, chmod, mktemp, timeout, grep, md5sum)
- Hakuna kingine: mfululizo wa uzalishaji hauhitaji gcc, ld, clang, libc, Rust, wala LLVM

### Kujenga
```sh
git clone https://github.com/lugha-swa/swa.git
cd swa
./msingi/mjenzi.bin gharama/jenga.mapishi   # baiti 393 -> kusanyiko -> awali -> bin/swa
./msingi/mjenzi.bin gharama/jaribu-mfululizo.mapishi   # mfululizo mzima + majaribio
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
| `msingi/` | Chanzo cha uaminifu: kwanza (baiti za mkono), awali, bootstrap |
| `msingi/maktaba/` | Maktaba ya kawaida ya Swa (kumbukumbu, mfuatano, orodha, ramani, hesabu, faili) |
| `msingi/mkusanyaji/` | Mkusanyaji wa kujikusanya wa Swa (msomaji, msambazaji, mkaguzi, uzalishaji, mteremko -- IR, mtafsiri, kamili) |
| `majaribio/` | Programu za majaribio za Swa (ORODHA.txt) |
| `gharama/` | Zana za ujenzi na majaribio |
| `hati/` | Nyaraka za mradi |

Dereva wa zamani wa Rust/LLVM uko kwenye hazina ya kumbukumbu
[lugha-swa/swa-dereva](https://github.com/lugha-swa/swa-dereva).

---

## Sheria ya Chanzo cha Uaminifu

`msingi/awali.swm` na `msingi/kwanza.bin` ni CHANZO CHA UAMINIFU wa
mfululizo mzima. Sheria isiyo na mbadala:

**Mabadiliko ya chanzo cha uaminifu hujengwa na Kujaribiwa KABLA ya
kugandishwa -- hakuna ubaguzi.**

Kwa nini: hitilafu kwenye chanzo cha uaminifu ni ya darasa baya zaidi -- awali
iliyovunjika inaweza bado kutoa uthabiti imara LAKINI mbaya
(kamili2 == kamili3 == takataka), na mfululizo mzima unaweza kushindwa
kimya kimya. Mfano halisi: uhariri mdogo wa njia ya urekebishaji uliwahi
kufanya kila mkusanyiko kushindwa -- ulipatikana tu kwa sababu jaribio
la wito 1,000 lilikimbizwa kabla ya kugandisha. Jaribio hilo sasa ni
mkazo wa RELA ndani ya `gharama/jaribu-mfululizo.mapishi` (`majaribio/mkazo/urekebishaji_1000.swa`) -- la kudumu.

Mpangilio wa lazima:
1. Badilisha `msingi/awali.swm`.
2. `./msingi/mjenzi.bin gharama/awali-mpya.mapishi` -> awali ya muda
   `jengo/awali` (mfululizo huru: kwanza.bin -> maoni -> mahali -> kusanyiko0
   -> kusanyiko) -> jaribu
   kwa chanzo KIDOGO (sio mkusanyiko mzima kwanza) na kwa mkusanyiko
   mzima.
3. Jaribio la mkazo la wito wa mbele (angalau 1,000) -- njia za RELA
   na nje zinazopigwa mara nyingi.
4. Baada ya hapo tu: gandisha (`./msingi/mjenzi.bin gharama/gandisha.mapishi`
   huandika `awali.bin` + `awali.heksa` na `mjenzi.bin` + `mjenzi.heksa`,
   kisha huthibitisha mfululizo mzima), endesha
   `./msingi/mjenzi.bin gharama/jaribu-mfululizo-huru.mapishi`, na endesha
   `./msingi/mjenzi.bin gharama/jaribu-mfululizo.mapishi` nzima.
5. Tazama `hati/mipaka.md` kwa mipaka inayojulikana ya awali.

---

## Falsafa ya Msimbo

1. **Kiswahili kwanza.** Vigeu, kazi, na maoni yote kwa Kiswahili.
2. **Rahisi.** Swa haihitaji kuwa na kila kipengele. Inalenga kuwa mbadala wa C, si C++ au Rust.
3. **Imara.** Hakuna paniki, hakuna tabia isiyotabirika. Kila hitilafu lazima ishughulikiwe.
4. **Inayojikusanya.** Utegemezi wa Rust na LLVM umeondolewa kabisa -- mfululizo wa uzalishaji ni Swa pekee.

---

## Viongozi wa Maeneo

Kila eneo lina kiongozi anayeamua mwelekeo wake na kukagua PR zake. Kandemark huchanganya kwenye `main`.

| Eneo | Kiongozi |
|---|---|
| Swa nzima: lugha, mkusanyaji, `main` | @Kandemark |
| ARM64 ([#321](https://github.com/lugha-swa/swa/issues/321)) | @Sakajara |
| RISC-V 64 | @techlunatic-ctrl |
| WebAssembly na kivinjari | @blackmatrx-code |
| Mfululizo huru na chanzo cha uaminifu | @anzania-btrfs |

Kipengele kipya hutumia kilichopo kabla ya kuongeza kipya: lengo jipya la mashine hujengwa kutoka IR (`msingi/mkusanyaji/mteremko.swa`), si nakala ya `uzalishaji.swa`.

---

## Mawasiliano

- [GitHub Discussions](https://github.com/lugha-swa/swa/discussions)
- [GitHub Issues](https://github.com/lugha-swa/swa/issues)

---

*Asante kwa kuchangia Swa!*
