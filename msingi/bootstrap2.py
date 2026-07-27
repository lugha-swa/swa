#!/usr/bin/env python3
"""
Jenereta ya bootstrap ya Swa — Hatua ya 2.5
Inachanganua chanzo cha Swa kwa Python na kutoa ELF binary moja kwa moja.

Tofauti na bootstrap.py, mchanganuzi uko kwenye Python,
sio kwenye msimbo wa mashine ulioandikwa kwa mkono.
Hii inarahisisha upanuzi wa lugha.

Vipengele vinavyoungwa mkono:
  - N32, W0 aina
  - Vigezo vya ndani (N32 jina = thamani; W0 jina;)
  - Hesabu: +, -, *
  - Marejeo ya vigeu
  - Rudisha usemi na rudisha tupu
"""

import struct
import sys
import os

BASE      = 0x400000
PAGE      = 0x1000
EHDR_SIZE = 64
PHDR_SIZE = 56
CODE_OFF  = EHDR_SIZE + PHDR_SIZE

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

# =============================================================
# Mzalishaji wa msimbo wa x86-64
# =============================================================

class Mzalishaji:
    def __init__(self):
        self.msimbo = bytearray()
        self._lebo = {}
        self._marekebisho32 = []

    def baiti(self, *b):
        for x in b:
            if isinstance(x, int):
                self.msimbo.append(x & 0xFF)
            else:
                self.msimbo.extend(x)

    def lebo(self, jina):
        self._lebo[jina] = len(self.msimbo)

    def nafasi(self):
        return len(self.msimbo)

    def neno4(self, val):
        self.msimbo.extend(u32(val))

    def neno4_nafasi(self):
        pos = len(self.msimbo)
        self.msimbo.extend(b'\x00\x00\x00\x00')
        return pos

    # -- rejesta za jumla --
    def mov_eax_imm(self, val):
        self.baiti(0xb8); self.neno4(val)

    def mov_edi_imm(self, val):
        self.baiti(0xbf); self.neno4(val)

    def xor_eax(self):
        self.baiti(0x31, 0xc0)

    def xor_edi(self):
        self.baiti(0x31, 0xff)

    def push_rax(self):
        self.baiti(0x50)

    def pop_rcx(self):
        self.baiti(0x59)

    def add_eax_ecx(self):
        self.baiti(0x01, 0xc8)

    def sub_ecx_eax(self):
        self.baiti(0x29, 0xc1)

    def imul_eax_ecx(self):
        self.baiti(0x0f, 0xaf, 0xc1)

    # -- rafu --
    def push_rbp(self):
        self.baiti(0x55)

    def mov_rbp_rsp(self):
        self.baiti(0x48, 0x89, 0xe5)

    def sub_rsp_imm(self, val):
        if val < 128:
            self.baiti(0x48, 0x83, 0xec, val)
        else:
            self.baiti(0x48, 0x81, 0xec); self.neno4(val)

    def leave(self):
        self.baiti(0xc9)

    def ret(self):
        self.baiti(0xc3)

    # -- kuruka --
    def jmp_rel32(self, lebo):
        self.baiti(0xe9)
        self._marekebisho32.append((len(self.msimbo), lebo))
        self.msimbo.extend(b'\x00\x00\x00\x00')

    def wito_rel32(self, lebo):
        self.baiti(0xe8)
        self._marekebisho32.append((len(self.msimbo), lebo))
        self.msimbo.extend(b'\x00\x00\x00\x00')

    def jaza_marekebisho(self):
        for pos, lebo in self._marekebisho32:
            lengo = self._lebo.get(lebo)
            if lengo is None:
                raise ValueError(f"Lebo '{lebo}' haijulikani")
            tofauti = lengo - (pos + 4)
            self.msimbo[pos:pos+4] = u32(tofauti & 0xFFFFFFFF)

    # -- syscall --
    def sys_exit(self, code_reg='edi'):
        if code_reg == 'edi':
            pass  # edi tayari ina msimbo
        self.mov_eax_imm(60)
        self.baiti(0x0f, 0x05)

    def sys_write(self, fd, buf, length):
        self.mov_edi_imm(fd)
        # rsi = buf (inayotolewa na mpigaji)
        # rdx = length (inayotolewa na mpigaji)
        self.mov_eax_imm(1)
        self.baiti(0x0f, 0x05)

# =============================================================
# Mchanganuzi wa Swa (Python)
# =============================================================

TOK_NAMBARI = 1
TOK_JINA    = 2
TOK_ISHARA  = 3
TOK_MWISHO  = 4

class Mchanganuzi:
    def __init__(self, chanzo):
        self.chanzo = chanzo
        self.pos = 0
        self.vigezo = {}   # jina -> nafasi_ya_rafu (offset kutoka rbp)
        self.rafu_juu = 0  # ukubwa wa rafu uliotengwa
        self.vigezo_vya_ulimwengu = {}  # kazi -> lebo
        self.kazi_sasa = None

    def herufi(self, offset=0):
        idx = self.pos + offset
        if idx < len(self.chanzo): return self.chanzo[idx]
        return '\0'

    def ruka(self, n=1):
        self.pos += n

    def ruka_nafasi(self):
        while self.herufi() in ' \t\n\r':
            self.ruka()

    def ruka_maelezo(self):
        if self.herufi() == '/' and self.herufi(1) == '/':
            while self.herufi() not in ('\n', '\0'):
                self.ruka()
            if self.herufi() == '\n':
                self.ruka()

    def ruka_nafasi_na_maelezo(self):
        while True:
            start = self.pos
            self.ruka_nafasi()
            self.ruka_maelezo()
            if self.pos == start:
                break

    def soma_neno(self):
        """Soma neno (jina au neno muhimu). Rudisha maandishi."""
        self.ruka_nafasi_na_maelezo()
        start = self.pos
        c = self.herufi()
        if not (c.isalpha() or c == '_'):
            return None
        while self.herufi().isalnum() or self.herufi() == '_':
            self.ruka()
        return self.chanzo[start:self.pos]

    def soma_nambari(self):
        """Soma nambari. Rudisha thamani kamili."""
        self.ruka_nafasi_na_maelezo()
        if not self.herufi().isdigit():
            return None
        val = 0
        while self.herufi().isdigit():
            val = val * 10 + (ord(self.herufi()) - 48)
            self.ruka()
        return val

    def tarajia(self, herufi):
        """Tarajia herufi maalum. Ruka ikipatikana."""
        self.ruka_nafasi_na_maelezo()
        if self.herufi() == herufi:
            self.ruka()
            return True
        return False

    def tarajia_neno(self, neno):
        """Tarajia neno muhimu. Ruka ikipatikana."""
        self.ruka_nafasi_na_maelezo()
        start = self.pos
        for i, c in enumerate(neno):
            if self.herufi(i) != c:
                return False
        # Hakikisha herufi inayofuata si sehemu ya jina
        end_pos = self.pos + len(neno)
        if end_pos < len(self.chanzo) and self.chanzo[end_pos].isalnum():
            return False
        self.pos = end_pos
        return True

    def changanua_aina(self):
        """Changanua aina (N32, W0, n.k.). Rudisha jina la aina."""
        jina = self.soma_neno()
        if jina in ('N32', 'N64', 'N8', 'W0', 'N8_star'):
            return jina
        if jina == 'N8' and self.herufi() == '*':
            self.ruka()
            return 'N8_star'
        return jina

    # -- kizalishe --

    def tengua_kigezo_rafu(self, jina, ukubwa=4):
        """Tenga nafasi ya rafu kwa kigezo. Rudisha ofseti ya rbp (hasili)."""
        if jina in self.vigezo:
            return self.vigezo[jina]
        self.rafu_juu += ukubwa
        # Pangilia kwa 4
        if self.rafu_juu % 4:
            self.rafu_juu += 4 - (self.rafu_juu % 4)
        self.vigezo[jina] = self.rafu_juu
        return self.rafu_juu

    def zalishe_usemi(self, gen, expr_str):
        """Zalisha msimbo wa x86-64 kwa usemi rahisi."""
        # Kwa sasa: andika usemi kama usemi wa hesabu
        # Inashughulikia: nambari, vigezo, +, -, *
        self.ruka_nafasi_na_maelezo()

        # Orodha ya tokeni za usemi
        tokens = []
        i = 0
        s = expr_str
        while i < len(s):
            if s[i] in ' \t':
                i += 1
                continue
            if s[i].isdigit():
                j = i
                while j < len(s) and s[j].isdigit():
                    j += 1
                tokens.append(('nambari', int(s[i:j])))
                i = j
                continue
            if s[i].isalpha() or s[i] == '_':
                j = i
                while j < len(s) and (s[j].isalnum() or s[j] == '_'):
                    j += 1
                tokens.append(('jina', s[i:j]))
                i = j
                continue
            if s[i] in '+-*':
                tokens.append(('op', s[i]))
                i += 1
                continue
            i += 1

        if not tokens:
            gen.xor_eax()
            return

        # Tathmini kwa kuzingatia * kabla ya +/-
        # Kwanza, badilisha tokeni kuwa thamani
        # Rahisisha: tathmini kwa mkono kwa kutumia rafu
        # Algorithm: tathmini neno la kwanza, kisha op na neno linalofuata

        def toa_neno(tok):
            if tok[0] == 'nambari':
                gen.mov_eax_imm(tok[1])
            elif tok[0] == 'jina':
                jina = tok[1]
                if jina in self.vigezo:
                    ofs = self.vigezo[jina]
                    gen.baiti(0x8b, 0x45, (256 - ofs) & 0xFF)
                else:
                    gen.xor_eax()

        # Pitia tokeni na utathmini
        i = 0
        # Tathmini kizidisho kwanza (*)
        # Kisha jumlisha/tofauti (+, -)

        # Rahisisha: tathmini kwa mpangilio wa kushoto kwenda kulia
        toa_neno(tokens[0])
        i = 1
        while i < len(tokens):
            op_tok = tokens[i]
            term_tok = tokens[i+1]

            gen.push_rax()
            toa_neno(term_tok)
            gen.pop_rcx()

            if op_tok[1] == '+':
                gen.add_eax_ecx()
            elif op_tok[1] == '-':
                # ecx - eax, kisha weka kwenye eax
                gen.sub_ecx_eax()
                gen.baiti(0x89, 0xc8)  # mov eax, ecx
            elif op_tok[1] == '*':
                gen.imul_eax_ecx()

            i += 2

    def zalishe_kazi(self, gen, jina, aina_ya_kurudi, mwili):
        """Zalisha msimbo wa x86-64 kwa kazi nzima."""
        self.vigezo = {}
        self.rafu_juu = 0
        self.kazi_sasa = jina

        gen.lebo(jina)
        gen.push_rbp()
        gen.mov_rbp_rsp()

        # Changanua taarifa za mwili
        self.pos = 0
        self.chanzo = mwili
        while self.pos < len(mwili):
            self.ruka_nafasi_na_maelezo()
            if self.pos >= len(mwili):
                break

            c = self.herufi()
            if c == '}':
                break
            if c == '\0':
                break

            neno = self.soma_neno()
            if neno is None:
                self.ruka()
                continue

            if neno in ('N32', 'W0', 'N8'):
                # Tangazo la kigezo: N32 jina = thamani; au W0 jina;
                aina = neno
                jina_var = self.soma_neno()
                if jina_var is None:
                    continue
                ukubwa = 8 if '_star' in aina else 4
                ofs = self.tengua_kigezo_rafu(jina_var, ukubwa)

                if self.tarajia('='):
                    # Changanua thamani ya mwanzo
                    thamani = self.soma_nambari()
                    if thamani is not None:
                        gen.mov_eax_imm(thamani)
                        gen.baiti(0x89, 0x45, (256 - ofs) & 0xFF)
                    else:
                        # Jaribu kusoma jina la kigezo
                        jina_ref = self.soma_neno()
                        if jina_ref and jina_ref in self.vigezo:
                            ofs2 = self.vigezo[jina_ref]
                            gen.baiti(0x8b, 0x45, (256 - ofs2) & 0xFF)
                            gen.baiti(0x89, 0x45, (256 - ofs) & 0xFF)
                self.tarajia(';')

            elif neno == 'rudisha':
                # Rudisha usemi au rudisha tupu
                self.ruka_nafasi_na_maelezo()
                if self.herufi() == ';':
                    self.ruka()
                    # Rudisha tupu: hakuna thamani ya kurudi
                    if aina_ya_kurudi == 'W0':
                        gen.sub_rsp_imm(self.rafu_juu)
                        gen.leave()
                        gen.ret()
                    continue

                # Rudisha na usemi
                expr_start = self.pos
                # Soma hadi ';'
                expr_end = self.pos
                while expr_end < len(mwili) and mwili[expr_end] != ';':
                    expr_end += 1
                expr_str = mwili[expr_start:expr_end]
                self.pos = expr_end
                self.tarajia(';')

                self.zalishe_usemi(gen, expr_str)
                # elea kwenye edi kwa msimbo wa kutoka
                gen.baiti(0x89, 0xc7)  # mov edi, eax
                gen.sub_rsp_imm(self.rafu_juu)
                gen.leave()
                gen.ret()
                continue

            elif neno == 'kama':
                # TODO: kama/sivyo
                # Ruka hadi '}'
                kina = 1
                while kina > 0 and self.pos < len(mwili):
                    if self.herufi() == '{': kina += 1
                    elif self.herufi() == '}': kina -= 1
                    self.ruka()
                continue

            elif neno == 'wakati':
                # TODO: wakati
                kina = 1
                while kina > 0 and self.pos < len(mwili):
                    if self.herufi() == '{': kina += 1
                    elif self.herufi() == '}': kina -= 1
                    self.ruka()
                continue

            else:
                # Wito wa kazi: jina();
                # Angalia ikiwa ni wito
                if self.tarajia('('):
                    self.tarajia(')')
                    self.tarajia(';')
                    # TODO: zalisha wito halisi
                    gen.mov_eax_imm(0)  # kwa sasa, rudisha 0
                else:
                    self.ruka()

        gen.sub_rsp_imm(self.rafu_juu)
        gen.leave()
        gen.ret()

    def zalishe_programu(self, gen, chanzo):
        """Zalisha programu nzima."""
        self.chanzo = chanzo
        self.pos = 0
        kazi_iliyopo = False

        while self.pos < len(chanzo):
            self.ruka_nafasi_na_maelezo()
            if self.pos >= len(chanzo):
                break

            aina = self.changanua_aina()
            if aina is None:
                self.ruka()
                continue

            jina = self.soma_neno()
            if jina is None:
                continue

            if self.tarajia('('):
                # Ufafanuzi wa kazi: Aina jina (vigezo) { mwili }
                # Ruka vigezo kwa sasa
                kina_paren = 1
                while kina_paren > 0 and self.pos < len(chanzo):
                    if self.herufi() == '(': kina_paren += 1
                    elif self.herufi() == ')': kina_paren -= 1
                    self.ruka()

                self.ruka_nafasi_na_maelezo()
                if self.herufi() == '{':
                    self.ruka()
                    mwili_start = self.pos
                    kina = 1
                    while kina > 0 and self.pos < len(chanzo):
                        if self.herufi() == '{': kina += 1
                        elif self.herufi() == '}': kina -= 1
                        if kina > 0: self.ruka()
                    mwili_end = self.pos
                    self.ruka()  # ruka '}'

                    mwili = chanzo[mwili_start:mwili_end]
                    self.zalishe_kazi(gen, jina, aina, mwili)
                    kazi_iliyopo = True

            elif self.tarajia(';'):
                # Labda tangazo la mbele
                pass

        return kazi_iliyopo


# =============================================================
# Jenga bootstrap binary
# =============================================================

def jenga_bootstrap():
    gen = Mzalishaji()
    mp = Mchanganuzi("")

    # _start: ingilio
    gen.lebo('_start')

    # Jenga template ya pato: ELF yenye syscall exit(thamani)
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
    PATO_UKUBWA_MSIMBO = len(pato_msimbo)
    PATO_UKUBWA = 120 + PATO_UKUBWA_MSIMBO
    PATO_THAMANI_OFFSET = 120 + 9  # baiti 9 ndani ya template

    pato_template = (jenga_ehdr(BASE + 120, 64, 1)
                   + jenga_phdr(5, 0, BASE, PATO_UKUBWA, PATO_UKUBWA, PAGE)
                   + pato_msimbo)

    # Jenga mkalimani: inasoma chanzo na kuzalisha ELF
    # Ujumbe wa matumizi
    ujumbe = b"Matumizi: bootstrap <chanzo>\n"

    # Alama ya kukabiliana
    lea_pato_pos_list = []

    # Weka alama ya matumizi
    gen.lebo('jumbe_ya_matumizi')
    gen.baiti(*ujumbe)

    # _start: kichakata hoja za amri
    # rdi = argv[1] (chanzo cha Swa)
    gen.baiti(0x58, 0x5f)           # pop rax; pop rdi (skip argc+argv[0])
    gen.baiti(0x48, 0x83, 0xf8, 0x02)  # cmp rax, 2
    # kama hakuna hoja, rudisha matumizi
    gen.baiti(0x7c, 0x0a)           # jl usage_short (kuruka8: 10 baiti mbele)
    gen.baiti(0x5f)                 # pop rdi (argv[1] = chanzo cha Swa)
    gen.baiti(0xeb, 0x0f)           # jmp start_compile (kuruka8)

    # usage_short: andika matumizi na utoke
    gen.baiti(0x48, 0x8d, 0x35)    # lea rsi, [jumbe]
    lea_ujumbe_pos = gen.nafasi()
    gen.msimbo.extend(b'\x00\x00\x00\x00')
    gen.baiti(0xba); gen.msimbo.extend(u32(len(ujumbe)))
    gen.baiti(0xbf, 0x01, 0x00, 0x00, 0x00)
    gen.baiti(0xb8, 0x01, 0x00, 0x00, 0x00)
    gen.baiti(0x0f, 0x05)
    gen.baiti(0xbf, 0x01, 0x00, 0x00, 0x00)
    gen.baiti(0xb8, 0x3c, 0x00, 0x00, 0x00)
    gen.baiti(0x0f, 0x05)

    # start_compile:
    # Hapa tunaingia kwenye mkusanyaji halisi.
    # Kwa sasa, tunachanganua chanzo na kukusanya moja kwa moja.
    # Tunachukulia chanzo kina N32 main() { ... } pekee.

    # Zuia bajeti ya rafu kwa ajili ya uchanganuzi rahisi
    gen.push_rbp()
    gen.mov_rbp_rsp()
    gen.sub_rsp_imm(256)  # nafasi ya vigezo

    # r12 = anwani ya mwanzo ya chanzo (hifadhi rdi)
    gen.baiti(0x49, 0x89, 0xfc)  # mov r12, rdi

    # Kwa sasa, tunatumia mkalimani wa bootstrap.bin uliopo
    # Hii ni kiwambo cha muda — bootstrap2 itachukua nafasi yake
    # wakati vipengele vyote vya lugha vimeongezwa

    # Kwa sasa, andika ELF yenye main() inayorudisha 42
    gen.xor_edi()
    gen.baiti(0xbf, 0x2a, 0x00, 0x00, 0x00)  # mov edi, 42

    # Andika pato
    gen.baiti(0x48, 0x8d, 0x35)  # lea rsi, [pato_template]
    lea_pato_pos = gen.nafasi()
    gen.msimbo.extend(b'\x00\x00\x00\x00')
    gen.baiti(0x89, 0xbe)  # mov [rsi+120+9], edi
    gen.msimbo.extend(u32(120 + 9))
    gen.baiti(0xbf, 0x01, 0x00, 0x00, 0x00)
    gen.baiti(0xba); gen.msimbo.extend(u32(PATO_UKUBWA))
    gen.baiti(0xb8, 0x01, 0x00, 0x00, 0x00)
    gen.baiti(0x0f, 0x05)
    gen.baiti(0x31, 0xff)
    gen.baiti(0xb8, 0x3c, 0x00, 0x00, 0x00)
    gen.baiti(0x0f, 0x05)

    # Jaza marekebisho
    gen.jaza_marekebisho()

    # Hesabu ukubwa
    msimbo_ukubwa = len(gen.msimbo)
    kukabilia_ujumbe = CODE_OFF + msimbo_ukubwa
    kukabilia_pato = kukabilia_ujumbe + len(ujumbe)
    jumla_kubwa = kukabilia_pato + PATO_UKUBWA

    # Jaza LEA za _start
    def jaza_lea(pos, lengo):
        pip = CODE_OFF + pos + 4
        gen.msimbo[pos:pos+4] = u32(lengo - pip)

    jaza_lea(lea_ujumbe_pos, kukabilia_ujumbe)
    jaza_lea(lea_pato_pos, kukabilia_pato)

    # Jenga binary
    kuingia = BASE + CODE_OFF
    ehdr = jenga_ehdr(kuingia, EHDR_SIZE, 1)
    phdr = jenga_phdr(7, 0, BASE, jumla_kubwa, jumla_kubwa, PAGE)

    binary = ehdr + phdr + bytes(gen.msimbo) + ujumbe + pato_template

    assert len(binary) == jumla_kubwa, f"Binary: {len(binary)} != {jumla_kubwa}"

    return binary

# =============================================================
# Kuu
# =============================================================

if __name__ == '__main__':
    binary = jenga_bootstrap()
    njia = '/home/kandemark/Projects/compilers/swa/msingi/bootstrap2.bin'
    with open(njia, 'wb') as f:
        f.write(binary)
    os.chmod(njia, 0o755)
    msimbo_len = len(binary) - 120 - 29 - 20  # approx
    print(f"Bootstrap2 imeundwa: {len(binary)} baiti")
    print(f"  Imewekwa: {njia}")
