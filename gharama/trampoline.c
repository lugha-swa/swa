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

#include <stdarg.h>
#include <stdio.h>

int andika(const char* muundo, ...) {
    va_list hoja;
    va_start(hoja, muundo);
    int matokeo = vfprintf(stdout, muundo, hoja);
    va_end(hoja);
    fflush(stdout);
    return matokeo;
}
