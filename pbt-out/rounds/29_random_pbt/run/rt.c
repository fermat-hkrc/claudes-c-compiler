#include <stdio.h>
#include <math.h>
int main(void) {
    double d0 = -2.871211;
    double d1 = 9.349787;
    double d2 = -0.189791;
    double d3 = 0.580187;
    d1 += -14.468622;
    d2 *= d1;
    d0 *= 38.069736;
    d2 -= 91.408495;
    d2 *= (-86.730047 * 24.842617);
    printf("%.6f ", d0);
    printf("%.6f ", d1);
    printf("%.6f ", d2);
    printf("%.6f ", d3);
    printf("\n");
    return 0;
}
