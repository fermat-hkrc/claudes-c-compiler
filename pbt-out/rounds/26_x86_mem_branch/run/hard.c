#include <stdio.h>
#include <string.h>
typedef struct { long a, b, c, d; } S16;
S16 byval(S16 s, int k) { s.a += k; s.b *= k; return s; }
long dispatch(int op, long x, long y) {
    switch (op) {
        case 0: return x + y; case 1: return x - y; case 2: return x * y;
        case 3: return x / (y ? y : 1); case 100: return x ^ y;
        case 101: return x | y; case 102: return x & y;
        case 103: return x << (y & 63); case 104: return x >> (y & 63);
        case 200: return ~x; case 201: return -x;
        case 300: return __builtin_popcountl(x);
        case 301: return __builtin_clzl(x > 0 ? x : 1);
        default: return x;
    }
}
int main(void) {
    S16 s = { 1, 2, 3, 4 };
    s = byval(s, 5);
    long t = 0;
    for (int i = 0; i < 20; i++) {
        t += dispatch(i % 12, i, 3) + dispatch(i + 200, t, 1);
        if (t > 1000000) t = t % 977;
    }
    char buf[64];
    memset(buf, 'A', 63); buf[63] = 0;
    printf("%ld %ld %ld %ld %ld %zu\n", s.a, s.b, s.c, s.d, t, strlen(buf));
    return 0;
}
