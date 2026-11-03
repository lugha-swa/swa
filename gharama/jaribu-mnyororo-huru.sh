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

# Hatua 3: lebo hujenga kusanyiko0 kutoka .hex2; kusanyiko0 lazima
# ijijenge yenyewe sawa kwa baiti kutoka chanzo chake cha makro za
# Kiswahili (.kus -> kusanyiko0 -> lebo), kisha kila kipengele.
"$TMP/lebo" < $M/hatua3_kusanyiko0.hex2 > "$TMP/k0"
chmod +x "$TMP/k0"
"$TMP/k0" < $M/hatua3_kusanyiko0.kus > "$TMP/k0.hex2" && "$TMP/lebo" < "$TMP/k0.hex2" > "$TMP/k02"
kagua cmp -s "$TMP/k0" "$TMP/k02"
k0_ni() {  # $1 chanzo, $2 kutoka tarajiwa, $3 baiti tarajiwa (hex)
    printf '%s\n' "$1" | "$TMP/k0" > "$TMP/ko" 2> /dev/null
    local rc=$? got
    if [ "$rc" = 0 ]; then "$TMP/lebo" < "$TMP/ko" > "$TMP/o"; rc=$?; else : > "$TMP/o"; fi
    got=$(od -An -tx1 "$TMP/o" | tr -s ' ' | sed 's/^ //;s/ $//')
    [ "$rc" = "$2" ] && [ "$got" = "$3" ]
}
kagua k0_ni 'rudi
FAFANUA rudi c3' 0 "c3"
kagua k0_ni '!5 !-1 %258 ^-2' 0 "05 ff 02 01 00 00 fe ff ff ff ff ff ff ff"
kagua k0_ni '"Habari"' 0 "48 61 62 61 72 69"
kagua k0_ni ':a FAFANUA ruka8 eb
ruka8 !a' 0 "eb fe"
kagua k0_ni 'FAFANUA x 90 90 ; maoni
x' 0 "90 90"
kagua k0_ni 'hamisha_r9,r10' 4 ""
kagua k0_ni '%12x' 4 ""

# Hatua 4: kusanyiko0 + lebo hujenga kusanyiko (lugha ya mashine ya Swa).
"$TMP/k0" < $M/hatua4_kusanyiko.kus > "$TMP/k4.hex2" && "$TMP/lebo" < "$TMP/k4.hex2" > "$TMP/kus"
chmod +x "$TMP/kus"
kus_endesha() {  # $1 jina, $2 kutoka tarajiwa, $3 towe tarajiwa
    "$TMP/kus" < $M/majaribio/$1.swm > "$TMP/$1" || return 1
    chmod +x "$TMP/$1"
    local towe; towe=$("$TMP/$1"); local rc=$?
    [ "$rc" = "$2" ] && [ "$towe" = "$3" ]
}
kagua kus_endesha habari 0 "habari kutoka lugha ya mashine ya Swa"
kagua kus_endesha tawi 42 ""
kagua kus_endesha mbali 7 ""
# Maumbo 107 ya maagizo: msimbo lazima uwe sawa kwa baiti na matokeo
# yaliyohifadhiwa (yalikaguliwa mara moja dhidi ya NASM kama mkaguzi).
kus_maagizo() {
    "$TMP/kus" < $M/majaribio/maagizo.swm > "$TMP/m.elf" || return 1
    local csz; csz=$(od -An -tu8 -j 96 -N 8 "$TMP/m.elf" | tr -d ' ')
    [ "$(dd if="$TMP/m.elf" bs=1 skip=4096 count="$csz" status=none | od -An -tx1 -v | tr -s ' \n' ' ' | sed 's/^ //;s/ $//')" = "$(cat $M/majaribio/maagizo.tarajiwa)" ]
}
kagua kus_maagizo

kagua kus_endesha data2 110 ""

# Hatua 5: mbegu kwa lugha ya mashine ya Swa (mbegu.swm), ikikusanywa na
# kusanyiko, lazima itoe msimbo (.text, baiti 43209 kwenye 0x401000),
# data (baiti 3032 kwenye 0x40c000) na mahali pa kuanzia SAWA KWA BAITI
# na msingi/mbegu.bin -- iliyotengenezwa zamani na NASM + GNU ld.
"$TMP/kus" < $M/mbegu.swm > "$TMP/mbegu_huru"
mbegu_sawa() {
    cmp -s <(dd if="$TMP/mbegu_huru" bs=1 skip=4096 count=43209 status=none) \
           <(dd if=msingi/mbegu.bin bs=1 skip=4096 count=43209 status=none) &&
    cmp -s <(dd if="$TMP/mbegu_huru" bs=4096 skip=12 count=1 status=none | head -c 3032) \
           <(dd if=msingi/mbegu.bin bs=4096 skip=12 count=1 status=none | head -c 3032) &&
    cmp -s <(dd if="$TMP/mbegu_huru" bs=1 skip=24 count=8 status=none) \
           <(dd if=msingi/mbegu.bin bs=1 skip=24 count=8 status=none)
}
kagua mbegu_sawa

echo "===== Matokeo ya mnyororo huru: $PASS yamefaulu, $FAIL yameshindwa ====="
[ "$FAIL" -eq 0 ]
