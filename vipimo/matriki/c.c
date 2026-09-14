#include <stdio.h>
#include <stdlib.h>

#define N_KIPIMO 300

int main(void) {
    int n = N_KIPIMO;
    double *a = malloc(sizeof(double) * n * n);
    double *b = malloc(sizeof(double) * n * n);
    double *c = malloc(sizeof(double) * n * n);

    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            a[i * n + j] = (double)(i + j) * 0.5;
            b[i * n + j] = (double)(i - j) * 0.25;
            c[i * n + j] = 0.0;
        }
    }

    for (int i = 0; i < n; i++) {
        for (int k = 0; k < n; k++) {
            double aik = a[i * n + k];
            for (int j = 0; j < n; j++) {
                c[i * n + j] += aik * b[k * n + j];
            }
        }
    }

    printf("c00=%d cmid=%d clast=%d\n", (int)c[0], (int)c[(n / 2) * n + (n / 2)], (int)c[n * n - 1]);
    return 0;
}
