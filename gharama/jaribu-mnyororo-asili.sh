#!/bin/bash
# jaribu-mnyororo-asili.sh — kiunganishi KIDOGO cha bash kinachofunga
# pengo la kuzaliwa (bootstrap) KABLA ya kuachia Swa ASILIA (
# gharama/jaribu_mnyororo.swa) kuendesha mnyororo WOTE wa majaribio
# (sehemu 2-4 za jaribu-mnyororo.sh: ujenzi wa stage1, fixpoint,
# mzunguko wa MANIFEST.txt) kupitia sys_fork/sys_execve/sys_wait4.
#
# KWA NINI FAILI HILI (bash) BADO LIPO: hakuna kinachoweza kuendesha
# chanzo cha Swa KABLA mbegu.bin haijatengenezwa (mzunguko wa
# kuzaliwa) — sawa na jenga-kwanza.sh, hii ni SHARTI isiyoweza
# kuepukika, SI uzembe. Mistari michache iliyobaki hapa (jenga
# mbegu.bin, jenga msuluhishi, tatua+kusanya jaribu_mnyororo.swa
# LENYEWE mara MOJA) ndiyo "glue" ya chini kabisa isiyoweza
# kuandikwa kwa Swa kwa ufafanuzi — baada ya hapo, EXEC inaachia
# programu ya Swa KABISA kuendelea na kazi NZIMA.
set -eu
cd "$(dirname "$0")/.."

bash gharama/jenga-kwanza.sh > /dev/null
MBEGU="/tmp/mbegu2.bin"

TMP="$(mktemp -d /tmp/swa-mnyororo-asili-boot-XXXXXX)"
# HAKUNA `trap ... EXIT` ya kusafisha $TMP hapa KWA MAKUSUDI: `exec`
# ya mwisho ya faili hili HUBADILISHA mchakato huu wa bash KABISA --
# bash yenyewe HAIPO tena baada ya hapo, kwa hiyo EXIT trap YOYOTE
# HAINGEWAHI kuchochewa. $TMP (ikiwa ni pamoja na msuluhishi
# iliyojengwa) LAZIMA ibaki kwenye diski -- programu ya Swa
# inayofuata inahitaji njia yake (argv[2]) kuendelea kutumika.

cat msingi/maktaba/kumbukumbu.swa msingi/maktaba/mfuatano.swa \
    msingi/maktaba/faili.swa gharama/msuluhishi.swa > "$TMP/msulu.swa"
"$MBEGU" --exe "$TMP/msulu.swa" > "$TMP/msuluhishi"
chmod +x "$TMP/msuluhishi"

"$TMP/msuluhishi" gharama/jaribu_mnyororo.swa > "$TMP/zima.swa"
"$MBEGU" --exe "$TMP/zima.swa" > "$TMP/jaribu_mnyororo"
chmod +x "$TMP/jaribu_mnyororo"

# Kutoka hapa kuendelea, "$TMP/jaribu_mnyororo" (programu ya SWA
# ASILIA) ndiyo inayoendesha kila kitu kingine -- ujenzi wa stage1
# halisi, fixpoint, na mzunguko mzima wa MANIFEST.txt -- kupitia
# fork/execve/wait4, bila bash yoyote zaidi.
exec "$TMP/jaribu_mnyororo" "$MBEGU" "$TMP/msuluhishi"
