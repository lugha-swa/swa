# SWA -- Lugha ya Kupanga ya Kiswahili

**Swa** ni lugha ya kupanga yenye sintaksia kamili ya Kiswahili. Hakuna neno la Kiingereza linatumika katika sintaksia yake. Inakusanya moja kwa moja hadi msimbo wa mashine kupitia LLVM.

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

## Vipengele

- **Maneno muhimu 42** ya Kiswahili -- hakuna Kiingereza katika sintaksia
- **Kujitegemea** -- mkusanyaji umeandikwa kwa Swa yenyewe (bootstrap)
- **LLVM backend** -- inatoa msimbo wa mashine wa majukwaa mbalimbali (x86, ARM, AArch64, RISC-V)
- **Aina 25 za nambari** -- N8 hadi N128, A8 hadi A128, D16 hadi D80, B1 hadi B64, W0 hadi W64
- **Kumbukumbu ya moja kwa moja** -- tenga, achilia na badili kumbukumbu wewe mwenyewe, hakuna ukusanyaji taka
- **ABI thabiti** -- Swa ABI v1.0 kwa mwingiliano wa lugha
- **Hali ya sasa**: Majaribio 174 yanapita (145 ya maktaba + 28 ya ujumuishaji + 1 wa nyaraka). Alloca-in-loop imerekebishwa. K6 (kujikusanya kamili) inapita. uzalishaji.swa native x86-64 backend inajengwa.

## Muundo wa Mradi

| Njia | Maelezo |
|---|---|
| `src/` | Mkusanyaji wa Rust (msomaji, mchanganuzi, IR, LLVM backend) |
| `msingi/` | Maktaba ya msingi ya kujitegemea kwa Swa |
| `stage1.swa` | Kiendeshi cha bootstrap -- huanzisha mnyororo wa kujikusanya |

## Kujenga

**Mahitaji:**
- LLVM 18+ (C API) -- imejaribiwa kwenye LLVM 22.1 (Arch Linux) na LLVM 18 (Windows)
- Rust (toleo jipya zaidi)
- GCC au Clang (kwa kiunganishi)

```sh
cargo build --release
cargo test          # Majaribio 174: 145 ya maktaba + 28 ya ujumuishaji + 1 wa nyaraka
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
| Majaribio | 174/174 [PASS] |
| Kujikusanya (K6) | Inapita [PASS] |
| Mkusanyaji wa Rust | Inajenga bila hitilafu [PASS] |
| Mchanganuzi wa Swa | Vipengele vyote vya lugha [PASS] |
| Kiteremshi cha Swa | Kinaendelea (40%) [WIP] |
| Mkaguzi wa Swa | Kinaendelea (20%) [WIP] |

## Jumuiya

Tunawakaribisha wachangiaji wote! Hata kama hujui Kiswahili, unaweza kuchangia
kwa kujifunza lugha yetu tukufu na kusaidia kujenga mkusanyaji wa kwanza wa
Kiswahili duniani.

- **[CONTRIBUTING.md](CONTRIBUTING.md)** -- Jinsi ya kuchangia
- **[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)** -- Kanuni za maadili
- **[SECURITY.md](SECURITY.md)** -- Sera ya usalama
- **[SUPPORT.md](SUPPORT.md)** -- Kupata msaada
- **[hati/ramani.md](hati/ramani.md)** -- Ramani ya mradi
- **[GitHub Discussions](https://github.com/Kandemark/swa/discussions)** -- Majadiliano
- **[GitHub Issues](https://github.com/Kandemark/swa/issues)** -- Ripoti za hitilafu na maombi ya vipengele

### Kwa Waanzishaji

Tafuta lebo [`good-first-issue`](https://github.com/Kandemark/swa/labels/good-first-issue).
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
