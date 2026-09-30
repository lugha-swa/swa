#!/bin/bash
# jaribu-kernel.sh -- Majaribio ya backend ya --kernel (Kawira, Sehemu 2)
#
# Dada mdogo wa jaribu-mnyororo.sh/jaribu-wasm.sh: hujenga stage1
# (hatua zile zile za mwanzo), kisha kwa kila mstari wa
# majaribio/kernel/MANIFEST.txt:
#   - tarajiwa "KATA": "stage1 --kernel" LAZIMA ishindwe (toka != 0,
#     stdout tupu) -- ujenzi ulio NJE ya wigo wa --kernel (tenga,
#     andika, andika_stderr, wito_wa_mfumo, tekeleza, muundo, D32/D64,
#     chagua, kernel_main isiyo ya kwanza).
#   - tarajiwa ya namba: "stage1 --kernel" LAZIMA ifaulu; baiti ghafi
#     za .text zinazotokana zinaunganishwa (nasm + incbin) na
#     majaribio/kernel/stub.s (boot stub ndogo ya Multiboot1, kioo
#     kilichopunguzwa cha Kawira's kiini.s), zinaendeshwa QEMU+KVM
#     (-kernel, hakuna BIOS/linker), na oracle.py inasoma HALI HALISI
#     ya CPU (RDI kupitia monitor socket, "info registers") na
#     kumbukumbu ya VGA (0xB8000 kupitia "xp") kulinganisha na
#     tarajiwa -- SI "haikuanguka", ni ulinganisho wa thamani halisi.
#
# mbegu.bin/mbegu.s HAZIHITAJI --kernel kabisa -- ni kipengele cha
# stage1 pekee (angalia msingi/mkusanyaji/uzalishaji.swa, hali_exe==2).
set -u

cd "$(dirname "$0")/.." || exit 1
TMP="$(mktemp -d /tmp/swa-kernel-XXXXXX)"
trap 'rm -rf "$TMP"' EXIT

PASS=0; FAIL=0

for dep in qemu-system-x86_64 nasm python3; do
    if ! command -v "$dep" > /dev/null 2>&1; then
        echo "SHINDWA: $dep haipatikani kwenye PATH -- inahitajika na oracle ya --kernel"
        exit 1
    fi
done

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
MANIFEST="majaribio/kernel/MANIFEST.txt"
while IFS=$'\t' read -r tarajiwa jina njia vga ujumbe; do
    [ -z "$tarajiwa" ] && continue
    case "$tarajiwa" in \#*) continue ;; esac

    OUT="$TMP/$jina.bin"
    ERR="$TMP/$jina.err"
    "$STAGE1" --kernel "$njia" > "$OUT" 2> "$ERR"
    KUTOKA=$?

    if [ "$tarajiwa" = "KATA" ]; then
        if [ "$KUTOKA" -eq 0 ] || [ -s "$OUT" ]; then
            echo "SHINDWA: $jina -- ujenzi ulio nje ya wigo ulikubaliwa kimya (kutoka=$KUTOKA, ukubwa=$(stat -c%s "$OUT" 2>/dev/null || echo 0))"
            FAIL=$((FAIL+1))
            continue
        fi
        if [ "$ujumbe" != "-" ] && ! grep -qF "$ujumbe" "$ERR"; then
            echo "SHINDWA: $jina -- ujumbe wa kosa haukuwa na kipande '$ujumbe':"; cat "$ERR"
            FAIL=$((FAIL+1))
            continue
        fi
        PASS=$((PASS+1))
        continue
    fi

    if [ "$KUTOKA" -ne 0 ] || [ ! -s "$OUT" ]; then
        echo "SHINDWA: $jina -- kukusanya kwa --kernel kulishindwa"; cat "$ERR"
        FAIL=$((FAIL+1))
        continue
    fi

    FULL="$TMP/$jina-full.bin"
    if ! nasm -f bin -D JARIBIO="\"$OUT\"" majaribio/kernel/stub.s -o "$FULL" 2> "$TMP/$jina.nasm.err"; then
        echo "SHINDWA: $jina -- nasm (stub.s + incbin) kulishindwa"; cat "$TMP/$jina.nasm.err"
        FAIL=$((FAIL+1))
        continue
    fi

    VGA_ARG=()
    if [ "$vga" != "-" ]; then VGA_ARG=("$vga"); fi
    if python3 majaribio/kernel/oracle.py "$FULL" "$tarajiwa" "${VGA_ARG[@]}" 2> "$TMP/$jina.oracle.err"; then
        PASS=$((PASS+1))
    else
        echo "SHINDWA: $jina"; cat "$TMP/$jina.oracle.err"
        FAIL=$((FAIL+1))
    fi
done < "$MANIFEST"

echo ""
echo "===== Matokeo ya --kernel: $PASS yamefaulu, $FAIL yameshindwa ====="
[ "$FAIL" -eq 0 ]
