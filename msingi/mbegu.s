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
%define TEXT_BUF_SIZE     262144     ; 256 KB ya msimbo wa .text
%define DATA_BUF_SIZE     4096       ; 4 KB ya data ya ulimwengu
%define MAX_LABELS        16384      ; upeo wa lebo
%define MAX_EXTERNS       16384      ; upeo wa alama za nje
%define MAX_RELOCS        16384      ; upeo wa marekebisho
%define MAX_GLOBALS       512        ; upeo wa vigezo vya ulimwengu
%define STR_POOL_SIZE     262144     ; bwawa la herufi (256 KB kwa faili kubwa)
%define MAX_LOCALS        512        ; upeo wa vigezo vya ndani kwa kazi

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
%define TOK_HERUFI        19         ; mfuatano wa herufi ("...")
%define TOK_SWALI         20         ; ?

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
%define AST_HALISI_D       46          ; desimali halisi (D64)
%define AST_ENDELEA        44
%define AST_HASILI         45

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
%define OP_NA_BITI         17         ; & (NA ya biti, binary)
%define OP_AU_BITI         18         ; | (AU ya biti, binary)
%define OP_XOR_BITI        19         ; ^ (XOR ya biti)
%define OP_HUU             20         ; ?: (uchaguzi)

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
msg_assignerr:  db "Hitilafu: uwekaji usiotumika", 10, 0
msg_databuf:    db "Hitilafu: data_buf imejaa (sret)", 10, 0
msg_hoja9:      db "Hitilafu: wito wenye hoja zaidi ya 9", 10, 0
msg_main_kukosa: db "Hitilafu: main haipo", 10, 0
msg_rela_full:   db "Hitilafu: jedwali la RELA limejaa", 10, 0
; Nguvu za kumi (10^0 .. 10^17) — kigeuzi cha desimali → double
nguvu_za_kumi:
        dq 1.0
        dq 10.0
        dq 100.0
        dq 1000.0
        dq 10000.0
        dq 100000.0
        dq 1000000.0
        dq 10000000.0
        dq 100000000.0
        dq 1000000000.0
        dq 10000000000.0
        dq 100000000000.0
        dq 1000000000000.0
        dq 10000000000000.0
        dq 100000000000000.0
        dq 1000000000000000.0
        dq 10000000000000000.0
        dq 100000000000000000.0

msg_extern_full: db "Hitilafu: jedwali la nje limejaa", 10, 0
msg_kazi_kukosa: db "Hitilafu: kazi haijafafanuliwa: ", 0
msg_d64_wito:    db "Hitilafu: D64 kwenye wito wa kazi haisaidiwi bado na mbegu — tumia mkusanyaji wa .swa", 10, 0
msg_mstari_mpya: db 10, 0
msg_fixup_full:  db "Hitilafu: jedwali la fixup limejaa", 10, 0
msg_global_full: db "Hitilafu: jedwali la ulimwengu limejaa", 10, 0
msg_label_full:  db "Hitilafu: jedwali la lebo limejaa", 10, 0
msg_chanzo_kikubwa: db "Hitilafu: chanzo ni kikubwa mno", 10, 0
msg_herufi_baya: db "Hitilafu: herufi isiyojulikana", 10, 0
msg_tokeni_jaa: db "Hitilafu: chanzo kina tokeni nyingi mno", 10, 0
msg_ast_full:    db "Hitilafu: jedwali la AST limejaa", 10, 0
msg_local_full:  db "Hitilafu: jedwali la vigezo vya ndani limejaa", 10, 0
msg_muundo_full: db "Hitilafu: jedwali la miundo limejaa", 10, 0
msg_nyuga_full:  db "Hitilafu: jedwali la nyuga limejaa", 10, 0
msg_kaziret_full: db "Hitilafu: jedwali la kazi-sret limejaa", 10, 0
msg_genlabel_full: db "Hitilafu: jedwali la lebo-za-gen limejaa", 10, 0
msg_strpool_full: db "Hitilafu: bwawa la herufi limejaa", 10, 0
msg_textbuf_full: db "Hitilafu: bafa la .text limejaa", 10, 0
msg_breakfix_full: db "Hitilafu: jedwali la fixup-za-vunja limejaa", 10, 0

; ---------- Majina maalum kwa hali ya exe ----------
jina_exe_flag:  db "--exe", 0
jina_wito_mfumo: db "wito_wa_mfumo", 0
jina_tekeleza:  db "tekeleza", 0
jina_main:      db "main", 0

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
tn_d64:         db "D64", 0
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
token_desimali:  resb MAX_TOKENS         ; 1 = tokeni ni desimali (D64)
token_line:     resd MAX_TOKENS
token_col:      resw MAX_TOKENS
token_len:      resw MAX_TOKENS
token_count:    resq 1
token_pos:      resq 1                  ; nafasi ya sasa ya usomaji wa tokeni
line_sasa:      resq 1                  ; mstari wa sasa (1-msingi) kwa mchanganyaji

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
rela_addend:    resd MAX_RELOCS         ; addend maalum (kwa rekebisho la data)
rela_count:     resq 1

; ---------- Vigezo vya ulimwengu ----------
global_name:    resq MAX_GLOBALS
global_offset:  resd MAX_GLOBALS
global_size:    resd MAX_GLOBALS
global_is_bss:  resd MAX_GLOBALS         ; 1 ikiwa ni .bss, 0 ikiwa ni .data
global_base_type: resd MAX_GLOBALS       ; aina msingi: N8=1, N16=2, N32=3, N64=4, W0=5, muundo=6+
global_star_count: resd MAX_GLOBALS      ; idadi ya nyota: 0=kawaida, 1=nyota moja, 2=nyota mbili, n.k.
global_is_array: resd MAX_GLOBALS        ; 1 ikiwa ni safu, 0 ikiwa ni kigeu rahisi
global_count:   resq 1
bss_size:       resq 1                   ; ukubwa wa jumla wa .bss

; ---------- Vigezo vya ndani (kwa kazi ya sasa) ----------
local_name:     resq MAX_LOCALS         ; anwani ya jina
local_offset:   resd MAX_LOCALS         ; ofseti ya rafu (kutoka rbp, hasi)
local_size:     resd MAX_LOCALS         ; ukubwa kwa baiti
local_base_type: resd MAX_LOCALS        ; aina msingi: N8=1, N16=2, N32=3, N64=4, W0=5, muundo=6+
local_star_count: resd MAX_LOCALS       ; idadi ya nyota: 0=kawaida, 1=nyota moja, 2=nyota mbili, n.k.
local_count:    resq 1

; ---------- Kina cha mzunguko ----------
loop_depth:     resq 1
loop_break_label: resq 16               ; lebo za vunja (ufungwaji wa mipaka ya rafu)
break_fixup_pos: resd 65536              ; nafasi za marekebisho ya vunja
cl_hatua:        resq 1                  ; nafasi ya hatua ya kwa (semantiki ya C ya endelea)
cl_lengo:        resq 1                  ; lengo la sasa la endelea
break_fixup_count: resq 1               ; idadi ya marekebisho ya vunja

; ---------- Hali ya mkusanyaji ----------
compiler_state: resq 1                  ; 0=sawa, 1=kosa
compiler_error_msg: resq 1              ; ujumbe wa kosa
muundo_jina:     resd 1                 ; ofseti ya jina la muundo (kwa aina za mtumiaji)
exe_mode:        resb 1                 ; 1 = toa ET_EXEC badala ya .o
tmp_argc:        resq 1                 ; hifadhi ya argc wakati wa kuchanganua hoja

; ---------- Jedwali la miundo ----------
muundo_count:    resq 1                 ; idadi ya miundo
muundo_jina_off: resd 64                ; ofseti ya jina la muundo
muundo_ukubwa:   resd 64                ; ukubwa wa muundo kwa baiti
muundo_pangilio: resd 64                ; upangilio wa muundo
muundo_nyuga_anza: resd 64              ; nyuga ya kwanza
muundo_nyuga_mwisho: resd 64            ; nyuga ya mwisho (isiyojumuishwa)

; ---------- Jedwali la nyuga ----------
nyuga_count:     resq 1                 ; idadi ya nyuga
nyuga_muundo:    resd 512               ; faharisi ya muundo wa nyuga
nyuga_jina_off:  resd 512               ; ofseti ya jina la nyuga
nyuga_aina:      resd 512               ; aina msingi ya nyuga (1-6)
nyuga_muundo_id: resd 512               ; faharisi ya muundo ndani au -1
nyuga_nyota:     resd 512               ; idadi ya nyota
nyuga_ofseti:    resd 512               ; ofseti ya nyuga ndani ya muundo

; ---------- Viwezeshaji vya muundo kwa vigezo vya ndani ----------
local_muundo_id: resd MAX_LOCALS       ; faharisi ya muundo wa kigezo au -1
local_array_size: resd MAX_LOCALS      ; idadi ya elementi za safu ya ndani au 0
frame_wapi:      resq 1                 ; ofseti inayofuata ya fremu ya rafu

; ---------- Habari ya kurejesha muundo ----------
kazi_ret_aina:   resd 1                 ; aina ya kurejesha ya kazi ya sasa (uzalishaji)
kazi_ret_muundo_id: resd 1              ; faharisi ya muundo unaorudishwa au -1 (uzalishaji)
kazi_ret_jina:   resd 256               ; ofseti ya jina la kazi inayorudisha muundo
kazi_ret_muundo_jina: resd 256          ; ofseti ya jina la muundo unaorudishwa
kazi_ret_idadi:  resq 1                 ; idadi ya kazi zinazorudisha muundo

; ---------- Marekebisho ya endelea ----------
continue_fixup_pos: resd 65536          ; nafasi za marekebisho ya endelea
continue_fixup_count: resq 1            ; idadi ya marekebisho ya endelea

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
        lea     rdi, [msg_textbuf_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

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
        lea     rdi, [msg_textbuf_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

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
        lea     rdi, [msg_textbuf_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

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
        ; Soma hadi EOF au bafa lijae. Kusoma mara moja tu hakutoshi
        ; kwa stdin (bomba): sys_read inarudisha tu kile kilichopo
        ; wakati huo — chanzo kinakatwa bila mpangilio. Huu ulikuwa
        ; chanzo cha kutokubalika kwa mbegu: matokeo tofauti kwa
        ; pembejeo moja (kazi za mwisho wa faili zikipotea).
        push    rbx
        push    r12
        push    r13
        mov     rbx, rsi                ; nafasi ya sasa kwenye bafa
        mov     r13, rdx                ; upeo wa ukubwa
        xor     r12, r12                ; jumla ya baiti zilizosomwa
.loop:
        cmp     r12, r13
        jae     .done                   ; bafa limejaa — acha (mlio utafuata)
        mov     rsi, rbx
        mov     rdx, r13
        sub     rdx, r12
        mov     rax, 0                  ; sys_read
        syscall
        cmp     rax, 0
        je      .done                   ; EOF
        jl      .error                  ; kosa la syscall — rudisha -1
        add     r12, rax
        add     rbx, rax
        jmp     .loop
.done:
        mov     rax, r12
        pop     r13
        pop     r12
        pop     rbx
        ret
.error:
        mov     rax, -1
        pop     r13
        pop     r12
        pop     rbx
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
        cmp     rax, MAX_SOURCE
        jae     .kikubwa_mno
        ret
.kikubwa_mno:
        lea     rdi, [msg_chanzo_kikubwa]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit
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

        ; Soma faili lote (hadi EOF) — si kusoma mara moja tu
        mov     rdi, rax
        lea     rsi, [source_buf]
        mov     rdx, MAX_SOURCE
        call    sys_read_all
        mov     [source_len], rax
        cmp     rax, 0
        jl      .error
        cmp     rax, MAX_SOURCE
        jae     .kikubwa_mno

        pop     r12
        ret
.kikubwa_mno:
        lea     rdi, [msg_chanzo_kikubwa]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit
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
        inc     qword [line_sasa]       ; mstari mpya
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
        cmp     al, 10
        jne     .skip_no_inc_line
        inc     qword [line_sasa]        ; mstari mpya
.skip_no_inc_line:
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
        jae     .angalia_desimali
        mov     al, [r13 + r12]
        call    ni_tarakimu
        cmp     eax, 1
        jne     .angalia_desimali
        movzx   ecx, byte [r13 + r12]   ; soma tena baada ya ni_tarakimu kuharibu al
        sub     ecx, '0'
        imul    rbx, rbx, 10
        add     rbx, rcx
        inc     r12
        jmp     .loop
.angalia_desimali:
        ; Angalia desimali: '.' ikifuatiwa na tarakimu
        cmp     r12, r14
        jae     .done
        mov     al, [r13 + r12]
        cmp     al, '.'
        jne     .done
        mov     r10, r12
        inc     r10
        cmp     r10, r14
        jae     .done
        mov     al, [r13 + r10]
        call    ni_tarakimu
        cmp     eax, 1
        jne     .done

        ; Desimali: changanua sehemu ndogo na ubadilishe hadi double
        inc     r12                     ; ruka '.'
        xor     ecx, ecx                ; idadi ya tarakimu ndogo
        xor     edx, edx                ; thamani ya sehemu ndogo
.frac_loop:
        cmp     r12, r14
        jae     .frac_badilisha
        mov     al, [r13 + r12]
        call    ni_tarakimu
        cmp     eax, 1
        jne     .frac_badilisha
        movzx   eax, byte [r13 + r12]
        sub     eax, '0'
        imul    rdx, rdx, 10
        add     rdx, rax
        inc     rcx
        inc     r12
        cmp     ecx, 17                 ; usahihi wa juu wa double
        jb      .frac_loop
.frac_badilisha:
        ; double = (double)rbx + (double)rdx / 10^rcx  (SSE2 ya mbegu)
        cvtsi2sd xmm0, rbx
        cvtsi2sd xmm1, rdx
        mov     rax, rcx
        lea     r10, [nguvu_za_kumi]
        divsd   xmm1, [r10 + rax*8]
        addsd   xmm0, xmm1
        movq    rbx, xmm0
        mov     eax, TOK_NAMBARI
        mov     edx, 1                  ; alama: hii ni desimali
        jmp     .ret
.done:
        mov     eax, TOK_NAMBARI
        xor     edx, edx                ; si desimali
        jmp     .ret
.fail:
        mov     eax, 0
        xor     ebx, ebx
        xor     edx, edx
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
        ; Angalia ikiwa ni &&
        cmp     r12, r14
        jae     .is_and_addr
        mov     al, [r13 + r12]
        cmp     al, '&'
        je      .logical_and
.is_and_addr:
        mov     eax, TOK_ALAMA
        xor     ebx, ebx
        jmp     .ret
.logical_and:
        inc     r12
        mov     eax, TOK_ISHARA
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
        mov     ebx, OP_AU_BITI
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
        mov     qword [line_sasa], 1         ; mstari wa kwanza

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
        jae     .tokeni_jaa

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
        mov     r15, [token_count]
        mov     [token_desimali + r15], dl   ; alama ya desimali
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
        mov     edx, [line_sasa]        ; maneno pia lazima yawe na mstari
        mov     [token_line + r15*4], edx

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
        cmp     al, '"'
        je      .tok_string
        cmp     al, '?'
        je      .tok_question

        ; Herufi isiyojulikana — kosa LAUTI, si ruka kimya
        lea     rdi, [msg_herufi_baya]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

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
.tok_question:
        mov     eax, TOK_SWALI
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

.tok_string:
        ; Ruka '"' ya kufungua
        inc     r12
        ; Hifadhi ofseti ya kwanza kwenye bwawa la herufi
        mov     r8, [str_pool_pos]
        mov     r9d, r8d                       ; ofseti ya kwanza ya mfuatano
        xor     r10d, r10d                     ; hesabu ya baiti

.str_loop:
        cmp     r12, r14
        jae     .str_eof                       ; mwisho wa faili bila '"'
        mov     al, [r13 + r12]
        cmp     al, '"'
        je      .str_done
        cmp     al, '\'
        je      .str_escape
        cmp     al, 10
        je      .str_eof                       ; newline kwenye mfuatano

        ; Herufi ya kawaida
        mov     rdi, [str_pool_pos]
        cmp     rdi, STR_POOL_SIZE - 2
        jae     .str_overflow
        mov     [str_pool + rdi], al
        inc     qword [str_pool_pos]
        inc     r12
        inc     r10d
        jmp     .str_loop

.str_escape:
        inc     r12
        cmp     r12, r14
        jae     .str_eof
        mov     al, [r13 + r12]
        cmp     al, 'n'
        je      .esc_newline
        cmp     al, 't'
        je      .esc_tab
        cmp     al, '\'
        je      .esc_backslash
        cmp     al, '"'
        je      .esc_quote
        cmp     al, '0'
        je      .esc_null
        ; Herufi isiyotambulika baada ya '\' — tumia kama ilivyo
        jmp     .str_store_char

.esc_newline:
        mov     al, 10
        jmp     .str_store_char
.esc_tab:
        mov     al, 9
        jmp     .str_store_char
.esc_backslash:
        mov     al, '\'
        jmp     .str_store_char
.esc_quote:
        mov     al, '"'
        jmp     .str_store_char
.esc_null:
        mov     al, 0

.str_store_char:
        mov     rdi, [str_pool_pos]
        cmp     rdi, STR_POOL_SIZE - 2
        jae     .str_overflow
        mov     [str_pool + rdi], al
        inc     qword [str_pool_pos]
        inc     r12
        inc     r10d
        jmp     .str_loop

.str_done:
        inc     r12                             ; ruka '"' ya kufunga
        ; Ongeza '\0' mwishoni
        mov     rdi, [str_pool_pos]
        cmp     rdi, STR_POOL_SIZE - 2
        jae     .str_overflow
        mov     byte [str_pool + rdi], 0
        inc     qword [str_pool_pos]
        inc     r10d                            ; urefu pamoja na '\0'

        ; Andika tokeni moja kwa moja (token_len != 1)
        mov     r15, [token_count]
        cmp     r15, MAX_TOKENS - 1
        jae     .tokeni_jaa
        mov     dword [token_ty + r15*4], TOK_HERUFI
        mov     [token_val + r15*8], r9         ; ofseti ya str_pool
        mov     edx, [line_sasa]
        mov     [token_line + r15*4], edx
        mov     [token_len + r15*2], r10w       ; urefu wa baiti (pamoja na '\0')
        inc     qword [token_count]
        jmp     .changanua_loop

.str_eof:
.str_overflow:
        ; Kosa LAUTI — kuruka kimya ni uharibifu
        lea     rdi, [msg_strpool_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

.hifadhi_tokeni_kwa_ishara:
        mov     r15, [token_count]
        cmp     r15, MAX_TOKENS - 1
        jae     .tokeni_jaa
        mov     [token_ty + r15*4], eax
        mov     [token_val + r15*8], rbx
        mov     edx, [line_sasa]
        mov     [token_line + r15*4], edx
        mov     word [token_len + r15*2], 1
        inc     qword [token_count]
        jmp     .changanua_loop

.hifadhi_tokeni:
        mov     r15, [token_count]
        cmp     r15, MAX_TOKENS - 1
        jae     .tokeni_jaa
        mov     [token_ty + r15*4], eax
        mov     [token_val + r15*8], rbx
        mov     edx, [line_sasa]
        mov     [token_line + r15*4], edx
        mov     word [token_len + r15*2], 1
        inc     qword [token_count]
        jmp     .changanua_loop

.tokeni_jaa:
        ; Kosa LAUTI — kukata chanzo kimya ni uharibifu
        lea     rdi, [msg_tokeni_jaa]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

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
        ; Kosa LAUTI — kurudisha -1 ("hakuna nodi") kimya ni uharibifu:
        ; mchanganuzi ungeendelea na mti usio sahihi
        lea     rdi, [msg_ast_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

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

        lea     rdi, [tn_d64]
        call    tarajia_neno
        cmp     eax, 1
        je      .is_d64

        ; Jaribu aina ya muundo (jina la mtumiaji)
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NENO
        jne     .unknown

        ; Ni jina la muundo — hifadhi na urudi aina=6
        mov     rsi, [token_text + rdi*8]
        movzx   ecx, word [token_len + rdi*2]
        push    rdi
        call    hifadhi_jina
        pop     rdi
        mov     [muundo_jina], eax        ; hifadhi kwa matumizi ya baadaye
        inc     qword [token_pos]          ; tumia jina la muundo
        mov     eax, 6                     ; aina = muundo
        xor     ebx, ebx
        jmp     .check_star

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
.is_d64:
        mov     eax, 7                  ; D64 — desimali (double)
        xor     ebx, ebx
        jmp     .check_star

.check_star:
        ; Angalia ikiwa inafuatiwa na * (moja au zaidi)
        push    rax
        xor     ebx, ebx
.star_loop:
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NYOTA
        jne     .star_done
        inc     qword [token_pos]       ; tumia nyota
        inc     ebx                     ; ongeza hesabu ya nyota
        jmp     .star_loop
.star_done:
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
;   edi = nambari ya aina (1=N8, 2=N16, 3=N32, 4=N64, 5=W0, 6=muundo, 7=D64)
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
        cmp     edi, 7                  ; D64
        je      .size8
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
; tafuta_muundo: tafuta muundo kwa ofseti ya jina
;   edi = ofseti ya jina kwenye bwawa la herufi
;   rax = faharisi ya muundo au -1
; -------------------------------------------------------
tafuta_muundo:
        ; Linganisha jina kwa HERUFI, si kwa ofseti:
        ; hifadhi_jina hairudishi nakala rudufu, kwa hiyo ofseti za
        ; jina moja zinaweza kutofautiana kati ya matangazo.
        push    rcx
        push    rdi
        push    rsi
        push    rdx
        mov     edi, edi                ; sufuri juu ya rdi (ofseti ni N32)
        lea     rsi, [str_pool + rdi]   ; jina linalotafutwa
        xor     edx, edx
        mov     rcx, [muundo_count]
        test    rcx, rcx
        jz      .not_found
.tm_loop:
        mov     edi, [muundo_jina_off + rdx*4]
        lea     rdi, [str_pool + rdi]   ; jina la muundo la jedwali
        call    linganisha_mfuatano     ; eax = 0 ikiwa sawa
        cmp     eax, 0
        je      .found
        inc     edx
        cmp     rdx, rcx
        jb      .tm_loop
.not_found:
        mov     edx, -1
.found:
        mov     eax, edx
        pop     rdx
        pop     rsi
        pop     rdi
        pop     rcx
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

        ; Nambari halisi (kamili au desimali)
        mov     rdi, [token_pos]
        cmp     byte [token_desimali + rdi], 0
        jne     .nambari_desimali

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

.nambari_desimali:
        ; D64: biti 64 hugawanywa — lo32 kwenye ast_thamani, hi32 kwenye ast_tiga
        ; (mkongwe ule ule wa mnyororo wa .swa kwa AST_HALISI_D).
        mov     rdi, [token_pos]
        mov     r8d, AST_HALISI_D
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
        mov     [ast_thamani + rcx*4 - 4], edi          ; lo32
        shr     rdi, 32
        mov     [ast_tiga + rcx*4 - 4], edi             ; hi32
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
        ; Angalia kama ni mfuatano wa herufi
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_HERUFI
        jne     .fail
        ; Unda nodi ya AST_KAULI yenye kushoto=-1 kama kiashiria cha mfuatano
        mov     rcx, [ast_count]
        cmp     rcx, MAX_AST_NODES - 1
        jae     .fail
        mov     dword [ast_aina + rcx*4], AST_KAULI
        mov     dword [ast_kushoto + rcx*4], -1        ; kiashiria: si operesheni binary
        mov     dword [ast_kulia + rcx*4], -1
        mov     dword [ast_tiga + rcx*4], -1
        mov     dword [ast_nne + rcx*4], -1
        ; Hifadhi ofseti ya str_pool na urefu wa baiti
        mov     r8, [token_val + rdi*8]
        mov     [ast_jina_off + rcx*4], r8d            ; ofseti ya str_pool
        movzx   r8d, word [token_len + rdi*2]
        mov     [ast_thamani + rcx*4], r8d             ; urefu (pamoja na '\0')
        inc     qword [ast_count]
        inc     qword [token_pos]
        mov     eax, ecx
        jmp     .done
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
        mov     r8d, AST_HASILI
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
        je      .prec_6
        cmp     edi, OP_GAWANYA
        je      .prec_6
        cmp     edi, OP_MODULO
        je      .prec_6
        cmp     edi, OP_JUMLISHA
        je      .prec_5
        cmp     edi, OP_TOA
        je      .prec_5
        cmp     edi, OP_HAMISHA_KUSHOTO
        je      .prec_4
        cmp     edi, OP_HAMISHA_KULIA
        je      .prec_4
        cmp     edi, OP_KIDOGO
        je      .prec_3
        cmp     edi, OP_KUBWA
        je      .prec_3
        cmp     edi, OP_KIDOGO_SAWA
        je      .prec_3
        cmp     edi, OP_KUBWA_SAWA
        je      .prec_3
        cmp     edi, OP_SAWA
        je      .prec_2
        cmp     edi, OP_SAWA_SAWA
        je      .prec_2
        cmp     edi, OP_SIO_SAWA
        je      .prec_2
        cmp     edi, OP_NA
        je      .prec_1
        cmp     edi, OP_AU
        je      .prec_1
        cmp     edi, OP_NA_BITI
        je      .prec_1
        cmp     edi, OP_AU_BITI
        je      .prec_1
        cmp     edi, OP_XOR_BITI
        je      .prec_1
        mov     eax, 0
        ret
; Viwango vinaanza kutoka 1 kwa sababu changanua_usemi_na_utangulizi
; hutumia 0 kama ishara ya "acha kuchunguza" (cmp eax, 0 / je .done).
; Kwa hiyo && na || zinabaki kwenye 1, chini ya ulinganisho (2),
; kulinganisha (3), uhamishaji (4), kujumlisha (5), na kuzidisha (6).
.prec_6:
        mov     eax, 6
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
        jne     .check_alama
        mov     ebx, OP_SAWA
        jmp     .got_op

.check_alama:
        ; & kati ya vielezi — NA ya biti (binary)
        cmp     dword [token_ty + rdi*4], TOK_ALAMA
        jne     .check_swali
        mov     ebx, OP_NA_BITI
        jmp     .got_op

.check_swali:
        ; ?: — uchaguzi (ternary), kwenye kiwango cha juu pekee
        cmp     dword [token_ty + rdi*4], TOK_SWALI
        jne     .done
        mov     r14d, [rsp]             ; utangulizi wa chini
        cmp     r14d, 0
        jne     .done
        inc     qword [token_pos]       ; tumia ?

        ; Changanua upande wa kweli
        push    r12                     ; hifadhi sharti
        mov     edi, 0
        call    changanua_usemi_na_utangulizi
        mov     r10d, eax               ; nodi ya kweli
        pop     r8                      ; sharti
        cmp     r10d, -1
        je      .fail

        ; Tarajia :
        push    r8
        push    r10
        mov     edi, TOK_JUWILI
        call    tarajia_ishara
        pop     r10
        pop     r8
        cmp     eax, 0
        je      .fail

        ; Changanua upande wa uwongo
        push    r8
        push    r10
        mov     edi, 0
        call    changanua_usemi_na_utangulizi
        mov     r11d, eax               ; nodi ya uwongo
        pop     r10
        pop     r8
        cmp     r11d, -1
        je      .fail

        ; Unda nodi ya mfuatano (kweli, uwongo)
        push    r8                      ; sharti
        push    r10                     ; kweli
        push    r11                     ; uwongo
        mov     r8d, AST_MFUATANO
        pop     r11                     ; thamani = uwongo
        pop     r9                      ; kushoto = kweli
        pop     r10                     ; sharti (rudisha baadaye)
        push    r10                     ; rudisha sharti kwenye stack
        mov     r10d, r11d              ; kulia = uwongo
        push    r11
        call    ast_nodi_mpya
        pop     r11
        mov     rcx, [ast_count]
        mov     [ast_thamani + rcx*4 - 4], r11d
        mov     r13d, eax               ; nodi ya mfuatano

        ; Unda nodi ya ternary: kushoto = sharti, kulia = mfuatano
        push    r13                     ; mfuatano
        mov     r8d, AST_KAULI
        pop     r10                     ; kulia = mfuatano
        pop     r9                      ; kushoto = sharti
        mov     r11d, OP_HUU
        push    r11
        call    ast_nodi_mpya
        pop     r11
        mov     rcx, [ast_count]
        mov     [ast_thamani + rcx*4 - 4], r11d
        mov     r12d, eax               ; nodi ya ternary
        jmp     .loop

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

        ; --- Anzisha: tangazo/usemi au tupu (;) ---
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NUKTA_MKATO
        je      .kwa_init_tupu
        call    changanua_taarifa       ; tangazo au usemi (hutumia ;)
        mov     r13d, eax
        jmp     .kwa_hali
.kwa_init_tupu:
        inc     qword [token_pos]
        mov     r13d, -1

.kwa_hali:
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NUKTA_MKATO
        je      .kwa_hali_tupu
        call    changanua_usemi
        mov     r14d, eax
        mov     edi, TOK_NUKTA_MKATO
        call    tarajia_ishara
        jmp     .kwa_hatua
.kwa_hali_tupu:
        inc     qword [token_pos]
        mov     r14d, -1

.kwa_hatua:
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_MABANO_FUNGA
        je      .kwa_hatua_tupu
        call    changanua_usemi
        mov     r15d, eax
        mov     edi, TOK_MABANO_FUNGA
        call    tarajia_ishara
        jmp     .kwa_mwili
.kwa_hatua_tupu:
        inc     qword [token_pos]
        mov     r15d, -1

.kwa_mwili:
        call    changanua_block
        mov     r9d, eax                ; mwili asili (au -1)

        ; Funga mwili kwenye AST_BLOCK (mnyororo uko kwenye ast_kushoto)
        mov     r10d, -1
        mov     r11d, -1
        mov     r8d, AST_BLOCK
        call    ast_nodi_mpya
        mov     r8d, eax
        mov     [ast_kushoto + r8*4], r9d
        cmp     r15d, -1
        je      .kwa_hakuna_hatua_p

        ; Funga hatua kwenye block-mini yenye alama -777777 kwenye
        ; ast_kulia (kulia ya block huwa -1 kamwe — hakuna mgongano).
        ; Uzalishaji_block hurekodi nafasi yake kwa ajili ya endelea
        ; (semantiki ya C: endelea inaruka kwenye hatua).
        push    r8                      ; hifadhi wrapper
        mov     r9d, r15d               ; hatua
        mov     r10d, -1
        mov     r11d, -1
        mov     r8d, AST_BLOCK
        call    ast_nodi_mpya
        mov     r15d, eax               ; r15 = block-mini
        mov     [ast_kushoto + r15*4], r9d
        mov     dword [ast_kulia + r15*4], -777777
        pop     r8                      ; r8 = wrapper
        mov     dword [ast_kulia + r8*4], -777777  ; alama ya has_step

        ; Ambatisha block-mini mwishoni mwa mnyororo
        mov     r9d, [ast_kushoto + r8*4]
        cmp     r9d, -1
        jne     .kwa_mwisho_mnyororo
        mov     [ast_kushoto + r8*4], r15d
        jmp     .kwa_hakuna_hatua_p
.kwa_mwisho_mnyororo:
        mov     r10d, [ast_nne + r9*4]
        cmp     r10d, -1
        je      .kwa_ambatisha_sasa
        mov     r9d, r10d
        jmp     .kwa_mwisho_mnyororo
.kwa_ambatisha_sasa:
        mov     [ast_nne + r9*4], r15d
.kwa_hakuna_hatua_p:

        ; AST_WAKATI: kushoto=hali, kulia=mwili, tiga=anzisha
        mov     r9d, r14d
        mov     r10d, r8d
        mov     r11d, r13d
        mov     r8d, AST_WAKATI
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
        ; Angalia ikiwa ni "endelea"
        lea     rdx, [kw_endelea]
        call    linganisha_neno_muhimu
        cmp     eax, 0
        jne     .not_endelea

        inc     qword [token_pos]       ; tumia "endelea"
        mov     r8d, AST_ENDELEA
        mov     r9d, -1
        mov     r10d, -1
        mov     r11d, -1
        call    ast_nodi_mpya
        jmp     .expect_semicolon

.not_endelea:
        ; Labda ni tangazo la kigezo (N32 jina = ...) au usemi (wito wa kazi, n.k.)
        ; Jaribu kuchanganua kama tangazo la aina kwanza
        ; Hifadhi nafasi ya token — ikiwa tangazo litashindwa, rejesha na jaribu usemi
        push    qword [token_pos]
        call    changanua_aina
        cmp     eax, 0
        je      .not_decl_pop

        ; Tulipata aina! eax = nambari ya aina, ebx = nyota
        mov     r12d, eax               ; hifadhi aina
        mov     r13d, ebx               ; hifadhi nyota

        ; Hifadhi jina la muundo ndani kwenye rafu (ikiwa aina ni muundo)
        sub     rsp, 8
        mov     qword [rsp], -1
        cmp     r12d, 6
        jne     .tangazo_sio_muundo
        mov     eax, [muundo_jina]
        mov     [rsp], rax
.tangazo_sio_muundo:

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

        ; Angalia ikiwa ni tangazo la safu: N32 jina[N];
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_MABANO_MKOA
        je      .tangazo_safu

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
        ; tiga = ofseti ya jina la muundo (au -1)
        pop     r11                     ; ofseti ya muundo kutoka rafu
        mov     r8d, AST_TANGAZO
        mov     r9d, r15d               ; kushoto = kianzilishi
        mov     r10d, r13d              ; kulia = nyota
        push    r11                     ; hifadhi ofseti ya muundo
        mov     r11d, -1                ; tiga haitumiki
        call    ast_nodi_mpya
        pop     r11                     ; rejesha ofseti ya muundo
        mov     rcx, [ast_count]
        mov     [ast_thamani + rcx*4 - 4], r12d  ; aina
        mov     [ast_jina_off + rcx*4 - 4], r14d ; jina
        mov     [ast_tiga + rcx*4 - 4], r11d     ; muundo (au -1)
        add     rsp, 8                  ; toa token_pos bila kubadilisha eax
        jmp     .expect_semicolon

        ; Njia ya safu ya ndani: N32 jina[N];
.tangazo_safu:
        cmp     r12d, 6                 ; muundo hauwezi kuwa safu ya ndani
        je      .decl_error
        inc     qword [token_pos]       ; ruka [
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NAMBARI
        jne     .decl_error
        mov     r15, [token_val + rdi*8] ; idadi ya elementi
        test    r15, r15
        jz      .decl_error             ; elementi 0 si halali
        inc     qword [token_pos]       ; ruka nambari
        mov     edi, TOK_MABANO_MKOA_FUNGA
        call    tarajia_ishara          ; haibadilishi r15
        cmp     eax, 1
        jne     .decl_error
        neg     r15                     ; hasi inaashiria safu ya ndani (tiga)
        ; Unda nodi ya AST_TANGAZO: tiga = -N, hakuna kianzilishi
        pop     r11                     ; toa ofseti ya muundo (safu si muundo)
        mov     r8d, AST_TANGAZO
        mov     r9d, -1                 ; kushoto = hakuna kianzilishi
        mov     r10d, r13d              ; kulia = nyota
        push    r15                     ; hifadhi -N (ast_nodi_mpya inaharibu r11)
        mov     r11d, -1                ; thamani haitumiki
        call    ast_nodi_mpya
        pop     r11                     ; rejesha -N
        mov     rcx, [ast_count]
        mov     [ast_thamani + rcx*4 - 4], r12d  ; aina
        mov     [ast_jina_off + rcx*4 - 4], r14d ; jina
        mov     [ast_tiga + rcx*4 - 4], r11d     ; -N: safu ya ndani
        add     rsp, 8                  ; toa token_pos bila kubadilisha eax
        jmp     .expect_semicolon

.decl_error:
        ; Sio tangazo — rejesha nafasi ya token na jaribu kama usemi
        pop     rax                     ; toa ofseti ya muundo
        pop     rax
        mov     [token_pos], rax
        jmp     .not_decl

.not_decl_pop:
        pop     rax                     ; toa token_pos iliyohifadhiwa
        jmp     .not_decl

.not_decl:
        ; Sio aina — changanua kama usemi
        call    changanua_usemi
        cmp     eax, -1
        je      .fail
        jmp     .expect_semicolon

.try_semicolon:
        cmp     edi, TOK_NUKTA_MKATO
        je      .empty_statement
        jmp     .not_decl               ; si neno wala ; — jaribu kama usemi (*w = ..., n.k.)
.empty_statement:
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
        ; EOF kabla ya } — mabano yasiyofungwa ni kosa, si kukubalika
        cmp     dword [token_ty + rdi*4], TOK_MWISHO
        je      .block_eof_kosa
        ; Kinga ya mipaka ya mkondo wa tokeni
        cmp     rdi, [token_count]
        jae     .block_eof_kosa

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

.block_eof_kosa:
        ; Mwisho wa faili ndani ya block isiyofungwa
        lea     rdi, [msg_parseerr]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

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

        ; Hifadhi jina la muundo ndani kwenye rafu (ikiwa aina ni muundo)
        sub     rsp, 8
        mov     qword [rsp], -1
        cmp     r15d, 6
        jne     .param_sio_muundo
        mov     eax, [muundo_jina]
        mov     [rsp], rax
.param_sio_muundo:

        ; Changanua jina la kigezo
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NENO
        jne     .param_jina_hapana
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
        pop     rdx                     ; ofseti ya muundo au -1
        mov     [ast_kushoto + rcx*4 - 4], edx

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

.param_jina_hapana:
        add     rsp, 8                  ; toa ofseti ya muundo
        jmp     .done

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

        ; Nafasi ya muundo wa kurudi (sret) — chaguo-msingi: -1
        sub     rsp, 8
        mov     qword [rsp], -1

        ; Changanua aina ya kurudi
        call    changanua_aina
        cmp     eax, 0
        je      .fail_no_type
        mov     r12d, eax               ; aina ya kurudi
        ; rbx ina nyota

        ; Ikiwa aina ni muundo (na hakuna nyota), hifadhi jina lake
        test    ebx, ebx
        jnz     .kazi_sio_muundo_ret
        cmp     r12d, 6
        jne     .kazi_sio_muundo_ret
        mov     eax, [muundo_jina]
        mov     [rsp], rax
.kazi_sio_muundo_ret:

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
        mov     r11d, -1
        call    ast_nodi_mpya
        mov     rcx, [ast_count]
        mov     [ast_thamani + rcx*4 - 4], r12d  ; aina
        mov     [ast_jina_off + rcx*4 - 4], r15d ; jina
        mov     edx, [rsp]              ; ofseti ya muundo wa kurudi au -1
        mov     [ast_tiga + rcx*4 - 4], edx
        cmp     edx, -1
        je      .kazi_hakuna_ret_muundo
        ; Rekodi kazi inayorudisha muundo (kwa uzalishaji wa sret)
        ; Tahadhari: eax bado ina index ya nodi — usiiharibu!
        mov     r10, [kazi_ret_idadi]
        cmp     r10, 256
        jae     .kaziret_jaa
        mov     [kazi_ret_jina + r10*4], r15d
        mov     [kazi_ret_muundo_jina + r10*4], edx
        inc     qword [kazi_ret_idadi]
.kazi_hakuna_ret_muundo:
        jmp     .done
.kaziret_jaa:
        lea     rdi, [msg_kaziret_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

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
        mov     edx, [rsp]              ; ofseti ya muundo wa kurudi au -1
        mov     [ast_tiga + rcx*4 - 4], edx
        cmp     edx, -1
        je      .kazi_hakuna_ret_muundo_mbele
        ; Tahadhari: eax bado ina index ya nodi — usiiharibu!
        mov     r10, [kazi_ret_idadi]
        cmp     r10, 256
        jae     .kaziret_jaa
        mov     [kazi_ret_jina + r10*4], r15d
        mov     [kazi_ret_muundo_jina + r10*4], edx
        inc     qword [kazi_ret_idadi]
.kazi_hakuna_ret_muundo_mbele:
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
        add     rsp, 8                  ; toa nafasi ya muundo wa kurudi
        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; changanua_muundo: changanua tangazo la muundo
;   token_pos inaelekeza kwenye neno "muundo"
;   rax = 1 ikiwa ilifanikiwa, 0 ikiwa ilishindwa
; -------------------------------------------------------
changanua_muundo:
        push    r12
        push    r13
        push    r14
        push    r15

        ; Tumia neno "muundo"
        inc     qword [token_pos]

        ; Jina la muundo lazima liwe TOK_NENO
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NENO
        jne     .fail
        mov     rsi, [token_text + rdi*8]
        movzx   ecx, word [token_len + rdi*2]
        inc     qword [token_pos]
        call    hifadhi_jina
        mov     r12d, eax               ; ofseti ya jina la muundo

        ; Rekodi muundo kwenye jedwali (upeo: 64)
        mov     rax, [muundo_count]
        cmp     rax, 64
        jae     .muundo_jaa
        mov     [muundo_jina_off + rax*4], r12d
        mov     dword [muundo_ukubwa + rax*4], 0
        mov     dword [muundo_pangilio + rax*4], 0
        mov     ecx, [nyuga_count]
        mov     [muundo_nyuga_anza + rax*4], ecx
        inc     qword [muundo_count]

        ; Tarajia mabano ya mbele {
        mov     edi, TOK_FUNGO
        call    tarajia_ishara
        cmp     eax, 1
        jne     .fail

        ; Hali ya kupanga: ofseti inayoendelea na pangilio la juu
        mov     r13d, 0                 ; ofseti inayofuata
        mov     r15d, 0                 ; pangilio la juu

.nyuga_loop:
        ; Mwisho wa muundo?
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_FUNGA
        je      .mwisho
        ; EOF kabla ya } — muundo usiofungwa ni kosa
        cmp     dword [token_ty + rdi*4], TOK_MWISHO
        je      .nyuga_eof_kosa
        cmp     rdi, [token_count]
        jae     .nyuga_eof_kosa

        ; Changanua aina ya nyuga
        call    changanua_aina
        cmp     eax, 0
        je      .fail
        mov     r12d, eax               ; aina msingi (1-6)
        mov     r14d, ebx               ; nyota

        ; Hifadhi jina la muundo ndani kwenye rafu (ikiwa aina ni muundo)
        sub     rsp, 8
        mov     qword [rsp], -1
        cmp     r12d, 6
        jne     .nyuga_sio_muundo
        mov     eax, [muundo_jina]
        mov     [rsp], rax
.nyuga_sio_muundo:

        ; Jina la nyuga lazima liwe TOK_NENO
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NENO
        jne     .fail_pop
        mov     rsi, [token_text + rdi*8]
        movzx   ecx, word [token_len + rdi*2]
        inc     qword [token_pos]

        ; Hifadhi jina la nyuga (hifadhi_jina hufuta rejista)
        push    r12
        push    r13
        push    r14
        push    r15
        call    hifadhi_jina
        mov     r11d, eax               ; ofseti ya jina la nyuga
        pop     r15
        pop     r14
        pop     r13
        pop     r12

        ; Kokotoa ukubwa na pangilio la nyuga
        ; r8d = ukubwa, r9d = pangilio, r10d = muundo ndani au -1
        mov     r10d, -1
        mov     edx, [rsp]              ; jina la muundo ndani au -1
        ; Aina ya msingi 6 (muundo): tafuta faharisi ya muundo ndani
        ; hata kama nyuga ni pointer — pointer kwenda muundo bado
        ; inahitaji muundo id ili kufumbua wanachama wake
        cmp     r12d, 6
        jne     .nyuga_sio_id
        cmp     edx, -1
        je      .nyuga_sio_id
        mov     edi, edx
        call    tafuta_muundo
        cmp     eax, -1
        je      .nyuga_sio_id
        mov     r10d, eax               ; faharisi ya muundo ndani
.nyuga_sio_id:
        cmp     r14d, 0
        jg      .nyuga_ptr
        cmp     r12d, 1
        je      .nyuga_n8
        cmp     r12d, 2
        je      .nyuga_n16
        cmp     r12d, 3
        je      .nyuga_n32
        ; Muundo-thamani ndani — ukubwa na pangilio kutoka jedwali
        cmp     r10d, -1
        je      .nyuga_ptr
        mov     eax, [muundo_ukubwa + r10*4]
        mov     r8d, eax
        mov     eax, [muundo_pangilio + r10*4]
        mov     r9d, eax
        jmp     .nyuga_panga
.nyuga_n8:
        mov     r8d, 1
        mov     r9d, 1
        jmp     .nyuga_panga
.nyuga_n16:
        mov     r8d, 2
        mov     r9d, 2
        jmp     .nyuga_panga
.nyuga_n32:
        mov     r8d, 4
        mov     r9d, 4
        jmp     .nyuga_panga
.nyuga_ptr:
        mov     r8d, 8
        mov     r9d, 8
.nyuga_panga:
        ; Pangilia ofseti inayoendelea kwa pangilio la nyuga
        mov     eax, r13d
        mov     ecx, r9d
        dec     ecx
        add     eax, ecx
        not     ecx
        and     eax, ecx
        mov     r13d, eax

        ; Rekodi nyuga kwenye jedwali (upeo: 512)
        mov     rax, [nyuga_count]
        cmp     rax, 512
        jae     .nyuga_jaa
        mov     rcx, [muundo_count]
        dec     rcx
        mov     [nyuga_muundo + rax*4], ecx
        mov     [nyuga_jina_off + rax*4], r11d
        mov     [nyuga_aina + rax*4], r12d
        mov     [nyuga_muundo_id + rax*4], r10d
        mov     [nyuga_nyota + rax*4], r14d
        mov     [nyuga_ofseti + rax*4], r13d
        inc     qword [nyuga_count]

        ; Sogeza ofseti inayoendelea na pangilio la juu
        add     r13d, r8d
        cmp     r9d, r15d
        jbe     .nyuga_align_ok
        mov     r15d, r9d
.nyuga_align_ok:

        ; Toa nafasi ya muundo ndani na tarajia nukta mkato
        add     rsp, 8
        mov     edi, TOK_NUKTA_MKATO
        call    tarajia_ishara
        cmp     eax, 1
        jne     .fail
        jmp     .nyuga_loop

.nyuga_eof_kosa:
        ; Mwisho wa faili ndani ya muundo usiofungwa
        lea     rdi, [msg_parseerr]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

.mwisho:
        ; Tumia }
        inc     qword [token_pos]

        ; Hiari ya nukta mkato baada ya }
        mov     edi, TOK_NUKTA_MKATO
        call    tarajia_ishara

        ; Muundo tupu: pangilio la chini ni 1
        cmp     r15d, 0
        jne     .pangilio_ok
        mov     r15d, 1
.pangilio_ok:

        ; Pangilia ukubwa wa mwisho kwa pangilio la juu
        mov     eax, r13d
        mov     ecx, r15d
        dec     ecx
        add     eax, ecx
        not     ecx
        and     eax, ecx
        mov     r13d, eax

        ; Rekodi ukubwa, pangilio, na mwisho wa nyuga
        mov     rax, [muundo_count]
        dec     rax
        mov     [muundo_ukubwa + rax*4], r13d
        mov     [muundo_pangilio + rax*4], r15d
        mov     ecx, [nyuga_count]
        mov     [muundo_nyuga_mwisho + rax*4], ecx

        mov     eax, 1                  ; ilifanikiwa
        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret

.nyuga_jaa:
        lea     rdi, [msg_nyuga_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

.fail_pop:
        add     rsp, 8                  ; toa nafasi ya muundo ndani
.fail:
        xor     eax, eax
        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret
.muundo_jaa:
        lea     rdi, [msg_muundo_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

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
        ; Kinga ya mipaka: usisome zaidi ya mwisho wa mkondo wa tokeni
        cmp     rdi, [token_count]
        jae     .done
        cmp     dword [token_ty + rdi*4], TOK_MWISHO
        je      .done

        ; Jaribu changanua kazi au tangazo la nje
        ; Angalia ikiwa ni aina
        ; Kwanza, ikiwa sio neno, kosa LAUTI (si ruka kimya)
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

        ; Husisha: aina mbili
        ;   husisha { faili.swa }  — kiungo cha ndani chenye mabano
        ;   husisha C::stdio       — kiungo cha C, hakina mabano
        inc     qword [token_pos]       ; ruka neno 'husisha'
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_FUNGO
        je      .husisha_brace           ; ina { — tumia kuruka kwa mabano

        ; husisha C::xxx — ruka tokeni hadi mstari ubadilike
        mov     edx, [token_line + rdi*4] ; hifadhi nambari ya mstari
.husisha_c_line:
        inc     qword [token_pos]
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_MWISHO
        je      .programu_loop
        cmp     edx, [token_line + rdi*4]
        je      .husisha_c_line           ; bado kwenye mstari uleule
        jmp     .programu_loop            ; mstari mpya — tumeshamaliza

        ; husisha { faili.swa } — kuruka kwa mabano
.husisha_brace:
        mov     ecx, 1                   ; kina cha mabano: { = +1, } = -1
.husisha_skip:
        inc     qword [token_pos]
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_FUNGO
        je      .husisha_open
        cmp     dword [token_ty + rdi*4], TOK_FUNGA
        je      .husisha_close
        cmp     dword [token_ty + rdi*4], TOK_MWISHO
        je      .husisha_eof_kosa        ; husisha isiyofungwa ni kosa
        jmp     .husisha_skip
.husisha_eof_kosa:
        lea     rdi, [msg_parseerr]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit
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
        ; Angalia kwanza ikiwa ni tangazo la muundo
        lea     rdx, [kw_muundo]
        call    linganisha_neno_muhimu
        cmp     eax, 0
        jne     .sio_muundo
        call    changanua_muundo
        jmp     .programu_loop
.sio_muundo:

        ; Okoa nafasi ya tokeni kabla ya kujaribu changanua kazi
        ; (changanua_kazi hula aina na jina hata inaposhindwa)
        mov     rax, [token_pos]
        push    rax
        mov     qword [compiler_state], 10
        call    changanua_kazi
        mov     qword [compiler_state], 11
        cmp     eax, -1
        je      .try_global

        ; Ilifanikiwa — ni kazi; tupilie nafasi ya tokeni iliyookolewa
        add     rsp, 8

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

; ------------------------------------------------------------
; Tangazo la kigeu cha ulimwengu: N32 x = 42; au N8 buf[1024];
; ------------------------------------------------------------
.try_global:
        ; Rejesha nafasi ya tokeni iliyookolewa (changanua_kazi ilikula tokeni)
        pop     rax
        mov     [token_pos], rax
        mov     qword [compiler_state], 14

        ; Chunguza aina: eax = aina, ebx = idadi ya nyota
        call    changanua_aina
        cmp     eax, 0
        je      .skip_token             ; haikujulikana, haikutumia tokeni
        cmp     eax, 6
        jae     .skip_token             ; muundo — hatuutegemezi kama kigeu cha ulimwengu

        ; Hifadhi aina na nyota kwenye stack: [rsp] = aina, [rsp+8] = nyota
        push    rbx
        push    rax

        ; Jina la kigeu lazima liwe TOK_NENO
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NENO
        jne     .global_parse_fail
        mov     rsi, [token_text + rdi*8]
        movzx   ecx, word [token_len + rdi*2]
        call    hifadhi_jina
        mov     r14d, eax               ; ofseti ya jina kwenye str_pool
        inc     qword [token_pos]

        ; Kokotoa ukubwa wa aina
        mov     edi, [rsp]              ; aina
        call    ukubwa_kutoka_aina
        cmp     eax, 0
        je      .global_size8           ; W0 -> baiti 8
        cmp     dword [rsp+8], 0
        jg      .global_size8           ; nyota -> baiti 8
        mov     r15d, eax
        jmp     .global_size_ok
.global_size8:
        mov     r15d, 8
.global_size_ok:

        ; Aina tatu za tangazo: safu [n], = thamani, au tupu ;
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_MABANO_MKOA
        je      .global_array
        cmp     dword [token_ty + rdi*4], TOK_SAWA
        je      .global_init
        cmp     dword [token_ty + rdi*4], TOK_NUKTA_MKATO
        je      .global_bare
        jmp     .global_parse_fail

.global_array:
        inc     qword [token_pos]        ; ruka [
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NAMBARI
        jne     .global_parse_fail
        mov     rcx, [token_val + rdi*8] ; idadi ya elementi
        imul    rcx, r15
        mov     r15, rcx                 ; ukubwa wa jumla wa safu
        inc     qword [token_pos]
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_MABANO_MKOA_FUNGA
        jne     .global_parse_fail
        inc     qword [token_pos]
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NUKTA_MKATO
        jne     .global_parse_fail
        inc     qword [token_pos]
        mov     r8d, 1                   ; .bss
        xor     r9d, r9d
        mov     r10d, 1                  ; ni safu
        jmp     .global_register

.global_init:
        inc     qword [token_pos]        ; ruka =
        xor     r9d, r9d                 ; thamani ya awali
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_ISHARA
        jne     .global_init_num
        cmp     qword [token_val + rdi*8], OP_TOA
        jne     .global_parse_fail
        inc     qword [token_pos]        ; ruka -
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NAMBARI
        jne     .global_parse_fail
        mov     r9, [token_val + rdi*8]
        neg     r9                       ; hasi
        inc     qword [token_pos]
        jmp     .global_init_end
.global_init_num:
        cmp     dword [token_ty + rdi*4], TOK_NAMBARI
        jne     .global_parse_fail
        mov     r9, [token_val + rdi*8]
        inc     qword [token_pos]
.global_init_end:
        mov     rdi, [token_pos]
        cmp     dword [token_ty + rdi*4], TOK_NUKTA_MKATO
        jne     .global_parse_fail
        inc     qword [token_pos]
        mov     r8d, 0                   ; .data
        mov     r10d, 0                  ; si safu
        jmp     .global_register

.global_bare:
        inc     qword [token_pos]        ; ruka ;
        xor     r9d, r9d
        mov     r8d, 1                   ; .bss
        mov     r10d, 0                  ; si safu

.global_register:
        cmp     qword [global_count], MAX_GLOBALS
        jae     .global_fail_pops
        mov     rcx, [global_count]
        test    r8d, r8d
        jnz     .global_reg_bss

        ; — .data: hakikisha nafasi kabla ya kusajili
        mov     rax, [data_buf_pos]
        add     rax, r15
        cmp     rax, DATA_BUF_SIZE
        ja      .global_fail_pops
        mov     eax, [data_buf_pos]
        mov     [global_offset + rcx*4], eax
        mov     [global_is_bss + rcx*4], r8d  ; 0 = .data
        mov     dword [global_is_array + rcx*4], 0  ; .data ni tangazo la kuanzishwa tu
        ; Andika thamani kwa mpangilio wa little-endian
        mov     r10, [data_buf_pos]
        mov     rax, r9
        mov     r11, r15
.global_data_loop:
        cmp     r11, 0
        je      .global_data_done
        mov     [data_buf + r10], al
        shr     rax, 8
        inc     r10
        dec     r11
        jmp     .global_data_loop
.global_data_done:
        add     [data_buf_pos], r15
        jmp     .global_common

.global_reg_bss:
        mov     eax, [bss_size]
        mov     [global_offset + rcx*4], eax
        mov     [global_is_bss + rcx*4], r8d  ; 1 = .bss
        mov     [global_is_array + rcx*4], r10d
        add     [bss_size], r15

.global_common:
        ; Rekodi kigeu cha ulimwengu kwenye jedwali
        lea     rax, [str_pool + r14]
        mov     [global_name + rcx*8], rax
        mov     [global_size + rcx*4], r15d
        mov     eax, [rsp]
        mov     [global_base_type + rcx*4], eax
        mov     eax, [rsp+8]
        mov     [global_star_count + rcx*4], eax
        inc     qword [global_count]

        ; Tengeneza nodi ya AST_TANGAZO_ULIM
        mov     r8d, AST_TANGAZO_ULIM
        mov     r9d, -1
        mov     r10d, -1
        mov     r11d, -1
        call    ast_nodi_mpya
        cmp     eax, -1
        je      .global_fail_pops
        mov     ecx, [rsp]               ; aina -> ast_thamani
        mov     [ast_thamani + eax*4], ecx
        mov     [ast_jina_off + eax*4], r14d

        ; Ongeza kwenye orodha ya mzizi
        cmp     r13d, -1
        jne     .global_append
        mov     [ast_kushoto + r12*4], eax
        mov     r13d, eax
        jmp     .global_done
.global_append:
        mov     [ast_nne + r13*4], eax
        mov     r13d, eax
.global_done:
        add     rsp, 16
        mov     qword [compiler_state], 15
        jmp     .programu_loop

.global_parse_fail:
        ; Hitilafu ya ulichanganuzi wa tangazo la ulimwengu
        lea     rdi, [msg_parseerr]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

.global_fail_pops:
        ; Kosa LAUTI — jedwali la ulimwengu limejaa (si ruka kimya)
        lea     rdi, [msg_global_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

.skip_token:
        ; Tokeni isiyojulikana kwenye kiwango cha juu — kosa LAUTI
        lea     rdi, [msg_parseerr]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

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
gen_fixup_offset:       resd MAX_RELOCS
gen_fixup_label:        resd MAX_RELOCS
gen_fixup_count:        resq 1
gen_stack_size:         resq 1
gen_current_func:       resq 1
gen_return_label:       resq 1
gen_label_pos:          resd 1024

        section .text

; -------------------------------------------------------
; gen_label_mpya: tengeneza lebo mpya ya kipekee
;   rax = nambari ya lebo
; -------------------------------------------------------
gen_label_mpya:
        mov     rax, [gen_label_count]
        cmp     rax, 1024
        jae     .genlabel_jaa
        inc     qword [gen_label_count]
        ret
.genlabel_jaa:
        ; Kosa LAUTI — uharibifu wa kimya hauruhusiwi
        lea     rdi, [msg_genlabel_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

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
        lea     rdi, [msg_textbuf_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

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
        lea     rdi, [msg_textbuf_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

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
        lea     rdi, [msg_textbuf_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

; -------------------------------------------------------
; gen_fixup_ongeza: ongeza fixup kwa kuruka mbele
;   edi = ofseti ya sasa kwenye bafa la .text (ya baiti 4 zilizowekwa)
;   esi = lebo ya kulenga
; -------------------------------------------------------
gen_fixup_ongeza:
        push    rbx
        mov     rbx, [gen_fixup_count]
        cmp     rbx, MAX_RELOCS
        jae     .overflow
        mov     [gen_fixup_offset + rbx*4], edi
        mov     [gen_fixup_label + rbx*4], esi
        inc     qword [gen_fixup_count]
        pop     rbx
        ret
.overflow:
        ; Kosa LAUTI — uharibifu wa kimya hauruhusiwi
        lea     rdi, [msg_fixup_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

; -------------------------------------------------------
; gen_weka_lebo: weka lebo kwenye nafasi ya sasa ya bafa la .text
;   edi = nambari ya lebo
; -------------------------------------------------------
gen_weka_lebo:
        ; Hifadhi nafasi ya lebo: edi = nafasi ya text_buf, esi = nambari ya lebo
        cmp     esi, 1024
        jae     .overflow
        mov     [gen_label_pos + rsi*4], edi
        ret
.overflow:
        lea     rdi, [msg_genlabel_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

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

        ; Sajili kigezo cha ndani
        mov     rcx, [local_count]
        cmp     rcx, MAX_LOCALS
        jae     .local_jaa
        ; Hifadhi jina
        lea     rdi, [str_pool + r14]
        mov     [local_name + rcx*8], rdi
        ; Ofseti ya rafu kutoka frame_wapi (baada ya vigezo vyote)
        mov     r8d, [frame_wapi]
        mov     [local_offset + rcx*4], r8d
        ; Hifadhi aina msingi na idadi ya nyota
        mov     r10d, [ast_thamani + r12*4]  ; aina msingi (N8=1, n.k.)
        mov     [local_base_type + rcx*4], r10d
        mov     r10d, [ast_kulia + r12*4]    ; idadi ya nyota
        mov     [local_star_count + rcx*4], r10d
        ; Hifadhi kitambulisho cha muundo (tafuta kwa ofseti ya jina)
        mov     edi, [ast_tiga + r12*4]      ; ofseti ya jina la muundo au -1
        cmp     edi, -1
        jl      .safu_ya_ndani               ; hasi isiyo -1: -N, safu ya ndani
        je      .no_muundo
        call    tafuta_muundo               ; rax = faharisi ya muundo au -1
        mov     r10d, eax
        mov     dword [local_array_size + rcx*4], 0
        jmp     .muundo_hifadhiwa
.safu_ya_ndani:
        neg     edi                          ; idadi ya elementi N
        mov     [local_array_size + rcx*4], edi
        mov     r10d, -1
        jmp     .muundo_hifadhiwa
.no_muundo:
        mov     r10d, -1
        mov     dword [local_array_size + rcx*4], 0
.muundo_hifadhiwa:
        mov     [local_muundo_id + rcx*4], r10d
        inc     qword [local_count]
        jmp     .local_sajiliwa
.local_jaa:
        ; Kosa LAUTI — uharibifu wa kimya hauruhusiwi
        lea     rdi, [msg_local_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit
.local_sajiliwa:

        ; Safu ya ndani: sehemu ya rafu = N x ukubwa wa elementi,
        ; imepangiliwa kwa 8; ofseti ni msingi wa eneo (kama muundo)
        mov     r10d, [local_array_size + rcx*4]
        test    r10d, r10d
        jz      .sio_safu_slot
        ; Ukubwa wa elementi moja
        mov     r11d, [local_star_count + rcx*4]
        cmp     r11d, 0
        jne     .safu_uk_8
        mov     edi, [local_base_type + rcx*4]
        call    ukubwa_kutoka_aina       ; haibadilishi rcx/r10/r11
        test    eax, eax
        jnz     .safu_uk_ok
        mov     eax, 8                   ; W0 → 8
.safu_uk_ok:
        jmp     .safu_uk_hifadhiwa
.safu_uk_8:
        mov     eax, 8
.safu_uk_hifadhiwa:
        imul    rax, [local_array_size + rcx*4]
        add     rax, 7
        and     rax, ~7
        add     [frame_wapi], rax
        mov     r8d, [frame_wapi]
        sub     r8d, 8
        mov     [local_offset + rcx*4], r8d
        jmp     .slot_done
.sio_safu_slot:

        ; Amua ukubwa wa sehemu ya rafu
        ; Muundo bila nyota: ukubwa wa muundo uliopangiliwa kwa 8
        mov     r10d, [local_base_type + rcx*4]
        cmp     r10d, 6
        jne     .slot_8
        mov     r10d, [local_star_count + rcx*4]
        cmp     r10d, 0
        jne     .slot_8
        mov     edi, [local_muundo_id + rcx*4]
        cmp     edi, -1
        je      .slot_8
        mov     r10d, [muundo_ukubwa + rdi*4]
        add     r10d, 7
        and     r10d, ~7
        add     [frame_wapi], r10
        ; Ofseti ya muundo: eneo lake ni [rbp-O, rbp-O+ukubwa).
        ; Ili lisiingiliane na kigezo kinachofuata, msingi lazima
        ; uwe frame_wapi_mpya - 8 (na sloti ya scalar inaanzia
        ; frame_wapi_ya_zamani — miungo huongezwa juu ya msingi)
        mov     r8d, [frame_wapi]
        sub     r8d, 8
        mov     [local_offset + rcx*4], r8d
        jmp     .slot_done
.slot_8:
        add     qword [frame_wapi], 8
.slot_done:

        ; Muundo bila nyota: thamani yake ni anwani yake
        mov     r10d, [local_base_type + rcx*4]
        cmp     r10d, 6
        jne     .scalar
        mov     r10d, [local_star_count + rcx*4]
        cmp     r10d, 0
        jne     .scalar
        ; Hakuna kianzilishi: hakuna msimbo wa kuhifadhi
        cmp     r13d, -1
        je      .tangazo_mwisho
        ; Muundo wenye kianzilishi: lea rax, [rbp + disp32]
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x8D
        call    gen_baiti
        mov     al, 0x85
        call    gen_baiti
        mov     edi, r8d
        neg     edi
        call    gen_neno4
        ; Hifadhi anwani ya lengo kwenye rafu (push rax)
        mov     al, 0x50
        call    gen_baiti
        ; Tathmini kianzilishi — anwani ya chanzo inabaki kwenye rax
        push    r12
        mov     r12d, r13d
        call    uzalishaji_ast
        pop     r12
        ; Rejesha faharisi ya usajili baada ya tathmini
        mov     rcx, [local_count]
        dec     rcx
        ; Nakili muundo: mov rsi, rax; pop rdi; mov ecx, ukubwa; rep movsb
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0xC6
        call    gen_baiti
        mov     al, 0x5F
        call    gen_baiti
        mov     al, 0xB9
        call    gen_baiti
        mov     edi, [local_muundo_id + rcx*4]
        mov     edi, [muundo_ukubwa + rdi*4]
        call    gen_neno4
        mov     al, 0xF3
        call    gen_baiti
        mov     al, 0xA4
        call    gen_baiti
        jmp     .tangazo_mwisho

.scalar:
        ; Safu ya ndani: hakuna msimbo wa kuhifadhi kianzilishi
        cmp     dword [local_array_size + rcx*4], 0
        jg      .tangazo_mwisho
        ; Thamani ya kianzilishi (ikiwa ipo)
        cmp     r13d, -1
        je      .no_init
        push    r12
        mov     r12d, r13d
        call    uzalishaji_ast
        pop     r12
        jmp     .store_value
.no_init:
        xor     eax, eax                ; hakuna kianzilishi, thamani = 0
.store_value:
        ; Rejesha faharisi ya usajili na ofseti baada ya tathmini
        mov     rcx, [local_count]
        dec     rcx
        mov     r8d, [local_offset + rcx*4]

        ; Angalia kama tunahitaji 64-bit store
        mov     r10d, [local_star_count + rcx*4]
        cmp     r10d, 0
        jg      .store_64
        mov     r10d, [local_base_type + rcx*4]
        cmp     r10d, 4                      ; N64
        je      .store_64
        cmp     r10d, 5                      ; W0
        je      .store_64
        cmp     r10d, 7                      ; D64
        je      .store_d64

        ; 32-bit store: mov [rbp + disp32], eax  ->  89 85 d32
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x85
        call    gen_baiti
        mov     edi, r8d
        neg     edi
        call    gen_neno4
        jmp     .tangazo_mwisho

.store_64:
        ; 64-bit store: mov [rbp + disp32], rax  ->  48 89 85 d32
        mov     al, 0x48                     ; REX.W
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x85
        call    gen_baiti
        mov     edi, r8d
        neg     edi
        call    gen_neno4
        jmp     .tangazo_mwisho

.store_d64:
        ; movsd [rbp + disp32], xmm0  ->  f2 0f 11 85 d32
        mov     al, 0xF2
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x11
        call    gen_baiti
        mov     al, 0x85
        call    gen_baiti
        mov     edi, r8d
        neg     edi
        call    gen_neno4

.tangazo_mwisho:
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
; uzalishaji_halisi_d: zalisha msimbo kwa halisi ya D64
;   Biti 64 zimegawanywa: ast_thamani = lo32, ast_tiga = hi32.
;   Hutoa thabiti ya baiti 8 kwenye .data (iliyopangiliwa kwa 8) na
;   inapakia kwenye xmm0: movsd xmm0, [disp32].
;   CT: -1 (haijulikani — hesabu ya kuelea haikunjwi na mbegu).
; -------------------------------------------------------
uzalishaji_halisi_d:
        push    r12
        push    r13
        mov     r12d, r12d
        mov     r13d, [ast_thamani + r12*4]   ; lo32

        ; Pangilia data_buf_pos kwa 8 NA usonge mbele kwa baiti 8
        mov     rax, [data_buf_pos]
        add     rax, 7
        and     rax, ~7
        mov     rcx, rax                ; mwanzo wa thabiti
        add     rax, 8                  ; nafasi inayofuata
        cmp     rax, DATA_BUF_SIZE
        ja      .data_jaa
        mov     [data_buf_pos], rax

        ; Andika baiti 8 (little-endian: lo32 kwanza, kisha hi32)
        mov     [data_buf + rcx], r13d
        mov     r13d, [ast_tiga + r12*4]      ; hi32
        mov     [data_buf + rcx + 4], r13d

        ; movsd xmm0, [disp32]  ->  f2 0f 10 05 d32
        mov     al, 0xF2
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x10
        call    gen_baiti
        mov     al, 0x05
        call    gen_baiti

        ; Rekebisho la .data (sawa na mfuatano): addend = data_offset - 4
        mov     rdi, [rela_count]
        cmp     rdi, MAX_RELOCS - 1
        jae     .rela_jaa
        mov     edx, [text_buf_pos]
        mov     [rela_offset + rdi*4], edx
        mov     dword [rela_sym + rdi*4], -1    ; .data section
        sub     ecx, 4                          ; data_offset - 4
        mov     [rela_addend + rdi*4], ecx
        inc     qword [rela_count]
        ; Weka nafasi ya disp32
        mov     edi, 0
        call    gen_neno4

        mov     eax, -1                 ; CT: haijulikani
        pop     r13
        pop     r12
        ret
.rela_jaa:
        lea     rdi, [msg_rela_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit
.data_jaa:
        lea     rdi, [msg_databuf]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

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
        mov     edi, [local_offset + r13*4]
        neg     edi                         ; edi = hasi (kwa [rbp + disp32])

        ; Safu ya ndani — anwani ndiyo thamani (kama muundo)
        cmp     dword [local_array_size + r13*4], 0
        jg      .lea_ya_safu

        ; Muundo wa ndani (aina 6, bila nyota) — anwani ndiyo thamani
        mov     r8d, [local_base_type + r13*4]
        cmp     r8d, 6
        jne     .sio_muundo
        mov     r8d, [local_star_count + r13*4]
        cmp     r8d, 0
        jne     .load_ptr64                 ; muundo* — pakia pointer
        ; lea rax, [rbp+disp32]
.lea_ya_safu:
        mov     al, 0x48                    ; REX.W
        call    gen_baiti
        mov     al, 0x8D                    ; lea
        call    gen_baiti
        mov     al, 0x85                    ; ModRM: [rbp+disp32], reg=rax
        call    gen_baiti
        call    gen_neno4
        jmp     .load_done

.sio_muundo:
        ; Angalia ikiwa ni kigezo cha nyota (pointer) — paki baiti 8 daima
        mov     r8d, [local_star_count + r13*4]
        cmp     r8d, 0
        jg      .load_ptr64

        ; Sio pointer — paki kulingana na aina msingi
        mov     r8d, [local_base_type + r13*4]
        cmp     r8d, 1                      ; N8
        je      .load_n8
        cmp     r8d, 2                      ; N16
        je      .load_n16
        cmp     r8d, 4                      ; N64
        je      .load_n64
        cmp     r8d, 7                      ; D64
        je      .load_d64
        ; N32 (3) au chaguo-msingi
        mov     al, 0x8B                    ; mov eax, [rbp+disp32]
        call    gen_baiti
        mov     al, 0x85                    ; ModRM: [rbp+disp32], reg=eax
        call    gen_baiti
        call    gen_neno4
        jmp     .load_done

.load_n8:
        mov     al, 0x0F                    ; movzx eax, byte [rbp+disp32]
        call    gen_baiti
        mov     al, 0xB6
        call    gen_baiti
        mov     al, 0x85
        call    gen_baiti
        call    gen_neno4
        jmp     .load_done

.load_n16:
        mov     al, 0x0F                    ; movzx eax, word [rbp+disp32]
        call    gen_baiti
        mov     al, 0xB7
        call    gen_baiti
        mov     al, 0x85
        call    gen_baiti
        call    gen_neno4
        jmp     .load_done

.load_n64:
.load_ptr64:
        ; Pointer au N64 — paki baiti 8
        mov     al, 0x48                    ; REX.W
        call    gen_baiti
        mov     al, 0x8B                    ; mov rax, [rbp+disp32]
        call    gen_baiti
        mov     al, 0x85                    ; ModRM: [rbp+disp32], reg=rax
        call    gen_baiti
        call    gen_neno4

.load_d64:
        ; D64 — paki baiti 8 kwenye xmm0: f2 0f 10 85 d32
        mov     al, 0xF2
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x10
        call    gen_baiti
        mov     al, 0x85
        call    gen_baiti
        call    gen_neno4
        mov     eax, -1                 ; CT haijulikani kwa D64
        pop     r14
        pop     r13
        pop     r12
        ret

.load_done:
        ; Thamani itasomwa wakati wa utekelezaji; rudisha 0 kwa sasa
        xor     eax, eax
        pop     r14
        pop     r13
        pop     r12
        ret

.not_found:
        ; Tafuta kwenye orodha ya vigezo vya ulimwengu
        xor     r13d, r13d
.search_ulimwengu:
        cmp     r13, [global_count]
        jae     .not_found_exit
        mov     rdi, [global_name + r13*8]
        mov     rsi, r14
        call    linganisha_mfuatano
        cmp     eax, 0
        je      .found_ulimwengu
        inc     r13
        jmp     .search_ulimwengu

.found_ulimwengu:
        ; Kigezo cha ulimwengu — pakia kwa RIP-relative
        cmp     dword [global_is_array + r13*4], 0
        jne     .load_ulimwengu_lea      ; safu — rudisha anwani yake
        mov     r8d, [global_base_type + r13*4]
        cmp     r8d, 6
        je      .load_ulimwengu_lea      ; muundo — anwani ndiyo thamani
        mov     r8d, [global_star_count + r13*4]
        cmp     r8d, 0
        jg      .load_ulimwengu_n64
        mov     r8d, [global_base_type + r13*4]
        cmp     r8d, 1                      ; N8
        je      .load_ulimwengu_n8
        cmp     r8d, 2                      ; N16
        je      .load_ulimwengu_n16
        cmp     r8d, 4                      ; N64
        je      .load_ulimwengu_n64
        cmp     r8d, 5                      ; W0 — pointer, baiti 8
        je      .load_ulimwengu_n64
        ; N32 (3) au chaguo-msingi
.load_ulimwengu_n32:
        mov     al, 0x8B                    ; mov eax, [rip+disp32]
        call    gen_baiti
        mov     al, 0x05
        call    gen_baiti
        jmp     .load_ulimwengu_reloc
.load_ulimwengu_n8:
        mov     al, 0x0F                    ; movzx eax, byte [rip+disp32]
        call    gen_baiti
        mov     al, 0xB6
        call    gen_baiti
        mov     al, 0x05
        call    gen_baiti
        jmp     .load_ulimwengu_reloc
.load_ulimwengu_n16:
        mov     al, 0x0F                    ; movzx eax, word [rip+disp32]
        call    gen_baiti
        mov     al, 0xB7
        call    gen_baiti
        mov     al, 0x05
        call    gen_baiti
        jmp     .load_ulimwengu_reloc
.load_ulimwengu_n64:
        mov     al, 0x48                    ; mov rax, [rip+disp32]
        call    gen_baiti
        mov     al, 0x8B
        call    gen_baiti
        mov     al, 0x05
        call    gen_baiti
        jmp     .load_ulimwengu_reloc

.load_ulimwengu_lea:
        ; Safu ya ulimwengu — rudisha anwani: lea rax, [rip+disp32]
        mov     al, 0x48                    ; REX.W
        call    gen_baiti
        mov     al, 0x8D                    ; lea
        call    gen_baiti
        mov     al, 0x05
        call    gen_baiti

.load_ulimwengu_reloc:
        ; Ongeza rekebisho la R_X86_64_PC32 kwa kigezo cha ulimwengu
        mov     rdi, [rela_count]
        cmp     rdi, MAX_RELOCS - 1
        jae     .ulimwengu_rela_jaa
        mov     edx, [text_buf_pos]
        mov     [rela_offset + rdi*4], edx
        mov     eax, r13d
        add     eax, 2
        neg     eax
        mov     [rela_sym + rdi*4], eax
        mov     dword [rela_addend + rdi*4], -4
        inc     qword [rela_count]
.skip_ulimwengu_reloc:
        ; Weka nafasi ya disp32 (baiti 4 za sifuri)
        mov     edi, 0
        call    gen_neno4
        jmp     .load_done
.ulimwengu_rela_jaa:
        ; Kosa LAUTI — uharibifu wa kimya hauruhusiwi
        lea     rdi, [msg_rela_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

.not_found_exit:
        ; Labda ni kazi ya nje — tunarudisha 0 kwa sasa
        xor     eax, eax
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; uzalishaji_anwani_ya: zalisha msimbo kwa nodi ya & (chukua anwani)
;   r12d = faharisi ya nodi (AST_ALAMA_ELEKEZA)
;   mtoto wa kushoto = usemi unaochukuliwa anwani
;   huweka anwani kwenye eax
; -------------------------------------------------------
uzalishaji_anwani_ya:
        push    r12
        push    r13
        push    r14

        mov     r12d, r12d
        mov     r13d, [ast_kushoto + r12*4]   ; nodi ya mtoto

        ; Chunguza aina ya mtoto
        mov     r14d, [ast_aina + r13*4]
        cmp     r14d, AST_JINA
        jne     .kupitia_nodi

        ; Mtoto ni jina — tafuta kigezo cha ndani
        mov     r14d, [ast_jina_off + r13*4]
        lea     r14, [str_pool + r14]

        xor     r13d, r13d
.search_anwani:
        cmp     r13, [local_count]
        jae     .search_anwani_ulimwengu
        mov     rdi, [local_name + r13*8]
        mov     rsi, r14
        call    linganisha_mfuatano
        cmp     eax, 0
        je      .found_anwani
        inc     r13
        jmp     .search_anwani

.found_anwani:
        ; Pata ofseti ya kigezo
        mov     edi, [local_offset + r13*4]
        neg     edi

        ; Toa: lea rax, [rbp + disp32]
        ; Encoding: 48 8D 85 XX XX XX XX
        mov     al, 0x48                        ; REX.W
        call    gen_baiti
        mov     al, 0x8D                        ; lea
        call    gen_baiti
        mov     al, 0x85                        ; ModRM: [rbp+disp32], reg=rax
        call    gen_baiti
        call    gen_neno4

        pop     r14
        pop     r13
        pop     r12
        ret

.kupitia_nodi:
        ; Mtoto si jina rahisi — tumia mzalishaji wa anwani wa nodi
        ; (hushughulikia wanachama, vipengele vya safu, na *p)
        push    r12
        mov     r12d, r13d
        call    uzalishaji_anwani_ya_nodi
        pop     r12

        pop     r14
        pop     r13
        pop     r12
        ret

.search_anwani_ulimwengu:
        ; Tafuta kigezo cha ulimwengu
        xor     r13d, r13d
.search_anwani_ulimwengu_loop:
        cmp     r13, [global_count]
        jae     .not_lvalue
        mov     rdi, [global_name + r13*8]
        mov     rsi, r14
        call    linganisha_mfuatano
        cmp     eax, 0
        je      .found_anwani_ulimwengu
        inc     r13
        jmp     .search_anwani_ulimwengu_loop

.found_anwani_ulimwengu:
        ; Toa: lea rax, [rip + disp32]
        ; Encoding: 48 8D 05 XX XX XX XX
        mov     al, 0x48                        ; REX.W
        call    gen_baiti
        mov     al, 0x8D                        ; lea
        call    gen_baiti
        mov     al, 0x05                        ; ModRM: [rip+disp32], reg=rax
        call    gen_baiti

        ; Ongeza rekebisho la R_X86_64_PC32 kwa kigezo cha ulimwengu
        mov     rdi, [rela_count]
        cmp     rdi, MAX_RELOCS - 1
        jae     .anwani_rela_jaa
        mov     edx, [text_buf_pos]
        mov     [rela_offset + rdi*4], edx
        mov     eax, r13d
        add     eax, 2
        neg     eax
        mov     [rela_sym + rdi*4], eax
        mov     dword [rela_addend + rdi*4], -4
        inc     qword [rela_count]
.skip_anwani_reloc:
        ; Weka nafasi ya disp32 (baiti 4 za sifuri)
        mov     edi, 0
        call    gen_neno4
        pop     r14
        pop     r13
        pop     r12
        ret
.anwani_rela_jaa:
        ; Kosa LAUTI — uharibifu wa kimya hauruhusiwi
        lea     rdi, [msg_rela_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

.not_lvalue:
        xor     eax, eax
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; uzalishaji_nyota_ya: zalisha msimbo kwa nodi ya * (deref)
;   r12d = faharisi ya nodi (AST_NYOTA_ELEKEZA)
;   mtoto wa kushoto = usemi wa pointer
;   huweka thamani iliyofunguliwa kwenye eax
; -------------------------------------------------------
uzalishaji_nyota_ya:
        push    r12
        push    r13
        push    r14

        mov     r12d, r12d
        mov     r13d, [ast_kushoto + r12*4]   ; nodi ya mtoto (pointer)

        ; Tathmini usemi wa pointer
        push    r12
        mov     r12d, r13d
        call    uzalishaji_ast                  ; eax = anwani (pointer)
        pop     r12

        ; Jaribu kujua aina ya kipengele kutoka kwa kigezo
        mov     r14d, [ast_aina + r13*4]
        cmp     r14d, AST_JINA
        jne     .deref_n32                     ; sio jina — tumia N32 kwa chaguo-msingi

        ; Tafuta kigezo cha ndani
        mov     r14d, [ast_jina_off + r13*4]
        lea     r14, [str_pool + r14]
        xor     r13d, r13d
.deref_search:
        cmp     r13, [local_count]
        jae     .deref_n32
        mov     rdi, [local_name + r13*8]
        mov     rsi, r14
        call    linganisha_mfuatano
        cmp     eax, 0
        je      .deref_found
        inc     r13
        jmp     .deref_search

.deref_found:
        mov     r14d, [local_star_count + r13*4]
        cmp     r14d, 0
        je      .deref_n32                     ; sio pointer

        ; Ni pointer — tumia aina msingi kujua ukubwa wa kupakia
        mov     r14d, [local_base_type + r13*4]
        cmp     r14d, 1                        ; N8
        je      .deref_n8
        cmp     r14d, 2                        ; N16
        je      .deref_n16
        cmp     r14d, 4                        ; N64
        je      .deref_n64
        cmp     r14d, 5                        ; W0 — anwani, pakia baiti 8
        je      .deref_n64
        ; N32 (3) au chaguo-msingi — anguka hadi .deref_n32

.deref_n32:
        ; mov eax, [rax]  →  8B 00
        mov     al, 0x8B
        call    gen_baiti
        mov     al, 0x00
        call    gen_baiti
        jmp     .deref_done

.deref_n8:
        ; movzx eax, byte [rax]  →  0F B6 00
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0xB6
        call    gen_baiti
        mov     al, 0x00
        call    gen_baiti
        jmp     .deref_done

.deref_n16:
        ; movzx eax, word [rax]  →  0F B7 00
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0xB7
        call    gen_baiti
        mov     al, 0x00
        call    gen_baiti
        jmp     .deref_done

.deref_n64:
        ; mov rax, [rax]  →  48 8B 00
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x8B
        call    gen_baiti
        mov     al, 0x00
        call    gen_baiti

.deref_done:
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; uzalishaji_kielelezo: zalisha msimbo kwa nodi ya faharisi ya safu
;   mtoto wa kushoto = usemi wa msingi (pointer)
;   mtoto wa kulia  = usemi wa faharisi
;   Hutoa msimbo wa: [msingi + faharisi * ukubwa]
;   Njia kuu iko kwenye gen_kielelezo (r12d = nodi, edi = namna)
; -------------------------------------------------------
uzalishaji_kielelezo:
        push    r12
        mov     edi, 0                  ; namna: 0 = thamani
        call    gen_kielelezo
        pop     r12
        ret

; -------------------------------------------------------
; gen_kielelezo: injini ya uzalishaji wa faharisi ya safu
;   r12d = nodi ya KIELELEZO
;   edi  = namna: 0 = paki thamani, 1 = rudisha anwani
;   Hutoa msimbo unaoweka matokeo kwenye rax
; -------------------------------------------------------
gen_kielelezo:
        push    rbp
        mov     rbp, rsp
        sub     rsp, 48
        push    r12
        push    r13
        push    r14
        push    r15

        mov     [rbp-8], rdi            ; namna
        mov     r13d, [ast_kulia + r12*4]    ; usemi wa faharisi
        mov     r14d, [ast_kushoto + r12*4]  ; usemi wa msingi
        mov     [rbp-48], r14           ; hifadhi nodi ya msingi

        ; Hali maalum: safu ya ulimwengu (global_is_array == 1)
        cmp     r14d, -1
        je      .njia_kawaida
        cmp     dword [ast_aina + r14*4], AST_JINA
        jne     .njia_kawaida
        mov     r8d, [ast_jina_off + r14*4]
        lea     r8, [str_pool + r8]
        ; Tafuta safu ya ndani kwanza (inachagiza ulimwengu)
        xor     r10d, r10d
.saka_ndani:
        cmp     r10, [local_count]
        jae     .saka_ulimwengu_anza
        mov     rdi, [local_name + r10*8]
        mov     rsi, r8
        call    linganisha_mfuatano
        cmp     eax, 0
        je      .imepatikana_ndani
        inc     r10
        jmp     .saka_ndani
.imepatikana_ndani:
        cmp     dword [local_array_size + r10*4], 0
        je      .njia_kawaida             ; kigezo cha kawaida (kinachagiza ulimwengu)
        ; Kipengele: aina msingi ya safu, nyota zake moja kwa moja
        mov     r8d, [local_base_type + r10*4]
        mov     [rbp-24], r8d           ; aina ya kipengele
        mov     r8d, [local_star_count + r10*4]
        mov     [rbp-32], r8d           ; nyota za kipengele
        mov     dword [rbp-40], -1
        ; Ukubwa wa kipengele
        cmp     r8d, 0
        jg      .uk_8
        mov     r8d, [local_base_type + r10*4]
        cmp     r8d, 1
        je      .uk_1
        cmp     r8d, 2
        je      .uk_2
        cmp     r8d, 4
        je      .uk_8
        cmp     r8d, 5
        je      .uk_8
        mov     dword [rbp-16], 4
        jmp     .tathmini
.saka_ulimwengu_anza:
        xor     r9d, r9d
.saka_ulimwengu:
        cmp     r9, [global_count]
        jae     .njia_kawaida
        mov     rdi, [global_name + r9*8]
        mov     rsi, r8
        call    linganisha_mfuatano
        cmp     eax, 0
        je      .imepatikana_ulimwengu
        inc     r9
        jmp     .saka_ulimwengu
.imepatikana_ulimwengu:
        cmp     dword [global_is_array + r9*4], 0
        je      .njia_kawaida           ; sio safu — kipengele cha pointer
        ; Kipengele ni aina msingi ya safu, nyota zake moja kwa moja
        mov     r8d, [global_base_type + r9*4]
        mov     [rbp-24], r8d           ; aina ya kipengele
        mov     r8d, [global_star_count + r9*4]
        mov     [rbp-32], r8d           ; nyota za kipengele
        mov     dword [rbp-40], -1
        ; Ukubwa wa kipengele
        cmp     r8d, 0
        jg      .uk_8                   ; kipengele cha nyota — baiti 8
        mov     r8d, [global_base_type + r9*4]
        cmp     r8d, 1
        je      .uk_1
        cmp     r8d, 2
        je      .uk_2
        cmp     r8d, 4
        je      .uk_8
        cmp     r8d, 5
        je      .uk_8
        mov     dword [rbp-16], 4
        jmp     .tathmini

.njia_kawaida:
        ; Fumbua aina ya usemi wa msingi, kisha punguza nyota moja
        mov     r12d, r14d
        call    fumbua_aina
        cmp     ebx, 0
        je      .sio                    ; sio pointer
        dec     ebx                     ; nyota za kipengele
        mov     [rbp-24], eax           ; aina ya kipengele
        mov     [rbp-32], ebx           ; nyota za kipengele
        mov     [rbp-40], edx           ; muundo id ya kipengele

        ; Ukubwa wa kipengele kutoka kipengele (baada ya punguzo)
        cmp     ebx, 0
        jg      .uk_8
        cmp     eax, 6
        je      .uk_muundo
        cmp     eax, 1
        je      .uk_1
        cmp     eax, 2
        je      .uk_2
        cmp     eax, 4
        je      .uk_8
        cmp     eax, 5
        je      .uk_8
        mov     dword [rbp-16], 4
        jmp     .tathmini
.uk_muundo:
        ; Ukubwa wa muundo kutoka jedwali la muundo_ukubwa
        cmp     edx, 0
        jl      .sio
        cmp     rdx, [muundo_count]
        jae     .sio
        mov     r8d, [muundo_ukubwa + rdx*4]
        mov     [rbp-16], r8d
        jmp     .tathmini
.uk_1:
        mov     dword [rbp-16], 1
        jmp     .tathmini
.uk_2:
        mov     dword [rbp-16], 2
        jmp     .tathmini
.uk_8:
        mov     dword [rbp-16], 8
        jmp     .tathmini

.tathmini:
        ; Tathmini usemi wa faharisi
        mov     r12d, r13d
        call    uzalishaji_ast          ; rax = faharisi
        ; push rax — hifadhi faharisi kwenye rafu ya utekelezaji
        mov     al, 0x50
        call    gen_baiti
        ; Tathmini usemi wa msingi
        mov     r12d, [rbp-48]
        call    uzalishaji_ast          ; rax = anwani ya msingi
        ; pop rcx — rudisha faharisi
        mov     al, 0x59
        call    gen_baiti

        ; Zidisha faharisi kwa ukubwa wa kipengele
        mov     r15d, [rbp-16]
        cmp     r15d, 1
        je      .hakuna_shift
        cmp     r15d, 2
        je      .shift_1
        cmp     r15d, 4
        je      .shift_2
        cmp     r15d, 8
        je      .shift_3
        ; Ukubwa mwingine — imul rcx, rcx, imm32
        mov     al, 0x48                ; REX.W
        call    gen_baiti
        mov     al, 0x69                ; imul r/m64, r64, imm32
        call    gen_baiti
        mov     al, 0xC9                ; ModRM: r/m=rcx, reg=rcx
        call    gen_baiti
        mov     edi, r15d
        call    gen_neno4
        jmp     .do_add
.shift_1:
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0xC1                ; shl r/m64, imm8
        call    gen_baiti
        mov     al, 0xE1                ; ModRM: r/m=rcx, reg=4 (shl)
        call    gen_baiti
        mov     al, 1
        call    gen_baiti
        jmp     .do_add
.shift_2:
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0xC1
        call    gen_baiti
        mov     al, 0xE1
        call    gen_baiti
        mov     al, 2
        call    gen_baiti
        jmp     .do_add
.shift_3:
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0xC1
        call    gen_baiti
        mov     al, 0xE1
        call    gen_baiti
        mov     al, 3
        call    gen_baiti

.do_add:
.hakuna_shift:
        ; add rax, rcx -> 48 01 C8
        mov     al, 0x48                ; REX.W
        call    gen_baiti
        mov     al, 0x01                ; add r/m64, r64
        call    gen_baiti
        mov     al, 0xC8                ; ModRM: r/m=rax, reg=rcx
        call    gen_baiti

        ; Namna ya anwani: rudisha anwani bila kupakia
        cmp     qword [rbp-8], 0
        jne     .mwisho
        ; Kipengele cha muundo: anwani ndiyo thamani
        cmp     dword [rbp-24], 6
        je      .mwisho
        ; Paki thamani kutoka [rax] kulingana na kipengele
        cmp     dword [rbp-32], 0
        jg      .pakia_64
        mov     r8d, [rbp-24]
        cmp     r8d, 1
        je      .pakia_8
        cmp     r8d, 2
        je      .pakia_16
        cmp     r8d, 4
        je      .pakia_64
        cmp     r8d, 5
        je      .pakia_64
        ; N32: mov eax, [rax] -> 8B 00
        mov     al, 0x8B
        call    gen_baiti
        mov     al, 0x00
        call    gen_baiti
        jmp     .mwisho
.pakia_8:
        ; movzx eax, byte [rax] -> 0F B6 00
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0xB6
        call    gen_baiti
        mov     al, 0x00
        call    gen_baiti
        jmp     .mwisho
.pakia_16:
        ; movzx eax, word [rax] -> 0F B7 00
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0xB7
        call    gen_baiti
        mov     al, 0x00
        call    gen_baiti
        jmp     .mwisho
.pakia_64:
        ; mov rax, [rax] -> 48 8B 00
        mov     al, 0x48                ; REX.W
        call    gen_baiti
        mov     al, 0x8B
        call    gen_baiti
        mov     al, 0x00
        call    gen_baiti

.mwisho:
        xor     eax, eax                ; mafanikio ya kukusanya
        pop     r15
        pop     r14
        pop     r13
        pop     r12
        mov     rsp, rbp
        pop     rbp
        ret
.sio:
        xor     eax, eax                ; aina batili — hakuna msimbo
        jmp     .mwisho

; -------------------------------------------------------
; tafuta_nyuga: tafuta uga wa muundo kwa jina
;   edi = faharisi ya muundo kwenye jedwali la muundo
;   rsi = anwani ya jina la uga (ndani ya str_pool)
;   Matokeo: eax = ofseti, ebx = aina, ecx = nyota, edx = muundo id
;   eax = -1 kama haikupatikana
; -------------------------------------------------------
tafuta_nyuga:
        push    r12
        push    r13
        push    r14
        push    r15

        mov     r12d, edi               ; muundo id
        mov     r13, rsi                ; anwani ya jina la uga
        cmp     r12, [muundo_count]
        jae     .tn_hakuna
        mov     r14d, [muundo_nyuga_anza + r12*4]
        mov     r15d, [muundo_nyuga_mwisho + r12*4]
.tn_mzunguko:
        cmp     r14d, r15d
        jae     .tn_hakuna
        push    r14
        push    r15
        mov     r8d, [nyuga_jina_off + r14*4]
        lea     rdi, [str_pool + r8]
        mov     rsi, r13
        call    linganisha_mfuatano
        pop     r15
        pop     r14
        cmp     eax, 0
        je      .tn_iko
        inc     r14d
        jmp     .tn_mzunguko
.tn_iko:
        mov     eax, [nyuga_ofseti + r14*4]
        mov     ebx, [nyuga_aina + r14*4]
        mov     ecx, [nyuga_nyota + r14*4]
        mov     edx, [nyuga_muundo_id + r14*4]
        jmp     .tn_mwisho
.tn_hakuna:
        mov     eax, -1
        xor     ebx, ebx
        xor     ecx, ecx
        mov     edx, -1
.tn_mwisho:
        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; fumbua_aina: amua aina ya usemi kwa wakati wa ukusanyaji
;   r12d = faharisi ya nodi (hifadhiwa)
;   Matokeo: eax = aina msingi, ebx = idadi ya nyota,
;            edx = muundo id (au -1)
;   Batili: eax=0, ebx=0, edx=-1
; -------------------------------------------------------
fumbua_aina:
        push    r13
        push    r14
        push    r15
.fa_anza_tena:
        cmp     r12d, -1
        je      .fa_sio
        mov     r13d, [ast_aina + r12*4]

        cmp     r13d, AST_JINA
        je      .fa_jina
        cmp     r13d, AST_TANGAZO
        je      .fa_tangazo
        cmp     r13d, AST_ELEKEZA_JINA
        je      .fa_mwanachama
        cmp     r13d, AST_ENEKEZA_FUNGO
        je      .fa_mwanachama
        cmp     r13d, AST_KIELELEZO
        je      .fa_kielelezo
        cmp     r13d, AST_NYOTA_KIELELEZO
        je      .fa_kielelezo
        cmp     r13d, AST_NYOTA_ELEKEZA
        je      .fa_nyota
        cmp     r13d, AST_ALAMA_ELEKEZA
        je      .fa_alama
        cmp     r13d, AST_NAMBA
        je      .fa_namba
        cmp     r13d, AST_HALISI_D
        je      .fa_halisi_d
        cmp     r13d, AST_KAULI
        je      .fa_kauli
        cmp     r13d, AST_WAMBILE
        je      .fa_wambile
        jmp     .fa_sio

.fa_halisi_d:
        mov     eax, 7                  ; D64
        xor     ebx, ebx
        mov     edx, -1
        jmp     .fa_mwisho

.fa_jina:
        ; Tafuta kati ya vigezo vya ndani
        mov     r13d, [ast_jina_off + r12*4]
        lea     r13, [str_pool + r13]
        xor     r14d, r14d
.fa_jina_ndani:
        cmp     r14, [local_count]
        jae     .fa_jina_ulimwengu
        mov     rdi, [local_name + r14*8]
        mov     rsi, r13
        call    linganisha_mfuatano
        cmp     eax, 0
        je      .fa_jina_ndani_iko
        inc     r14
        jmp     .fa_jina_ndani
.fa_jina_ndani_iko:
        mov     eax, [local_base_type + r14*4]
        mov     ebx, [local_star_count + r14*4]
        mov     edx, [local_muundo_id + r14*4]
        jmp     .fa_mwisho
.fa_jina_ulimwengu:
        xor     r14d, r14d
.fa_jina_ulimwengu_mzunguko:
        cmp     r14, [global_count]
        jae     .fa_sio
        mov     rdi, [global_name + r14*8]
        mov     rsi, r13
        call    linganisha_mfuatano
        cmp     eax, 0
        je      .fa_jina_ulimwengu_iko
        inc     r14
        jmp     .fa_jina_ulimwengu_mzunguko
.fa_jina_ulimwengu_iko:
        mov     eax, [global_base_type + r14*4]
        mov     ebx, [global_star_count + r14*4]
        mov     edx, -1                 ; ulimwengu hana muundo id
        jmp     .fa_mwisho

.fa_tangazo:
        mov     eax, [ast_thamani + r12*4]
        mov     ebx, [ast_kulia + r12*4]
        mov     edx, -1
        cmp     eax, 6
        jne     .fa_mwisho
        ; Muundo: tafuta id kutoka jina la muundo (ast_tiga)
        mov     r15d, [ast_tiga + r12*4]
        cmp     r15d, -1
        je      .fa_mwisho
        push    rax
        push    rbx
        mov     edi, r15d
        call    tafuta_muundo
        mov     edx, eax
        pop     rbx
        pop     rax
        jmp     .fa_mwisho

.fa_mwanachama:
        ; Mwanachama wa muundo: .jina au ->jina
        mov     r15d, [ast_kushoto + r12*4]
        cmp     r15d, -1
        je      .fa_sio
        ; Fumbua aina ya msingi
        push    r12
        mov     r12d, r15d
        call    fumbua_aina
        pop     r12
        cmp     edx, -1
        je      .fa_sio
        ; Tafuta uga kwa jina ndani ya muundo wa msingi
        mov     r15d, [ast_kulia + r12*4]
        cmp     r15d, -1
        je      .fa_sio
        mov     r14d, [ast_jina_off + r15*4]
        lea     rsi, [str_pool + r14]
        mov     edi, edx
        ; Matokeo ya tafuta_nyuga (eax/ebx/ecx/edx) ni yale
        ; tunayohitaji — usiyarudishe kwa thamani za zamani
        call    tafuta_nyuga
        cmp     eax, -1
        je      .fa_sio
        ; eax = ofseti, ebx = aina, ecx = nyota, edx = muundo id
        mov     eax, ebx
        mov     ebx, ecx
        jmp     .fa_mwisho

.fa_kielelezo:
        ; Kipengele cha safu: fumbua msingi, punguza nyota moja
        mov     r15d, [ast_kushoto + r12*4]
        cmp     r15d, -1
        je      .fa_sio
        push    r12
        mov     r12d, r15d
        call    fumbua_aina
        pop     r12
        cmp     ebx, 0
        je      .fa_sio
        dec     ebx
        jmp     .fa_mwisho

.fa_nyota:
        ; Toleo la dereference: *p — punguza nyota moja
        mov     r15d, [ast_kushoto + r12*4]
        cmp     r15d, -1
        je      .fa_sio
        push    r12
        mov     r12d, r15d
        call    fumbua_aina
        pop     r12
        cmp     ebx, 0
        je      .fa_sio
        dec     ebx
        jmp     .fa_mwisho

.fa_alama:
        ; Anwani-ya: &x — ongeza nyota moja
        mov     r15d, [ast_kushoto + r12*4]
        cmp     r15d, -1
        je      .fa_sio
        push    r12
        mov     r12d, r15d
        call    fumbua_aina
        pop     r12
        inc     ebx
        jmp     .fa_mwisho

.fa_namba:
        mov     eax, 3                  ; N32
        xor     ebx, ebx
        mov     edx, -1
        jmp     .fa_mwisho

.fa_kauli:
        ; Kauli ya binary: aina ya operanda ya KUSHOTO huenea —
        ; hii inaruhusu minyororo kama (pi * r) * r kubaki D64
        ; (suala #135 lilikuwa na hitilafu sawa kwenye dereva wa Rust).
        ; Rudi kwenye kichwa cha fumbua_aina BAADA ya pushes —
        ; .fa_anza_tena iko mara moja baada ya kuzihifadhi.
        mov     r12d, [ast_kushoto + r12*4]
        jmp     .fa_anza_tena

.fa_wambile:
        mov     r15d, [ast_kushoto + r12*4]
        cmp     r15d, -1
        je      .fa_sio
        push    r12
        mov     r12d, r15d
        call    fumbua_aina
        pop     r12
        jmp     .fa_mwisho

.fa_sio:
        xor     eax, eax
        xor     ebx, ebx
        mov     edx, -1
.fa_mwisho:
        pop     r15
        pop     r14
        pop     r13
        ret

; -------------------------------------------------------
; uzalishaji_anwani_ya_nodi: zalisha msimbo wa anwani ya nodi
;   r12d = faharisi ya nodi
;   Hutoa msimbo unaoweka anwani kwenye rax
; -------------------------------------------------------
uzalishaji_anwani_ya_nodi:
        push    r12
        push    r13
        push    r14
        push    r15

        cmp     r12d, -1
        je      .uan_sio
        mov     r13d, [ast_aina + r12*4]

        cmp     r13d, AST_JINA
        je      .uan_jina
        cmp     r13d, AST_ELEKEZA_JINA
        je      .uan_mwanachama
        cmp     r13d, AST_ENEKEZA_FUNGO
        je      .uan_mwanachama
        cmp     r13d, AST_KIELELEZO
        je      .uan_kielelezo
        jmp     .uan_tathmini

.uan_jina:
        ; Tafuta kati ya vigezo vya ndani
        mov     r13d, [ast_jina_off + r12*4]
        lea     r13, [str_pool + r13]
        xor     r14d, r14d
.uan_jina_ndani:
        cmp     r14, [local_count]
        jae     .uan_jina_ulimwengu
        mov     rdi, [local_name + r14*8]
        mov     rsi, r13
        call    linganisha_mfuatano
        cmp     eax, 0
        je      .uan_jina_ndani_iko
        inc     r14
        jmp     .uan_jina_ndani
.uan_jina_ndani_iko:
        ; lea rax, [rbp + ofseti] — ofseti hasi (vigezo chini ya rbp)
        mov     edi, [local_offset + r14*4]
        neg     edi
        mov     al, 0x48                ; REX.W
        call    gen_baiti
        mov     al, 0x8D                ; lea r64, [rbp + disp32]
        call    gen_baiti
        mov     al, 0x85                ; ModRM: r/m=rbp, reg=rax
        call    gen_baiti
        call    gen_neno4
        jmp     .uan_mwisho
.uan_jina_ulimwengu:
        xor     r14d, r14d
.uan_jina_ulimwengu_mzunguko:
        cmp     r14, [global_count]
        jae     .uan_sio
        mov     rdi, [global_name + r14*8]
        mov     rsi, r13
        call    linganisha_mfuatano
        cmp     eax, 0
        je      .uan_jina_ulimwengu_iko
        inc     r14
        jmp     .uan_jina_ulimwengu_mzunguko
.uan_jina_ulimwengu_iko:
        ; lea rax, [rip + kitu] — kujazwa na reloketi
        mov     al, 0x48                ; REX.W
        call    gen_baiti
        mov     al, 0x8D                ; lea r64, [rip + disp32]
        call    gen_baiti
        mov     al, 0x05
        call    gen_baiti
        ; Rekodi reloketi R_X86_64_PC32
        mov     rdi, [rela_count]
        cmp     rdi, MAX_RELOCS - 1
        jae     .uan_rela_jaa
        mov     edx, [text_buf_pos]
        mov     [rela_offset + rdi*4], edx
        mov     eax, r14d
        add     eax, 2
        neg     eax
        mov     [rela_sym + rdi*4], eax
        mov     dword [rela_addend + rdi*4], -4
        inc     qword [rela_count]
.uan_jina_ulimwengu_skip:
        mov     edi, 0
        call    gen_neno4
        jmp     .uan_mwisho
.uan_rela_jaa:
        ; Kosa LAUTI — uharibifu wa kimya hauruhusiwi
        lea     rdi, [msg_rela_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

.uan_mwanachama:
        ; Mwanachama wa muundo: anwani ya msingi + ofseti ya uga
        mov     r15d, [ast_kushoto + r12*4]
        cmp     r15d, -1
        je      .uan_sio
        ; Fumbua aina ya msingi
        push    r12
        mov     r12d, r15d
        call    fumbua_aina
        pop     r12
        cmp     edx, -1
        je      .uan_sio
        mov     r13d, eax               ; aina ya msingi
        mov     r14d, ebx               ; nyota za msingi
        ; Tafuta uga
        mov     r8d, [ast_kulia + r12*4]
        cmp     r8d, -1
        je      .uan_sio
        mov     r9d, [ast_jina_off + r8*4]
        lea     rsi, [str_pool + r9]
        mov     edi, edx
        call    tafuta_nyuga
        cmp     eax, -1
        je      .uan_sio
        mov     r15d, eax               ; ofseti ya uga
        ; Msingi wa muundo-thamani: anwani ya msingi; sivyo: thamani ya msingi
        cmp     r13d, 6
        jne     .uan_m_tathmini
        cmp     r14d, 0
        jne     .uan_m_tathmini
        push    r15
        mov     r8d, [ast_kushoto + r12*4]
        mov     r12d, r8d
        call    uzalishaji_anwani_ya_nodi
        pop     r15
        jmp     .uan_m_ofseti
.uan_m_tathmini:
        push    r15
        mov     r8d, [ast_kushoto + r12*4]
        mov     r12d, r8d
        call    uzalishaji_ast
        pop     r15
.uan_m_ofseti:
        ; add rax, ofseti ya uga
        cmp     r15d, 0
        je      .uan_mwisho
        mov     al, 0x48                ; REX.W
        call    gen_baiti
        mov     al, 0x05                ; add rax, imm32
        call    gen_baiti
        mov     edi, r15d
        call    gen_neno4
        jmp     .uan_mwisho

.uan_kielelezo:
        mov     edi, 1                  ; namna ya anwani
        call    gen_kielelezo
        jmp     .uan_mwisho

.uan_tathmini:
        ; Zingatia usemi: thamani yake ndiyo anwani (mf. *p)
        call    uzalishaji_ast
.uan_mwisho:
        xor     eax, eax
        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret
.uan_sio:
        xor     eax, eax                ; hali batili — hakuna msimbo
        jmp     .uan_mwisho

; -------------------------------------------------------
; uzalishaji_mwanachama: zalisha msimbo wa mwanachama wa muundo
;   r12d = nodi ya mwanachama (ELEKEZA_JINA / ENEKEZA_FUNGO)
;   Hutoa msimbo unaoweka thamani kwenye rax
;   Kwa muundo: anwani ya mwanachama ndiyo thamani
; -------------------------------------------------------
uzalishaji_mwanachama:
        push    r12
        push    r13
        push    r14
        push    r15

        ; Fumbua aina ya mwanachama kwanza
        call    fumbua_aina
        push    rax                     ; aina
        push    rbx                     ; nyota

        ; Kisha anwani yake
        call    uzalishaji_anwani_ya_nodi

        pop     r14                     ; nyota
        pop     r13                     ; aina

        ; Nyota: paki baiti 8 — hata kwa pointer kwenda muundo,
        ; thamani yake ni pointer iliyo kwenye kumbukumbu
        cmp     r14d, 0
        jg      .um_pakia_64
        ; Muundo-thamani: anwani ndiyo thamani
        cmp     r13d, 6
        je      .um_mwisho
        cmp     r13d, 1
        je      .um_pakia_8
        cmp     r13d, 2
        je      .um_pakia_16
        cmp     r13d, 4
        je      .um_pakia_64
        cmp     r13d, 5
        je      .um_pakia_64
        ; N32: mov eax, [rax] -> 8B 00
        mov     al, 0x8B
        call    gen_baiti
        mov     al, 0x00
        call    gen_baiti
        jmp     .um_mwisho
.um_pakia_8:
        ; movzx eax, byte [rax] -> 0F B6 00
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0xB6
        call    gen_baiti
        mov     al, 0x00
        call    gen_baiti
        jmp     .um_mwisho
.um_pakia_16:
        ; movzx eax, word [rax] -> 0F B7 00
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0xB7
        call    gen_baiti
        mov     al, 0x00
        call    gen_baiti
        jmp     .um_mwisho
.um_pakia_64:
        ; mov rax, [rax] -> 48 8B 00
        mov     al, 0x48                ; REX.W
        call    gen_baiti
        mov     al, 0x8B
        call    gen_baiti
        mov     al, 0x00
        call    gen_baiti
.um_mwisho:
        xor     eax, eax
        pop     r15
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

        ; Kazi inayorudisha D64: ABI ya xmm0 bado haijatekelezwa
        ; kwenye mbegu — kosa LAUTI badala ya thamani isiyo sahihi.
        cmp     dword [kazi_ret_aina], 7
        jne     .rudisha_sio_d64
        lea     rdi, [msg_d64_wito]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit
.rudisha_sio_d64:

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

        ; Kwa sret, nakili muundo hadi eneo la kurudia
        cmp     dword [kazi_ret_muundo_id], 0
        jl      .sio_nakala
        ; mov rdi, [rbp + 8]
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x8b
        call    gen_baiti
        mov     al, 0x7d
        call    gen_baiti
        mov     al, 0x08
        call    gen_baiti
        ; mov rsi, rax
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0xc6
        call    gen_baiti
        ; mov ecx, ukubwa wa muundo
        mov     al, 0xb9
        call    gen_baiti
        mov     edi, [kazi_ret_muundo_id]
        mov     edi, [muundo_ukubwa + rdi*4]
        call    gen_neno4
        ; rep movsb
        mov     al, 0xf3
        call    gen_baiti
        mov     al, 0xa4
        call    gen_baiti
.sio_nakala:

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

        ; Angalia ikiwa ni mfuatano wa herufi (kushoto == -1)
        cmp     r13d, -1
        je      .do_string

        ; Angalia ikiwa ni assignment (=) — inahitaji utoaji maalum
        cmp     r15d, OP_SAWA
        je      .do_assign

        ; Angalia ikiwa ni uchaguzi (?:) — operesheni haizalishwi mapema
        cmp     r15d, OP_HUU
        je      .do_huu

        ; && na || zinahitaji mzunguko mfupi (short-circuit), sawa na
        ; uzalishaji_na/uzalishaji_au kwenye msingi/uzalishaji.swa.
        ; Bila hii, upande wa kulia unatathminiwa hata wakati kushoto
        ; tayari umeamua matokeo — dereferensi kama `j >= 0 && a[j]`
        ; zinavunjika kwa a[j] nje ya mipaka.
        cmp     r15d, OP_NA
        je      .do_and_sc
        cmp     r15d, OP_AU
        je      .do_or_sc

        ; Hesabu za kuelea (D64): kama operanda ya kushoto ni ya
        ; kuelea, chukua njia ya SSE — thamani husafiri kupitia rafu
        ; kama baiti 8 (movq) na oparesheni hutumia xmm0/xmm1.
        push    r12
        mov     r12d, r13d
        call    fumbua_aina
        pop     r12
        cmp     eax, 7
        jne     .sio_kuelea
        cmp     r15d, OP_JUMLISHA
        je      .fl_add
        cmp     r15d, OP_TOA
        je      .fl_sub
        cmp     r15d, OP_ZIDISHA
        je      .fl_mul
        cmp     r15d, OP_GAWANYA
        je      .fl_div
        cmp     r15d, OP_SAWA_SAWA
        je      .fl_eq
        cmp     r15d, OP_SIO_SAWA
        je      .fl_ne
        cmp     r15d, OP_KIDOGO
        je      .fl_lt
        cmp     r15d, OP_KIDOGO_SAWA
        je      .fl_le
        cmp     r15d, OP_KUBWA
        je      .fl_gt
        cmp     r15d, OP_KUBWA_SAWA
        je      .fl_ge
        ; Kuelea na ishara isiyojulikana — njia kamili itaanguka kwa
        ; sauti (k.m. % ya desimali haisaidiwi).
.sio_kuelea:

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
        push    rax                     ; hifadhi CT ya kushoto kabla ya kutoa
        mov     al, 0x59
        call    gen_baiti
        pop     rax

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
        cmp     r15d, OP_NA
        je      .do_and
        cmp     r15d, OP_AU
        je      .do_or
        cmp     r15d, OP_NA_BITI
        je      .do_and
        cmp     r15d, OP_AU_BITI
        je      .do_or
        cmp     r15d, OP_XOR_BITI
        je      .do_xor
        cmp     r15d, OP_HAMISHA_KUSHOTO
        je      .do_shl
        cmp     r15d, OP_HAMISHA_KULIA
        je      .do_shr
        ; chaguo-msingi
        jmp     .done

.do_add:
        add     eax, ecx                ; wakati wa kukusanya
        push    rax                     ; hifadhi CT kabla ya kutoa
        ; add eax, ecx → 01 c8
        mov     al, 0x01
        call    gen_baiti
        mov     al, 0xc8
        call    gen_baiti
        pop     rax
        jmp     .done
.do_sub:
        sub     eax, ecx                ; wakati wa kukusanya
        push    rax                     ; hifadhi CT kabla ya kutoa
        ; sub eax, ecx → 29 c8
        mov     al, 0x29
        call    gen_baiti
        mov     al, 0xc8
        call    gen_baiti
        pop     rax
        jmp     .done
.do_mul:
        imul    eax, ecx                ; wakati wa kukusanya
        push    rax                     ; hifadhi CT kabla ya kutoa
        ; imul eax, ecx → 0f af c1
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0xaf
        call    gen_baiti
        mov     al, 0xc1
        call    gen_baiti
        pop     rax
        jmp     .done
.do_div:
        ; Epuka idiv kwa wakati wa kukusanya ikiwa kigawanyo ni sifuri
        cmp     ecx, 0
        je      .div_skip_ct
        cdq                             ; wakati wa kukusanya
        idiv    ecx
.div_skip_ct:
        push    rax                     ; hifadhi CT kabla ya kutoa
        ; cdq → 99
        mov     al, 0x99
        call    gen_baiti
        ; idiv ecx → f7 f9
        mov     al, 0xf7
        call    gen_baiti
        mov     al, 0xf9
        call    gen_baiti
        pop     rax
        jmp     .done
.do_mod:
        ; Epuka idiv kwa wakati wa kukusanya ikiwa kigawanyo ni sifuri
        cmp     ecx, 0
        je      .mod_skip_ct
        cdq                             ; wakati wa kukusanya
        idiv    ecx
        mov     eax, edx
.mod_skip_ct:
        push    rax                     ; hifadhi CT kabla ya kutoa
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
        pop     rax
        jmp     .done
.do_eq:
        cmp     eax, ecx                ; wakati wa kukusanya
        sete    al
        movzx   eax, al
        push    rax                     ; hifadhi CT kabla ya kutoa
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
        pop     rax
        jmp     .done
.do_ne:
        cmp     eax, ecx                ; wakati wa kukusanya
        setne   al
        movzx   eax, al
        push    rax                     ; hifadhi CT kabla ya kutoa
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
        pop     rax
        jmp     .done
.do_lt:
        cmp     eax, ecx                ; wakati wa kukusanya
        setl    al
        movzx   eax, al
        push    rax                     ; hifadhi CT kabla ya kutoa
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
        pop     rax
        jmp     .done
.do_gt:
        cmp     eax, ecx                ; wakati wa kukusanya
        setg    al
        movzx   eax, al
        push    rax                     ; hifadhi CT kabla ya kutoa
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
        pop     rax
        jmp     .done
.do_le:
        cmp     eax, ecx                ; wakati wa kukusanya
        setle   al
        movzx   eax, al
        push    rax                     ; hifadhi CT kabla ya kutoa
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
        pop     rax
        jmp     .done
.do_ge:
        cmp     eax, ecx                ; wakati wa kukusanya
        setge   al
        movzx   eax, al
        push    rax                     ; hifadhi CT kabla ya kutoa
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
        pop     rax
        jmp     .done
.do_and:
        ; wakati wa kukusanya: and eax, ecx
        and     eax, ecx
        push    rax                     ; hifadhi CT kabla ya kutoa
        ; and eax, ecx -> 21 c8
        mov     al, 0x21
        call    gen_baiti
        mov     al, 0xc8
        call    gen_baiti
        pop     rax
        jmp     .done
.do_or:
        ; wakati wa kukusanya: or eax, ecx
        or      eax, ecx
        push    rax                     ; hifadhi CT kabla ya kutoa
        ; or eax, ecx -> 09 c8
        mov     al, 0x09
        call    gen_baiti
        mov     al, 0xc8
        call    gen_baiti
        pop     rax
        jmp     .done
.do_xor:
        ; wakati wa kukusanya: xor eax, ecx
        xor     eax, ecx
        push    rax                     ; hifadhi CT kabla ya kutoa
        ; xor eax, ecx -> 31 c8
        mov     al, 0x31
        call    gen_baiti
        mov     al, 0xc8
        call    gen_baiti
        pop     rax
        jmp     .done
.do_shl:
        ; shl eax, cl -> d3 e0
        ; Hifadhi operesheni kabla ya kutoa (al itaharibiwa na mwito)
        mov     r8d, eax                ; kushoto (wakati wa kukusanya)
        mov     r9d, ecx                ; kulia (wakati wa kukusanya)
        mov     al, 0xd3
        call    gen_baiti
        mov     al, 0xe0
        call    gen_baiti
        ; wakati wa kukusanya — operesheni halisi baada ya kutoa
        cmp     r8d, -1
        je      .shift_isiyojulikana
        cmp     r9d, -1
        je      .shift_isiyojulikana
        mov     eax, r8d
        mov     ecx, r9d
        shl     eax, cl
        jmp     .done
.do_shr:
        ; sar eax, cl -> d3 f8
        ; Hifadhi operesheni kabla ya kutoa (al itaharibiwa na mwito)
        mov     r8d, eax                ; kushoto (wakati wa kukusanya)
        mov     r9d, ecx                ; kulia (wakati wa kukusanya)
        mov     al, 0xd3
        call    gen_baiti
        mov     al, 0xf8
        call    gen_baiti
        ; wakati wa kukusanya — operesheni halisi baada ya kutoa
        cmp     r8d, -1
        je      .shift_isiyojulikana
        cmp     r9d, -1
        je      .shift_isiyojulikana
        mov     eax, r8d
        mov     ecx, r9d
        sar     eax, cl
        jmp     .done
.shift_isiyojulikana:
        mov     eax, -1
        jmp     .done
.do_huu:
        ; Uchaguzi ?: — r13d = sharti, r14d = mfuatano (kweli, uwongo)
        ; Zalisha sharti
        push    r12
        mov     r12d, r13d
        call    uzalishaji_ast
        pop     r12
        mov     r8d, eax                ; CT ya sharti

        ; test eax, eax -> 85 c0
        mov     al, 0x85
        call    gen_baiti
        mov     al, 0xc0
        call    gen_baiti

        ; je — ofseti itarekebishwa baadaye -> 0f 84
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0x84
        call    gen_baiti
        mov     r9d, [text_buf_pos]     ; nafasi ya fixup ya je
        xor     edi, edi
        call    gen_neno4               ; kishikilia

        ; Tawi la kweli
        push    r12
        push    r8                      ; hifadhi CT ya sharti
        push    r9                      ; hifadhi fixup ya je
        mov     r12d, [ast_kushoto + r14*4]
        call    uzalishaji_ast
        pop     r9
        pop     r8
        pop     r12
        mov     r10d, eax               ; CT ya kweli

        ; jmp — ruka juu ya tawi la uwongo -> e9
        mov     al, 0xe9
        call    gen_baiti
        mov     r11d, [text_buf_pos]    ; nafasi ya fixup ya jmp
        xor     edi, edi
        call    gen_neno4               ; kishikilia

        ; Rekebisha je: elekeza kwenye mwanzo wa tawi la uwongo
        mov     edi, [text_buf_pos]
        sub     edi, r9d
        sub     edi, 4
        mov     r15d, [text_buf_pos]    ; hifadhi nafasi ya sasa
        mov     [text_buf_pos], r9d
        call    gen_neno4
        mov     [text_buf_pos], r15d    ; rejesha nafasi

        ; Tawi la uwongo
        push    r12
        push    r8                      ; CT ya sharti
        push    r10                     ; CT ya kweli
        push    r11                     ; fixup ya jmp
        mov     r12d, [ast_kulia + r14*4]
        call    uzalishaji_ast
        pop     r11
        pop     r10
        pop     r8
        pop     r12
        mov     ecx, eax                ; CT ya uwongo

        ; Rekebisha jmp: elekeza baada ya tawi la uwongo
        mov     edi, [text_buf_pos]
        sub     edi, r11d
        sub     edi, 4
        mov     r15d, [text_buf_pos]
        mov     [text_buf_pos], r11d
        call    gen_neno4
        mov     [text_buf_pos], r15d

        ; Chagua thamani ya wakati wa kukusanya
        cmp     r8d, -1
        je      .huu_isiyojulikana
        test    r8d, r8d
        jz      .huu_chagua_uwongo
        mov     eax, r10d
        jmp     .done
.huu_chagua_uwongo:
        mov     eax, ecx
        jmp     .done
.huu_isiyojulikana:
        cmp     r10d, ecx
        jne     .huu_tofauti
        mov     eax, r10d
        jmp     .done
.huu_tofauti:
        mov     eax, -1
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

        ; Upande wa kushoto: jina la kigezo, faharisi ya safu, dereferensi
        ; au mwanachama wa muundo
        mov     ebx, [ast_aina + r13*4]
        cmp     ebx, AST_JINA
        je      .assign_jina
        cmp     ebx, AST_KIELELEZO
        je      .assign_kielelezo
        cmp     ebx, AST_NYOTA_ELEKEZA
        je      .assign_nyota
        cmp     ebx, AST_ELEKEZA_JINA
        je      .assign_mwanachama
        cmp     ebx, AST_ENEKEZA_FUNGO
        je      .assign_mwanachama
        jmp     .assign_err

.assign_jina:
        ; Pata anwani ya jina la kigezo
        mov     r9d, [ast_jina_off + r13*4]
        lea     r9, [str_pool + r9]

        ; Tafuta kigezo kwenye orodha ya vigezo vya ndani
        xor     r10d, r10d
.as_search:
        cmp     r10, [local_count]
        jae     .as_global_search
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
        ; Toa maelekezo: mov [rbp - ofseti], eax  ->  89 85 XX XX XX XX
        mov     edi, [local_offset + r10*4]
        neg     edi
        mov     al, 0x89                ; MOV r/m32, r32
        call    gen_baiti
        mov     al, 0x85                ; ModRM: mod=10, reg=000(eax), r/m=101(rbp+disp32)
        call    gen_baiti
        call    gen_neno4               ; disp32 (hasi) — edi tayari imewekwa
        mov     eax, r8d                ; rudisha thamani iliyowekwa
        jmp     .done

        ; Kigezo hakikupatikana ndani — tafuta kati ya vigezo vya ulimwengu
.as_global_search:
        xor     r10d, r10d
.as_global_search_loop:
        cmp     r10, [global_count]
        jae     .assign_err
        mov     rdi, [global_name + r10*8]
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
        je      .as_found_global
        inc     r10
        jmp     .as_global_search_loop

.as_found_global:
        ; Kigezo cha ulimwengu — hifadhi kwa [rip+disp32]
        cmp     dword [global_is_array + r10*4], 0
        jne     .assign_err                     ; safu bila faharisi — si lvalue
        mov     r15d, [global_star_count + r10*4]
        cmp     r15d, 0
        jg      .as_store_64                    ; pointer — baiti 8
        mov     r15d, [global_base_type + r10*4]
        cmp     r15d, 1
        je      .as_store_8
        cmp     r15d, 2
        je      .as_store_16
        cmp     r15d, 4
        je      .as_store_64
        cmp     r15d, 5
        je      .as_store_64
        ; N32 (3) au chaguo-msingi
        mov     al, 0x89                        ; mov [rip+disp32], eax
        call    gen_baiti
        mov     al, 0x05
        call    gen_baiti
        jmp     .as_global_reloc

.as_store_8:
        mov     al, 0x88                        ; mov [rip+disp32], al
        call    gen_baiti
        mov     al, 0x05
        call    gen_baiti
        jmp     .as_global_reloc

.as_store_16:
        mov     al, 0x66                        ; mov [rip+disp32], ax
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x05
        call    gen_baiti
        jmp     .as_global_reloc

.as_store_64:
        mov     al, 0x48                        ; mov [rip+disp32], rax
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x05
        call    gen_baiti

.as_global_reloc:
        ; Ongeza rekebisho la R_X86_64_PC32 kwa kigezo cha ulimwengu
        mov     rdi, [rela_count]
        cmp     rdi, MAX_RELOCS - 1
        jae     .as_rela_jaa
        mov     edx, [text_buf_pos]
        mov     [rela_offset + rdi*4], edx
        mov     eax, r10d
        add     eax, 2
        neg     eax
        mov     [rela_sym + rdi*4], eax
        mov     dword [rela_addend + rdi*4], -4
        inc     qword [rela_count]
.as_skip_global_reloc:
        ; Weka nafasi ya disp32 (baiti 4 za sifuri)
        mov     edi, 0
        call    gen_neno4
        mov     eax, r8d                        ; rudisha thamani iliyowekwa
        jmp     .done
.as_rela_jaa:
        ; Kosa LAUTI — uharibifu wa kimya hauruhusiwi
        lea     rdi, [msg_rela_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

        ; Nodi ya KIELELEZO — hifadhi kwa [msingi + faharisi * ukubwa]
.assign_kielelezo:
        ; Hifadhi thamani ya upande wa kulia (tayari iko kwenye rax)
        mov     al, 0x50                        ; push rax — thamani
        call    gen_baiti

        mov     r14d, [ast_kulia + r13*4]       ; nodi ya faharisi
        mov     r12d, [ast_kushoto + r13*4]     ; nodi ya msingi (pointer)

        ; Amua ukubwa wa kipengele (r15d) — chaguo-msingi: baiti 4 (N32)
        mov     r15d, 4
        mov     ebx, [ast_aina + r12*4]
        cmp     ebx, AST_JINA
        jne     .ak_size_done

        ; Ni AST_JINA — tafuta kwenye orodha ya vigezo vya ndani
        mov     r9d, [ast_jina_off + r12*4]
        lea     r9, [str_pool + r9]
        xor     r10d, r10d
.ak_scan_locals:
        cmp     r10, [local_count]
        jae     .ak_scan_globals                ; haikupatikana ndani — jaribu ulimwengu
        mov     rdi, [local_name + r10*8]
        mov     rsi, r9
        call    linganisha_mfuatano
        cmp     eax, 0
        je      .ak_found_local
        inc     r10
        jmp     .ak_scan_locals

.ak_scan_globals:
        ; Tafuta kati ya vigezo vya ulimwengu
        xor     r10d, r10d
.ak_scan_globals_loop:
        cmp     r10, [global_count]
        jae     .ak_size_done                   ; haikupatikana kabisa — chaguo-msingi
        mov     rdi, [global_name + r10*8]
        mov     rsi, r9
        call    linganisha_mfuatano
        cmp     eax, 0
        je      .ak_found_global
        inc     r10
        jmp     .ak_scan_globals_loop

.ak_found_global:
        ; Safu ya ulimwengu: elementi ya safu ya nyota ina ukubwa 8,
        ; sivyo ukubwa wa aina msingi
        cmp     dword [global_is_array + r10*4], 0
        jne     .ak_found_global_safu

        ; Sio safu — kama .ak_found_local
        mov     r11d, [global_star_count + r10*4]
        cmp     r11d, 0
        je      .ak_size_done                   ; sio pointer — hali isiyowezekana
        cmp     r11d, 1
        jg      .ak_set_size_8                  ; nyota >= 2 — ukubwa wa nyota ni 8 (x86-64)
        mov     r11d, [global_base_type + r10*4]
        cmp     r11d, 1
        je      .ak_set_size_1
        cmp     r11d, 2
        je      .ak_set_size_2
        cmp     r11d, 4
        je      .ak_set_size_8
        mov     r15d, 4
        jmp     .ak_size_done

.ak_found_global_safu:
        mov     r11d, [global_star_count + r10*4]
        cmp     r11d, 0
        jg      .ak_set_size_8                  ; safu ya nyota — elementi ni pointer
        mov     r11d, [global_base_type + r10*4]
        cmp     r11d, 1
        je      .ak_set_size_1
        cmp     r11d, 2
        je      .ak_set_size_2
        cmp     r11d, 4
        je      .ak_set_size_8
        mov     r15d, 4
        jmp     .ak_size_done

.ak_found_local:
        ; Kigeu cha ndani chenye faharisi — pointer au safu ya ndani.
        ; Ukubwa wa elementi: safu -> pointer 8 (nyota >= 1) au aina
        ; msingi (nyota == 0); pointer -> aina msingi (nyota == 1) au
        ; 8 (nyota >= 2). Zamani safu tupu (nyota == 0) iliruka hadi
        ; .ak_size_done na r15d chakavu (4) — N8/N16/N64 zilivunjika.
        mov     r11d, [local_array_size + r10*4]
        test    r11d, r11d
        jnz     .ak_local_ni_safu
        ; --- Sio safu (pointer) ---
        mov     r11d, [local_star_count + r10*4]
        cmp     r11d, 0
        je      .ak_size_done                   ; hali isiyowezekana
        cmp     r11d, 1
        jg      .ak_set_size_8                  ; nyota >= 2
        jmp     .ak_local_aina_msingi
.ak_local_ni_safu:
        ; --- Safu ya ndani ---
        mov     r11d, [local_star_count + r10*4]
        cmp     r11d, 0
        jg      .ak_set_size_8                  ; safu ya pointer -> elementi 8
.ak_local_aina_msingi:
        mov     r11d, [local_base_type + r10*4]
        cmp     r11d, 1
        je      .ak_set_size_1
        cmp     r11d, 2
        je      .ak_set_size_2
        cmp     r11d, 4
        je      .ak_set_size_8
        ; N32 (3) au chaguo-msingi
        mov     r15d, 4
        jmp     .ak_size_done

.ak_set_size_1:
        mov     r15d, 1
        jmp     .ak_size_done
.ak_set_size_2:
        mov     r15d, 2
        jmp     .ak_size_done
.ak_set_size_8:
        mov     r15d, 8
        ; angukia .ak_size_done

.ak_size_done:
        ; r15d = ukubwa wa kipengele (1, 2, 4, au 8)

        ; Tathmini usemi wa faharisi
        push    r12                             ; hifadhi nodi ya msingi
        push    r8
        push    r15
        mov     r12d, r14d
        call    uzalishaji_ast                  ; eax = thamani ya faharisi
        pop     r15
        pop     r8
        pop     r12                             ; rudisha nodi ya msingi

        ; push rax — hifadhi faharisi kwenye rafu ya utekelezaji
        mov     al, 0x50
        call    gen_baiti

        ; Tathmini usemi wa msingi
        push    r8
        push    r15
        call    uzalishaji_ast                  ; eax = anwani ya msingi (pointer)
        pop     r15
        pop     r8

        ; pop rcx — rudisha faharisi
        mov     al, 0x59
        call    gen_baiti

        ; Zidisha faharisi kwa ukubwa wa kipengele
        cmp     r15d, 1
        je      .ak_no_shift
        cmp     r15d, 2
        je      .ak_shift_1
        cmp     r15d, 8
        je      .ak_shift_3
        ; shift_2 (kwa N32, ukubwa 4)
        mov     al, 0x48                        ; REX.W
        call    gen_baiti
        mov     al, 0xC1                        ; shl r/m64, imm8
        call    gen_baiti
        mov     al, 0xE1                        ; ModRM: r/m=rcx, reg=4(shl)
        call    gen_baiti
        mov     al, 2                           ; shl rcx, 2
        call    gen_baiti
        jmp     .ak_do_add

.ak_shift_1:
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0xC1
        call    gen_baiti
        mov     al, 0xE1
        call    gen_baiti
        mov     al, 1                           ; shl rcx, 1
        call    gen_baiti
        jmp     .ak_do_add

.ak_shift_3:
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0xC1
        call    gen_baiti
        mov     al, 0xE1
        call    gen_baiti
        mov     al, 3                           ; shl rcx, 3
        call    gen_baiti

.ak_do_add:
.ak_no_shift:
        ; add rax, rcx -> 48 01 C8
        mov     al, 0x48                        ; REX.W
        call    gen_baiti
        mov     al, 0x01                        ; add r/m64, r64
        call    gen_baiti
        mov     al, 0xC8                        ; ModRM: r/m=rax, reg=rcx
        call    gen_baiti

        ; pop rcx — rudisha thamani
        mov     al, 0x59
        call    gen_baiti

        ; Hifadhi [rax], rcx kulingana na ukubwa
        cmp     r15d, 1
        je      .ak_store_8
        cmp     r15d, 2
        je      .ak_store_16
        cmp     r15d, 8
        je      .ak_store_64

        ; N32: mov [rax], ecx -> 89 08
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x08                        ; ModRM: [rax], reg=ecx
        call    gen_baiti
        mov     eax, r8d                        ; rudisha thamani iliyowekwa
        jmp     .done

.ak_store_8:
        ; mov [rax], cl -> 88 08
        mov     al, 0x88
        call    gen_baiti
        mov     al, 0x08
        call    gen_baiti
        mov     eax, r8d
        jmp     .done

.ak_store_16:
        ; mov [rax], cx -> 66 89 08
        mov     al, 0x66
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x08
        call    gen_baiti
        mov     eax, r8d
        jmp     .done

.ak_store_64:
        ; mov [rax], rcx -> 48 89 08
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x08
        call    gen_baiti
        mov     eax, r8d
        jmp     .done

        ; Nodi ya NYOTA_ELEKEZA — hifadhi kupitia pointer
.assign_nyota:
        ; Hifadhi thamani ya upande wa kulia (tayari iko kwenye rax)
        mov     al, 0x50                        ; push rax — thamani
        call    gen_baiti

        mov     r12d, [ast_kushoto + r13*4]     ; nodi ya mtoto (pointer)

        ; Amua ukubwa wa kipengele (r15d) — chaguo-msingi: baiti 4 (N32)
        mov     r15d, 4
        mov     ebx, [ast_aina + r12*4]
        cmp     ebx, AST_JINA
        jne     .an_size_done

        ; Ni AST_JINA — tafuta kwenye orodha ya vigezo vya ndani
        mov     r9d, [ast_jina_off + r12*4]
        lea     r9, [str_pool + r9]
        xor     r10d, r10d
.an_scan_locals:
        cmp     r10, [local_count]
        jae     .an_scan_globals                ; haikupatikana ndani — jaribu ulimwengu
        mov     rdi, [local_name + r10*8]
        mov     rsi, r9
        call    linganisha_mfuatano
        cmp     eax, 0
        je      .an_found_local
        inc     r10
        jmp     .an_scan_locals

.an_scan_globals:
        ; Tafuta kati ya vigezo vya ulimwengu
        xor     r10d, r10d
.an_scan_globals_loop:
        cmp     r10, [global_count]
        jae     .an_size_done                   ; haikupatikana kabisa — chaguo-msingi
        mov     rdi, [global_name + r10*8]
        mov     rsi, r9
        call    linganisha_mfuatano
        cmp     eax, 0
        je      .an_found_global
        inc     r10
        jmp     .an_scan_globals_loop

.an_found_local:
        ; Kigezo cha ndani — pointer ikiwa star_count > 0
        mov     r11d, [local_star_count + r10*4]
        cmp     r11d, 0
        je      .an_size_done                   ; sio pointer — chaguo-msingi N32
        mov     r11d, [local_base_type + r10*4]
        cmp     r11d, 1
        je      .an_set_size_1
        cmp     r11d, 2
        je      .an_set_size_2
        cmp     r11d, 4
        je      .an_set_size_8
        cmp     r11d, 5
        je      .an_set_size_8
        mov     r15d, 4
        jmp     .an_size_done

.an_found_global:
        ; Kigezo cha ulimwengu — pointer au safu ni anwani, tumia aina msingi
        mov     r11d, [global_star_count + r10*4]
        cmp     r11d, 0
        jg      .an_global_base                 ; pointer — tumia aina msingi
        cmp     dword [global_is_array + r10*4], 0
        jne     .an_global_base                 ; safu — anwani, tumia aina msingi
        mov     r15d, 4
        jmp     .an_size_done                   ; kigeu rahisi — chaguo-msingi

.an_global_base:
        mov     r11d, [global_base_type + r10*4]
        cmp     r11d, 1
        je      .an_set_size_1
        cmp     r11d, 2
        je      .an_set_size_2
        cmp     r11d, 4
        je      .an_set_size_8
        cmp     r11d, 5
        je      .an_set_size_8
        mov     r15d, 4
        jmp     .an_size_done

.an_set_size_1:
        mov     r15d, 1
        jmp     .an_size_done
.an_set_size_2:
        mov     r15d, 2
        jmp     .an_size_done
.an_set_size_8:
        mov     r15d, 8
        ; angukia .an_size_done

.an_size_done:
        ; Tathmini usemi wa pointer
        push    r8
        push    r15
        call    uzalishaji_ast                  ; eax = anwani (pointer)
        pop     r15
        pop     r8

        ; pop rcx — rudisha thamani
        mov     al, 0x59
        call    gen_baiti

        ; Hifadhi [rax], rcx kulingana na ukubwa
        cmp     r15d, 1
        je      .an_store_8
        cmp     r15d, 2
        je      .an_store_16
        cmp     r15d, 8
        je      .an_store_64

        ; N32: mov [rax], ecx -> 89 08
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x08                        ; ModRM: [rax], reg=ecx
        call    gen_baiti
        mov     eax, r8d                        ; rudisha thamani iliyowekwa
        jmp     .done

.an_store_8:
        ; mov [rax], cl -> 88 08
        mov     al, 0x88
        call    gen_baiti
        mov     al, 0x08
        call    gen_baiti
        mov     eax, r8d
        jmp     .done

.an_store_16:
        ; mov [rax], cx -> 66 89 08
        mov     al, 0x66
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x08
        call    gen_baiti
        mov     eax, r8d
        jmp     .done

.an_store_64:
        ; mov [rax], rcx -> 48 89 08
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x08
        call    gen_baiti
        mov     eax, r8d
        jmp     .done

        ; Uwekaji wa mwanachama wa muundo — m.chanzo = ... au p->inayofuata = ...
.assign_mwanachama:
        ; Hifadhi thamani ya upande wa kulia (tayari iko kwenye rax)
        mov     al, 0x50                ; push rax — thamani
        call    gen_baiti

        ; Amua aina ya mwanachama kwa wakati wa kukusanya
        push    r8
        push    r13
        push    r14
        push    r15
        mov     r12d, r13d
        call    fumbua_aina             ; eax=aina, ebx=nyota, edx=muundo
        mov     r9d, eax                ; aina ya msingi
        mov     r10d, ebx               ; idadi ya nyota
        mov     r11d, edx               ; id ya muundo (-1 ikiwa si muundo)
        pop     r15
        pop     r14
        pop     r13
        pop     r8

        ; Nakala ya muundo mzima? (aina 6, bila nyota)
        cmp     r9d, 6
        jne     .am_kawaida
        cmp     r10d, 0
        je      .am_nakala

.am_kawaida:
        ; Tathmini anwani ya mwanachama (matokeo kwenye rax)
        push    r8
        push    r9
        push    r10
        push    r13
        push    r15
        mov     r12d, r13d
        call    uzalishaji_anwani_ya_nodi   ; rax = anwani ya mwanachama
        pop     r15
        pop     r13
        pop     r10
        pop     r9
        pop     r8

        ; pop rcx — rudisha thamani kutoka kwenye rafu
        mov     al, 0x59
        call    gen_baiti

        ; Hifadhi [rax], rcx kulingana na ukubwa wa aina
        cmp     r10d, 0
        jg      .am_store_64            ; pointer — baiti 8
        cmp     r9d, 7
        je      .am_store_d64
        cmp     r9d, 1
        je      .am_store_8
        cmp     r9d, 2
        je      .am_store_16
        cmp     r9d, 4
        je      .am_store_64
        cmp     r9d, 5
        je      .am_store_64
        ; N32: mov [rax], ecx -> 89 08
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x08
        call    gen_baiti
        mov     eax, r8d
        jmp     .done

.am_store_8:
        ; mov [rax], cl -> 88 08
        mov     al, 0x88
        call    gen_baiti
        mov     al, 0x08
        call    gen_baiti
        mov     eax, r8d
        jmp     .done

.am_store_16:
        ; mov [rax], cx -> 66 89 08
        mov     al, 0x66
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x08
        call    gen_baiti
        mov     eax, r8d
        jmp     .done

.am_store_64:
        ; mov [rax], rcx -> 48 89 08
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x08
        call    gen_baiti
        mov     eax, r8d
        jmp     .done

.am_store_d64:
        ; movsd [rax], xmm0 -> f2 0f 11 00
        mov     al, 0xF2
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x11
        call    gen_baiti
        mov     al, 0x00
        call    gen_baiti
        mov     eax, r8d
        jmp     .done

.am_nakala:
        ; Nakala ya muundo mzima: chanzo (anwani) iko kwenye rafu,
        ; lengo ni anwani ya mwanachama
        push    r8
        push    r11
        push    r13
        push    r15
        mov     r12d, r13d
        call    uzalishaji_anwani_ya_nodi   ; rax = anwani ya lengo
        pop     r15
        pop     r13
        pop     r11
        pop     r8

        ; mov rdi, rax — lengo
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0xC7
        call    gen_baiti
        ; pop rsi — chanzo
        mov     al, 0x5E
        call    gen_baiti
        ; mov ecx, ukubwa wa muundo
        mov     al, 0xB9
        call    gen_baiti
        mov     edi, [muundo_ukubwa + r11*4]
        call    gen_neno4
        ; rep movsb
        mov     al, 0xF3
        call    gen_baiti
        mov     al, 0xA4
        call    gen_baiti
        mov     eax, r8d
        jmp     .done

.assign_err:
        ; Uwekaji usiotumika — arifu na usitishe
        lea     rdi, [msg_assignerr]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

.do_string:
        ; r13d = -1 (kiashiria cha mfuatano)
        ; r14d = -1 (haitumiki)
        ; r15d = urefu wa baiti (pamoja na '\0')
        ; r12d = faharisi ya nodi ya AST

        push    rsi
        push    rdi
        push    rcx

        mov     r8d, [ast_jina_off + r12*4]   ; ofseti ya mfuatano kwenye str_pool
        mov     r9d, r15d                      ; urefu wa baiti

        ; Nakili mfuatano kutoka str_pool hadi data_buf
        mov     rcx, [data_buf_pos]
        mov     r10d, ecx                      ; hifadhi ofseti kwa addend
        mov     ecx, r9d
        mov     rsi, str_pool
        add     rsi, r8
        lea     rdi, [data_buf + r10]
        cld
        rep     movsb
        mov     rcx, r9
        add     [data_buf_pos], rcx

        pop     rcx
        pop     rdi
        pop     rsi

        ; LEA RAX, [RIP + disp32] — pakia anwani ya mfuatano
        ; 48 8d 05 XX XX XX XX
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x8d
        call    gen_baiti
        mov     al, 0x05
        call    gen_baiti

        ; Ongeza rekebisho la .data
        mov     rdi, [rela_count]
        cmp     rdi, MAX_RELOCS - 1
        jae     .string_rela_jaa
        mov     edx, [text_buf_pos]
        mov     [rela_offset + rdi*4], edx
        mov     dword [rela_sym + rdi*4], -1    ; kiashiria: tumia .data section
        sub     r10d, 4
        mov     [rela_addend + rdi*4], r10d     ; addend = data_offset - 4
        inc     qword [rela_count]
.skip_string_reloc:
        ; Weka nafasi ya disp32 (baiti 4 za sifuri)
        mov     edi, 0
        call    gen_neno4
        jmp     .string_rela_endelea
.string_rela_jaa:
        ; Kosa LAUTI — uharibifu wa kimya hauruhusiwi
        lea     rdi, [msg_rela_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit
.string_rela_endelea:

        ; Ulinganisho wa mifuatano hauwezi kukunjwa wakati wa kukusanya
        mov     eax, -1
        jmp     .done
.do_and_sc:
        ; && kwa mzunguko mfupi:
        ;   kushoto; test; je si_kweli; kulia; test; je si_kweli;
        ;   mov eax,1; jmp mwisho; si_kweli: mov eax,0; mwisho:
        push    r12
        mov     r12d, r13d              ; kushoto
        call    uzalishaji_ast
        pop     r12
        mov     r8d, eax                ; CT ya kushoto

        ; test eax, eax -> 85 c0
        mov     al, 0x85
        call    gen_baiti
        mov     al, 0xc0
        call    gen_baiti
        ; je -> 0f 84 + kishikilia
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0x84
        call    gen_baiti
        mov     r9d, [text_buf_pos]     ; fixup 1: baada ya kushoto
        xor     edi, edi
        call    gen_neno4

        push    r8                      ; CT ya kushoto
        push    r9                      ; fixup 1
        push    r12
        mov     r12d, r14d              ; kulia
        call    uzalishaji_ast
        pop     r12
        pop     r9
        pop     r8
        mov     r10d, eax               ; CT ya kulia

        ; test eax, eax
        mov     al, 0x85
        call    gen_baiti
        mov     al, 0xc0
        call    gen_baiti
        ; je -> 0f 84 + kishikilia
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0x84
        call    gen_baiti
        mov     r11d, [text_buf_pos]    ; fixup 2: baada ya kulia
        xor     edi, edi
        call    gen_neno4

        ; mov eax, 1 -> b8 01 00 00 00
        mov     al, 0xb8
        call    gen_baiti
        mov     edi, 1
        call    gen_neno4
        ; jmp -> e9 + kishikilia
        mov     al, 0xe9
        call    gen_baiti
        mov     r15d, [text_buf_pos]    ; fixup 3: jmp ya mwisho
        xor     edi, edi
        call    gen_neno4

        ; Rekebisha fixup 1 na 2: elekeza kwenye si_kweli (hapa)
        mov     edi, [text_buf_pos]
        sub     edi, r9d
        sub     edi, 4
        mov     eax, [text_buf_pos]     ; hifadhi nafasi ya sasa
        mov     [text_buf_pos], r9d
        call    gen_neno4
        mov     [text_buf_pos], eax
        mov     edi, [text_buf_pos]
        sub     edi, r11d
        sub     edi, 4
        mov     eax, [text_buf_pos]
        mov     [text_buf_pos], r11d
        call    gen_neno4
        mov     [text_buf_pos], eax

        ; si_kweli: mov eax, 0 -> b8 00 00 00 00
        mov     al, 0xb8
        call    gen_baiti
        xor     edi, edi
        call    gen_neno4

        ; Rekebisha fixup 3: jmp ya mwisho -> hapa (mwisho)
        mov     edi, [text_buf_pos]
        sub     edi, r15d
        sub     edi, 4
        mov     eax, [text_buf_pos]
        mov     [text_buf_pos], r15d
        call    gen_neno4
        mov     [text_buf_pos], eax

        ; CT: kushoto ni 0 -> 0; kushoto si 0 -> (kulia si 0 -> 1, sivyo 0)
        cmp     r8d, -1
        je      .and_ct_isiyojulikana
        test    r8d, r8d
        jz      .and_ct_sifuri
        cmp     r10d, -1
        je      .and_ct_isiyojulikana
        test    r10d, r10d
        jz      .and_ct_sifuri
        mov     eax, 1
        jmp     .done
.and_ct_sifuri:
        xor     eax, eax
        jmp     .done
.and_ct_isiyojulikana:
        mov     eax, -1
        jmp     .done
.do_or_sc:
        ; || kwa mzunguko mfupi:
        ;   kushoto; test; jne kweli; kulia; test; jne kweli;
        ;   mov eax,0; jmp mwisho; kweli: mov eax,1; mwisho:
        push    r12
        mov     r12d, r13d              ; kushoto
        call    uzalishaji_ast
        pop     r12
        mov     r8d, eax                ; CT ya kushoto

        ; test eax, eax -> 85 c0
        mov     al, 0x85
        call    gen_baiti
        mov     al, 0xc0
        call    gen_baiti
        ; jne -> 0f 85 + kishikilia
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0x85
        call    gen_baiti
        mov     r9d, [text_buf_pos]     ; fixup 1: baada ya kushoto
        xor     edi, edi
        call    gen_neno4

        push    r8                      ; CT ya kushoto
        push    r9                      ; fixup 1
        push    r12
        mov     r12d, r14d              ; kulia
        call    uzalishaji_ast
        pop     r12
        pop     r9
        pop     r8
        mov     r10d, eax               ; CT ya kulia

        ; test eax, eax
        mov     al, 0x85
        call    gen_baiti
        mov     al, 0xc0
        call    gen_baiti
        ; jne -> 0f 85 + kishikilia
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0x85
        call    gen_baiti
        mov     r11d, [text_buf_pos]    ; fixup 2: baada ya kulia
        xor     edi, edi
        call    gen_neno4

        ; mov eax, 0 -> b8 00 00 00 00
        mov     al, 0xb8
        call    gen_baiti
        xor     edi, edi
        call    gen_neno4
        ; jmp -> e9 + kishikilia
        mov     al, 0xe9
        call    gen_baiti
        mov     r15d, [text_buf_pos]    ; fixup 3: jmp ya mwisho
        xor     edi, edi
        call    gen_neno4

        ; Rekebisha fixup 1 na 2: elekeza kwenye kweli (hapa)
        mov     edi, [text_buf_pos]
        sub     edi, r9d
        sub     edi, 4
        mov     eax, [text_buf_pos]     ; hifadhi nafasi ya sasa
        mov     [text_buf_pos], r9d
        call    gen_neno4
        mov     [text_buf_pos], eax
        mov     edi, [text_buf_pos]
        sub     edi, r11d
        sub     edi, 4
        mov     eax, [text_buf_pos]
        mov     [text_buf_pos], r11d
        call    gen_neno4
        mov     [text_buf_pos], eax

        ; kweli: mov eax, 1 -> b8 01 00 00 00
        mov     al, 0xb8
        call    gen_baiti
        mov     edi, 1
        call    gen_neno4

        ; Rekebisha fixup 3: jmp ya mwisho -> hapa (mwisho)
        mov     edi, [text_buf_pos]
        sub     edi, r15d
        sub     edi, 4
        mov     eax, [text_buf_pos]
        mov     [text_buf_pos], r15d
        call    gen_neno4
        mov     [text_buf_pos], eax

        ; CT: kushoto si 0 -> 1; kushoto ni 0 -> (kulia si 0 -> 1, sivyo 0)
        cmp     r8d, -1
        je      .or_ct_isiyojulikana
        test    r8d, r8d
        jnz     .or_ct_moja
        cmp     r10d, -1
        je      .or_ct_isiyojulikana
        test    r10d, r10d
        jnz     .or_ct_moja
        xor     eax, eax
        jmp     .done
.or_ct_moja:
        mov     eax, 1
        jmp     .done
.or_ct_isiyojulikana:
        mov     eax, -1
        jmp     .done

; -----------------------------------------------------------
; Njia za hesabu za kuelea (D64) — xmm0/xmm1
; -----------------------------------------------------------
.fl_rhs:
        ; Tathmini kulia → xmm0; hifadhi kwenye rafu kama baiti 8.
        push    r12
        push    r15
        mov     r12d, r14d
        call    uzalishaji_ast
        pop     r15
        pop     r12
        ; movq rax, xmm0 -> 66 48 0F 7E C0 ; push rax -> 50
        mov     al, 0x66
        call    gen_baiti
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x7E
        call    gen_baiti
        mov     al, 0xC0
        call    gen_baiti
        mov     al, 0x50
        call    gen_baiti
        ; Tathmini kushoto → xmm0.
        push    r12
        push    r15
        mov     r12d, r13d
        call    uzalishaji_ast
        pop     r15
        pop     r12
        ; pop rcx -> 59 ; movq xmm1, rcx -> 66 48 0F 6E C9
        mov     al, 0x59
        call    gen_baiti
        mov     al, 0x66
        call    gen_baiti
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x6E
        call    gen_baiti
        mov     al, 0xC9
        call    gen_baiti
        ret

.fl_add:
        call    .fl_rhs
        ; addsd xmm0, xmm1 -> f2 0f 58 c1
        mov     al, 0xF2
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x58
        call    gen_baiti
        mov     al, 0xC1
        call    gen_baiti
        mov     eax, -1
        jmp     .done
.fl_sub:
        call    .fl_rhs
        ; subsd xmm0, xmm1 -> f2 0f 5c c1
        mov     al, 0xF2
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x5C
        call    gen_baiti
        mov     al, 0xC1
        call    gen_baiti
        mov     eax, -1
        jmp     .done
.fl_mul:
        call    .fl_rhs
        ; mulsd xmm0, xmm1 -> f2 0f 59 c1
        mov     al, 0xF2
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x59
        call    gen_baiti
        mov     al, 0xC1
        call    gen_baiti
        mov     eax, -1
        jmp     .done
.fl_div:
        call    .fl_rhs
        ; divsd xmm0, xmm1 -> f2 0f 5e c1
        mov     al, 0xF2
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x5E
        call    gen_baiti
        mov     al, 0xC1
        call    gen_baiti
        mov     eax, -1
        jmp     .done
.fl_eq:
        call    .fl_rhs
        ; ucomisd xmm0, xmm1 -> 66 0f 2e c1
        mov     al, 0x66
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x2E
        call    gen_baiti
        mov     al, 0xC1
        call    gen_baiti
        ; sete al -> 0f 94 c0 ; movzx eax, al -> 0f b6 c0
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x94
        call    gen_baiti
        mov     al, 0xC0
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0xB6
        call    gen_baiti
        mov     al, 0xC0
        call    gen_baiti
        jmp     .done
.fl_ne:
        call    .fl_rhs
        mov     al, 0x66
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x2E
        call    gen_baiti
        mov     al, 0xC1
        call    gen_baiti
        ; setne al -> 0f 95 c0
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x95
        call    gen_baiti
        mov     al, 0xC0
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0xB6
        call    gen_baiti
        mov     al, 0xC0
        call    gen_baiti
        jmp     .done
.fl_lt:
        call    .fl_rhs
        mov     al, 0x66
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x2E
        call    gen_baiti
        mov     al, 0xC1
        call    gen_baiti
        ; setb al -> 0f 92 c0 (xmm0 < xmm1)
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x92
        call    gen_baiti
        mov     al, 0xC0
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0xB6
        call    gen_baiti
        mov     al, 0xC0
        call    gen_baiti
        jmp     .done
.fl_le:
        call    .fl_rhs
        mov     al, 0x66
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x2E
        call    gen_baiti
        mov     al, 0xC1
        call    gen_baiti
        ; setbe al -> 0f 96 c0
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x96
        call    gen_baiti
        mov     al, 0xC0
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0xB6
        call    gen_baiti
        mov     al, 0xC0
        call    gen_baiti
        jmp     .done
.fl_gt:
        call    .fl_rhs
        mov     al, 0x66
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x2E
        call    gen_baiti
        mov     al, 0xC1
        call    gen_baiti
        ; seta al -> 0f 97 c0 (xmm0 > xmm1)
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x97
        call    gen_baiti
        mov     al, 0xC0
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0xB6
        call    gen_baiti
        mov     al, 0xC0
        call    gen_baiti
        jmp     .done
.fl_ge:
        call    .fl_rhs
        mov     al, 0x66
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x2E
        call    gen_baiti
        mov     al, 0xC1
        call    gen_baiti
        ; setae al -> 0f 93 c0
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x93
        call    gen_baiti
        mov     al, 0xC0
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0xB6
        call    gen_baiti
        mov     al, 0xC0
        call    gen_baiti
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
        ; Hoja zote zinatathminiwa na kusukumwa — hoja 7+ zitabaki rafu

        push    r12
        push    r13
        push    r14
        push    r15
        push    r8
        push    r9

        mov     r12d, r8d
        call    uzalishaji_ast         ; matokeo kwenye eax

        ; D64 kwenye wito: ABI ya xmm0-xmm7 bado haijatekelezwa
        ; kwenye mbegu (mnyororo wa .swa unayo). Kosa LAUTI badala ya
        ; thamani isiyo sahihi kimya.
        mov     r12d, r8d
        call    fumbua_aina
        cmp     eax, 7
        jne     .hoja_sio_d64
        lea     rdi, [msg_d64_wito]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit
.hoja_sio_d64:

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
        ; Hoja zilisukumwa kwa mpangilio: arg1, arg2, ..., argN
        ; Rafu sasa: [argN, ..., arg2, arg1] (juu kwenda chini)
        ;
        ; Makubaliano ya mpokeaji (na ya ABI): rejista = hoja 1..6
        ; (rdi, rsi, rdx, rcx, r8, r9); hoja 7..N kwenye rafu kwa
        ; mpangilio wa kinyume, arg7 juu kabisa wakati wa wito.
        ;
        ; Kwa N > 6: toa hoja za ziada (argN..arg7) kwenye rejista za
        ; muda (rax, r10, r11), kisha toa sita kwenye r9..rdi, kisha
        ; rudisha za ziada kwa mpangilio wa kutoa ili arg7 ibaki juu.
        ; Rejista za muda hazina thamani hai hapa: matokeo ya hoja
        ; yako rafu, na lea ya sret hutolewa baadaye kwenye .do_call.

        cmp     r15d, 6
        jbe     .pop_regs
        ; Kikomo cha mbegu: hoja 9 (rejista tatu za muda). Maktaba ya
        ; .swa haizidi hoja 9 — andika_elf_shdr ndiyo kazi kubwa zaidi.
        cmp     r15d, 9
        jbe     .scratch_pop
        lea     rdi, [msg_hoja9]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit
.scratch_pop:
        ; pop rax = 58
        mov     al, 0x58
        call    gen_baiti
        cmp     r15d, 7
        je      .pop_regs
        ; pop r10 = 41 5A
        mov     al, 0x41
        call    gen_baiti
        mov     al, 0x5A
        call    gen_baiti
        cmp     r15d, 8
        je      .pop_regs
        ; pop r11 = 41 5B
        mov     al, 0x41
        call    gen_baiti
        mov     al, 0x5B
        call    gen_baiti
.pop_regs:
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
        ; Rudisha hoja za ziada kwenye rafu kwa mpangilio uleule wa kutoa:
        ; kusukuma argN kwanza, arg7 mwisho, ili arg7 ibaki juu ya rafu.
        cmp     r15d, 6
        jbe     .sio_rudisha
        ; push rax = 50
        mov     al, 0x50
        call    gen_baiti
        cmp     r15d, 7
        je      .sio_rudisha
        ; push r10 = 41 52
        mov     al, 0x41
        call    gen_baiti
        mov     al, 0x52
        call    gen_baiti
        cmp     r15d, 8
        je      .sio_rudisha
        ; push r11 = 41 53
        mov     al, 0x41
        call    gen_baiti
        mov     al, 0x53
        call    gen_baiti
.sio_rudisha:
        ; Builtin ya wito_wa_mfumo — badala ya wito halisi, pangilia hoja
        ; kwa ABI ya syscall (sawa na builtin ya uzalishaji.swa):
        ; rax=namba, rdi=a1, rsi=a2, rdx=a3, r10=a4, r8=a5, r9=0.
        ; Hoja ya 7 (ikiwa ipo) inatolewa na pop r9.
        mov     rdi, r13
        lea     rsi, [jina_wito_mfumo]
        call    linganisha_mfuatano
        cmp     eax, 0
        jne     .sio_builtin_syscall
        ; mov r10, r8 — 4D 89 C2 (a4: hoja ya 5)
        mov     al, 0x4D
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0xC2
        call    gen_baiti
        ; mov rax, rdi — 48 89 F8 (namba ya syscall)
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0xF8
        call    gen_baiti
        ; mov rdi, rsi — 48 89 F7 (a1)
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0xF7
        call    gen_baiti
        ; mov rsi, rdx — 48 89 D6 (a2)
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0xD6
        call    gen_baiti
        ; mov rdx, rcx — 48 89 CA (a3)
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0xCA
        call    gen_baiti
        ; pop r9 — 41 59 (hoja ya 7: ofseti, daima 0 kwenye maktaba)
        mov     al, 0x41
        call    gen_baiti
        mov     al, 0x59
        call    gen_baiti
        ; syscall — 0F 05
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x05
        call    gen_baiti
        jmp     .baada_ya_wito
.sio_builtin_syscall:
        ; Builtin ya tekeleza — ita bafa ya JIT kama kazi N32(N32, N8**).
        ; Hoja (kazi, argc, argv, ofseti) ziko rdi/rsi/rdx/rcx kwa
        ; mpangilio wa kawaida → r11=kazi, rdi=argc, rsi=argv+ofseti*8,
        ; al=0, call r11. Sawa na builtin ya uzalishaji.swa.
        mov     rdi, r13
        lea     rsi, [jina_tekeleza]
        call    linganisha_mfuatano
        cmp     eax, 0
        jne     .sio_builtin_tekeleza
        ; mov r11, rdi — 49 89 FB
        mov     al, 0x49
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0xFB
        call    gen_baiti
        ; mov rdi, rsi — 48 89 F7
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0xF7
        call    gen_baiti
        ; mov rsi, rdx — 48 89 D6
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0xD6
        call    gen_baiti
        ; mov rdx, rcx — 48 89 CA
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0xCA
        call    gen_baiti
        ; shl rdx, 3 — 48 C1 E2 03
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0xC1
        call    gen_baiti
        mov     al, 0xE2
        call    gen_baiti
        mov     al, 3
        call    gen_baiti
        ; add rsi, rdx — 48 01 D6
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x01
        call    gen_baiti
        mov     al, 0xD6
        call    gen_baiti
        ; mov al, 0 — B0 00
        mov     al, 0xB0
        call    gen_baiti
        mov     al, 0
        call    gen_baiti
        ; call r11 — 41 FF D3
        mov     al, 0x41
        call    gen_baiti
        mov     al, 0xFF
        call    gen_baiti
        mov     al, 0xD3
        call    gen_baiti
        jmp     .baada_ya_wito
.sio_builtin_tekeleza:
        ; Kwa wito wa kazi inayorudisha muundo, weka r10 = eneo la muda la .data
        push    r12
        push    r8
        push    r9
        push    r10
        push    r11
        push    rdi
        push    rsi
        xor     r8d, r8d
.cr_scan:
        cmp     r8, [kazi_ret_idadi]
        jae     .cr_sio
        mov     rdi, r13
        mov     r9d, [kazi_ret_jina + r8*4]
        lea     rsi, [str_pool + r9]
        call    linganisha_mfuatano
        cmp     eax, 0
        je      .cr_ipatikana
        inc     r8
        jmp     .cr_scan
.cr_ipatikana:
        mov     edi, [kazi_ret_muundo_jina + r8*4]
        call    tafuta_muundo
        cmp     eax, -1
        je      .cr_sio
        ; rax = faharisi ya muundo
        mov     r9d, [muundo_ukubwa + rax*4]
        mov     r10, [data_buf_pos]
        mov     r11d, r10d
        add     [data_buf_pos], r9
        cmp     qword [data_buf_pos], DATA_BUF_SIZE
        jbe     .cr_nafasi
        lea     rdi, [msg_databuf]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit
.cr_nafasi:
        ; lea r10, [rip + disp32]
        mov     al, 0x4c
        call    gen_baiti
        mov     al, 0x8d
        call    gen_baiti
        mov     al, 0x15
        call    gen_baiti
        ; Rekebisho la .data — kama .do_string
        mov     rdi, [rela_count]
        cmp     rdi, MAX_RELOCS - 1
        jae     .cr_rela_jaa
        mov     edx, [text_buf_pos]
        mov     [rela_offset + rdi*4], edx
        mov     dword [rela_sym + rdi*4], -1
        sub     r11d, 4
        mov     [rela_addend + rdi*4], r11d
        inc     qword [rela_count]
        jmp     .cr_skip_reloc
.cr_rela_jaa:
        ; Kosa LAUTI — uharibifu wa kimya hauruhusiwi
        lea     rdi, [msg_rela_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit
.cr_skip_reloc:
        mov     edi, 0
        call    gen_neno4
.cr_sio:
        pop     rsi
        pop     rdi
        pop     r11
        pop     r10
        pop     r9
        pop     r8
        pop     r12

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
        ; Kosa LAUTI — kurejea faharisi 0 kimya ni uharibifu
        lea     rdi, [msg_extern_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit
.gen_reloc:
        ; Ongeza rekebisho
        mov     rdi, [rela_count]
        cmp     rdi, MAX_RELOCS - 1
        jae     .wito_rela_jaa
        mov     edx, [text_buf_pos]
        mov     [rela_offset + rdi*4], edx
        mov     [rela_sym + rdi*4], r14d
        mov     dword [rela_addend + rdi*4], -4
        inc     qword [rela_count]
.skip_reloc:
        ; Weka nafasi ya rel32 (baiti 4 za sifuri)
        mov     edi, 0
        call    gen_neno4
        jmp     .wito_rela_endelea
.wito_rela_jaa:
        ; Kosa LAUTI — uharibifu wa kimya hauruhusiwi
        lea     rdi, [msg_rela_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit
.wito_rela_endelea:

        ; Safisha hoja za rafu zaidi ya sita baada ya wito
        ; Hoja 7+ zilibaki rafu wakati wa call — ziondoe sasa
        cmp     r15d, 6
        jbe     .sio_kusafisha
        ; add rsp, imm32 = 48 81 C4 + (r15d - 6) * 8
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x81
        call    gen_baiti
        mov     al, 0xC4
        call    gen_baiti
        mov     edi, r15d
        sub     edi, 6
        imul    edi, 8
        call    gen_neno4
.sio_kusafisha:

        ; Matokeo yatakuwa kwenye eax baada ya wito
.baada_ya_wito:
        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret

; -------------------------------------------------------
; uzalishaji_vunja: zalisha msimbo kwa taarifa ya vunja
;   huongeza fixup ya jmp kwenye orodha ya break_fixup_pos
;   uzalishaji_wakati utarekebisha fixup hizi baadaye
; -------------------------------------------------------
uzalishaji_vunja:
        push    r12
        push    rdi

        ; jmp rel32 yenye kishikilia
        mov     al, 0xe9
        call    gen_baiti
        mov     r12d, [text_buf_pos]      ; nafasi ya fixup
        xor     edi, edi
        call    gen_neno4                  ; kishikilia cha baiti 4

        ; Ongeza nafasi ya fixup kwenye orodha
        mov     rax, [break_fixup_count]
        cmp     rax, 65536
        jae     .breakfix_jaa
        mov     [break_fixup_pos + rax*4], r12d
        inc     qword [break_fixup_count]
.overflow:
        pop     rdi
        pop     r12
        ret
.breakfix_jaa:
        lea     rdi, [msg_breakfix_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

; -------------------------------------------------------
; uzalishaji_endelea: zalisha endelea (jump hadi mwanzo wa kitanzi)
;   huongeza fixup ya jmp kwenye orodha ya continue_fixup_pos
;   uzalishaji_wakati utarekebisha fixup hizi baadaye
; -------------------------------------------------------
uzalishaji_endelea:
        push    r12
        push    rdi

        ; jmp rel32 yenye kishikilia
        mov     al, 0xe9
        call    gen_baiti
        mov     r12d, [text_buf_pos]      ; nafasi ya fixup
        xor     edi, edi
        call    gen_neno4                  ; kishikilia cha baiti 4

        ; Ongeza nafasi ya fixup kwenye orodha
        mov     rax, [continue_fixup_count]
        cmp     rax, 65536
        jae     .continuefix_jaa
        mov     [continue_fixup_pos + rax*4], r12d
        inc     qword [continue_fixup_count]
.overflow:
        pop     rdi
        pop     r12
        ret
.continuefix_jaa:
        lea     rdi, [msg_breakfix_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

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
        push    qword [local_count]     ; upeo wa kizuizi: hifadhi hesabu ya vigezo vya ndani
        push    r12
        push    r8                      ; hifadhi r8d (nafasi ya fixup ya je) — uzalishaji_ast inaharibu r8d
        mov     r12d, r14d
        call    uzalishaji_ast
        pop     r8
        pop     r12
        pop     qword [local_count]     ; rejesha upeo wa nje

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
        push    qword [local_count]     ; upeo wa kizuizi: hifadhi hesabu ya vigezo vya ndani
        push    r12
        push    r9                      ; hifadhi r9d (nafasi ya fixup ya jmp) — uzalishaji_ast inaharibu r9d
        mov     r12d, r15d
        call    uzalishaji_ast
        pop     r9
        pop     r12
        pop     qword [local_count]     ; rejesha upeo wa nje

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
; uzalishaji_wakati: zalisha msimbo kwa kitanzi cha wakati
;   r12d = faharisi ya nodi
;   hutumia break_fixup_pos kurekebisha vunja
; -------------------------------------------------------
uzalishaji_wakati:
        push    r12
        push    r13
        push    r14
        push    r15

        mov     r13d, [ast_kushoto + r12*4]  ; hali ya kitanzi
        mov     r14d, [ast_kulia + r12*4]    ; mwili wa kitanzi

        ; Anzisha ya kwa (ast_tiga) — itolewe kabla ya kitanzi
        mov     r15d, [ast_tiga + r12*4]
        cmp     r15d, -1
        je      .hakuna_anzisha
        push    r12
        push    r13
        push    r14
        mov     r12d, r15d
        call    uzalishaji_ast
        pop     r14
        pop     r13
        pop     r12
.hakuna_anzisha:

        ; Rekodi mwanzo wa kitanzi
        mov     r15d, [text_buf_pos]          ; start_pos

        ; Hifadhi hali ya vunja ya awali (kwa vitanzi vilivyo ndani)
        ; Heshima sasa inakuwa base — fixups za kitanzi hiki zitaanza hapa,
        ; si kwenye 0, ili zisiharibu nafasi za fixups za vitanzi vya nje
        mov     rax, [break_fixup_count]
        push    rax

        ; Hifadhi hali ya endelea ya awali (kwa vitanzi vilivyo ndani)
        ; Kama hapo juu: hesabu inakuwa base, hairejeshwi kwenye 0
        mov     rax, [continue_fixup_count]
        push    rax
        ; Alama ya has_step ya kwa (kulia ya block ya mwili)
        mov     r9d, 0
        cmp     dword [ast_kulia + r14*4], -777777
        jne     .sio_hatua_loop
        mov     r9d, 1
.sio_hatua_loop:
        push    r9

        ; Zalisha hali (kitanzi kipofu: ruka hali na jz)
        cmp     r13d, -1
        je      .kitanzi_kipofu
        push    r12
        mov     r12d, r13d
        call    uzalishaji_ast
        pop     r12

        ; test eax, eax
        mov     al, 0x85
        call    gen_baiti
        mov     al, 0xc0
        call    gen_baiti

        ; jz rel32 yenye kishikilia (ikiwa hali ni sifuri, toka kitanzi)
        mov     al, 0x0f
        call    gen_baiti
        mov     al, 0x84
        call    gen_baiti
        mov     r8d, [text_buf_pos]           ; nafasi ya fixup ya jz
        xor     edi, edi
        call    gen_neno4                      ; kishikilia cha baiti 4

.kitanzi_kipofu:

        ; Zalisha mwili wa kitanzi
        push    qword [local_count]     ; upeo wa kizuizi: hifadhi hesabu ya vigezo vya ndani
        push    r12
        push    r8
        push    r15
        mov     r12d, r14d
        call    uzalishaji_ast
        pop     r15
        pop     r8
        pop     r12
        pop     qword [local_count]     ; rejesha upeo wa nje

        ; jmp rel32 kurudi mwanzo wa kitanzi
        mov     al, 0xe9
        call    gen_baiti
        mov     r9d, [text_buf_pos]           ; nafasi ya fixup ya jmp
        xor     edi, edi
        call    gen_neno4                      ; kishikilia cha baiti 4

        ; Kokotoa ofseti ya kuruka nyuma: start_pos - fixup_pos - 4
        mov     edi, r15d                     ; start_pos
        sub     edi, r9d                      ; start_pos - fixup_pos
        sub     edi, 4                        ; start_pos - fixup_pos - 4
        mov     r10d, [text_buf_pos]
        mov     [text_buf_pos], r9d
        call    gen_neno4                      ; andika ofseti sahihi ya jmp
        mov     [text_buf_pos], r10d

        ; Rekebisha jz: elekeza mwisho wa kitanzi
        ; (kitanzi kipofu: hakuna jz — ruka; hali inasomwa upya kutoka AST
        ; kwa sababu r13d imeharibiwa na mkusanyaji wa mwili)
        cmp     dword [ast_kushoto + r12*4], -1
        je      .kitanzi_kipofu_jz_done
        mov     edi, [text_buf_pos]           ; mwisho wa kitanzi
        sub     edi, r8d                      ; umbali kutoka fixup
        sub     edi, 4                        ; toa ukubwa wa kishikilia
        mov     r10d, [text_buf_pos]
        mov     [text_buf_pos], r8d
        call    gen_neno4                      ; andika ofseti sahihi ya jz
        mov     [text_buf_pos], r10d
.kitanzi_kipofu_jz_done:

        ; Lengo la endelea: start_pos au nafasi ya hatua (semantiki ya C)
        pop     r9                         ; has_step
        mov     [cl_lengo], r15d           ; chaguo-msingi: start_pos
        test    r9d, r9d
        je      .sio_hatua_lengo
        mov     rax, [cl_hatua]
        mov     [cl_lengo], rax
.sio_hatua_lengo:

        ; Toa mipaka ya awali kutoka rafu
        pop     r14                        ; base ya endelea
        pop     r13                        ; base ya vunja

        ; Rekebisha vunja zote: zielekeze mwisho wa kitanzi
        ; Anza kutoka base, si 0, ili kuhifadhi fixups za vitanzi vya nje
        mov     r9d, [text_buf_pos]           ; lengo = mwisho wa kitanzi
        mov     ecx, r13d
.fix_breaks:
        cmp     ecx, [break_fixup_count]
        jae     .fix_breaks_done
        mov     edi, [break_fixup_pos + rcx*4] ; nafasi ya fixup ya vunja hii
        push    rcx
        mov     r10d, r9d
        sub     r10d, edi                      ; target - fixup_pos
        sub     r10d, 4                        ; target - fixup_pos - 4
        mov     r11d, [text_buf_pos]
        mov     [text_buf_pos], edi
        mov     edi, r10d
        call    gen_neno4                       ; andika ofseti sahihi ya jmp
        mov     [text_buf_pos], r11d
        pop     rcx
        inc     ecx
        jmp     .fix_breaks
.fix_breaks_done:
        mov     [break_fixup_count], r13       ; tupa fixups za ndani: rejesha base

        ; Rekebisha endelea zote: zielekeze mwanzo wa kitanzi
        mov     ecx, r14d
.fix_continues:
        cmp     ecx, [continue_fixup_count]
        jae     .fix_continues_done
        mov     edi, [continue_fixup_pos + rcx*4] ; nafasi ya fixup ya endelea hii
        push    rcx
        mov     r10d, [cl_lengo]                ; lengo = hatua au mwanzo
        sub     r10d, edi                        ; target - fixup_pos
        sub     r10d, 4                          ; target - fixup_pos - 4
        mov     r11d, [text_buf_pos]
        mov     [text_buf_pos], edi
        mov     edi, r10d
        call    gen_neno4                         ; andika ofseti sahihi ya jmp
        mov     [text_buf_pos], r11d
        pop     rcx
        inc     ecx
        jmp     .fix_continues
.fix_continues_done:
        mov     [continue_fixup_count], r14     ; tupa fixups za ndani: rejesha base

        xor     eax, eax                       ; wakati haitoi thamani
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
        push    qword [local_count]     ; upeo wa kizuizi: hifadhi hesabu ya vigezo vya ndani

        mov     r12d, r12d
        mov     r13d, [ast_kushoto + r12*4]  ; taarifa ya kwanza

.loop:
        cmp     r13d, -1
        je      .done

        ; Alama ya hatua ya kwa: rekodi nafasi yake kwa endelea
        cmp     dword [ast_kulia + r13*4], -777777
        jne     .sio_hatua_kw
        mov     rax, [text_buf_pos]
        mov     [cl_hatua], rax
.sio_hatua_kw:

        push    r12
        push    r13
        mov     r12d, r13d
        call    uzalishaji_ast
        pop     r13
        pop     r12

        mov     r13d, [ast_nne + r13*4]      ; taarifa inayofuata
        jmp     .loop
.done:
        pop     qword [local_count]     ; rejesha upeo wa nje
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

        ; Tambua kazi inayorudisha muundo (sret)
        cmp     r13d, 6
        jne     .sio_sret
        mov     r8d, [ast_tiga + r12*4]
        cmp     r8d, -1
        je      .sio_sret
        mov     dword [kazi_ret_aina], 6
        mov     edi, r8d
        call    tafuta_muundo
        mov     [kazi_ret_muundo_id], eax
        jmp     .sret_imewekwa
.sio_sret:
        mov     dword [kazi_ret_aina], 0
        mov     dword [kazi_ret_muundo_id], -1
.sret_imewekwa:

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
        ; Kosa LAUTI — kurejea faharisi 0 kimya ni uharibifu
        lea     rdi, [msg_label_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit
.gen_code:
        ; push r10 — hifadhi kielekezi cha eneo la kurudia
        ; (baada ya kusajili lebo, ili lebo iwe inafunika push r10)
        cmp     dword [kazi_ret_muundo_id], 0
        jl      .sio_push_r10
        mov     al, 0x41
        call    gen_baiti
        mov     al, 0x52
        call    gen_baiti
.sio_push_r10:

        ; Weka upya hesabu ya lebo za ndani kwa kila kazi
        ; (fixups hutatuliwa mwishoni mwa kila kazi, hivyo nambari
        ;  za lebo zinaweza kuanza upya hapa bila mgongano)
        mov     qword [gen_label_count], 0

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
        ; sub rsp, imm32 — tutaweka baada ya kujua ukubwa
        ; Kwa sasa, weka nafasi ya baiti 4 kwa imm32
        mov     r8d, [text_buf_pos]     ; hifadhi nafasi ya kurekebisha
        push    r8
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x81
        call    gen_baiti
        mov     al, 0xec
        call    gen_baiti
        mov     al, 0                    ; imm32 — baiti 4, zitajazwa baadaye
        call    gen_baiti
        mov     al, 0
        call    gen_baiti
        mov     al, 0
        call    gen_baiti
        mov     al, 0
        call    gen_baiti

        ; Hifadhi vigezo vya kazi (parameters) kwenye rafu
        push    r14                     ; hifadhi nodi ya mwili
        push    r13                     ; hifadhi faharisi ya lebo

        mov     r10d, [ast_kushoto + r12*4] ; orodha ya vigezo
        xor     r11d, r11d              ; faharisi ya hoja (0-5)
.param_loop:
        cmp     r10d, -1
        je      .params_done
        ; Hoja zote zinasajiliwa — zaidi ya 6 zimekuja kupitia rafu ya mwitaji

        ; Sajili kigezo kwenye orodha ya ndani
        mov     rdi, [local_count]
        mov     r15d, [ast_jina_off + r10*4]
        lea     rcx, [str_pool + r15]
        mov     [local_name + rdi*8], rcx

        ; Kokotoa ofseti ya rafu: (local_count + 1) * 8
        mov     r8d, edi
        inc     r8d
        imul    r8d, 8
        mov     [local_offset + rdi*4], r8d
        ; Hifadhi aina msingi na idadi ya nyota kwa hoja pia
        mov     r15d, [ast_thamani + r10*4]
        mov     [local_base_type + rdi*4], r15d
        mov     r15d, [ast_tiga + r10*4]
        mov     [local_star_count + rdi*4], r15d
        ; Kigezo kinaweza kuwa cha aina ya muundo — tafuta kitambulisho chake
        mov     r15d, -1
        cmp     dword [local_base_type + rdi*4], 6    ; 6 = aina ya muundo
        jne     .param_sio_muundo
        mov     r15d, [ast_kushoto + r10*4]           ; ofseti ya jina la muundo au -1
        cmp     r15d, -1
        je      .param_sio_muundo
        push    r10                    ; hifadhi hali ya mzunguko
        push    r11
        push    rdi
        push    r12
        mov     edi, r15d
        call    tafuta_muundo          ; eax = kitambulisho cha muundo au -1
        pop     r12
        pop     rdi
        pop     r11
        pop     r10
        mov     r15d, eax
.param_sio_muundo:
        mov     [local_muundo_id + rdi*4], r15d
        mov     dword [local_array_size + rdi*4], 0   ; si safu ya ndani
        inc     qword [local_count]

        ; Toa maelekezo ya kuhifadhi hoja kwenye rafu
        ; Amua kama tunahitaji kuhifadhi baiti 8 (nyota/N64/W0) au 4 (N32/chaguo-msingi)
        neg     r8d                     ; ofseti hasi kwa rbp

        ; Hoja ya 7 au zaidi inakaa kwenye rafu ya mwitaji, si kwenye rejista
        cmp     r11d, 6
        jae     .store_stack_param

        ; Angalia kama kigezo ni cha baiti 8
        mov     r14d, [ast_tiga + r10*4]    ; star_count
        cmp     r14d, 0
        jg      .store_64
        mov     r14d, [ast_thamani + r10*4]  ; base_type
        cmp     r14d, 4                      ; N64
        je      .store_64
        cmp     r14d, 5                      ; W0
        je      .store_64
        cmp     r14d, 2                      ; N16
        je      .store_16
        cmp     r14d, 1                      ; N8
        je      .store_8
        ; N32 — anguka hadi .store_32

.store_32:
        cmp     r11d, 0
        jne     .try_arg1_32
        ; mov [rbp + disp8], edi → 89 7D XX
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x7D
        call    gen_baiti
        mov     al, r8b
        call    gen_baiti
        jmp     .next_param
.try_arg1_32:
        cmp     r11d, 1
        jne     .try_arg2_32
        ; mov [rbp + disp8], esi → 89 75 XX
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x75
        call    gen_baiti
        mov     al, r8b
        call    gen_baiti
        jmp     .next_param
.try_arg2_32:
        cmp     r11d, 2
        jne     .try_arg3_32
        ; mov [rbp + disp8], edx → 89 55 XX
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x55
        call    gen_baiti
        mov     al, r8b
        call    gen_baiti
        jmp     .next_param
.try_arg3_32:
        cmp     r11d, 3
        jne     .try_arg4_32
        ; mov [rbp + disp8], ecx → 89 4D XX
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x4D
        call    gen_baiti
        mov     al, r8b
        call    gen_baiti
        jmp     .next_param
.try_arg4_32:
        cmp     r11d, 4
        jne     .try_arg5_32
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
.try_arg5_32:
        ; mov [rbp + disp8], r9d → 44 89 4D XX
        mov     al, 0x44
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x4D
        call    gen_baiti
        mov     al, r8b
        call    gen_baiti
        jmp     .next_param

; -------------------------------------------------------
; Hoja ya 7 au zaidi: haziko kwenye rejista bali kwenye rafu
; ya mwitaji. Baada ya push rbp (na push r10 kwa sret):
; hoja i (1-basi) iko kwenye [rbp + 8*(i-5) + (sret ? 8 : 0)]
; -------------------------------------------------------
.store_stack_param:
        ; Kokotoa ofseti chanya: 8 * (r11d - 4), +8 kwa sret
        mov     r9d, r11d
        sub     r9d, 4
        shl     r9d, 3
        cmp     dword [kazi_ret_muundo_id], 0
        jl      .stack_param_hakuna_sret
        add     r9d, 8
.stack_param_hakuna_sret:

        ; Angalia kama kigezo ni cha baiti 8
        mov     r14d, [ast_tiga + r10*4]
        cmp     r14d, 0
        jg      .stack_param_64
        mov     r14d, [ast_thamani + r10*4]
        cmp     r14d, 4                      ; N64
        je      .stack_param_64
        cmp     r14d, 5                      ; W0
        je      .stack_param_64

        ; mov eax, [rbp + disp8] — 8B 45 XX
        mov     al, 0x8B
        call    gen_baiti
        mov     al, 0x45
        call    gen_baiti
        mov     al, r9b
        call    gen_baiti
        ; mov [rbp + disp8], eax — 89 45 YY
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x45
        call    gen_baiti
        mov     al, r8b
        call    gen_baiti
        jmp     .next_param

.stack_param_64:
        ; mov rax, [rbp + disp8] — 48 8B 45 XX
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x8B
        call    gen_baiti
        mov     al, 0x45
        call    gen_baiti
        mov     al, r9b
        call    gen_baiti
        ; mov [rbp + disp8], rax — 48 89 45 YY
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x45
        call    gen_baiti
        mov     al, r8b
        call    gen_baiti
        jmp     .next_param

.store_64:
        cmp     r11d, 0
        jne     .try_arg1_64
        ; mov [rbp + disp8], rdi → 48 89 7D XX
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x7D
        call    gen_baiti
        mov     al, r8b
        call    gen_baiti
        jmp     .next_param
.try_arg1_64:
        cmp     r11d, 1
        jne     .try_arg2_64
        ; mov [rbp + disp8], rsi → 48 89 75 XX
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x75
        call    gen_baiti
        mov     al, r8b
        call    gen_baiti
        jmp     .next_param
.try_arg2_64:
        cmp     r11d, 2
        jne     .try_arg3_64
        ; mov [rbp + disp8], rdx → 48 89 55 XX
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x55
        call    gen_baiti
        mov     al, r8b
        call    gen_baiti
        jmp     .next_param
.try_arg3_64:
        cmp     r11d, 3
        jne     .try_arg4_64
        ; mov [rbp + disp8], rcx → 48 89 4D XX
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x4D
        call    gen_baiti
        mov     al, r8b
        call    gen_baiti
        jmp     .next_param
.try_arg4_64:
        cmp     r11d, 4
        jne     .try_arg5_64
        ; mov [rbp + disp8], r8 → 4C 89 45 XX
        mov     al, 0x4C
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x45
        call    gen_baiti
        mov     al, r8b
        call    gen_baiti
        jmp     .next_param
.try_arg5_64:
        ; mov [rbp + disp8], r9 → 4C 89 4D XX
        mov     al, 0x4C
        call    gen_baiti
        mov     al, 0x89
        call    gen_baiti
        mov     al, 0x4D
        call    gen_baiti
        mov     al, r8b
        call    gen_baiti
        jmp     .next_param

.store_8:
        ; Kwa N8, tunatumia kusajili kwa 32-bit cmpa mov, kisha kuhifadhi baiti
        ; Hii ni rahisi zaidi — tunahifadhi kama 32-bit cmpa mov kwa sasa
        ; TODO: hifadhi baiti moja kwa usahihi
        jmp     .store_32

.store_16:
        ; Kwa N16, tunahifadhi kama 32-bit kwa sasa
        ; TODO: hifadhi neno kwa usahihi
        jmp     .store_32
.next_param:
        inc     r11d
        mov     r10d, [ast_nne + r10*4]  ; kigezo kinachofuata
        jmp     .param_loop
.params_done:
        pop     r13                     ; rejesha faharisi ya lebo
        pop     r14                     ; rejesha nodi ya mwili

        ; frame_wapi: sehemu ya kwanza ya ndani ni baada ya vigezo vyote
        mov     rdi, [local_count]
        inc     rdi
        imul    rdi, 8
        mov     [frame_wapi], rdi

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
        mov     rdi, [frame_wapi]
        add     rdi, 15
        and     rdi, ~15                ; pangilia kwa 16

        ; Hoja za rafu zaidi ya 6: ikiwa idadi yao ni isiyo ya kawaida,
        ; mwito uliingia na rsp iliyobadilika kwa 8 — rekebisha fremu
        ; ili rsp iwe 16-aligned kabla ya miito ya ndani
        mov     r9d, 0
        mov     r10d, [ast_kushoto + r12*4]
.fr_count_params:
        cmp     r10d, -1
        je      .fr_counted
        inc     r9d
        mov     r10d, [ast_nne + r10*4]
        jmp     .fr_count_params
.fr_counted:
        cmp     r9d, 6
        jbe     .fr_hakuna_pad
        sub     r9d, 6
        test    r9b, 1
        jz      .fr_hakuna_pad
        ; Hoja za rafu zisizo za kawaida: non-sret anahitaji +8,
        ; sret anaingia tayari aligned — hakuna nyongeza kwake
        cmp     dword [kazi_ret_muundo_id], 0
        jge     .fr_imepita
        add     rdi, 8
        jmp     .fr_imepita
.fr_hakuna_pad:
        ; Kwa sret, push r10 tayari imetumia baiti 8 — ongeza ili mwili ubaki 16-aligned
        cmp     dword [kazi_ret_muundo_id], 0
        jl      .fr_imepita
        add     rdi, 8
.fr_imepita:
        ; r8 ina nafasi ya baiti ya sub rsp — tunaweka imm32
        mov     dword [text_buf + r8 + 3], edi

        ; Weka lebo ya kurudi hapa — kabla ya epilogue
        mov     rdi, [gen_return_label]
        mov     esi, edi
        mov     edi, [text_buf_pos]
        call    gen_weka_lebo

        ; Kwa sret, rudisha kielekezi cha eneo la kurudia kwenye rax
        cmp     dword [kazi_ret_muundo_id], 0
        jl      .epilogue
        ; mov rax, [rbp + 8]
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x8b
        call    gen_baiti
        mov     al, 0x45
        call    gen_baiti
        mov     al, 0x08
        call    gen_baiti

        ; Epilogue
.epilogue:
        ; leave
        mov     al, 0xc9
        call    gen_baiti
        ; Kwa sret, toa nafasi ya r10 iliyohifadhiwa
        cmp     dword [kazi_ret_muundo_id], 0
        jl      .sio_sret_epilogue
        ; add rsp, 8
        mov     al, 0x48
        call    gen_baiti
        mov     al, 0x83
        call    gen_baiti
        mov     al, 0xc4
        call    gen_baiti
        mov     al, 0x08
        call    gen_baiti
.sio_sret_epilogue:
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
        cmp     ebx, AST_KIELELEZO
        je      .call_kielelezo
        cmp     ebx, AST_ALAMA_ELEKEZA
        je      .call_anwani_ya
        cmp     ebx, AST_NYOTA_ELEKEZA
        je      .call_nyota_ya
        cmp     ebx, AST_WAKATI
        je      .call_wakati
        cmp     ebx, AST_VUNJA
        je      .call_vunja

        cmp     ebx, AST_ENDELEA
        je      .call_endelea

        ; Wanachama wa muundo: m.chanzo (nukta) na p->inayofuata (mshale)
        cmp     ebx, AST_ELEKEZA_JINA
        je      .call_mwanachama
        cmp     ebx, AST_ENEKEZA_FUNGO
        je      .call_mwanachama

        ; Unari hasi: -usemi (AST_HASILI)
        cmp     ebx, AST_HASILI
        je      .call_hasili
        ; Kukanusha kimantiki: !usemi (AST_MAKOSA)
        cmp     ebx, AST_MAKOSA
        je      .call_makosa

        cmp     ebx, AST_HALISI_D
        je      .call_halisi_d

        ; Chaguo-msingi: rudisha 0
        xor     eax, eax
        jmp     .done

.call_halisi_d:
        call    uzalishaji_halisi_d
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
.call_kielelezo:
        call    uzalishaji_kielelezo
        jmp     .done
.call_anwani_ya:
        call    uzalishaji_anwani_ya
        jmp     .done
.call_nyota_ya:
        call    uzalishaji_nyota_ya
        jmp     .done
.call_wakati:
        call    uzalishaji_wakati
        xor     eax, eax
        jmp     .done
.call_vunja:
        call    uzalishaji_vunja
        xor     eax, eax
        jmp     .done

.call_endelea:
        call    uzalishaji_endelea
        xor     eax, eax
        jmp     .done
.call_mwanachama:
        call    uzalishaji_mwanachama
        jmp     .done

.call_hasili:
        mov     r12d, [ast_kushoto + r12*4]
        ; Kuelea? (operanda ni D64) — zidisha kwa -1.0
        ; (xorpd yenye operanda ya kumbukumbu imeonekana kuvunjika
        ; kwenye VM ya mtumiaji — SI_KERNEL — wakati movsd/mulsd m64
        ; zinafanya kazi; mulsd kwa -1.0 ni sawa na inayobebeka.)
        push    r12
        call    fumbua_aina
        pop     r12
        cmp     eax, 7
        jne     .hasili_kamili

        call    uzalishaji_ast          ; operanda → xmm0

        ; Thabiti ya -1.0 (0xBFF0000000000000) kwenye .data
        mov     rax, [data_buf_pos]
        add     rax, 7
        and     rax, ~7
        mov     rcx, rax                ; mwanzo wa thabiti
        add     rax, 8
        cmp     rax, DATA_BUF_SIZE
        ja      .hasili_data_jaa
        mov     [data_buf_pos], rax
        mov     dword [data_buf + rcx], 0
        mov     dword [data_buf + rcx + 4], 0xBFF00000

        ; mulsd xmm0, [disp32] -> f2 0f 59 05 d32
        mov     al, 0xF2
        call    gen_baiti
        mov     al, 0x0F
        call    gen_baiti
        mov     al, 0x59
        call    gen_baiti
        mov     al, 0x05
        call    gen_baiti
        ; RELA ya .data (addend = data_offset - 4)
        mov     rdi, [rela_count]
        cmp     rdi, MAX_RELOCS - 1
        jae     .hasili_rela_jaa
        mov     edx, [text_buf_pos]
        mov     [rela_offset + rdi*4], edx
        mov     dword [rela_sym + rdi*4], -1
        sub     ecx, 4
        mov     [rela_addend + rdi*4], ecx
        inc     qword [rela_count]
        mov     edi, 0
        call    gen_neno4

        mov     eax, -1                 ; CT haijulikani
        jmp     .done
.hasili_rela_jaa:
        lea     rdi, [msg_rela_full]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit
.hasili_data_jaa:
        lea     rdi, [msg_databuf]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit

.hasili_kamili:
        call    uzalishaji_ast
        ; neg eax → f7 d8
        mov     al, 0xf7
        call    gen_baiti
        mov     al, 0xd8
        call    gen_baiti
        neg     eax
        jmp     .done

.call_makosa:
        mov     r12d, [ast_kushoto + r12*4]
        call    uzalishaji_ast
        ; test eax,eax → 85 c0
        mov     al, 0x85
        call    gen_baiti
        mov     al, 0xc0
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
        test    eax, eax
        sete    al
        movzx   eax, al
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

        ; Alama ya sehemu: .text (faharisi 1)
        mov     edi, 0                  ; st_name = 0
        call    andika_neno4_moja_kwa_moja
        mov     dil, 3                  ; STB_LOCAL | STT_SECTION
        call    andika_baiti_moja_kwa_moja
        mov     dil, 0
        call    andika_baiti_moja_kwa_moja
        mov     di, 1                   ; st_shndx = .text
        call    andika_neno2_moja_kwa_moja
        mov     edi, 0                  ; st_value
        call    andika_neno8_moja_kwa_moja
        mov     edi, 0                  ; st_size
        call    andika_neno8_moja_kwa_moja

        ; Alama ya sehemu: .data (faharisi 2)
        mov     edi, 0                  ; st_name = 0
        call    andika_neno4_moja_kwa_moja
        mov     dil, 3                  ; STB_LOCAL | STT_SECTION
        call    andika_baiti_moja_kwa_moja
        mov     dil, 0
        call    andika_baiti_moja_kwa_moja
        mov     di, 2                   ; st_shndx = .data
        call    andika_neno2_moja_kwa_moja
        mov     edi, 0                  ; st_value
        call    andika_neno8_moja_kwa_moja
        mov     edi, 0                  ; st_size
        call    andika_neno8_moja_kwa_moja

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
        ; Angalia ikiwa ni rekebisho la .data (rela_sym == -1)
        cmp     edi, -1
        jne     .reloc_extern
        ; Rekebisho la .data: tumia faharisi ya sehemu 2 (.data)
        mov     edi, 2
        jmp     .reloc_info
.reloc_extern:
        ; Rekebisho la nje: faharisi = 3 + label_count + global_count + extern_idx
        add     edi, 3
        add     edi, [label_count]
        add     edi, [global_count]
.reloc_info:
        shl     rdi, 32
        or      rdi, 2
        call    andika_neno8_moja_kwa_moja
        ; r_addend kutoka rela_addend (inaweza kuwa hasi)
        movsxd  rdi, dword [rela_addend + r12*4]
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
        mov     qword [bss_size], 0
        mov     qword [local_count], 0
        mov     qword [gen_label_count], 0
        mov     qword [gen_fixup_count], 0

        ; Weka '\0' mwanzoni mwa bwawa la herufi
        mov     byte [str_pool], 0
        inc     qword [str_pool_pos]

        ; Angalia hoja za mstari wa amri
        pop     rax                     ; argc
        mov     [tmp_argc], rax
        cmp     rax, 1
        jle     .tumia_stdin

        ; Tuna hoja — argv[1] inaweza kuwa "--exe" au jina la faili
        pop     rdi                     ; argv[0] — ruka
        pop     rdi                     ; argv[1]
        lea     rsi, [jina_exe_flag]
        call    linganisha_mfuatano
        cmp     eax, 0
        jne     .soma_faili_argv1
        mov     byte [exe_mode], 1
        ; Kama kuna argv[2], soma faili — sivyo tumia stdin
        mov     rax, [tmp_argc]
        cmp     rax, 3
        jl      .tumia_stdin
        pop     rdi                     ; argv[2]
        call    soma_chanzo_kutoka_faili
        jmp     .anza_kukusanya

.soma_faili_argv1:
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
        ; Toa ELF kwa stdout — hali ya exe inatoa ET_EXEC tuli
        ; moja kwa moja (hakuna ld, hakuna libc): 0% bootstrap gap.
        cmp     byte [exe_mode], 0
        je      .toa_o
        call    toa_exe
        xor     edi, edi
        call    sys_exit
.toa_o:
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
; toa_exe: toa ET_EXEC tuli moja kwa moja — hakuna ld, hakuna libc.
;   Hii ndiyo hatua ya 0% bootstrap gap: mbegu inajenga exe ya
;   stage1 bila kiunganishi chochote.
;   Mpangilio wa faili: kichwa (64) + phdr (56) + stub ya _start
;   (28) + .text + .data. .bss inafunikwa na p_memsz pekee.
;   Awamu ya 1: rekebisha RELA zote kwenye text_buf (disp32).
;   Awamu ya 2: tafuta ofseti ya main.
;   Awamu ya 3: toa kila kitu kwa stdout.
; -------------------------------------------------------
toa_exe:
        push    r12
        push    r13
        push    r14
        push    r15
        push    rbx

        mov     r12d, [text_buf_pos]    ; text_size
        mov     r15d, [data_buf_pos]    ; data_size

        ; === Awamu ya 1: rekebisha RELA ===
        ; Ulimwengu -> data_vaddr + off (+data_size kwa bss);
        ; .data -> data_vaddr + (addend + 4); nje -> lebo ya ndani
        ; kwa jina (wito wa mbele uliandikwa kama nje).
        ; disp32 = tgt - pos - 4 (stub ya 28 hubatilika pande zote).
        xor     r13d, r13d
.rela_loop:
        cmp     r13, [rela_count]
        jae     .rela_done
        mov     r14d, [rela_offset + r13*4]    ; pos
        mov     eax, [rela_sym + r13*4]        ; sym
        mov     ecx, [rela_addend + r13*4]     ; addend

        cmp     eax, -1
        je      .rela_sehemu
        cmp     eax, -1
        jg      .rela_nje
        ; --- Ulimwengu: gidx = -sym - 2 ---
        neg     eax
        sub     eax, 2
        mov     edx, [global_offset + rax*4]
        cmp     dword [global_is_bss + rax*4], 0
        je      .rela_ulimwengu_tgt
        add     edx, r15d               ; bss -> baada ya .data
.rela_ulimwengu_tgt:
        add     edx, r12d               ; + text_size (data_vaddr = text_vaddr + text_size)
        jmp     .rela_andika

.rela_sehemu:
        ; Rekebisho la .data: addend = data_offset - 4 (imefupishwa
        ; katika mbegu, si -4 kawaida). disp32 = S + addend - P:
        ; (text_size + data_offset) - 4 - pos = text_size + addend - pos.
        ; Ongeza 4 hapa kwa sababu .rela_andika huondoa pos + 4.
        mov     edx, r12d
        add     edx, ecx
        add     edx, 4
        jmp     .rela_andika

.rela_nje:
        ; Tafuta lebo ya ndani kwa jina extern_name[sym].
        push    r13
        push    r14
        mov     rbx, [extern_name + rax*8]
        xor     ecx, ecx
.rela_nje_scan:
        cmp     rcx, [label_count]
        jae     .rela_nje_sio
        mov     rdi, [label_name + rcx*8]
        mov     rsi, rbx
        push    rcx
        call    linganisha_mfuatano
        pop     rcx
        cmp     eax, 0
        je      .rela_nje_iko
        inc     rcx
        jmp     .rela_nje_scan
.rela_nje_iko:
        mov     edx, [label_offset + rcx*4]
        jmp     .rela_nje_safu
.rela_nje_sio:
        ; Kazi iliyoitwa haijafafanuliwa popote. Zamani hii ilikuwa
        ; ikitulia kimya kwa anwani 0 na kuleta SEGV wakati wa
        ; utekelezaji — k.m. `husisha { faili.swa }` hailiwii mbegu
        ; (chanzo kinapaswa kuunganishwa) na makosa ya tahajia ya
        ; majina ya kazi yalipita hadi kwenye mchakato. Sasa inalia
        ; kwa sauti wakati wa kukusanya.
        lea     rdi, [msg_kazi_kukosa]
        call    andika_mfuatano
        mov     rdi, rbx                ; jina la kazi
        call    andika_mfuatano
        lea     rdi, [msg_mstari_mpya]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit
.rela_nje_safu:
        pop     r14
        pop     r13

.rela_andika:
        sub     edx, r14d
        sub     edx, 4                  ; disp32 = tgt - pos - 4
        mov     [text_buf + r14], edx   ; andika disp32 (baiti 4, LE)
        inc     r13
        jmp     .rela_loop
.rela_done:

        ; === Awamu ya 2: tafuta ofseti ya main ===
        xor     ecx, ecx
.main_scan:
        cmp     rcx, [label_count]
        jae     .main_kukosa
        mov     rdi, [label_name + rcx*8]
        lea     rsi, [jina_main]
        push    rcx
        call    linganisha_mfuatano
        pop     rcx
        cmp     eax, 0
        je      .main_iko
        inc     rcx
        jmp     .main_scan
.main_iko:
        mov     ebx, [label_offset + rcx*4]    ; main_off
        jmp     .main_ok
.main_kukosa:
        lea     rdi, [msg_main_kukosa]
        call    andika_mfuatano
        mov     edi, 1
        call    sys_exit
.main_ok:

        ; === Awamu ya 3: toa kichwa + phdr + stub + text + data ===

        ; --- Kichwa cha ELF (baiti 64) ---
        ; e_ident: 7F 45 4C 46 02 01 01 00 + sufuri
        mov     edi, 0x464C457F
        call    andika_neno4_moja_kwa_moja
        mov     edi, 0x00010102
        call    andika_neno4_moja_kwa_moja
        mov     edi, 0
        call    andika_neno4_moja_kwa_moja
        mov     edi, 0
        call    andika_neno4_moja_kwa_moja
        ; e_type=2 (EXEC), e_machine=0x3E
        mov     edi, 0x003E0002
        call    andika_neno4_moja_kwa_moja
        ; e_version=1
        mov     edi, 1
        call    andika_neno4_moja_kwa_moja
        ; e_entry = 0x400078
        mov     edi, 0x400078
        call    andika_neno4_moja_kwa_moja
        mov     edi, 0
        call    andika_neno4_moja_kwa_moja
        ; e_phoff = 64
        mov     edi, 64
        call    andika_neno4_moja_kwa_moja
        mov     edi, 0
        call    andika_neno4_moja_kwa_moja
        ; e_shoff = 0
        mov     edi, 0
        call    andika_neno4_moja_kwa_moja
        mov     edi, 0
        call    andika_neno4_moja_kwa_moja
        ; e_flags = 0
        mov     edi, 0
        call    andika_neno4_moja_kwa_moja
        ; e_ehsize=64, e_phentsize=56
        mov     edi, 0x00380040
        call    andika_neno4_moja_kwa_moja
        ; e_phnum=1, e_shentsize=0
        mov     edi, 1
        call    andika_neno4_moja_kwa_moja
        ; e_shnum=0, e_shstrndx=0
        mov     edi, 0
        call    andika_neno4_moja_kwa_moja

        ; --- Kichwa cha programu (baiti 56): PT_LOAD, RWX, p_offset=0 ---
        mov     edi, 1                  ; p_type = PT_LOAD
        call    andika_neno4_moja_kwa_moja
        mov     edi, 7                  ; p_flags = R|W|X
        call    andika_neno4_moja_kwa_moja
        mov     rdi, 0                  ; p_offset = 0
        call    andika_neno8_moja_kwa_moja
        mov     rdi, 0x400000           ; p_vaddr
        call    andika_neno8_moja_kwa_moja
        mov     rdi, 0x400000           ; p_paddr
        call    andika_neno8_moja_kwa_moja
        ; p_filesz = 120 (kichwa+phdr) + 28 (stub) + text + data
        mov     r14d, r12d
        add     r14d, r15d
        add     r14d, 148
        mov     rdi, r14
        call    andika_neno8_moja_kwa_moja
        ; p_memsz = filesz + bss
        add     r14d, dword [bss_size]
        mov     rdi, r14
        call    andika_neno8_moja_kwa_moja
        mov     rdi, 0x1000             ; p_align
        call    andika_neno8_moja_kwa_moja

        ; --- Stub ya _start (baiti 28) ---
        ; Maneno 7 ya N32 — sawa na uzalishaji.swa (kipande32).
        ; 48 8B 3C 24 — mov rdi, [rsp] (argc)
        mov     edi, 607947592
        call    andika_neno4_moja_kwa_moja
        ; 48 8D 74 24 — lea rsi, [rsp+8] (argv), baiti ya mwisho kwenye neno linalofuata
        mov     edi, 611618120
        call    andika_neno4_moja_kwa_moja
        ; 08 B0 00 E8 — mwisho wa lea + mov al,0 + call (disp32 inafuata)
        mov     edi, 3892359176
        call    andika_neno4_moja_kwa_moja
        ; disp32 = main_off + 12 (wito uko baiti 11-14, kifuatacho 16)
        mov     edi, ebx
        add     edi, 12
        call    andika_neno4_moja_kwa_moja
        ; 48 89 C7 B8 — mov rdi, rax + mwanzo wa mov eax, 60
        mov     edi, 3100084552
        call    andika_neno4_moja_kwa_moja
        ; 3C 00 00 00 — mwisho wa mov eax, 60
        mov     edi, 60
        call    andika_neno4_moja_kwa_moja
        ; 0F 05 00 00 — syscall + padding
        mov     edi, 1295
        call    andika_neno4_moja_kwa_moja

        ; --- .text (stub tayari imetolewa) ---
        xor     ecx, ecx
.text_loop:
        cmp     ecx, r12d
        jae     .text_done
        movzx   edi, byte [text_buf + rcx]
        push    rcx
        call    andika_baiti_moja_kwa_moja
        pop     rcx
        inc     rcx
        jmp     .text_loop
.text_done:

        ; --- .data ---
        xor     ecx, ecx
.data_loop:
        cmp     ecx, r15d
        jae     .data_done
        movzx   edi, byte [data_buf + rcx]
        push    rcx
        call    andika_baiti_moja_kwa_moja
        pop     rcx
        inc     rcx
        jmp     .data_loop
.data_done:

        pop     rbx
        pop     r15
        pop     r14
        pop     r13
        pop     r12
        ret

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
        add     r14, 4                  ; +1 kwa null + 3 za alama za sehemu (.text, .data, .bss)
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
        ; e_shnum = 8 (.null, .text, .data, .symtab, .strtab, .rela.text, .shstrtab, .bss)
        mov     word [r15 + 60], 8
        ; e_shstrndx = 6 (.shstrtab iko kabla ya .bss)
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

        ; Alama ya sehemu: .text (faharisi 1)
        mov     edi, 0                  ; st_name = 0
        call    andika_neno4_moja_kwa_moja
        mov     dil, 3                  ; STB_LOCAL | STT_SECTION
        call    andika_baiti_moja_kwa_moja
        mov     dil, 0
        call    andika_baiti_moja_kwa_moja
        mov     di, 1                   ; st_shndx = .text
        call    andika_neno2_moja_kwa_moja
        mov     edi, 0                  ; st_value = 0
        call    andika_neno8_moja_kwa_moja
        mov     edi, 0                  ; st_size = 0
        call    andika_neno8_moja_kwa_moja

        ; Alama ya sehemu: .data (faharisi 2)
        mov     edi, 0                  ; st_name = 0
        call    andika_neno4_moja_kwa_moja
        mov     dil, 3                  ; STB_LOCAL | STT_SECTION
        call    andika_baiti_moja_kwa_moja
        mov     dil, 0
        call    andika_baiti_moja_kwa_moja
        mov     di, 2                   ; st_shndx = .data
        call    andika_neno2_moja_kwa_moja
        mov     edi, 0                  ; st_value = 0
        call    andika_neno8_moja_kwa_moja
        mov     edi, 0                  ; st_size = 0
        call    andika_neno8_moja_kwa_moja

        ; Alama ya sehemu: .bss (faharisi 3)
        mov     edi, 0                  ; st_name = 0
        call    andika_neno4_moja_kwa_moja
        mov     dil, 3                  ; STB_LOCAL | STT_SECTION
        call    andika_baiti_moja_kwa_moja
        mov     dil, 0
        call    andika_baiti_moja_kwa_moja
        mov     di, 7                   ; st_shndx = .bss (sehemu ya 7)
        call    andika_neno2_moja_kwa_moja
        mov     edi, 0                  ; st_value = 0
        call    andika_neno8_moja_kwa_moja
        mov     edi, 0                  ; st_size = 0
        call    andika_neno8_moja_kwa_moja

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
        ; st_shndx = 2 (.data) au 7 (.bss) kulingana na global_is_bss
        mov     edi, [global_is_bss + r12*4]
        cmp     edi, 0
        je      .ulimwengu_data
        mov     di, 7                   ; .bss
        jmp     .ulimwengu_shndx
.ulimwengu_data:
        mov     di, 2                   ; .data
.ulimwengu_shndx:
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
        ; Angalia ikiwa ni rekebisho la .data (rela_sym == -1)
        cmp     r14d, -1
        jne     .reloc_global
        ; Rekebisho la .data: tumia faharisi ya sehemu 2
        mov     r14d, 2
        jmp     .reloc_info
.reloc_global:
        ; Angalia ikiwa ni rekebisho la kigezo cha ulimwengu (rela_sym <= -2)
        cmp     r14d, -1
        jg      .reloc_extern
        ; Faharisi ya alama = 4 (null + .text + .data + .bss) + label_count + global_idx
        neg     r14d
        sub     r14d, 2                 ; global_idx = -(rela_sym + 2)
        mov     rax, [label_count]
        add     r14d, eax
        add     r14d, 4
        jmp     .reloc_info
.reloc_extern:
        ; Faharisi ya alama = 4 (null + .text + .data + .bss) + label_count + global_count + extern_idx
        add     r14d, 4
        mov     rax, [label_count]
        add     r14d, eax
        add     r14d, [global_count]
.reloc_info:
        shl     r14, 32
        or      r14, 2                  ; R_X86_64_PC32
        mov     rdi, r14
        call    andika_neno8_moja_kwa_moja

        ; r_addend kutoka rela_addend (inaweza kuwa hasi)
        movsxd  rdi, dword [rela_addend + r12*4]
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
;   Tunaandika vichwa 8 (0 hadi 7)
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
        ; symtab_size = (3 + label_count + global_count + extern_count) * 24
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
        add     r14, 4                  ; +1 kwa null + 3 za alama za sehemu (.text, .data, .bss)
        imul    r14, 24
        mov     edi, r14d
        call    andika_neno8_moja_kwa_moja
        mov     edi, 4                  ; sh_link = .strtab
        call    andika_neno4_moja_kwa_moja
        mov     edi, 4                  ; sh_info = faharisi ya alama ya kwanza ya ulimwengu (baada ya .text, .data, .bss)
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

        ; 7: .bss — sh_name=13, SHT_NOBITS (8), SHF_WRITE|SHF_ALLOC (3)
        ; .bss haichukui nafasi kwenye faili (SHT_NOBITS)
        mov     edi, 13                 ; sh_name = ".bss"
        call    andika_neno4_moja_kwa_moja
        mov     edi, 8                  ; sh_type = NOBITS
        call    andika_neno4_moja_kwa_moja
        mov     rdi, 3                  ; sh_flags = WRITE|ALLOC
        call    andika_neno8_moja_kwa_moja
        mov     rdi, 0                  ; sh_addr
        call    andika_neno8_moja_kwa_moja
        mov     rdi, 0                  ; sh_offset = 0 (hakuna data kwenye faili)
        call    andika_neno8_moja_kwa_moja
        mov     rdi, [bss_size]         ; sh_size = ukubwa wa .bss
        call    andika_neno8_moja_kwa_moja
        mov     edi, 0                  ; sh_link
        call    andika_neno4_moja_kwa_moja
        mov     edi, 0                  ; sh_info
        call    andika_neno4_moja_kwa_moja
        mov     rdi, 8                  ; sh_addralign
        call    andika_neno8_moja_kwa_moja
        mov     rdi, 0                  ; sh_entsize
        call    andika_neno8_moja_kwa_moja

        ret
