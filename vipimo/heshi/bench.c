#include <stdio.h>
#include <string.h>

#define IDADI 2000000

static int heshi_fnv(const char* s, int urefu) {
    int h = -2128831035;
    for (int i = 0; i < urefu; i++) {
        h = h ^ (unsigned char)s[i];
        h = h * 16777619;
    }
    return h;
}

int main() {
    const char* neno = "kompyuta_ya_mfano_kwa_uzito_wa_kati_12345";
    int urefu = (int)strlen(neno);
    long long jumla = 0;
    for (int i = 0; i < IDADI; i++) {
        int h = heshi_fnv(neno, urefu);
        jumla += h + i;
    }
    printf("jumla=%lld urefu=%d\n", jumla, urefu);
    return 0;
}
