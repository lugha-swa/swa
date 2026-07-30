# SWA — Lugha ya Kupanga ya Kiswahili

**Swa** ni lugha ya kupanga yenye sintaksia kamili ya Kiswahili. Hakuna neno
la Kiingereza linatumika katika sintaksia yake. Inakusanya moja kwa moja hadi
msimbo wa mashine — kwanza kwa njia asilia (`uzalishaji.swa`) inayotoa ELF
binary moja kwa moja bila LLVM wala mkusanyaji msaidizi, na pia kwa njia ya
LLVM kwa majukwaa zaidi.

Makao rasmi: **[lugha-swa](https://github.com/lugha-swa)**

## Mfano

```swa
husisha C::stdio

W0 salamu(N8* jina) {
    andika("Habari, %s!\n", jina);
}

W0 hesabu_na_onyesha(N32 a, N32 b) {
    N32 jumla = a + b;
    N32 tofauti = a - b;
    andika("%d + %d = %d\n", a, b, jumla);
    andika("%d - %d = %d\n", a, b, tofauti);
}

salamu("Dunia");
hesabu_na_onyesha(15, 7);
```

## Mifano Zaidi

### Vigezo na Aina

```swa
N32 umri = 25;
N64 idadi_ya_watu = 8000000000;
D32 wastani = 3.14;
B1 imewashwa = kweli;
N8 herufi = 'A';
```

### Mtiririko wa Udhibiti

```swa
// Kama/Sivyo
N32 kadirifu(N32 x) {
    kama (x > 0) {
        rudisha 1;
    } sivyo kama (x < 0) {
        rudisha -1;
    } sivyo {
        rudisha 0;
    }
}

// Wakati (kitanzi)
W0 hesabu_hadi(N32 n) {
    N32 i = 0;
    wakati (i < n) {
        andika("%d\n", i);
        i = i + 1;
    }
}

// Chagua (switch)
N32 siku_kwa_namba(N32 n) {
    chagua (n) {
        hali 1: rudisha 100;
        hali 2: rudisha 200;
        hali 3: rudisha 300;
        sivyo: rudisha 0;
    }
}
```

### Miundo

```swa
muundo Nukta {
    N32 x;
    N32 y;
};

N32 pata_x(Nukta p) {
    rudisha p.x;
}

W0 weka_x(Nukta* p, N32 v) {
    p->x = v;
}
```

### Safu na Kumbukumbu

```swa
N8 bafa[1024];              // safu ya ulimwengu
N32 namba[5] = {1, 2, 3, 4, 5};

W0 andika_bafa() {
    bafa[0] = 65;           // andika kwenye safu
    N32 ya_kwanza = namba[0];
}

// Kumbukumbu ya moja kwa moja
W0 mfano_kumbukumbu() {
    N32* p = tenga N32;     // tenga kumbukumbu
    *p = 42;                // andika thamani
    achilia(p);             // achilia kumbukumbu
}
```

### Miito na Urejeshaji

```swa
// Tangazo la mbele
W0 mkuu() {
    msaidizi(42);
}

W0 msaidizi(N32 x) {
    andika("Thamani: %d\n", x);
}

// Kujirudia
N32 kitanzi(N32 n) {
    kama (n <= 0) { rudisha 0; }
    rudisha 1 + kitanzi(n - 1);
}
```

## Vipengele

- **Maneno muhimu 42** ya Kiswahili -- hakuna Kiingereza katika sintaksia
- **Kujitegemea (~85%)** -- mkusanyaji umeandikwa kwa Swa yenyewe (bootstrap)
- **Vizalishe viwili**: LLVM (majukwaa yote) + asilia (x86-64 ELF moja kwa moja)
- **Aina 25 za nambari** — N8–N128, A8–A128, D16–D80, B1–B64, W0–W64 zote zinashughulikiwa
- **Kumbukumbu ya moja kwa moja** -- tenga, achilia, hakuna ukusanyaji taka
- **Majaribio**: 185 yanapita (145 maktaba + 39 ujumuishaji + 1 nyaraka). K6 bootstrap inafanya kazi.

## Muundo wa Mradi

| Njia | Maelezo |
|---|---|
| `src/` | Mkusanyaji wa Rust (msomaji, mchanganuzi, IR, LLVM backend) |
| `msingi/` | Maktaba ya msingi ya kujitegemea kwa Swa — bomba zima |
| `msingi/msomaji.swa` | Msomaji (lexer) — kamili |
| `msingi/msambazaji.swa` | Mchanganuzi (parser) — kamili, nodi 47 za AST |
| `msingi/mkaguzi.swa` | Mkaguzi wa kisemantiki — kamili (aina, hoja, ugawaji) |
| `msingi/mteremko.swa` | Kiteremshi cha AST→IR — kamili |
| `msingi/uzalishaji.swa` | Kizalishe asilia cha x86-64 — kamili (aina zote, sret, alloca) |
| `msingi/ramani.swa` | Jedwali la hashi |
| `msingi/orodha.swa` | Safu inayobadilika |
| `msingi/mfuatano.swa` | Shughuli za mifuatano |
| `msingi/kumbukumbu.swa` | Shughuli za kumbukumbu |
| `gharama/` | Zana za ujenzi na majaribio |

## Kujenga

**Mahitaji:**
- LLVM 18+ (C API) -- imejaribiwa kwenye LLVM 22.1 (Arch Linux) na LLVM 18 (Windows)
- Rust (toleo jipya zaidi)
- GCC au Clang (kwa kiunganishi)

```sh
cargo build --release
cargo test          # Majaribio 185: 145 ya maktaba + 39 ya ujumuishaji + 1 wa nyaraka
```

## Matumizi

```sh
# Kusanya faili ya Swa
cargo run -- programu.swa

# Kutumia stage1 ya kujitegemea
./stage1 msingi/msomaji.swa
```

## Hatua ya Bootstrap

Mkusanyaji wa Swa unajikusanya yenyewe kupitia hatua mbili:

1. **stage1.swa** -- kiendeshi kinachopakia maktaba ya `msingi/` na kuchakata faili yoyote ya `.swa`
2. **msingi/** -- msomaji, mchanganuzi, kiteremshi, na mkaguzi zilizoandikwa kwa Swa yenyewe

Lengo ni kuondoa utegemezi wa Rust na kuwa na mkusanyaji ulioandikwa kwa Swa pekee.

## Hali ya Mradi

| Kipimo | Thamani |
|--------|---------|
| **Majaribio** | 185/185 [PASS] |
| **Kujikusanya (K6)** | Inapita [PASS] |
| **Mchanganuzi wa Swa** | Kamili [DONE] |
| **Mkaguzi wa Swa** | Kamili [DONE] |
| **Kiteremshi cha Swa** | Kamili [DONE] |
| **Kizalishe asilia cha x86-64** | Kamili [DONE] |
| **Usambazaji wa aina** | Aina zote 25 [DONE] |
| **Urejeshaji wa makosa** | Kamili [DONE] |
| **Alloca-in-loop** | Imerekebishwa [DONE] |
| **Sret (struct return)** | Imetekelezwa [DONE] |
| **`--opt` (LLVM passes)** | Inafanya kazi [DONE] |
| **Uhuru wa jumla** | **~85%** |

## Ramani

Angalia **[hati/ramani.md](hati/ramani.md)** kwa mpango kamili.

| Hatua | Maelezo | Hali |
|-------|---------|------|
| 0 | Mkusanyaji wa bootstrap wa Rust | Imekamilika |
| 1 | Kujikusanya kwa msingi | Imekamilika |
| 2 | Mkusanyaji kamili wa kujikusanya | Imekamilika |
| 3 | Ondoa utegemezi wa Rust | Lengo |
| 4 | Ondoa utegemezi wa LLVM | Lengo |
| 5 | Lugha kamili ya mifumo | Baadaye |

## Jumuiya

Tunawakaribisha wachangiaji wote! Hata kama hujui Kiswahili, unaweza kuchangia
kwa kujifunza lugha yetu tukufu na kusaidia kujenga mkusanyaji wa kwanza wa
Kiswahili duniani.

- **[CONTRIBUTING.md](CONTRIBUTING.md)** -- Jinsi ya kuchangia
- **[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)** -- Kanuni za maadili
- **[SECURITY.md](SECURITY.md)** -- Sera ya usalama
- **[SUPPORT.md](SUPPORT.md)** -- Kupata msaada
- **[hati/ramani.md](hati/ramani.md)** -- Ramani ya mradi
- **[GitHub Discussions](https://github.com/lugha-swa/swa/discussions)** -- Majadiliano
- **[GitHub Issues](https://github.com/lugha-swa/swa/issues)** -- Ripoti za hitilafu na maombi ya vipengele

### Kwa Waanzishaji

Tafuta lebo [`good-first-issue`](https://github.com/lugha-swa/swa/labels/good-first-issue).
Masuala haya yameandaliwa mahsusi kwa wachangiaji wapya!

## Leseni

Mradi huu una leseni mbili:

- [Apache 2.0](LICENSE-APACHE)
- [MIT](LICENSE-MIT)

kwa chaguo lako.

## Mchango

Michango inakaribishwa. Tafadhali tumia:

1. Tenga tawi la kipengele (`feat/jina` au `kurekebisha/jina`)
2. Fanya mabadiliko yako
3. Wasilisha ombi la kuvuta (pull request)
4. Hakikisha majaribio yote yanapita

Tawi kuu (`main`) linalindwa. Mabadiliko yote huingia kupitia ombi la kuvuta.

---
*Imetengenezwa Afrika ya Mashariki*
