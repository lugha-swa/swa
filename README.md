# SWA — Lugha ya Kupanga ya Kiswahili

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

- **Maneno muhimu 57** ya Kiswahili — hakuna Kiingereza katika sintaksia
- **Kujitegemea** — mkusanyaji umeandikwa kwa Swa yenyewe (bootstrap)
- **LLVM backend** — inatoa msimbo wa mashine wa majukwaa mbalimbali
- **Mfumo wa aina tuli** — aina 25 za nambari kwa usahihi kamili (N8 hadi N128, A8 hadi A128, D16 hadi D80, B1 hadi B64, W0 hadi W64)
- **Kumbukumbu ya moja kwa moja** — tenga na achilia kumbukumbu wewe mwenyewe, hakuna ukusanyaji taka
- **ABI thabiti** — Swa ABI v1.0 kwa mwingiliano wa lugha

## Muundo wa Mradi

| Njia | Maelezo |
|---|---|
| `src/` | Mkusanyaji wa Rust (msomaji, mchanganuzi, IR, LLVM backend) |
| `msingi/` | Maktaba ya msingi ya kujitegemea kwa Swa |
| `stage1.swa` | Kiendeshi cha bootstrap — huanzisha mnyororo wa kujikusanya |

## Kujenga

**Mahitaji:**
- LLVM 18 (C API)
- Rust (toleo jipya zaidi)
- GCC au Clang (kwa kiunganishi)

```sh
cargo build --release
cargo test
```

## Matumizi

```sh
# Kusanya faili ya Swa
cargo run -- programu.swa

# Kutumia stage1 ya kujitegemea
./stage1.exe msingi/msomaji.swa
```

## Hatua ya Bootstrap

Mkusanyaji wa Swa unajikusanya yenyewe kupitia hatua mbili:

1. **stage1.swa** — kiendeshi kinachopakia maktaba ya `msingi/` na kuchakata faili yoyote ya `.swa`
2. **msingi/** — msomaji, mchanganuzi, kiteremshi, na mkaguzi zilizoandikwa kwa Swa yenyewe

Lengo ni kuondoa utegemezi wa Rust na kuwa na mkusanyaji ulioandikwa kwa Swa pekee.

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
*Imetengenezwa kwa ❤️ katika Afrika ya Mashariki*
