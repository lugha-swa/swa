#!/usr/bin/env python3
"""
Jenereta ya bootstrap ya Swa — Hatua 2
Inazalisha programu ambayo inachukua chanzo cha Swa kama hoja
na kutoa ELF inayoauni vigezo, marejeleo, na hesabu.

Matumizi:
  python3 bootstrap.py                     # andika bootstrap.bin
  ./bootstrap 'N32 main() { rudisha 42; }' > out.o && ./out.o && echo $?
  # -> 42
  ./bootstrap 'N32 main() { N32 x = 100; rudisha x; }' > out.o && ./out.o && echo $?
  # -> 100
  ./bootstrap 'N32 main() { N32 x = 50; N32 y = 27; rudisha x - y; }' > out.o && ./out.o && echo $?
  # -> 23
  ./bootstrap 'N32 main() { N32 x = 10; N32 y = 32; rudisha x + y; }' > out.o && ./out.o && echo $?
  # -> 42
"""

import struct
import os

BASE      = 0x400000
PAGE      = 0x1000
EHDR_SIZE = 64
PHDR_SIZE = 56
CODE_OFF  = EHDR_SIZE + PHDR_SIZE  # 0x78

UKUBWA_UJUMBE = 29
UKUBWA_SIMTAB = 256
PATO_THAMANI = 9  # offset in template (baiti 9-12)

def u16(x): return struct.pack('<H', x)
def u32(x): return struct.pack('<I', x)
def u64(x): return struct.pack('<Q', x)

def jenga_ehdr(entry, phoff, phnum):
    return b''.join([
        b'\x7fELF', bytes([2, 1, 1, 0]), b'\x00' * 8,
        u16(2), u16(0x3E), u32(1), u64(entry), u64(phoff),
        u64(0), u32(0), u16(64), u16(56), u16(phnum),
        u16(0), u16(0), u16(0)])

def jenga_phdr(flags, offset, vaddr, filesz, memsz, align):
    return b''.join([
        u32(1), u32(flags), u64(offset), u64(vaddr),
        u64(vaddr), u64(filesz), u64(memsz), u64(align)])

class Msanidi:
    def __init__(self, anza=0):
        self.baiti = bytearray()
        self.anza = anza
        self._lebo = {}
        self._marekebisho8 = []
        self._marekebisho32 = []

    def pato(self, data):
        self.baiti.extend(data)

    def lebo(self, jina):
        self._lebo[jina] = self.anza + len(self.baiti)

    def kuruka8(self, aina, lebo):
        opcodes = {
            'jb': 0x72, 'jae': 0x73, 'je': 0x74, 'jz': 0x74,
            'jne': 0x75, 'jnz': 0x75, 'js': 0x78, 'jns': 0x79,
            'jg': 0x7f, 'jge': 0x7d, 'jl': 0x7c, 'jle': 0x7e,
            'jmp': 0xeb, 'jbe': 0x76, 'ja': 0x77,
        }
        self.baiti.append(opcodes[aina])
        self._marekebisho8.append((len(self.baiti), lebo))
        self.baiti.append(0x00)

    def kuruka32(self, aina, lebo):
        opcodes = {
            'jl': 0x0f8c, 'jle': 0x0f8e, 'jg': 0x0f8f, 'jge': 0x0f8d,
            'je': 0x0f84, 'jne': 0x0f85, 'js': 0x0f88, 'jns': 0x0f89,
            'jb': 0x0f82, 'jae': 0x0f83, 'jz': 0x0f84, 'jnz': 0x0f85,
        }
        if aina == 'jmp':
            self.baiti.extend([0xe9])
            self._marekebisho32.append((len(self.baiti), lebo))
            self.baiti.extend(b'\x00\x00\x00\x00')
        else:
            opc = opcodes[aina]
            self.baiti.extend([opc >> 8, opc & 0xFF])
            self._marekebisho32.append((len(self.baiti), lebo))
            self.baiti.extend(b'\x00\x00\x00\x00')

    def wito32(self, lebo):
        self.baiti.append(0xe8)
        self._marekebisho32.append((len(self.baiti), lebo))
        self.baiti.extend(b'\x00\x00\x00\x00')

    def rudi(self):
        self.baiti.append(0xc3)

    def jaza_kuruka(self):
        for pos, lebo in self._marekebisho8:
            lengo = self._lebo.get(lebo)
            if lengo is None:
                raise ValueError(f"Lebo '{lebo}' haijapatikana (kuruka8)")
            sasa = self.anza + pos + 1
            tofauti = lengo - sasa
            if tofauti < -128 or tofauti > 127:
                raise ValueError(f"Kuruka8 kwa {lebo} hakutoshi: {tofauti}")
            self.baiti[pos] = tofauti & 0xFF
        for pos, lebo in self._marekebisho32:
            lengo = self._lebo.get(lebo)
            if lengo is None:
                raise ValueError(f"Lebo '{lebo}' haijapatikana (kuruka32/wito32)")
            sasa = self.anza + pos + 4
            tofauti = lengo - sasa
            self.baiti[pos:pos+4] = u32(tofauti & 0xFFFFFFFF)

    def urefu(self):
        return len(self.baiti)

# =============================================================
# Jenga bootstrap binary
# =============================================================

def jenga_bootstrap():
    ujumbe = b"Matumizi: bootstrap <chanzo>\n"

    # Pato template: ELF kamili (140 baiti), thamani kwenye baiti 129-132
    pato_msimbo = b''.join([
        bytes([0x55]),                     # push rbp
        bytes([0x48, 0x89, 0xe5]),         # mov rbp, rsp
        bytes([0x48, 0x83, 0xec, 0x20]),   # sub rsp, 32
        bytes([0xbf]),                     # mov edi, imm32
        b'\x00\x00\x00\x00',               # thamani (itajazwa)
        bytes([0xb8]),                     # mov eax, imm32
        u32(60),                           # sys_exit
        bytes([0x0f, 0x05]),               # syscall
    ])
    pato_ukubwa_msimbo = len(pato_msimbo)  # 20
    pato_ukubwa = 120 + pato_ukubwa_msimbo  # 140 (vichwa + msimbo)
    PATO_THAMANI_KUKABILIA = 120 + 9       # 129
    # Template kamili: ELF header + Program header + Code
    pato_template = (jenga_ehdr(BASE + 120, 64, 1)
                   + jenga_phdr(5, 0, BASE, pato_ukubwa, pato_ukubwa, PAGE)
                   + pato_msimbo)
    assert len(pato_template) == pato_ukubwa, f"Template: {len(pato_template)} != {pato_ukubwa}"

    m = Msanidi(anza=CODE_OFF)

    # ========================
    # _start: pata argc na argv
    # ========================
    m.pato(b'\x58\x5f')            # pop rax; pop rdi (argc, argv[0])
    m.pato(b'\x48\x83\xf8\x02')    # cmp rax, 2
    m.kuruka8('jl', 'usage')
    m.pato(b'\x5f')                # pop rdi (argv[1])
    m.kuruka32('jmp', 'target_parse')

    # ========================
    # usage
    # ========================
    m.lebo('usage')
    m.pato(b'\xbf\x02\x00\x00\x00')
    m.pato(b'\x48\x8d\x35')
    lea_ujumbe_pos = m.urefu()
    m.pato(u32(0xDEADBEEF))
    m.pato(b'\xba' + u32(len(ujumbe)))
    m.pato(b'\xb8\x01\x00\x00\x00')
    m.pato(b'\x0f\x05')
    m.pato(b'\xbf\x01\x00\x00\x00')
    m.pato(b'\xb8\x3c\x00\x00\x00')
    m.pato(b'\x0f\x05')

    # ========================
    # target_parse: setup
    # ========================
    m.lebo('target_parse')

    # r11 = simtab
    m.pato(b'\x49\x8d\x35')
    lea_simtab_pos = m.urefu()
    m.pato(u32(0xDEADBEEF))
    m.pato(b'\x49\x89\xf3')        # mov r11, rsi

    # var_count = 0
    m.pato(b'\x41\xc7\x03\x00\x00\x00\x00')

    # Hifadhi rdi (argv[1]), safisha simtab, rejesha rdi
    m.pato(b'\x57')                # push rdi
    m.pato(b'\x49\x8d\x7b\x08')   # lea rdi, [r11+8]
    m.pato(b'\x31\xc0')            # xor eax, eax
    m.pato(b'\xb9\x3e\x00\x00\x00')  # mov ecx, 62 (248/4)
    m.pato(b'\xf2\xab')            # rep stosd
    m.pato(b'\x5f')                # pop rdi

    # r8 = 0
    m.pato(b'\x45\x31\xc0')        # xor r8d, r8d

    # ========================
    # Ruka hadi '{'
    # ========================
    m.lebo('skip_to_brace')
    m.pato(b'\x42\x0f\xb6\x14\x07')
    m.pato(b'\x84\xd2')
    m.kuruka32('jz', 'usage')
    m.pato(b'\x80\xfa\x7b')
    m.kuruka8('je', 'body_start')
    m.pato(b'\x49\xff\xc0')
    m.kuruka8('jmp', 'skip_to_brace')

    m.lebo('body_start')
    m.pato(b'\x49\xff\xc0')        # ruka '{'

    # ========================
    # Kitanzi cha kauli
    # ========================
    m.lebo('stmt_loop')

    m.lebo('skip_ws')
    m.pato(b'\x42\x0f\xb6\x14\x07')
    m.pato(b'\x80\xfa\x20')
    m.kuruka8('je', 'sws_n')
    m.pato(b'\x80\xfa\x09')
    m.kuruka8('je', 'sws_n')
    m.pato(b'\x80\xfa\x0a')
    m.kuruka8('je', 'sws_n')
    m.pato(b'\x80\xfa\x0d')
    m.kuruka8('je', 'sws_n')
    m.kuruka8('jmp', 'stmt_d')
    m.lebo('sws_n')
    m.pato(b'\x49\xff\xc0')
    m.kuruka8('jmp', 'skip_ws')

    m.lebo('stmt_d')
    m.pato(b'\x42\x0f\xb6\x14\x07')
    m.pato(b'\x80\xfa\x7d')
    m.kuruka32('je', 'finish')
    m.pato(b'\x80\xfa\x4e')
    m.kuruka8('je', 'parse_var')
    m.pato(b'\x80\xfa\x72')
    m.kuruka8('je', 'check_ru')
    m.kuruka32('jmp', 'usage')

    # ========================
    # Angalia "rudisha"
    # ========================
    m.lebo('check_ru')
    m.pato(b'\x42\x81\x7c\x07\x01\x75\x64\x69\x73')
    m.kuruka8('jne', 'not_ru')
    m.pato(b'\x66\x42\x81\x7c\x07\x05\x68\x61')
    m.kuruka8('jne', 'not_ru')
    # Tumepata "rudisha" — ruka
    m.pato(b'\x49\x83\xc0\x07')    # add r8, 7
    m.kuruka32('jmp', 'eval_expr')

    m.lebo('not_ru')
    m.pato(b'\x49\xff\xc0')        # inc r8
    m.kuruka32('jmp', 'stmt_loop')

    # ========================
    # Parse "N32 jina = namba;"
    # ========================
    m.lebo('parse_var')
    # Tukio: r8 inaelekeza 'N' — ruka "N32" (baiti 3)
    m.pato(b'\x49\x83\xc0\x03')    # add r8, 3
    # Ruka nafasi baada ya N32
    m.lebo('pv_ws1')
    m.pato(b'\x42\x0f\xb6\x14\x07')
    m.pato(b'\x80\xfa\x20')
    m.kuruka8('jne', 'pv_rd_name')
    m.pato(b'\x49\xff\xc0')
    m.kuruka8('jmp', 'pv_ws1')

    # Soma jina la kigezo — r9 itakuwa anwani ya ingizo
    m.lebo('pv_rd_name')
    # entry_addr = simtab + 8 + var_count * 12
    m.pato(b'\x41\x8b\x0b')        # mov ecx, [r11] (var_count)
    m.pato(b'\x89\xc8')            # mov eax, ecx
    m.pato(b'\x6b\xc0\x0c')        # imul eax, 12
    m.pato(b'\x49\x8d\x73\x08')    # lea rsi, [r11+8]
    m.pato(b'\x48\x01\xc6')        # add rsi, rax (rsi = entry addr)

    # r10 = faharisi ya jina (0-7)
    m.pato(b'\x45\x31\xd2')        # xor r10d, r10d

    m.lebo('pv_cpyn')
    m.pato(b'\x42\x0f\xb6\x14\x07')
    m.pato(b'\x80\xfa\x20')
    m.kuruka8('je', 'pv_nd')
    m.pato(b'\x80\xfa\x3d')
    m.kuruka8('je', 'pv_nd')
    m.pato(b'\x80\xfa\x3b')
    m.kuruka8('je', 'pv_nd')
    m.pato(b'\x42\x88\x14\x16')    # mov [rsi+r10], dl
    m.pato(b'\x49\xff\xc0')
    m.pato(b'\x49\xff\xc2')
    m.pato(b'\x49\x83\xfa\x08')
    m.kuruka8('jb', 'pv_cpyn')

    m.lebo('pv_nd')
    m.pato(b'\x42\xc6\x04\x16\x00')  # mov byte [rsi+r10], 0

    # Ruka hadi '=', ruka nafasi
    m.lebo('pv_skeq')
    m.pato(b'\x42\x0f\xb6\x14\x07')
    m.pato(b'\x80\xfa\x3d')
    m.kuruka8('je', 'pv_ateq')
    m.pato(b'\x80\xfa\x20')
    m.kuruka8('je', 'pv_skeq_n')
    m.pato(b'\x80\xfa\x3b')
    m.kuruka32('je', 'usage')
    m.pato(b'\x80\xfa\x30')
    m.kuruka8('jb', 'pv_skeq_n')
    m.pato(b'\x80\xfa\x39')
    m.kuruka8('ja', 'pv_skeq_n')
    m.kuruka8('jmp', 'pv_parse_val')  # tarakimu moja kwa moja
    m.lebo('pv_skeq_n')
    m.pato(b'\x49\xff\xc0')
    m.kuruka8('jmp', 'pv_skeq')

    m.lebo('pv_ateq')
    m.pato(b'\x49\xff\xc0')

    m.lebo('pv_ws_v')
    m.pato(b'\x42\x0f\xb6\x14\x07')
    m.pato(b'\x80\xfa\x20')
    m.kuruka8('jne', 'pv_parse_val')
    m.pato(b'\x49\xff\xc0')
    m.kuruka8('jmp', 'pv_ws_v')

    m.lebo('pv_parse_val')
    m.pato(b'\x31\xc0')            # xor eax, eax (thamani = 0)
    m.lebo('pv_val_l')
    m.pato(b'\x42\x0f\xb6\x14\x07')
    m.pato(b'\x80\xea\x30')
    m.pato(b'\x80\xfa\x09')
    m.kuruka8('ja', 'pv_val_d')
    m.pato(b'\x6b\xc0\x0a')
    m.pato(b'\x01\xd0')
    m.pato(b'\x49\xff\xc0')
    m.kuruka8('jmp', 'pv_val_l')

    m.lebo('pv_val_d')
    m.pato(b'\x89\x46\x08')        # mov [rsi+8], eax (hifadhi thamani)
    m.pato(b'\x41\xff\x03')        # inc dword [r11] (var_count++)

    # Ruka hadi ';'
    m.lebo('pv_ss')
    m.pato(b'\x42\x0f\xb6\x14\x07')
    m.pato(b'\x80\xfa\x3b')
    m.kuruka8('je', 'pv_ss_n')
    m.pato(b'\x49\xff\xc0')
    m.kuruka8('jmp', 'pv_ss')
    m.lebo('pv_ss_n')
    m.pato(b'\x49\xff\xc0')
    m.kuruka32('jmp', 'stmt_loop')

    # ========================
    # parse_term — tathmini neno (namba au kigezo)
    # Kuingia: r8 inaelekeza kwenye neno
    # Kutoka: eax = thamani
    # ========================
    m.lebo('parse_term')

    # Ruka nafasi
    m.lebo('pt_ws')
    m.pato(b'\x42\x0f\xb6\x14\x07')
    m.pato(b'\x80\xfa\x20')
    m.kuruka8('je', 'pt_ws_n')
    m.kuruka8('jmp', 'pt_disp')
    m.lebo('pt_ws_n')
    m.pato(b'\x49\xff\xc0')
    m.kuruka8('jmp', 'pt_ws')

    m.lebo('pt_disp')
    m.pato(b'\x42\x0f\xb6\x14\x07')
    m.pato(b'\x80\xfa\x30')
    m.kuruka32('jb', 'pt_err')
    m.pato(b'\x80\xfa\x39')
    m.kuruka8('ja', 'pt_id')

    # Namba
    m.pato(b'\x31\xc0')            # xor eax, eax
    m.lebo('pt_nl')
    m.pato(b'\x42\x0f\xb6\x14\x07')
    m.pato(b'\x80\xea\x30')
    m.pato(b'\x80\xfa\x09')
    m.kuruka8('ja', 'pt_nd')
    m.pato(b'\x6b\xc0\x0a')
    m.pato(b'\x01\xd0')
    m.pato(b'\x49\xff\xc0')
    m.kuruka8('jmp', 'pt_nl')
    m.lebo('pt_nd')
    m.rudi()

    # Kitambulishi — soma jina na tafuta kwenye jedwali
    m.lebo('pt_id')
    m.pato(b'\x6a\x00')            # push 0 (bafa ya jina)
    m.pato(b'\x31\xc9')            # xor ecx, ecx

    m.lebo('pt_rn')
    m.pato(b'\x42\x0f\xb6\x14\x07')
    m.pato(b'\x80\xfa\x20')
    m.kuruka8('je', 'pt_rnd')
    m.pato(b'\x80\xfa\x2b')
    m.kuruka8('je', 'pt_rnd')
    m.pato(b'\x80\xfa\x2d')
    m.kuruka8('je', 'pt_rnd')
    m.pato(b'\x80\xfa\x2a')
    m.kuruka8('je', 'pt_rnd')
    m.pato(b'\x80\xfa\x3b')
    m.kuruka8('je', 'pt_rnd')
    m.pato(b'\x80\xfa\x7d')
    m.kuruka8('je', 'pt_rnd')
    m.pato(b'\x80\xfa\x28')
    m.kuruka8('je', 'pt_rnd')
    m.pato(b'\x80\xfa\x29')
    m.kuruka8('je', 'pt_rnd')
    # Hifadhi herufi
    m.pato(b'\x88\x14\x0c')        # mov [rsp+rcx], dl
    m.pato(b'\x49\xff\xc0')
    m.pato(b'\xff\xc1')
    m.pato(b'\x83\xf9\x08')
    m.kuruka8('jb', 'pt_rn')

    m.lebo('pt_rnd')
    m.pato(b'\x4c\x8b\x0c\x24')    # mov r9, [rsp]
    m.pato(b'\x48\x83\xc4\x08')    # add rsp, 8

    # Tafuta kwenye jedwali
    m.pato(b'\x41\x8b\x0b')        # mov ecx, [r11] (var_count)
    m.pato(b'\x45\x31\xd2')        # xor r10d, r10d

    m.lebo('pt_sl')
    m.pato(b'\x44\x39\xd1')        # cmp ecx, r10d (ecx - r10d)
    m.kuruka8('jbe', 'pt_snf')
    m.pato(b'\x4c\x89\xd0')        # mov eax, r10d
    m.pato(b'\x6b\xc0\x0c')        # imul eax, 12
    m.pato(b'\x49\x8d\x53\x08')    # lea rdx, [r11+8]
    m.pato(b'\x48\x01\xc2')        # add rdx, rax
    m.pato(b'\x4c\x39\x0a')        # cmp [rdx], r9
    m.kuruka8('je', 'pt_sf')
    m.pato(b'\x49\xff\xc2')
    m.kuruka8('jmp', 'pt_sl')

    m.lebo('pt_sf')
    m.pato(b'\x8b\x42\x08')        # mov eax, [rdx+8] (thamani)
    m.rudi()

    m.lebo('pt_snf')
    m.kuruka32('jmp', 'usage')

    m.lebo('pt_err')
    m.kuruka32('jmp', 'usage')

    # ========================
    # eval_expr: tathmini usemi
    # ========================
    m.lebo('eval_expr')
    m.wito32('parse_term')          # eax = neno la kwanza

    m.lebo('ee_ol')
    # Ruka nafasi
    m.lebo('ee_ws')
    m.pato(b'\x42\x0f\xb6\x14\x07')
    m.pato(b'\x80\xfa\x20')
    m.kuruka8('je', 'ee_ws_n')
    m.kuruka8('jmp', 'ee_op')
    m.lebo('ee_ws_n')
    m.pato(b'\x49\xff\xc0')
    m.kuruka8('jmp', 'ee_ws')

    m.lebo('ee_op')
    m.pato(b'\x42\x0f\xb6\x14\x07')
    m.pato(b'\x80\xfa\x2b')
    m.kuruka8('je', 'ee_add')
    m.pato(b'\x80\xfa\x2d')
    m.kuruka8('je', 'ee_sub')
    m.pato(b'\x80\xfa\x2a')
    m.kuruka8('je', 'ee_mul')
    m.pato(b'\x80\xfa\x3b')
    m.kuruka8('je', 'ee_done')
    m.pato(b'\x80\xfa\x7d')
    m.kuruka8('je', 'ee_done')
    m.kuruka32('jmp', 'usage')

    m.lebo('ee_add')
    m.pato(b'\x49\xff\xc0')
    m.pato(b'\x50')                # push rax (LHS)
    m.wito32('parse_term')
    m.pato(b'\x59')                # pop rcx (LHS)
    m.pato(b'\x01\xc8')            # add eax, ecx
    m.kuruka8('jmp', 'ee_ol')

    m.lebo('ee_sub')
    m.pato(b'\x49\xff\xc0')
    m.pato(b'\x50')                # push rax
    m.wito32('parse_term')
    m.pato(b'\x59')                # pop rcx
    m.pato(b'\x29\xc1')            # sub ecx, eax
    m.pato(b'\x89\xc8')            # mov eax, ecx
    m.kuruka8('jmp', 'ee_ol')

    m.lebo('ee_mul')
    m.pato(b'\x49\xff\xc0')
    m.pato(b'\x50')
    m.wito32('parse_term')
    m.pato(b'\x59')
    m.pato(b'\x0f\xaf\xc1')        # imul eax, ecx
    m.kuruka8('jmp', 'ee_ol')

    # ========================
    # eval_done: andika matokeo na toa ELF
    # ========================
    m.lebo('ee_done')
    # eax = matokeo ya usemi
    m.pato(b'\x48\x8d\x35')
    lea_pato_pos = m.urefu()
    m.pato(u32(0xDEADBEEF))        # template address
    m.pato(b'\x89\x86' + u32(120 + 9))  # mov [rsi+129], eax
    m.pato(b'\xbf\x01\x00\x00\x00')  # stdout
    m.pato(b'\xba' + u32(pato_ukubwa))
    m.pato(b'\xb8\x01\x00\x00\x00')
    m.pato(b'\x0f\x05')
    m.pato(b'\x31\xff')
    m.pato(b'\xb8\x3c\x00\x00\x00')
    m.pato(b'\x0f\x05')

    # ========================
    # finish (hakuna "rudisha")
    # ========================
    m.lebo('finish')
    # Hakuna rudisha - andika ELF yenye 0
    m.pato(b'\x48\x8d\x35')
    lea_pato2_pos = m.urefu()
    m.pato(u32(0xDEADBEEF))
    m.pato(b'\x31\xc0')
    m.pato(b'\x89\x46\x09')        # weka 0
    m.pato(b'\xbf\x01\x00\x00\x00')
    m.pato(b'\xba' + u32(pato_ukubwa))
    m.pato(b'\xb8\x01\x00\x00\x00')
    m.pato(b'\x0f\x05')
    m.pato(b'\x31\xff')
    m.pato(b'\xb8\x3c\x00\x00\x00')
    m.pato(b'\x0f\x05')

    # ========================
    # Maliza
    # ========================
    m.jaza_kuruka()

    msimbo_ukubwa = m.urefu()
    kukabilia_ujumbe = CODE_OFF + msimbo_ukubwa
    kukabilia_simtab = kukabilia_ujumbe + len(ujumbe)
    kukabilia_pato = kukabilia_simtab + UKUBWA_SIMTAB
    jumla_kubwa = kukabilia_pato + pato_ukubwa

    # Jaza maadili ya LEA
    def jaza_lea(pos, lengo):
        pip = CODE_OFF + pos + 4
        m.baiti[pos:pos+4] = u32(lengo - pip)

    jaza_lea(lea_ujumbe_pos, kukabilia_ujumbe)
    jaza_lea(lea_simtab_pos, kukabilia_simtab)
    jaza_lea(lea_pato_pos, kukabilia_pato)
    if lea_pato2_pos is not None:
        jaza_lea(lea_pato2_pos, kukabilia_pato)

    # Jenga binary kamili
    kuingia = BASE + CODE_OFF
    ehdr = jenga_ehdr(kuingia, EHDR_SIZE, 1)
    phdr = jenga_phdr(7, 0, BASE, jumla_kubwa, jumla_kubwa, PAGE)

    binary = ehdr + phdr + bytes(m.baiti) + ujumbe + b'\x00' * UKUBWA_SIMTAB + pato_template

    assert len(binary) == jumla_kubwa, f"Binary: {len(binary)} != {jumla_kubwa}"

    return binary

# =============================================================
# Kuu
# =============================================================

if __name__ == '__main__':
    binary = jenga_bootstrap()
    njia = '/home/kandemark/projects/swa/msingi/bootstrap.bin'
    with open(njia, 'wb') as f:
        f.write(binary)
    os.chmod(njia, 0o755)
    msimbo_len = len(binary) - 120 - UKUBWA_UJUMBE - UKUBWA_SIMTAB - 20
    print(f"Bootstrap imeundwa: {len(binary)} baiti")
    print(f"  Vichwa:  120 baiti")
    print(f"  Msimbo:  {msimbo_len} baiti")
    print(f"  Ujumbe:  {UKUBWA_UJUMBE} baiti")
    print(f"  Jedwali: {UKUBWA_SIMTAB} baiti")
    print(f"  Pato:    20 baiti")
    print(f"  Jumla:   {len(binary)} baiti")
    print(f"  Imewekwa: {njia}")
