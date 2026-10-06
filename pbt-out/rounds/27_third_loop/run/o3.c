#include <stdio.h>
long f(int a, int b, int c) {
    long x;
    if (a) {
        if (b) { x = c * 2; } else { x = c * 2; }  // same in both → GVN + if-convert
    } else {
        x = c * 3;
    }
    // Another diamond → select
    long y = (a > b) ? (x + 1) : (x - 1);
    return x + y;
}
int main(void) {
    printf("%ld %ld %ld %ld\n", f(1,1,10), f(1,0,10), f(0,1,10), f(0,0,10));
    return 0;
}
