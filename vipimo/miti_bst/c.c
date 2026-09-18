#include <stdio.h>
#include <stdlib.h>

#define N_INGIZA 150000
#define N_TAFUTA 150000

typedef struct Nodi {
    int thamani;
    struct Nodi *kushoto;
    struct Nodi *kulia;
} Nodi;

static long long lcg_hali = 98765;

static int nasibu_ijayo(void) {
    lcg_hali = (lcg_hali * 1103515245 + 12345) % 2147483648LL;
    return (int)lcg_hali;
}

static Nodi *ingiza(Nodi *mzizi, int v) {
    if (mzizi == NULL) {
        Nodi *n = malloc(sizeof(Nodi));
        n->thamani = v;
        n->kushoto = NULL;
        n->kulia = NULL;
        return n;
    }
    Nodi *sasa = mzizi;
    while (1) {
        if (v < sasa->thamani) {
            if (sasa->kushoto == NULL) {
                Nodi *n = malloc(sizeof(Nodi));
                n->thamani = v;
                n->kushoto = NULL;
                n->kulia = NULL;
                sasa->kushoto = n;
                return mzizi;
            }
            sasa = sasa->kushoto;
        } else {
            if (sasa->kulia == NULL) {
                Nodi *n = malloc(sizeof(Nodi));
                n->thamani = v;
                n->kushoto = NULL;
                n->kulia = NULL;
                sasa->kulia = n;
                return mzizi;
            }
            sasa = sasa->kulia;
        }
    }
}

static int tafuta(Nodi *mzizi, int v) {
    Nodi *sasa = mzizi;
    int h = 0;
    while (sasa != NULL) {
        h = h + 1;
        if (sasa->thamani == v) return h;
        if (v < sasa->thamani) sasa = sasa->kushoto;
        else sasa = sasa->kulia;
    }
    return 0 - h;
}

int main(void) {
    Nodi *mzizi = NULL;
    for (int i = 0; i < N_INGIZA; i++) {
        int v = nasibu_ijayo() % 20000000;
        mzizi = ingiza(mzizi, v);
    }

    int hatua_jumla = 0;
    int idadi_patikana = 0;
    for (int i = 0; i < N_TAFUTA; i++) {
        int v = nasibu_ijayo() % 20000000;
        int r = tafuta(mzizi, v);
        if (r > 0) {
            idadi_patikana++;
            hatua_jumla += r;
        } else {
            hatua_jumla -= r;
        }
    }

    printf("idadi_patikana=%d hatua_jumla=%d\n", idadi_patikana, hatua_jumla);
    return 0;
}
