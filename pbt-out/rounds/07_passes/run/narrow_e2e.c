#include <stdio.h>

long long or_i64(int a, int b)  { return (long long)a | (long long)b; }
long long and_i64(int a, int b) { return (long long)a & (long long)b; }
long long xor_i64(int a, int b) { return (long long)a ^ (long long)b; }

int main(void) {
    printf("%lld\n", or_i64(1, -1));
    printf("%lld\n", or_i64(-2147483648, 2147483647));
    printf("%lld\n", or_i64(0, 0));
    printf("%lld\n", and_i64(-1, 5));
    printf("%lld\n", xor_i64(-1, 0));
    return 0;
}
