#include <stdio.h>
#include <stdlib.h>

typedef struct { int thamani; int kiwango; } Mfumo;

static Mfumo* mfumo_unda(int mwanzo) {
    Mfumo* m = malloc(sizeof(Mfumo));
    m->thamani = mwanzo;
    m->kiwango = 1;
    return m;
}
static int mfumo_pata_thamani(Mfumo* m) { return m->thamani; }
static int mfumo_pata_kiwango(Mfumo* m) { return m->kiwango; }
static void mfumo_sasisha(Mfumo* m, int pembejeo) {
    int sasa = mfumo_pata_thamani(m);
    int kw = mfumo_pata_kiwango(m);
    int mpya = sasa + (pembejeo % 7) - (kw % 3);
    if (mpya < 0) mpya = -mpya;
    m->thamani = mpya;
    if (mpya % 500 == 0) m->kiwango = kw + 1;
}

typedef struct { Mfumo *m1,*m2,*m3,*m4,*m5,*m6,*m7,*m8,*m9,*m10,*m11,*m12; } Mchezo;

static Mchezo* mchezo_unda(void) {
    Mchezo* g = malloc(sizeof(Mchezo));
    g->m1=mfumo_unda(10); g->m2=mfumo_unda(20); g->m3=mfumo_unda(30);
    g->m4=mfumo_unda(40); g->m5=mfumo_unda(50); g->m6=mfumo_unda(60);
    g->m7=mfumo_unda(70); g->m8=mfumo_unda(80); g->m9=mfumo_unda(90);
    g->m10=mfumo_unda(100); g->m11=mfumo_unda(110); g->m12=mfumo_unda(120);
    return g;
}

static void mchezo_sasisha(Mchezo* g, int zamu) {
    mfumo_sasisha(g->m1, zamu+1); mfumo_sasisha(g->m2, zamu+2);
    mfumo_sasisha(g->m3, zamu+3); mfumo_sasisha(g->m4, zamu+4);
    mfumo_sasisha(g->m5, zamu+5); mfumo_sasisha(g->m6, zamu+6);
    mfumo_sasisha(g->m7, zamu+7); mfumo_sasisha(g->m8, zamu+8);
    mfumo_sasisha(g->m9, zamu+9); mfumo_sasisha(g->m10, zamu+10);
    mfumo_sasisha(g->m11, zamu+11); mfumo_sasisha(g->m12, zamu+12);
}

#define ZAMU_IDADI 200000

int main() {
    Mchezo* g = mchezo_unda();
    int z = 0;
    while (z < ZAMU_IDADI) { mchezo_sasisha(g, z); z++; }
    int jumla = mfumo_pata_thamani(g->m1) + mfumo_pata_thamani(g->m6) + mfumo_pata_thamani(g->m12);
    printf("jumla=%d z=%d\n", jumla, z);
    return 0;
}
