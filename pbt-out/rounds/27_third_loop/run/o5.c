#include <stdio.h>
static inline int clamp(int v, int lo, int hi) {
    return v < lo ? lo : (v > hi ? hi : v);
}
static long process(int *data, int n, int lim) {
    long sum = 0;
    int bound = clamp(lim, 0, 100);
    for (int i = 0; i < n; i++) {
        int v = clamp(data[i], -bound, bound);  // inline
        sum += v * (long)i;                      // IVSR potential
    }
    return sum;
}
int main(void) {
    int data[20];
    for (int i = 0; i < 20; i++) data[i] = i * 3 - 30;
    printf("%ld %ld\n", process(data, 20, 10), process(data, 20, 100));
    return 0;
}
