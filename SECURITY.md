# Sera ya Usalama / Security Policy

## Matoleo Yanayotumika

Swa bado iko katika hatua ya maendeleo. Toleo la sasa linalotumika ni:

| Toleo | Inatumika? |
|-------|-------------|
| `main` (HEAD) | Ndiyo |

## Kuripoti Athari za Usalama

Ikiwa umegundua athari ya usalama katika mkusanyaji wa Swa, tafadhali **usitumie
GitHub Issues**. Badala yake, tuma barua pepe kwa msimamizi wa mradi.

Maelezo yajumuishe:
- Maelezo ya athari
- Hatua za kuzalisha
- Athari inayowezekana
- Marekebisho yanayopendekezwa (ikiwa yapo)

Tutajibu ndani ya saa 72 na kutoa mrejesho wa awali ndani ya siku 7.

## Wigo wa Usalama

Masuala ya usalama yanayohusiana na:
- Utekelezaji wa msimbo holela (arbitrary msimbo execution) kupitia mkusanyaji
- Uvujaji wa kumbukumbu (kumbukumbu leaks) unaoweza kutumiwa vibaya
- Ufikiaji wa kumbukumbu nje ya mipaka (out-of-bounds kumbukumbu access)
- Hitilafu za aina zinazosababisha tabia isiyotabirika

## Mazoea ya Usalama

Mradi huu unafuata mazoea haya:
- Msimbo wote unapitia ukaguzi kabla ya kuunganishwa
- Majaribio yanaendeshwa kiotomatiki kwa kila badiliko
- Hakuna utegemezi wa nje: mkusanyaji hujengwa kutoka baiti 393 za mkono
  (`msingi/kwanza.bin`), bila kusanyiko, kiungo, libc wala bash
  (`hati/mfululizo-huru.md`)
- Mkusanyaji haitekelezi msimbo wa mtumiaji -- inauandaa tu
