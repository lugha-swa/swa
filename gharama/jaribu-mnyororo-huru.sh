#!/bin/bash
# jaribu-mnyororo-huru.sh -- thibitisha mnyororo huru kutoka baiti 393.
#
# Kila hatua hujengwa NA hatua iliyo chini yake tu, kuanzia kwanza.bin
# (baiti 393 za mkono). Hakuna assembler, linker wala compiler ya nje.
# Angalia hati/mnyororo-huru.md.
set -u
cd "$(dirname "$0")/.." || exit 1
TMP="$(mktemp -d /tmp/swa-huru-XXXXXX)"
trap 'rm -rf "$TMP"' EXIT
M=msingi/mnyororo
PASS=0; FAIL=0
kagua() { if "$@"; then PASS=$((PASS+1)); else echo "SHINDWA: $*"; FAIL=$((FAIL+1)); fi; }

# Hatua 1: kwanza hujenga maoni kutoka hex safi; maoni lazima ijijenge
# yenyewe sawa kwa baiti kutoka hex yenye maoni (inathibitisha kwamba
# matoleo mawili ya chanzo ni baiti zile zile), na ijenge kwanza.bin.
./msingi/kwanza.bin < $M/hatua1_maoni.hex > "$TMP/maoni"
chmod +x "$TMP/maoni"
"$TMP/maoni" < $M/hatua1_maoni.hexc > "$TMP/maoni2"
kagua cmp -s "$TMP/maoni" "$TMP/maoni2"
"$TMP/maoni" < msingi/kwanza.hex > "$TMP/kwanza2"
kagua cmp -s "$TMP/kwanza2" msingi/kwanza.bin

echo "===== Matokeo ya mnyororo huru: $PASS yamefaulu, $FAIL yameshindwa ====="
[ "$FAIL" -eq 0 ]
