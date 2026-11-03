#!/bin/bash
# jaribu-mifano.sh — kila mifano/*.swa lazima ikusanywe na itoe HASA
# mifano/*.tarajiwa. README hunukuu mifano hii; jaribio hili ndilo
# linalohakikisha msimbo ulio kwenye ukurasa wa mbele bado unafanya kazi.
set -u
cd "$(dirname "$0")/.." || exit 1
[ -x bin/swa ] || ./msingi/mjenzi.bin gharama/jenga.mapishi > /dev/null || exit 1
TMP="$(mktemp -d /tmp/swa-mifano-XXXXXX)"
trap 'rm -rf "$TMP"' EXIT

PASS=0; FAIL=0
for f in mifano/*.swa; do
    jina="${f%.swa}"
    if ! bin/swa --exe "$f" > "$TMP/p" 2> "$TMP/e"; then
        echo "SHINDWA: kukusanya $f"; cat "$TMP/e"; FAIL=$((FAIL+1)); continue
    fi
    chmod +x "$TMP/p"
    if timeout 5 "$TMP/p" > "$TMP/out" && cmp -s "$TMP/out" "$jina.tarajiwa"; then
        PASS=$((PASS+1))
    else
        echo "SHINDWA: matokeo ya $f hayalingani na $jina.tarajiwa"
        diff "$jina.tarajiwa" "$TMP/out" | head -10
        FAIL=$((FAIL+1))
    fi
done
echo "===== Matokeo ya mifano: $PASS yamefaulu, $FAIL yameshindwa ====="
[ "$FAIL" -eq 0 ]
