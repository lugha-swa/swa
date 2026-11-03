#!/bin/bash
# jenga-kusanyiko.sh <faili> -- jenga kusanyiko (assembler ya lugha ya
# mashine ya Swa) kutoka baiti 393 za mkono pekee, kwa mnyororo huru:
#
#   kwanza.bin -> maoni -> lebo -> kusanyiko0 -> kusanyiko (hatua 4, .kus)
#   kusanyiko < msingi/mnyororo/kusanyiko.swm -> kusanyiko2
#   kusanyiko2 < kusanyiko.swm lazima itoe kusanyiko2 yenyewe (sehemu thabiti)
#
# kusanyiko2 huandikwa kwenye <faili>. Hakuna zana ya nje.
set -eu
cd "$(dirname "$0")/.." || exit 1
LENGO="${1:?matumizi: $0 <faili>}"
TMP="$(mktemp -d /tmp/swa-kusanyiko-XXXXXX)"
trap 'rm -rf "$TMP"' EXIT
M=msingi/mnyororo

./msingi/kwanza.bin < $M/hatua1_maoni.hex > "$TMP/maoni"
chmod +x "$TMP/maoni"
"$TMP/maoni" < $M/hatua2_lebo.hexc > "$TMP/lebo"
chmod +x "$TMP/lebo"
"$TMP/lebo" < $M/hatua3_kusanyiko0.hex2 > "$TMP/k0"
chmod +x "$TMP/k0"
"$TMP/k0" < $M/hatua4_kusanyiko.kus > "$TMP/kus.hex2"
"$TMP/lebo" < "$TMP/kus.hex2" > "$TMP/kus"
chmod +x "$TMP/kus"
"$TMP/kus" < $M/kusanyiko.swm > "$TMP/kus2"
chmod +x "$TMP/kus2"
"$TMP/kus2" < $M/kusanyiko.swm | cmp -s - "$TMP/kus2" || {
    echo "hitilafu: kusanyiko.swm halijijengi sawa" >&2; exit 1; }
cp "$TMP/kus2" "$LENGO"
