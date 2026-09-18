#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define IDADI_MANENO 400000
#define IDADI_KAMUSI 16
#define URUFU_JUU_NENO 12

static long long lcg_hali = 555777;

static int nasibu_ijayo(void) {
    lcg_hali = (lcg_hali * 1103515245 + 12345) % 2147483648LL;
    return (int)lcg_hali;
}

static const char *kamusi_neno(int idx) {
    switch (idx) {
        case 0: return "paka";
        case 1: return "mbwa";
        case 2: return "ndege";
        case 3: return "samaki";
        case 4: return "nyoka";
        case 5: return "tembo";
        case 6: return "simba";
        case 7: return "chui";
        case 8: return "nyani";
        case 9: return "kobe";
        case 10: return "sungura";
        case 11: return "kuku";
        case 12: return "bata";
        case 13: return "mbuzi";
        case 14: return "ngamia";
        default: return "farasi";
    }
}

int main(void) {
    long long ukubwa_bafa = ((long long)IDADI_MANENO * (URUFU_JUU_NENO + 1)) + 1;
    char *maandishi = malloc((size_t)ukubwa_bafa);
    long long wapi = 0;
    for (int w = 0; w < IDADI_MANENO; w++) {
        int idx = nasibu_ijayo() % IDADI_KAMUSI;
        const char *neno = kamusi_neno(idx);
        int ul = (int)strlen(neno);
        for (int k = 0; k < ul; k++) {
            maandishi[wapi++] = neno[k];
        }
        maandishi[wapi++] = ' ';
    }
    maandishi[wapi] = 0;

    int hesabu[IDADI_KAMUSI];
    memset(hesabu, 0, sizeof(hesabu));

    long long p = 0;
    while (p < wapi) {
        long long mwanzo = p;
        while (maandishi[p] != ' ' && maandishi[p] != 0) p++;
        int urefu_neno = (int)(p - mwanzo);
        for (int idx = 0; idx < IDADI_KAMUSI; idx++) {
            const char *kn = kamusi_neno(idx);
            int ukn = (int)strlen(kn);
            if (ukn == urefu_neno && memcmp(maandishi + mwanzo, kn, ukn) == 0) {
                hesabu[idx]++;
                break;
            }
        }
        p++;
    }

    long long cheki = 0;
    for (int idx = 0; idx < IDADI_KAMUSI; idx++) {
        cheki += (long long)hesabu[idx] * (idx + 1);
    }

    printf("cheki=%lld hesabu0=%d hesabu15=%d\n", cheki, hesabu[0], hesabu[15]);
    return 0;
}
