#!/bin/bash
# mbegu-hadi-hex.sh: badilisha binary (mfano msingi/mbegu.bin) kuwa
# umbizaji la hex linalosomwa na msingi/kwanza.bin (baiti mbili za
# heksadesimali ndogo, zikitenganishwa na nafasi, baiti 16 kwa mstari).
#
# Hii ni upande wa NYUMA wa mnyororo wa uzalishaji (bin -> hex) --
# upande wa mbele (hex -> bin) tayari upo: msingi/kwanza.bin, uliothibitishwa
# na gharama/jenga-kwanza.sh. Kifaa hiki kilikuwa hakipo kabla ya
# 2026-09-24 -- kimeandaliwa kwa ajili ya kugandisha upya mbegu.bin
# (angalia CONTRIBUTING.md, "Sheria ya Mzizi wa Uaminifu").
#
# UTHIBITISHO (LAZIMA kabla ya kutumia kwa gandisho halisi): matokeo
# ya kifaa hiki dhidi ya msingi/mbegu.bin ya SASA (kabla ya mabadiliko
# yoyote) LAZIMA yawe SAWA KABISA (diff 0) na msingi/mbegu.hex
# iliyopo -- hii inathibitisha mzunguko kamili (bin->hex->bin kupitia
# kwanza.bin) unafanya kazi kwa usahihi kabla ya kukiamini kwa
# gandisho jipya.
#
# Matumizi: bash gharama/mbegu-hadi-hex.sh <faili.bin> > <faili.hex>

set -e

if [ -z "$1" ]; then
    echo "matumizi: $0 <faili.bin>" >&2
    exit 1
fi

od -An -v -tx1 "$1" \
    | tr -s ' ' \
    | sed 's/^ //' \
    | awk '{for(i=1;i<=NF;i++){printf "%s ", $i; c++; if(c%16==0)print ""}} END{if(c%16!=0)print ""}' \
    | sed 's/ $//'
