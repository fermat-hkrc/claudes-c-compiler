#include <stdio.h>
#define N 32
static unsigned char lookup[N] = { 0 };
static void init(void) { for (int i = 0; i < N; i++) lookup[i] = (unsigned char)(i * i); }
static inline unsigned char get(int i) { return lookup[i & (N - 1)]; }
static long accumulate(int start, int len) {
    long s = 0;
    unsigned char prev = get(start);
    for (int i = start + 1; i < start + len; i++) {
        unsigned char cur = get(i);
        s += (long)(cur - prev) * (cur + prev);
        prev = cur;
    }
    return s;
}
int main(void) {
    init();
    printf("%ld %ld %ld\n", accumulate(0, 32), accumulate(5, 20), accumulate(100, 10));
    return 0;
}
