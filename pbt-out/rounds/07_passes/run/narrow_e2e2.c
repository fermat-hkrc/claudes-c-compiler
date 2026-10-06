#include <stdio.h>
long long v;
long long f1(int a, int b) { long long x = a, y = b; return x | y; }
long long f2(int a, int b) { v = a; return v | (long long)b; }
long long f3(int a, int b) { return a > 0 ? (long long)a | (long long)b : 0; }
int main(void) {
    printf("%lld %lld %lld\n", f1(1, -1), f2(1, -1), f3(1, -1));
    return 0;
}
