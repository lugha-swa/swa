#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define N_JUMLA 1000000

static int64_t lcg_hali = 12345LL;

static int nasibu_ijayo(void) {
    lcg_hali = (lcg_hali * 1103515245 + 12345) % 2147483648LL;
    return (int)lcg_hali;
}

static void badilishana(int *arr, int i, int j) {
    int t = arr[i];
    arr[i] = arr[j];
    arr[j] = t;
}

static int gawanya(int *arr, int chini, int juu) {
    int mhimili = arr[juu];
    int i = chini - 1;
    for (int j = chini; j < juu; j++) {
        if (arr[j] <= mhimili) {
            i++;
            badilishana(arr, i, j);
        }
    }
    badilishana(arr, i + 1, juu);
    return i + 1;
}

static void pangaharaka(int *arr, int chini, int juu) {
    if (chini < juu) {
        int p = gawanya(arr, chini, juu);
        pangaharaka(arr, chini, p - 1);
        pangaharaka(arr, p + 1, juu);
    }
}

int main(void) {
    int *arr = malloc(sizeof(int) * N_JUMLA);
    for (int i = 0; i < N_JUMLA; i++) {
        arr[i] = nasibu_ijayo();
    }

    pangaharaka(arr, 0, N_JUMLA - 1);

    int sahihi = 1;
    for (int i = 0; i < N_JUMLA - 1; i++) {
        if (arr[i] > arr[i + 1]) sahihi = 0;
    }

    printf("sahihi=%d kwanza=%d katikati=%d mwisho=%d\n", sahihi, arr[0], arr[N_JUMLA / 2], arr[N_JUMLA - 1]);
    return 0;
}
