#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int *c = calloc(8, sizeof(int));
    long N = 40000000L;
    for (long i = 0; i < N; i++) {
        int idx = i % 8;
        c[idx] += 1;
    }
    int jumla = 0;
    for (int i = 0; i < 8; i++) jumla += c[i];
    printf("jumla=%d\n", jumla);
    return 0;
}
