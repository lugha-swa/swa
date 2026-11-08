<div align="center">
  <p>
    <a href="https://lugha-swa.github.io">
      <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/lugha-swa/.github/main/rasilimali/mabango/swa-giza.png">
        <img width="100%" src="https://raw.githubusercontent.com/lugha-swa/.github/main/rasilimali/mabango/swa-mwanga.png" alt="Swa — lugha ya kupanga ya Kiswahili inayojikusanya yenyewe">
      </picture>
    </a>
  </p>

[Tovuti](https://lugha-swa.github.io) | [Jifunze](https://lugha-swa.github.io/jifunze.html) | [Marejeo](https://lugha-swa.github.io/marejeo.html) | [Sakinisha](https://lugha-swa.github.io/sakinisha.html) | [Habari](https://lugha-swa.github.io/habari.html) | [Jamii](https://lugha-swa.github.io/jamii.html)

<br>

<a href="https://github.com/lugha-swa/swa/actions/workflows/ci.yml"><img src="https://github.com/lugha-swa/swa/actions/workflows/ci.yml/badge.svg" alt="Majaribio"></a>
<a href="https://github.com/lugha-swa/swa/releases/latest"><img src="https://img.shields.io/github/v/release/lugha-swa/swa?label=toleo&color=A34E14" alt="Toleo"></a>
<a href="#leseni"><img src="https://img.shields.io/badge/leseni-MIT%20%2F%20Apache--2.0-6B5E4A" alt="Leseni"></a>
<a href="https://github.com/lugha-swa/swa/discussions"><img src="https://img.shields.io/github/discussions/lugha-swa/swa?label=majadiliano&color=6B5E4A" alt="Majadiliano"></a>

</div>
<br>

**Swa** ni lugha ya kupanga ya mifumo yenye sintaksia kamili ya Kiswahili. Mkusanyaji wake umeandikwa kwa Swa yenyewe na hutoa ELF ya x86-64 moja kwa moja — bila LLVM, bila assembler, bila kiunganishi, bila libc. Mnyororo mzima wa kujikusanya huanzia **baiti 393 za mkono** na hujifunga hadi `stage2 == stage3`, sawa kwa baiti.

Swa hiyo hiyo inakusanya pia kwenda **WebAssembly** (kivinjari) na kwenda **ring 0** ([Kawira](https://github.com/lugha-swa/kawira), kernel iliyoandikwa kwa Swa).

## Nyaraka

Mwongozo kamili wa lugha uko kwenye [lugha-swa.github.io](https://lugha-swa.github.io). Hapa chini ni mwanzo wa haraka.

<details open>
<summary><b>Sakinisha</b></summary>
<br>

Toleo lililojengwa tayari (Linux x86-64) liko kwenye [matoleo](https://github.com/lugha-swa/swa/releases/latest): fungua tarball popote, `bin/swa` hupata `msingi/maktaba/` iliyo kando yake. Au kwa Docker:

```sh
docker run --rm -v "$PWD:/kazi" ghcr.io/lugha-swa/swa --exe p.swa > p
```

Kujenga kutoka chanzo, mahitaji ni Linux ya x86-64 pekee -- hakuna gcc, ld, nasm, libc wala bash:

```sh
git clone https://github.com/lugha-swa/swa && cd swa
./msingi/mjenzi.bin gharama/jenga.mapishi   # baiti 393 -> kusanyiko -> mbegu -> stage1 -> bin/swa
```

Kwa anayetaka kuthibitisha mzizi bila kuamini `msingi/mjenzi.bin`: `./msingi/kizinduzi.bin gharama/mwanzo.kizinduzi` hujenga mjenzi kutoka chanzo chake kwa mbegu mbili za mkono pekee (`kwanza.bin` na `kizinduzi.bin`), kisha hulinganisha kila binary iliyogandishwa ([hati/mnyororo-huru.md](hati/mnyororo-huru.md)).

`jenga.mapishi` huanzia `msingi/kwanza.bin` (baiti 393 zilizoandikwa kwa mkono): kila hatua ya mnyororo huru hujengwa na iliyo chini yake, kisha `mbegu`, kisha `stage1`, kisha `stage1` hujijenga yenyewe kuwa `bin/swa`. `mjenzi` (kiendeshaji cha mapishi, `msingi/mnyororo/mjenzi.swm`) yenyewe ni programu ya lugha ya mashine ya Swa iliyojengwa kwa mnyororo huo huo; mapishi huthibitisha kwamba `msingi/mjenzi.bin` ni sawa kwa baiti na inayojengwa kutoka chanzo chake.

</details>

<details open>
<summary><b>Tumia</b></summary>
<br>

[`mifano/salamu.swa`](mifano/salamu.swa):

```swa
husisha { msingi/maktaba/kumbukumbu.swa }

muundo Nukta {
    n32 x;
    n32 y;
};

n32 umbali_mraba(Nukta* a, Nukta* b) {
    n32 dx = a->x - b->x;
    n32 dy = a->y - b->y;
    rudisha dx * dx + dy * dy;
}

n32 main() {
    Nukta* a = tenga(ukubwa(Nukta));
    Nukta* b = tenga(ukubwa(Nukta));
    a->x = 1;  a->y = 2;
    b->x = 4;  b->y = 6;

    andika("Habari, %s!\n", "Dunia");
    andika("umbali^2 = %d\n", umbali_mraba(a, b));

    n32 i = 0;
    wakati (i < 3) {
        kama (i % 2 == 0) { andika("%d ni shufwa\n", i); }
        sivyo             { andika("%d ni witiri\n", i); }
        i = i + 1;
    }
    rudisha 0;
}
```

```sh
bin/swa --exe mifano/salamu.swa > salamu && chmod +x salamu && ./salamu
```

```
Habari, Dunia!
umbali^2 = 25
0 ni shufwa
1 ni witiri
2 ni shufwa
```

Mfano huu hukusanywa na kuendeshwa kwenye CI kila PR (`gharama/jaribu-mifano.mapishi`) — msimbo ulio hapa juu hauwezi kupitwa na wakati kimya.

Chini ya Swa yenyewe kuna lugha ya mashine ya Swa ([`hati/kusanyiko-x86.md`](hati/kusanyiko-x86.md)). Mfano wake: [`mifano/mkusanyaji_mdogo.swm`](mifano/mkusanyaji_mdogo.swm), mkusanyaji mdogo kamili (vigeu vya `n32`, `rudisha` ya `+ - *`) -- `./jengo/kusanyiko < mifano/mkusanyaji_mdogo.swm > m` baada ya `jenga.mapishi`, kisha `./m 'n32 main() { rudisha 42; }' > p`.

</details>

<details>
<summary><b>Malengo mengine</b></summary>
<br>

| Lengo | Amri | Matokeo |
|---|---|---|
| Linux x86-64 | `bin/swa --exe p.swa > p` | ELF inayojitegemea |
| WebAssembly | `bin/swa --wasm p.swa > p.wasm` | moduli ya `.wasm` kwa kivinjari; [`msingi/kivinjari/swa-kivinjari.js`](msingi/kivinjari/swa-kivinjari.js) huiunganisha na DOM na matukio (mfano: [`mifano/kivinjari/`](mifano/kivinjari/)) |
| Ring 0 | `bin/swa --kernel p.swa > p.bin` | msimbo bapa kwa [Kawira](https://github.com/lugha-swa/kawira) (kazi ya kwanza lazima iwe `kernel_main`) |
| IR | `bin/swa --tafsiri p.swa` / `bin/swa --ir p.swa` | tekeleza au chapisha [IR ya mstari](hati/ir.md) -- msingi wa ARM64 na Mizani |

</details>

## Lugha kwa ufupi

| Kipengele | Maelezo |
|---|---|
| **Maneno muhimu 13** | `muundo` `rudisha` `kama` `sivyo` `wakati` `kwa` `fanya` `vunja` `endelea` `chagua` `hali` `husisha` `achilia` — mengine yote ni maktaba |
| **Aina** | familia tano: `n` (nambari), `a` (bila ishara), `d` (desimali), `b` (mifumo ya biti), `w` (tupu) — upana 8/16/32/64, mf. `n32`, `d64`, `b8`, `w0`; pia `b1` (biti moja) |
| **Biti** | familia `b` ni bendera na maski: `& \| ^ ~ << >> == !=` pekee — hesabu na mpangilio (`+ - * / %`, `< >`) hazina maana kwa biti na zinakataliwa kwa sauti |
| **Kumbukumbu** | `tenga` / `achilia` / `badili` — moja kwa moja, bila ukusanyaji taka |
| **Moduli** | `husisha { njia/ya/faili.swa }` — hutatuliwa na mkusanyaji; mzunguko hukataliwa kwa sauti |
| **Kanuni** | programu hukusanywa sahihi **au** hukataliwa kwa sauti. Jibu baya la kimya ni mdudu |

## Hali ya mradi

| Kipimo | Hali |
|---|---|
| Mnyororo wa majaribio (mbegu + stage1, fixpoint) | [yote yanapita](https://github.com/lugha-swa/swa/actions/workflows/ci.yml) |
| Mkimbiaji wa majaribio wa Swa asili (bila bash) | [yote yanapita](https://github.com/lugha-swa/swa/actions/workflows/ci.yml) |
| WebAssembly | [yote yanapita](https://github.com/lugha-swa/swa/actions/workflows/ci.yml) |
| `--kernel` (ring 0) | [yote yanapita](https://github.com/lugha-swa/swa/actions/workflows/ci.yml) |
| IR (mtafsiri dhidi ya msimbo wa mashine) | programu 161 sawa kabisa ([hati/ir.md](hati/ir.md)) |
| Kujikusanya | `stage2 == stage3`, sawa kwa baiti |
| Mbegu (mzizi wa uaminifu) | hujengwa na kujaribiwa kabla ya kila kugandishwa |

Mipaka inayojulikana imeandikwa wazi kwenye [hati/mipaka.md](hati/mipaka.md); ramani iko kwenye [hati/ramani.md](hati/ramani.md).

## Muundo wa hazina

| Njia | Maelezo |
|---|---|
| [`msingi/kwanza.bin`](msingi) | baiti 393 za mkono — mwanzo wa mnyororo |
| [`msingi/kizinduzi.bin`](msingi) | baiti 839 za mkono — mbegu ya pili: huendesha hatua za mwanzo hadi mjenzi ujengwe kutoka chanzo chake ([`kizinduzi.hexc`](msingi/kizinduzi.hexc)) |
| [`msingi/mbegu.swm`](msingi) | mbegu — mkusanyaji mdogo uliogandishwa, kwa lugha ya mashine ya Swa; hujengwa na mnyororo huru |
| [`msingi/mkusanyaji/`](msingi/mkusanyaji) | msomaji, mchanganuzi, mkaguzi, uzalishaji (x86-64 na WASM) |
| [`msingi/maktaba/`](msingi/maktaba) | kumbukumbu, mfuatano, orodha, ramani, faili, hesabu, takwimu, ... |
| [`majaribio/`](majaribio) | majaribio ya programu, WASM na kernel |
| [`gharama/`](gharama) | ujenzi na wakimbiaji wa majaribio |
| [`hati/`](hati) | uthibitisho wa lugha, mipaka, ramani |

## Ikolojia

| Hazina | Maelezo |
|---|---|
| [lugha-swa/swa](https://github.com/lugha-swa/swa) | mkusanyaji na maktaba ya msingi — hazina hii |
| [lugha-swa/kawira](https://github.com/lugha-swa/kawira) | Swa kwenye ring 0 — kernel inayoanzisha, iliyoandikwa kwa Swa |
| [lugha-swa/uchapaji-swa](https://github.com/lugha-swa/uchapaji-swa) | msomaji wa TrueType na kijazaji cha glyph, bila FreeType |
| [lugha-swa/tree-sitter-swa](https://github.com/lugha-swa/tree-sitter-swa) | sarufi ya Tree-sitter — rangi za sintaksia kwenye vihariri |
| [lugha-swa.github.io](https://github.com/lugha-swa/lugha-swa.github.io) | tovuti na mwongozo wa lugha |

## Changia

Michango inakaribishwa — kila kitu kwenye mradi (msimbo, maoni, commits, PR) kiko kwa Kiswahili. Soma [CONTRIBUTING.md](CONTRIBUTING.md), kisha angalia [masuala ya waanzishaji](https://github.com/lugha-swa/swa/labels/good-first-issue). Mabadiliko yote huingia kupitia PR; `main` inalindwa na CI lazima iwe kijani.

Ripoti mdudu kwenye [Issues](https://github.com/lugha-swa/swa/issues), uliza swali kwenye [Discussions](https://github.com/lugha-swa/swa/discussions). Masuala ya usalama: [SECURITY.md](SECURITY.md).

## Leseni

Kwa chaguo lako, [Apache 2.0](LICENSE-APACHE) au [MIT](LICENSE-MIT).

<br>
<div align="center">
  <a href="https://github.com/lugha-swa"><img src="https://raw.githubusercontent.com/lugha-swa/.github/main/rasilimali/nembo/nembo-64.png" width="28" alt="lugha-swa"></a>
  <br>
  <sub>lugha-swa · Afrika ya Mashariki</sub>
</div>
