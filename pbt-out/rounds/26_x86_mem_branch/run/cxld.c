#include <stdio.h>
#include <complex.h>
int main(void) {
    long double complex a = 1.5L + 2.5L * I;
    long double complex b = 0.5L - 1.5L * I;
    long double complex s = a + b;
    long double complex p = a * b;
    printf("%.1Lf%+.1Lf %.1Lf%+.1Lf\n", creall(s), cimagl(s), creall(p), cimagl(p));
    return 0;
}
