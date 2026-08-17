# Kuchangia Mradi wa Swa / Contributing to Swa

Karibu! Swa ni lugha ya programu ya Kiswahili inayojikusanya. Tunafurahi unapotaka kuchangia.

**Lugha:** Michango yote (commits, PR, nyaraka, majadiliano) lazima iwe **kwa Kiswahili**. Hii ni sehemu ya dhamira ya mradi. Isipokuwa: majina ya vigeu vya Rust yanaweza kuwa Kiingereza.

---

## Njia za Kuchangia

### 1. Kwa Waanzishaji -- "Good First Issues"

Tafuta lebo `good-first-issue` kwenye [ukurasa wa masuala](https://github.com/lugha-swa/swa/issues). Masuala haya yamechaguliwa kwa wachangiaji wapya:

- Kuongeza maoni ya Kiswahili kwenye msimbo
- Kutafsiri nyaraka
- Kuandika majaribio rahisi
- Kurekebisha maonyo ya mkusanyaji

### 2. Kuripoti Hitilafu

Tumia kiolezo cha **Ripoti ya Hitilafu**. Hakikisha umejumuisha:
- Hatua za kuzalisha hitilafu
- Matokeo halisi na yanayotarajiwa
- Mazingira yako (OS, LLVM, Rust)

### 3. Kupendekeza Vipengele

Tumia kiolezo cha **Ombi la Kipengele**. Kumbuka:
- Swa inalenga kuwa lugha rahisi ya mifumo
- Vipengele vinapaswa kuendana na falsafa ya Kiswahili
- Jadili kwanza kabla ya kuanza kutekeleza

### 4. Kutuma Mabadiliko (Pull Requests)

1. **Fork** repo na unda tawi lako
2. Andika msimbo wako kwa Kiswahili
3. Hakikisha majaribio yote yanapita: `cargo test`
4. Tumia ujumbe wa commit kwa Kiswahili
5. Eleza **kwa nini** unafanya mabadiliko, si **nini** tu

---

## Mazingira ya Ujenzi

### Mahitaji
- LLVM 18+ (inapendekezwa 22)
- Rust (latest stable)
- clang (kwa kuunganisha)
- Optional: Nix (`nix-shell`)

### Kujenga
```sh
git clone https://github.com/lugha-swa/swa.git
cd swa
cargo build --release
cargo test  # Majaribio 174 yanapaswa kupita
```

### Kujaribu Mkusanyaji
```sh
cargo run --release -- --check mfano.swa
cargo run --release -- --llvm mfano.swa
cargo run --release -- mfano.swa -o mfano.o
```

---

## Muundo wa Mradi

| Saraka | Maelezo |
|--------|---------|
| `src/` | Mkusanyaji wa bootstrap wa Rust (lexer, parser, sema, ir, codegen, driver) |
| `msingi/` | Maktaba ya kujikusanya ya Swa (msomaji, msambazaji, mteremko, mkaguzi, kumbukumbu, mfuatano, orodha) |
| `majaribio/` | Majaribio ya Rust na Swa |
| `hati/` | Nyaraka za mradi |

---

## Sheria ya Mzizi wa Uaminifu

`msingi/mbegu.s` na `msingi/kwanza.bin` ni MZIZI WA UAMINIFU wa
mnyororo mzima. Sheria isiyo na mbadala:

**Mabadiliko ya mzizi wa uaminifu hujengwa na Kujaribiwa KABLA ya
kugandishwa — hakuna ubaguzi.**

Kwa nini: hitilafu kwenye mzizi ni ya darasa baya zaidi — mbegu
iliyovunjika inaweza bado kutoa fixpoint imara LAKINI mbaya
(stage2 == stage3 == takataka), na mnyororo mzima unaweza kushindwa
kimya kimya. Mfano halisi: uhariri mdogo wa njia ya fixup uliwahi
kufanya kila mkusanyiko kushindwa — ulipatikana tu kwa sababu jaribio
la wito 1,000 lilikimbizwa kabla ya kugandisha. Jaribio hilo sasa ni
`jaribio_mbegu_mkazo_wito` (majaribio/integration.rs) — la kudumu.

Mpangilio wa lazima:
1. Badilisha `msingi/mbegu.s`.
2. `nasm` + `ld` → mbegu ya muda → jaribu kwa chanzo KIDOGO (sio
   mkusanyiko mzima kwanza) na kwa mkusanyiko mzima.
3. Jaribio la mkazo la wito wa mbele (angalau 1,000) — njia za RELA
   na nje zinazopigwa mara nyingi.
4. Baada ya hapo tu: gandisha (`mbegu.bin` + `mbegu.hex`), thibitisha
   kwa `bash gharama/jenga-kwanza.sh`, na endesha `cargo test` nzima.
5. Tazama `hati/mipaka.md` kwa mipaka inayojulikana ya mbegu.

---

## Falsafa ya Msimbo

1. **Kiswahili kwanza.** Vigeu, kazi, na maoni yote kwa Kiswahili.
2. **Rahisi.** Swa haihitaji kuwa na kila kipengele. Inalenga kuwa mbadala wa C, si C++ au Rust.
3. **Imara.** Hakuna paniki, hakuna tabia isiyotabirika. Kila hitilafu lazima ishughulikiwe.
4. **Inayojikusanya.** Lengo kuu ni kuondoa utegemezi wa Rust kabisa.

---

## Mawasiliano

- [GitHub Discussions](https://github.com/lugha-swa/swa/discussions)
- [GitHub Issues](https://github.com/lugha-swa/swa/issues)

---

*Asante kwa kuchangia Swa!*
