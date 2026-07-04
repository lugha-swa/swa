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
- Utekelezaji wa msimbo holela (arbitrary code execution) kupitia mkusanyaji
- Uvujaji wa kumbukumbu (memory leaks) unaoweza kutumiwa vibaya
- Ufikiaji wa kumbukumbu nje ya mipaka (out-of-bounds memory access)
- Hitilafu za aina zinazosababisha tabia isiyotabirika

## Mazoea ya Usalama

Mradi huu unafuata mazoea haya:
- Msimbo wote unapitia ukaguzi kabla ya kuunganishwa
- Majaribio yanaendeshwa kiotomatiki kwa kila commit
- Hakuna utegemezi wa nje isipokuwa LLVM
- Mkusanyaji haitekelezi msimbo wa mtumiaji -- inauandaa tu
