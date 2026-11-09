# Kamusi ya istilahi za Swa

Swa ni Kiswahili kote: lugha, mkusanyaji, maktaba, majaribio, mifano, hati.
Kamusi hii huamua neno moja la Kiswahili kwa kila dhana ya kiufundi, ili
kila faili litumie neno lile lile. Ubadilishaji wa majina hufuata kamusi
hii kwa mashine (kitambulisho kwa kitambulisho, nje ya mifuatano na
maoni), na kila hatua huthibitishwa kwa:

- fixpoint ya mkusanyaji (stage2 == stage3);
- kila programu ya majaribio na mifano, `--exe` na `--wasm`, sawa kwa
  baiti na mkusanyaji wa kabla ya kubadilisha;
- vipimo vyote vya `gharama/`.

Kubadilisha jina hakubadilishi maana -- kwa hiyo hakuna baiti ya matokeo
inayoruhusiwa kubadilika.

## Kanuni

1. **Majina yetu ni ya Kiswahili; majina yao ni data.** Jina lililofafanuliwa
   na mtu mwingine -- alama ya maktaba (`cos`, `snprintf`), syscall ya Linux
   (`mmap`), sehemu ya ELF (`.strtab`, `e_phoff`), agizo la WASM
   (`i32.const`), njia ya DOM (`createElement`) -- huingia kama mfuatano
   au kwenye maoni kama marejeo, kamwe kama kitambulisho chetu.
2. **Majina ya umbizo ni majina halisi.** ELF, WASM, JS, DOM, x86, GOT,
   System V ni majina ya viwango, kama "Linux" -- yanabaki. Lakini
   **dhana** za Kiingereza (AST, IR, buffer, offset, immediate) hutafsiriwa.
3. **Rejista na maagizo hutumia lugha ya mashine ya Swa**
   (`hati/kusanyiko-x86.md`): `r0`-`r15` na upana `d`/`w`/`b`
   (`rax` = `r0`, `eax` = `r0d`, `al` = `r0b`, `rcx` = `r1`, `rdx` = `r2`,
   `rbx` = `r3`, `rsp` = `r4`, `rbp` = `r5`, `rsi` = `r6`, `rdi` = `r7`),
   `x0`-`x15` kwa desimali. Jina la Intel hubaki kwenye maoni ya baiti
   (`// 48 89 C7  mov rdi, rax`) -- hiyo ni marejeo ya mwongozo wa Intel,
   si jina letu.
4. **Vifupisho vya herufi moja au mbili** (`p`, `k`, `i`, `j`) si maneno --
   vinabaki. Kifupisho cha neno la Kiingereza (`idx`, `buf`, `tmp`,
   `ptr`) si kifupisho tena: hutumia neno kamili la Kiswahili au kifupisho
   chake kilichoamuliwa hapa.

5. **Mpangilio wa asili wa Kiswahili.** Kichwa kwanza, kisha kiunganishi
   kinachokubaliana na ngeli yake: `idadi_ya_lebo` (si `lebo_idadi`),
   `jina_la_kazi`, `urefu_wa_mfuatano`, `kina_cha_kitanzi`,
   `ofseti_ya_jina_la_kigeu`. Kichwa kinachohesabu huchukua wingi:
   `idadi_ya_vigeu`, `dimbwi_la_majina`. Majina yanayoanza na kitenzi
   (`andika_`, `soma_`, `pata_`) tayari ni ya asili. Viambishi vya nafasi
   ya majina (`kigeni_`, `wasm_`, `mti_`, `jr_`) hubaki mbele:
   `kigeni_idadi_ya_kazi`, `mti_ofseti_ya_jina`.
6. **Neno la lugha halilazimishwi na msimbo.** Mgongano wa majina
   hutatuliwa kwa kubadilisha msimbo, si kwa kuchagua neno lisilo sahihi
   (mf. `toa` ni kutoa; kiambishi cha kutoa msimbo kilihamishwa kuwa
   `zalisha_`).

## Lugha yenyewe

| Kiingereza | Swa | Maelezo |
|---|---|---|
| `main` | `anza` | mlango wa programu -- sawa na `anza:` ya lugha ya mashine |
| `andika_stderr` | `andika_kosa` | kazi ya ndani: andika kwenye mkondo wa makosa (2) |
| `--exe` | `--programu` | ELF inayotekelezeka |
| `--kernel` | `--kiini` | picha ya kiini (Kawira) |
| `--ir` | `--kati` | uwakilishi wa kati |
| `--jit` | `--papo` | tekeleza papo hapo |
| `--wasm`, `--wasm-js` | (zinabaki) | WASM na JS ni majina ya umbizo |
| `stage1.swa` | `kiendeshaji.swa` | dereva wa mkusanyaji (hatua 1 ya mnyororo ni `hatua1_maoni`) |

`main`, `andika_stderr` na `call` zinajulikana na mbegu iliyogandishwa
(`msingi/mbegu.swm`), kwa hiyo sehemu hii hufanywa pamoja na kugandisha
mbegu upya. Kwa kipindi kifupi mkusanyaji hukubali majina yote mawili.

## Dhana za mkusanyaji

| Kiingereza | Swa |
|---|---|
| AST (`ast_`) | `mti_` (mti wa sintaksia) |
| node | `nodi` |
| IR | `kati` |
| enc / encoding | `usimbaji` (kifupisho `usb`) |
| offset / off | `ofseti` |
| buffer / buf | `bafa` |
| pool | `dimbwi` |
| index / idx | `faharasa` (kifupisho `fah`) |
| position / pos | `nafasi` |
| register / reg | `rejista` |
| virtual register / vreg | `rejista_pepe` |
| temporary / tmp | `muda` |
| value / val | `thamani` |
| pointer / ptr | `kielekezi` |
| length / len | `urefu` |
| size / sz | `ukubwa` |
| count | `idadi` |
| max / min | `upeo` / `chini` |
| prev / next / nxt | `iliyotangulia` / `inayofuata` |
| first / last | `kwanza` / `mwisho` |
| start / end | `mwanzo` / `mwisho` |
| old / new | `zamani` / `mpya` |
| lhs / rhs | `kushoto` / `kulia` |
| operand | `hoja` |
| op / operator | `kitendo` |
| expr / expression | `usemi` |
| stmt / statement | `taarifa` |
| cond / condition | `sharti` |
| body | `mwili` |
| then / else | `basi` / `sivyo` |
| default | `chaguo_msingi` |
| block | `kizuizi` |
| init / initializer | `kianzio` |
| decl / declaration | `tangazo` |
| param / parm / arg / args | `kigezo` / `vigezo` (tangazo); `hoja` (wito) |
| field | `sehemu` |
| struct | `muundo` |
| element / elem | `kipengele` |
| array / arr | `safu` |
| string / str | `mfuatano` |
| char / ch | `herufi` |
| text / txt | `maandishi` (sehemu ya ELF `.text`: `msimbo`) |
| literal / lit | `halisi` |
| constant / const | `kisichobadilika` |
| global | `ulimwengu` |
| local / locals | `ndani` / `vya_ndani` |
| stack | `rafu` |
| heap | `lundo` |
| slot | `sloti` |
| label | `lebo` |
| fixup / relocation | `urekebishaji` |
| target / tgt | `lengo` |
| source / src | `chanzo` |
| scratch | `rasimu` |
| flags | `bendera` |
| mask | `kinyago` |
| negative / neg | `hasi` (`ni_hasi` = ni hasi; si `has` ya Kiingereza) |
| positive | `chanya` |
| signed / unsigned | `yenye_ishara` / `bila_ishara` (kwenye maagizo: `_n` / `_a`, kama familia za aina) |
| has (ina) | `ina` (`ina_koma` = ina koma) |
| bits | `biti` |
| bytes | `baiti` |
| raw | `ghafi` |
| content | `maudhui` |
| status | `hali` (kama kitambulisho: `hali_ya`) |
| kind / type | `aina` |
| list | `orodha` |
| map | `ramani` |
| hash | `heshi` |
| entry / entries | `ingizo` / `maingizo` |
| visited | `imetembelewa` |
| pass (kipitishi) | `kipitishi` |
| scan | `chunguza` |
| check / chk | `kagua` |
| fail / bail | `shindwa` / `acha` |
| reset | `weka_upya` |
| copy | `nakala` |
| get / set | `pata` / `weka` |
| load / store | `pakia` / `hifadhi` |
| drop | `tupa` |
| tail | `mkia` |
| ternary | `chaguzi_tatu` |
| binary op / binop | `kitendo_cha_pande_mbili` |
| primary | `msingi` |
| helper | `msaidizi` |
| registry | `sajili` |
| thunk | `kipande` |
| LICM (licm) | `pandisha` (kupandisha kisichobadilika nje ya kitanzi) |
| CSE (cse) | `rudufu` (kuondoa usemi uliorudiwa) |
| inline (inl) | `pachika` |
| sret | `matokeo_ya_muundo` (mf. `kazi_inarudisha_muundo`, `ofseti_ya_bafa_ya_matokeo`) |
| SIMD (simd) | `sambamba` |
| syscall / sys_ | `mfumo` / `mfumo_` (mf. `sys_fork` -> `mfumo_gawanyika`) |
| argc / argv / envp | `idadi_ya_hoja` / `hoja_za_mstari` / `mazingira` |
| fd | `kifafanuzi` (cha faili) |
| pid | `kitambulisho_cha_mchakato` |

## Usimbaji wa maagizo (mkusanyaji)

Kazi za kuandika baiti za maagizo hufuata lugha ya mashine:
`andika_<tendo>_<lengo>_<chanzo>`.

| Agizo la Intel | Tendo la Swa |
|---|---|
| mov (rejista <- rejista) | `hamisha` |
| mov (rejista <- namba) | `weka` |
| mov (rejista <- kumbukumbu) | `soma` |
| mov (kumbukumbu <- rejista) | `hifadhi` |
| movzx / movsx / movsxd | `soma_a` / `soma_n` (panua kwa sifuri / kwa ishara) |
| lea | `anwani` |
| push / pop | `sukuma` / `vuta` |
| call / ret | `ita` / `rudi` |
| jmp / jcc | `ruka` / `ruka_kama` |
| cmp / test | `linganisha` / `pima` |
| add / sub | `jumlisha` / `toa` (kutoa -- neno la hesabu za shule) |
| imul / mul | `zidisha_n` / `zidisha_a` |
| idiv / div | `gawanya_n` / `gawanya_a` |
| cdq | `panua` |
| and / or / xor / not | `na` / `au` / `au_pekee` / `geuza` |
| neg | `hasi` |
| shl / shr / sar | `sogeza_kushoto` / `sogeza_kulia_a` / `sogeza_kulia_n` |
| setcc | `weka_kama` |
| rep movsb | `nakili_baiti` (`nakili` ya lugha ya mashine ni nakili ya baiti) |
| syscall | `mfumo` |
| movsd / movss / movapd / movupd / movd / movq | `soma_d64` / `soma_d32` / `nakili_x` / ... (kwa upana) |
| addsd subsd mulsd divsd (na ss, pd) | `jumlisha_d64` `toa_d64` `zidisha_d64` `gawanya_d64` (`_d32`, `_d64x2`) |
| comisd / comiss | `linganisha_d64` / `linganisha_d32` |
| cvtsi2sd / cvtsi2ss | `geuza_kuwa_d64` / `geuza_kuwa_d32` |
| imm (namba ndani ya agizo) | `namba` (`imm32` -> `namba32`) |
| disp (ofseti ya anwani) | `ofseti` |
| ModRM / SIB / REX | `modrm` / `sib` / `rex` -- majina ya baiti za Intel, kama data |
| rip-relative | `jirani` |

Kazi za kutoa msimbo kwa ngazi ya juu (zamani `toa_`) ni `zalisha_`
(`zalisha_ruka_lebo`), ili `toa` ibaki na maana yake ya hesabu.

Mfano: `andika_mov_rax_rbp_off` -> `andika_soma_r0_r5_ofseti`;
`andika_push_rax` -> `andika_sukuma_r0`; `andika_xor_eax_eax` ->
`andika_au_pekee_r0d_r0d`.

## ELF na WASM

Sehemu za viwango hivi hupewa majina ya Kiswahili kwa maana, na jina la
kiwango kwenye maoni pale linapofafanuliwa:

| Kiwango | Swa |
|---|---|
| phdr / shdr | `kichwa_cha_kipande` / `kichwa_cha_sehemu` |
| strtab / shstrtab | `jedwali_la_majina` / `jedwali_la_majina_ya_sehemu` |
| symtab / sym | `jedwali_la_alama` / `alama` |
| vaddr / filesz / memsz | `anwani_pepe` / `ukubwa_faili` / `ukubwa_kumbukumbu` |
| bss | `tupu` (kama lugha ya mashine) |
| interp / dyn | `kipakiaji` / `kiungo_hai` |
| WASM import / export | `uingizaji` / `uhamishaji` |
| WASM function / element / memory | `kazi` / `kipengele` / `kumbukumbu` |
| WASM local.get / local.set | majina ya maagizo -- mfuatano/maoni; kitambulisho `wasm_kitendo_pata_ndani` |
| LEB128 (uleb, sleb) | `leb_a` / `leb_n` |

## Maneno ya sitiari yaliyobadilishwa kwa maana

Kanuni ya mwanzilishi: hakuna tafsiri ya neno-kwa-neno ya sitiari ya
Kiingereza (`mdudu` <- "bug" ni kosa). Jina hufuata kazi halisi ya kitu.
Kila neno jipya limethibitishwa kwenye kamusi (Wiktionary ya Kiswahili,
inayotokana na TUKI na Johnson), na kila kitambulisho kilikaguliwa kwa
kazi yake -- neno moja la zamani linaweza kupata maneno tofauti.

| Zamani | Sasa | Kazi kwenye msimbo | Kamusi |
|---|---|---|---|
| kitanzi | mzunguko | msimbo unaorudiwa | mzunguko: "cycle, circuit"; kitanzi: "noose" |
| rafu | mrundikano | mwisho-kuingia-kwanza-kutoka | "vitu vilivyowekwa juu ya vingine kwa utaratibu"; rafu: "shelf" |
| sloti | nafasi | nafasi ya thamani moja | nafasi: "space" |
| dimbwi | ghala | hifadhi ya majina | ghala: "storeroom"; dimbwi: pond |
| mnyororo | mfululizo | hatua zinazofuatana | mfululizo: "series" (mlolongo = "queueing" -- imekataliwa) |
| daraja | kiolesura | mawasiliano na msimbo uliokusanywa | "inayowezesha mawasiliano kati ya programu" |
| kinyago | kichujio | huchagua biti | kichujio: "(computing) filter" |
| bendera | machaguo / sifa | jinsi ya kufungua au kupanga / kipande cha ELF ni nini | chaguo: "choice"; sifa: "trait, quality" |
| jani | moja_kwa_moja | hoja isiyohitaji hesabu | -- (sitiari ya mti imeondolewa) |
| kizuizi, bloku | ubeti | kundi la taarifa linalounda kitu kimoja | ubeti (ushairi): kundi la mishororo linaloleta wazo moja |
| kichwa | utangulizi / mwanzo / tangazo | kichwa cha faili / mwanzo wa orodha / tangazo la kazi ya nje | utangulizi: "introduction" |
| mwili | maudhui | taarifa zilizomo kwenye kazi au mzunguko | maudhui: "content" |
| lala | subiri | kusimama kwa muda | subiri: "to wait" |
| ua | batilisha / tuma_ishara / upana | thamani zisizo halali tena / ishara kwa mchakato / upana wa biti | batilisha: "make invalid" |
| kaburi | alama_ya_kufutwa | nafasi iliyofutwa kwenye ramani | alama: "mark, sign" |
| kandida | mgombea | vigeu vinavyoshindania rejista | mgombea: "candidate, contestant" |
| mzizi | nodi_ya_programu / saraka_ya_swa / nodi_kuu | nodi ya programu nzima / saraka ya usakinishaji / nodi ya ukurasa | (mzizi wa mraba wa hisabati unabaki) |
| pandisha | hamisha_nje | hesabu isiyobadilika hutolewa nje ya mzunguko | -- |
| jirani | kutoka_agizo | anwani huhesabiwa kutoka agizo la sasa | -- |
| tawi | upande | upande mmoja wa && / \|\| | -- |
| sikiliza | itikia | kazi huitwa tukio linapotokea | kuitikia: kujibu unapoitwa |
| kizazi | zamu | duru ya kutembelea nodi | -- |
| mtoto, mzazi | nodi_iliyomo, nodi_inayoibeba (mchakato_mpya kwa fork) | nodi ndani ya nodi / mchakato mpya | -- |
| kaunta, selecta | kihesabu, kiteuzi | | -- (maneno yaliyoundwa kutoka hesabu, teua) |
| mti_* (majedwali ya nodi) | aina_ya_nodi, thamani_ya_nodi, nodi_ya_kushoto/kulia, tatu_ya_nodi, nne_ya_nodi, ghala_la_majina, NODI_* | kila ingizo ni nodi moja; tiga/nne ni nafasi za jumla ambazo maudhui yake hutegemea aina ya nodi | -- (sitiari ya mti imeondolewa) |
| lebo_mkia | lebo_mabaki | marudio yaliyobaki baada ya uvekta | -- |

`husisha` linabaki: lina maana ya kuhusisha, na ndivyo linavyofanya --
huleta faili jingine kwenye programu. Yanayofuata kwenye PR zake: `ofseti` -> `umbali`, `lebo` -> `mahali`, `bafa` -> `chombo`,
neno muhimu `vunja` -> `acha`, `mbegu` -> mkusanyaji wa awali, na majina
ya faili (`mende`/`mdudu` -> `hitilafu`).

## Hatua za kazi

1. Kamusi hii.
2. Vitambulisho vya ndani vya `msingi/` (si vile vinavyojulikana na mbegu),
   faili kwa faili -- matokeo sawa kwa baiti.
3. Maktaba (`msingi/maktaba/`), majaribio, mifano.
4. Lugha: `anza`, `andika_kosa`, bendera, `kiendeshaji.swa` -- pamoja na
   kugandisha mbegu upya.
5. Maoni, hati, faili za CI (`ci.yml` -> `ukaguzi.yml`), `dom_mock.js` ->
   `hati_bandia.js`. Faili zinazotakiwa na GitHub kwa jina hilo
   (`README.md`, `LICENSE-*`, `SECURITY.md`, `CONTRIBUTING.md`,
   `.github/ISSUE_TEMPLATE`, `pull_request_template.md`, `FUNDING.yml`)
   hubaki kwa jina -- yaliyomo ni Kiswahili.

Hakuna toleo jipya kabla hatua hizi zote hazijakamilika.
