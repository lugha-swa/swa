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

# Hatua 2: maoni hujenga lebo kutoka .hexc; lebo lazima ijijenge yenyewe
# sawa kwa baiti kutoka chanzo chake cha lebo (.hex2 -- matawi 61 kwa
# majina), kisha kila kipengele na kila kosa kwa baiti kamili.
"$TMP/maoni" < $M/hatua2_lebo.hexc > "$TMP/lebo"
chmod +x "$TMP/lebo"
"$TMP/lebo" < $M/hatua2_lebo.hex2 > "$TMP/lebo2"
kagua cmp -s "$TMP/lebo" "$TMP/lebo2"
lebo_ni() {  # $1 chanzo, $2 kutoka tarajiwa, $3 baiti tarajiwa (hex)
    printf '%s\n' "$1" | "$TMP/lebo" > "$TMP/o" 2> /dev/null
    local rc=$? got
    got=$(od -An -tx1 "$TMP/o" | tr -s ' ' | sed 's/^ //;s/ $//')
    [ "$rc" = "$2" ] && [ "$got" = "$3" ]
}
kagua lebo_ni ':a 90 &a' 0 "90 00 00 40 00"
kagua lebo_ni ':x 90 e9 %x' 0 "90 e9 fa ff ff ff"
kagua lebo_ni 'eb !y 90 :y' 0 "eb 01 90"
kagua lebo_ni 'e8 %f 90 :f c3' 0 "e8 01 00 00 00 90 c3"
kagua lebo_ni '# abc def
90 ; fe' 0 "90"
kagua lebo_ni 'e9 %hakuna' 2 ""
kagua lebo_ni "eb !mbali $(printf '90 %.0s' $(seq 1 200)):mbali" 3 ""

echo "===== Matokeo ya mnyororo huru: $PASS yamefaulu, $FAIL yameshindwa ====="
[ "$FAIL" -eq 0 ]
