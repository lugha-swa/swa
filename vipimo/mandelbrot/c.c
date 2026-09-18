#include <stdio.h>

#define UKUBWA 600
#define KIWANGO_JUU 200

int main(void) {
    long long jumla_iter = 0;
    for (int py = 0; py < UKUBWA; py++) {
        for (int px = 0; px < UKUBWA; px++) {
            double x0 = ((double)px * 3.5 / UKUBWA) - 2.5;
            double y0 = ((double)py * 2.0 / UKUBWA) - 1.0;
            double x = 0.0, y = 0.0;
            int iter = 0;
            int hai_bado = 1;
            while (iter < KIWANGO_JUU && hai_bado == 1) {
                double x2 = x * x;
                double y2 = y * y;
                if ((x2 + y2) > 4.0) {
                    hai_bado = 0;
                } else {
                    double xt = (x2 - y2) + x0;
                    y = (2.0 * x * y) + y0;
                    x = xt;
                    iter++;
                }
            }
            jumla_iter += iter;
        }
    }
    printf("jumla_iter=%lld\n", jumla_iter);
    return 0;
}
