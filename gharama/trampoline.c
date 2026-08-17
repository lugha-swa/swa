// trampoline.c — Kiunganishi cha wakati wa utekelezaji cha Swa
//
// Hutoa utekelezaji wa kazi za nje za mfumo wa Swa
// ambazo hazipatikani moja kwa moja kwenye libc.
// Inaunganishwa pamoja na faili la kitu lililotolewa na
// mkusanyaji wa Swa (kande au stage1).
//
// Maelezo ya kazi:
//   andika/andika_stderr -> vfprintf (kwa program bila maktaba;
//       kumbukumbu.swa ina utekelezaji wake wa ndani kwa mbegu/exe)
//   tekeleza/anwani_ya_kazi -> daraja za JIT
//   wito_wa_mfumo -> kwa mkusanyaji wa LLVM; mbegu ina builtin yake

#include <dlfcn.h>
#include <stdarg.h>
#include <stdlib.h>
#include <stdio.h>

int andika(const char* muundo, ...) {
    va_list hoja;
    va_start(hoja, muundo);
    int matokeo = vfprintf(stdout, muundo, hoja);
    va_end(hoja);
    fflush(stdout);
    return matokeo;
}

int andika_stderr(const char* muundo, ...) {
    va_list hoja;
    va_start(hoja, muundo);
    int matokeo = vfprintf(stderr, muundo, hoja);
    va_end(hoja);
    fflush(stderr);
    return matokeo;
}

// tekeleza — ita bafa ya JIT kama kazi N32(N32, N8**) (daraja kwa JIT)
int tekeleza(void* kazi, int argc, void* argv, int ofseti) {
    int (*f)(int, void*) = (int (*)(int, void*))kazi;
    return f(argc, (void*)((char**)argv + ofseti));
}

// anwani_ya_kazi — tafuta anwani ya kazi ya nje kwa jina (kwa JIT)
void* anwani_ya_kazi(const char* jina) {
    return dlsym(RTLD_DEFAULT, jina);
}

/* wito_wa_mfumo — daraja la syscall kwa mbegu (codegen ya Swa ni builtin) */
long wito_wa_mfumo(long namba, long a1, long a2, long a3, long a4, long a5) {
    register long r10 __asm__("r10") = a4;
    register long r8 __asm__("r8") = a5;
    register long r9 __asm__("r9") = 0;
    __asm__ volatile(
        "syscall"
        : "+a"(namba)
        : "D"(a1), "S"(a2), "d"(a3), "r"(r10), "r"(r8), "r"(r9)
        : "rcx", "r11", "memory");
    return namba;
}

