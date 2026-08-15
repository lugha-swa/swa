#!/bin/bash
# Mnyororo wa kwanza: baiti za mkono → mbegu.bin (bila NASM)
# Hakuna zana za nje — bash na syscalls pekee.
# Kwanza (msingi/kwanza.bin) ni program ya baiti za mkono inayobadilisha
# hex → binary. Inajijenga yenyewe na inazalisha mbegu.bin.
cd "$(dirname "$0")/.." || exit 1

# 1. Kwanza inajithibitisha: kwanza.hex → kwanza2.bin == kwanza.bin
./msingi/kwanza.bin < msingi/kwanza.hex > /tmp/kwanza2.bin
cmp msingi/kwanza.bin /tmp/kwanza2.bin || {
    echo "hitilafu: kwanza haijijengi sawa"
    exit 1
}
echo "kwanza inajijenga sawa"

# 2. Kwanza inazalisha mbegu.bin kutoka mbegu.hex
./msingi/kwanza.bin < msingi/mbegu.hex > /tmp/mbegu2.bin
chmod +x /tmp/mbegu2.bin
cmp msingi/mbegu.bin /tmp/mbegu2.bin || {
    echo "hitilafu: mbegu haizaliki sawa kutoka hex"
    exit 1
}
echo "mbegu.bin inazalika kutoka baiti za mkono"

exit 0
