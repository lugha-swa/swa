; =============================================================================
; mbegu.s — Mkusanyaji Mbegu wa Swa
;
; Mkusanyaji wa Swa ulioandikwa kwa lugha ya assembly ya x86-64 (NASM).
; Husoma chanzo cha Swa na kutoa ELF .o kwa stdout.
;
; Hii inafunga pengo la bootstrap: assembly → Swahili pekee.
;
; Kujenga:
;   nasm -f elf64 msingi/mbegu.s -o msingi/mbegu.o
;   ld msingi/mbegu.o -o msingi/mbegu.bin
;
; Kutumia:
;   ./msingi/mbegu.bin < chanzo.swa > pato.o
;   au: ./msingi/mbegu.bin chanzo.swa > pato.o
; =============================================================================

        bits 64
        default rel

; =============================================================================
; Sehemu ya 0: Vifafanuzi vya ukubwa
; =============================================================================

%define MAX_SOURCE        1048576    ; 1 MB ya chanzo
%define MAX_TOKENS        65536      ; upeo wa tokeni
%define MAX_AST_NODES     65536      ; upeo wa nodi za AST
%define TEXT_BUF_SIZE     131072     ; 128 KB ya msimbo wa .text
%define DATA_BUF_SIZE     4096       ; 4 KB ya data ya ulimwengu
%define MAX_LABELS        512        ; upeo wa lebo
%define MAX_EXTERNS       256        ; upeo wa alama za nje
%define MAX_RELOCS        512        ; upeo wa marekebisho
%define MAX_GLOBALS       64         ; upeo wa vigezo vya ulimwengu
%define STR_POOL_SIZE     131072     ; bwawa la herufi (pangisho la kutosha kwa faili kubwa)
%define MAX_LOCALS        256        ; upeo wa vigezo vya ndani kwa kazi

; Aina za tokeni
%define TOK_NENO          1          ; jina au neno muhimu
%define TOK_NAMBARI       2          ; nambari kamili
%define TOK_NYOTA         3          ; *
%define TOK_MSHALE_KULIA  4          ; ->
%define TOK_FUNGO         5          ; {
%define TOK_FUNGA         6          ; }
%define TOK_MABANO_FUNGO      7      ; (
%define TOK_MABANO_FUNGA      8      ; )
%define TOK_MABANO_MKOA       9      ; [
%define TOK_MABANO_MKOA_FUNGA 10     ; ]
%define TOK_NUKTA_MKATO   11         ; ;
%define TOK_KOMA          12         ; ,
%define TOK_SAWA          13         ; =
%define TOK_JUWILI        14         ; :
%define TOK_NUKTA         15         ; .
%define TOK_ALAMA         16         ; &
%define TOK_ISHARA        17         ; alama ya hesabu (+ - / % | < > !)
%define TOK_MWISHO        18         ; mwisho wa faili

; Aina za nodi za AST (zinazolingana na msambazaji.swa)
%define AST_KAZI          2
%define AST_NAMBA         3
%define AST_KITAMBU       4
%define AST_JINA           5
%define AST_RUDISHA       6
%define AST_BLOCK          7
%define AST_TANGAZO        8
%define AST_WAMBILE        9
%define AST_KAZI_JINA      10
%define AST_PARAM          11
%define AST_KAMA           12
%define AST_WAKATI         13
%define AST_KWA            14
%define AST_VUNJA          15
%define AST_ENEKEZA        16
%define AST_LINGA          17
%define AST_ELEKEZA        18
%define AST_ENEKEZA_JINA   19
%define AST_LINGA_JINA     20
%define AST_NYOTA_JINA     21
%define AST_FUNGO_ELEKEZA  22
%define AST_ALAMA_ELEKEZA  23
%define AST_KIELELEZO      24
%define AST_NYOTA_KIELELEZO 25
%define AST_NYOTA_ELEKEZA  26
%define AST_NYOTA_FUNGO_ELEKEZA 27
%define AST_ELEKEZA_JINA   28
%define AST_ENEKEZA_FUNGO  29
%define AST_ELEKEZA_FUNGO  30
%define AST_MFUATANO       31
%define AST_HUSISHA        32
%define AST_TANGAZO_ULIM   35
%define AST_MUUNDO         36
%define AST_MUUNDO_KIELELEZO 37
%define AST_NYOTA          38
%define AST_ELEKEZA_PARAM  39
%define AST_MUUNDO_TANGAZO 40
%define AST_ELEKEZA_MFANO  41
%define AST_KAULI          42
%define AST_MAKOSA         43

; Ishara za hesabu
%define OP_JUMLISHA       0
%define OP_TOA             1
%define OP_ZIDISHA         2
%define OP_GAWANYA         3
%define OP_MODULO          4
%define OP_NA              5
%define OP_AU              6
%define OP_HAMISHA_KUSHOTO 7
%define OP_HAMISHA_KULIA   8
%define OP_SAWA_SAWA       9
%define OP_SIO_SAWA        10
%define OP_KIDOGO          11
%define OP_KUBWA           12
%define OP_KIDOGO_SAWA     13
%define OP_KUBWA_SAWA      14
%define OP_MAKOSA          15
%define OP_SAWA            16

; =============================================================================
; Sehemu ya 1: Data iliyosanifiwa
; =============================================================================

        section .data

; ---------- Ujumbe wa makosa ----------

msg_matamshi:   db "Matumizi: mbegu <chanzo.swa>", 10, 0
msg_elferr:     db "Hitilafu: kushindwa kuandika ELF", 10, 0
msg_parseerr:   db "Hitilafu: ulichanganuzi", 10, 0
msg_lexerr:     db "Hitilafu: ulisomaji", 10, 0
msg_oom:        db "Hitilafu: hakuna kumbukumbu", 10, 0

; ---------- Vifunguo vya maneno muhimu ----------

; Jedwali la maneno muhimu na aina zao za AST
; Kila ingizo: (neno, aina)
kw_tengeneza:   db "tengeneza", 0
kw_muundo:      db "muundo", 0
kw_rudisha:     db "rudisha", 0
kw_kama:        db "kama", 0
kw_sivyo:       db "sivyo", 0
kw_wakati:      db "wakati", 0
kw_kwa:         db "kwa", 0
kw_vunja:       db "vunja", 0
kw_husisha:     db "husisha", 0
kw_endelea:     db "endelea", 0
kw_ukubwa:      db "ukubwa", 0
kw_kama_sivyo:  db "kamasivyo", 0

; ---------- Majina ya aina ----------

tn_n8:          db "N8", 0
tn_n16:         db "N16", 0
tn_n32:         db "N32", 0
tn_n64:         db "N64", 0
tn_w0:          db "W0", 0
tn_muundo:      db "muundo", 0

; ---------- Herufi halisi ya .shstrtab ----------

; "\0.text\0.data\0.bss\0.symtab\0.strtab\0.rela.text\0.shstrtab\0"
; Hii ni baiti 55 zilizohesabiwa tayari
shstrtab_data:
        db 0
        db ".text", 0                   ; offset 1
        db ".data", 0                   ; offset 7
        db ".bss", 0                    ; offset 13
        db ".symtab", 0                 ; offset 18
        db ".strtab", 0                 ; offset 26
        db ".rela.text", 0              ; offset 34
        db ".shstrtab", 0               ; offset 45
shstrtab_data_end:
%define SHSTRTAB_SIZE 55

; ---------- Kiolezo cha kichwa cha ELF ----------

; Kichwa cha ELF (baiti 64) — tunatumia kiolezo na kujaza maeneo yanayobadilika
ehdr_template:
        db 0x7f, 0x45, 0x4c, 0x46      ; e_ident[0:4] — ELF magic
        db 2                            ; EI_CLASS = 64-bit
        db 1                            ; EI_DATA = little-endian
        db 1                            ; EI_VERSION
        db 0                            ; EI_OSABI = System V
        db 0, 0, 0, 0, 0, 0, 0, 0      ; e_ident[8:16] — padding
        dw 1                            ; e_type = ET_REL
        dw 62                           ; e_machine = x86-64 (0x3E)
        dd 1                            ; e_version
        dq 0                            ; e_entry (0 kwa .o)
        dq 0                            ; e_phoff (0 kwa .o)
        dq 0                            ; e_shoff (itajazwa)
        dd 0                            ; e_flags
        dw 64                           ; e_ehsize
        dw 0                            ; e_phentsize (0 kwa .o)
        dw 0                            ; e_phnum
        dw 64                           ; e_shentsize
        dw 7                            ; e_shnum (itajazwa ikiwa tofauti)
        dw 6                            ; e_shstrndx

; =============================================================================
; Sehemu ya 2: BSS (vibafa na vigezo visivyosanifiwa)
; =============================================================================

        section .bss

; ---------- Bafa la chanzo ----------
source_buf:     resb MAX_SOURCE
source_len:     resq 1

; ---------- Bafa la tokeni ----------
; Kila tokeni: baiti 20 (aina:4, thamani:8, mstari:4, safu:2, urefu:2)
token_ty:       resd MAX_TOKENS
token_val:      resq MAX_TOKENS
token_line:     resd MAX_TOKENS
token_col:      resw MAX_TOKENS
token_len:      resw MAX_TOKENS
token_count:    resq 1
token_pos:      resq 1                  ; nafasi ya sasa ya usomaji wa tokeni

; ---------- Bafa la chanzo kwa mchanganuzi ----------
; Hifadhi anwani ya mwanzo ya kila tokeni ya neno/jina kwenye chanzo
token_text:     resq MAX_TOKENS

; ---------- AST (safu sambamba) ----------
ast_aina:       resd MAX_AST_NODES      ; aina ya nodi
ast_kushoto:    resd MAX_AST_NODES      ; mtoto wa kushoto
ast_kulia:      resd MAX_AST_NODES      ; mtoto wa kulia
ast_tiga:       resd MAX_AST_NODES      ; mtoto wa tatu
ast_nne:        resd MAX_AST_NODES      ; ndugu anayefuata
ast_thamani:    resd MAX_AST_NODES      ; thamani (kwa nambari)
ast_jina_off:   resd MAX_AST_NODES      ; ofseti ya jina kwenye bwawa la herufi
ast_count:      resq 1

; ---------- Bwawa la herufi ----------
str_pool:       resb STR_POOL_SIZE
str_pool_pos:   resq 1

; ---------- Bafa za msimbo ----------
text_buf:       resb TEXT_BUF_SIZE
text_buf_pos:   resq 1

data_buf:       resb DATA_BUF_SIZE
data_buf_pos:   resq 1

; ---------- Lebo za kazi ----------
label_name:     resq MAX_LABELS         ; anwani ya jina (kwenye str_pool)
label_offset:   resd MAX_LABELS         ; ofseti kwenye text_buf
label_size:     resd MAX_LABELS         ; ukubwa wa kazi
label_count:    resq 1

; ---------- Nje (alama za nje) ----------
extern_name:    resq MAX_EXTERNS        ; anwani ya jina
extern_count:   resq 1

; ---------- Marekebisho (relocations) ----------
rela_offset:    resd MAX_RELOCS         ; ofseti kwenye text_buf
rela_sym:       resd MAX_RELOCS         ; faharisi ya alama au -1
rela_count:     resq 1

; ---------- Vigezo vya ulimwengu ----------
global_name:    resq MAX_GLOBALS
global_offset:  resd MAX_GLOBALS
global_size:    resd MAX_GLOBALS
global_count:   resq 1

; ---------- Vigezo vya ndani (kwa kazi ya sasa) ----------
local_name:     resq MAX_LOCALS         ; anwani ya jina
local_offset:   resd MAX_LOCALS         ; ofseti ya rafu (kutoka rbp, hasi)
local_size:     resd MAX_LOCALS         ; ukubwa kwa baiti
local_count:    resq 1

; ---------- Kina cha mzunguko ----------
loop_depth:     resq 1
loop_break_label: resq 16               ; lebo za vunja (ufungwaji wa mipaka ya rafu)

; ---------- Hali ya mkusanyaji ----------
compiler_state: resq 1                  ; 0=sawa, 1=kosa
compiler_error_msg: resq 1              ; ujumbe wa kosa

; ---------- Bafa la matokeo ya ELF ----------
; ELF header (64) + section data + section headers
; Tutatoa moja kwa moja kwa stdout, kwa hivyo hatuhitaji bafa kubwa la matokeo

; ---------- Bafa la muda la uunganishi ----------
tmp_buf:        resb 4096               ; bafa la matumizi ya jumla

; =============================================================================
; Sehemu ya 3: Msaada — Kazi za kumbukumbu na herufi
; =============================================================================

        section .text

; -------------------------------------------------------
; urefu_wa_mfuatano: rudisha urefu wa mfuatano unaoishia '\0'
;   rdi = anwani ya mfuatano
;   rax = urefu
; -------------------------------------------------------
urefu_wa_mfuatano:
        xor     eax, eax
.loop:
        cmp     byte [rdi + rax], 0
        je      .done
        inc     rax
        jmp     .loop
.done:
        ret

; -------------------------------------------------------
; linganisha_mfuatano: linganisha nyuzi mbili
;   rdi = a, rsi = b
;   rax = 0 ikiwa sawa, !=0 ikiwa tofauti
; -------------------------------------------------------
linganisha_mfuatano:
        push    rdi
        push    rsi
.loop:
        mov     al, [rdi]
        mov     cl, [rsi]
        cmp     al, cl
        jne     .not_equal
        cmp     al, 0
        je      .equal
        inc     rdi
        inc     rsi
        jmp     .loop
.equal:
        xor     eax, eax
        pop     rsi
        pop     rdi
        ret
.not_equal:
        mov     eax, 1
        pop     rsi
        pop     rdi
        ret

; -------------------------------------------------------
; linganisha_neno_muhimu: angalia ikiwa tokeni inalingana na neno
;   rdi = anwani ya tokeni (kwenye chanzo)
;   esi = urefu wa tokeni
;   rdx = anwani ya neno muhimu
;   rax = 0 ikiwa sawa, !=0 ikiwa tofauti
; -------------------------------------------------------
linganisha_neno_muhimu:
        ; rdx = anwani ya neno muhimu (mfuatano unaoishia '\0')
        ; rsi = anwani ya maandishi ya tokeni
        ; ecx = urefu wa tokeni (idadi ya herufi)
        push    rdi
        push    rsi
        push    rcx
        push    rdx
        mov     rdi, rdx                ; neno muhimu kwenye rdi kwa kulinganisha
        cld
.loop:
        cmp     ecx, 0
        je      .check_keyword_end
        mov     al, [rsi]               ; herufi kutoka tokeni
        cmp     byte [rdi], 0
        je      .not_equal              ; neno muhimu fupi kuliko tokeni
        cmp     al, [rdi]
        jne     .not_equal
        inc     rdi
        inc     rsi
        dec     ecx
        jmp     .loop
.check_keyword_end:
        cmp     byte [rdi], 0
        jne     .not_equal              ; neno muhimu refu kuliko tokeni
        xor     eax, eax                ; zinalingana
        jmp     .done
.not_equal:
        mov     eax, 1
.done:
        pop     rdx
        pop     rcx
        pop     rsi
        pop     rdi
        ret

; -------------------------------------------------------
; nakili: nakili kumbukumbu (kama memcpy)
;   rdi = lengwa, rsi = chanzo, rdx = urefu
; -------------------------------------------------------
nakili:
        push    rdi
        push    rsi
        push    rcx
        mov     rcx, rdx
        cld
        rep movsb
        pop     rcx
        pop     rsi
        pop     rdi
        ret

; -------------------------------------------------------
; andika_baiti: andika baiti moja kwa bafa la .text
;   al = baiti ya kuandika
;   inatumia na kurekebisha text_buf_pos
; -------------------------------------------------------
andika_baiti:
        push    rdi
        mov     rdi, [text_buf_pos]
        cmp     rdi, TEXT_BUF_SIZE
        jae     .overflow
        lea     rdi, [text_buf + rdi]
        mov     [rdi], al
        inc     qword [text_buf_pos]
        pop     rdi
        ret
.overflow:
        ; TODO: ripoti kosa
        pop     rdi
        ret

; -------------------------------------------------------
; andika_neno4: andika neno la baiti 4 kwa bafa la .text
;   edi = thamani ya kuandika
; -------------------------------------------------------
andika_neno4:
        push    rdi
        push    rcx
        mov     rcx, [text_buf_pos]
        cmp     rcx, TEXT_BUF_SIZE - 4
        jae     .overflow
        lea     rcx, [text_buf + rcx]
        mov     [rcx], edi
        add     qword [text_buf_pos], 4
        pop     rcx
        pop     rdi
        ret
.overflow:
        pop     rcx
        pop     rdi
        ret

; -------------------------------------------------------
; andika_neno8: andika neno la baiti 8 kwa bafa la .text
;   rdi = thamani ya kuandika
; -------------------------------------------------------
andika_neno8:
        push    rdi
        push    rcx
        mov     rcx, [text_buf_pos]
        cmp     rcx, TEXT_BUF_SIZE - 8
        jae     .overflow
        lea     rcx, [text_buf + rcx]
        mov     [rcx], rdi
        add     qword [text_buf_pos], 8
        pop     rcx
        pop     rdi
        ret
.overflow:
        pop     rcx
        pop     rdi
        ret

; -------------------------------------------------------
; andika_neno8_moja_kwa_moja: andika neno8 moja kwa moja kwa stdout
;   rdi = thamani
; -------------------------------------------------------
andika_neno8_moja_kwa_moja:
        ; Hifadhi thamani kwenye bafa la muda na tumia sys_write
        push    rdi
        push    rsi
        push    rdx
        push    rax
        mov     qword [tmp_buf], rdi
        mov     rdi, 1                  ; stdout
        lea     rsi, [tmp_buf]
        mov     rdx, 8
        mov     rax, 1                  ; sys_write
        syscall
        pop     rax
        pop     rdx
        pop     rsi
        pop     rdi
        ret

; -------------------------------------------------------
; andika_neno4_moja_kwa_moja: andika baiti 4 kwa stdout
;   edi = thamani
; -------------------------------------------------------
andika_neno4_moja_kwa_moja:
        push    rdi
        push    rsi
        push    rdx
        push    rax
        mov     dword [tmp_buf], edi
        mov     rdi, 1                  ; stdout
        lea     rsi, [tmp_buf]
        mov     rdx, 4
        mov     rax, 1                  ; sys_write
        syscall
        pop     rax
        pop     rdx
        pop     rsi
        pop     rdi
        ret

; -------------------------------------------------------
; andika_baiti_moja_kwa_moja: andika baiti 1 kwa stdout
;   dil = baiti
; -------------------------------------------------------
andika_baiti_moja_kwa_moja:
        push    rdi
        push    rsi
        push    rdx
        push    rax
        mov     byte [tmp_buf], dil
        mov     rdi, 1                  ; stdout
        lea     rsi, [tmp_buf]
        mov     rdx, 1
        mov     rax, 1                  ; sys_write
        syscall
        pop     rax
        pop     rdx
        pop     rsi
        pop     rdi
        ret

; -------------------------------------------------------
; andika_neno2_moja_kwa_moja: andika baiti 2 kwa stdout
;   di = thamani
; -------------------------------------------------------
andika_neno2_moja_kwa_moja:
        push    rdi
        push    rsi
        push    rdx
        push    rax
        mov     word [tmp_buf], di
        mov     rdi, 1                  ; stdout
        lea     rsi, [tmp_buf]
        mov     rdx, 2
        mov     rax, 1                  ; sys_write
        syscall
        pop     rax
        pop     rdx
        pop     rsi
        pop     rdi
        ret

; -------------------------------------------------------
; sys_exit: toka kwa msimbo
;   edi = msimbo wa kutoka
; -------------------------------------------------------
sys_exit:
        mov     eax, 60                 ; sys_exit
        syscall

; -------------------------------------------------------
; sys_write_buf: andika bafa kwa stdout
;   rsi = anwani, rdx = urefu
; -------------------------------------------------------
sys_write_buf:
        mov     rdi, 1                  ; stdout
        mov     rax, 1                  ; sys_write
        syscall
        ret

; -------------------------------------------------------
; sys_read_all: soma faili lote kwenye bafa
;   rdi = fd, rsi = anwani ya bafa, rdx = upeo wa ukubwa
;   rax = idadi ya baiti zilizosomwa
; -------------------------------------------------------
sys_read_all:
        mov     rax, 0                  ; sys_read
        syscall
        ret

; -------------------------------------------------------
; andika_mfuatano: andika mfuatano wa C kwa stdout
;   rdi = anwani ya mfuatano
; -------------------------------------------------------
andika_mfuatano:
        push    rdi
        call    urefu_wa_mfuatano
        mov     rsi, rdi
        mov     rdx, rax
        call    sys_write_buf
        pop     rdi
        ret

; -------------------------------------------------------
; soma_chanzo_kutoka_stdin: soma chanzo chote kutoka stdin
; -------------------------------------------------------
soma_chanzo_kutoka_stdin:
        lea     rsi, [source_buf]
        mov     rdx, MAX_SOURCE
        mov     rdi, 0                  ; stdin
        call    sys_read_all
        cmp     rax, 0
        jl      .error
        mov     [source_len], rax
        ret
.error:
        lea     rdi, [msg_lexerr]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

; -------------------------------------------------------
; soma_chanzo_kutoka_faili: soma chanzo kutoka jina la faili
;   rdi = anwani ya jina la faili
; -------------------------------------------------------
soma_chanzo_kutoka_faili:
        push    r12
        mov     r12, rdi                ; hifadhi jina la faili

        ; Fungua faili
        mov     rdi, r12
        mov     rsi, 0                  ; O_RDONLY
        mov     rax, 2                  ; sys_open
        syscall
        cmp     rax, 0
        jl      .error

        ; Soma faili
        mov     rdi, rax
        lea     rsi, [source_buf]
        mov     rdx, MAX_SOURCE
        mov     rax, 0                  ; sys_read
        syscall
        mov     [source_len], rax

        pop     r12
        ret
.error:
        lea     rdi, [msg_lexerr]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

; =============================================================================
; Sehemu ya 4: Msomaji (Lexer)
; =============================================================================

; -------------------------------------------------------
; ni_nafasi: angalia ikiwa herufi ni nafasi
;   al = herufi
;   kurudisha: ZF=1 ikiwa ni nafasi
; -------------------------------------------------------
ni_nafasi:
        cmp     al, ' '
        je      .yes
        cmp     al, 9                   ; tab
        je      .yes
        cmp     al, 10                  ; newline
        je      .yes
        cmp     al, 13                  ; carriage return
        je      .yes
        cmp     al, 0
.yes:
        ret

; -------------------------------------------------------
; ni_herufi: angalia ikiwa ni herufi au _
;   al = herufi
; -------------------------------------------------------
ni_herufi:
        cmp     al, 'a'
        jl      .check_upper
        cmp     al, 'z'
        jle     .yes
.check_upper:
        cmp     al, 'A'
        jl      .no
        cmp     al, 'Z'
        jle     .yes
        cmp     al, '_'
        je      .yes
.no:
        mov     eax, 0
        ret
.yes:
        mov     eax, 1
        ret

; -------------------------------------------------------
; ni_tarakimu: angalia ikiwa ni tarakimu
; -------------------------------------------------------
ni_tarakimu:
        cmp     al, '0'
        jl      .no
        cmp     al, '9'
        jg      .no
        mov     eax, 1
        ret
.no:
        xor     eax, eax
        ret

; -------------------------------------------------------
; ni_herufi_au_tarakimu: herufi, tarakimu, au _
; -------------------------------------------------------
ni_herufi_au_tarakimu:
        push    rax                     ; hifadhi al (herufi halisi)
        call    ni_herufi
        cmp     eax, 1
        je      .yes_pop
        pop     rax                     ; rejesha al kwa ni_tarakimu
        call    ni_tarakimu
        cmp     eax, 1
        je      .yes
        xor     eax, eax
        ret
.yes_pop:
        pop     rax                     ; tupa rax iliyohifadhiwa
.yes:
        mov     eax, 1
        ret

; -------------------------------------------------------
; ruka_maelezo: ruka maoni ya // hadi mwisho wa mstari
;   r12 = nafasi ya sasa (faharisi kwenye chanzo)
;   r13 = anwani ya chanzo
;   r14 = urefu wa chanzo
;   inarekebisha r12
; -------------------------------------------------------
ruka_maelezo:
        cmp     r12, r14
        jae     .done
        mov     al, [r13 + r12]
        cmp     al, '/'
        jne     .done
        cmp     r12, r14
        je      .done
        mov     al, [r13 + r12 + 1]
        cmp     al, '/'
        jne     .done
        ; Ni maoni ya //
        add     r12, 2
.loop:
        cmp     r12, r14
        jae     .done
        mov     al, [r13 + r12]
        cmp     al, 10                  ; newline
        je      .newline
        cmp     al, 0
        je      .done
        inc     r12
        jmp     .loop
.newline:
        inc     r12                     ; ruka newline pia
.done:
        ret

; -------------------------------------------------------
; ruka_nafasi_na_maelezo: ruka nafasi nyeupe na maoni
;   inarekebisha r12 (faharisi)
; -------------------------------------------------------
ruka_nafasi_na_maelezo:
        cmp     r12, r14
        jae     .done
.loop:
        cmp     r12, r14
        jae     .done
        mov     al, [r13 + r12]
        cmp     al, '/'
        je      .check_comment
        cmp     al, ' '
        je      .skip
        cmp     al, 9
        je      .skip
        cmp     al, 10
        je      .skip
        cmp     al, 13
        je      .skip
        jmp     .done
.check_comment:
        mov     al, [r13 + r12 + 1]
        cmp     al, '/'
        jne     .done
        call    ruka_maelezo
        jmp     .loop
.skip:
        inc     r12
        jmp     .loop
.done:
        ret

; -------------------------------------------------------
; soma_neno: soma neno kutoka chanzo
;   r12 = faharisi ya sasa
;   r13 = anwani ya chanzo
;   r14 = urefu wa chanzo
;   rax = TOK_NENO ikifaulu, 0 ikiwa hakuna neno
;   rbx = anwani ya mwanzo ya neno
;   rcx = urefu wa neno
;   inarekebisha r12
; -------------------------------------------------------
soma_neno:
        push    r12
        call    ruka_nafasi_na_maelezo
        cmp     r12, r14
        jae     .fail
        mov     al, [r13 + r12]
        call    ni_herufi
        cmp     eax, 1
        jne     .fail
        mov     rbx, r12                ; anwani ya mwanzo (faharisi)
        add     rbx, r13                ; anwani halisi
        xor     ecx, ecx
.loop:
        cmp     r12, r14
        jae     .done
        mov     al, [r13 + r12]
        call    ni_herufi_au_tarakimu
        cmp     eax, 1
        jne     .done
        inc     r12
        inc     ecx
        jmp     .loop
.done:
        mov     eax, TOK_NENO
        jmp     .ret
.fail:
        mov     eax, 0
        mov     rbx, 0
        mov     ecx, 0
.ret:
        add     rsp, 8                  ; tupa r12 iliyohifadhiwa (usiharibu rax/eax)
        ret

; -------------------------------------------------------
; soma_nambari: soma nambari kamili
;   r12 = faharisi ya sasa
;   rax = TOK_NAMBARI ikifaulu, 0 ikiwa hakuna
;   rbx = thamani ya nambari (64-bit)
;   inarekebisha r12
; -------------------------------------------------------
soma_nambari:
        push    r12
        call    ruka_nafasi_na_maelezo
        cmp     r12, r14
        jae     .fail
        mov     al, [r13 + r12]
        call    ni_tarakimu
        cmp     eax, 1
        jne     .fail
        xor     ebx, ebx
.loop:
        cmp     r12, r14
        jae     .done
        mov     al, [r13 + r12]
        call    ni_tarakimu
        cmp     eax, 1
        jne     .done
        movzx   ecx, byte [r13 + r12]   ; soma tena baada ya ni_tarakimu kuharibu al
        sub     ecx, '0'
        imul    rbx, rbx, 10
        add     rbx, rcx
        inc     r12
        jmp     .loop
.done:
        mov     eax, TOK_NAMBARI
        jmp     .ret
.fail:
        mov     eax, 0
        xor     ebx, ebx
.ret:
        pop     rcx                     ; tupa r12 ya awali
        ret

; -------------------------------------------------------
; soma_ishara: soma alama ya hesabu au ulinganisho
;   inarekebisha r12
;   rax = TOK_ISHARA, 0 ikiwa hakuna
;   rbx = msimbo wa ishara
; -------------------------------------------------------
soma_ishara:
        push    r12
        call    ruka_nafasi_na_maelezo
        cmp     r12, r14
        jae     .fail
        mov     al, [r13 + r12]
        cmp     al, '+'
        je      .plus
        cmp     al, '-'
        je      .arrow_or_minus
        cmp     al, '*'
        je      .star
        cmp     al, '/'
        je      .slash
        cmp     al, '%'
        je      .mod
        cmp     al, '&'
        je      .and
        cmp     al, '|'
        je      .or
        cmp     al, '<'
        je      .lt_or_shift_or_le
        cmp     al, '>'
        je      .gt_or_shift_or_ge
        cmp     al, '='
        je      .eq_or_assign
        cmp     al, '!'
        je      .ne
        jmp     .fail
.plus:
        inc     r12
        mov     eax, TOK_ISHARA
        mov     ebx, OP_JUMLISHA
        jmp     .ret
.arrow_or_minus:
        inc     r12
        cmp     r12, r14
        jae     .is_minus
        mov     al, [r13 + r12]
        cmp     al, '>'
        je      .arrow
.is_minus:
        mov     eax, TOK_ISHARA
        mov     ebx, OP_TOA
        jmp     .ret
.arrow:
        inc     r12
        mov     eax, TOK_MSHALE_KULIA
        mov     ebx, 0
        jmp     .ret
.star:
        inc     r12
        mov     eax, TOK_NYOTA
        mov     ebx, OP_ZIDISHA
        jmp     .ret
.slash:
        inc     r12
        ; Angalia ikiwa ni maoni
        cmp     r12, r14
        jae     .is_div
        mov     al, [r13 + r12]
        cmp     al, '/'
        je      .comment_in_expr
.is_div:
        mov     eax, TOK_ISHARA
        mov     ebx, OP_GAWANYA
        jmp     .ret
.comment_in_expr:
        dec     r12                     ; rudi nyuma, ruka_maelezo itashughulikia
        call    ruka_maelezo
        jmp     soma_ishara             ; jaribu tena baada ya maoni
.mod:
        inc     r12
        mov     eax, TOK_ISHARA
        mov     ebx, OP_MODULO
        jmp     .ret
.and:
        inc     r12
        mov     eax, TOK_ALAMA
        mov     ebx, OP_NA
        jmp     .ret
.or:
        inc     r12
        ; Angalia ikiwa ni ||
        cmp     r12, r14
        jae     .is_or
        mov     al, [r13 + r12]
        cmp     al, '|'
        je      .bor
.is_or:
        mov     eax, TOK_ISHARA
        mov     ebx, OP_AU
        jmp     .ret
.bor:
        inc     r12
        mov     eax, TOK_ISHARA
        mov     ebx, OP_AU
        jmp     .ret
.lt_or_shift_or_le:
        inc     r12
        cmp     r12, r14
        jae     .is_lt
        mov     al, [r13 + r12]
        cmp     al, '<'
        je      .shl
        cmp     al, '='
        je      .le
.is_lt:
        mov     eax, TOK_ISHARA
        mov     ebx, OP_KIDOGO
        jmp     .ret
.shl:
        inc     r12
        mov     eax, TOK_ISHARA
        mov     ebx, OP_HAMISHA_KUSHOTO
        jmp     .ret
.le:
        inc     r12
        mov     eax, TOK_ISHARA
        mov     ebx, OP_KIDOGO_SAWA
        jmp     .ret
.gt_or_shift_or_ge:
        inc     r12
        cmp     r12, r14
        jae     .is_gt
        mov     al, [r13 + r12]
        cmp     al, '>'
        je      .shr
        cmp     al, '='
        je      .ge
.is_gt:
        mov     eax, TOK_ISHARA
        mov     ebx, OP_KUBWA
        jmp     .ret
.shr:
        inc     r12
        mov     eax, TOK_ISHARA
        mov     ebx, OP_HAMISHA_KULIA
        jmp     .ret
.ge:
        inc     r12
        mov     eax, TOK_ISHARA
        mov     ebx, OP_KUBWA_SAWA
        jmp     .ret
.eq_or_assign:
        inc     r12
        cmp     r12, r14
        jae     .is_assign
        mov     al, [r13 + r12]
        cmp     al, '='
        je      .eq
.is_assign:
        mov     eax, TOK_SAWA
        mov     ebx, 0
        jmp     .ret
.eq:
        inc     r12
        mov     eax, TOK_ISHARA
        mov     ebx, OP_SAWA_SAWA
        jmp     .ret
.ne:
        inc     r12
        cmp     r12, r14
        jae     .fail
        mov     al, [r13 + r12]
        cmp     al, '='
        jne     .fail
        inc     r12
        mov     eax, TOK_ISHARA
        mov     ebx, OP_SIO_SAWA
        jmp     .ret
.fail:
        mov     eax, 0
        xor     ebx, ebx
.ret:
        pop     rcx                     ; tupa r12 ya awali
        ret

; -------------------------------------------------------
; changanua_chanzo: badilisha chanzo chote kuwa tokeni
;   inajaza safu za token_*
; -------------------------------------------------------
changanua_chanzo:
        ; Weka upya vihesabio
        mov     qword [token_count], 0
        mov     qword [token_pos], 0

        ; r12 = faharisi ya sasa kwenye chanzo
        ; r13 = anwani ya chanzo
        ; r14 = urefu wa chanzo
        xor     r12d, r12d
        lea     r13, [source_buf]
        mov     r14, [source_len]

.changanua_loop:
        cmp     r12, r14
        jae     .changanua_done

        call    ruka_nafasi_na_maelezo
        cmp     r12, r14
        jae     .changanua_done

        mov     r15, [token_count]
        cmp     r15, MAX_TOKENS - 1
        jae     .changanua_done

        ; Anwani ya mwanzo ya tokeni
        mov     [token_text + r15*8], r12  ; hifadhi faharisi ya mwanzo

        mov     al, [r13 + r12]

        ; Nambari
        cmp     al, '0'
        jl      .try_word
        cmp     al, '9'
        jg      .try_word

        call    soma_nambari
        cmp     eax, 0
        je      .try_word
        jmp     .hifadhi_tokeni

.try_word:
        ; Neno (jina au neno muhimu)
        call    ni_herufi
        cmp     eax, 1
        jne     .try_symbols

        call    soma_neno
        cmp     eax, 0
        je      .try_symbols
        mov     [token_ty + r15*4], eax
        mov     [token_val + r15*8], rbx
        movzx   ecx, cx                 ; hakikisha urefu uko safi kwa kuhifadhi
        mov     [token_len + r15*2], cx

        mov     [token_text + r15*8], rbx ; hifadhi anwani halisi ya neno
        inc     qword [token_count]
        jmp     .changanua_loop

.try_symbols:
        ; Alama za hesabu na ulinganisho
        call    soma_ishara
        cmp     eax, 0
        je      .try_punctuation
        jmp     .hifadhi_tokeni_kwa_ishara

.try_punctuation:
        ; Alama za uakifishaji
        mov     al, [r13 + r12]
        cmp     al, '{'
        je      .tok_brace_open
        cmp     al, '}'
        je      .tok_brace_close
        cmp     al, '('
        je      .tok_paren_open
        cmp     al, ')'
        je      .tok_paren_close
        cmp     al, '['
        je      .tok_bracket_open
        cmp     al, ']'
        je      .tok_bracket_close
        cmp     al, ';'
        je      .tok_semicolon
        cmp     al, ','
        je      .tok_comma
        cmp     al, ':'
        je      .tok_colon
        cmp     al, '.'
        je      .tok_dot
        cmp     al, '&'
        je      .tok_ampersand

        ; Herufi isiyojulikana — ruka
        inc     r12
        jmp     .changanua_loop

.tok_brace_open:
        mov     eax, TOK_FUNGO
        mov     ebx, 0
        inc     r12
        jmp     .hifadhi_tokeni
.tok_brace_close:
        mov     eax, TOK_FUNGA
        mov     ebx, 0
        inc     r12
        jmp     .hifadhi_tokeni
.tok_paren_open:
        mov     eax, TOK_MABANO_FUNGO
        mov     ebx, 0
        inc     r12
        jmp     .hifadhi_tokeni
.tok_paren_close:
        mov     eax, TOK_MABANO_FUNGA
        mov     ebx, 0
        inc     r12
        jmp     .hifadhi_tokeni
.tok_bracket_open:
        mov     eax, TOK_MABANO_MKOA
        mov     ebx, 0
        inc     r12
        jmp     .hifadhi_tokeni
.tok_bracket_close:
        mov     eax, TOK_MABANO_MKOA_FUNGA
        mov     ebx, 0
        inc     r12
        jmp     .hifadhi_tokeni
.tok_semicolon:
        mov     eax, TOK_NUKTA_MKATO
        mov     ebx, 0
        inc     r12
        jmp     .hifadhi_tokeni
.tok_comma:
        mov     eax, TOK_KOMA
        mov     ebx, 0
        inc     r12
        jmp     .hifadhi_tokeni
.tok_colon:
        mov     eax, TOK_JUWILI
        mov     ebx, 0
        inc     r12
        jmp     .hifadhi_tokeni
.tok_dot:
        inc     r12
        cmp     r12, r14
        jae     .hifadhi_dot
        mov     al, [r13 + r12]
        cmp     al, '.'
        je      .tok_dotdot
.hifadhi_dot:
        mov     eax, TOK_NUKTA
        mov     ebx, 0
        jmp     .hifadhi_tokeni
.tok_dotdot:
        inc     r12
        mov     eax, TOK_ISHARA
        mov     ebx, OP_MAKOSA
        jmp     .hifadhi_tokeni_kwa_ishara
.tok_ampersand:
        inc     r12
        cmp     r12, r14
        jae     .hifadhi_amp
        mov     al, [r13 + r12]
        cmp     al, '&'
        je      .tok_and
.hifadhi_amp:
        mov     eax, TOK_ALAMA
        mov     ebx, 0
        jmp     .hifadhi_tokeni
.tok_and:
        inc     r12
        mov     eax, TOK_ISHARA
        mov     ebx, OP_NA
        jmp     .hifadhi_tokeni_kwa_ishara

.hifadhi_tokeni_kwa_ishara:
        mov     r15, [token_count]
        cmp     r15, MAX_TOKENS - 1
        jae     .changanua_done
        mov     [token_ty + r15*4], eax
        mov     [token_val + r15*8], rbx
        mov     word [token_len + r15*2], 1
        inc     qword [token_count]
        jmp     .changanua_loop

.hifadhi_tokeni:
        mov     r15, [token_count]
        cmp     r15, MAX_TOKENS - 1
        jae     .changanua_done
        mov     [token_ty + r15*4], eax
        mov     [token_val + r15*8], rbx
        mov     word [token_len + r15*2], 1
        inc     qword [token_count]
        jmp     .changanua_loop

.changanua_done:
        ; Ongeza tokeni ya mwisho
        mov     r15, [token_count]
        mov     dword [token_ty + r15*4], TOK_MWISHO
        mov     qword [token_val + r15*8], 0
        mov     word [token_len + r15*2], 0
        inc     qword [token_count]
        ret

; -------------------------------------------------------
; tokeni_sasa: rudisha aina ya tokeni ya sasa
;   rax = aina ya tokeni
; -------------------------------------------------------
tokeni_sasa:
        mov     rax, [token_pos]
        mov     eax, [token_ty + rax*4]
        ret

; -------------------------------------------------------
; tokeni_songa: sogea mbele kwenye tokeni
; -------------------------------------------------------
tokeni_songa:
        inc     qword [token_pos]
        ret

; -------------------------------------------------------
; tokeni_val_sasa: rudisha thamani ya tokeni ya sasa
;   rax = thamani
; -------------------------------------------------------
tokeni_val_sasa:
        mov     rax, [token_pos]
        mov     rax, [token_val + rax*8]
        ret

; -------------------------------------------------------
; tokeni_text_sasa: rudisha anwani ya maandishi ya tokeni ya sasa
;   rax = anwani
; -------------------------------------------------------
tokeni_text_sasa:
        mov     rax, [token_pos]
        mov     rax, [token_text + rax*8]
        ret

; -------------------------------------------------------
; tokeni_len_sasa: rudisha urefu wa tokeni ya sasa
;   rax = urefu
; -------------------------------------------------------
tokeni_len_sasa:
        mov     rax, [token_pos]
        movzx   eax, word [token_len + rax*2]
        ret

; -------------------------------------------------------
; tarajia_ishara: angalia ikiwa tokeni ya sasa ni aina fulani
;   edi = aina inayotarajiwa
;   rax = 1 ikiwa sawa, 0 ikiwa tofauti
;   ikifaulu, inasongeza mbele
; -------------------------------------------------------
tarajia_ishara:
        push    rbx
        mov     rbx, [token_pos]
        mov     ebx, [token_ty + rbx*4]
        cmp     ebx, edi
        jne     .fail
        inc     qword [token_pos]
        mov     eax, 1
        pop     rbx
        ret
.fail:
        xor     eax, eax
        pop     rbx
        ret

; -------------------------------------------------------
; tarajia_ishara_hesabu: angalia ikiwa tokeni ya sasa ni ishara maalum
;   edi = msimbo wa ishara inayotarajiwa
;   rax = 1 ikiwa sawa, 0 ikiwa tofauti
; -------------------------------------------------------
tarajia_ishara_hesabu:
        push    rbx
        mov     rbx, [token_pos]
        cmp     dword [token_ty + rbx*4], TOK_ISHARA
        jne     .fail
        cmp     qword [token_val + rbx*8], rdi
        jne     .fail
        inc     qword [token_pos]
        mov     eax, 1
        pop     rbx
        ret
.fail:
        xor     eax, eax
        pop     rbx
        ret

; -------------------------------------------------------
; tarajia_neno: angalia ikiwa tokeni ya sasa inalingana na neno
;   rdi = anwani ya neno linalotarajiwa
;   rax = 1 ikiwa sawa, 0 ikiwa tofauti
; -------------------------------------------------------
tarajia_neno:
        push    rbx
        push    rcx
        push    rdx
        mov     rbx, [token_pos]
        cmp     dword [token_ty + rbx*4], TOK_NENO
        jne     .fail
        mov     rsi, [token_text + rbx*8]
        movzx   ecx, word [token_len + rbx*2]
        mov     rdx, rdi                ; neno muhimu (kutoka rdi) → rdx
        call    linganisha_neno_muhimu
        cmp     eax, 0
        jne     .fail
        inc     qword [token_pos]
        mov     eax, 1
        pop     rdx
        pop     rcx
        pop     rbx
        ret
.fail:
        xor     eax, eax
        pop     rdx
        pop     rcx
        pop     rbx
        ret

; =============================================================================
; Sehemu ya 5: Mchanganuzi (Parser)
; =============================================================================

; -------------------------------------------------------
; ast_nodi_mpya: unda nodi mpya ya AST
;   r8d = aina ya nodi
;   r9d = kushoto, r10d = kulia, r11d = tiga
;   rax = faharisi ya nodi mpya (N32)
; -------------------------------------------------------
ast_nodi_mpya:
        push    rbx
        mov     rbx, [ast_count]
        cmp     rbx, MAX_AST_NODES - 1
        jae     .fail
        mov     [ast_aina + rbx*4], r8d
        mov     [ast_kushoto + rbx*4], r9d
        mov     [ast_kulia + rbx*4], r10d
        mov     [ast_tiga + rbx*4], r11d
        mov     dword [ast_nne + rbx*4], -1
        mov     dword [ast_thamani + rbx*4], 0
        mov     dword [ast_jina_off + rbx*4], 0
        inc     qword [ast_count]
        mov     eax, ebx
        pop     rbx
        ret
.fail:
        mov     eax, -1
        pop     rbx
        ret

; -------------------------------------------------------
; hifadhi_jina: hifadhi jina kwenye bwawa la herufi
;   rsi = anwani ya jina
;   ecx = urefu wa jina
;   rax = ofseti kwenye str_pool
; -------------------------------------------------------
hifadhi_jina:
        push    rdi
        push    rsi
        push    rcx
        push    rdx
        mov     rdx, [str_pool_pos]
        ; Angalia kufurika kwa str_pool
        lea     rax, [rdx + rcx + 1]     ; nafasi inayohitajika
        cmp     rax, STR_POOL_SIZE
        jae     .furika
        lea     rdi, [str_pool + rdx]
        cld
        rep movsb
        mov     byte [rdi], 0            ; mwisho wa mfuatano
        mov     rax, [str_pool_pos]
        pop     rdx
        pop     rcx
        ; Ongeza urefu + 1 kwa str_pool_pos
        add     rcx, 1
        add     [str_pool_pos], rcx
        pop     rsi
        pop     rdi
        ret
.furika:
        ; Rudisha faharisi batili
        mov     rax, 0
        pop     rdx
        pop     rcx
        pop     rsi
        pop     rdi
        ret

; -------------------------------------------------------
; tangaza mbele kwa vitendakazi
; -------------------------------------------------------
; Tunahitaji kutangaza mbele kazi za kuchanganua
; Katika assembly, tunatumia lebo za kawaida

; -------------------------------------------------------
; changanua_aina: changanua jina la aina
;   rax = nambari ya aina (0=haijulikani, 1=N8, 2=N16, 3=N32, 4=N64, 5=W0)
;   rbx = nyota_ya_aina (1 ikiwa ni nyota, 0 la sivyo)
; -------------------------------------------------------
changanua_aina:
        push    r12
        push    r13
        push    r14

        ; Angalia ikiwa ni aina ya msingi
        lea     rdi, [tn_w0]
        call    tarajia_neno
        cmp     eax, 1
        je      .is_w0

        lea     rdi, [tn_n8]
        call    tarajia_neno
        cmp     eax, 1
        je      .is_n8

        lea     rdi, [tn_n16]
        call    tarajia_neno
        cmp     eax, 1
        je      .is_n16

        lea     rdi, [tn_n32]
        call    tarajia_neno
        cmp     eax, 1
        je      .is_n32

        lea     rdi, [tn_n64]
        call    tarajia_neno
        cmp     eax, 1
        je      .is_n64

        ; Hakuna aina inayojulikana - rudi kwa unknown
        jmp     .unknown

.is_w0:
        mov     eax, 5
        xor     ebx, ebx
        jmp     .check_star
.is_n8:
        mov     eax, 1
        xor     ebx, ebx
        jmp     .check_star
.is_n16:
        mov     eax, 2
        xor     ebx, ebx
        jmp     .check_star
.is_n32:
        mov     eax, 3
        xor     ebx, ebx
        jmp     .check_star
.is_n64:
        mov     eax, 4
        xor     ebx, ebx
        jmp     .check_star

.check_star:
        ; Angalia ikiwa inafuatiwa na *
        push    rax
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NYOTA
        jne     .no_star
        inc     qword [token_pos]       ; tumia nyota
        mov     ebx, 1
        pop     rax
        jmp     .done
.no_star:
        xor     ebx, ebx
        pop     rax
        jmp     .done

.unknown:
        xor     eax, eax
        xor     ebx, ebx
.done:
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; ukubwa_kutoka_aina: rudisha ukubwa wa aina kwa baiti
;   edi = nambari ya aina (1=N8, 2=N16, 3=N32, 4=N64, 5=W0, 6=muundo)
;   rax = ukubwa kwa baiti
; -------------------------------------------------------
ukubwa_kutoka_aina:
        cmp     edi, 1                  ; N8
        je      .size1
        cmp     edi, 2                  ; N16
        je      .size2
        cmp     edi, 3                  ; N32
        je      .size4
        cmp     edi, 4                  ; N64
        je      .size8
        cmp     edi, 5                  ; W0
        je      .size0
        ; Muundo au nyota — tumia 8 (ukubwa wa anwani)
        mov     eax, 8
        ret
.size1:
        mov     eax, 1
        ret
.size2:
        mov     eax, 2
        ret
.size4:
        mov     eax, 4
        ret
.size8:
        mov     eax, 8
        ret
.size0:
        xor     eax, eax
        ret

; -------------------------------------------------------
; changanua_kipengele_msingi: changanua kipengele cha msingi cha usemi
;   rax = faharisi ya nodi ya AST
; -------------------------------------------------------
changanua_kipengele_msingi:
        ; Angalia nambari
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NAMBARI
        jne     .try_name

        ; Nambari halisi
        mov     rdi, [token_pos]
        mov     r8d, AST_NAMBA
        mov     r9d, -1
        mov     r10d, -1
        mov     r11d, -1
        call    ast_nodi_mpya
        cmp     eax, -1
        je      .fail
        push    rax
        mov     rdi, [token_pos]
        mov     rdi, [token_val + rdi*8]
        mov     rcx, [ast_count]
        mov     [ast_thamani + rcx*4 - 4], edi  ; nodi iliyoundwa hivi punde
        inc     qword [token_pos]
        pop     rax
        jmp     .done

.try_name:
        ; Angalia jina
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NENO
        jne     .try_paren

        ; Jina — angalia ikiwa ni kitambulisho rahisi
        mov     rdi, [token_pos]
        mov     rsi, [token_text + rdi*8]
        movzx   ecx, word [token_len + rdi*2]

        ; Unda nodi ya jina
        mov     r8d, AST_JINA
        mov     r9d, -1
        mov     r10d, -1
        mov     r11d, -1
        call    ast_nodi_mpya
        cmp     eax, -1
        je      .fail
        push    rax
        push    rcx
        push    rsi
        ; Hifadhi jina kwenye bwawa
        pop     rsi
        pop     rcx
        call    hifadhi_jina
        mov     rcx, [ast_count]
        mov     [ast_jina_off + rcx*4 - 4], eax
        inc     qword [token_pos]
        pop     rax
        jmp     .done

.try_paren:
        ; Angalia mabano (usemi)
        mov     edi, TOK_MABANO_FUNGO
        call    tarajia_ishara
        cmp     eax, 1
        jne     .try_string
        call    changanua_usemi
        push    rax
        mov     edi, TOK_MABANO_FUNGA
        call    tarajia_ishara
        pop     rax
        jmp     .done

.try_string:
        ; TODO: mfuatano halisi
        mov     eax, -1
.done:
        ret
.fail:
        mov     eax, -1
        ret

; -------------------------------------------------------
; changanua_kiambishi: changanua viambishi vya usemi ([], ->, .)
;   edi = nodi ya msingi
;   rax = nodi ya matokeo
; -------------------------------------------------------
changanua_kiambishi:
        push    r12
        mov     r12d, edi               ; hifadhi nodi ya kushoto
.loop:
        mov     rdi, [token_pos]
        mov     edi, [token_ty + rdi*4]

        ; Angalia [
        cmp     edi, TOK_MABANO_MKOA
        je      .faharisi

        ; Angalia ->
        cmp     edi, TOK_MSHALE_KULIA
        je      .mshale

        ; Angalia .
        cmp     edi, TOK_NUKTA
        je      .nukta

        ; Angalia (
        cmp     edi, TOK_MABANO_FUNGO
        je      .wito

        jmp     .done

.faharisi:
        inc     qword [token_pos]       ; tumia [
        call    changanua_usemi
        mov     r9d, r12d               ; kushoto
        mov     r10d, eax               ; kulia = faharisi
        mov     r8d, AST_KIELELEZO      ; faharisi ya safu
        mov     r11d, -1
        call    ast_nodi_mpya
        mov     r12d, eax
        mov     edi, TOK_MABANO_MKOA_FUNGA ; ]
        call    tarajia_ishara
        jmp     .loop

.mshale:
        inc     qword [token_pos]       ; tumia ->
        mov     rdi, [token_pos]
        mov     rsi, [token_text + rdi*8]
        movzx   ecx, word [token_len + rdi*2]
        inc     qword [token_pos]       ; tumia jina la nyuga

        ; Unda nodi ya jina kwa nyuga
        mov     r8d, AST_JINA
        mov     r9d, -1
        mov     r10d, -1
        mov     r11d, -1
        call    ast_nodi_mpya
        push    r12
        push    rax
        call    hifadhi_jina
        pop     r10                     ; nodi ya jina
        mov     rdx, [ast_count]
        mov     [ast_jina_off + rdx*4 - 4], eax
        pop     r9                      ; nodi ya kushoto

        ; Unda nodi ya mshale (eneekeza au elekeza)
        mov     r8d, AST_ENEKEZA_FUNGO
        mov     r11d, -1
        call    ast_nodi_mpya
        mov     r12d, eax
        jmp     .loop

.nukta:
        inc     qword [token_pos]       ; tumia .
        mov     rdi, [token_pos]
        mov     rsi, [token_text + rdi*8]
        movzx   ecx, word [token_len + rdi*2]
        inc     qword [token_pos]       ; tumia jina la nyuga

        ; Unda nodi ya jina
        mov     r8d, AST_JINA
        mov     r9d, -1
        mov     r10d, -1
        mov     r11d, -1
        call    ast_nodi_mpya
        push    r12
        push    rax
        call    hifadhi_jina
        pop     r10
        mov     rdx, [ast_count]
        mov     [ast_jina_off + rdx*4 - 4], eax
        pop     r9

        ; Unda nodi ya nukta (elekeza_jina)
        mov     r8d, AST_ELEKEZA_JINA
        mov     r11d, -1
        call    ast_nodi_mpya
        mov     r12d, eax
        jmp     .loop

.wito:
        inc     qword [token_pos]       ; tumia (

        ; Kusanya hoja
        mov     r8d, AST_WAMBILE        ; nodi ya wito
        mov     r9d, -1                 ; kushoto = orodha ya hoja (itajazwa baadaye)
        mov     r10d, r12d              ; kulia = jina la kazi
        mov     r11d, -1
        call    ast_nodi_mpya
        push    rax
        mov     r15d, eax               ; hifadhi nodi ya wito

        ; Changanua hoja
        mov     r14d, -1                ; mwanzo wa orodha ya hoja
        mov     r13d, -1                ; hoja iliyotangulia

        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_MABANO_FUNGA
        je      .wito_mwisho

.wito_hoja_loop:
        call    changanua_usemi
        cmp     eax, -1
        je      .wito_mwisho
        cmp     r14d, -1
        jne     .hoja_ifuatayo
        mov     r14d, eax
        mov     r13d, eax
        jmp     .hoja_angalia_koma
.hoja_ifuatayo:
        mov     [ast_nne + r13*4], eax
        mov     r13d, eax
.hoja_angalia_koma:
        mov     edi, TOK_KOMA
        call    tarajia_ishara
        cmp     eax, 1
        je      .wito_hoja_loop
.wito_mwisho:
        mov     edi, TOK_MABANO_FUNGA
        call    tarajia_ishara

        pop     rax
        mov     rcx, [ast_count]        ; rax = faharisi ya nodi ya wito
        mov     [ast_kushoto + rax*4], r14d ; weka orodha ya hoja
        mov     r12d, eax
        jmp     .loop

.done:
        mov     eax, r12d
        pop     r12
        ret

; -------------------------------------------------------
; changanua_kipengele_kimoja: changanua kipengele kimoja cha usemi
;   rax = nodi ya AST
; -------------------------------------------------------
changanua_kipengele_kimoja:
        ; Angalia viambishi awali: &, *, -, !
        mov     rdi, [token_pos]
        mov     edi, [token_ty + rdi*4]

        cmp     edi, TOK_ALAMA
        je      .anwani_ya
        cmp     edi, TOK_NYOTA
        je      .nyota_ya
        cmp     edi, TOK_ISHARA
        jne     .no_prefix

        mov     rdi, [token_pos]
        cmp     qword [token_val + rdi*8], OP_TOA
        je      .hasili
        cmp     qword [token_val + rdi*8], OP_MAKOSA
        je      .makosa

.no_prefix:
        call    changanua_kipengele_msingi
        cmp     eax, -1
        je      .done
        mov     edi, eax
        call    changanua_kiambishi
        jmp     .done

.anwani_ya:
        inc     qword [token_pos]       ; tumia &
        call    changanua_kipengele_kimoja
        cmp     eax, -1
        je      .done
        mov     r9d, eax
        mov     r8d, AST_ALAMA_ELEKEZA
        mov     r10d, -1
        mov     r11d, -1
        call    ast_nodi_mpya
        jmp     .done

.nyota_ya:
        inc     qword [token_pos]       ; tumia *
        call    changanua_kipengele_kimoja
        cmp     eax, -1
        je      .done
        mov     r9d, eax
        mov     r8d, AST_NYOTA_ELEKEZA
        mov     r10d, -1
        mov     r11d, -1
        call    ast_nodi_mpya
        jmp     .done

.hasili:
        inc     qword [token_pos]       ; tumia -
        call    changanua_kipengele_kimoja
        cmp     eax, -1
        je      .done
        mov     r9d, eax
        mov     r8d, AST_MAKOSA
        mov     r10d, -1
        mov     r11d, -1
        call    ast_nodi_mpya
        jmp     .done

.makosa:
        inc     qword [token_pos]       ; tumia !
        call    changanua_kipengele_kimoja
        cmp     eax, -1
        je      .done
        mov     r9d, eax
        mov     r8d, AST_MAKOSA
        mov     r10d, -1
        mov     r11d, -1
        call    ast_nodi_mpya
.done:
        ret

; -------------------------------------------------------
; utangulizi_wa_ishara: rudisha utangulizi wa ishara
;   edi = msimbo wa ishara
;   rax = utangulizi (1..5) au 0 ikiwa haijulikani
; -------------------------------------------------------
utangulizi_wa_ishara:
        cmp     edi, OP_ZIDISHA
        je      .prec_5
        cmp     edi, OP_GAWANYA
        je      .prec_5
        cmp     edi, OP_MODULO
        je      .prec_5
        cmp     edi, OP_JUMLISHA
        je      .prec_4
        cmp     edi, OP_TOA
        je      .prec_4
        cmp     edi, OP_HAMISHA_KUSHOTO
        je      .prec_3
        cmp     edi, OP_HAMISHA_KULIA
        je      .prec_3
        cmp     edi, OP_KIDOGO
        je      .prec_2
        cmp     edi, OP_KUBWA
        je      .prec_2
        cmp     edi, OP_KIDOGO_SAWA
        je      .prec_2
        cmp     edi, OP_KUBWA_SAWA
        je      .prec_2
        cmp     edi, OP_SAWA
        je      .prec_1
        cmp     edi, OP_SAWA_SAWA
        je      .prec_1
        cmp     edi, OP_SIO_SAWA
        je      .prec_1
        cmp     edi, OP_NA
        je      .prec_0_5
        cmp     edi, OP_AU
        je      .prec_0
        mov     eax, 0
        ret
.prec_5:
        mov     eax, 5
        ret
.prec_4:
        mov     eax, 4
        ret
.prec_3:
        mov     eax, 3
        ret
.prec_2:
        mov     eax, 2
        ret
.prec_1:
        mov     eax, 1
        ret
.prec_0_5:
        mov     eax, 1
        ret
.prec_0:
        xor     eax, eax
        ret

; -------------------------------------------------------
; changanua_usemi_na_utangulizi: precedence climbing
;   edi = utangulizi wa chini
;   rax = nodi ya AST
; -------------------------------------------------------
changanua_usemi_na_utangulizi:
        push    r12
        push    r13
        push    r14
        push    rbx
        push    rdi                     ; hifadhi utangulizi wa chini (min_prec) kwenye stack

        call    changanua_kipengele_kimoja
        cmp     eax, -1
        je      .fail
        mov     r12d, eax               ; nodi ya kushoto

.loop:
        ; Angalia ikiwa tokeni ya sasa ni ishara
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_ISHARA
        je      .check_prec
        cmp     dword [token_ty + rdi*4], TOK_NYOTA
        jne     .check_sawa

        ; * inaweza kuwa ishara ya zidisha
        mov     ebx, OP_ZIDISHA
        jmp     .got_op

.check_sawa:
        cmp     dword [token_ty + rdi*4], TOK_SAWA
        jne     .done
        mov     ebx, OP_SAWA
        jmp     .got_op

.check_prec:
        mov     rbx, [token_val + rdi*8]
.got_op:
        mov     edi, ebx
        call    utangulizi_wa_ishara
        cmp     eax, 0
        je      .done
        mov     r13d, eax               ; utangulizi wa sasa
        mov     r14d, [rsp]             ; utangulizi wa chini (kutoka stack)
        cmp     r13d, r14d
        jl      .done

        ; Tumia ishara
        inc     qword [token_pos]

        push    rbx                     ; hifadhi ishara
        push    r12                     ; hifadhi kushoto

        ; Changanua upande wa kulia kwa utangulizi wa juu
        mov     edi, r13d
        inc     edi
        call    changanua_usemi_na_utangulizi
        mov     r10d, eax               ; kulia
        pop     r9                      ; kushoto
        pop     r8                      ; ishara (tunahifadhi kama thamani)

        ; Unda nodi ya hesabu au ulinganisho
        push    r10
        push    r9
        push    r8
        mov     r8d, AST_KAULI          ; operesheni binary
        pop     r11                     ; thamani = ishara
        pop     r9                      ; kushoto
        pop     r10                     ; kulia
        push    r11
        call    ast_nodi_mpya
        pop     r11
        mov     rcx, [ast_count]
        mov     [ast_thamani + rcx*4 - 4], r11d
        mov     r12d, eax
        jmp     .loop

.done:
        mov     eax, r12d
        pop     rdi
        pop     rbx
        pop     r14
        pop     r13
        pop     r12
        ret
.fail:
        mov     eax, -1
        pop     rdi
        pop     rbx
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; changanua_usemi: changanua usemi kamili
;   rax = nodi ya AST
; -------------------------------------------------------
changanua_usemi:
        mov     edi, 0                  ; utangulizi wa chini kabisa
        call    changanua_usemi_na_utangulizi
        ret

; -------------------------------------------------------
; changanua_taarifa: changanua taarifa moja
;   rax = nodi ya AST, -1 ikiwa hakuna taarifa zaidi
; -------------------------------------------------------
changanua_taarifa:
        push    r12
        push    r13
        push    r14
        push    r15

        mov     rdi, [token_pos]
        mov     edi, [token_ty + rdi*4]

        ; Angalia aina ya taarifa
        cmp     edi, TOK_MWISHO
        je      .fail
        cmp     edi, TOK_FUNGA
        je      .fail

        ; { block } — taarifa ya mchanganyiko
        cmp     edi, TOK_FUNGO
        jne     .not_block
        inc     qword [token_pos]       ; tumia {
        call    changanua_block
        jmp     .done

.not_block:
        ; rudisha ...
        cmp     edi, TOK_NENO
        jne     .try_semicolon

        ; Angalia ikiwa ni "rudisha"
        mov     rdi, [token_pos]
        mov     rsi, [token_text + rdi*8]
        movzx   ecx, word [token_len + rdi*2]
        lea     rdx, [kw_rudisha]
        call    linganisha_neno_muhimu
        cmp     eax, 0
        jne     .not_return

        inc     qword [token_pos]       ; tumia "rudisha"

        ; Angalia ikiwa ni rudisha tupu (ikifuatiwa na ;)
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NUKTA_MKATO
        je      .return_void

        call    changanua_usemi
        mov     r9d, eax
        mov     r8d, AST_RUDISHA
        mov     r10d, -1
        mov     r11d, -1
        call    ast_nodi_mpya
        jmp     .expect_semicolon

.return_void:
        mov     r8d, AST_RUDISHA
        mov     r9d, -1
        mov     r10d, -1
        mov     r11d, -1
        call    ast_nodi_mpya
        jmp     .expect_semicolon

.not_return:
        ; Angalia ikiwa ni "kama"
        lea     rdx, [kw_kama]
        call    linganisha_neno_muhimu
        cmp     eax, 0
        jne     .not_if

        inc     qword [token_pos]       ; tumia "kama"
        mov     edi, TOK_MABANO_FUNGO
        call    tarajia_ishara
        call    changanua_usemi
        mov     r12d, eax
        mov     edi, TOK_MABANO_FUNGA
        call    tarajia_ishara
        call    changanua_block
        mov     r13d, eax

        ; Angalia sivyo
        mov     r14d, -1
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NENO
        jne     .no_else
        mov     rdi, [token_pos]
        mov     rsi, [token_text + rdi*8]
        movzx   ecx, word [token_len + rdi*2]
        lea     rdx, [kw_sivyo]
        call    linganisha_neno_muhimu
        cmp     eax, 0
        jne     .no_else
        inc     qword [token_pos]       ; tumia "sivyo"
        call    changanua_block
        mov     r14d, eax
.no_else:
        ; Unda nodi ya kama: kushoto=hali, kulia=mwili, tiga=sivyo
        mov     r8d, AST_KAMA
        mov     r9d, r12d
        mov     r10d, r13d
        mov     r11d, r14d
        call    ast_nodi_mpya
        jmp     .done

.not_if:
        ; Angalia ikiwa ni "wakati"
        lea     rdx, [kw_wakati]
        call    linganisha_neno_muhimu
        cmp     eax, 0
        jne     .not_while

        inc     qword [token_pos]       ; tumia "wakati"
        mov     edi, TOK_MABANO_FUNGO
        call    tarajia_ishara
        call    changanua_usemi
        mov     r12d, eax
        mov     edi, TOK_MABANO_FUNGA
        call    tarajia_ishara
        call    changanua_block
        mov     r13d, eax
        mov     r8d, AST_WAKATI
        mov     r9d, r12d
        mov     r10d, r13d
        mov     r11d, -1
        call    ast_nodi_mpya
        jmp     .done

.not_while:
        ; Angalia ikiwa ni "kwa"
        lea     rdx, [kw_kwa]
        call    linganisha_neno_muhimu
        cmp     eax, 0
        jne     .not_for

        inc     qword [token_pos]       ; tumia "kwa"
        mov     edi, TOK_MABANO_FUNGO
        call    tarajia_ishara
        ; TODO: changanua kwa kikamilifu
        ; Kwa sasa, ruka hadi )
        mov     r12d, 1
.kwa_skip:
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_MABANO_FUNGA
        je      .kwa_skip_done
        cmp     dword [token_ty + rdi*4], TOK_MWISHO
        je      .kwa_skip_done
        inc     qword [token_pos]
        jmp     .kwa_skip
.kwa_skip_done:
        inc     qword [token_pos]       ; tumia )
        call    changanua_block
        mov     r13d, eax
        mov     r8d, AST_KWA
        mov     r9d, -1
        mov     r10d, r13d
        mov     r11d, -1
        call    ast_nodi_mpya
        jmp     .done

.not_for:
        ; Angalia ikiwa ni "vunja"
        lea     rdx, [kw_vunja]
        call    linganisha_neno_muhimu
        cmp     eax, 0
        jne     .not_break

        inc     qword [token_pos]       ; tumia "vunja"
        mov     r8d, AST_VUNJA
        mov     r9d, -1
        mov     r10d, -1
        mov     r11d, -1
        call    ast_nodi_mpya
        jmp     .expect_semicolon

.not_break:
        ; Labda ni tangazo la kigezo (N32 jina = ...) au usemi (wito wa kazi, n.k.)
        ; Jaribu kuchanganua kama tangazo la aina kwanza
        call    changanua_aina
        cmp     eax, 0
        je      .not_decl

        ; Tulipata aina! eax = nambari ya aina, ebx = nyota
        mov     r12d, eax               ; hifadhi aina
        mov     r13d, ebx               ; hifadhi nyota

        ; Soma jina la kigezo
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NENO
        jne     .decl_error
        mov     rsi, [token_text + rdi*8]
        movzx   ecx, word [token_len + rdi*2]
        inc     qword [token_pos]       ; tumia jina

        ; Hifadhi jina kwenye bwawa
        push    r13                     ; nyota
        push    r12                     ; aina
        push    rcx
        push    rsi
        pop     rsi
        pop     rcx
        call    hifadhi_jina
        mov     r14d, eax               ; ofseti ya jina
        pop     r12                     ; aina
        pop     r13                     ; nyota

        ; Angalia ikiwa kuna sawa (=) kwa thamani ya awali
        mov     r15d, -1                ; chaguo-msingi: hakuna kianzilishi
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_SAWA
        jne     .no_init

        inc     qword [token_pos]       ; tumia =
        push    r14                     ; ofseti ya jina
        push    r13                     ; nyota
        push    r12                     ; aina
        call    changanua_usemi
        mov     r15d, eax               ; nodi ya kianzilishi
        pop     r12                     ; aina
        pop     r13                     ; nyota
        pop     r14                     ; ofseti ya jina
.no_init:
        ; Unda nodi ya AST_TANGAZO
        ; kushoto = kianzilishi, kulia = nyota, thamani = aina, jina_off = jina
        mov     r8d, AST_TANGAZO
        mov     r9d, r15d               ; kushoto = kianzilishi
        mov     r10d, r13d              ; kulia = nyota
        mov     r11d, -1                ; tiga haitumiki
        call    ast_nodi_mpya
        push    r12                     ; aina
        mov     rcx, [ast_count]
        mov     [ast_thamani + rcx*4 - 4], r12d  ; aina
        mov     [ast_jina_off + rcx*4 - 4], r14d ; jina
        pop     r12
        jmp     .expect_semicolon

.decl_error:
        ; Haiwezi kuwa tangazo sahihi — endelea
        mov     eax, -1
        jmp     .fail

.not_decl:
        ; Sio aina — changanua kama usemi
        call    changanua_usemi
        cmp     eax, -1
        je      .fail
        jmp     .expect_semicolon

.try_semicolon:
        cmp     edi, TOK_NUKTA_MKATO
        jne     .fail
        inc     qword [token_pos]
        mov     eax, -1                 ; taarifa tupu
        jmp     .done

.expect_semicolon:
        push    rax
        mov     edi, TOK_NUKTA_MKATO
        call    tarajia_ishara
        pop     rax
        jmp     .done

.fail:
        mov     eax, -1
.done:
        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; changanua_block: changanua block ya taarifa { ... }
;   rax = nodi ya AST_BLOCK
; -------------------------------------------------------
changanua_block:
        push    r12
        push    r13
        push    r14

        mov     edi, TOK_FUNGO
        call    tarajia_ishara
        cmp     eax, 1
        jne     .single_statement

        ; Unda nodi ya block
        mov     r8d, AST_BLOCK
        mov     r9d, -1
        mov     r10d, -1
        mov     r11d, -1
        call    ast_nodi_mpya
        mov     r12d, eax
        mov     r13d, -1                ; taarifa iliyotangulia

.block_loop:
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_FUNGA
        je      .block_done
        cmp     dword [token_ty + rdi*4], TOK_MWISHO
        je      .block_done

        call    changanua_taarifa

        cmp     eax, -1
        je      .block_done

        cmp     r13d, -1
        jne     .append_statement
        mov     [ast_kushoto + r12*4], eax
        mov     r13d, eax
        jmp     .block_loop
.append_statement:
        mov     [ast_nne + r13*4], eax
        mov     r13d, eax
        jmp     .block_loop

.block_done:
        mov     edi, TOK_FUNGA
        call    tarajia_ishara
        mov     eax, r12d
        jmp     .done

.single_statement:
        call    changanua_taarifa
.done:
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; changanua_vigezo: changanua orodha ya vigezo vya kazi
;   rax = nodi ya kwanza ya param au -1
; -------------------------------------------------------
changanua_vigezo:
        push    r12
        push    r13
        push    r14
        push    r15

        ; Angalia ikiwa mabano ni tupu
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_MABANO_FUNGA
        je      .empty

        mov     r12d, -1                ; param ya kwanza
        mov     r13d, -1                ; param iliyotangulia

.param_loop:
        ; Changanua aina
        call    changanua_aina
        cmp     eax, 0
        je      .skip_unknown_param

        ; r15d = aina (msingi)
        ; rbx = nyota
        mov     r15d, eax
        mov     r14d, ebx               ; hifadhi nyota

        ; Changanua jina la kigezo
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NENO
        jne     .done
        mov     rsi, [token_text + rdi*8]
        movzx   ecx, word [token_len + rdi*2]
        inc     qword [token_pos]

        ; Unda nodi ya param
        mov     r8d, AST_PARAM
        mov     r9d, -1
        mov     r10d, -1
        mov     r11d, -1
        call    ast_nodi_mpya
        ; Hifadhi faharisi ya nodi kabla ya hifadhi_jina kuiharibu
        push    rax                     ; hifadhi faharisi ya nodi
        push    r15
        push    r14
        call    hifadhi_jina
        mov     rcx, [ast_count]
        mov     [ast_jina_off + rcx*4 - 4], eax ; jina (ofseti ya bwawa)
        pop     r14
        pop     r15
        mov     [ast_thamani + rcx*4 - 4], r15d ; aina
        ; Weka nyota kwenye tiga
        mov     [ast_tiga + rcx*4 - 4], r14d
        pop     rax                     ; rejesha faharisi ya nodi

        cmp     r12d, -1
        jne     .append_param
        mov     r12d, eax
        mov     r13d, eax
        jmp     .param_check_comma
.append_param:
        mov     [ast_nne + r13*4], eax
        mov     r13d, eax
        jmp     .param_check_comma

.skip_unknown_param:
        ; Aina haijulikani (k.m. jina la muundo) - ruka kigezo hiki
        ; token_pos bado iko kwenye jina la aina
        inc     qword [token_pos]       ; ruka jina la aina
        ; Angalia kama kuna nyota
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NYOTA
        jne     .skip_param_name
        inc     qword [token_pos]       ; ruka nyota
.skip_param_name:
        ; Ruka jina la kigezo kama lipo
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NENO
        jne     .param_check_comma
        inc     qword [token_pos]       ; ruka jina la kigezo
        ; Angalia koma au mabano ya kufunga

.param_check_comma:
        mov     edi, TOK_KOMA
        call    tarajia_ishara
        cmp     eax, 1
        je      .param_loop
        jmp     .done

.empty:
        mov     eax, -1
        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret
.done:
        mov     eax, r12d
        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; changanua_kazi: changanua kazi nzima
;   rax = nodi ya AST_KAZI, -1 ikiwa haikuweza
; -------------------------------------------------------
changanua_kazi:
        push    r12
        push    r13
        push    r14
        push    r15

        ; Changanua aina ya kurudi
        call    changanua_aina
        cmp     eax, 0
        je      .fail_no_type
        mov     r12d, eax               ; aina ya kurudi
        ; rbx ina nyota

        ; Changanua jina la kazi
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NENO
        jne     .fail_name
        mov     rsi, [token_text + rdi*8]
        movzx   ecx, word [token_len + rdi*2]
        inc     qword [token_pos]

        ; Hifadhi jina la kazi
        push    r12
        push    rbx
        call    hifadhi_jina
        mov     r15d, eax               ; hifadhi ofseti ya jina
        pop     rbx
        pop     r12

        ; Tarajia mabano ya kufungua
        mov     edi, TOK_MABANO_FUNGO
        call    tarajia_ishara
        cmp     eax, 1
        jne     .fail_paren

        ; Changanua vigezo
        call    changanua_vigezo
        mov     r13d, eax               ; orodha ya vigezo

        ; Tarajia mabano ya kufunga
        mov     edi, TOK_MABANO_FUNGA
        call    tarajia_ishara
        cmp     eax, 1
        jne     .fail_close_paren

        ; Angalia ikiwa ni tangazo la mbele (;) au mwili ({)
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NUKTA_MKATO
        je      .tangazo_mbele

        ; Changanua mwili (block)
        call    changanua_block
        mov     r14d, eax               ; mwili

        ; Unda nodi ya kazi
        mov     r8d, AST_KAZI
        mov     r9d, r13d               ; vigezo
        mov     r10d, r14d              ; mwili
        mov     r11d, r15d              ; tiga = ofseti ya jina? hapana
        ; Hapa tunahitaji kuweka jina na aina kwenye nodi
        ; Tutaweka thamani = aina, jina_off = ofseti ya jina
        mov     r11d, -1
        call    ast_nodi_mpya
        mov     rcx, [ast_count]
        mov     [ast_thamani + rcx*4 - 4], r12d  ; aina
        mov     [ast_jina_off + rcx*4 - 4], r15d ; jina
        jmp     .done

.tangazo_mbele:
        inc     qword [token_pos]       ; tumia ;
        ; Unda nodi ya kazi isiyo na mwili
        mov     r8d, AST_KAZI
        mov     r9d, r13d
        mov     r10d, -1                ; mwili = -1
        mov     r11d, -1
        call    ast_nodi_mpya
        mov     rcx, [ast_count]
        mov     [ast_thamani + rcx*4 - 4], r12d
        mov     [ast_jina_off + rcx*4 - 4], r15d
        jmp     .done

.fail_no_type:
        mov     qword [compiler_state], 2
        jmp     .fail_common
.fail_name:
        mov     qword [compiler_state], 3
        jmp     .fail_common
.fail_paren:
        mov     qword [compiler_state], 4
        jmp     .fail_common
.fail_close_paren:
        mov     qword [compiler_state], 5
        jmp     .fail_common
.fail_common:
        mov     eax, -1
.done:
        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; changanua_programu: changanua programu nzima
;   rax = nodi ya mzizi (orodha ya matamko)
; -------------------------------------------------------
changanua_programu:
        push    r12
        push    r13

        ; Unda nodi ya mzizi (orodha)
        mov     r8d, -1                 ; aina maalum kwa mzizi
        mov     r9d, -1
        mov     r10d, -1
        mov     r11d, -1
        call    ast_nodi_mpya
        mov     r12d, eax               ; mzizi
        mov     r13d, -1                ; tamko lililotangulia

.programu_loop:
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_MWISHO
        je      .done

        ; Jaribu changanua kazi au tangazo la nje
        ; Angalia ikiwa ni aina
        ; Kwanza, ikiwa sio neno, ruka
        cmp     dword [token_ty + rdi*4], TOK_NENO
        jne     .skip_token

        ; Angalia ikiwa ni husisha
        mov     rsi, [token_text + rdi*8]
        movzx   ecx, word [token_len + rdi*2]
        lea     rdx, [kw_husisha]
        push    rdi
        call    linganisha_neno_muhimu
        pop     rdi
        cmp     eax, 0
        jne     .try_function

        ; Husisha - ruka hadi kwenye mabano ya kufunga }
        ; Syntax: husisha { faili.swa }
        inc     qword [token_pos]       ; ruka neno 'husisha', sasa tuko kwenye '{'
        mov     ecx, 1                   ; kina cha mabano: { = +1, } = -1
.husisha_skip:
        inc     qword [token_pos]
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_FUNGO
        je      .husisha_open
        cmp     dword [token_ty + rdi*4], TOK_FUNGA
        je      .husisha_close
        cmp     dword [token_ty + rdi*4], TOK_MWISHO
        je      .programu_loop           ; mwisho wa faili, acha
        jmp     .husisha_skip
.husisha_open:
        inc     ecx
        jmp     .husisha_skip
.husisha_close:
        dec     ecx
        jnz     .husisha_skip             ; sio mabano ya kufunga ya nje
        ; kuanguka hadi .husisha_done - tulipata } inayofunga
.husisha_done:
        inc     qword [token_pos]         ; ruka }
        jmp     .programu_loop

.try_function:
        ; DEBUG: weka alama kwamba tulifika hapa
        mov     qword [compiler_state], 10
        ; Jaribu changanua kazi
        call    changanua_kazi
        mov     qword [compiler_state], 11
        cmp     eax, -1
        je      .skip_token

        ; Ongeza kwenye orodha ya mzizi
        cmp     r13d, -1
        jne     .append_decl
        mov     [ast_kushoto + r12*4], eax
        mov     r13d, eax
        mov     qword [compiler_state], 12
        jmp     .programu_loop
.append_decl:
        mov     [ast_nne + r13*4], eax
        mov     r13d, eax
        mov     qword [compiler_state], 13
        jmp     .programu_loop

.skip_token:
        inc     qword [token_pos]
        jmp     .programu_loop

.done:
        mov     eax, r12d
        pop     r13
        pop     r12
        ret

; =============================================================================
; Sehemu ya 6: Mzalishaji Msimbo (Code Generator)
; =============================================================================

; -------------------------------------------------------
; uzalishaji: linda la kazi ya uzalishaji
;   r12d = nodi ya sasa ya AST
; -------------------------------------------------------

; Hali ya uzalishaji
        section .bss
gen_label_count:        resq 1
gen_fixup_offset:       resd 512
gen_fixup_label:        resd 512
gen_fixup_count:        resq 1
gen_stack_size:         resq 1
gen_current_func:       resq 1
gen_return_label:       resq 1
gen_label_pos:          resd 128

        section .text

; -------------------------------------------------------
; gen_label_mpya: tengeneza lebo mpya ya kipekee
;   rax = nambari ya lebo
; -------------------------------------------------------
gen_label_mpya:
        mov     rax, [gen_label_count]
        inc     qword [gen_label_count]
        ret

; -------------------------------------------------------
; gen_andika_baiti_text: ongeza baiti kwenye bafa la .text
;   al = baiti
; -------------------------------------------------------
gen_baiti:
        push    rdi
        mov     rdi, [text_buf_pos]
        cmp     rdi, TEXT_BUF_SIZE - 16
        jae     .overflow
        lea     rdi, [text_buf + rdi]
        mov     [rdi], al
        inc     qword [text_buf_pos]
        pop     rdi
        ret
.overflow:
        pop     rdi
        ret

; -------------------------------------------------------
; gen_neno4: ongeza baiti 4 kwenye bafa la .text
;   edi = thamani
; -------------------------------------------------------
gen_neno4:
        push    rdi
        push    rcx
        mov     rcx, [text_buf_pos]
        cmp     rcx, TEXT_BUF_SIZE - 16
        jae     .overflow
        lea     rcx, [text_buf + rcx]
        mov     [rcx], edi
        add     qword [text_buf_pos], 4
        pop     rcx
        pop     rdi
        ret
.overflow:
        pop     rcx
        pop     rdi
        ret

; -------------------------------------------------------
; gen_neno8: ongeza baiti 8 kwenye bafa la .text
;   rdi = thamani
; -------------------------------------------------------
gen_neno8:
        push    rdi
        push    rcx
        mov     rcx, [text_buf_pos]
        cmp     rcx, TEXT_BUF_SIZE - 16
        jae     .overflow
        lea     rcx, [text_buf + rcx]
        mov     [rcx], rdi
        add     qword [text_buf_pos], 8
        pop     rcx
        pop     rdi
        ret
.overflow:
        pop     rcx
        pop     rdi
        ret

; -------------------------------------------------------
; gen_fixup_ongeza: ongeza fixup kwa kuruka mbele
;   edi = ofseti ya sasa kwenye bafa la .text (ya baiti 4 zilizowekwa)
;   esi = lebo ya kulenga
; -------------------------------------------------------
gen_fixup_ongeza:
        push    rbx
        mov     rbx, [gen_fixup_count]
        cmp     rbx, 512
        jae     .overflow
        mov     [gen_fixup_offset + rbx*4], edi
        mov     [gen_fixup_label + rbx*4], esi
        inc     qword [gen_fixup_count]
        pop     rbx
        ret
.overflow:
        pop     rbx
        ret

; -------------------------------------------------------
; gen_weka_lebo: weka lebo kwenye nafasi ya sasa ya bafa la .text
;   edi = nambari ya lebo
; -------------------------------------------------------
gen_weka_lebo:
        ; Hifadhi nafasi ya lebo: edi = nafasi ya text_buf, esi = nambari ya lebo
        cmp     esi, 128
        jae     .overflow
        mov     [gen_label_pos + rsi*4], edi
.overflow:
        ret

; -------------------------------------------------------
; uzalishaji_tangazo: zalisha msimbo kwa nodi ya tangazo la kigezo
;   r12d = faharisi ya nodi
;   matokeo yanawekwa kwenye eax (thamani ya kianzilishi)
; -------------------------------------------------------
uzalishaji_tangazo:
        push    r12
        push    r13
        push    r14

        mov     r12d, r12d
        mov     r13d, [ast_kushoto + r12*4]  ; nodi ya kianzilishi
        mov     r14d, [ast_jina_off + r12*4] ; ofseti ya jina

        ; Zalisha thamani ya kianzilishi (ikiwa ipo)
        cmp     r13d, -1
        je      .no_init
        push    r12
        mov     r12d, r13d
        call    uzalishaji_ast
        pop     r12
        jmp     .store_local
.no_init:
        xor     eax, eax                ; hakuna kianzilishi, thamani = 0
.store_local:
        ; Sajili kigezo cha ndani
        mov     rcx, [local_count]
        ; Hifadhi jina
        lea     rdi, [str_pool + r14]
        mov     [local_name + rcx*8], rdi
        ; Kokotoa ofseti ya rafu: (local_count + 1) * 4
        mov     r8d, ecx
        inc     r8d
        imul    r8d, 4
        mov     [local_offset + rcx*4], r8d
        inc     qword [local_count]

        ; Toa maelekezo: mov [rbp - ofseti], eax
        ; mov [rbp + disp8], eax  →  89 45 XX
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x45
        call    gen_baiti
        neg     r8d                     ; hasi
        mov     al, r8b
        call    gen_baiti

        ; Rudisha thamani ya kianzilishi (bado iko kwenye eax)
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; uzalishaji_nambari: zalisha msimbo kwa nodi ya nambari
;   r12d = faharisi ya nodi
;   matokeo yanawekwa kwenye eax (32-bit) au rax (64-bit)
; -------------------------------------------------------
uzalishaji_nambari:
        push    r12
        mov     r12d, r12d              ; hakikisha ni 32-bit
        mov     edi, [ast_thamani + r12*4]
        ; Toa maelekezo: mov eax, imm32
        mov     al, 0xb8                ; opcode ya "mov eax, imm32"
        call    gen_baiti
        call    gen_neno4               ; edi = thamani ya haraka
        mov     eax, edi                ; rudisha thamani
        pop     r12
        ret

; -------------------------------------------------------
; uzalishaji_jina: zalisha msimbo kwa nodi ya jina (rejeleo ya kigezo)
;   r12d = faharisi ya nodi
;   matokeo kwenye eax
; -------------------------------------------------------
uzalishaji_jina:
        push    r12
        push    r13
        push    r14

        mov     r12d, r12d

        ; Tafuta kigezo kwenye orodha ya vigezo vya ndani
        mov     r14d, [ast_jina_off + r12*4] ; ofseti ya jina
        lea     r14, [str_pool + r14]        ; anwani ya jina

        xor     r13d, r13d
.search_loop:
        cmp     r13, [local_count]
        jae     .not_found
        mov     rdi, [local_name + r13*8]
        mov     rsi, r14
        call    linganisha_mfuatano
        cmp     eax, 0
        je      .found
        inc     r13
        jmp     .search_loop

.found:
        ; Kigezo kiko kwenye rafu
        ; Toa maelekezo: mov eax, [rbp - ofseti]  ->  8B 45 XX
        mov     edi, [local_offset + r13*4]
        neg     edi
        ; Toa maelekezo ya kusoma kutoka rafu kwa wakati wa utekelezaji
        mov     al, 0x8B                ; opcode ya "mov eax, r/m32"
        call    gen_baiti
        mov     al, 0x45                ; ModRM: mod=01, reg=000(eax), r/m=101(rbp+disp8)
        call    gen_baiti
        mov     al, dil                 ; disp8 (hasi, mfano -4 = 0xFC)
        call    gen_baiti
        ; Thamani itasomwa wakati wa utekelezaji; rudisha 0 kwa sasa
        xor     eax, eax
        pop     r14
        pop     r13
        pop     r12
        ret

.not_found:
        ; Labda ni kazi ya nje — tunarudisha 0 kwa sasa
        xor     eax, eax
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; uzalishaji_rudisha: zalisha msimbo kwa nodi ya rudisha
; -------------------------------------------------------
uzalishaji_rudisha:
        push    r12
        push    r13

        mov     r12d, r12d
        mov     r13d, [ast_kushoto + r12*4]  ; usemi wa kurudisha

        cmp     r13d, -1
        je      .return_void

        ; Zalisha usemi
        push    r12
        push    r13
        mov     r12d, r13d
        call    uzalishaji_ast
        pop     r13
        pop     r12
        ; Thamani iko kwenye eax

        ; Ruka hadi mwisho wa kazi
        mov     al, 0xe9                ; jmp rel32
        call    gen_baiti
        mov     edi, [text_buf_pos]
        call    gen_neno4               ; nafasi ya kujazwa
        mov     rdi, [gen_return_label]
        mov     esi, edi                ; lebo
        mov     edi, [text_buf_pos]
        sub     edi, 4
        call    gen_fixup_ongeza
        jmp     .done

.return_void:
        ; Ruka hadi mwisho wa kazi
        mov     al, 0xe9                ; jmp rel32
        call    gen_baiti
        mov     edi, [text_buf_pos]
        call    gen_neno4
        mov     rdi, [gen_return_label]
        mov     esi, edi
        mov     edi, [text_buf_pos]
        sub     edi, 4
        call    gen_fixup_ongeza

.done:
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; uzalishaji_kauli_ya_binary: zalisha msimbo kwa operesheni binary
;   r12d = faharisi ya nodi
; -------------------------------------------------------
uzalishaji_kauli_ya_binary:
        push    r12
        push    r13
        push    r14
        push    r15

        mov     r12d, r12d
        mov     r13d, [ast_kushoto + r12*4]  ; kushoto
        mov     r14d, [ast_kulia + r12*4]    ; kulia
        mov     r15d, [ast_thamani + r12*4]   ; ishara

        ; Angalia ikiwa ni assignment (=) — inahitaji utoaji maalum
        cmp     r15d, OP_SAWA
        je      .do_assign

        ; Zalisha upande wa kulia kwanza
        push    r12
        push    r15
        mov     r12d, r14d
        call    uzalishaji_ast
        pop     r15
        pop     r12
        mov     ecx, eax                ; hifadhi kwa wakati wa kukusanya
        push    rcx                     ; hifadhi thamani ya wakati wa kukusanya
        ; Toa push rax — hifadhi thamani ya wakati wa utekelezaji
        mov     al, 0x50
        call    gen_baiti

        ; Zalisha upande wa kushoto
        push    r12
        push    r15
        mov     r12d, r13d
        call    uzalishaji_ast
        pop     r15
        pop     r12
        pop     rcx                     ; rejesha thamani ya kulia (wakati wa kukusanya)
        ; Toa pop rcx — rejesha thamani ya kulia (wakati wa utekelezaji)
        mov     al, 0x59
        call    gen_baiti

        ; eax = kushoto (wakati wa kukusanya), ecx = kulia (wakati wa kukusanya)
        ; Fanya operesheni kwa wakati wa kukusanya NA utoe maelekezo
        cmp     r15d, OP_JUMLISHA
        je      .do_add
        cmp     r15d, OP_TOA
        je      .do_sub
        cmp     r15d, OP_ZIDISHA
        je      .do_mul
        cmp     r15d, OP_GAWANYA
        je      .do_div
        cmp     r15d, OP_MODULO
        je      .do_mod
        cmp     r15d, OP_SAWA_SAWA
        je      .do_eq
        cmp     r15d, OP_SIO_SAWA
        je      .do_ne
        cmp     r15d, OP_KIDOGO
        je      .do_lt
        cmp     r15d, OP_KUBWA
        je      .do_gt
        cmp     r15d, OP_KIDOGO_SAWA
        je      .do_le
        cmp     r15d, OP_KUBWA_SAWA
        je      .do_ge
        ; chaguo-msingi
        jmp     .done

.do_add:
        add     eax, ecx                ; wakati wa kukusanya
        ; add eax, ecx → 01 c8
        mov     al, 0x01
        call    gen_baiti
        mov     al, 0xc8
        call    gen_baiti
        jmp     .done
.do_sub:
        sub     eax, ecx                ; wakati wa kukusanya
        ; sub eax, ecx → 29 c8
        mov     al, 0x29
        call    gen_baiti
        mov     al, 0xc8
        call    gen_baiti
        jmp     .done
.do_mul:
        imul    eax, ecx                ; wakati wa kukusanya
        ; imul eax, ecx → 0f af c1
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0xaf
        call    gen_baiti
        mov     al, 0xc1
        call    gen_baiti
        jmp     .done
.do_div:
        ; Epuka idiv kwa wakati wa kukusanya ikiwa kigawanyo ni sifuri
        cmp     ecx, 0
        je      .div_skip_ct
        cdq                             ; wakati wa kukusanya
        idiv    ecx
.div_skip_ct:
        ; cdq → 99
        mov     al, 0x99
        call    gen_baiti
        ; idiv ecx → f7 f9
        mov     al, 0xf7
        call    gen_baiti
        mov     al, 0xf9
        call    gen_baiti
        jmp     .done
.do_mod:
        ; Epuka idiv kwa wakati wa kukusanya ikiwa kigawanyo ni sifuri
        cmp     ecx, 0
        je      .mod_skip_ct
        cdq                             ; wakati wa kukusanya
        idiv    ecx
        mov     eax, edx
.mod_skip_ct:
        ; cdq → 99
        mov     al, 0x99
        call    gen_baiti
        ; idiv ecx → f7 f9
        mov     al, 0xf7
        call    gen_baiti
        mov     al, 0xf9
        call    gen_baiti
        ; mov eax, edx → 89 d0
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0xd0
        call    gen_baiti
        jmp     .done
.do_eq:
        cmp     eax, ecx                ; wakati wa kukusanya
        sete    al
        movzx   eax, al
        ; cmp eax, ecx → 39 c8
        mov     al, 0x39
        call    gen_baiti
        mov     al, 0xc8
        call    gen_baiti
        ; sete al → 0f 94 c0
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0x94
        call    gen_baiti
        mov     al, 0xc0
        call    gen_baiti
        ; movzx eax, al → 0f b6 c0
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0xb6
        call    gen_baiti
        mov     al, 0xc0
        call    gen_baiti
        jmp     .done
.do_ne:
        cmp     eax, ecx                ; wakati wa kukusanya
        setne   al
        movzx   eax, al
        ; cmp eax, ecx → 39 c8
        mov     al, 0x39
        call    gen_baiti
        mov     al, 0xc8
        call    gen_baiti
        ; setne al → 0f 95 c0
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0x95
        call    gen_baiti
        mov     al, 0xc0
        call    gen_baiti
        ; movzx eax, al → 0f b6 c0
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0xb6
        call    gen_baiti
        mov     al, 0xc0
        call    gen_baiti
        jmp     .done
.do_lt:
        cmp     eax, ecx                ; wakati wa kukusanya
        setl    al
        movzx   eax, al
        ; cmp eax, ecx → 39 c8
        mov     al, 0x39
        call    gen_baiti
        mov     al, 0xc8
        call    gen_baiti
        ; setl al → 0f 9c c0
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0x9c
        call    gen_baiti
        mov     al, 0xc0
        call    gen_baiti
        ; movzx eax, al → 0f b6 c0
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0xb6
        call    gen_baiti
        mov     al, 0xc0
        call    gen_baiti
        jmp     .done
.do_gt:
        cmp     eax, ecx                ; wakati wa kukusanya
        setg    al
        movzx   eax, al
        ; cmp eax, ecx → 39 c8
        mov     al, 0x39
        call    gen_baiti
        mov     al, 0xc8
        call    gen_baiti
        ; setg al → 0f 9f c0
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0x9f
        call    gen_baiti
        mov     al, 0xc0
        call    gen_baiti
        ; movzx eax, al → 0f b6 c0
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0xb6
        call    gen_baiti
        mov     al, 0xc0
        call    gen_baiti
        jmp     .done
.do_le:
        cmp     eax, ecx                ; wakati wa kukusanya
        setle   al
        movzx   eax, al
        ; cmp eax, ecx → 39 c8
        mov     al, 0x39
        call    gen_baiti
        mov     al, 0xc8
        call    gen_baiti
        ; setle al → 0f 9e c0
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0x9e
        call    gen_baiti
        mov     al, 0xc0
        call    gen_baiti
        ; movzx eax, al → 0f b6 c0
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0xb6
        call    gen_baiti
        mov     al, 0xc0
        call    gen_baiti
        jmp     .done
.do_ge:
        cmp     eax, ecx                ; wakati wa kukusanya
        setge   al
        movzx   eax, al
        ; cmp eax, ecx → 39 c8
        mov     al, 0x39
        call    gen_baiti
        mov     al, 0xc8
        call    gen_baiti
        ; setge al → 0f 9d c0
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0x9d
        call    gen_baiti
        mov     al, 0xc0
        call    gen_baiti
        ; movzx eax, al → 0f b6 c0
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0xb6
        call    gen_baiti
        mov     al, 0xc0
        call    gen_baiti
        jmp     .done
.do_assign:
        ; Assignment: tathmini kulia, hifadhi kwa upande wa kushoto
        push    r12
        push    r15
        mov     r12d, r14d              ; nodi ya usemi wa kulia
        call    uzalishaji_ast          ; toa msimbo wa kutathmini usemi
        pop     r15
        pop     r12
        mov     r8d, eax                ; hifadhi thamani ya wakati wa kukusanya

        ; Upande wa kushoto lazima uwe jina la kigezo (AST_JINA)
        mov     ebx, [ast_aina + r13*4]
        cmp     ebx, AST_JINA
        jne     .assign_err

        ; Pata anwani ya jina la kigezo
        mov     r9d, [ast_jina_off + r13*4]
        lea     r9, [str_pool + r9]

        ; Tafuta kigezo kwenye orodha ya vigezo vya ndani
        xor     r10d, r10d
.as_search:
        cmp     r10, [local_count]
        jae     .assign_err
        mov     rdi, [local_name + r10*8]
        mov     rsi, r9
        push    r8
        push    r9
        push    r10
        push    r11
        push    r12
        push    r13
        push    r14
        push    r15
        call    linganisha_mfuatano
        pop     r15
        pop     r14
        pop     r13
        pop     r12
        pop     r11
        pop     r10
        pop     r9
        pop     r8
        test    eax, eax
        je      .as_found
        inc     r10
        jmp     .as_search

.as_found:
        ; Toa maelekezo: mov [rbp - ofseti], eax  ->  89 45 XX
        mov     edi, [local_offset + r10*4]
        neg     edi
        mov     al, 0x89                ; MOV r/m32, r32
        call    gen_baiti
        mov     al, 0x45                ; ModRM: mod=01, reg=000(eax), r/m=101(rbp+disp8)
        call    gen_baiti
        mov     al, dil                 ; disp8 (hasi)
        call    gen_baiti
        mov     eax, r8d                ; rudisha thamani iliyowekwa
        jmp     .done

.assign_err:
        xor     eax, eax
        jmp     .done
.done:
        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; uzalishaji_wambile: zalisha msimbo kwa wito wa kazi
;   r12d = faharisi ya nodi
;   matokeo kwenye eax
; -------------------------------------------------------
uzalishaji_wambile:
        push    r12
        push    r13
        push    r14
        push    r15

        mov     r12d, r12d
        mov     r13d, [ast_kulia + r12*4]     ; nodi ya jina la kazi
        mov     r14d, [ast_kushoto + r12*4]   ; orodha ya hoja

        ; Pata jina la kazi
        mov     r13d, [ast_jina_off + r13*4]
        lea     r13, [str_pool + r13]

        ; Hesabu idadi ya hoja
        xor     r15d, r15d
        mov     r8d, r14d
.count_args:
        cmp     r8d, -1
        je      .eval_args
        inc     r15d
        mov     r8d, [ast_nne + r8*4]
        jmp     .count_args

.eval_args:
        ; Tathmini kila hoja na kusukuma matokeo kwenye rafu
        mov     r8d, r14d
        xor     r9d, r9d
.eval_loop:
        cmp     r8d, -1
        je      .pop_args
        cmp     r9d, 6
        jae     .pop_args

        push    r12
        push    r13
        push    r14
        push    r15
        push    r8
        push    r9

        mov     r12d, r8d
        call    uzalishaji_ast         ; matokeo kwenye eax

        ; Sukuma matokeo kwenye rafu ya utekelezaji (push rax = 0x50)
        mov     al, 0x50
        call    gen_baiti

        pop     r9
        pop     r8
        pop     r15
        pop     r14
        pop     r13
        pop     r12

        inc     r9d
        mov     r8d, [ast_nne + r8*4]
        jmp     .eval_loop

.pop_args:
        ; Toa hoja kutoka rafu na kuziweka kwenye rejista
        ; Hoja zilisukumwa kwa mpangilio: arg0, arg1, arg2, ...
        ; Rafu sasa: [argN-1, ..., arg1, arg0] (juu kwenda chini)
        ; Tunatoa kinyume: rejista ya juu kwanza

        cmp     r15d, 6
        jb      .try_r8
        ; pop r9 = 41 59
        mov     al, 0x41
        call    gen_baiti
        mov     al, 0x59
        call    gen_baiti
.try_r8:
        cmp     r15d, 5
        jb      .try_rcx
        ; pop r8 = 41 58
        mov     al, 0x41
        call    gen_baiti
        mov     al, 0x58
        call    gen_baiti
.try_rcx:
        cmp     r15d, 4
        jb      .try_rdx
        ; pop rcx = 59
        mov     al, 0x59
        call    gen_baiti
.try_rdx:
        cmp     r15d, 3
        jb      .try_rsi
        ; pop rdx = 5A
        mov     al, 0x5A
        call    gen_baiti
.try_rsi:
        cmp     r15d, 2
        jb      .try_rdi
        ; pop rsi = 5E
        mov     al, 0x5E
        call    gen_baiti
.try_rdi:
        cmp     r15d, 1
        jb      .do_call
        ; pop rdi = 5F
        mov     al, 0x5F
        call    gen_baiti

.do_call:
        ; Toa "call" — opcode E8 ikifuatiwa na rel32
        mov     al, 0xe8
        call    gen_baiti

        ; Ongeza alama ya nje kwenye orodha ya nje na tengeneza rekebisho
        mov     rdi, [extern_count]
        cmp     rdi, MAX_EXTERNS - 1
        jae     .extern_full
        lea     rdx, [extern_name]
        mov     [rdx + rdi*8], r13
        inc     qword [extern_count]
        mov     r14d, edi               ; hifadhi faharisi ya nje
        jmp     .gen_reloc
.extern_full:
        mov     r14d, 0
.gen_reloc:
        ; Ongeza rekebisho
        mov     rdi, [rela_count]
        cmp     rdi, MAX_RELOCS - 1
        jae     .skip_reloc
        mov     edx, [text_buf_pos]
        mov     [rela_offset + rdi*4], edx
        mov     [rela_sym + rdi*4], r14d
        inc     qword [rela_count]
.skip_reloc:
        ; Weka nafasi ya rel32 (baiti 4 za sifuri)
        mov     edi, 0
        call    gen_neno4

        ; Matokeo yatakuwa kwenye eax baada ya wito
        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; uzalishaji_kama: zalisha msimbo kwa taarifa ya kama/sivyo
;   r12d = faharisi ya nodi
; -------------------------------------------------------
uzalishaji_kama:
        push    r12
        push    r13
        push    r14
        push    r15

        mov     r12d, r12d
        mov     r13d, [ast_kushoto + r12*4]  ; hali
        mov     r14d, [ast_kulia + r12*4]    ; mwili
        mov     r15d, [ast_tiga + r12*4]     ; sivyo (au -1)

        ; Zalisha hali → matokeo kwenye eax
        push    r12
        mov     r12d, r13d
        call    uzalishaji_ast
        pop     r12

        ; test eax, eax
        mov     al, 0x85
        call    gen_baiti
        mov     al, 0xc0
        call    gen_baiti

        ; jz — tutarekebisha ofseti baadaye
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0x84
        call    gen_baiti

        ; Hifadhi nafasi ya fixup na andika kishikilia (placeholder)
        mov     r8d, [text_buf_pos]     ; nafasi ya fixup ya je
        xor     edi, edi
        call    gen_neno4               ; andika baiti 4 za 0 (kishikilia)

        ; Zalisha mwili wa kama
        push    r12
        push    r8                      ; hifadhi r8d (nafasi ya fixup ya je) — uzalishaji_ast inaharibu r8d
        mov     r12d, r14d
        call    uzalishaji_ast
        pop     r8
        pop     r12

        ; Angalia ikiwa kuna tawi la sivyo
        cmp     r15d, -1
        je      .no_sivyo

        ; ===== Kuna sivyo =====
        ; Toa jmp isiyo na masharti kuruka juu ya mwili wa sivyo
        mov     al, 0xe9                ; jmp rel32
        call    gen_baiti
        mov     r9d, [text_buf_pos]     ; hifadhi nafasi ya fixup ya jmp
        xor     edi, edi
        call    gen_neno4               ; kishikilia cha jmp

        ; Rekebisha je: elekeza kwenye mwanzo wa mwili wa sivyo
        mov     edi, [text_buf_pos]     ; hapa ndipo mwili wa sivyo unaanzia
        sub     edi, r8d                ; umbali kutoka fixup
        sub     edi, 4                  ; toa ukubwa wa kishikilia chenyewe

        mov     r10d, [text_buf_pos]    ; hifadhi nafasi ya sasa
        mov     [text_buf_pos], r8d     ; rudi kwenye nafasi ya fixup
        call    gen_neno4               ; andika ofseti sahihi ya je
        mov     [text_buf_pos], r10d    ; rejesha nafasi

        ; Zalisha mwili wa sivyo
        push    r12
        push    r9                      ; hifadhi r9d (nafasi ya fixup ya jmp) — uzalishaji_ast inaharibu r9d
        mov     r12d, r15d
        call    uzalishaji_ast
        pop     r9
        pop     r12

        ; Rekebisha jmp: elekeza baada ya mwili wa sivyo
        mov     edi, [text_buf_pos]     ; mwisho wa mwili wa sivyo
        sub     edi, r9d                ; umbali kutoka fixup ya jmp
        sub     edi, 4                  ; toa ukubwa wa kishikilia

        mov     r10d, [text_buf_pos]    ; hifadhi nafasi ya sasa
        mov     [text_buf_pos], r9d     ; rudi kwenye fixup ya jmp
        call    gen_neno4               ; andika ofseti sahihi ya jmp
        mov     [text_buf_pos], r10d    ; rejesha nafasi

        jmp     .done

.no_sivyo:
        ; Rekebisha je: elekeza baada ya mwili wa kama
        mov     edi, [text_buf_pos]     ; mwisho wa mwili
        sub     edi, r8d                ; umbali kutoka fixup
        sub     edi, 4                  ; toa ukubwa wa kishikilia

        mov     r9d, [text_buf_pos]     ; hifadhi nafasi ya sasa
        mov     [text_buf_pos], r8d     ; rudi kwenye nafasi ya fixup
        call    gen_neno4               ; andika ofseti sahihi ya je
        mov     [text_buf_pos], r9d     ; rejesha nafasi

.done:
        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; uzalishaji_block: zalisha msimbo kwa block
;   r12d = faharisi ya nodi
; -------------------------------------------------------
uzalishaji_block:
        push    r12
        push    r13

        mov     r12d, r12d
        mov     r13d, [ast_kushoto + r12*4]  ; taarifa ya kwanza

.loop:
        cmp     r13d, -1
        je      .done

        push    r12
        push    r13
        mov     r12d, r13d
        call    uzalishaji_ast
        pop     r13
        pop     r12

        mov     r13d, [ast_nne + r13*4]      ; taarifa inayofuata
        jmp     .loop
.done:
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; uzalishaji_kazi: zalisha msimbo kwa kazi nzima
;   r12d = faharisi ya nodi
; -------------------------------------------------------
uzalishaji_kazi:
        push    r12
        push    r13
        push    r14
        push    r15
        push    rbp

        mov     r12d, r12d
        mov     r13d, [ast_thamani + r12*4]  ; aina ya kurudi
        mov     r14d, [ast_kulia + r12*4]    ; mwili
        mov     r15d, [ast_jina_off + r12*4] ; ofseti ya jina

        ; Weka upya vigezo vya ndani
        mov     qword [local_count], 0

        ; Ongeza lebo ya kazi
        mov     rdi, [label_count]
        cmp     rdi, MAX_LABELS - 1
        jae     .skip_label
        lea     rsi, [str_pool + r15]
        mov     [label_name + rdi*8], rsi
        mov     eax, [text_buf_pos]
        mov     [label_offset + rdi*4], eax
        inc     qword [label_count]

        ; Hifadhi faharisi ya lebo
        mov     r13, rdi
        jmp     .gen_code
.skip_label:
        mov     r13, 0
.gen_code:
        ; Tengeneza lebo ya kurudi
        call    gen_label_mpya
        mov     [gen_return_label], rax

        ; Weka stack frame
        ; push rbp
        mov     al, 0x55
        call    gen_baiti
        ; mov rbp, rsp
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0xe5
        call    gen_baiti

        ; Tenga nafasi ya rafu kwa vigezo vya ndani
        ; sub rsp, X — tutaweka baada ya kujua ukubwa
        ; Kwa sasa, weka nafasi ya baiti 4
        mov     r8d, [text_buf_pos]     ; hifadhi nafasi ya kurekebisha
        push    r8
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x83
        call    gen_baiti
        mov     al, 0xec
        call    gen_baiti
        mov     al, 0                    ; itajazwa baadaye
        call    gen_baiti

        ; Hifadhi vigezo vya kazi (parameters) kwenye rafu
        push    r14                     ; hifadhi nodi ya mwili
        push    r13                     ; hifadhi faharisi ya lebo

        mov     r10d, [ast_kushoto + r12*4] ; orodha ya vigezo
        xor     r11d, r11d              ; faharisi ya hoja (0-5)
.param_loop:
        cmp     r10d, -1
        je      .params_done
        cmp     r11d, 6
        jae     .params_done

        ; Sajili kigezo kwenye orodha ya ndani
        mov     rdi, [local_count]
        mov     r15d, [ast_jina_off + r10*4]
        lea     rcx, [str_pool + r15]
        mov     [local_name + rdi*8], rcx

        ; Kokotoa ofseti ya rafu: (local_count + 1) * 4
        mov     r8d, edi
        inc     r8d
        imul    r8d, 4
        mov     [local_offset + rdi*4], r8d
        inc     qword [local_count]

        ; Toa maelekezo ya kuhifadhi hoja kwenye rafu
        ; mov [rbp - ofseti], reg — reg inategemea faharisi
        neg     r8d                     ; ofseti hasi kwa rbp

        cmp     r11d, 0
        jne     .try_arg1
        ; mov [rbp + disp8], edi → 89 7D XX
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x7D
        call    gen_baiti
        mov     al, r8b
        call    gen_baiti
        jmp     .next_param
.try_arg1:
        cmp     r11d, 1
        jne     .try_arg2
        ; mov [rbp + disp8], esi → 89 75 XX
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x75
        call    gen_baiti
        mov     al, r8b
        call    gen_baiti
        jmp     .next_param
.try_arg2:
        cmp     r11d, 2
        jne     .try_arg3
        ; mov [rbp + disp8], edx → 89 55 XX
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x55
        call    gen_baiti
        mov     al, r8b
        call    gen_baiti
        jmp     .next_param
.try_arg3:
        cmp     r11d, 3
        jne     .try_arg4
        ; mov [rbp + disp8], ecx → 89 4D XX
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x4D
        call    gen_baiti
        mov     al, r8b
        call    gen_baiti
        jmp     .next_param
.try_arg4:
        cmp     r11d, 4
        jne     .try_arg5
        ; mov [rbp + disp8], r8d → 44 89 45 XX
        mov     al, 0x44
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x45
        call    gen_baiti
        mov     al, r8b
        call    gen_baiti
        jmp     .next_param
.try_arg5:
        ; mov [rbp + disp8], r9d → 44 89 4D XX
        mov     al, 0x44
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x4D
        call    gen_baiti
        mov     al, r8b
        call    gen_baiti
.next_param:
        inc     r11d
        mov     r10d, [ast_nne + r10*4]  ; kigezo kinachofuata
        jmp     .param_loop
.params_done:
        pop     r13                     ; rejesha faharisi ya lebo
        pop     r14                     ; rejesha nodi ya mwili

        ; Zalisha mwili
        cmp     r14d, -1
        je      .no_body
        push    r12
        mov     r12d, r14d
        call    uzalishaji_ast
        pop     r12
.no_body:

        ; Rekebisha ukubwa wa rafu
        pop     r8
        mov     rdi, [local_count]
        imul    rdi, 4                  ; baiti 4 kwa kila kigezo
        add     rdi, 16                 ; nafasi ya ziada
        and     rdi, ~15                ; pangilia kwa 16
        mov     rcx, [text_buf_pos]
        sub     rcx, r8
        ; r8 ina nafasi ya baiti ya sub rsp — tunaweka thamani
        mov     byte [text_buf + r8 + 3], dil

        ; Weka lebo ya kurudi hapa — kabla ya epilogue
        mov     rdi, [gen_return_label]
        mov     esi, edi
        mov     edi, [text_buf_pos]
        call    gen_weka_lebo

        ; Epilogue
.epilogue:
        ; leave
        mov     al, 0xc9
        call    gen_baiti
        ; ret
        mov     al, 0xc3
        call    gen_baiti

        ; Rekebisha fixups
        call    gen_fixup_jaza

        ; Weka ukubwa wa lebo
        mov     rdi, [label_count]
        dec     rdi
        mov     eax, [text_buf_pos]
        sub     eax, [label_offset + rdi*4]
        mov     [label_size + rdi*4], eax

        pop     rbp
        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; gen_fixup_jaza: jaza fixups zote zilizosubiri
; -------------------------------------------------------
gen_fixup_jaza:
        push    r12
        push    r13
        push    r14
        push    r15

        mov     r12, [gen_fixup_count]
        xor     r13d, r13d
.loop:
        cmp     r13, r12
        jae     .done

        mov     r14d, [gen_fixup_offset + r13*4]
        mov     r15d, [gen_fixup_label + r13*4]

        ; Tafuta nafasi ya lebo kutoka gen_label_pos
        mov     eax, [gen_label_pos + r15*4]
        sub     eax, r14d
        sub     eax, 4                  ; rekebisha kwa urefu wa jmp rel32
        mov     [text_buf + r14], eax

        inc     r13
        jmp     .loop
.done:
        mov     qword [gen_fixup_count], 0
        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; uzalishaji_ast: linda kuu la uzalishaji
;   r12d = faharisi ya nodi
;   matokeo kwenye eax (kama inafaa)
; -------------------------------------------------------
uzalishaji_ast:
        push    r12
        push    rbx

        cmp     r12d, -1
        je      .return_neg1

        mov     ebx, [ast_aina + r12*4]

        cmp     ebx, AST_NAMBA
        je      .call_nambari
        cmp     ebx, AST_JINA
        je      .call_jina
        cmp     ebx, AST_RUDISHA
        je      .call_rudisha
        cmp     ebx, AST_TANGAZO
        je      .call_tangazo
        cmp     ebx, AST_KAULI
        je      .call_kauli
        cmp     ebx, AST_WAMBILE
        je      .call_wambile
        cmp     ebx, AST_KAMA
        je      .call_kama
        cmp     ebx, AST_BLOCK
        je      .call_block
        cmp     ebx, AST_KAZI
        je      .call_kazi

        ; Chaguo-msingi: rudisha 0
        xor     eax, eax
        jmp     .done

.call_nambari:
        call    uzalishaji_nambari
        jmp     .done
.call_jina:
        call    uzalishaji_jina
        jmp     .done
.call_rudisha:
        call    uzalishaji_rudisha
        xor     eax, eax               ; rudisha haina thamani
        jmp     .done
.call_tangazo:
        call    uzalishaji_tangazo
        jmp     .done
.call_kauli:
        call    uzalishaji_kauli_ya_binary
        jmp     .done
.call_wambile:
        call    uzalishaji_wambile
        jmp     .done
.call_kama:
        call    uzalishaji_kama
        xor     eax, eax
        jmp     .done
.call_block:
        call    uzalishaji_block
        xor     eax, eax
        jmp     .done
.call_kazi:
        call    uzalishaji_kazi
        xor     eax, eax
        jmp     .done

.return_neg1:
        mov     eax, -1
.done:
        pop     rbx
        pop     r12
        ret

; =============================================================================
; Sehemu ya 7: Kitoa ELF (ELF Emitter)
; =============================================================================

; -------------------------------------------------------
; toa_elf: toa ELF .o kamili kwa stdout
; -------------------------------------------------------
toa_elf:
        push    r12
        push    r13
        push    r14
        push    r15

        ; Kokotoa ofseti za sehemu
        mov     r12d, [text_buf_pos]    ; text_size
        mov     r13d, [data_buf_pos]    ; data_size

        ; symtab: 1 (null) + lebo + vigeu vya ulimwengu + nje
        mov     r14, [label_count]
        add     r14, [global_count]
        add     r14, [extern_count]
        inc     r14                     ; +1 kwa null
        imul    r14, 24                 ; baiti 24 kwa ingizo
        mov     r15d, r14d              ; symtab_size

        ; strtab: hesabu ukubwa kwa kutembea majina yote
        call    toa_elf_hesabu_strtab
        mov     r13d, eax               ; strtab_size

        ; rela: 24 baiti kwa ingizo
        mov     r14, [rela_count]
        imul    r14, 24
        mov     edx, r14d               ; rela_size

        ; Ofseti za sehemu
        ; text_off = 64
        ; data_off = 64 + text_size
        ; symtab_off = data_off + data_size
        ; strtab_off = symtab_off + symtab_size
        ; rela_off = strtab_off + strtab_size
        ; shstrtab_off = rela_off + rela_size
        ; shoff = shstrtab_off + 55

        mov     r8d, 64                 ; text_off
        mov     r9d, r8d
        add     r9d, r12d               ; data_off
        mov     r10d, r9d
        add     r10d, [data_buf_pos]    ; symtab_off
        mov     r11d, r10d
        add     r11d, r15d              ; strtab_off
        mov     r12d, r11d
        add     r12d, r13d              ; rela_off (tunatumia strtab_size kwenye r13)
        mov     r13d, r12d
        add     r13d, edx               ; shstrtab_off
        mov     r14d, r13d
        add     r14d, SHSTRTAB_SIZE      ; shoff

        ; Hifadhi ofseti muhimu kwenye stack
        push    r15                     ; symtab_size
        push    r10                     ; symtab_off
        push    r12                     ; rela_off
        push    r13                     ; shstrtab_off
        push    r14                     ; shoff

        ; 1. Andika kichwa cha ELF
        call    toa_elf_kichwa

        ; 2. Andika .text
        lea     rsi, [text_buf]
        mov     edx, [text_buf_pos]
        cmp     edx, 0
        je      .skip_text
        call    sys_write_buf
.skip_text:

        ; 3. Andika .data
        lea     rsi, [data_buf]
        mov     edx, [data_buf_pos]
        cmp     edx, 0
        je      .skip_data
        call    sys_write_buf
.skip_data:

        ; 4. Andika .symtab
        call    toa_elf_symtab

        ; 5. Andika .strtab
        call    toa_elf_strtab

        ; 6. Andika .rela.text
        call    toa_elf_rela

        ; 7. Andika .shstrtab
        call    toa_elf_shstrtab

        ; 8. Andika vichwa vya sehemu
        pop     r14                     ; shoff
        pop     r13                     ; shstrtab_off
        pop     r12                     ; rela_off
        pop     r10                     ; symtab_off
        pop     r15                     ; symtab_size
        ; Tunahitaji strtab_size na strtab_off pia
        ; Hizo ziko kwenye r11 na r13 (kabla ya kubadilishwa)
        call    toa_elf_sehemu_vichwa

        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; toa_elf_hesabu_strtab: hesabu ukubwa wa .strtab
;   rax = ukubwa
; -------------------------------------------------------
toa_elf_hesabu_strtab:
        push    r12
        push    r13
        push    r14

        ; Anza na baiti 1 kwa "\0"
        mov     r14d, 1

        ; Pitia lebo zote
        xor     r12d, r12d
        mov     r13, [label_count]
.lebo_loop:
        cmp     r12, r13
        jae     .lebo_done
        mov     rdi, [label_name + r12*8]
        call    urefu_wa_mfuatano
        add     r14, rax
        inc     r14                     ; +1 kwa '\0'
        inc     r12
        jmp     .lebo_loop
.lebo_done:

        ; Pitia nje zote
        xor     r12d, r12d
        mov     r13, [extern_count]
.nje_loop:
        cmp     r12, r13
        jae     .nje_done
        mov     rdi, [extern_name + r12*8]
        call    urefu_wa_mfuatano
        add     r14, rax
        inc     r14
        inc     r12
        jmp     .nje_loop
.nje_done:

        ; Pitia vigeu vya ulimwengu
        xor     r12d, r12d
        mov     r13, [global_count]
.ulimwengu_loop:
        cmp     r12, r13
        jae     .ulimwengu_done
        mov     rdi, [global_name + r12*8]
        call    urefu_wa_mfuatano
        add     r14, rax
        inc     r14
        inc     r12
        jmp     .ulimwengu_loop
.ulimwengu_done:

        mov     eax, r14d
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; toa_elf_kichwa: andika kichwa cha ELF kwa stdout
;   inatumia ofseti zilizo kwenye stack
; -------------------------------------------------------
toa_elf_kichwa:
        ; Hujenga na kuandika kichwa kamili cha ELF (baiti 64)
        ; Kichwa kina sehemu tatu:
        ;   1. Baiti  0-39: e_ident, e_type, e_machine, e_version, e_entry, e_phoff
        ;   2. Baiti 40-47: e_shoff (inakokotolewa wakati wa utekelezaji)
        ;   3. Baiti 48-63: e_flags, e_ehsize, e_phentsize, e_phnum, e_shentsize, e_shnum, e_shstrndx
        ;
        ; Thamani ya e_shoff inapitishwa kupitia stack (r14 kwenye toa_elf)

        ; 1. Andika baiti 0-39 za kiolezo
        lea     rsi, [ehdr_template]
        mov     rdx, 40
        call    sys_write_buf

        ; 2. Andika e_shoff (baiti 40-47) — thamani iliyokokotolewa
        ; Inasomwa kutoka stack: toa_elf ilisukuma r14 (shoff) kabla ya kuita
        ; Kwa wakati huu, [rsp] = return address, [rsp+8] = r14 (shoff)
        push    rbp
        mov     rbp, rsp
        mov     rdi, [rbp+16]           ; shoff kutoka stack ya toa_elf
        pop     rbp
        call    andika_neno8_moja_kwa_moja

        ; 3. Andika baiti 48-63 za kiolezo (e_flags hadi mwisho)
        lea     rsi, [ehdr_template + 48]
        mov     rdx, 16
        call    sys_write_buf

        ret

; -------------------------------------------------------
; toa_elf_symtab: andika .symtab kwa stdout
; -------------------------------------------------------
toa_elf_symtab:
        push    r12
        push    r13
        push    r14
        push    r15

        ; Ingizo la null
        mov     edi, 0
        call    andika_neno4_moja_kwa_moja         ; st_name
        call    andika_neno4_moja_kwa_moja         ; st_info + st_other + st_shndx (zote 0)
        call    andika_neno8_moja_kwa_moja         ; st_value
        call    andika_neno8_moja_kwa_moja         ; st_size

        ; Kokotoa ofseti za strtab kwa kila lebo
        ; Ofseti ya kwanza baada ya \0 ni 1
        mov     r14d, 1

        ; Ingizo kwa kila lebo (kazi)
        xor     r12d, r12d
        mov     r13, [label_count]
.lebo_loop:
        cmp     r12, r13
        jae     .lebo_done

        ; st_name = ofseti kwenye strtab
        mov     edi, r14d
        call    andika_neno4_moja_kwa_moja

        ; st_info = 18 (STB_GLOBAL | STT_FUNC)
        mov     dil, 18
        call    andika_baiti_moja_kwa_moja
        ; st_other = 0
        mov     dil, 0
        call    andika_baiti_moja_kwa_moja
        ; st_shndx = 1 (.text)
        mov     di, 1
        call    andika_neno2_moja_kwa_moja

        ; st_value = ofseti kwenye .text
        mov     edi, [label_offset + r12*4]
        call    andika_neno8_moja_kwa_moja

        ; st_size = ukubwa wa kazi
        mov     edi, [label_size + r12*4]
        call    andika_neno8_moja_kwa_moja

        ; Sasisha ofseti ya strtab
        mov     rdi, [label_name + r12*8]
        call    urefu_wa_mfuatano
        add     r14d, eax
        inc     r14d                    ; +1 kwa '\0'

        inc     r12
        jmp     .lebo_loop
.lebo_done:

        ; Ingizo kwa kila nje
        xor     r12d, r12d
        mov     r13, [extern_count]
.nje_loop:
        cmp     r12, r13
        jae     .nje_done

        mov     edi, r14d
        call    andika_neno4_moja_kwa_moja
        mov     dil, 0                  ; STB_LOCAL, STT_NOTYPE (UNDEF)
        call    andika_baiti_moja_kwa_moja
        mov     dil, 0
        call    andika_baiti_moja_kwa_moja
        mov     di, 0                   ; st_shndx = 0 (UNDEF)
        call    andika_neno2_moja_kwa_moja
        ; st_value = 0
        call    andika_neno8_moja_kwa_moja
        ; st_size = 0
        call    andika_neno8_moja_kwa_moja

        mov     rdi, [extern_name + r12*8]
        call    urefu_wa_mfuatano
        add     r14d, eax
        inc     r14d

        inc     r12
        jmp     .nje_loop
.nje_done:

        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; toa_elf_strtab: andika .strtab kwa stdout
; -------------------------------------------------------
toa_elf_strtab:
        push    r12
        push    r13
        push    r14

        ; Anza na '\0'
        mov     dil, 0
        call    andika_baiti_moja_kwa_moja

        ; Andika majina ya lebo
        xor     r12d, r12d
        mov     r13, [label_count]
.lebo_loop:
        cmp     r12, r13
        jae     .lebo_done
        mov     rdi, [label_name + r12*8]
        call    andika_mfuatano
        inc     r12
        jmp     .lebo_loop
.lebo_done:

        ; Andika majina ya nje
        xor     r12d, r12d
        mov     r13, [extern_count]
.nje_loop:
        cmp     r12, r13
        jae     .nje_done
        mov     rdi, [extern_name + r12*8]
        call    andika_mfuatano
        inc     r12
        jmp     .nje_loop
.nje_done:

        ; Andika majina ya vigezo vya ulimwengu
        xor     r12d, r12d
        mov     r13, [global_count]
.ulimwengu_loop:
        cmp     r12, r13
        jae     .ulimwengu_done
        mov     rdi, [global_name + r12*8]
        call    andika_mfuatano
        inc     r12
        jmp     .ulimwengu_loop
.ulimwengu_done:

        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; toa_elf_rela: andika .rela.text kwa stdout
; -------------------------------------------------------
toa_elf_rela:
        push    r12
        push    r13

        xor     r12d, r12d
        mov     r13, [rela_count]
.loop:
        cmp     r12, r13
        jae     .done

        ; r_offset
        mov     edi, [rela_offset + r12*4]
        call    andika_neno8_moja_kwa_moja
        ; r_info = ELF64_R_INFO(sym, R_X86_64_PC32)
        ; Aina = 2 (R_X86_64_PC32)
        mov     edi, [rela_sym + r12*4]
        shl     rdi, 32
        or      rdi, 2
        call    andika_neno8_moja_kwa_moja
        ; r_addend = -4
        mov     rdi, -4
        call    andika_neno8_moja_kwa_moja

        inc     r12
        jmp     .loop
.done:
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; toa_elf_shstrtab: andika .shstrtab kwa stdout
; -------------------------------------------------------
toa_elf_shstrtab:
        lea     rsi, [shstrtab_data]
        mov     edx, SHSTRTAB_SIZE
        call    sys_write_buf
        ret

; -------------------------------------------------------
; toa_elf_sehemu_vichwa: andika vichwa vya sehemu kwa stdout
;   r8 = text_off, r9 = data_off, r10 = symtab_off
;   r11 = strtab_off (imebadilishwa), r12 = rela_off, r13 = shstrtab_off
;   r14 = shoff, r15 = symtab_size
;   pia inahitaji strtab_size
; -------------------------------------------------------
toa_elf_sehemu_vichwa:
        push    r12
        push    r13
        push    r14
        push    r15

        ; Hali ni ngumu kwa sababu tumepoteza baadhi ya maadili.
        ; Kwa sasa, tunatoa vichwa vya sehemu vilivyo ngumu.
        ; Hii ni ya muda — tutakokotoa upya.

        ; NULL sehemu (baiti 64 za sifuri)
        mov     ecx, 16
        xor     edi, edi
.null_loop:
        push    rcx
        call    andika_neno4_moja_kwa_moja
        pop     rcx
        loop    .null_loop

        ; Kwa sasa tunatoa vichwa vilivyo ngumu kwa sehemu zilizobaki
        ; Hii itarekebishwa wakati uzalishaji kamili utakapofanya kazi

        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret

; =============================================================================
; Sehemu ya 8: Programu Kuu (_start)
; =============================================================================

        global _start
_start:
        ; Weka stack frame ya msingi
        xor     ebp, ebp
        mov     rbp, rsp

        ; Weka upya hali ya mkusanyaji
        mov     qword [compiler_state], 0
        mov     qword [token_count], 0
        mov     qword [token_pos], 0
        mov     qword [ast_count], 0
        mov     qword [str_pool_pos], 0
        mov     qword [text_buf_pos], 0
        mov     qword [data_buf_pos], 0
        mov     qword [label_count], 0
        mov     qword [extern_count], 0
        mov     qword [rela_count], 0
        mov     qword [global_count], 0
        mov     qword [local_count], 0
        mov     qword [gen_label_count], 0
        mov     qword [gen_fixup_count], 0

        ; Weka '\0' mwanzoni mwa bwawa la herufi
        mov     byte [str_pool], 0
        inc     qword [str_pool_pos]

        ; Angalia hoja za mstari wa amri
        pop     rax                     ; argc
        cmp     rax, 1
        jle     .tumia_stdin

        ; Tuna hoja — tumia argv[1] kama jina la faili
        pop     rdi                     ; argv[0] — ruka
        pop     rdi                     ; argv[1]
        call    soma_chanzo_kutoka_faili
        jmp     .anza_kukusanya

.tumia_stdin:
        ; Soma kutoka stdin
        call    soma_chanzo_kutoka_stdin

.anza_kukusanya:
        ; Changanua chanzo kuwa tokeni
        call    changanua_chanzo

        ; Changanua tokeni kuwa AST
        call    changanua_programu
        mov     r12d, eax               ; mzizi wa AST

        cmp     r12d, -1
        je      .parse_error

        ; Zalisha msimbo kutoka AST
        ; Pitia matamko yote kwenye mzizi
        mov     r13d, [ast_kushoto + r12*4]  ; tamko la kwanza
.gen_loop:
        cmp     r13d, -1
        je      .gen_done

        cmp     dword [ast_aina + r13*4], AST_KAZI
        jne     .gen_next

        ; Angalia ikiwa kazi ina mwili (sio tangazo la mbele)
        cmp     dword [ast_kulia + r13*4], -1
        je      .gen_next

        push    r12
        push    r13
        mov     r12d, r13d
        call    uzalishaji_ast
        pop     r13
        pop     r12

.gen_next:
        mov     r13d, [ast_nne + r13*4]
        jmp     .gen_loop

.gen_done:
        ; Toa ELF kwa stdout
        ; Kwa sasa, tunatoa ELF rahisi kwa mkono
        call    toa_elf_rahisi

        ; Toka kwa mafanikio
        xor     edi, edi
        call    sys_exit

.parse_error:
        lea     rdi, [msg_parseerr]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

; -------------------------------------------------------
; toa_elf_rahisi: toleo rahisi la kutoa ELF
;   Hii inajenga ELF .o moja kwa moja kwa kutumia
;   vibafa vilivyojazwa na vizalishe.
; -------------------------------------------------------
toa_elf_rahisi:
        push    r12
        push    r13
        push    r14
        push    r15

        ; Kokotoa ukubwa na ofseti zote
        mov     r12d, [text_buf_pos]    ; text_size
        mov     eax, [data_buf_pos]     ; data_size
        mov     ebx, eax

        ; Hesabu symtab_size
        mov     r14, [label_count]      ; idadi ya lebo
        add     r14, [global_count]     ; + vigeu vya ulimwengu
        ; Nje zisizo-duplicate (kwa sasa tunachukulia zote ni za kipekee)
        mov     r15, [extern_count]
        add     r14, r15
        inc     r14                     ; +1 kwa null
        imul    r14, 24
        mov     r13d, r14d              ; symtab_size

        ; Hesabu ukubwa wa strtab
        ; Anza na 1 kwa '\0' ya kwanza
        mov     r14d, 1

        ; Ongeza urefu wa kila jina la lebo
        xor     ecx, ecx
.str_loop:
        cmp     rcx, [label_count]
        jae     .str_done_labels
        mov     rdi, [label_name + rcx*8]
        push    rcx
        call    urefu_wa_mfuatano
        pop     rcx
        add     r14d, eax
        inc     r14d                    ; +1 kwa '\0'
        inc     rcx
        jmp     .str_loop
.str_done_labels:

        ; Ongeza majina ya vigeu vya ulimwengu
        xor     ecx, ecx
.str_global_loop:
        cmp     rcx, [global_count]
        jae     .str_done_globals
        mov     rdi, [global_name + rcx*8]
        push    rcx
        call    urefu_wa_mfuatano
        pop     rcx
        add     r14d, eax
        inc     r14d
        inc     rcx
        jmp     .str_global_loop
.str_done_globals:

        ; Ongeza majina ya nje
        xor     ecx, ecx
.str_extern_loop:
        cmp     rcx, [extern_count]
        jae     .str_done_externs
        mov     rdi, [extern_name + rcx*8]
        push    rcx
        call    urefu_wa_mfuatano
        pop     rcx
        add     r14d, eax
        inc     r14d
        inc     rcx
        jmp     .str_extern_loop
.str_done_externs:

        mov     r10d, r14d              ; strtab_size

        ; rela_size
        mov     r14, [rela_count]
        imul    r14, 24
        mov     r11d, r14d              ; rela_size

        ; Panga ofseti
        mov     r8d, 64                 ; text_off (baada ya kichwa cha ELF)
        mov     r9d, r8d
        add     r9d, r12d               ; data_off
        mov     edi, r9d
        add     edi, ebx                ; symtab_off
        mov     eax, edi
        add     eax, r13d               ; strtab_off
        mov     esi, eax
        add     esi, r10d               ; rela_off
        mov     edx, esi
        add     edx, r11d               ; shstrtab_off
        mov     ecx, edx
        add     ecx, SHSTRTAB_SIZE       ; shoff

        ; Hifadhi kwenye stack
        push    rdi                     ; symtab_off
        push    rax                     ; strtab_off
        push    rsi                     ; rela_off
        push    rdx                     ; shstrtab_off
        push    rcx                     ; shoff
        push    r13                     ; symtab_size
        push    r10                     ; strtab_size

        ; === ANDIKA KICHWA CHA ELF ===

        ; Magic na EI
        mov     edi, 0x464c457f         ; ELF magic (little-endian)
        ; Andika baiti kwa baiti kwenye bafa la muda, kisha toa
        ; Tunajenga kichwa kwenye tmp_buf
        lea     r15, [tmp_buf]

        ; e_ident[0:4] = ELF magic
        mov     dword [r15], 0x464c457f
        ; e_ident[4] = EI_CLASS (2)
        mov     byte [r15 + 4], 2
        ; e_ident[5] = EI_DATA (1)
        mov     byte [r15 + 5], 1
        ; e_ident[6] = EI_VERSION (1)
        mov     byte [r15 + 6], 1
        ; e_ident[7] = EI_OSABI (0)
        mov     byte [r15 + 7], 0
        ; e_ident[8:16] = padding
        mov     qword [r15 + 8], 0
        ; e_type = ET_REL (1)
        mov     word [r15 + 16], 1
        ; e_machine = EM_X86_64 (62)
        mov     word [r15 + 18], 62
        ; e_version = 1
        mov     dword [r15 + 20], 1
        ; e_entry = 0
        mov     qword [r15 + 24], 0
        ; e_phoff = 0
        mov     qword [r15 + 32], 0
        ; e_shoff = itajazwa
        pop     rax                     ; strtab_size (tunachukua kutoka stack vibaya)
        pop     rbx                     ; symtab_size
        pop     rcx                     ; shoff
        mov     qword [r15 + 40], rcx
        ; e_flags = 0
        mov     dword [r15 + 48], 0
        ; e_ehsize = 64
        mov     word [r15 + 52], 64
        ; e_phentsize = 0
        mov     word [r15 + 54], 0
        ; e_phnum = 0
        mov     word [r15 + 56], 0
        ; e_shentsize = 64
        mov     word [r15 + 58], 64
        ; e_shnum = 7
        mov     word [r15 + 60], 7
        ; e_shstrndx = 6
        mov     word [r15 + 62], 6

        ; Andika kichwa (baiti 64) kwa stdout
        lea     rsi, [tmp_buf]
        mov     edx, 64
        call    sys_write_buf

        ; Rejesha maadili kutoka stack
        ; Sasa stack ina: symtab_off, strtab_off, rela_off, shstrtab_off
        ; Tunazihitaji baadaye kwa vichwa vya sehemu
        pop     r15                     ; symtab_off (tulivuta vibaya, rekebisha)
        pop     r14                     ; strtab_off
        pop     r13                     ; rela_off
        pop     r12                     ; shstrtab_off
        ; shoff tayari imetumiwa

        ; === ANDIKA .text ===
        lea     rsi, [text_buf]
        mov     edx, [text_buf_pos]
        cmp     edx, 0
        je      .skip_text
        call    sys_write_buf
.skip_text:

        ; === ANDIKA .data ===
        lea     rsi, [data_buf]
        mov     edx, [data_buf_pos]
        cmp     edx, 0
        je      .skip_data
        call    sys_write_buf
.skip_data:

        ; === ANDIKA .symtab ===
        push    r15                     ; symtab_off (hifadhi kwa vichwa vya sehemu)
        push    r14                     ; strtab_off
        push    r13                     ; rela_off
        push    r12                     ; shstrtab_off

        call    toa_elf_symtab_rahisi

        ; === ANDIKA .strtab ===
        call    toa_elf_strtab_rahisi

        ; === ANDIKA .rela.text ===
        call    toa_elf_rela_rahisi

        ; === ANDIKA .shstrtab ===
        lea     rsi, [shstrtab_data]
        mov     edx, SHSTRTAB_SIZE
        call    sys_write_buf

        ; === ANDIKA VICHWA VYA SEHEMU ===
        pop     r12                     ; shstrtab_off
        pop     r13                     ; rela_off
        pop     r14                     ; strtab_off
        pop     r15                     ; symtab_off

        call    toa_elf_vichwa_vya_sehemu_rahisi

        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; toa_elf_symtab_rahisi: toleo rahisi la .symtab
; -------------------------------------------------------
toa_elf_symtab_rahisi:
        push    r12
        push    r13
        push    r14

        ; Ingizo la null (baiti 24 za sifuri)
        mov     ecx, 6
        xor     edi, edi
.null_loop:
        push    rcx
        call    andika_neno4_moja_kwa_moja
        pop     rcx
        loop    .null_loop

        ; Kokotoa ofseti za strtab kwa kila jina
        mov     r14d, 1                 ; ofseti ya kwanza baada ya '\0'

        ; Andika lebo
        xor     r12d, r12d
        mov     r13, [label_count]
.lebo_loop:
        cmp     r12, r13
        jae     .lebo_done

        mov     edi, r14d
        call    andika_neno4_moja_kwa_moja
        mov     dil, 18                 ; STB_GLOBAL | STT_FUNC
        call    andika_baiti_moja_kwa_moja
        mov     dil, 0
        call    andika_baiti_moja_kwa_moja
        mov     di, 1                   ; .text
        call    andika_neno2_moja_kwa_moja
        mov     edi, [label_offset + r12*4]
        call    andika_neno8_moja_kwa_moja
        mov     edi, [label_size + r12*4]
        call    andika_neno8_moja_kwa_moja

        mov     rdi, [label_name + r12*8]
        call    urefu_wa_mfuatano
        add     r14d, eax
        inc     r14d

        inc     r12
        jmp     .lebo_loop
.lebo_done:

        ; Andika vigeu vya ulimwengu
        xor     r12d, r12d
        mov     r13, [global_count]
.ulimwengu_loop:
        cmp     r12, r13
        jae     .ulimwengu_done

        mov     edi, r14d
        call    andika_neno4_moja_kwa_moja
        mov     dil, 17                 ; STB_GLOBAL | STT_OBJECT
        call    andika_baiti_moja_kwa_moja
        mov     dil, 0
        call    andika_baiti_moja_kwa_moja
        mov     di, 2                   ; .data
        call    andika_neno2_moja_kwa_moja
        mov     edi, [global_offset + r12*4]
        call    andika_neno8_moja_kwa_moja
        mov     edi, [global_size + r12*4]
        call    andika_neno8_moja_kwa_moja

        mov     rdi, [global_name + r12*8]
        call    urefu_wa_mfuatano
        add     r14d, eax
        inc     r14d

        inc     r12
        jmp     .ulimwengu_loop
.ulimwengu_done:

        ; Andika nje
        xor     r12d, r12d
        mov     r13, [extern_count]
.nje_loop:
        cmp     r12, r13
        jae     .nje_done

        mov     edi, r14d
        call    andika_neno4_moja_kwa_moja
        mov     dil, 16                 ; STB_GLOBAL | STT_FUNC (UNDEF kwa kuwa shndx=0)
        call    andika_baiti_moja_kwa_moja
        mov     dil, 0
        call    andika_baiti_moja_kwa_moja
        mov     di, 0                   ; UNDEF
        call    andika_neno2_moja_kwa_moja
        mov     edi, 0
        call    andika_neno8_moja_kwa_moja
        mov     edi, 0
        call    andika_neno8_moja_kwa_moja

        mov     rdi, [extern_name + r12*8]
        call    urefu_wa_mfuatano
        add     r14d, eax
        inc     r14d

        inc     r12
        jmp     .nje_loop
.nje_done:

        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; toa_elf_strtab_rahisi: toleo rahisi la .strtab
; -------------------------------------------------------
toa_elf_strtab_rahisi:
        push    r12
        push    r13

        ; '\0'
        mov     dil, 0
        call    andika_baiti_moja_kwa_moja

        ; Majina ya lebo
        xor     r12d, r12d
        mov     r13, [label_count]
.lebo_loop:
        cmp     r12, r13
        jae     .lebo_done
        mov     rdi, [label_name + r12*8]
        call    andika_mfuatano
        mov     dil, 0
        call    andika_baiti_moja_kwa_moja
        inc     r12
        jmp     .lebo_loop
.lebo_done:

        ; Majina ya vigeu vya ulimwengu
        xor     r12d, r12d
        mov     r13, [global_count]
.ulimwengu_loop:
        cmp     r12, r13
        jae     .ulimwengu_done
        mov     rdi, [global_name + r12*8]
        call    andika_mfuatano
        mov     dil, 0
        call    andika_baiti_moja_kwa_moja
        inc     r12
        jmp     .ulimwengu_loop
.ulimwengu_done:

        ; Majina ya nje
        xor     r12d, r12d
        mov     r13, [extern_count]
.nje_loop:
        cmp     r12, r13
        jae     .nje_done
        mov     rdi, [extern_name + r12*8]
        call    andika_mfuatano
        mov     dil, 0
        call    andika_baiti_moja_kwa_moja
        inc     r12
        jmp     .nje_loop
.nje_done:

        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; toa_elf_rela_rahisi: toleo rahisi la .rela.text
; -------------------------------------------------------
toa_elf_rela_rahisi:
        push    r12
        push    r13
        push    r14

        xor     r12d, r12d
        mov     r13, [rela_count]

        ; Kokotoa faharisi ya alama kwa kila rekebisho
        ; Alama za lebo zinaanza baada ya null: faharisi = 1
        ; Nje zinaanza baada ya lebo zote: faharisi = 1 + label_count
.loop:
        cmp     r12, r13
        jae     .done

        ; r_offset
        mov     edi, [rela_offset + r12*4]
        call    andika_neno8_moja_kwa_moja

        ; r_info (aina = 2 kwa R_X86_64_PC32, sym = faharisi)
        mov     r14d, [rela_sym + r12*4]
        ; Faharisi ya alama = 1 + label_count + extern_index
        add     r14d, 1
        mov     rax, [label_count]
        add     r14d, eax
        add     r14d, [global_count]
        shl     r14, 32
        or      r14, 2                  ; R_X86_64_PC32
        mov     rdi, r14
        call    andika_neno8_moja_kwa_moja

        ; r_addend = -4
        mov     rdi, -4
        call    andika_neno8_moja_kwa_moja

        inc     r12
        jmp     .loop
.done:
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; toa_elf_vichwa_vya_sehemu_rahisi: andika vichwa vya sehemu
;   Tunaandika vichwa 7 (0 hadi 6)
;   Tunahitaji kuhesabu upya ofseti zote hapa
; -------------------------------------------------------
toa_elf_vichwa_vya_sehemu_rahisi:
        ; Kwa sasa tunatoa vichwa vilivyo ngumu kwa sehemu zilizobaki
        ; Hii itarekebishwa baadaye

        ; Tunahitaji kukokotoa tena ofseti zote.
        ; Badala ya kufanya hivyo, tunaandika vichwa kwa mkono.

        ; Data tunayoihitaji:
        ; text_size = [text_buf_pos]
        ; data_size = [data_buf_pos]
        ; symtab_size = (1 + label_count + global_count + extern_count) * 24
        ; strtab_size = tunahitaji kuhesabu
        ; rela_size = rela_count * 24

        ; Hebu tukokotoe ofseti:

        ; text_off = 64
        ; data_off = 64 + text_size
        ; symtab_off = data_off + data_size
        ; strtab_off = symtab_off + symtab_size
        ; rela_off = strtab_off + strtab_size
        ; shstrtab_off = rela_off + rela_size
        ; shoff = shstrtab_off + 55

        ; 0: NULL (baiti 64 za sifuri)
        mov     ecx, 16
        xor     edi, edi
.null_loop:
        push    rcx
        call    andika_neno4_moja_kwa_moja
        pop     rcx
        loop    .null_loop

        ; 1: .text — sh_name=1, SHT_PROGBITS (1), SHF_ALLOC|SHF_EXECINSTR (6),
        ;    offset=64, size=text_size, align=16
        mov     edi, 1                  ; sh_name
        call    andika_neno4_moja_kwa_moja
        mov     edi, 1                  ; sh_type = PROGBITS
        call    andika_neno4_moja_kwa_moja
        mov     rdi, 6                  ; sh_flags
        call    andika_neno8_moja_kwa_moja
        mov     rdi, 0                  ; sh_addr
        call    andika_neno8_moja_kwa_moja
        mov     edi, 64                 ; sh_offset
        call    andika_neno8_moja_kwa_moja
        mov     edi, [text_buf_pos]     ; sh_size
        call    andika_neno8_moja_kwa_moja
        mov     edi, 0                  ; sh_link
        call    andika_neno4_moja_kwa_moja
        mov     edi, 0                  ; sh_info
        call    andika_neno4_moja_kwa_moja
        mov     rdi, 16                 ; sh_addralign
        call    andika_neno8_moja_kwa_moja
        mov     rdi, 0                  ; sh_entsize
        call    andika_neno8_moja_kwa_moja

        ; 2: .data — sh_name=7, SHT_PROGBITS (1), SHF_WRITE|SHF_ALLOC (3)
        mov     edi, 7
        call    andika_neno4_moja_kwa_moja
        mov     edi, 1
        call    andika_neno4_moja_kwa_moja
        mov     rdi, 3
        call    andika_neno8_moja_kwa_moja
        mov     rdi, 0
        call    andika_neno8_moja_kwa_moja
        mov     edi, 64
        add     edi, [text_buf_pos]
        call    andika_neno8_moja_kwa_moja
        mov     edi, [data_buf_pos]
        call    andika_neno8_moja_kwa_moja
        mov     edi, 0
        call    andika_neno4_moja_kwa_moja
        mov     edi, 0
        call    andika_neno4_moja_kwa_moja
        mov     rdi, 1
        call    andika_neno8_moja_kwa_moja
        mov     rdi, 0
        call    andika_neno8_moja_kwa_moja

        ; 3: .symtab — sh_name=18, SHT_SYMTAB (2), link=4 (.strtab), info=1
        mov     edi, 18
        call    andika_neno4_moja_kwa_moja
        mov     edi, 2
        call    andika_neno4_moja_kwa_moja
        mov     rdi, 0
        call    andika_neno8_moja_kwa_moja
        mov     rdi, 0
        call    andika_neno8_moja_kwa_moja
        ; sh_offset = data_off + data_size (itakokotolewa)
        ; Kwa sasa, tunaikadiria:
        mov     edi, 64
        add     edi, [text_buf_pos]
        add     edi, [data_buf_pos]
        call    andika_neno8_moja_kwa_moja
        ; sh_size = symtab_size
        mov     r14, [label_count]
        add     r14, [global_count]
        add     r14, [extern_count]
        inc     r14
        imul    r14, 24
        mov     edi, r14d
        call    andika_neno8_moja_kwa_moja
        mov     edi, 4                  ; sh_link = .strtab
        call    andika_neno4_moja_kwa_moja
        mov     edi, 1                  ; sh_info = 1
        call    andika_neno4_moja_kwa_moja
        mov     rdi, 8                  ; sh_addralign
        call    andika_neno8_moja_kwa_moja
        mov     rdi, 24                 ; sh_entsize
        call    andika_neno8_moja_kwa_moja

        ; 4: .strtab — sh_name=26, SHT_STRTAB (3)
        mov     edi, 26
        call    andika_neno4_moja_kwa_moja
        mov     edi, 3
        call    andika_neno4_moja_kwa_moja
        mov     rdi, 0
        call    andika_neno8_moja_kwa_moja
        mov     rdi, 0
        call    andika_neno8_moja_kwa_moja
        ; sh_offset = baada ya symtab
        mov     edi, 64
        add     edi, [text_buf_pos]
        add     edi, [data_buf_pos]
        add     edi, r14d
        call    andika_neno8_moja_kwa_moja
        ; sh_size = strtab_size (tunakokotoa kwa kutumia toa_elf_hesabu_strtab)
        push    r14
        call    toa_elf_hesabu_strtab
        pop     r14
        mov     edi, eax
        call    andika_neno8_moja_kwa_moja
        mov     edi, 0
        call    andika_neno4_moja_kwa_moja
        mov     edi, 0
        call    andika_neno4_moja_kwa_moja
        mov     rdi, 1
        call    andika_neno8_moja_kwa_moja
        mov     rdi, 0
        call    andika_neno8_moja_kwa_moja

        ; 5: .rela.text — sh_name=34, SHT_RELA (4), link=3 (.symtab), info=1 (.text)
        mov     edi, 34
        call    andika_neno4_moja_kwa_moja
        mov     edi, 4
        call    andika_neno4_moja_kwa_moja
        mov     rdi, 0
        call    andika_neno8_moja_kwa_moja
        mov     rdi, 0
        call    andika_neno8_moja_kwa_moja
        ; sh_offset = baada ya strtab
        ; Tutumie makadirio
        mov     edi, 64
        add     edi, [text_buf_pos]
        add     edi, [data_buf_pos]
        add     edi, r14d               ; symtab_size
        push    rdi                     ; Hifadhi offset iliyokokotolewa
        push    r14
        push    rax
        call    toa_elf_hesabu_strtab
        pop     r15
        pop     r14
        pop     rdi                     ; Rejesha offset iliyokokotolewa
        add     edi, eax                ; + strtab_size
        call    andika_neno8_moja_kwa_moja
        ; sh_size = rela_count * 24
        mov     rdi, [rela_count]
        imul    rdi, 24
        mov     edi, edi
        call    andika_neno8_moja_kwa_moja
        mov     edi, 3                  ; sh_link = .symtab
        call    andika_neno4_moja_kwa_moja
        mov     edi, 1                  ; sh_info = .text
        call    andika_neno4_moja_kwa_moja
        mov     rdi, 8
        call    andika_neno8_moja_kwa_moja
        mov     rdi, 24
        call    andika_neno8_moja_kwa_moja

        ; 6: .shstrtab — sh_name=45, SHT_STRTAB (3)
        mov     edi, 45
        call    andika_neno4_moja_kwa_moja
        mov     edi, 3
        call    andika_neno4_moja_kwa_moja
        mov     rdi, 0
        call    andika_neno8_moja_kwa_moja
        mov     rdi, 0
        call    andika_neno8_moja_kwa_moja
        ; sh_offset = baada ya rela.text
        mov     edi, 64
        add     edi, [text_buf_pos]
        add     edi, [data_buf_pos]
        add     edi, r14d               ; symtab_size tuliyohifadhi
        push    rdi                     ; Hifadhi offset iliyokokotolewa
        push    rax
        call    toa_elf_hesabu_strtab
        pop     rcx
        pop     rdi                     ; Rejesha offset iliyokokotolewa
        add     edi, eax                ; + strtab_size
        mov     rcx, [rela_count]
        imul    rcx, 24
        add     edi, ecx                ; + rela_size
        call    andika_neno8_moja_kwa_moja
        mov     edi, SHSTRTAB_SIZE
        call    andika_neno8_moja_kwa_moja
        mov     edi, 0
        call    andika_neno4_moja_kwa_moja
        mov     edi, 0
        call    andika_neno4_moja_kwa_moja
        mov     rdi, 1
        call    andika_neno8_moja_kwa_moja
        mov     rdi, 0
        call    andika_neno8_moja_kwa_moja

        ret
