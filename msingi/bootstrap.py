#!/usr/bin/env python3
"""
Jenereta ya bootstrap ya Swa — Hatua 1
Inazalisha programu ambayo inachukua chanzo cha Swa kama hoja
na kutoa ELF inayorudisha nambari inayofuata baada ya 'rudisha'.

Matumizi:
  python3 bootstrap.py                    # andika bootstrap.bin
  ./bootstrap 'N32 main() { rudisha 42; }' > out.o && ./out.o && echo $?
  # -> 42
"""

import struct
import os

# =============================================================
# Vigezo vya msingi
# =============================================================

BASE      = 0x400000
PAGE      = 0x1000
EHDR_SIZE = 64          # ukubwa wa kichwa cha ELF
PHDR_SIZE = 56          # ukubwa wa kichwa cha programu
CODE_OFF  = EHDR_SIZE + PHDR_SIZE  # mwanzo wa msimbo (0x78)

def u16(x): return struct.pack('<H', x)
def u32(x): return struct.pack('<I', x)
def u64(x): return struct.pack('<Q', x)

# =============================================================
# Kichwa cha ELF (baiti 64)
# =============================================================

def jenga_ehdr(entry, phoff, phnum):
    """Jenga kichwa cha ELF baiti 64."""
    return b''.join([
        b'\x7fELF',            # e_ident magic
        bytes([2]),             # 64-bit
        bytes([1]),             # little-endian
        bytes([1]),             # e_ident[EI_VERSION]
        bytes([0]),             # OS/ABI
        b'\x00' * 8,            # padding
        u16(2),                 # ET_EXEC
        u16(0x3E),              # x86-64
        u32(1),                 # e_version
        u64(entry),             # e_entry
        u64(phoff),             # e_phoff
        u64(0),                 # e_shoff
        u32(0),                 # e_flags
        u16(EHDR_SIZE),         # e_ehsize
        u16(PHDR_SIZE),         # e_phentsize
        u16(phnum),             # e_phnum
        u16(0),                 # e_shentsize
        u16(0),                 # e_shnum
        u16(0),                 # e_shstrndx
    ])

# =============================================================
# Kichwa cha programu PT_LOAD (baiti 56)
# =============================================================

def jenga_phdr(flags, offset, vaddr, filesz, memsz, align):
    """Jenga kichwa cha programu PT_LOAD baiti 56."""
    return b''.join([
        u32(1),                 # p_type = PT_LOAD
        u32(flags),             # p_flags
        u64(offset),            # p_offset
        u64(vaddr),             # p_vaddr
        u64(vaddr),             # p_paddr
        u64(filesz),            # p_filesz
        u64(memsz),             # p_memsz
        u64(align),             # p_align
    ])

# =============================================================
# Kizalishe cha ELF — pato linaloundwa wakati wa utekelezaji
# =============================================================

def jenga_template():
    """
    Jenga kizalishe cha ELF (132 baiti).
    Thamani ya kutoka iko kwenye template[baiti_121:125].
    """
    entry = BASE + CODE_OFF       # 0x400078
    template_total = CODE_OFF + 12  # 64 + 56 + 12 = 132

    template = b''.join([
        # --- Kichwa cha ELF (baiti 64) ---
        b'\x7fELF',
        bytes([2, 1, 1, 0]),
        b'\x00' * 8,
        u16(2),                    # ET_EXEC
        u16(0x3E),                 # x86-64
        u32(1),                    # e_version
        u64(entry),                # e_entry
        u64(64),                   # e_phoff
        u64(0),                    # e_shoff
        u32(0),                    # e_flags
        u16(64),                   # e_ehsize
        u16(56),                   # e_phentsize
        u16(1),                    # e_phnum
        u16(0),                    # e_shentsize
        u16(0),                    # e_shnum
        u16(0),                    # e_shstrndx

        # --- Kichwa cha programu PT_LOAD (baiti 56) ---
        u32(1),                    # PT_LOAD
        u32(5),                    # RX
        u64(0),                    # p_offset
        u64(BASE),                 # p_vaddr
        u64(BASE),                 # p_paddr
        u64(template_total),       # p_filesz
        u64(template_total),       # p_memsz
        u64(PAGE),                 # p_align

        # --- Msimbo (12 baiti) ---
        bytes([0xbf]),            # mov edi, imm32
        b'\x00\x00\x00\x00',     # thamani ya kutoka (itabadilishwa)
        bytes([0xb8]),            # mov eax, imm32
        u32(60),                  # sys_exit
        b'\x0f\x05',             # syscall
    ])

    assert len(template) == template_total, f"template: {len(template)} != {template_total}"
    # Thamani ya kutoka iko kwenye baiti 121-124 (CODE_OFF + 1)
    exit_off = CODE_OFF + 1  # 121
    return template, exit_off

# =============================================================
# Mkusanyaji wa msimbo (x86-64 raw bytes)
# =============================================================

class Msanidi:
    """Mkusanyaji rahisi wa msimbo wa x86-64."""

    def __init__(self, anza=0):
        self.baiti = bytearray()
        self.anza = anza          # kukabilia ya mwanzo (k.m. 0x78)
        self._lebo = {}           # lebo -> kukabilia
        self._marekebisho8 = []   # (kukabilia_ya_maagizo, lebo) kuruka REL8
        self._marekebisho32 = []  # (kukabilia_ya_maagizo, lebo) kuruka REL32

    def pato(self, data):
        """Ongeza baiti kwenye msimbo."""
        self.baiti.extend(data)

    def lebo(self, jina):
        """Weka lebo katika nafasi ya sasa."""
        self._lebo[jina] = self.anza + len(self.baiti)

    def kuruka8(self, aina, lebo):
        """
        Ongeza mruko wa REL8 (baiti 2).
        aina: 'jb','jae','je','jne','js','jns','jg','jge','jl','jle','jmp'
        """
        opcodes = {
            'jb': 0x72, 'jae': 0x73,
            'je': 0x74, 'jz': 0x74,
            'jne': 0x75, 'jnz': 0x75,
            'js': 0x78, 'jns': 0x79,
            'jg': 0x7f, 'jge': 0x7d,
            'jl': 0x7c, 'jle': 0x7e,
            'jmp': 0xeb,
        }
        self.baiti.append(opcodes[aina])
        self._marekebisho8.append((len(self.baiti), lebo))
        self.baiti.append(0x00)  # nafasi

    def kuruka32(self, aina, lebo):
        """
        Ongeza mruko wa REL32 (baiti 5-6).
        aina: 'jl','jle','jg','jge','je','jne','js','jns','jb','jae','jmp'
        """
        opcodes = {
            'jl': 0x0f8c, 'jle': 0x0f8e, 'jg': 0x0f8f, 'jge': 0x0f8d,
            'je': 0x0f84, 'jne': 0x0f85, 'js': 0x0f88, 'jns': 0x0f89,
            'jb': 0x0f82, 'jae': 0x0f83, 'jz': 0x0f84, 'jnz': 0x0f85,
        }
        if aina == 'jmp':
            self.baiti.extend([0xe9])                 # jmp rel32 (5 baiti)
            self._marekebisho32.append((len(self.baiti), lebo))
            self.baiti.extend(b'\x00\x00\x00\x00')
        else:
            opc = opcodes[aina]
            self.baiti.extend([opc >> 8, opc & 0xFF])  # 0f 8X (6 baiti)
            self._marekebisho32.append((len(self.baiti), lebo))
            self.baiti.extend(b'\x00\x00\x00\x00')

    def jaza_kuruka(self):
        """Jaza thamani za kuruka baada ya msimbo wote kujengwa."""
        for pos, lebo in self._marekebisho8:
            lengo = self._lebo[lebo]
            # RIP = anwani baada ya mruko wa baiti 2 (opcode + displacement)
            sasa = self.anza + pos + 1
            tofauti = lengo - sasa
            if tofauti < -128 or tofauti > 127:
                raise ValueError(
                    f"Kuruka8 kwa {lebo} hakutoshi: {tofauti} (baiti {pos})")
            # pos ni fahirisi ya displacement (baiti ya pili)
            self.baiti[pos] = tofauti & 0xFF

        for pos, lebo in self._marekebisho32:
            lengo = self._lebo[lebo]
            # RIP = anwani baada ya displacement ya baiti 4
            sasa = self.anza + pos + 4
            tofauti = lengo - sasa
            # pos ni fahirisi ya mwanzo wa displacement ya baiti 4
            self.baiti[pos:pos+4] = u32(tofauti & 0xFFFFFFFF)

    def urefu(self):
        return len(self.baiti)


# =============================================================
# Jenga binary kamili
# =============================================================

def jenga_bootstrap():
    # --- Jenga kizalishe cha pato ---
    template, exit_field_katika_template = jenga_template()
    template_len = len(template)    # 132

    # --- Ujumbe wa makosa ---
    ujumbe = b"Matumizi: bootstrap <chanzo>\n"
    ujumbe_len = len(ujumbe)       # 29

    # --- Msimbo ---
    m = Msanidi(anza=CODE_OFF)

    # _start: pata argc na argv
    m.pato(b'\x58\x5f')            # pop rax; pop rdi (argc, argv[0])
    m.pato(b'\x48\x83\xf8\x02')    # cmp rax, 2
    m.kuruka8('jl', 'usage')       # ikiwa argc < 2, nenda usage

    m.pato(b'\x5f')                # pop rdi (argv[1])
    m.kuruka32('jmp', 'main')      # ruka sehemu ya usage

    # ================================================================
    # .usage: onyesha ujumbe na toka (iko karibu na mwanzo kwa kuruka fupi)
    # ================================================================
    m.lebo('usage')
    # Andika ujumbe wa makosa kwenye stderr
    m.pato(b'\xbf\x02\x00\x00\x00')   # mov edi, 2 (stderr)
    m.pato(b'\x48\x8d\x35')           # lea rsi, [rip + displ]
    lea_msg_pos = m.urefu()
    m.pato(u32(0xDEADBEEF))           # nafasi
    m.pato(b'\xba' + u32(ujumbe_len)) # mov edx, len(ujumbe)
    m.pato(b'\xb8\x01\x00\x00\x00')   # mov eax, 1 (sys_write)
    m.pato(b'\x0f\x05')               # syscall
    m.pato(b'\xbf\x01\x00\x00\x00')   # mov edi, 1
    m.pato(b'\xb8\x3c\x00\x00\x00')   # mov eax, 60 (sys_exit)
    m.pato(b'\x0f\x05')               # syscall

    # ================================================================
    # .main: msimbo mkuu wa kuchanganua
    # ================================================================
    m.lebo('main')

    # Tafuta "rudisha" kwenye mfuatano
    m.pato(b'\x31\xc9')            # xor ecx, ecx (faharisi = 0)

    # .scan:
    m.lebo('scan')
    m.pato(b'\x40\x0f\xb6\x14\x0f')  # movzx edx, byte [rdi+rcx]
    m.pato(b'\x84\xd2')               # test dl, dl
    m.kuruka32('jz', 'usage')         # mwisho wa mfuatano -> hitilafu
    m.pato(b'\x80\xfa\x72')          # cmp dl, 'r' (0x72)
    m.kuruka8('jne', 'next')

    # Angalia "udisha" (baiti 6 zilizobaki)
    # dword [rdi+rcx+1] == 0x73696475 ("udis" kwa LE)
    m.pato(b'\x81\x7c\x0f\x01\x75\x64\x69\x73')
    m.kuruka8('jne', 'next')
    # word [rdi+rcx+5] == 0x6168 ("ha" kwa LE)
    m.pato(b'\x66\x81\x7c\x0f\x05\x68\x61')
    m.kuruka8('jne', 'next')

    # Tumeipata "rudisha"!
    m.pato(b'\x83\xc1\x07')        # add ecx, 7 (ruka "rudisha")
    m.kuruka8('jmp', 'skip_ws')

    # .next: endelea kutafuta
    m.lebo('next')
    m.pato(b'\xff\xc1')            # inc ecx
    m.kuruka8('jmp', 'scan')

    # .skip_ws: ruka nafasi tupu
    m.lebo('skip_ws')
    m.pato(b'\x40\x0f\xb6\x14\x0f')  # movzx edx, byte [rdi+rcx]
    m.pato(b'\x80\xfa\x20')          # cmp dl, ' '
    m.kuruka8('je', 'skip_next')
    m.pato(b'\x80\xfa\x09')          # cmp dl, 0x09 (tab)
    m.kuruka8('je', 'skip_next')
    m.pato(b'\x80\xfa\x0a')          # cmp dl, 0x0a (newline)
    m.kuruka8('je', 'skip_next')
    m.kuruka8('jmp', 'parse')         # herufi ya kwanza isiyo nafasi

    # .skip_next:
    m.lebo('skip_next')
    m.pato(b'\xff\xc1')             # inc ecx
    m.kuruka8('jmp', 'skip_ws')

    # .parse: badilisha tarakimu kuwa nambari
    m.lebo('parse')
    m.pato(b'\x31\xc0')             # xor eax, eax (matokeo = 0)

    # .parse_loop:
    m.lebo('parse_loop')
    m.pato(b'\x40\x0f\xb6\x14\x0f')  # movzx edx, byte [rdi+rcx]
    m.pato(b'\x84\xd2')               # test dl, dl
    m.kuruka8('jz', 'patch')          # mwisho
    m.pato(b'\x80\xea\x30')          # sub dl, '0'
    m.kuruka8('js', 'patch')          # si tarakimu
    m.pato(b'\x80\xfa\x09')          # cmp dl, 9
    m.kuruka8('jg', 'patch')          # si tarakimu
    m.pato(b'\x6b\xc0\x0a')          # imul eax, 10
    m.pato(b'\x01\xd0')              # add eax, edx
    m.pato(b'\xff\xc1')              # inc ecx
    m.kuruka8('jmp', 'parse_loop')

    # .patch: andika nambari kwenye template
    m.lebo('patch')

    # LEA kwa thamani ya kutoka kwenye template (itajazwa baadaye)
    m.pato(b'\x48\x8d\x3d')           # lea rdi, [rip + displ]
    lea_exit_pos = m.urefu()
    m.pato(u32(0xDEADBEEF))           # nafasi

    m.pato(b'\x89\x07')               # mov [rdi], eax

    # Andika ELF kwenye stdout
    m.pato(b'\xbf\x01\x00\x00\x00')   # mov edi, 1 (stdout)
    m.pato(b'\x48\x8d\x35')           # lea rsi, [rip + displ]
    lea_template_pos = m.urefu()
    m.pato(u32(0xDEADBEEF))           # nafasi
    m.pato(b'\xba' + u32(template_len))  # mov edx, len(template)
    m.pato(b'\xb8\x01\x00\x00\x00')   # mov eax, 1 (sys_write)
    m.pato(b'\x0f\x05')               # syscall

    # Toka kwa mafanikio
    m.pato(b'\x31\xff')               # xor edi, edi
    m.pato(b'\xb8\x3c\x00\x00\x00')   # mov eax, 60 (sys_exit)
    m.pato(b'\x0f\x05')               # syscall

    # Jaza maadili ya kuruka
    m.jaza_kuruka()

    # --- Panga data kwenye faili ---
    msimbo_ukubwa = m.urefu()
    kukabilia_ujumbe = CODE_OFF + msimbo_ukubwa
    kukabilia_template = kukabilia_ujumbe + ujumbe_len
    jumla_kubwa = kukabilia_template + template_len

    # Jaza LEA za RIP-relative
    # LEA kwa thamani ya kutoka kwenye template
    # lea_exit_pos inaelekeza kwenye displacement ya LEA (baiti ya 4)
    # RIP = anwani baada ya LEA nzima (baiti 7)
    pip = CODE_OFF + lea_exit_pos + 4
    lengo_exit = kukabilia_template + exit_field_katika_template
    m.baiti[lea_exit_pos:lea_exit_pos+4] = u32(lengo_exit - pip)

    # LEA kwa mwanzo wa template
    pip = CODE_OFF + lea_template_pos + 4
    m.baiti[lea_template_pos:lea_template_pos+4] = u32(kukabilia_template - pip)

    # LEA kwa ujumbe wa makosa
    pip = CODE_OFF + lea_msg_pos + 4
    m.baiti[lea_msg_pos:lea_msg_pos+4] = u32(kukabilia_ujumbe - pip)

    # --- Jenga binary kamili ---
    kuingia = BASE + CODE_OFF    # 0x400078
    ehdr = jenga_ehdr(kuingia, EHDR_SIZE, 1)
    phdr = jenga_phdr(7, 0, BASE, jumla_kubwa, jumla_kubwa, PAGE)

    binary = ehdr + phdr + bytes(m.baiti) + ujumbe + template

    assert len(binary) == jumla_kubwa, \
        f"Binary: {len(binary)} != {jumla_kubwa}"

    return binary, template_len, ujumbe_len


# =============================================================
# Kuu
# =============================================================

if __name__ == '__main__':
    binary, template_len, msg_len = jenga_bootstrap()

    njia = '/home/kandemark/projects/swa/msingi/bootstrap.bin'
    with open(njia, 'wb') as f:
        f.write(binary)

    os.chmod(njia, 0o755)

    msimbo_len = len(binary) - 64 - 56 - template_len - msg_len

    print(f"Bootstrap imeundwa: {len(binary)} baiti")
    print(f"  ELF header:       64 baiti")
    print(f"  Program header:   56 baiti")
    print(f"  Msimbo:           {msimbo_len} baiti")
    print(f"  Ujumbe wa makosa: {msg_len} baiti")
    print(f"  Pato la template: {template_len} baiti")
    print(f"  Jumla:            {len(binary)} baiti")
    print(f"  Imewekwa:         {njia}")
