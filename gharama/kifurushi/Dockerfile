# Kifurushi cha Swa: mkusanyaji na maktaba yake, hakuna mfumo wa
# uendeshaji ndani. bin/swa haitumii libc wala kiungo chochote cha
# nje (syscalls pekee), na hupata maktaba yake chini ya mzizi wake
# (/opt/swa/msingi/maktaba), kwa hiyo picha ni baiti zetu tu.
#
# Muktadha wa ujenzi ni saraka ya tarball ya toleo
# (swa-<toleo>-linux-x86_64/), ambayo ina bin/ na msingi/.
#
#   docker run --rm -v "$PWD:/kazi" ghcr.io/lugha-swa/swa --exe p.swa > p
FROM scratch
LABEL org.opencontainers.image.source="https://github.com/lugha-swa/swa"
LABEL org.opencontainers.image.description="Swa -- lugha ya kupanga ya mifumo kwa Kiswahili"
LABEL org.opencontainers.image.licenses="MIT OR Apache-2.0"
COPY bin/swa /opt/swa/bin/swa
COPY msingi /opt/swa/msingi
WORKDIR /kazi
ENTRYPOINT ["/opt/swa/bin/swa"]
