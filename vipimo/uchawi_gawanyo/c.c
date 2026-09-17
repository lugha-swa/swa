#include <stdio.h>

int main(void) {
    long N = 50000000L;
    int jumla_q = 0, jumla_r = 0;
    for (int i = 0; i < N; i++) {
        jumla_q += i / 7;
        jumla_r += i % 7;
    }
    printf("jumla_q=%d jumla_r=%d\n", jumla_q, jumla_r);
    return 0;
}
