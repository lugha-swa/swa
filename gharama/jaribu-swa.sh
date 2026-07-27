#!/bin/bash
# jaribu-swa.sh — Endesha majaribio yote ya .swa kupitia mkusanyaji wa stage1
#
# Matumizi:
#   jaribu-swa.sh <njia-ya-stage1> <saraka-ya-majaribio>
#
# Kila faili la .swa linakusanywa na stage1, kuunganishwa na trampoline,
# na kuendeshwa. Matokeo yanachapishwa kama PASS au FAIL.

set -uo pipefail

# ------------------------------------------------------------
# Kazi ya kusafisha faili za muda
# ------------------------------------------------------------
cleanup() {
    rm -f "$TRAMPOLINE_O"
    rm -rf "$TMPDIR"
}
trap cleanup EXIT

# ------------------------------------------------------------
# Thibitisha hoja
# ------------------------------------------------------------
if [ $# -lt 2 ]; then
    echo "Matumizi: $0 <stage1-binary> <saraka-ya-majaribio>" >&2
    exit 2
fi

STAGE1="$(realpath "$1")"
TEST_DIR="$(realpath "$2")"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TRAMPOLINE_C="$SCRIPT_DIR/trampoline.c"

if [ ! -x "$STAGE1" ]; then
    echo "HITILAFU: $STAGE1 si faili linaloweza kutekelezwa" >&2
    exit 1
fi

if [ ! -d "$TEST_DIR" ]; then
    echo "HITILAFU: $TEST_DIR si saraka" >&2
    exit 1
fi

if [ ! -f "$TRAMPOLINE_C" ]; then
    echo "HITILAFU: trampoline.c haipatikani kwenye $TRAMPOLINE_C" >&2
    exit 1
fi

# ------------------------------------------------------------
# Tafuta clang
# ------------------------------------------------------------
CLANG=""
for c in clang clang-22 clang-18 clang-17 clang-16 clang-15; do
    if command -v "$c" &>/dev/null; then
        CLANG="$c"
        break
    fi
done

if [ -z "$CLANG" ]; then
    echo "HITILAFU: clang haipatikani. Weka clang (clang, clang-22, au toleo lingine)." >&2
    exit 1
fi

echo "; K6: kutumia $CLANG"

# ------------------------------------------------------------
# Kusanya trampoline mara moja
# ------------------------------------------------------------
TRAMPOLINE_O="$(mktemp /tmp/swa-trampoline-XXXXXX.o)"

if ! "$CLANG" -c "$TRAMPOLINE_C" -o "$TRAMPOLINE_O" 2>/tmp/swa-trampoline-err; then
    echo "HITILAFU: imeshindwa kukusanya trampoline.c" >&2
    cat /tmp/swa-trampoline-err >&2
    rm -f /tmp/swa-trampoline-err
    exit 1
fi
rm -f /tmp/swa-trampoline-err

echo "; trampoline imekusanywa: $TRAMPOLINE_O"

# ------------------------------------------------------------
# Endesha majaribio
# ------------------------------------------------------------
PASS=0
FAIL=0
BUILD_FAIL=0
TOTAL=0

# Vipeperushi vya muda kwa matokeo
TMPDIR="$(mktemp -d /tmp/swa-jaribu-XXXXXX)"

# Pata faili zote za .swa zilizopangwa.
# Tumia process substitution (< <(...)) badala ya bomba ili kuhifadhi
# hesabu za PASS/FAIL ndani ya kitanzi.
while IFS= read -r -d '' swa_file; do
    rel="${swa_file#$TEST_DIR/}"
    TOTAL=$((TOTAL + 1))

    test_dir="$(dirname "$swa_file")"

    # Unda saraka ya muda kwa jaribio hili
    jaribio_tmp="$(mktemp -d "$TMPDIR/jaribio-XXXXXX")"

    # Hatua ya 1: Endesha stage1 (cd hadi saraka ya faili la jaribio
    # ili njia za husisha zifanye kazi kwa usahihi).
    # Stage1 huandika faili la kitu hadi stdout.
    stage1_exit=0
    (cd "$test_dir" && "$STAGE1" "$(basename "$swa_file")" \
        > "$jaribio_tmp/test.o" \
        2> "$jaribio_tmp/stage1.err") || stage1_exit=$?

    if [ "$stage1_exit" -ne 0 ]; then
        echo "JENGA-FAIL: $rel (stage1 msimbo=$stage1_exit)"
        if [ -s "$jaribio_tmp/stage1.err" ]; then
            grep -v '^\s*$' "$jaribio_tmp/stage1.err" | while IFS= read -r line; do
                echo "  $line"
            done
        fi
        BUILD_FAIL=$((BUILD_FAIL + 1))
        rm -rf "$jaribio_tmp"
        continue
    fi

    # Hatua ya 2: Unganisha test.o + trampoline.o -> test_bin
    link_exit=0
    "$CLANG" "$jaribio_tmp/test.o" "$TRAMPOLINE_O" \
        -o "$jaribio_tmp/test_bin" \
        -no-pie \
        2> "$jaribio_tmp/link.err" || link_exit=$?

    if [ "$link_exit" -ne 0 ]; then
        echo "UNGANISHA-FAIL: $rel"
        cat "$jaribio_tmp/link.err" >&2
        FAIL=$((FAIL + 1))
        rm -rf "$jaribio_tmp"
        continue
    fi

    # Hatua ya 3: Endesha binary iliyounganishwa
    run_exit=0
    "$jaribio_tmp/test_bin" \
        > "$jaribio_tmp/stdout" \
        2> "$jaribio_tmp/stderr" || run_exit=$?

    if [ "$run_exit" -eq 0 ]; then
        echo "PASS: $rel"
        PASS=$((PASS + 1))
    else
        echo "FAIL: $rel (msimbo=$run_exit)"
        if [ -s "$jaribio_tmp/stderr" ]; then
            cat "$jaribio_tmp/stderr" >&2
        fi
        FAIL=$((FAIL + 1))
    fi

    rm -rf "$jaribio_tmp"
done < <(find "$TEST_DIR" -name "*.swa" -type f -print0 | sort -z)

# ------------------------------------------------------------
# Matokeo
# ------------------------------------------------------------
echo ""
echo "===== Matokeo: $PASS/$TOTAL yamefaulu, $FAIL yameshindwa, $BUILD_FAIL kushindwa kujenga ====="

if [ "$FAIL" -gt 0 ] || [ "$BUILD_FAIL" -gt 0 ]; then
    exit 1
fi
exit 0
