; stub.s -- boot stub NDOGO ya majaribio ya "stage1 --kernel" PEKEE
; (SI Kawira's kiini.s halisi -- hii ni sehemu ya gharama/jaribu-kernel.sh
; tu). Kioo cha kiini.s (Multiboot1, mabadiliko ya 32-bit->64-bit) lakini
; imepunguzwa kwa mahitaji ya jaribio tu: haihifadhi taarifa za Multiboot,
; haina ukaguzi wa magic (majaribio yanayoendeshwa hapa yanadhibitiwa na
; sisi wenyewe, si "real world" boot ya Kawira).
;
; %defassign JARIBIO (jina la faili la .bin lililotengenezwa na
; "stage1 --kernel", lililopitishwa kupitia -D kwenye amri ya nasm)
; inajumuishwa moja kwa moja (incbin) kama kernel_main.

[BITS 32]
[ORG 0x100000]

MB_MAGIC     equ 0x1BADB002
MB_FLAGS     equ 0x00010000
MB_CHECKSUM  equ -(MB_MAGIC + MB_FLAGS)

mb_header:
    dd MB_MAGIC
    dd MB_FLAGS
    dd MB_CHECKSUM
    dd mb_header
    dd mb_header
    dd 0
    dd 0
    dd _start32

_start32:
    cli
    mov esp, stack_top32

    mov eax, pdpt
    or eax, 0x03
    mov [pml4], eax
    mov dword [pml4 + 4], 0

    mov eax, pd
    or eax, 0x03
    mov [pdpt], eax
    mov dword [pdpt + 4], 0

    mov eax, 0x83
    mov [pd], eax
    mov dword [pd + 4], 0

    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    mov eax, pml4
    mov cr3, eax

    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    lgdt [gdt64.pointer]
    jmp CODE_SEG64:_start64

[BITS 64]
_start64:
    mov ax, DATA_SEG64
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    mov rsp, stack_top64

    call kernel_main

    ; kernel_main (jaribio) hurudisha n32 kwenye eax -- ihamishe kwenda
    ; edi ili QEMU monitor iweze kuisoma kutoka register inayoonekana
    ; kirahisi baada ya halt (angalia oracle.py).
    mov edi, eax
    cli
.halt64:
    hlt
    jmp .halt64

align 8
gdt64:
    dq 0
.code equ $ - gdt64
    dq 0x00209A0000000000
.data equ $ - gdt64
    dq 0x0000920000000000
.pointer:
    dw $ - gdt64 - 1
    dq gdt64

CODE_SEG64 equ gdt64.code
DATA_SEG64 equ gdt64.data

align 4096
pml4: times 512 dq 0
pdpt: times 512 dq 0
pd:   times 512 dq 0

align 16
resb 4096
stack_top32:

align 16
resb 16384
stack_top64:

kernel_main:
incbin JARIBIO
