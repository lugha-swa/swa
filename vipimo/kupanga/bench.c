#include <stdio.h>
#include <stdlib.h>

#define KIASI 1000000
static unsigned long long rand_hali = 4101842887655102017ULL;

static int nasibu_ijayo(void) {
    rand_hali ^= rand_hali >> 32;
    rand_hali *= 2685821657736338717ULL;
    rand_hali ^= rand_hali >> 32;
    return (int)(rand_hali % 1000000000ULL);
}

static void badilisha(int* arr, int i, int j) {
    int tmp = arr[i]; arr[i] = arr[j]; arr[j] = tmp;
}

static int gawanya(int* arr, int chini, int juu) {
    int kati = chini + (juu - chini) / 2;
    if (arr[kati] < arr[chini]) badilisha(arr, kati, chini);
    if (arr[juu] < arr[chini]) badilisha(arr, juu, chini);
    if (arr[juu] < arr[kati]) badilisha(arr, juu, kati);
    badilisha(arr, kati, juu);
    int pivot = arr[juu];
    int i = chini - 1;
    int j = chini;
    while (j < juu) {
        if (arr[j] < pivot) { i++; badilisha(arr, i, j); }
        j++;
    }
    badilisha(arr, i + 1, juu);
    return i + 1;
}

static void panga(int* arr, int chini, int juu) {
    if (chini < juu) {
        int p = gawanya(arr, chini, juu);
        panga(arr, chini, p - 1);
        panga(arr, p + 1, juu);
    }
}

int main() {
    int* arr = malloc(sizeof(int) * KIASI);
    for (int i = 0; i < KIASI; i++) arr[i] = nasibu_ijayo();
    panga(arr, 0, KIASI - 1);
    int sahihi = 1;
    for (int i = 0; i < KIASI - 1; i++) if (arr[i] > arr[i+1]) sahihi = 0;
    printf("sahihi=%d kwanza=%d mwisho=%d\n", sahihi, arr[0], arr[KIASI-1]);
    return 0;
}
