#!/bin/bash
# jaribu-msuluhishi.sh — Majaribio ya msuluhishi wa husisha (gharama/msuluhishi.swa)
#
# Hujenga msuluhishi kwa mbegu, kisha huthibitisha:
#   1) jaribio dogo (mfuatano) — towe linakusanywa na kuendeshwa kwa
#      minyororo yote miwili (mbegu na stage1)
#   2) mnyororo wa ngazi tatu (c <- b <- a) kwenye saraka ya sasa
#   3) mzunguko wa faili mbili unakataliwa kwa sauti (msimbo 1)
#   4) uhakika: mara mbili pato sawa kwa baiti (hoja na kituo cha kuingiza)
#   5) grafu kamili ya mkusanyaji (msingi/mkusanyaji/stage1.swa) — fixpoint stage2 == stage3
#   6) kujitatua: towe la msuluhishi kujengwa linalingana kwa baiti
#   7) kutoathirika: kutatua towe tena ni kitendo tupu
#   8) husisha isiyo ya faili (mf. C::), faili lisilopo na { bila kufunga zinalia
set -u

cd "$(dirname "$0")/.." || exit 1
TMP="$(mktemp -d /tmp/swa-msuluhishi-XXXXXX)"
trap 'rm -rf "$TMP"' EXIT

PASS=0; FAIL=0

kagua() { # kagua <msimbo-halisi> <msimbo-tarajiwa> <maelezo>
    if [ "$1" = "$2" ]; then
        PASS=$((PASS+1))
    else
        echo "SHINDWA: $3 (ilitarajiwa $2, ilipata $1)"
        FAIL=$((FAIL+1))
    fi
}

# ============ 1. Jenga msuluhishi kwa mbegu ============
ZIMA="$TMP/zima-msuluhishi.swa"
for f in kumbukumbu mfuatano faili; do
    cat msingi/maktaba/$f.swa >> "$ZIMA"
done
cat gharama/msuluhishi.swa >> "$ZIMA"
./msingi/mbegu.bin --exe "$ZIMA" > "$TMP/msuluhishi" 2> "$TMP/e" || {
    echo "SHINDWA: ujenzi wa msuluhishi"; cat "$TMP/e"; exit 1; }
chmod +x "$TMP/msuluhishi"
PASS=$((PASS+1))

# ============ 2. Jenga stage1 mpya (mnyororo wa .swa) ============
# Kama jenga.sh: msuluhishi hutatua grafu ya husisha ya mkusanyaji.
ZIMA1="$TMP/zima.swa"
"$TMP/msuluhishi" msingi/mkusanyaji/stage1.swa > "$ZIMA1" || {
    echo "SHINDWA: kutatua msingi/mkusanyaji/stage1.swa"; exit 1; }
./msingi/mbegu.bin --exe "$ZIMA1" > "$TMP/stage1" 2> /dev/null || {
    echo "SHINDWA: ujenzi wa stage1"; exit 1; }
chmod +x "$TMP/stage1"
PASS=$((PASS+1))

# ============ 3. Jaribio dogo ============
cat > "$TMP/dogo.swa" <<'EOF'
husisha { mfuatano.swa }
n32 main() {
    n64 n = urefu_wa_mfuatano("habari");
    kama (n == 6) { rudisha 0; }
    rudisha 1;
}
EOF
"$TMP/msuluhishi" "$TMP/dogo.swa" > "$TMP/dogo-towe.swa" || { echo "SHINDWA: kutatua dogo"; exit 1; }
for mk in "mbegu" "stage1"; do
    if [ "$mk" = "mbegu" ]; then
        ./msingi/mbegu.bin --exe "$TMP/dogo-towe.swa" > "$TMP/dogo-exe" 2> /dev/null
    else
        "$TMP/stage1" --exe "$TMP/dogo-towe.swa" > "$TMP/dogo-exe" 2> /dev/null
    fi
    chmod +x "$TMP/dogo-exe"; "$TMP/dogo-exe"; rc=$?
    kagua "$rc" "0" "dogo kwa $mk"
done

# ============ 4. Mnyororo wa ngazi tatu (saraka ya sasa) ============
MNYORO="$TMP/mnyororo"; mkdir -p "$MNYORO"
printf 'n32 kazi_c() { rudisha 3; }\n' > "$MNYORO/c.swa"
printf 'husisha { c.swa }\nn32 kazi_b() { rudisha kazi_c() + 1; }\n' > "$MNYORO/b.swa"
printf 'husisha { b.swa }\nn32 main() { kama (kazi_b() == 4) { rudisha 0; } rudisha 1; }\n' > "$MNYORO/a.swa"
( cd "$MNYORO" && "$TMP/msuluhishi" a.swa > towe-a.swa ) || { echo "SHINDWA: kutatua mnyororo"; exit 1; }
grep -q "kazi_c" "$MNYORO/towe-a.swa" && grep -q "kazi_b" "$MNYORO/towe-a.swa"
kagua "$?" "0" "mnyororo: c na b ziko kwenye towe"
( cd "$MNYORO" && "$TMP/msuluhishi" a.swa > towe-a.swa ) 2> /dev/null
./msingi/mbegu.bin --exe "$MNYORO/towe-a.swa" > "$MNYORO/exe-a" 2> /dev/null && chmod +x "$MNYORO/exe-a" \
    && ( cd "$MNYORO" && ./exe-a ); kagua "$?" "0" "mnyororo wa ngazi 3 (mbegu)"

# ============ 5. Mzunguko ============
printf 'husisha { y.swa }\nn32 main() { rudisha 0; }\n' > "$MNYORO/x.swa"
printf 'husisha { x.swa }\nn32 kazi_y() { rudisha 1; }\n' > "$MNYORO/y.swa"
( cd "$MNYORO" && "$TMP/msuluhishi" x.swa > /dev/null 2> "mz.txt" ); rc=$?
grep -q "mzunguko" "$MNYORO/mz.txt"
kagua "$rc|$?" "1|0" "mzunguko unakataliwa kwa sauti"
[ -s "$MNYORO/mz.txt" ] && grep -q "x.swa -> y.swa -> x.swa" "$MNYORO/mz.txt"
kagua "$?" "0" "mzunguko unataja mzunguko wenyewe"

# ============ 6. Uhakika (mara 3 + kituo cha kuingiza) ============
H1=$("$TMP/msuluhishi" "$TMP/dogo.swa" | md5sum | cut -c1-16)
H2=$("$TMP/msuluhishi" "$TMP/dogo.swa" | md5sum | cut -c1-16)
H3=$(cat "$TMP/dogo.swa" | "$TMP/msuluhishi" | md5sum | cut -c1-16)
kagua "$H1$H2$H3" "$H1$H1$H1" "uhakika wa pato"

# ============ 7. Grafu kamili ya mkusanyaji ============
"$TMP/msuluhishi" msingi/mkusanyaji/stage1.swa > "$TMP/gurafu.swa" || { echo "SHINDWA: kutatua grafu kamili"; exit 1; }
grep -c "^husisha {" "$TMP/gurafu.swa"
kagua "$?" "1" "gurafu: hakuna maelekezo ya { yaliyosalia"
./msingi/mbegu.bin --exe "$TMP/gurafu.swa" > "$TMP/stage1-k" 2> /dev/null || { echo "SHINDWA: kusanya grafu kamili"; exit 1; }
chmod +x "$TMP/stage1-k"
"$TMP/stage1-k" --exe "$TMP/gurafu.swa" > "$TMP/stage2-k" 2> /dev/null || { echo "SHINDWA: stage1-k"; exit 1; }
chmod +x "$TMP/stage2-k"
"$TMP/stage2-k" --exe "$TMP/gurafu.swa" > "$TMP/stage3-k" 2> /dev/null || { echo "SHINDWA: stage2-k"; exit 1; }
cmp -s "$TMP/stage2-k" "$TMP/stage3-k"
kagua "$?" "0" "fixpoint wa grafu kamili (stage2 == stage3)"
"$TMP/stage1-k" --exe "$TMP/dogo-towe.swa" > "$TMP/dogo-via-k" 2> /dev/null && chmod +x "$TMP/dogo-via-k" \
    && "$TMP/dogo-via-k"; kagua "$?" "0" "stage1-k inakusanya dogo"

# ============ 8. Kujitatua na kutoathirika ============
"$TMP/msuluhishi" gharama/msuluhishi.swa > "$TMP/kujitatua.swa" || { echo "SHINDWA: kujitatua"; exit 1; }
./msingi/mbegu.bin --exe "$TMP/kujitatua.swa" > "$TMP/kujitatua.bin" 2> /dev/null && chmod +x "$TMP/kujitatua.bin"
cmp -s "$TMP/kujitatua.bin" "$TMP/msuluhishi"
kagua "$?" "0" "kujitatua: binari sawa kwa baiti"
"$TMP/msuluhishi" "$TMP/kujitatua.swa" > "$TMP/kutodhani.swa" 2> /dev/null
cmp -s "$TMP/kutodhani.swa" "$TMP/kujitatua.swa"
kagua "$?" "0" "kutoathirika: pato la pili == la kwanza"

# ============ 9. C:: na makosa ============
printf 'husisha C::stdio\nhusisha { mfuatano.swa }\nn32 main() { rudisha 0; }\n' > "$TMP/c.swa"
"$TMP/msuluhishi" "$TMP/c.swa" > /dev/null 2> "$TMP/ce"; rc=$?
grep -q "husisha isiyo sahihi" "$TMP/ce"
kagua "$rc|$?" "1|0" "husisha C:: inakataliwa kwa sauti (Swa haina kiungo cha C)"
printf 'husisha { haipo.swa }\nn32 main() { rudisha 0; }\n' > "$TMP/hp.swa"
"$TMP/msuluhishi" "$TMP/hp.swa" > /dev/null 2> "$TMP/hpe"; rc=$?
grep -q "halipatikani" "$TMP/hpe"
kagua "$rc|$?" "1|0" "faili lisilopo linalia kwa sauti"
printf 'husisha { x\nn32 main() { rudisha 0; }\n' > "$TMP/bf.swa"
"$TMP/msuluhishi" "$TMP/bf.swa" > /dev/null 2> "$TMP/bfe"; rc=$?
grep -q "kufunga" "$TMP/bfe"
kagua "$rc|$?" "1|0" "husisha { bila '}' linalia kwa sauti"

# ============ Matokeo ============
echo ""
echo "===== Matokeo ya msuluhishi: $PASS yamefaulu, $FAIL yameshindwa ====="
[ "$FAIL" -eq 0 ] || exit 1
exit 0
