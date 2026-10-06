#include <stdio.h>
typedef unsigned long long u64;
unsigned long long f0(a0, a1, a2, a3) {
    unsigned long long t0;
    t0 = a0 + 0ull;
    unsigned long long t1;
    t1 = a0 + 0ull;
    unsigned long long t2;
    t2 = a0 + 0ull;
    unsigned long long t3;
    t3 = a1 << 32ull;
    unsigned long long t4;
    t4 = a0 + 0ull;
    unsigned long long t5;
    t5 = a0 + 0ull;
    unsigned long long t6;
    t6 = t3 & 1ull;
    return t6;
}
u64 loopk(u64 n, u64 k) { u64 s = 0; for (u64 i = 0; i < (n & 15u); i++) s = s * k + i; return s; }
int main(void) {
    printf("%llu\n", f0(0ull, 0ull, 0ull, 0ull));
    printf("%llu\n", f0(18446744073709551615ull, 18446744073709551615ull, 18446744073709551615ull, 18446744073709551615ull));
    printf("%llu\n", f0(1ull, 2ull, 3ull, 4ull));
    printf("%llu\n", f0(708125762596286020ull, 12108840581919484505ull, 5062811327533131374ull, 16463526146856329859ull));
    printf("%llu\n", loopk(0, 3322321199));
    return 0;
}
