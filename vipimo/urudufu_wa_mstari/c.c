#include <stdio.h>

int main(void) {
    int suma1 = 0;
    int suma2 = 0;
    long N = 50000000L;
    for (long i = 0; i < N; i++) {
        int h1 = (int)(i * i + i);
        int h2 = (int)(i * i - i);
        suma1 = suma1 + h1;
        suma2 = suma2 + h2;
    }
    printf("suma1=%d suma2=%d\n", suma1, suma2);
    return 0;
}
