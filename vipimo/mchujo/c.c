#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N_KIPIMO 25000000

int main(void) {
    int n = N_KIPIMO;
    unsigned char *mchujo = malloc((size_t)n + 1);
    memset(mchujo, 0, (size_t)n + 1);
    mchujo[0] = 1;
    mchujo[1] = 1;
    for (int i = 2; i * i <= n; i++) {
        if (mchujo[i] == 0) {
            for (int j = i * i; j <= n; j += i) {
                mchujo[j] = 1;
            }
        }
    }
    int idadi_kuu = 0;
    for (int i = 0; i <= n; i++) {
        if (mchujo[i] == 0) idadi_kuu++;
    }
    printf("idadi_kuu=%d\n", idadi_kuu);
    return 0;
}
