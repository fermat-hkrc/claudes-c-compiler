#include <stdio.h>
typedef unsigned long long u64;
unsigned long long f0(u64 a0) {
    unsigned long long t0;
    t0 = a0 + 0ull;
    unsigned long long t1;
    t1 = a0 + 0ull;
    return t1;
}
double d0(double a0, double a1) {
    double u0;
    u0 = a0 + a1;
    double u1;
    u1 = a0 + a1;
    return u1;
}
u64 loopk(u64 n, u64 k) { u64 s = 0; for (u64 i = 0; i < (n & 15u); i++) s = s * k + i; return s; }
int main(void) {
    printf("%llu\n", f0(0ull));
    printf("%llu\n", f0(18446744073709551615ull));
    printf("%llu\n", f0(1ull));
    printf("%llu\n", f0(708125762596286020ull));
    printf("%.17g\n", d0(0, 1.5));
    printf("%.17g\n", d0(3.25, -2));
    printf("%llu\n", loopk(0, 16988));
    return 0;
}
