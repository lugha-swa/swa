#!/bin/bash
# jaribu-wasm.sh — Majaribio ya backend ya WebAssembly (Awamu 1)
#
# Dada mdogo wa jaribu-mnyororo.sh: hujenga stage1 (hatua zile zile
# za mwanzo), kisha kwa kila mstari wa majaribio/wasm/MANIFEST.txt:
#   - thamani ya "KOSA": "stage1 --wasm" LAZIMA ishindwe (kutoka
#     != 0, stdout tupu) — ujenzi ulio NJE ya wigo wa Awamu 1.
#   - thamani ya namba: "stage1 --wasm" LAZIMA ifaulu, na moduli
#     inayotokana LAZIMA ipite majaribio/wasm/kimbiza.js (Node:
#     WebAssembly.validate + instantiate + main() == thamani).
#
# mbegu.bin/mbegu.s HAZIHITAJI --wasm kabisa — ni kipengele cha
# stage1 pekee (angalia msingi/mkusanyaji/uzalishaji_wasm.swa).
# Hakuna wat2wasm, hakuna wasmtime — node pekee (WebAssembly asili).
set -u

cd "$(dirname "$0")/.." || exit 1
TMP="$(mktemp -d /tmp/swa-wasm-XXXXXX)"
trap 'rm -rf "$TMP"' EXIT

PASS=0; FAIL=0

if ! command -v node > /dev/null 2>&1; then
    echo "SHINDWA: node haipatikani kwenye PATH -- inahitajika kama oracle ya WASM"
    exit 1
fi

# ============ 1. Mnyororo wa kwanza ============
bash gharama/jenga-kwanza.sh > /dev/null || { echo "SHINDWA: jenga-kwanza"; exit 1; }
MBEGU="/tmp/mbegu2.bin"

# ============ 2. Jenga mkusanyaji wa .swa (stage1) ============
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
STAGE1="$TMP/stage1"

# ============ 3. Majaribio ya MANIFEST ============
MANIFEST="majaribio/wasm/MANIFEST.txt"
while IFS=$'\t' read -r tarajiwa jina njia; do
    [ -z "$tarajiwa" ] && continue
    case "$tarajiwa" in \#*) continue ;; esac

    WASM_OUT="$TMP/$jina.wasm"
    WASM_ERR="$TMP/$jina.err"
    "$STAGE1" --wasm "$njia" > "$WASM_OUT" 2> "$WASM_ERR"
    KUTOKA=$?

    if [ "$tarajiwa" = "KOSA" ]; then
        if [ "$KUTOKA" -eq 0 ] || [ -s "$WASM_OUT" ]; then
            echo "SHINDWA: $jina -- ujenzi ulio nje ya wigo ulikubaliwa kimya (kutoka=$KUTOKA, ukubwa wa .wasm=$(stat -c%s "$WASM_OUT" 2>/dev/null || echo 0))"
            FAIL=$((FAIL+1))
        else
            PASS=$((PASS+1))
        fi
        continue
    fi

    if [ "$KUTOKA" -ne 0 ] || [ ! -s "$WASM_OUT" ]; then
        echo "SHINDWA: $jina -- kukusanya kwa --wasm kulishindwa"; cat "$WASM_ERR"
        FAIL=$((FAIL+1))
        continue
    fi

    if node majaribio/wasm/kimbiza.js "$WASM_OUT" "$tarajiwa" 2> "$TMP/$jina.node.err"; then
        PASS=$((PASS+1))
    else
        echo "SHINDWA: $jina"; cat "$TMP/$jina.node.err"
        FAIL=$((FAIL+1))
    fi
done < "$MANIFEST"

echo ""
echo "===== Matokeo ya WASM: $PASS yamefaulu, $FAIL yameshindwa ====="
[ "$FAIL" -eq 0 ]
