#include <stdio.h>
#include <stdlib.h>

typedef struct { double pato; double mfumuko; } Uchumi;
typedef struct { int watu; double ukuaji; } Idadi;
typedef struct { double ufanisi; double rushwa; } Utawala;
typedef struct { double kiwango_cha_makazi; double msongamano; } Makazi;
typedef struct { double kiwango_cha_kitisho; double uwezekano_wa_vita; } Akili;
typedef struct { double amani; double muungano_wa_nguvu; } Diplomasia;
typedef struct { double nguvu_ya_jeshi; double morali; } Jeshi;
typedef struct { double kiwango_cha_furaha; double utambulisho; } Utamaduni;

typedef struct {
    Uchumi *uchumi;
    Idadi *idadi;
    Utawala *utawala;
    Makazi *makazi;
    Akili *akili;
    Diplomasia *diplomasia;
    Jeshi *jeshi;
    Utamaduni *utamaduni;
} Mchezo;

static void uchumi_sasisha(Uchumi *u, double dt, int watu) {
    u->pato = u->pato + ((double)watu * 0.01 * dt);
    u->mfumuko = 0.02 + (u->pato * 0.0000001);
}

static void idadi_sasisha(Idadi *p, double pato) {
    p->ukuaji = 0.01 + (pato * 0.00000005);
    double nyongeza = p->ukuaji * p->watu;
    p->watu = p->watu + (int)nyongeza;
    if (p->watu > 1000000) p->watu = 1000000;
}

static void utawala_sasisha(Utawala *t, double mfumuko) {
    t->rushwa = t->rushwa + (mfumuko * 0.1);
    if (t->rushwa > 1.0) t->rushwa = 1.0;
    t->ufanisi = 1.0 - t->rushwa;
}

static void makazi_sasisha(Makazi *m, int watu, double ufanisi) {
    m->msongamano = (double)watu * 0.00001 * (2.0 - ufanisi);
    m->kiwango_cha_makazi = m->kiwango_cha_makazi + (ufanisi * 0.01);
}

static void akili_sasisha(Akili *a, double msongamano, double rushwa) {
    a->kiwango_cha_kitisho = (msongamano * 0.1) + (rushwa * 0.2);
    a->uwezekano_wa_vita = a->kiwango_cha_kitisho * 0.05;
}

static void diplomasia_sasisha(Diplomasia *d, double uwezekano_wa_vita) {
    d->amani = d->amani - (uwezekano_wa_vita * 0.02);
    if (d->amani < 0.0) d->amani = 0.0;
    d->muungano_wa_nguvu = d->amani * 0.5;
}

static void jeshi_sasisha(Jeshi *j, double pato, double uwezekano_wa_vita) {
    j->nguvu_ya_jeshi = j->nguvu_ya_jeshi + (pato * 0.001);
    j->morali = 1.0 - (uwezekano_wa_vita * 0.1);
}

static void utamaduni_sasisha(Utamaduni *c, double ufanisi, double amani) {
    c->kiwango_cha_furaha = (ufanisi * 0.5) + (amani * 0.5);
    c->utambulisho = c->utambulisho + 0.0001;
}

static void mchezo_sasisha(Mchezo *g) {
    double dt = 1.0;

    Idadi *idadi = g->idadi;
    int watu = idadi->watu;

    Uchumi *uchumi = g->uchumi;
    uchumi_sasisha(uchumi, dt, watu);
    double pato = uchumi->pato;
    double mfumuko = uchumi->mfumuko;

    idadi_sasisha(idadi, pato);

    Utawala *utawala = g->utawala;
    utawala_sasisha(utawala, mfumuko);
    double ufanisi = utawala->ufanisi;
    double rushwa = utawala->rushwa;

    Makazi *makazi = g->makazi;
    makazi_sasisha(makazi, watu, ufanisi);
    double msongamano = makazi->msongamano;

    Akili *akili = g->akili;
    akili_sasisha(akili, msongamano, rushwa);
    double uwezekano_wa_vita = akili->uwezekano_wa_vita;

    Diplomasia *diplomasia = g->diplomasia;
    diplomasia_sasisha(diplomasia, uwezekano_wa_vita);
    double amani = diplomasia->amani;

    Jeshi *jeshi = g->jeshi;
    jeshi_sasisha(jeshi, pato, uwezekano_wa_vita);

    Utamaduni *utamaduni = g->utamaduni;
    utamaduni_sasisha(utamaduni, ufanisi, amani);
}

int main(void) {
    Mchezo *g = malloc(sizeof(Mchezo));

    Uchumi *uchumi = malloc(sizeof(Uchumi));
    uchumi->pato = 1000.0;
    uchumi->mfumuko = 0.02;
    g->uchumi = uchumi;

    Idadi *idadi = malloc(sizeof(Idadi));
    idadi->watu = 100;
    idadi->ukuaji = 0.01;
    g->idadi = idadi;

    Utawala *utawala = malloc(sizeof(Utawala));
    utawala->ufanisi = 0.5;
    utawala->rushwa = 0.1;
    g->utawala = utawala;

    Makazi *makazi = malloc(sizeof(Makazi));
    makazi->kiwango_cha_makazi = 0.5;
    makazi->msongamano = 0.1;
    g->makazi = makazi;

    Akili *akili = malloc(sizeof(Akili));
    akili->kiwango_cha_kitisho = 0.1;
    akili->uwezekano_wa_vita = 0.01;
    g->akili = akili;

    Diplomasia *diplomasia = malloc(sizeof(Diplomasia));
    diplomasia->amani = 0.8;
    diplomasia->muungano_wa_nguvu = 0.4;
    g->diplomasia = diplomasia;

    Jeshi *jeshi = malloc(sizeof(Jeshi));
    jeshi->nguvu_ya_jeshi = 100.0;
    jeshi->morali = 0.9;
    g->jeshi = jeshi;

    Utamaduni *utamaduni = malloc(sizeof(Utamaduni));
    utamaduni->kiwango_cha_furaha = 0.6;
    utamaduni->utambulisho = 0.5;
    g->utamaduni = utamaduni;

    int MZUNGUKO_JUMLA = 200000;
    for (int t = 0; t < MZUNGUKO_JUMLA; t++) {
        mchezo_sasisha(g);
    }

    printf("watu=%d pato=%d furaha_x1000=%d\n", idadi->watu, (int)uchumi->pato, (int)(utamaduni->kiwango_cha_furaha * 1000.0));
    return 0;
}
