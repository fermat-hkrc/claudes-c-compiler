#include <stdio.h>
#include <math.h>
#include <float.h>
int main(void) {
    float f_min = FLT_MIN, f_max = FLT_MAX;
    double d_min = DBL_MIN, d_max = DBL_MAX;
    double sub = 4.9406564584124654e-324; // smallest subnormal
    double big = 1.7976931348623157e308;  // DBL_MAX

    printf("%d %d %d %d\n",
        f_min > 0, f_max > 1e30,
        d_min > 0, big == DBL_MAX);
    printf("%.10g %.10g\n", sub, sub * 2);
    printf("%d %d\n", (float)big == HUGE_VALF, isnan(0.0/0.0));

    // 精度链
    double d = 1.0 / 3.0;
    float f = (float)d;
    double back = (double)f;
    printf("%.17g %.17g %d\n", d, back, back != d);

    // 边界整数↔浮点
    printf("%d %lld %d\n",
        (int)(double)2147483647,
        (long long)(double)9007199254740992.0, // 2^53 exact
        (int)(double)2147483648.0 != 2147483648); // out of int range
    return 0;
}
