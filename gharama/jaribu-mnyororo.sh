#!/bin/bash
# jaribu-mnyororo.sh — Majaribio ya mnyororo wa uzalishaji (Swa pekee)
#
# Huendesha program za majaribio (majaribio/programu/*.swa) kupitia
# minyororo YOTE MIWILI — mbegu na stage1 (mnyororo wa .swa) — na
# kuthibitisha msimbo wa kutoka unaotarajiwa (MANIFEST.txt).
# Pia: fixpoint (stage2 == stage3), mkazo wa RELA, kwa-endelea,
# desimali, mzunguko mfupi, kazi kukosa, bomba kubwa, na umbizaji
# kujijenga. Hakuna Rust, hakuna LLVM, hakuna clang.
set -u

cd "$(dirname "$0")/.." || exit 1
TMP="$(mktemp -d /tmp/swa-mnyororo-XXXXXX)"
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

# ============ 1. Mnyororo wa kwanza ============
bash gharama/jenga-kwanza.sh > /dev/null || { echo "SHINDWA: jenga-kwanza"; exit 1; }
MBEGU="/tmp/mbegu2.bin"

# ============ 2. Jenga mkusanyaji wa .swa (stage1) ============
# Msuluhishi wa husisha (gharama/msuluhishi.swa) hujengwa kwanza na
# mbegu, kisha hutatua mnyororo wa msingi/mkusanyaji/stage1.swa:
# utegemezi hufuatwa kwa mpangilio wa topolojia, marudio yanaondolewa,
# na mistari ya husisha yanafutwa. ZIMA linalotokana ndilo linalojengwa.
ZIMA="$TMP/zima.swa"
MSULU="$TMP/zima-msuluhishi.swa"
cat msingi/maktaba/kumbukumbu.swa msingi/maktaba/mfuatano.swa \
    msingi/maktaba/faili.swa gharama/msuluhishi.swa > "$MSULU"
"$MBEGU" --exe "$MSULU" > "$TMP/msuluhishi" 2> "$TMP/msuluhishi.err" || {
    echo "SHINDWA: mbegu --exe msuluhishi"; cat "$TMP/msuluhishi.err"; exit 1; }
chmod +x "$TMP/msuluhishi"
"$TMP/msuluhishi" msingi/mkusanyaji/stage1.swa > "$ZIMA" || {
    echo "SHINDWA: kutatua msingi/mkusanyaji/stage1.swa"; exit 1; }
"$MBEGU" --exe "$ZIMA" > "$TMP/stage1" 2> "$TMP/stage1.err" || {
    echo "SHINDWA: mbegu --exe zima.swa"; cat "$TMP/stage1.err"; exit 1; }
chmod +x "$TMP/stage1"
PASS=$((PASS+1))  # ujenzi wa stage1

# ============ 3. Fixpoint: stage2 == stage3 (sawa kwa baiti) ============
"$TMP/stage1" --exe "$ZIMA" > "$TMP/stage2" 2> /dev/null || { echo "SHINDWA: stage1 --exe"; exit 1; }
chmod +x "$TMP/stage2"
"$TMP/stage2" --exe "$ZIMA" > "$TMP/stage3" 2> /dev/null || { echo "SHINDWA: stage2 --exe"; exit 1; }
if cmp -s "$TMP/stage2" "$TMP/stage3"; then
    PASS=$((PASS+1))
else
    echo "SHINDWA: fixpoint (stage2 != stage3)"
    FAIL=$((FAIL+1))
fi

# ============ 4. Program za majaribio (minyororo yote miwili) ============
# Safu ya nne (hiari) inachagua mnyororo: "stage1" = stage1 pekee.
# Hii inatumiwa na jaribio_chagua_* — mbegu haijui neno la chagua,
# kwa hiyo mnyororo wa mbegu huachwa (mbegu kukataa ni halali).
# Safu ya kwanza "KATA" = chanzo lazima KIKATALIWE na mkusanyaji:
# toka isiyo 0 pamoja na ujumbe wa kosa, hakuna faili la tokeo.
# Safu ya tano (hiari) ni kipande cha ujumbe unaotarajiwa kwenye
# KATA; kilichopo kinabaki kwa kigezo (jina la aina).
while IFS=$'\t' read -r code jina faili mnyororo ujumbe; do
    [ -z "$code" ] && continue
    [ "$code" = "FAIL" ] && continue  # kukataliwa kunashughulikiwa tofauti
    if [ "$code" = "KATA" ]; then
        if [ "$mnyororo" = "stage1" ]; then
            "$TMP/stage1" --exe "$faili" > "$TMP/p" 2> "$TMP/p.err"; rc=$?
        else
            "$MBEGU" --exe "$faili" > "$TMP/p" 2> "$TMP/p.err"; rc=$?
        fi
        kipande="${ujumbe:-jina la aina}"
        grep -qF "$kipande" "$TMP/p" "$TMP/p.err"
        ni_ujumbe=$?
        [ -s "$TMP/p" ]; ni_faili=$?
        kagua "$rc|$ni_ujumbe|$ni_faili" "1|0|1" "kukataa $faili ($mnyororo)"
        continue
    fi
    if [ "$mnyororo" = "stage1" ]; then
        "$TMP/stage1" --exe "$faili" > "$TMP/p" 2> /dev/null
        if [ $? -ne 0 ]; then
            echo "SHINDWA: kukusanya $faili kwa stage1"
            FAIL=$((FAIL+1)); continue
        fi
        chmod +x "$TMP/p"
        timeout 5 "$TMP/p"; rc=$?
        kagua "$rc" "$code" "$faili kwa stage1"
        continue
    fi
    for mk in "mbegu" "stage1"; do
        if [ "$mk" = "mbegu" ]; then
            "$MBEGU" --exe "$faili" > "$TMP/p" 2> /dev/null
        else
            "$TMP/stage1" --exe "$faili" > "$TMP/p" 2> /dev/null
        fi
        if [ $? -ne 0 ]; then
            echo "SHINDWA: kukusanya $faili kwa $mk"
            FAIL=$((FAIL+1)); continue
        fi
        chmod +x "$TMP/p"
        timeout 5 "$TMP/p"; rc=$?
        kagua "$rc" "$code" "$faili kwa $mk"
    done
done < majaribio/programu/MANIFEST.txt

# ============ 5. Mkazo wa RELA (wito 1,000 wa mbele) ============
MK="$TMP/mkazo.swa"
{
    echo "n32 main() { n32 s = 0;"
    i=0; while [ $i -lt 1000 ]; do echo "s = s + lengo(1);"; i=$((i+1)); done
    echo "kama (s != 1000) rudisha 1; rudisha 0; }"
    echo "n32 lengo(n32 x) { rudisha x; }"
} > "$MK"
"$MBEGU" --exe "$MK" > "$TMP/mkazo-exe" 2> /dev/null && chmod +x "$TMP/mkazo-exe" \
    && "$TMP/mkazo-exe"; kagua "$?" "0" "mkazo wa RELA (mbegu)"

# ============ 6. Semantiki ya C ya endelea-kwenye-kwa ============
KWC="$TMP/kwa_c.swa"
cat > "$KWC" <<'EOF'
n32 main() {
    n32 i = 0;
    n32 s = 0;
    kwa (; i < 6; i = i + 1) {
        kama (i == 2) { endelea; }
        s = s + 1;
    }
    rudisha s;
}
EOF
for mk in "mbegu" "stage1"; do
    if [ "$mk" = "mbegu" ]; then
        "$MBEGU" --exe "$KWC" > "$TMP/kwc" 2> /dev/null
    else
        "$TMP/stage1" --exe "$KWC" > "$TMP/kwc" 2> /dev/null
    fi
    chmod +x "$TMP/kwc"; timeout 5 "$TMP/kwc"; rc=$?
    kagua "$rc" "5" "endelea-kwenye-kwa ($mk) — C-semantiki"
done

# ============ 7. Desimali (D64) ============
DES="$TMP/des.swa"
cat > "$DES" <<'EOF'
n32 main() {
    d64 pi = 3.14;
    d64 r = 2.0;
    d64 eneo = pi * r * r;
    kama (eneo > 12.55 && eneo < 12.57) { } sivyo { rudisha 1; }
    d64 a = 1.5 + 0.5;
    kama (a > 1.99 && a < 2.01) { } sivyo { rudisha 2; }
    d64 b = 6.0 / 2.0;
    kama (b > 2.99 && b < 3.01) { } sivyo { rudisha 3; }
    d64 c = -2.5;
    kama (c > -2.51 && c < -2.49) { } sivyo { rudisha 4; }
    d64 d = 10.0 - 3.25;
    kama (d > 6.74 && d < 6.76) { } sivyo { rudisha 5; }
    rudisha 0;
}
EOF
for mk in "mbegu" "stage1"; do
    if [ "$mk" = "mbegu" ]; then
        "$MBEGU" --exe "$DES" > "$TMP/des" 2> /dev/null
    else
        "$TMP/stage1" --exe "$DES" > "$TMP/des" 2> /dev/null
    fi
    chmod +x "$TMP/des"; timeout 5 "$TMP/des"; rc=$?
    kagua "$rc" "0" "desimali ($mk)"
done

# ============ 8. Mzunguko mfupi wa && / || ============
MZ="$TMP/mz.swa"
cat > "$MZ" <<'EOF'
n32 main() {
    n32 bwete = 0;
    n32 a[4];
    kama (bwete && a[99999999] == 1) rudisha 1;
    n32 x = 2 && 4;
    kama (x != 1) rudisha 2;
    rudisha 0;
}
EOF
"$MBEGU" --exe "$MZ" > "$TMP/mz" 2> /dev/null && chmod +x "$TMP/mz" \
    && timeout 5 "$TMP/mz"; kagua "$?" "0" "mzunguko mfupi (mbegu)"

# ============ 9. Kazi isiyofafanuliwa inalia kwa sauti ============
# Kesi ya kwanza: jina la kazi halipo kabisa (minyororo yote miwili).
# Kesi ya pili: chanzo kinatangaza husisha { maktaba/mfuatano.swa } —
# mbegu (ya zamani) inasoma kwenye saraka ya faili pekee na kuacha
# mstari, kwa hiyo kazi za maktaba hazipo na inalia; stage1 mpya
# inatatua msingi/maktaba/mfuatano.swa yenyewe na kufaulu (urefu wa
# "habari" = 6).
echo 'n32 main() { rudisha kazi_haipo(); }' > "$TMP/kk.swa"
echo 'husisha { maktaba/mfuatano.swa }' > "$TMP/khus.swa"
echo 'n32 main() { rudisha urefu_wa_mfuatano("habari"); }' >> "$TMP/khus.swa"
for mk in "mbegu" "stage1"; do
    if [ "$mk" = "mbegu" ]; then
        "$MBEGU" --exe "$TMP/kk.swa" > "$TMP/kk" 2> "$TMP/kk.err"; rc=$?
    else
        "$TMP/stage1" --exe "$TMP/kk.swa" > "$TMP/kk" 2> "$TMP/kk.err"; rc=$?
    fi
    grep -q "haijafafanuliwa" "$TMP/kk" "$TMP/kk.err"
    kagua "$rc|$?" "1|0" "kazi kukosa inalia kwa sauti ($mk)"
done
"$MBEGU" --exe "$TMP/khus.swa" > "$TMP/kh" 2> "$TMP/kh.err"; rc=$?
grep -q "haijafafanuliwa" "$TMP/kh" "$TMP/kh.err"
kagua "$rc|$?" "1|0" "husisha-kazi kukosa inalia kwa sauti (mbegu)"
"$TMP/stage1" --exe "$TMP/khus.swa" > "$TMP/kh2" 2> "$TMP/kh2.err"; rc1=$?
chmod +x "$TMP/kh2"
if [ "$rc1" -eq 0 ]; then timeout 5 "$TMP/kh2"; rc2=$?; else rc2=99; fi
kagua "$rc1|$rc2" "0|6" "husisha { maktaba/mfuatano.swa } inatatuliwa na stage1 (jibu 6)"

# ============ 10. Bomba la stdin (chanzo kubwa hadi EOF) ============
BOM="$TMP/bomba.swa"
{
    i=0; while [ $i -lt 3000 ]; do echo "// mstari wa kujaza bafa $i"; i=$((i+1)); done
    echo "n32 main() { rudisha kazi_ya_mwisho(); }"
    echo "n32 kazi_ya_mwisho() { rudisha 7; }"
} > "$BOM"
cat "$BOM" | "$MBEGU" --exe > "$TMP/bomba-exe" 2> /dev/null
chmod +x "$TMP/bomba-exe"; timeout 5 "$TMP/bomba-exe"; rc=$?
kagua "$rc" "7" "bomba la stdin (mbegu)"

# ============ 11. ABI ya xmm: D64 kwenye wito wa mbegu ============
# Mbegu sasa ina ABI ya xmm (hoja na kurudi kwa D64). Hoja za D64
# zilizochanganywa na nafasi 7-9 bado hazisaidiwi — zinalia kwa sauti.
D64F="$TMP/d64w.swa"
cat > "$D64F" <<'EOF'
d64 mara_mbili(d64 x) { rudisha x * 2.0; }
d64 jumla3(d64 a, d64 b, d64 c) { rudisha a + b + c; }
n32 main() {
    d64 r = mara_mbili(3.5);
    kama (r < 6.99 || r > 7.01) rudisha 1;
    d64 s = jumla3(1.5, 2.5, 3.0);
    kama (s < 6.99 || s > 7.01) rudisha 2;
    rudisha 0;
}
EOF
for mk in "mbegu" "stage1"; do
    if [ "$mk" = "mbegu" ]; then
        "$MBEGU" --exe "$D64F" > "$TMP/d64w" 2> /dev/null
    else
        "$TMP/stage1" --exe "$D64F" > "$TMP/d64w" 2> /dev/null
    fi
    chmod +x "$TMP/d64w"; timeout 5 "$TMP/d64w"; rc=$?
    kagua "$rc" "0" "hoja na kurudi kwa D64 ($mk)"
done

# Kikomo: D64 iliyochanganywa na hoja 7-9 inakataliwa kwa sauti
D64M="$TMP/d64m.swa"
cat > "$D64M" <<'EOF'
d64 nyingi(n32 a, n32 b, n32 c, n32 d, n32 e, n32 f, n32 g, d64 h) { rudisha h; }
n32 main() { d64 r = nyingi(1, 2, 3, 4, 5, 6, 7, 8.5); rudisha 0; }
EOF
"$MBEGU" --exe "$D64M" > "$TMP/d64m" 2> /dev/null; rc=$?
grep -q "hoja 7-9" "$TMP/d64m"
kagua "$rc|$?" "1|0" "D64 na hoja 7-9 inalia kwa sauti (mbegu)"

# ============ 12. Formatter inajijenga (fixpoint) ============
UMB="$TMP/umbizaji-zima.swa"
cat msingi/maktaba/kumbukumbu.swa msingi/maktaba/mfuatano.swa zana/umbizaji.swa > "$UMB"
"$MBEGU" --exe "$UMB" > "$TMP/umbizaji-exe" 2> /dev/null || { echo "SHINDWA: umbizaji --exe"; FAIL=$((FAIL+1)); }
chmod +x "$TMP/umbizaji-exe"
"$TMP/umbizaji-exe" zana/umbizaji.swa > "$TMP/umbizaji-towe" 2> /dev/null
cmp -s zana/umbizaji.swa "$TMP/umbizaji-towe"
kagua "$?" "0" "umbizaji kujijenga (fixpoint)"

# ============ 13. Uhakika wa bomba la stdin ============
H1=$(cat msingi/maktaba/hesabu.swa msingi/maktaba/mfuatano.swa | "$MBEGU" --exe 2>/dev/null | md5sum | cut -c1-16)
H2=$(cat msingi/maktaba/hesabu.swa msingi/maktaba/mfuatano.swa | "$MBEGU" --exe 2>/dev/null | md5sum | cut -c1-16)
H3=$(cat msingi/maktaba/hesabu.swa msingi/maktaba/mfuatano.swa | "$MBEGU" --exe 2>/dev/null | md5sum | cut -c1-16)
kagua "$H1$H2$H3" "$H1$H1$H1" "uhakika wa bomba (mara 3)"

# ============ 14. Jenerali-2 haipotezi maandishi ya andika_stderr ============
# lugha-swa/swa#232: stage1 (jenerali-1, iliyojengwa na mbegu) ikitumika
# KUJIKUSANYA YENYEWE (kuzalisha jenerali-2) ilipoteza maandishi HALISI
# yanayozunguka %s kwenye ujumbe wa makosa uliochapishwa na andika_stderr
# -- jina la faili pekee lilibaki, si "; KOSA: faili la chanzo
# halifunguki: <jina>". Fixpoint (sehemu ya 3 hapo juu) HAIGUNDUI hili
# kwa sababu jenerali-2 na jenerali-3 zote mbili zilipoteza maandishi
# KWA NAMNA ILE ILE (thabiti lakini SI SAHIHI) -- usawa wa baiti si
# usawa na usahihi. Kikomo cha kweli kilikuwa jedwali la sehemu za
# andika (sehemu_buf/sehemu_off/sehemu_len ndani ya uzalishaji.swa)
# lililokuwa dogo mno kwa ukusanyaji mkubwa (mkusanyaji mzima
# unajikusanya una wito zaidi ya 140 wa andika_stderr) -- lilipofurika
# lilirudisha -1 kimya, likidondosha sehemu za maandishi. Jaribio hili
# huthibitisha kwa KUSOMA MAANDISHI HALISI ya toleo la jenerali-2,
# si tu kwamba linatoka bila kuanguka.
JENERALI2="$TMP/jenerali2"
"$TMP/stage1" --exe "$ZIMA" > "$JENERALI2" 2> /dev/null
chmod +x "$JENERALI2"
"$JENERALI2" --exe /tmp/hii-faili-halipo-kabisa-232.swa > "$TMP/j2out" 2> "$TMP/j2err"
grep -qF "; KOSA: faili la chanzo halifunguki: /tmp/hii-faili-halipo-kabisa-232.swa" "$TMP/j2err"
kagua "$?" "0" "jenerali-2 haipotezi maandishi ya andika_stderr (#232)"

# ============ Matokeo ============
echo ""
echo "===== Matokeo ya mnyororo: $PASS yamefaulu, $FAIL yameshindwa ====="
[ "$FAIL" -eq 0 ] || exit 1
exit 0
