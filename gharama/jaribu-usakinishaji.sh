#!/bin/bash
# jaribu-usakinishaji.sh — swa iliyosakinishwa NJE ya hazina lazima
# ipate maktaba yake yenyewe. Mpangilio ni ule ule wa hazina:
#
#   <mzizi>/bin/swa
#   <mzizi>/msingi/maktaba/*.swa
#
# Kila mifano/*.swa inakusanywa kutoka saraka isiyo na msingi/ ndani
# yake, kwa njia nne za kuita: njia kamili, kiungo (symlink), njia ya
# jamaa, na kutoka saraka ya ndani zaidi. Matokeo lazima yalingane na
# mifano/*.tarajiwa. Hili ndilo linalothibitisha kwamba binary ya
# toleo (na picha ya ghcr) inafanya kazi baada ya kupakuliwa.
set -u
cd "$(dirname "$0")/.." || exit 1
[ -x bin/swa ] || bash gharama/jenga.sh > /dev/null || exit 1
HAZINA="$(pwd)"
TMP="$(mktemp -d /tmp/swa-usakinishaji-XXXXXX)"
trap 'rm -rf "$TMP"' EXIT

mkdir -p "$TMP/mzizi/bin" "$TMP/mzizi/msingi" "$TMP/kiungo" "$TMP/mradi/ndani"
cp bin/swa "$TMP/mzizi/bin/swa"
cp -r msingi/maktaba "$TMP/mzizi/msingi/maktaba"
ln -s "$TMP/mzizi/bin/swa" "$TMP/kiungo/swa"
cp mifano/*.swa "$TMP/mradi/"

PASS=0; FAIL=0
jaribu() {  # $1 = maelezo, $2 = saraka ya kazi, $3 = swa, $4 = faili
    local jina; jina="$(basename "$4" .swa)"
    if ! (cd "$2" && "$3" --exe "$4") > "$TMP/p" 2> "$TMP/e"; then
        echo "SHINDWA: $1 -- kukusanya $jina"; cat "$TMP/e"; FAIL=$((FAIL+1)); return
    fi
    chmod +x "$TMP/p"
    if timeout 5 "$TMP/p" > "$TMP/out" && cmp -s "$TMP/out" "$HAZINA/mifano/$jina.tarajiwa"; then
        PASS=$((PASS+1))
    else
        echo "SHINDWA: $1 -- matokeo ya $jina"; FAIL=$((FAIL+1))
    fi
}

for f in mifano/*.swa; do
    b="$(basename "$f")"
    jaribu "njia kamili"     "$TMP/mradi"       "$TMP/mzizi/bin/swa" "$b"
    jaribu "kiungo"          "$TMP/mradi"       "$TMP/kiungo/swa"    "$b"
    jaribu "njia ya jamaa"   "$TMP"             "./mzizi/bin/swa"    "mradi/$b"
    jaribu "saraka ya ndani" "$TMP/mradi/ndani" "$TMP/mzizi/bin/swa" "../$b"
done

# Kosa bado ni la sauti: faili lisilopo popote linataja njia zote.
printf 'husisha { haipo_kabisa.swa }\nn32 main() { rudisha 0; }\n' > "$TMP/mradi/haipo.swa"
if (cd "$TMP/mradi" && "$TMP/mzizi/bin/swa" --exe haipo.swa) > /dev/null 2> "$TMP/e"; then
    echo "SHINDWA: husisha ya faili lisilopo ilikubaliwa"; FAIL=$((FAIL+1))
elif grep -q "chini ya usakinishaji: $TMP/mzizi/msingi/" "$TMP/e"; then
    PASS=$((PASS+1))
else
    echo "SHINDWA: ujumbe wa husisha haukutaja mzizi wa usakinishaji"; cat "$TMP/e"; FAIL=$((FAIL+1))
fi

echo "===== Matokeo ya usakinishaji: $PASS yamefaulu, $FAIL yameshindwa ====="
[ "$FAIL" -eq 0 ]
