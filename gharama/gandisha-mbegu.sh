#!/bin/bash
# gandisha-mbegu.sh -- jenga mbegu kutoka msingi/mbegu.swm kwa mnyororo huru
# (hati/mnyororo-huru.md) na uigandishe kama msingi/mbegu.bin + mbegu.hex.
#
#   kwanza.bin (baiti 393) -> maoni -> lebo -> kusanyiko0 -> kusanyiko
#   kusanyiko < msingi/mnyororo/kusanyiko.swm -> kusanyiko2 (hujijenga lenyewe)
#   kusanyiko2 < msingi/mbegu.swm -> mbegu.bin
#
# Hakuna assembler, linker wala compiler ya nje. Uthibitisho baada ya
# kugandisha: kwanza.bin < mbegu.hex lazima itoe mbegu.bin hiyo hiyo
# (gharama/jenga-kwanza.sh), na gharama/jaribu-mnyororo-huru.sh hukagua
# kwamba mbegu.bin iliyogandishwa == kusanyiko(mbegu.swm).
#
# Matumizi:
#   bash gharama/gandisha-mbegu.sh            gandisha msingi/mbegu.bin + .hex
#   bash gharama/gandisha-mbegu.sh --muda F   andika mbegu ya muda kwenye F tu
#                                             (ya kujaribu KABLA ya kugandisha)
set -eu
cd "$(dirname "$0")/.." || exit 1
TMP="$(mktemp -d /tmp/swa-gandisha-XXXXXX)"
trap 'rm -rf "$TMP"' EXIT

bash gharama/jenga-kusanyiko.sh "$TMP/kus2"
"$TMP/kus2" < msingi/mbegu.swm > "$TMP/mbegu" || {
    echo "hitilafu: kusanyiko limekataa msingi/mbegu.swm" >&2; exit 1; }
chmod +x "$TMP/mbegu"

if [ "${1:-}" = "--muda" ]; then
    cp "$TMP/mbegu" "${2:?matumizi: $0 --muda <faili>}"
    echo "mbegu ya muda: $2"
    exit 0
fi

cp "$TMP/mbegu" msingi/mbegu.bin
bash gharama/mbegu-hadi-hex.sh msingi/mbegu.bin > msingi/mbegu.hex
./msingi/kwanza.bin < msingi/mbegu.hex | cmp -s - msingi/mbegu.bin || {
    echo "hitilafu: kwanza.bin < mbegu.hex haitoi mbegu.bin" >&2; exit 1; }
echo "mbegu imegandishwa: msingi/mbegu.bin ($(wc -c < msingi/mbegu.bin) baiti) + msingi/mbegu.hex"
