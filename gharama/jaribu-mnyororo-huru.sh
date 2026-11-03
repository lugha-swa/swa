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

# Hatua 4d: kusanyiko limeandikwa kwa lugha yake (msingi/mnyororo/kusanyiko.swm).
# Kusanyiko la hatua 4 hulijenga (kus2); kus2 lazima lijijenge lenyewe sawa
# kwa baiti (sehemu thabiti) na litoe matokeo yale yale kwa kila jaribio.
"$TMP/kus" < $M/kusanyiko.swm > "$TMP/kus2"
chmod +x "$TMP/kus2"
"$TMP/kus2" < $M/kusanyiko.swm > "$TMP/kus3"
kagua cmp -s "$TMP/kus2" "$TMP/kus3"
kus2_sawa() {  # kila jaribio: towe la kus2 == towe la kus
    local t
    for t in habari tawi mbali data2 maagizo; do
        "$TMP/kus" < $M/majaribio/$t.swm > "$TMP/a" || return 1
        "$TMP/kus2" < $M/majaribio/$t.swm > "$TMP/b" || return 1
        cmp -s "$TMP/a" "$TMP/b" || return 1
    done
    printf 'anza:\n        r0 = r99\n' | "$TMP/kus2" > /dev/null 2> "$TMP/e"
    [ "$?" = 1 ] && [ "$(cat "$TMP/e")" = "kosa 2:3" ]
}
kagua kus2_sawa
# Picha tambarare, biti 32/64 na maagizo ya mfumo (katizo_zima, simama,
# soma_msr, cr0 = r0d, ds = r0w, pakia_gdt, ruka_mbali, pangilia ...):
# baiti zote sawa na matokeo yaliyohifadhiwa (yalikaguliwa mara moja dhidi
# ya NASM). Makosa: REX kwenye biti 32 -> 5, ruka_mbali kwenye biti 64 -> 2.
kus2_mfumo() {
    "$TMP/kus2" < $M/majaribio/mfumo.swm > "$TMP/mf" || return 1
    [ "$(od -An -tx1 -v "$TMP/mf" | tr -s ' \n' ' ' | sed 's/^ //;s/ $//')" = "$(cat $M/majaribio/mfumo.tarajiwa)" ]
}
kagua kus2_mfumo
kus2_kosa() {  # $1 chanzo, $2 kosa tarajiwa
    printf '%s\n' "$1" | "$TMP/kus2" > /dev/null 2> "$TMP/e"
    [ "$?" = 1 ] && [ "$(cat "$TMP/e")" = "$2" ]
}
kagua kus2_kosa 'biti 32
        r0 = r1' "kosa 2:5"
kagua kus2_kosa 'picha 0
        ruka_mbali 8, x
x:' "kosa 2:2"
# Bandari za I/O, & | << >> kwa mpangilio wa C, jumuisha "faili", RUKA
# (sawa na NASM); faili lisilopatikana -> kosa 8.
kus2_bandari() {
    ( cd $M/majaribio && "$TMP/kus2" < bandari.swm ) > "$TMP/bd" || return 1
    [ "$(od -An -tx1 -v "$TMP/bd" | tr -s ' \n' ' ' | sed 's/^ //;s/ $//')" = "$(cat $M/majaribio/bandari.tarajiwa)" ]
}
kagua kus2_bandari
kagua kus2_kosa 'picha 0
        jumuisha "hakuna/faili/hili"' "kosa 2:8"

# Hatua 5: mbegu kwa lugha ya mashine ya Swa (msingi/mbegu.swm),
# ikikusanywa na kusanyiko, lazima iwe SAWA KWA BAITI na msingi/mbegu.bin
# iliyogandishwa -- faili zima, si sehemu tu. Hivyo mbegu.bin (na
# mbegu.hex, ambayo kwanza.bin huigeuza kuwa mbegu.bin hiyo hiyo --
# gharama/mnyororo.mapishi) hutoka kwenye baiti 393 za mkono pekee.
# (Kabla ya kugandishwa upya 2026-11-03, msimbo na data ya kusanyiko
# yalithibitishwa kuwa sawa kwa baiti na mbegu ya zamani ya NASM + ld.)
"$TMP/kus" < msingi/mbegu.swm > "$TMP/mbegu_huru"
kagua cmp -s "$TMP/mbegu_huru" msingi/mbegu.bin
"$TMP/kus2" < msingi/mbegu.swm > "$TMP/mbegu_huru2"
kagua cmp -s "$TMP/mbegu_huru2" msingi/mbegu.bin
# mjenzi (kiendeshaji cha mapishi): msingi/mjenzi.bin na .hex yake ni sawa
# kwa baiti na kusanyiko(mjenzi.swm); amri zake na makosa yake.
"$TMP/kus2" < $M/mjenzi.swm > "$TMP/mjenzi"
kagua cmp -s "$TMP/mjenzi" msingi/mjenzi.bin
mjenzi_hex_sawa() { ./msingi/kwanza.bin < msingi/mjenzi.hex | cmp -s - msingi/mjenzi.bin; }
kagua mjenzi_hex_sawa
mjenzi_ni() {  # $1 mapishi, $2 kutoka tarajiwa, $3 towe tarajiwa (stdout+stderr)
    printf '%s\n' "$1" > "$TMP/m.mapishi"
    local towe; towe=$(cd "$TMP" && "$OLDPWD/msingi/mjenzi.bin" m.mapishi 2>&1); local rc=$?
    [ "$rc" = "$2" ] && [ "$towe" = "$3" ]
}
kagua mjenzi_ni 'andika habari dunia ; maoni' 0 "habari dunia"
kagua mjenzi_ni "unganisha a m.mapishi m.mapishi
hex a b
sawa a a
nakili b c
sawa b c" 0 ""
kagua mjenzi_ni 'endesha_kosa 127 hakuna/programu' 0 ""
kagua mjenzi_ni 'kitu' 1 "mjenzi: mstari 1: kitu imeshindwa"
kagua mjenzi_ni 'andika a
sawa m.mapishi hakuna' 1 "a
mjenzi: mstari 2: sawa imeshindwa"
kagua mjenzi_ni 'endesha hakuna/programu' 1 "mjenzi: mstari 1: endesha imeshindwa (kutoka 127)"
mbegu_hex_sawa() { ./msingi/kwanza.bin < msingi/mbegu.hex | cmp -s - msingi/mbegu.bin; }
kagua mbegu_hex_sawa

echo "===== Matokeo ya mnyororo huru: $PASS yamefaulu, $FAIL yameshindwa ====="
[ "$FAIL" -eq 0 ]
