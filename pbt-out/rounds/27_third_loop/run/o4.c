#include <stdio.h>
long f(int x) {
    int dead1 = x + 100;     // never used → DCE
    long wide = (long)x;     // widen
    int narrow = (int)wide;  // narrow back → narrow pass
    int dead2 = dead1 * 2;   // never used → DCE
    long result = narrow * 3;
    return result;
}
int main(void) { printf("%ld %ld\n", f(5), f(-7)); return 0; }
