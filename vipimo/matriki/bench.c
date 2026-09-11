#include <stdio.h>
#include <stdlib.h>

#define N 300

int main() {
    int* a = malloc(sizeof(int) * N * N);
    int* b = malloc(sizeof(int) * N * N);
    int* c = malloc(sizeof(int) * N * N);
    for (int i = 0; i < N * N; i++) {
        a[i] = (i % 97) + 1;
        b[i] = (i % 89) + 1;
        c[i] = 0;
    }
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            int jumla = 0;
            for (int k = 0; k < N; k++) {
                jumla += a[i * N + k] * b[k * N + j];
            }
            c[i * N + j] = jumla;
        }
    }
    long long hesabu = 0;
    for (int i = 0; i < N * N; i++) hesabu += c[i];
    printf("hesabu=%lld c00=%d\n", hesabu, c[0]);
    return 0;
}
