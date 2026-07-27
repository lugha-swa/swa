#!/usr/bin/env python3
"""
Mkusanyaji wa Swa → x86-64 ELF (Python)
Hatua ya 0 ya bootstrap — inachukua nafasi ya bootstrap.py

Hukusanya chanzo cha Swa hadi ELF binary inayoweza kutekelezwa.
Imeandikwa kwa Python 3, hakuna tegemezi za nje.

Vipengele vinavyoungwa mkono:
  Aina: N32, N64, N8, W0, N8*
  Udhibiti: kama/sivyo, wakati, rudisha
  Hesabu: +, -, *, /, %, <, >, <=, >=, ==, !=
  Kazi: ufafanuzi wenye vigezo, wito
  Vielekezi: N8*, tupu
  Safu: [] kwenye N8*
"""

import struct
import sys
import os

# ============================================================
# 1. Mzalishaji wa x86-64
# ============================================================

class X64:
    """Mzalishaji wa msimbo wa mashine wa x86-64."""
    def __init__(self):
        self.b = bytearray()
        self.labels = {}
        self.fixups_32 = []

    def emit(self, *args):
        for a in args:
            if isinstance(a, int):
                self.b.append(a & 0xFF)
            else:
                self.b.extend(a)

    def label(self, name):
        self.labels[name] = len(self.b)

    def pos(self):
        return len(self.b)

    def i32(self, v):
        self.b.extend(struct.pack('<i', v))

    def u32(self, v):
        self.b.extend(struct.pack('<I', v))

    def placeholder4(self):
        p = len(self.b)
        self.b.extend(b'\x00\x00\x00\x00')
        return p

    # -- rejesta --
    def mov_rr(self, dst, src):
        """mov dst, src (64-bit). rex_w + 89 + modrm"""
        regs = {'rax':0,'rcx':1,'rdx':2,'rbx':3,'rsp':4,'rbp':5,'rsi':6,'rdi':7,
                'r8':0,'r9':1,'r10':2,'r11':3,'r12':4,'r13':5,'r14':6,'r15':7}
        d, s = regs[dst], regs[src]
        if dst.startswith('r') and dst[1:].isdigit():
            self.emit(0x4d, 0x89, 0xc0 | (s << 3) | d)
        elif src.startswith('r') and src[1:].isdigit():
            self.emit(0x4c, 0x89, 0xc0 | (d << 3) | s)
        else:
            self.emit(0x48, 0x89, 0xc0 | (s << 3) | d)

    def mov_ri(self, reg, imm):
        """mov reg, imm64"""
        r = {'rax':0,'rcx':1,'rdx':2,'rbx':3,'rsp':4,'rbp':5,'rsi':6,'rdi':7}.get(reg,0)
        self.emit(0x48, 0xb8 | r)
        self.b.extend(struct.pack('<q', imm))

    def mov_rbp_off(self, reg, off, size=4):
        """mov reg, [rbp-off] (load from stack)"""
        r = {'eax':0,'rax':0,'ecx':1,'edx':2,'esi':6,'edi':7}.get(reg, 0)
        modrm = 0x40 | (r << 3) | 5  # mod=01, reg=r, rm=rbp
        if size == 8:
            self.emit(0x48, 0x8b, modrm, (256 - off) & 0xFF)
        else:
            self.emit(0x8b, modrm, (256 - off) & 0xFF)

    def mov_off_rbp(self, off, reg, size=4):
        """mov [rbp-off], reg (store to stack)."""
        rmap = {'eax':0,'rax':0,'ecx':1,'edx':2,'ebx':3,'esi':6,'edi':7}
        r = rmap.get(reg, 0)
        modrm = 0x40 | (r << 3) | 5  # mod=01, reg=r, rm=rbp
        if size == 8:
            self.emit(0x48, 0x89, modrm, (256 - off) & 0xFF)
        else:
            self.emit(0x89, modrm, (256 - off) & 0xFF)

    def mov_eax_imm(self, v):
        self.emit(0xb8); self.i32(v)

    def xor_reg(self, reg):
        r = {'eax':0,'rax':0,'ecx':1,'edi':7}.get(reg,0)
        self.emit(0x31, 0xc0 | (r << 3) | r)

    # -- rafu --
    def push(self, reg='rax'):
        r = {'rax':0x50,'rcx':0x51,'rdx':0x52,'rbx':0x53,'rbp':0x55,'rdi':0x57,'rsi':0x56}.get(reg,0x50)
        self.emit(r)

    def pop(self, reg='rax'):
        r = {'rax':0x58,'rcx':0x59,'rdx':0x5a,'rbx':0x5b,'rbp':0x5d,'rdi':0x5f,'rsi':0x5e}.get(reg,0x58)
        self.emit(r)

    def push_rbp(self): self.emit(0x55)
    def mov_rbp_rsp(self): self.emit(0x48, 0x89, 0xe5)
    def leave(self): self.emit(0xc9)
    def ret(self): self.emit(0xc3)
    def syscall(self): self.emit(0x0f, 0x05)

    def sub_rsp(self, n):
        if n < 128:
            self.emit(0x48, 0x83, 0xec, n)
        else:
            self.emit(0x48, 0x81, 0xec); self.i32(n)

    def add_rsp(self, n):
        if n < 128:
            self.emit(0x48, 0x83, 0xc4, n)
        else:
            self.emit(0x48, 0x81, 0xc4); self.i32(n)

    # -- hesabu --
    def add_eax_ecx(self): self.emit(0x01, 0xc8)
    def sub_eax_ecx(self): self.emit(0x29, 0xc8)  # eax -= ecx
    def imul_eax_ecx(self): self.emit(0x0f, 0xaf, 0xc1)
    def cdq(self): self.emit(0x99)
    def idiv_ecx(self): self.emit(0xf7, 0xf9)
    def cmp_eax_ecx(self): self.emit(0x39, 0xc8)

    def add_rax_rcx(self): self.emit(0x48, 0x01, 0xc8)
    def sub_rax_rcx(self): self.emit(0x48, 0x29, 0xc8)
    def cmp_rax_rcx(self): self.emit(0x48, 0x39, 0xc8)
    def imul_rax_rcx(self): self.emit(0x48, 0x0f, 0xaf, 0xc1)
    def cqo(self): self.emit(0x48, 0x99)
    def idiv_rcx(self): self.emit(0x48, 0xf7, 0xf9)

    # -- setCC --
    def sete_al(self): self.emit(0x0f, 0x94, 0xc0)
    def setne_al(self): self.emit(0x0f, 0x95, 0xc0)
    def setl_al(self): self.emit(0x0f, 0x9c, 0xc0)
    def setg_al(self): self.emit(0x0f, 0x9f, 0xc0)
    def setle_al(self): self.emit(0x0f, 0x9e, 0xc0)
    def setge_al(self): self.emit(0x0f, 0x9d, 0xc0)
    def movzx_eax_al(self): self.emit(0x0f, 0xb6, 0xc0)

    # -- kuruka --
    def jmp_rel32(self, label):
        self.emit(0xe9)
        self.fixups_32.append((len(self.b), label))
        self.b.extend(b'\x00\x00\x00\x00')

    def je_rel32(self, label):
        self.emit(0x0f, 0x84)
        self.fixups_32.append((len(self.b), label))
        self.b.extend(b'\x00\x00\x00\x00')

    def jne_rel32(self, label):
        self.emit(0x0f, 0x85)
        self.fixups_32.append((len(self.b), label))
        self.b.extend(b'\x00\x00\x00\x00')

    def jl_rel32(self, label):
        self.emit(0x0f, 0x8c)
        self.fixups_32.append((len(self.b), label))
        self.b.extend(b'\x00\x00\x00\x00')

    def jg_rel32(self, label):
        self.emit(0x0f, 0x8f)
        self.fixups_32.append((len(self.b), label))
        self.b.extend(b'\x00\x00\x00\x00')

    def jle_rel32(self, label):
        self.emit(0x0f, 0x8e)
        self.fixups_32.append((len(self.b), label))
        self.b.extend(b'\x00\x00\x00\x00')

    def jge_rel32(self, label):
        self.emit(0x0f, 0x8d)
        self.fixups_32.append((len(self.b), label))
        self.b.extend(b'\x00\x00\x00\x00')

    def call_rel32(self, label):
        self.emit(0xe8)
        self.fixups_32.append((len(self.b), label))
        self.b.extend(b'\x00\x00\x00\x00')

    def test_eax_eax(self): self.emit(0x85, 0xc0)
    def test_rax_rax(self): self.emit(0x48, 0x85, 0xc0)

    def jz_rel8(self, target_label, current_pos_for_fixup):
        """Jump if zero, rel8. Caller computes offset."""
        pass  # use je_rel32 for simplicity

    def resolve_fixups(self):
        for pos, label in self.fixups_32:
            target = self.labels.get(label)
            if target is None:
                # Lebo ya nje (mf. malloc, free, printf) —
                # acha kama ilivyo; kiunganishi kitashughulikia
                continue
            delta = target - (pos + 4)
            self.b[pos:pos+4] = struct.pack('<i', delta)


# ============================================================
# 2. ELF wrapper
# ============================================================

BASE = 0x400000
PAGE = 0x1000

# ============================================================
# Visaidizi vya syscall (badala ya libc)
# ============================================================

def syscall_open():
    """sys_open(path, flags, mode) — syscall 2"""
    return bytes([
        0xb8, 0x02, 0x00, 0x00, 0x00,  # mov eax, 2
        0x0f, 0x05,                      # syscall
        0xc3,                            # ret
    ])

def syscall_read():
    """sys_read(fd, buf, count) — syscall 0"""
    return bytes([
        0xb8, 0x00, 0x00, 0x00, 0x00,  # mov eax, 0
        0x0f, 0x05,                      # syscall
        0xc3,                            # ret
    ])

def syscall_write():
    """sys_write(fd, buf, count) — syscall 1"""
    return bytes([
        0xb8, 0x01, 0x00, 0x00, 0x00,  # mov eax, 1
        0x0f, 0x05,                      # syscall
        0xc3,                            # ret
    ])

def syscall_close():
    """sys_close(fd) — syscall 3"""
    return bytes([
        0xb8, 0x03, 0x00, 0x00, 0x00,  # mov eax, 3
        0x0f, 0x05,                      # syscall
        0xc3,                            # ret
    ])

def syscall_exit():
    """sys_exit(code) — syscall 60"""
    return bytes([
        0xb8, 0x3c, 0x00, 0x00, 0x00,  # mov eax, 60
        0x0f, 0x05,                      # syscall
    ])

def syscall_mmap(addr_reg='rdi', length_reg='rsi'):
    """sys_mmap(addr, length, prot, flags, fd, offset) — syscall 9
    Inapokea hoja kwenye rejesta. Hii ni rahisi zaidi kuiweka
    kwenye muktadha wa wito badala ya kama kazi huru."""
    return bytes([
        0xb8, 0x09, 0x00, 0x00, 0x00,  # mov eax, 9
        0x0f, 0x05,                      # syscall
    ])


def make_elf(code_bytes):
    """Funga msimbo wa mashine kwenye ELF executable inayojitosheleza."""
    entry = BASE + 0x78
    ehdr = b'\x7fELF\x02\x01\x01\x00' + b'\x00'*8
    ehdr += struct.pack('<HHIQQQIHHHHHH',
        2, 0x3E, 1, entry, 64, 0, 0, 64, 56, 1, 0, 0, 0)

    total_file_size = 64 + 56 + len(code_bytes)
    memsz = len(code_bytes) + 0x1000
    phdr = struct.pack('<IIQQQQQQ', 1, 7, 0, BASE, BASE, total_file_size, memsz, PAGE)

    return ehdr + phdr + code_bytes


# ============================================================
# 3. Mfumo wa Aina
# ============================================================

class Aina:
    def __init__(self, jina, ukubwa=4, ni_kielekezi=False):
        self.jina = jina
        self.ukubwa = ukubwa
        self.ni_kielekezi = ni_kielekezi

AINA_TUPU = Aina('W0', 0)
AINA_N32 = Aina('N32', 4)
AINA_N64 = Aina('N64', 8)
AINA_N8 = Aina('N8', 1)
AINA_N8P = Aina('N8*', 8, True)

def aina_kutoka_jina(jina):
    if jina == 'N32': return AINA_N32
    if jina == 'N64': return AINA_N64
    if jina == 'N8': return AINA_N8
    if jina == 'W0': return AINA_TUPU
    if jina == 'N8_star' or jina == 'N8*': return AINA_N8P
    # Vielekezi vingi: N8**, N32*, n.k.
    if '_star' in jina or jina.endswith('*'):
        return Aina(jina, 8, True)
    return Aina(jina, 4)


# ============================================================
# 4. Msomaji (Lexer)
# ============================================================

TOK_NAMBARI = 'nambari'
TOK_JINA = 'jina'
TOK_MANENO = 'neno_muhimu'
TOK_ISHARA = 'ishara'

MANENO_MSINGI = {
    'kama', 'sivyo', 'wakati', 'kwa', 'rudisha', 'vunja', 'endelea',
    'muundo', 'chagua', 'hali', 'tupu', 'tenga', 'achilia', 'husisha',
    'ukubwa', 'kweli', 'uongo',
}

class Tokeni:
    def __init__(self, aina, thamani, mstari=0):
        self.aina = aina
        self.thamani = thamani
        self.mstari = mstari

class Msomaji:
    def __init__(self, chanzo):
        self.src = chanzo
        self.pos = 0
        self.mstari = 1

    def herufi(self, off=0):
        i = self.pos + off
        return self.src[i] if i < len(self.src) else '\0'

    def advance(self, n=1):
        for _ in range(n):
            if self.pos < len(self.src) and self.src[self.pos] == '\n':
                self.mstari += 1
            self.pos += 1

    def ruka_nafasi(self):
        while self.herufi() in ' \t\n\r':
            self.advance()

    def ruka_maelezo(self):
        if self.herufi() == '/' and self.herufi(1) == '/':
            while self.herufi() not in ('\n', '\0'):
                self.advance()

    def ruka_nafasi_na_maelezo(self):
        while True:
            start = self.pos
            self.ruka_nafasi()
            self.ruka_maelezo()
            if self.pos == start:
                break

    def soma_neno(self):
        self.ruka_nafasi_na_maelezo()
        c = self.herufi()
        if not (c.isalpha() or c == '_'):
            return None
        start = self.pos
        while self.herufi().isalnum() or self.herufi() == '_':
            self.advance()
        neno = self.src[start:self.pos]
        if neno in MANENO_MSINGI:
            return Tokeni(TOK_MANENO, neno)
        return Tokeni(TOK_JINA, neno)

    def soma_nambari(self):
        self.ruka_nafasi_na_maelezo()
        if not self.herufi().isdigit():
            return None
        v = 0
        while self.herufi().isdigit():
            v = v * 10 + (ord(self.herufi()) - 48)
            self.advance()
        return Tokeni(TOK_NAMBARI, v)

    def tarajia_herufi(self, c):
        self.ruka_nafasi_na_maelezo()
        if self.herufi() == c:
            self.advance()
            return True
        return False

    def tarajia_neno(self, neno):
        self.ruka_nafasi_na_maelezo()
        start = self.pos
        for i, ch in enumerate(neno):
            if self.herufi(i) != ch:
                return False
        end = start + len(neno)
        if end < len(self.src) and (self.src[end].isalnum() or self.src[end] == '_'):
            return False
        self.pos = end
        return True

    def angalia_neno(self, neno):
        """Angalia kama neno linalofuata ni husika, bila kusogeza nafasi."""
        self.ruka_nafasi_na_maelezo()
        for i, ch in enumerate(neno):
            if self.herufi(i) != ch:
                return False
        end = self.pos + len(neno)
        if end < len(self.src) and (self.src[end].isalnum() or self.src[end] == '_'):
            return False
        return True

    def angalia_herufi(self, c):
        self.ruka_nafasi_na_maelezo()
        return self.herufi() == c


# ============================================================
# 5. Mchanganuzi (Parser) na AST
# ============================================================

class Nodi:
    pass

class Programu(Nodi):
    def __init__(self, kazi, miundo=None):
        self.kazi = kazi  # dict: jina -> Kazi
        self.miundo = miundo or {}  # dict: jina -> Muundo

class Kazi(Nodi):
    def __init__(self, jina, aina_ya_kurudi, vigezo, mwili):
        self.jina = jina
        self.aina_ya_kurudi = aina_ya_kurudi
        self.vigezo = vigezo  # list of (jina, Aina)
        self.mwili = mwili    # list of Taarifa

class Tangazo(Nodi):
    def __init__(self, jina, aina, kianzio):
        self.jina = jina
        self.aina = aina
        self.kianzio = kianzio  # Usemi or None

class Rudisha(Nodi):
    def __init__(self, usemi):
        self.usemi = usemi  # Usemi or None

class Kama(Nodi):
    def __init__(self, hali, basi, sivyo=None):
        self.hali = hali
        self.basi = basi
        self.sivyo = sivyo

class Wakati(Nodi):
    def __init__(self, hali, mwili):
        self.hali = hali
        self.mwili = mwili

class Vunja(Nodi): pass
class Endelea(Nodi): pass
class Wito(Nodi):
    def __init__(self, jina, hoja):
        self.jina = jina
        self.hoja = hoja  # list of Usemi

class Nambari(Nodi):
    def __init__(self, thamani):
        self.thamani = thamani

class Kitambulisho(Nodi):
    def __init__(self, jina):
        self.jina = jina

class Operesheni(Nodi):
    def __init__(self, op, kushoto, kulia):
        self.op = op
        self.kushoto = kushoto
        self.kulia = kulia

class Muundo(Nodi):
    def __init__(self, jina, sehemu):
        self.jina = jina
        self.sehemu = sehemu  # list of (jina, Aina)

class Sehemu(Nodi):
    def __init__(self, msingi, sehemu_jina, ni_mshale=False):
        self.msingi = msingi      # usemi wa msingi (kitambulisho au *)
        self.sehemu_jina = sehemu_jina  # jina la sehemu
        self.ni_mshale = ni_mshale      # True kwa ->, False kwa .

class Tenga(Nodi):
    def __init__(self, ukubwa):
        self.ukubwa = ukubwa

class Achilia(Nodi):
    def __init__(self, ptr):
        self.ptr = ptr

class Tupu(Nodi): pass

class Ugawaji(Nodi):
    def __init__(self, jina, usemi):
        self.jina = jina
        self.usemi = usemi


class Mchanganuzi:
    def __init__(self, chanzo):
        self.lex = Msomaji(chanzo)

    def hitilafu(self, ujumbe):
        raise SyntaxError(f"Mstari {self.lex.mstari}: {ujumbe}")

    def changanua_aina(self):
        """Changanua aina. Rudisha None ikiwa si aina inayojulikana."""
        pos_awali = self.lex.pos
        tok = self.lex.soma_neno()
        if tok is None:
            return None
        jina = tok.thamani
        # Angalia kama ni aina inayojulikana
        aina_zinazojulikana = {'N32', 'N64', 'N8', 'W0', 'N8*', 'N16', 'A32', 'A64',
                               'D32', 'D64', 'B1', 'B8', 'B16', 'B32', 'B64'}
        if jina not in aina_zinazojulikana:
            # Rudisha nafasi — si aina
            self.lex.pos = pos_awali
            return None
        if self.lex.angalia_herufi('*'):
            self.lex.advance()
            jina += '_star'
            while self.lex.angalia_herufi('*'):
                self.lex.advance()
                jina += '_star'
        return aina_kutoka_jina(jina)

    def changanua_usemi_msingi(self):
        """Usemi wa msingi: nambari, kitambulisho, wito, (usemi)"""
        self.lex.ruka_nafasi_na_maelezo()
        # Nambari
        if self.lex.herufi().isdigit():
            tok = self.lex.soma_nambari()
            return Nambari(tok.thamani)

        # ( usemi )
        if self.lex.angalia_herufi('('):
            self.lex.advance()
            expr = self.changanua_usemi()
            self.lex.tarajia_herufi(')')
            return expr

        # tupu
        if self.lex.angalia_neno('tupu'):
            self.lex.soma_neno()
            return Tupu()

        # tenga(ukubwa)
        if self.lex.angalia_neno('tenga'):
            self.lex.soma_neno()
            self.lex.tarajia_herufi('(')
            ukubwa = self.changanua_usemi()
            self.lex.tarajia_herufi(')')
            return Tenga(ukubwa)

        # achilia(ptr)
        if self.lex.angalia_neno('achilia'):
            self.lex.soma_neno()
            self.lex.tarajia_herufi('(')
            ptr = self.changanua_usemi()
            self.lex.tarajia_herufi(')')
            return Achilia(ptr)

        # badili(ptr, size) — realloc
        if self.lex.angalia_neno('badili'):
            self.lex.soma_neno()
            self.lex.tarajia_herufi('(')
            ptr = self.changanua_usemi()
            self.lex.tarajia_herufi(',')
            size = self.changanua_usemi()
            self.lex.tarajia_herufi(')')
            return Wito('badili', [ptr, size])

        # Jina au wito
        tok = self.lex.soma_neno()
        if tok is None:
            return None

        # Wito wa kazi?
        if self.lex.angalia_herufi('('):
            self.lex.advance()
            hoja = []
            if not self.lex.angalia_herufi(')'):
                hoja.append(self.changanua_usemi())
                while self.lex.tarajia_herufi(','):
                    hoja.append(self.changanua_usemi())
            self.lex.tarajia_herufi(')')
            return Wito(tok.thamani, hoja)

        return Kitambulisho(tok.thamani)

    def changanua_usemi_juu(self):
        """Safu, uga: expr[expr], expr.jina, expr->jina"""
        node = self.changanua_usemi_msingi()
        if node is None:
            return None

        while True:
            # [ faharasa ]
            if self.lex.angalia_herufi('['):
                self.lex.advance()
                idx = self.changanua_usemi()
                self.lex.tarajia_herufi(']')
                node = Operesheni('taja', node, idx)
                continue
            # .sehemu
            if self.lex.angalia_herufi('.'):
                self.lex.advance()
                tok = self.lex.soma_neno()
                if tok:
                    node = Sehemu(node, tok.thamani, ni_mshale=False)
                continue
            # ->sehemu
            if self.lex.angalia_herufi('-') and self.lex.herufi(1) == '>':
                self.lex.advance(); self.lex.advance()
                tok = self.lex.soma_neno()
                if tok:
                    node = Sehemu(node, tok.thamani, ni_mshale=True)
                continue
            break

        return node

    def changanua_usemi_hesabu(self):
        """*, /, %"""
        node = self.changanua_usemi_juu()
        if node is None: return None

        while True:
            op = None
            if self.lex.angalia_herufi('*'): op = '*'
            elif self.lex.angalia_herufi('/'): op = '/'
            elif self.lex.angalia_herufi('%'): op = '%'
            else: break
            self.lex.advance()
            right = self.changanua_usemi_juu()
            node = Operesheni(op, node, right)
        return node

    def changanua_usemi(self):
        """+, -, ulinganishi, &&"""
        node = self.changanua_usemi_hesabu()
        if node is None: return None

        while True:
            op = None
            c = self.lex.herufi()
            if c == '+': op = '+'
            elif c == '-': op = '-'
            elif c == '<':
                if self.lex.herufi(1) == '=': op = '<='; self.lex.advance()
                elif self.lex.herufi(1) == '<': op = '<<'; self.lex.advance()
                else: op = '<'
            elif c == '>':
                if self.lex.herufi(1) == '=': op = '>='; self.lex.advance()
                elif self.lex.herufi(1) == '>': op = '>>'; self.lex.advance()
                else: op = '>'
            elif c == '=' and self.lex.herufi(1) == '=': op = '=='; self.lex.advance()
            elif c == '!' and self.lex.herufi(1) == '=': op = '!='; self.lex.advance()
            elif c == '&' and self.lex.herufi(1) == '&': op = '&&'; self.lex.advance()
            elif c == '|' and self.lex.herufi(1) == '|': op = '||'; self.lex.advance()
            else: break
            self.lex.advance()
            right = self.changanua_usemi_hesabu()
            node = Operesheni(op, node, right)
        return node

    def changanua_taarifa(self):
        """Changanua taarifa moja."""
        self.lex.ruka_nafasi_na_maelezo()

        c = self.lex.herufi()
        if c == '}' or c == '\0':
            return None

        # Tangazo la kigezo: Aina jina [= usemi];
        aina = self.changanua_aina()
        if aina and self.lex.herufi().isalpha():
            jina_tok = self.lex.soma_neno()
            if jina_tok and jina_tok.aina == TOK_JINA:
                init = None
                # Angalia [ukubwa]
                if self.lex.tarajia_herufi('['):
                    ukubwa_expr = self.changanua_usemi()
                    self.lex.tarajia_herufi(']')
                    init = Operesheni('safu_mpya', Nambari(ukubwa_expr), None)
                elif self.lex.tarajia_herufi('='):
                    init = self.changanua_usemi()
                self.lex.tarajia_herufi(';')
                return Tangazo(jina_tok.thamani, aina, init)

        # Ugawaji: jina = usemi;
        # Angalia kama ni kitambulisho kikifuatiwa na =
        if c.isalpha():
            pos_awali = self.lex.pos
            tok = self.lex.soma_neno()
            if tok and tok.aina == TOK_JINA and self.lex.angalia_herufi('='):
                self.lex.advance()
                expr = self.changanua_usemi()
                self.lex.tarajia_herufi(';')
                return Ugawaji(tok.thamani, expr)
            # Rudisha nafasi — si ugawaji
            self.lex.pos = pos_awali

        # kama
        if self.lex.tarajia_neno('kama'):
            self.lex.tarajia_herufi('(')
            hali = self.changanua_usemi()
            self.lex.tarajia_herufi(')')
            basi = self.changanua_bloku()
            sivyo = None
            if self.lex.tarajia_neno('sivyo'):
                if self.lex.angalia_herufi('{'):
                    sivyo = self.changanua_bloku()
                else:
                    sivyo = self.changanua_taarifa()
            return Kama(hali, basi, sivyo)

        # wakati
        if self.lex.tarajia_neno('wakati'):
            self.lex.tarajia_herufi('(')
            hali = self.changanua_usemi()
            self.lex.tarajia_herufi(')')
            mwili = self.changanua_bloku()
            return Wakati(hali, mwili)

        # rudisha
        if self.lex.tarajia_neno('rudisha'):
            if self.lex.angalia_herufi(';'):
                self.lex.advance()
                return Rudisha(None)
            expr = self.changanua_usemi()
            self.lex.tarajia_herufi(';')
            return Rudisha(expr)

        # vunja
        if self.lex.tarajia_neno('vunja'):
            self.lex.tarajia_herufi(';')
            return Vunja()

        # endelea
        if self.lex.tarajia_neno('endelea'):
            self.lex.tarajia_herufi(';')
            return Endelea()

        # Wito wa kazi kama taarifa: jina(hoja);
        if c.isalpha():
            tok = self.lex.soma_neno()
            if tok and self.lex.angalia_herufi('('):
                self.lex.advance()
                hoja = []
                if not self.lex.angalia_herufi(')'):
                    hoja.append(self.changanua_usemi())
                    while self.lex.tarajia_herufi(','):
                        hoja.append(self.changanua_usemi())
                self.lex.tarajia_herufi(')')
                self.lex.tarajia_herufi(';')
                return Wito(tok.thamani, hoja)

        # Usemi kama taarifa
        expr = self.changanua_usemi()
        self.lex.tarajia_herufi(';')
        return expr

    def changanua_bloku(self):
        """Changanua { taarifa* }"""
        if not self.lex.tarajia_herufi('{'):
            # Bloku ya taarifa moja
            return self.changanua_taarifa()

        taarifa = []
        while not self.lex.angalia_herufi('}') and self.lex.herufi() != '\0':
            stmt = self.changanua_taarifa()
            if stmt:
                taarifa.append(stmt)
            else:
                self.lex.advance()  # ruka herufi isiyotambulika
        self.lex.tarajia_herufi('}')
        return taarifa

    def changanua_kazi(self):
        """Changanua ufafanuzi wa kazi: Aina jina (vigezo) { mwili } au muundo"""
        self.lex.ruka_nafasi_na_maelezo()
        if self.lex.herufi() == '\0':
            return None

        # muundo Jina { sehemu* };
        if self.lex.tarajia_neno('muundo'):
            tok_jina = self.lex.soma_neno()
            if tok_jina is None:
                return None
            self.lex.tarajia_herufi('{')
            sehemu = []
            while not self.lex.angalia_herufi('}') and self.lex.herufi() != '\0':
                aina_sehemu = self.changanua_aina()
                jina_sehemu_tok = self.lex.soma_neno()
                if jina_sehemu_tok:
                    sehemu.append((jina_sehemu_tok.thamani, aina_sehemu or AINA_N32))
                self.lex.tarajia_herufi(';')
            self.lex.tarajia_herufi('}')
            self.lex.tarajia_herufi(';')
            return Muundo(tok_jina.thamani, sehemu)

        aina = self.changanua_aina()
        if aina is None:
            return None

        tok_jina = self.lex.soma_neno()
        if tok_jina is None:
            return None

        self.lex.tarajia_herufi('(')
        vigezo = []
        if not self.lex.angalia_herufi(')'):
            while True:
                pa = self.changanua_aina()
                if pa is None: break
                pn = self.lex.soma_neno()
                if pn is None: break
                vigezo.append((pn.thamani, pa))
                if not self.lex.tarajia_herufi(','):
                    break
        self.lex.tarajia_herufi(')')

        mwili = self.changanua_bloku()
        return Kazi(tok_jina.thamani, aina, vigezo, mwili)

    def changanua_programu(self):
        """Changanua programu nzima."""
        kazi = {}
        miundo = {}
        while self.lex.herufi() != '\0':
            k = self.changanua_kazi()
            if k is None:
                self.lex.advance()
                continue
            if isinstance(k, Muundo):
                miundo[k.jina] = k
            else:
                kazi[k.jina] = k
        return Programu(kazi, miundo)


# ============================================================
# 6. Kizalishe cha Msimbo (Code Generator)
# ============================================================

class Mazingira:
    def __init__(self):
        self.vigezo = {}     # jina -> (rbp_offset, Aina)
        self.rafu_juu = 0    # baiti zilizotengwa kwenye rafu
        self.vunja_lebo = [] # stack ya mizunguko
        self.endelea_lebo = []
        self.kazi_sasa = None
        self.aina_ya_kurudi = None
        self.kazi_zote = {}  # jina -> Kazi (kwa wito)

class Kizalishe:
    def __init__(self, programu):
        self.prog = programu
        self.x = X64()
        self.env = Mazingira()
        self.env.kazi_zote = programu.kazi
        # Jenga jedwali la ofseti za sehemu za miundo
        self.miundo_ofseti = {}  # (jina_la_muundo, jina_la_sehemu) -> (offset, ukubwa)
        for jina, muundo in programu.miundo.items():
            offset = 0
            for sehemu_jina, aina in muundo.sehemu:
                ukubwa = aina.ukubwa if aina else 4
                # Pangilia
                if ukubwa > 1 and offset % ukubwa:
                    offset += ukubwa - (offset % ukubwa)
                self.miundo_ofseti[(jina, sehemu_jina)] = (offset, ukubwa)
                offset += ukubwa

    def tengua_nafasi(self, jina, aina):
        """Tenga nafasi ya rafu kwa kigezo kipya."""
        if jina in self.env.vigezo:
            return self.env.vigezo[jina]
        ukubwa = aina.ukubwa
        # Pangilia kwa ukubwa wa neno
        align = min(ukubwa, 8)
        if self.env.rafu_juu % align:
            self.env.rafu_juu += align - (self.env.rafu_juu % align)
        self.env.rafu_juu += ukubwa
        self.env.vigezo[jina] = (self.env.rafu_juu, aina)
        return (self.env.rafu_juu, aina)

    def tafuta_kigezo(self, jina):
        """Tafuta kigezo kwenye wigo wa sasa."""
        if jina in self.env.vigezo:
            return self.env.vigezo[jina]
        return None

    def zalishe_usemi(self, node, reg='eax'):
        """Zalisha msimbo kwa usemi. Matokeo yako kwenye rejesta."""
        if node is None:
            self.x.xor_reg(reg)
            return

        if isinstance(node, Nambari):
            try:
                v = int(node.thamani)
            except (ValueError, TypeError):
                self.x.xor_reg(reg)
                return
            if reg == 'eax':
                self.x.mov_eax_imm(v)
            else:
                self.x.mov_ri('rax', v)

        elif isinstance(node, Kitambulisho):
            info = self.tafuta_kigezo(node.jina)
            if info:
                off, aina = info
                if aina.ukubwa <= 4:
                    self.x.mov_rbp_off('eax', off, 4)
                else:
                    self.x.mov_rbp_off('rax', off, 8)
            else:
                self.x.xor_reg(reg)

        elif isinstance(node, Tupu):
            self.x.xor_reg(reg)

        elif isinstance(node, Tenga):
            # malloc(ukubwa)
            self.zalishe_usemi(node.ukubwa, 'eax')
            # mov edi, eax; call malloc
            self.x.emit(0x89, 0xc7)  # mov edi, eax
            self._wito_nje('malloc')

        elif isinstance(node, Achilia):
            # free(ptr)
            self.zalishe_usemi(node.ptr, 'eax')
            self.x.emit(0x89, 0xc7)  # mov edi, eax
            self._wito_nje('free')

        elif isinstance(node, Sehemu):
            self.zalishe_sehemu(node)

        elif isinstance(node, Wito):
            self.zalishe_wito(node)

        elif isinstance(node, Operesheni):
            self.zalishe_operesheni(node)

        else:
            self.x.xor_reg(reg)

    def _wito_nje(self, jina):
        """Wito wa kazi ya nje — hutumia visaidizi vya syscall vilivyojengwa ndani."""
        self.x.call_rel32('_' + jina)  # _malloc, _free, n.k.

    def zalishe_sehemu(self, node):
        """Zalisha ufikiaji wa sehemu ya muundo (-> au .)."""
        # Pakia anwani ya msingi
        if node.ni_mshale:
            # ->: msingi ni kielekezi — pakia anwani
            self.zalishe_usemi(node.msingi, 'eax')
            # eax = anwani ya muundo (tayari ni kielekezi)
        else:
            # .: msingi ni muundo kwenye rafu — pata anwani yake
            if isinstance(node.msingi, Kitambulisho):
                info = self.tafuta_kigezo(node.msingi.jina)
                if info:
                    off, aina = info
                    self.x.emit(0x8d, 0x45, (256 - off) & 0xFF)  # lea eax, [rbp-off]
                else:
                    self.x.xor_reg('eax')
                    return
            else:
                self.zalishe_usemi(node.msingi, 'eax')

        # Tafuta ofseti ya sehemu
        # Tunahitaji kujua aina ya msingi ili kupata muundo sahihi.
        # Kwa sasa, tafuta kwenye miundo yote.
        offset = 0; ukubwa = 4
        found = False
        for (mj, sj), (off, sz) in self.miundo_ofseti.items():
            if sj == node.sehemu_jina:
                offset = off; ukubwa = sz; found = True
                break

        if found and offset > 0:
            self.x.emit(0x48, 0x83, 0xc0, offset)  # add rax, offset

        # Pakia thamani kutoka [eax]
        if ukubwa == 8:
            self.x.emit(0x48, 0x8b, 0x00)  # mov rax, [rax]
        elif ukubwa == 4:
            self.x.emit(0x8b, 0x00)  # mov eax, [eax]
        elif ukubwa == 1:
            self.x.emit(0x0f, 0xb6, 0x00)  # movzx eax, byte [eax]
        elif ukubwa == 2:
            self.x.emit(0x0f, 0xb7, 0x00)  # movzx eax, word [eax]
        # Matokeo yako kwenye eax

    def zalishe_wito(self, node):
        """Zalisha wito wa kazi."""
        kazi = self.env.kazi_zote.get(node.jina)
        num_params = len(kazi.vigezo) if kazi else len(node.hoja)

        # Pitisha hoja kwenye rejesta za 64-bit (sahihi kwa vielekezi)
        for i, arg in enumerate(node.hoja[:6]):
            self.zalishe_usemi(arg, 'eax')
            if i == 0:
                self.x.emit(0x48, 0x89, 0xc7)  # mov rdi, rax
            elif i == 1:
                self.x.emit(0x48, 0x89, 0xc6)  # mov rsi, rax
            elif i == 2:
                self.x.emit(0x48, 0x89, 0xc2)  # mov rdx, rax
            elif i == 3:
                self.x.emit(0x48, 0x89, 0xc1)  # mov rcx, rax
            elif i == 4:
                self.x.emit(0x49, 0x89, 0xc0)  # mov r8, rax
            elif i == 5:
                self.x.emit(0x49, 0x89, 0xc1)  # mov r9, rax

        self.x.call_rel32(node.jina)
        # Matokeo yako kwenye eax

    def zalishe_operesheni(self, node):
        """Zalisha msimbo kwa operesheni."""
        op = node.op
        size = 4  # default N32

        if op == '=':
            # Ugawaji: shughulikiwa kwenye taarifa
            self.zalishe_usemi(node.kulia, 'eax')
            return

        # Tathmini upande wa kulia kwanza, hifadhi
        self.zalishe_usemi(node.kulia, 'eax')
        self.x.push('rax')

        # Tathmini upande wa kushoto
        self.zalishe_usemi(node.kushoto, 'eax')

        # Vuta upande wa kulia hadi ecx
        self.x.pop('rcx')

        if op == '+':
            if size >= 8: self.x.add_rax_rcx()
            else: self.x.add_eax_ecx()
        elif op == '-':
            if size >= 8: self.x.sub_rax_rcx()
            else:
                # eax -= ecx
                self.x.sub_eax_ecx()
        elif op == '*':
            if size >= 8: self.x.imul_rax_rcx()
            else: self.x.imul_eax_ecx()
        elif op == '/':
            if size >= 8: self.x.cqo()
            else: self.x.cdq()
            self.x.idiv_ecx() if size < 8 else self.x.idiv_rcx()
        elif op == '==':
            if size >= 8: self.x.cmp_rax_rcx()
            else: self.x.cmp_eax_ecx()
            self.x.sete_al()
            self.x.movzx_eax_al()
        elif op == '!=':
            if size >= 8: self.x.cmp_rax_rcx()
            else: self.x.cmp_eax_ecx()
            self.x.setne_al()
            self.x.movzx_eax_al()
        elif op == '<':
            if size >= 8: self.x.cmp_rax_rcx()
            else: self.x.cmp_eax_ecx()
            self.x.setl_al()
            self.x.movzx_eax_al()
        elif op == '>':
            if size >= 8: self.x.cmp_rax_rcx()
            else: self.x.cmp_eax_ecx()
            self.x.setg_al()
            self.x.movzx_eax_al()
        elif op == '<=':
            if size >= 8: self.x.cmp_rax_rcx()
            else: self.x.cmp_eax_ecx()
            self.x.setle_al()
            self.x.movzx_eax_al()
        elif op == '>=':
            if size >= 8: self.x.cmp_rax_rcx()
            else: self.x.cmp_eax_ecx()
            self.x.setge_al()
            self.x.movzx_eax_al()
        elif op == 'taja':
            # base[idx] — pakia kulingana na aina ya msingi
            elem_size = 1
            if isinstance(node.kushoto, Kitambulisho):
                info = self.tafuta_kigezo(node.kushoto.jina)
                if info:
                    aina = info[1]
                    kina = aina.jina.count('_star') + aina.jina.count('*')
                    if kina >= 2:
                        elem_size = 8  # kielekezi-cha-kielekezi
                    elif kina == 1:
                        # Angalia aina ya msingi
                        jina_msingi = aina.jina.replace('_star', '').replace('*', '')
                        if jina_msingi == 'N8': elem_size = 1
                        elif jina_msingi == 'N32': elem_size = 4
                        elif jina_msingi == 'N64': elem_size = 8
                        else: elem_size = 4
            # eax = base, ecx = idx
            # Kwanza, zidisha idx kwa ukubwa wa elementi
            if elem_size > 1:
                if elem_size == 8:
                    self.x.emit(0x48, 0xc1, 0xe1, 0x03)  # shl rcx, 3
                elif elem_size == 4:
                    self.x.emit(0xc1, 0xe1, 0x02)        # shl ecx, 2
                elif elem_size == 2:
                    self.x.emit(0xc1, 0xe1, 0x01)        # shl ecx, 1
            self.x.add_eax_ecx()       # eax = base + idx*size
            if elem_size == 8:
                self.x.emit(0x48, 0x8b, 0x00)  # mov rax, [rax]
            elif elem_size == 4:
                self.x.emit(0x8b, 0x00)        # mov eax, [eax]
            elif elem_size == 2:
                self.x.emit(0x0f, 0xb7, 0x00)  # movzx eax, word [eax]
            else:
                self.x.emit(0x0f, 0xb6, 0x00)  # movzx eax, byte [eax]
        elif op == '&&':
            # Tathmini ya fupi-hali
            self.x.test_eax_eax()
            skip_label = f'_and_skip_{node.__hash__() & 0xFFFF}'
            end_label = f'_and_end_{node.__hash__() & 0xFFFF}'
            self.x.je_rel32(skip_label)
            self.x.pop('rcx')
            self.x.test_eax_eax()  # tayari iko eax kutoka kushoto
            self.x.je_rel32(skip_label)
            self.x.mov_eax_imm(1)
            self.x.jmp_rel32(end_label)
            self.x.label(skip_label)
            self.x.xor_reg('eax')
            self.x.label(end_label)

    def zalishe_taarifa(self, node):
        """Zalisha msimbo kwa taarifa."""
        if node is None:
            return

        if isinstance(node, list):
            for stmt in node:
                self.zalishe_taarifa(stmt)
            return

        if isinstance(node, Tangazo):
            info = self.tengua_nafasi(node.jina, node.aina)
            off, aina = info
            if node.kianzio:
                self.zalishe_usemi(node.kianzio, 'eax')
                self.x.mov_off_rbp(off, 'eax', aina.ukubwa)

        elif isinstance(node, Ugawaji):
            info = self.tafuta_kigezo(node.jina)
            if info:
                off, aina = info
                self.zalishe_usemi(node.usemi, 'eax')
                self.x.mov_off_rbp(off, 'eax', aina.ukubwa)

        elif isinstance(node, Rudisha):
            if node.usemi:
                self.zalishe_usemi(node.usemi, 'eax')
                self.x.emit(0x89, 0xc7)  # mov edi, eax
            self.x.leave()
            self.x.ret()

        elif isinstance(node, Kama):
            else_label = f'_else_{id(node)}'
            end_label = f'_endif_{id(node)}'

            self.zalishe_usemi(node.hali)
            self.x.test_eax_eax()
            if node.sivyo:
                self.x.je_rel32(else_label)
            else:
                self.x.je_rel32(end_label)

            self.zalishe_taarifa(node.basi)
            if node.sivyo:
                self.x.jmp_rel32(end_label)
                self.x.label(else_label)
                self.zalishe_taarifa(node.sivyo)
            self.x.label(end_label)

        elif isinstance(node, Wakati):
            start_label = f'_while_{id(node)}'
            end_label = f'_wend_{id(node)}'

            self.env.vunja_lebo.append(end_label)
            self.env.endelea_lebo.append(start_label)

            self.x.label(start_label)
            self.zalishe_usemi(node.hali)
            self.x.test_eax_eax()
            self.x.je_rel32(end_label)
            self.zalishe_taarifa(node.mwili)
            self.x.jmp_rel32(start_label)
            self.x.label(end_label)

            self.env.vunja_lebo.pop()
            self.env.endelea_lebo.pop()

        elif isinstance(node, Vunja):
            if self.env.vunja_lebo:
                self.x.jmp_rel32(self.env.vunja_lebo[-1])

        elif isinstance(node, Endelea):
            if self.env.endelea_lebo:
                self.x.jmp_rel32(self.env.endelea_lebo[-1])

        elif isinstance(node, Wito):
            self.zalishe_wito(node)

        elif isinstance(node, Operesheni):
            self.zalishe_usemi(node)

    def _tengua_vigezo_awali(self, mwili):
        """Tembea AST na utenge nafasi ya rafu kwa vigezo vyote vya ndani."""
        if isinstance(mwili, list):
            for s in mwili:
                self._tengua_vigezo_awali(s)
        elif isinstance(mwili, Tangazo):
            self.tengua_nafasi(mwili.jina, mwili.aina)
        elif isinstance(mwili, Kama):
            self._tengua_vigezo_awali(mwili.basi)
            self._tengua_vigezo_awali(mwili.sivyo)
        elif isinstance(mwili, Wakati):
            self._tengua_vigezo_awali(mwili.mwili)

    def zalishe_kazi(self, kazi):
        """Zalisha msimbo kwa kazi nzima."""
        self.env.vigezo = {}
        self.env.rafu_juu = 0
        self.env.vunja_lebo = []
        self.env.endelea_lebo = []
        self.env.kazi_sasa = kazi

        # Sajili vigezo vya kazi
        for jina, aina in kazi.vigezo:
            self.tengua_nafasi(jina, aina)

        # Tengua vigezo vyote vya ndani KABLA ya kutoa mwili
        self._tengua_vigezo_awali(kazi.mwili)

        self.x.label(kazi.jina)

        # Utangulizi wenye nafasi kamili ya rafu
        self.x.push_rbp()
        self.x.mov_rbp_rsp()
        self.x.sub_rsp(self.env.rafu_juu)

        # Hifadhi vigezo vya kazi kutoka kwenye rejesta
        for i, (jina, aina) in enumerate(kazi.vigezo):
            off, _ = self.env.vigezo[jina]
            if i == 0:
                self.x.mov_off_rbp(off, 'edi', aina.ukubwa)
            elif i == 1:
                self.x.mov_off_rbp(off, 'esi', aina.ukubwa)
            elif i == 2:
                self.x.mov_off_rbp(off, 'edx', aina.ukubwa)

        # Toa mwili
        self.zalishe_taarifa(kazi.mwili)

        # Angalia kama mwili unaishia na rudisha
        ina_rudisha = False
        if isinstance(kazi.mwili, list) and kazi.mwili:
            mwisho = kazi.mwili[-1]
            if isinstance(mwisho, Rudisha):
                ina_rudisha = True

        if not ina_rudisha:
            self.x.leave()
            self.x.ret()

    def zalishe_kiingilio(self):
        """Zalisha _start inayowita main() kwa kupitisha argc/argv."""
        self.x.label('_start')
        # Linux x86-64: [rsp] = argc, [rsp+8] = argv
        self.x.emit(0x48, 0x31, 0xed)  # xor rbp, rbp (alama ya mwisho wa rafu)
        self.x.pop('rdi')               # pop rdi = argc
        self.x.emit(0x48, 0x89, 0xe6)  # mov rsi, rsp (argv = rsp)
        # Pangilia rafu kwa 16 kabla ya wito
        self.x.emit(0x48, 0x83, 0xe4, 0xf0)  # and rsp, -16
        self.x.call_rel32('main')
        # eax ina thamani ya kurudi
        self.x.emit(0x89, 0xc7)  # mov edi, eax
        self.x.mov_eax_imm(60)   # sys_exit
        self.x.syscall()

    def _zalishe_visaidizi_vya_syscall(self):
        """Zalisha utekelezaji wa kazi za nje kwa kutumia syscall."""
        # _malloc(size=edi) — tumia mmap
        self.x.label('_malloc')
        # mmap(0, size, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0)
        self.x.emit(0x48, 0x89, 0xfe)           # mov rsi, rdi (length = arg1)
        self.x.emit(0x48, 0x31, 0xff)           # xor rdi, rdi (addr = 0)
        self.x.emit(0xba, 0x03, 0x00, 0x00, 0x00)  # mov edx, 3 (PROT_READ|PROT_WRITE)
        self.x.emit(0x41, 0xb9, 0x22, 0x00, 0x00, 0x00)  # mov r9d, 0x22 (MAP_PRIVATE|MAP_ANONYMOUS)
        self.x.emit(0x41, 0xb8, 0xff, 0xff, 0xff, 0xff)  # mov r8d, -1 (fd = -1)
        self.x.emit(0x48, 0x31, 0xc9)           # xor rcx, rcx (offset = 0)
        self.x.emit(0xb8, 0x09, 0x00, 0x00, 0x00)  # mov eax, 9 (mmap)
        self.x.emit(0x0f, 0x05)                 # syscall
        self.x.ret()

        # _free(ptr=edi) — no-op kwa sasa
        self.x.label('_free')
        self.x.ret()

        # _printf — andika kwenye stdout
        # Kwa urahisi: tunachukulia hoja imeshughulikiwa tayari
        self.x.label('_printf')
        self.x.ret()

        # _fopen(njia=rdi, mode=rsi) -> fd
        self.x.label('_fopen')
        # rdi = njia (tayari), rsi = mode string (tunapuuza, tunatumia O_RDONLY)
        self.x.emit(0x48, 0x31, 0xf6)           # xor rsi, rsi (flags = O_RDONLY)
        self.x.emit(0x48, 0x31, 0xd2)           # xor rdx, rdx (mode = 0)
        self.x.emit(0xb8, 0x02, 0x00, 0x00, 0x00)  # mov eax, 2 (open)
        self.x.emit(0x0f, 0x05)                 # syscall
        self.x.ret()                            # rax = fd

        # _fread(buf=rdi, size=rsi, count=rdx, stream=rcx) -> bytes read
        self.x.label('_fread')
        # Hoja za Swa: buf(rdi), size=1(rsi), count(rdx), fd(rcx)
        # Hoja za sys_read: fd(rdi), buf(rsi), count(rdx)
        # Tunahitaji: rdi=fd, rsi=buf, rdx=count
        self.x.push('rbx')                      # hifadhi rbx
        self.x.emit(0x48, 0x89, 0xcb)           # mov rbx, rcx (rbx = fd)
        self.x.emit(0x48, 0x89, 0xfe)           # mov rsi, rdi (rsi = buf)
        self.x.emit(0x48, 0x89, 0xdf)           # mov rdi, rbx (rdi = fd)
        # rdx tayari ina count (sawa kwa sys_read)
        self.x.emit(0xb8, 0x00, 0x00, 0x00, 0x00)  # mov eax, 0 (read)
        self.x.emit(0x0f, 0x05)                 # syscall
        self.x.pop('rbx')                       # rejesha rbx
        self.x.ret()                            # rax = bytes read

        # _fclose(fd=rdi) -> 0
        self.x.label('_fclose')
        self.x.emit(0xb8, 0x03, 0x00, 0x00, 0x00)  # mov eax, 3 (close)
        self.x.emit(0x0f, 0x05)                 # syscall
        self.x.ret()

    def zalishe_programu(self):
        """Zalisha programu nzima."""
        # _start LAZIMA iwe mwanzo kabisa — ndio inayoitwa na kernel
        if 'main' in self.prog.kazi:
            self.zalishe_kiingilio()
        else:
            jina_ya_kwanza = list(self.prog.kazi.keys())[0]
            self.x.label('_start')
            self.x.call_rel32(jina_ya_kwanza)
            self.x.emit(0x89, 0xc7)
            self.x.mov_eax_imm(60)
            self.x.syscall()

        # Kisha, zalisha kazi zote za mtumiaji
        for jina, kazi in self.prog.kazi.items():
            self.zalishe_kazi(kazi)

        # Mwisho, zalisha visaidizi vya syscall
        self._zalishe_visaidizi_vya_syscall()

        self.x.resolve_fixups()
        return bytes(self.x.b)


# ============================================================
# 7. Kiingilio Kikuu
# ============================================================

def kusanya(chanzo, aina_ya_pato='exec'):
    """Kusanya chanzo cha Swa hadi binary ya ELF.
    aina_ya_pato: 'exec' kwa executable, 'obj' kwa .o (relocatable)."""
    try:
        mp = Mchanganuzi(chanzo)
        prog = mp.changanua_programu()

        if not prog.kazi:
            sys.stderr.write("Hitilafu: hakuna kazi iliyopatikana\n")
            return None

        gen = Kizalishe(prog)
        msimbo = gen.zalishe_programu()

        if aina_ya_pato == 'obj':
            return make_elf_obj(msimbo)
        return make_elf(msimbo)

    except SyntaxError as e:
        sys.stderr.write(f"{e}\n")
        return None


def main():
    aina_ya_pato = 'exec'
    faili_za_chanzo = []

    args = sys.argv[1:]
    i = 0
    while i < len(args):
        if args[i] == '-c':
            aina_ya_pato = 'obj'
        elif args[i] == '-o':
            i += 1
            njia_ya_pato = args[i]
        else:
            faili_za_chanzo.append(args[i])
        i += 1

    if not faili_za_chanzo:
        chanzo = sys.stdin.read()
    else:
        chanzo = ''
        for f in faili_za_chanzo:
            with open(f) as fh:
                chanzo += fh.read() + '\n'

    binary = kusanya(chanzo, aina_ya_pato)
    if binary is None:
        sys.exit(1)

    # Amua jina la pato
    njia_ya_pato = None
    for a in sys.argv[1:]:
        if a == '-o':
            idx = sys.argv.index('-o')
            if idx + 1 < len(sys.argv):
                njia_ya_pato = sys.argv[idx + 1]
            break
    if njia_ya_pato is None and faili_za_chanzo:
        njia_ya_pato = faili_za_chanzo[0].replace('.swa', '.o' if aina_ya_pato == 'obj' else '')

    if njia_ya_pato:
        with open(njia_ya_pato, 'wb') as f:
            f.write(binary)
        if aina_ya_pato == 'exec':
            os.chmod(njia_ya_pato, 0o755)
        print(f"  -> {njia_ya_pato} ({len(binary)} baiti)")
    else:
        sys.stdout.buffer.write(binary)


if __name__ == '__main__':
    main()
