#include <stdio.h>
static inline int square(int x) { return x * x; }
static long sum_squares(int n) {
    long s = 0;
    for (int i = 0; i < n; i++) s += square(i);  // inline + IVSR
    return s;
}
static int identity(int x) { return x; }
int main(void) {
    long t = 0;
    for (int i = 0; i < 20; i++) t += identity(i) * identity(i + 1);
    printf("%ld %ld\n", sum_squares(100), t);
    return 0;
}
