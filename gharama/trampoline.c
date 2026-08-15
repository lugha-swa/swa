// trampoline.c — Kiunganishi cha wakati wa utekelezaji cha Swa
//
// Hutoa utekelezaji wa kazi za nje za mfumo wa Swa
// ambazo hazipatikani moja kwa moja kwenye libc.
// Inaunganishwa pamoja na faili la kitu lililotolewa na
// mkusanyaji wa Swa (kande au stage1).
//
// Maelezo ya kazi:
//   andika  -> vfprintf (printf yenye hoja mbalimbali)
//   tenga   -> malloc   (imetolewa na libc, haihitaji kiunganishi)
//   achilia -> free     (imetolewa na libc, haihitaji kiunganishi)

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

/* bits_ya_d64 — faidika desimali ya chanzo na urudishe baiti zake za D64 */
unsigned long bits_ya_d64(const char* mwanzo, int urefu) {
    char bafa[64];
    int i;
    if (urefu > 63) urefu = 63;
    for (i = 0; i < urefu; i++) bafa[i] = mwanzo[i];
    bafa[urefu] = 0;
    double d = strtod(bafa, 0);
    union { double d; unsigned long u; } uu;
    uu.d = d;
    return uu.u;
}
