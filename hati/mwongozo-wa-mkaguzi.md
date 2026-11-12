# Mwongozo wa mkaguzi: mzizi wa mfululizo huru

Swa yote hujengwa kutoka programu mbili za mkono. Kila kinachofuata hujengwa
na hatua iliyo chini yake, na kila hatua hujijenga yenyewe sawa kwa baiti --
lakini hilo linathibitisha uthabiti, si uaminifu. Kitu pekee kinachothibitisha
kwamba hakuna kilichofichwa ni **mtu kusoma baiti**. Mwongozo huu unaeleza
nini cha kusoma, kwa mpangilio gani, na jinsi ya kukagua kila dai bila
kuamini zana zetu.

Hadi sasa hakuna mtu aliyefanya ukaguzi huu kamili. Ukiufanya, andika
matokeo yako (hata "nimekagua X, sikuona tatizo") kwenye issue ya hazina.

## Kinachohitaji kusomwa

| # | Programu | Baiti | Faili la kusoma | Wito wa mfumo (strace) |
|---|---|---|---|---|
| 0 | `kwanza` (heksa -> binary) | 393 | `hati/kwanza-baiti.md`, `msingi/kwanza.heksa` | read, write, exit |
| 0b | `kizinduzi` (kiendeshaji cha hatua) | 839 | `msingi/kizinduzi.maoni` (chanzo: `msingi/mfululizo/kizinduzi.swm`) | read, write, open, dup2, fork, wait4, execve, mkdir, exit |
| 1 | `maoni` (heksa + maoni) | 400 | `msingi/mfululizo/hatua1_maoni.maoni` | read, write, exit |
| 2 | `mahali` (mahali na matawi) | 730 | `msingi/mfululizo/hatua2_mahali.maoni` | read, write, exit |
| 3 | `kusanyiko0` (makro) | 1019 | `msingi/mfululizo/hatua3_kusanyiko0.mahali` | read, write, exit |

Jumla: baiti 3381. Mbegu mbili (0 na 0b) ndizo binary pekee zilizogandishwa
kwenye mzizi; hatua 1-3 hujengwa kutoka faili za maandishi wakati wa kila
ujenzi. Baada ya hatua 3, msimbo umeandikwa kwa makro (`.kus`) na kisha kwa
lugha ya mashine ya Swa (`.swm`) -- unasomeka kama programu, si baiti.

## Mpangilio unaopendekezwa

1. **`kwanza`** -- ndogo kuliko zote, na kila kitu kinaitegemea.
   `hati/kwanza-baiti.md` inaeleza kila sehemu ya utangulizi na msimbo.
2. **`kizinduzi`** -- huendesha hatua zote za mwanzo. Kosa hapa lingeweza
   kubadilisha matokeo ya hatua yoyote.
3. **`maoni`, `mahali`, `kusanyiko0`** -- kila moja huongeza kipengele kimoja
   juu ya iliyotangulia.

## Kwa kila programu, kagua

- **Utangulizi wa ELF:** kiingilio (`e_entry`), PT_LOAD moja, `p_filesz` sawa na
  ukubwa wa faili, `p_memsz` -- vyombo vyote viko ndani yake.
- **Kila agizo:** baiti kwenye mstari zinasimba agizo lililoandikwa kwenye
  maoni yake (rejea: Intel SDM Juzuu 2, au disassembler yoyote kama mkaguzi).
- **Wito wa mfumo:** kabla ya kila `syscall` (0f 05), thamani ya rax ni
  mojawapo ya zile kwenye jedwali hapo juu -- hakuna mtandao, hakuna
  kubadilisha ruhusa za kumbukumbu, hakuna kuandika mahali pengine.
- **Kumbukumbu:** kila uandishi unaenda kwenye chombo kilichotangazwa kwenye
  maoni ya utangulizi, ndani ya `p_memsz`.
- **Matawi:** kila umbali wa `ruka`/`call` unaelekea mwanzo wa agizo, si
  katikati yake.
- **Mwisho:** hakuna baiti baada ya msimbo na data zilizoelezwa.

## Kukagua bila kuamini zana zetu

Kila amri hapa chini hutumia zana za nje tu kama wakaguzi.

```sh
# baiti za faili == baiti za heksa iliyoelezwa (bila programu zetu;
# au `xxd -r -p msingi/kwanza.heksa | cmp - msingi/kwanza.bin`)
python3 -c 'import sys; print(bytes.fromhex(open(sys.argv[1]).read()) == open(sys.argv[2],"rb").read())' msingi/kwanza.heksa msingi/kwanza.bin
python3 -c 'import sys; print(bytes.fromhex(open(sys.argv[1]).read()) == open(sys.argv[2],"rb").read())' msingi/kizinduzi.heksa msingi/kizinduzi.bin

# disassembly huru ya msimbo (kiingilio 0x400078)
objdump -D -b binary -m i386:x86-64 --adjust-vma=0x400000 \
        --start-address=0x400078 msingi/kizinduzi.bin

# wito wa mfumo halisi wakati wa kukimbia
strace -o s.txt msingi/kwanza.bin < msingi/kwanza.heksa > /dev/null
```

Kisha linganisha disassembly na maoni ya `.maoni` mstari kwa mstari.

## Kinachothibitishwa kiotomatiki (hakuhitaji kusomwa)

- `./msingi/kizinduzi.bin gharama/mwanzo.kizinduzi`: hatua zote hujengwa
  kutoka mbegu mbili, kisha mjenzi uliojengwa kutoka chanzo hulinganisha
  kila binary iliyogandishwa (`mjenzi.bin`, `awali.bin`, `kizinduzi.bin` na
  `.heksa` zao). Hatua ya kwanza ya CI.
- `gharama/jaribu-mfululizo-huru.mapishi`: kila hatua hujijenga yenyewe sawa
  kwa baiti, na kila kipengele na kosa la kila hatua lina jaribio.

## Mipaka ya ukaguzi huu

- Mbegu haziwezi kujithibitisha zenyewe: mbegu iliyoharibiwa ndiyo
  inayoendesha ukaguzi wake. Kinga ni ukaguzi wa jicho tu -- ndiyo sababu
  mwongozo huu upo.
- Mfululizo unaendeshwa juu ya kernel ya Linux na CPU ya x86-64. Hizo
  haziko ndani ya wigo huu.
- Baada ya hatua 3, chanzo (`hatua4_kusanyiko.kus`, `kusanyiko.swm`,
  `mbegu.swm`, `mjenzi.swm`) ni maelfu ya mistari. Kinasomeka, lakini
  ukaguzi wake ni kazi kubwa zaidi; anza na mzizi hapa.
