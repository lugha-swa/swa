#include <stdio.h>
#include <stdint.h>

#define N_JUMLA 2000000LL

static uint64_t djb2(const char *s) {
    uint64_t h = 5381;
    int i = 0;
    while (s[i] != 0) {
        h = ((h << 5) + h) + (unsigned char)s[i];
        i++;
    }
    return h;
}

int main(void) {
    char bafa[32];
    int64_t jumla_hash = 0;
    for (int64_t i = 0; i < N_JUMLA; i++) {
        snprintf(bafa, 32, "mfuatano_%lld_jaribio", (long long)i);
        jumla_hash += (int64_t)djb2(bafa);
    }
    printf("jumla_hash=%lld\n", (long long)jumla_hash);
    return 0;
}
