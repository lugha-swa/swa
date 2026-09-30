#!/usr/bin/env python3
# oracle.py -- oracle ya QEMU+KVM kwa gharama/jaribu-kernel.sh.
#
# Huchukua binary iliyokamilika (stub.s + kernel_main iliyotolewa na
# "stage1 --kernel" kupitia incbin), huianzisha kwenye QEMU (-kernel,
# hakuna BIOS/GRUB -- muundo wa a.out kludge wa Multiboot1 unaruhusu
# hili), na kusoma HALI HALISI ya CPU/kumbukumbu kupitia monitor socket
# (human monitor protocol) baada ya HLT -- SI "haikuanguka", ni
# kulinganisha thamani halisi na tarajiwa.
#
# Matumizi: oracle.py <binary.bin> <rdi-tarajiwa> [maandishi-ya-vga]
#
# [maandishi-ya-vga] hutumia "\n" halisi (baiti mbili: 0x5c 0x6e)
# kuwakilisha mstari mpya -- huvunjwa kuwa safu-mlalo (rows), kila
# mstari ukilinganishwa kuanzia safuwima (column) 0 ya safu-mlalo
# yake (kioo cha jinsi kernel_vga_andika_mstari_mpya inavyosogeza
# kaka -- angalia uzalishaji.swa).
#
# Uthibitisho:
#   - "info registers": RDI LAZIMA iwe sawa na <rdi-tarajiwa> (n32
#     ya kurudi ya kernel_main, iliyohamishwa kwenda edi na stub.s
#     mara moja baada ya "call kernel_main").
#   - Ikiwa [maandishi-ya-vga] limetolewa: kila mstari (baada ya
#     kugawanywa kwa "\n") LAZIMA uonekane kwenye safu-mlalo yake
#     (0xb8000 + safu_mlalo*80*2), kila herufi ikifuatiwa na baiti
#     ya sifa 0x0f (nyeupe-mkali juu ya nyeusi -- ndiyo thamani ya
#     kudumu ya kernel_vga_andika_herufi kwenye uzalishaji.swa).
import re
import socket
import subprocess
import sys
import time

def toa(msg):
    sys.stderr.write(msg + "\n")
    sys.exit(1)

if len(sys.argv) < 3:
    toa("matumizi: oracle.py <binary.bin> <rdi-tarajiwa> [maandishi-ya-vga]")

BIN = sys.argv[1]
RDI_TARAJIWA = int(sys.argv[2])
VGA_TARAJIWA = sys.argv[3] if len(sys.argv) > 3 else None

SOCK = "/tmp/swa-jaribu-kernel-%d.sock" % (int(time.time() * 1000) % 1000000)

qemu = subprocess.Popen([
    "qemu-system-x86_64", "-accel", "kvm", "-kernel", BIN, "-m", "64",
    "-display", "none", "-serial", "none",
    "-monitor", "unix:%s,server,nowait" % SOCK,
    "-no-reboot", "-no-shutdown",
], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

def safisha():
    qemu.terminate()
    try:
        qemu.wait(timeout=2)
    except subprocess.TimeoutExpired:
        qemu.kill()

s = None
try:
    kwa_muda = 0
    while kwa_muda < 30:
        try:
            s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
            s.connect(SOCK)
            break
        except (FileNotFoundError, ConnectionRefusedError):
            s = None
            time.sleep(0.1)
            kwa_muda += 1
    if s is None:
        safisha()
        toa("KOSA: monitor socket ya QEMU haikuwahi kuonekana (%s)" % SOCK)

    time.sleep(0.3)
    s.recv(65536)

    def cmd(c):
        s.sendall((c + "\n").encode())
        time.sleep(0.4)
        return s.recv(65536).decode(errors="replace")

    regs = cmd("info registers")

    m = re.search(r"RDI=([0-9a-fA-F]+)", regs)
    if not m:
        safisha()
        toa("KOSA: RDI haikupatikana kwenye 'info registers':\n%s" % regs)
    rdi = int(m.group(1), 16)

    hlt = re.search(r"HLT=(\d)", regs)
    if not hlt or hlt.group(1) != "1":
        safisha()
        toa("KOSA: CPU haikufika HLT (kernel_main haikurudi/ilianguka?):\n%s" % regs)

    if rdi != RDI_TARAJIWA:
        safisha()
        toa("KOSA: RDI=%d (0x%x), ilitarajiwa %d (0x%x)\n%s" %
            (rdi, rdi, RDI_TARAJIWA, RDI_TARAJIWA, regs))

    if VGA_TARAJIWA is not None:
        mistari = VGA_TARAJIWA.split("\\n")
        for row, mstari in enumerate(mistari):
            if mstari == "":
                continue
            base = 0xb8000 + row * 80 * 2
            n = len(mstari) * 2
            out = cmd("xp /%dxb 0x%x" % (n, base))
            # Soma TU mistari ya matokeo halisi ya kumbukumbu (mfano
            # "000b8000: 0x4b 0x0f") -- SI mstari wa echo ya amri
            # iliyoandikwa (ambayo yenyewe ina "0xb8000" ndani yake,
            # kikiwa na kipande "0xb8" kinacholingana na regex ya
            # baiti kimakosa ikiwa hatuchujii kwa mstari sahihi).
            baiti = []
            for ln in out.splitlines():
                if not re.match(r"^[0-9a-f]{8}:", ln.strip()):
                    continue
                baiti.extend(int(x, 16) for x in re.findall(r"0x([0-9a-f]{2})", ln))
            if len(baiti) < n:
                safisha()
                toa("KOSA: xp ilirudisha baiti chache mno kwenye safu %d (%d < %d):\n%s" %
                    (row, len(baiti), n, out))
            halisi = ""
            kwa_wima = True
            for i, ch in enumerate(mstari):
                herufi_b = baiti[i * 2]
                sifa_b = baiti[i * 2 + 1]
                halisi += chr(herufi_b)
                if sifa_b != 0x0f:
                    kwa_wima = False
            if halisi != mstari or not kwa_wima:
                safisha()
                toa("KOSA: VGA safu %d ilionyesha %r (sifa sahihi=%s), ilitarajiwa %r\n%s" %
                    (row, halisi, kwa_wima, mstari, out))

    safisha()
    sys.exit(0)
finally:
    if s is not None:
        try:
            s.close()
        except OSError:
            pass
